//! Pure-Rust extractor for stdarch SIMD intrinsics (`docs/intrinsics/complete_intrinsics.csv`).
//!
//! Replaces the legacy Python extractor so all codegen and toolchain
//! metadata extraction lives in Rust `xtask`.

use anyhow::{Context, Result, bail};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone)]
struct IntrinsicRecord {
    arch: String,
    name: String,
    features: String,
    is_unsafe: bool,
    stability: String,
    file: String,
    doc: String,
    signature: String,
    instruction: String,
}

fn extract_doc_comment(lines: &[&str], start_idx: usize) -> String {
    let mut doc_lines = Vec::new();
    let doc_attr = Regex::new(r#"^#\[doc\s*=\s*"(.*)"\]$"#).unwrap();
    let mut i = start_idx as isize - 1;
    while i >= 0 {
        let stripped = lines[i as usize].trim();
        if let Some(rest) = stripped.strip_prefix("///") {
            doc_lines.push(rest.trim().to_string());
            i -= 1;
            continue;
        }
        if let Some(caps) = doc_attr.captures(stripped) {
            doc_lines.push(caps[1].to_string());
            i -= 1;
            continue;
        }
        if stripped.is_empty() {
            i -= 1;
            continue;
        }
        if stripped.starts_with("#[") && !stripped.starts_with("#[doc") {
            let opens = stripped.matches('[').count() as isize;
            let closes = stripped.matches(']').count() as isize;
            if opens > closes {
                let mut depth = closes - opens;
                while i > 0 && depth < 0 {
                    i -= 1;
                    let s = lines[i as usize].trim();
                    depth += s.matches('[').count() as isize - s.matches(']').count() as isize;
                }
            }
            i -= 1;
            continue;
        }
        break;
    }
    doc_lines.reverse();
    doc_lines.join(" ")
}

fn extract_instruction(lines: &[&str], start_idx: usize, end_idx: usize) -> String {
    let re = Regex::new(r"assert_instr\(\s*([^,)]+)").unwrap();
    for line in &lines[start_idx..end_idx.min(lines.len())] {
        if let Some(caps) = re.captures(line.trim()) {
            return caps[1].trim().to_string();
        }
    }
    String::new()
}

fn extract_signature(lines: &[&str], fn_line_idx: usize) -> String {
    let mut sig_lines = Vec::new();
    for line in &lines[fn_line_idx..] {
        let s = line.trim();
        sig_lines.push(s);
        if s.contains('{') || s.contains(';') {
            break;
        }
    }
    let mut full = sig_lines.join(" ");
    let re_pub = Regex::new(r"\bpub\s+").unwrap();
    let re_unsafe = Regex::new(r"\bunsafe\s+").unwrap();
    let re_const = Regex::new(r"\bconst\s+").unwrap();
    let re_extern = Regex::new(r#"\bextern\s+"[^"]*"\s*"#).unwrap();
    let re_attr = Regex::new(r"#\[[^\]]*\]\s*").unwrap();
    full = re_pub.replace_all(&full, "").into_owned();
    full = re_unsafe.replace_all(&full, "").into_owned();
    full = re_const.replace_all(&full, "").into_owned();
    full = re_extern.replace_all(&full, "").into_owned();
    if let Some(pos) = full.find(['{', ';']) {
        full.truncate(pos);
    }
    re_attr.replace_all(full.trim(), "").trim().to_string()
}

fn cfg_applies(cond: &str, arch_name: &str) -> bool {
    let cond = cond.trim();
    let not_arch = Regex::new(r#"^not\(\s*target_arch\s*=\s*"(\w+)"\s*\)$"#).unwrap();
    let eq_arch = Regex::new(r#"^target_arch\s*=\s*"(\w+)"$"#).unwrap();
    if let Some(caps) = not_arch.captures(cond) {
        return &caps[1] != arch_name;
    }
    if let Some(caps) = eq_arch.captures(cond) {
        return &caps[1] == arch_name;
    }
    false
}

fn merge_features(base: &str, extra: &[String]) -> String {
    let mut out = Vec::new();
    for group in std::iter::once(base).chain(extra.iter().map(String::as_str)) {
        for f in group.split(',') {
            let f = f.trim();
            if !f.is_empty() && !out.iter().any(|x| x == f) {
                out.push(f.to_string());
            }
        }
    }
    out.join(",")
}

fn extract_dir(arch_dir: &Path, arch_name: &str) -> Result<Vec<IntrinsicRecord>> {
    let tf_re = Regex::new(r#"#\[target_feature\(enable\s*=\s*"([^"]+)"\)\]"#).unwrap();
    let tf_enable_re = Regex::new(r#"target_feature\(enable\s*=\s*"([^"]+)"\)"#).unwrap();
    let cfg_attr_tf_re = Regex::new(
        r#"#\[cfg_attr\(\s*(.+?)\s*,\s*target_feature\(enable\s*=\s*"([^"]+)"\)\s*\)\]"#,
    )
    .unwrap();
    let fn_re = Regex::new(r"^pub\s+(?:(?:const|unsafe)\s+)*fn\s+(\w+)").unwrap();
    let reexport_re = Regex::new(r"^pub\s+use\s+(\w+)\s+as\s+(\w+)\s*;").unwrap();

    let mut files: Vec<PathBuf> = WalkDir::new(arch_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "rs")
                && !p
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.contains("test"))
        })
        .collect();
    files.sort();

    let mut results = Vec::new();
    for filepath in files {
        let rel_path = filepath
            .strip_prefix(arch_dir)
            .unwrap_or(&filepath)
            .to_string_lossy()
            .replace('\\', "/");
        let content = fs::read_to_string(&filepath)?;
        let lines: Vec<&str> = content.lines().collect();
        let mut local_entries: HashMap<String, usize> = HashMap::new();

        let mut i = 0;
        while i < lines.len() {
            let stripped = lines[i].trim();
            if let Some(tf_caps) = tf_re.captures(stripped) {
                let base_features = tf_caps[1].to_string();
                let tf_line = i;
                let doc = extract_doc_comment(&lines, tf_line);
                let mut j = i + 1;
                let mut fn_name: Option<String> = None;
                let mut is_unsafe = false;
                let mut stability = "unknown".to_string();
                let mut extra_features = Vec::new();
                let mut bracket_depth: isize = 0;
                let mut in_arm_only_attr = false;
                let mut in_not_arm_attr = false;

                while j < lines.len() && j < i + 30 {
                    let fline = lines[j].trim();
                    if bracket_depth > 0 {
                        if fline.contains("not(target_arch = \"arm\")") {
                            in_not_arm_attr = true;
                            in_arm_only_attr = false;
                        } else if fline.contains("target_arch = \"arm\"") {
                            in_arm_only_attr = true;
                            in_not_arm_attr = false;
                        }
                        if let Some(inner_tf) = tf_enable_re.captures(fline)
                            && ((in_not_arm_attr && arch_name != "arm")
                                || (in_arm_only_attr && arch_name == "arm"))
                        {
                            extra_features.push(inner_tf[1].to_string());
                        }
                        if !fline.contains("rustc_const") {
                            if fline.contains("stable(") && !fline.contains("unstable(") {
                                if !in_arm_only_attr {
                                    stability = "stable".to_string();
                                }
                            } else if fline.contains("unstable(")
                                && !in_not_arm_attr
                                && !in_arm_only_attr
                            {
                                stability = "unstable".to_string();
                            }
                        }
                        bracket_depth += fline.matches('[').count() as isize
                            - fline.matches(']').count() as isize;
                        if bracket_depth <= 0 {
                            bracket_depth = 0;
                            in_arm_only_attr = false;
                            in_not_arm_attr = false;
                        }
                        j += 1;
                        continue;
                    }

                    if fline.contains("#[stable(") {
                        stability = "stable".to_string();
                    }
                    if fline.contains("#[unstable(") {
                        stability = "unstable".to_string();
                    }
                    if let Some(fn_caps) = fn_re.captures(fline) {
                        fn_name = Some(fn_caps[1].to_string());
                        let before_fn = fline.split("fn").next().unwrap_or("");
                        is_unsafe = before_fn.contains("unsafe");
                        break;
                    }
                    if fline.starts_with("#[") {
                        if let Some(cfg_tf) = cfg_attr_tf_re.captures(fline)
                            && cfg_applies(&cfg_tf[1], arch_name)
                        {
                            extra_features.push(cfg_tf[2].to_string());
                        }
                        if fline.contains("cfg_attr") {
                            if fline.contains("not(target_arch = \"arm\")") {
                                in_not_arm_attr = true;
                            } else if fline.contains("target_arch = \"arm\"") {
                                in_arm_only_attr = true;
                            }
                        }
                        if !fline.contains("rustc_const") {
                            if fline.contains("stable(") && !fline.contains("unstable(") {
                                if !in_arm_only_attr {
                                    stability = "stable".to_string();
                                }
                            } else if fline.contains("unstable(")
                                && !in_not_arm_attr
                                && !in_arm_only_attr
                            {
                                stability = "unstable".to_string();
                            }
                        }
                        bracket_depth = fline.matches('[').count() as isize
                            - fline.matches(']').count() as isize;
                        if bracket_depth <= 0 {
                            bracket_depth = 0;
                            in_arm_only_attr = false;
                            in_not_arm_attr = false;
                        }
                        j += 1;
                        continue;
                    }
                    if fline.starts_with("//")
                        || fline.is_empty()
                        || fline.starts_with('*')
                        || fline.starts_with("#[doc")
                    {
                        j += 1;
                        continue;
                    }
                    break;
                }

                if let Some(name) = fn_name {
                    let instruction = extract_instruction(&lines, tf_line, j);
                    let signature = extract_signature(&lines, j);
                    let idx = results.len();
                    results.push(IntrinsicRecord {
                        arch: arch_name.to_string(),
                        name: name.clone(),
                        features: merge_features(&base_features, &extra_features),
                        is_unsafe,
                        stability,
                        file: rel_path.clone(),
                        doc,
                        signature,
                        instruction,
                    });
                    local_entries.insert(name, idx);
                    i = j + 1;
                    continue;
                }
            }
            i += 1;
        }

        for line in &lines {
            if let Some(caps) = reexport_re.captures(line.trim()) {
                let source_name = &caps[1];
                let alias_name = &caps[2];
                if let Some(&src_idx) = local_entries.get(source_name) {
                    let mut cloned = results[src_idx].clone();
                    cloned.name = alias_name.to_string();
                    cloned.file = rel_path.clone();
                    cloned.signature = cloned.signature.replacen(source_name, alias_name, 1);
                    results.push(cloned);
                }
            }
        }
    }

    Ok(results)
}

fn csv_escape(field: &str) -> String {
    if field.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

/// Extract stdarch intrinsics into CSV matching `docs/intrinsics/complete_intrinsics.csv`.
pub fn extract_intrinsics_csv() -> Result<String> {
    let out = std::process::Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .context("Failed to execute rustc --print sysroot")?;
    if !out.status.success() {
        bail!("rustc --print sysroot failed");
    }
    let sysroot = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let stdarch_base =
        Path::new(&sysroot).join("lib/rustlib/src/rust/library/stdarch/crates/core_arch/src");
    if !stdarch_base.is_dir() {
        bail!(
            "stdarch source not found at {}. Install component with: rustup component add rust-src",
            stdarch_base.display()
        );
    }

    let mut all = Vec::new();
    for (subdir, arch) in [
        ("x86", "x86"),
        ("x86_64", "x86_64"),
        ("arm_shared", "aarch64"),
        ("aarch64", "aarch64"),
        ("wasm32", "wasm32"),
    ] {
        let dir = stdarch_base.join(subdir);
        if dir.is_dir() {
            all.extend(extract_dir(&dir, arch)?);
        }
    }

    let x86_64_names: HashSet<String> = all
        .iter()
        .filter(|r| r.arch == "x86_64")
        .map(|r| r.name.clone())
        .collect();

    let mut csv = String::with_capacity(512 * 1024);
    csv.push_str("arch,name,features,unsafe,stability,file,doc,signature,instruction\n");
    for mut r in all {
        if r.arch == "x86" {
            if x86_64_names.contains(&r.name) {
                continue;
            }
            r.arch = "x86_64".to_string();
        }
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            csv_escape(&r.arch),
            csv_escape(&r.name),
            csv_escape(&r.features),
            if r.is_unsafe { "True" } else { "False" },
            csv_escape(&r.stability),
            csv_escape(&r.file),
            csv_escape(&r.doc),
            csv_escape(&r.signature),
            csv_escape(&r.instruction),
        ));
    }
    Ok(csv)
}
