//! Token code generator — generates `src/tokens/generated/` from `token-registry.toml`.
//!
//! Produces:
//! - Capability lattice, arch markers, and `And` proof tags (`caps.rs`)
//! - Real implementations for native arch (with runtime detection)
//! - Stub implementations for cross-platform (summon → None)
//! - Trait definitions (`Has*` blanket impls + `IntoConcreteToken`)
//! - Module file with cfg-gated imports

use crate::registry::{Registry, TokenDef, TraitDef, tier_tag};
use indoc::formatdoc;

/// Convert token name to a screaming snake case variable name with given suffix.
fn screaming_snake(token_name: &str, suffix: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;
    for (i, c) in token_name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_was_upper {
                result.push('_');
            }
            result.push(c);
            prev_was_upper = true;
        } else if c.is_lowercase() {
            result.push(c.to_ascii_uppercase());
            prev_was_upper = false;
        } else {
            result.push(c);
            prev_was_upper = false;
        }
    }
    // Remove "TOKEN" suffix
    let result = result.trim_end_matches("_TOKEN").to_string();
    format!("{result}_{suffix}")
}

/// Convert token name to a lower_snake_case function name with given suffix.
pub(crate) fn lower_snake(token_name: &str, suffix: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;
    for (i, c) in token_name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_was_upper {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_was_upper = true;
        } else {
            result.push(c);
            prev_was_upper = c.is_uppercase();
        }
    }
    // Remove "_token" suffix
    let result = result.trim_end_matches("_token").to_string();
    format!("{result}_{suffix}")
}

/// Convert token name to a screaming snake case cache variable name.
fn cache_var_name(token_name: &str) -> String {
    screaming_snake(token_name, "CACHE")
}

/// Convert token name to a screaming snake case disabled variable name.
fn disabled_var_name(token_name: &str) -> String {
    screaming_snake(token_name, "DISABLED")
}

/// All generated token files as (relative_path, content) pairs.
///
/// Relative to `src/tokens/generated/`.
pub fn generate_token_files(reg: &Registry, major_version: u32) -> Vec<(String, String)> {
    let mut files = Vec::new();

    let deprecated_traits: Vec<&str> = Vec::new();

    // Group tokens by arch
    let x86_tokens: Vec<&TokenDef> = reg.token.iter().filter(|t| t.arch == "x86").collect();
    let arm_tokens: Vec<&TokenDef> = reg.token.iter().filter(|t| t.arch == "aarch64").collect();
    let wasm_tokens: Vec<&TokenDef> = reg.token.iter().filter(|t| t.arch == "wasm").collect();

    // Capability lattice, arch markers, Scalar/And assert tags
    files.push(("caps.rs".into(), gen_caps(reg, major_version)));

    // Real implementations (all x86 tokens in one file — no base/avx512 split)
    files.push((
        "x86.rs".into(),
        gen_real_tokens(reg, &x86_tokens, "x86", &deprecated_traits, major_version),
    ));
    files.push((
        "arm.rs".into(),
        gen_real_tokens(
            reg,
            &arm_tokens,
            "aarch64",
            &deprecated_traits,
            major_version,
        ),
    ));
    files.push((
        "wasm.rs".into(),
        gen_real_tokens(reg, &wasm_tokens, "wasm", &deprecated_traits, major_version),
    ));

    // Stubs (all x86 tokens in one file)
    files.push((
        "x86_stubs.rs".into(),
        gen_stub_tokens(reg, &x86_tokens, &deprecated_traits, major_version),
    ));
    files.push((
        "arm_stubs.rs".into(),
        gen_stub_tokens(reg, &arm_tokens, &deprecated_traits, major_version),
    ));
    files.push((
        "wasm_stubs.rs".into(),
        gen_stub_tokens(reg, &wasm_tokens, &deprecated_traits, major_version),
    ));

    // Traits
    files.push(("traits.rs".into(), gen_traits(reg)));

    // Module file
    files.push(("mod.rs".into(), gen_mod_rs()));

    files
}

// ============================================================================
// Real token implementations (native arch)
// ============================================================================

fn gen_real_tokens(
    reg: &Registry,
    tokens: &[&TokenDef],
    arch: &str,
    deprecated_traits: &[&str],
    major_version: u32,
) -> String {
    let _ = deprecated_traits;
    let mut out = String::with_capacity(4096);

    out.push_str(&formatdoc! {"
        //! Generated from token-registry.toml — DO NOT EDIT.
        //!
        //! Regenerate with: cargo xtask generate

        use crate::tokens::{{SimdLeafToken, SimdToken}};
    "});

    // Atomic imports (for runtime detection and process-wide token disabling)
    if !tokens.is_empty() {
        out.push_str("use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};\n");
    }

    // Collect all trait names used by tokens in this file (for compile-time verification)
    let trait_names: Vec<&str> = tokens
        .iter()
        .flat_map(|t| t.traits.iter().map(|s| s.as_str()))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    if !trait_names.is_empty() {
        let traits_list = trait_names.join(", ");
        out.push_str(&format!("use crate::tokens::{{{traits_list}}};\n"));
    }

    // Import ancestor tokens from other files (for extraction methods)
    let mut external_imports: std::collections::BTreeSet<(&str, &str)> =
        std::collections::BTreeSet::new();
    for token in tokens {
        let ancestors = reg.collect_ancestors(token);
        for ancestor in &ancestors {
            let in_file = tokens.iter().any(|other| other.name == ancestor.name);
            if !in_file {
                let mod_name = file_module_for_token(ancestor);
                external_imports.insert((mod_name, &ancestor.name));
            }
        }
    }
    for (mod_name, type_name) in &external_imports {
        out.push_str(&format!("use super::{mod_name}::{type_name};\n"));
    }

    out.push('\n');

    // Generate cache and disabled statics for all native tokens
    if !tokens.is_empty() {
        out.push_str("// Cache statics: 0 = unknown, 1 = unavailable, 2 = available\n");
        for token in tokens {
            let cache_name = cache_var_name(&token.name);
            let disabled_name = disabled_var_name(&token.name);

            // These statics are read/written ONLY by the runtime-detection
            // path, which is `cfg`'d out whenever every feature the token
            // asserts is already enabled at compile time (and
            // `testable_dispatch` is off). Whether that happens depends on the
            // TARGET's default feature set, which is not knowable here — so
            // every one of them is dead code on some target and live on
            // another, and `#[allow(dead_code)]` is unconditional.
            //
            // This used to be predicted from an architecture-baseline
            // heuristic (SSE/SSE2 on x86, NEON on aarch64). That was wrong:
            // `aarch64-apple-darwin` enables aes/sha2/sha3/crc by default, so
            // NEON_AES_*, NEON_SHA3_*, NEON_CRC_* and ARM64_V2_* were dead
            // there too and `cargo clippy -D warnings` failed on any Apple
            // Silicon host. CI never saw it because the Clippy lane runs on
            // ubuntu-latest (x86), where those cfgs resolve the other way.
            out.push_str(&formatdoc! {"
                #[allow(dead_code)]
                pub(super) static {cache_name}: AtomicU8 = AtomicU8::new(0);
                #[allow(dead_code)]
                pub(super) static {disabled_name}: AtomicBool = AtomicBool::new(false);
            "});
        }
        out.push('\n');
    }

    // Generate each token
    for token in tokens {
        gen_real_token_struct(&mut out, reg, token, arch, tokens);
        // Cold-path detection function (outside the impl block)
        match arch {
            "x86" => out.push_str(&gen_summon_cold_x86(reg, token)),
            "aarch64" => out.push_str(&gen_summon_cold_aarch64(reg, token)),
            "wasm" if token.runtime_probe => out.push_str(&gen_summon_cold_wasm(reg, token)),
            _ => {} // Compile-time only WASM tokens have no cold path
        }
        out.push('\n');
    }

    // Aliases
    for token in tokens {
        gen_aliases(&mut out, token);
    }

    // Tier tags — compile-time assertion constants
    gen_tier_tags(&mut out, tokens, major_version);

    // Trait verification (blanket impls live in traits.rs; verify each token satisfies its traits)
    gen_trait_impls(&mut out, tokens, deprecated_traits);

    out
}

fn gen_real_token_struct(
    out: &mut String,
    reg: &Registry,
    token: &TokenDef,
    arch: &str,
    all_tokens_in_file: &[&TokenDef],
) {
    let name = &token.name;
    let display = token.display_name.as_deref().unwrap_or(name);
    let dispatch_as = token
        .dispatch_as
        .as_deref()
        .expect("all registry tokens must have dispatch_as");

    // Doc comment
    let doc_block = gen_doc_comment(&token.doc);

    // Feature flag strings
    let (_, target_features, enable_flags, disable_flags) = feature_flag_strings(token);

    let (arch_marker, cap_set, cap_trait) = arch_cap_types(arch);
    let slot_defs = reg.cap_slots_for_arch(arch);
    let token_caps = reg.token_cap_slots(token);
    let slot_flags = slot_defs
        .iter()
        .map(|s| {
            if token_caps.contains(s.name.as_str()) {
                "super::caps::Yes"
            } else {
                "super::caps::No"
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    let mut cap_assoc_lines = String::new();
    for s in &slot_defs {
        let flag = if token_caps.contains(s.name.as_str()) {
            "super::caps::Yes"
        } else {
            "super::caps::No"
        };
        cap_assoc_lines.push_str(&format!("    type {} = {flag};\n", s.name));
    }

    let contract_bits_line = if token.runtime_probe {
        format!("{INDENT}const __ARCHMAGE_CONTRACT_BITS: u64 = 1;\n")
    } else {
        String::new()
    };

    out.push_str(&formatdoc! {"
        {doc_block}#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct {name} {{
            pub(in crate::tokens::generated) _private: (),
        }}

        impl crate::tokens::Sealed for {name} {{
            type Arch = super::caps::{arch_marker};
        }}

        impl super::caps::{cap_trait} for {name} {{
        {cap_assoc_lines}}}

        impl SimdToken for {name} {{
            type __Caps = super::caps::{cap_set}<{slot_flags}>;
        {contract_bits_line}    #[inline(always)]
            fn write_name(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                f.write_str(Self::NAME)
            }}

            #[inline(always)]
            fn write_target_features(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                f.write_str(Self::TARGET_FEATURES)
            }}

            #[inline(always)]
            fn has_target_feature(feature: &str) -> bool {{
                !feature.is_empty() && Self::TARGET_FEATURES.split(',').any(|f| f == feature)
            }}

            #[inline(always)]
            fn visit_target_features<F: FnMut(&'static str)>(f: &mut F) {{
                for feat in Self::TARGET_FEATURES.split(',') {{
                    if !feat.is_empty() {{
                        f(feat);
                    }}
                }}
            }}

    "});

    // compiled_with()
    out.push_str(&gen_compiled_with(token, arch));

    // summon()
    out.push('\n');
    out.push_str(&gen_summon(reg, token, arch));

    // Close SimdToken impl and emit leaf metadata + Display + IntoConcreteToken
    out.push_str(&formatdoc! {"
        }}

        impl SimdLeafToken for {name} {{
            const NAME: &'static str = \"{display}\";
            const TARGET_FEATURES: &'static str = \"{target_features}\";
            const ENABLE_TARGET_FEATURES: &'static str = \"{enable_flags}\";
            const DISABLE_TARGET_FEATURES: &'static str = \"{disable_flags}\";
        }}

        impl {name} {{
            /// Human-readable name for diagnostics and error messages.
            pub const NAME: &'static str = \"{display}\";

            /// Comma-delimited target features asserted by this token.
            pub const TARGET_FEATURES: &'static str = \"{target_features}\";

            /// RUSTFLAGS to enable these features at compile time.
            pub const ENABLE_TARGET_FEATURES: &'static str = \"{enable_flags}\";

            /// RUSTFLAGS to disable these features at compile time.
            pub const DISABLE_TARGET_FEATURES: &'static str = \"{disable_flags}\";

            /// Returns the human-readable name for this token.
            #[inline(always)]
            pub fn name(&self) -> &'static str {{
                Self::NAME
            }}

            /// Append another token from the same architecture, forming a conjunction proof.
            #[inline(always)]
            pub const fn and<Extra: SimdToken>(self, extra: Extra) -> crate::tokens::And<Self, Extra>
            where
                Self: crate::tokens::SameArch<Extra>,
            {{
                crate::tokens::And::join(self, extra)
            }}
        }}

        impl core::fmt::Display for {name} {{
            #[inline(always)]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                <Self as SimdToken>::write_name(f)
            }}
        }}

        impl crate::tokens::IntoConcreteToken for {name} {{
            #[inline(always)]
            fn {dispatch_as}(self) -> Option<Self> {{
                Some(self)
            }}
        }}
    "});

    // from_context(): safe constructor gated by #[target_feature] (omitted on runtime_probe tokens).
    out.push_str(&gen_from_context(token));

    // Extraction methods
    gen_extraction_methods(out, reg, token);

    // Disable/getter methods (all native tokens support disabling under testable_dispatch or runtime detection)
    gen_disable_methods(out, reg, token, all_tokens_in_file);
}

/// Generate the public `from_context()` constructor.
///
/// A **safe** `fn` carrying this tier's complete `#[target_feature]` list, which
/// makes rustc the arbiter of whether a given call site may construct the proof:
///
/// * From a caller whose own `#[target_feature]` set is this tier's set or a
///   superset (an `#[arcane]` / `#[rite]` / `#[magetypes]` body), the call is
///   safe and needs no `unsafe` block — the caller's attribute *is* the proof.
/// * From anywhere else the call is `unsafe`, and the caller carries the
///   obligation by hand. That is how the generated internals use it.
fn gen_from_context(token: &TokenDef) -> String {
    let name = &token.name;
    let features = token.features.join(",");
    let display = token.display_name.as_deref().unwrap_or(name);
    if token.runtime_probe {
        return String::new();
    }

    if token.arch == "wasm" {
        let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();
        let all_conditions = cfg_all_features(&check_features);
        return formatdoc! {r#"

            #[cfg(all(target_arch = "wasm32", {all_conditions}))]
            impl {name} {{
                /// Construct a {display} proof from the compile-time WASM target features.
                ///
                /// The required WASM features (`{features}`) are enabled at compile time,
                /// so the engine validates instruction support at module load time and no
                /// runtime detection is needed.
                ///
                /// # Safety
                ///
                /// When called through an `unsafe` block, the caller must ensure every
                /// feature in this tier is available. Safe calls have that obligation
                /// discharged by the compiler's `#[target_feature]` check.
                #[inline]
                #[target_feature(enable = "{features}")]
                pub fn from_context() -> Self {{
                    Self {{ _private: () }}
                }}
            }}

            #[cfg(all(target_arch = "wasm32", not(all({all_conditions}))))]
            impl {name} {{
                /// Construct a {display} proof without compile-time feature validation.
                ///
                /// The required WASM features (`{features}`) are NOT compile-time enabled
                /// in this build, so WebAssembly module load validation does not guarantee
                /// them and [`SimdToken::summon`](crate::SimdToken::summon) returns `None`.
                ///
                /// # Safety
                ///
                /// The caller must guarantee every feature in `{features}` is supported
                /// by the executing WASM engine. Constructing this token without engine
                /// support is undefined behavior when executing instructions from this tier.
                #[inline(always)]
                pub unsafe fn from_context() -> Self {{
                    Self {{ _private: () }}
                }}
            }}
        "#};
    }

    let context_docs = "/// A safe call requires a caller whose `#[target_feature]` attribute\n    /// enables every feature of this tier; a superset is sufficient. Outside\n    /// such a context rustc requires an `unsafe` block at the call site.\n    ///\n    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do\n    /// not satisfy this — rustc requires them on the caller's own attribute.";
    formatdoc! {r#"

        impl {name} {{
            /// Construct a {display} proof from the caller's statically proven
            /// feature context.
            ///
            {context_docs}
            ///
            /// No runtime detection happens here, so this also bypasses
            /// process-wide token disabling (including `testable_dispatch`):
            /// the caller's feature context is already the proof. Use
            /// [`SimdToken::summon`](crate::SimdToken::summon) when the
            /// features have to be detected at runtime.
            ///
            /// Being a `#[target_feature]` function, this cannot be coerced to
            /// a safe function pointer — there would be no call site left for
            /// rustc to check.
            ///
            /// # Safety
            ///
            /// When called through an `unsafe` block, the caller must ensure
            /// every feature in this tier is available on the executing CPU.
            /// Safe calls have that obligation discharged by the compiler.
            #[inline]
            #[target_feature(enable = "{features}")]
            pub fn from_context() -> Self {{
                Self {{ _private: () }}
            }}
        }}
    "#}
}

/// Generate an expression that evaluates to `true` if any ancestor of `token` is
/// currently disabled via `dangerously_disable_token_process_wide(true)`.
fn gen_ancestor_disabled_expr(reg: &Registry, token: &TokenDef) -> Option<String> {
    let ancestors = reg.collect_ancestors(token);
    if ancestors.is_empty() {
        return None;
    }
    Some(
        ancestors
            .iter()
            .map(|a| format!("{}.load(Ordering::Acquire)", disabled_var_name(&a.name)))
            .collect::<Vec<_>>()
            .join(" || "),
    )
}

/// Generate the `compiled_with()` method for a real token.
///
/// When the `testable_dispatch` feature is active, compile-time
/// detection always returns `None` so that runtime disable can work.
fn gen_compiled_with(token: &TokenDef, arch: &str) -> String {
    // The "testable_dispatch" feature suppresses the compile-time fast path:
    // compiled_with() returns None instead of Some(true), forcing runtime detection.
    let dct_guard = ", not(feature = \"testable_dispatch\")";

    match arch {
        "x86" => {
            // Check ALL features including sse/sse2: custom targets like
            // `-Ctarget-feature=-sse,-sse2` or soft-float kernels must fail closed.
            let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();
            let all_conditions = cfg_all_features(&check_features);
            formatdoc! {"
                {INDENT}#[inline]
                {INDENT}fn compiled_with() -> Option<bool> {{
                {INDENT}    #[cfg(all({all_conditions}{dct_guard}))]
                {INDENT}    {{ Some(true) }}
                {INDENT}    #[cfg(not(all({all_conditions}{dct_guard})))]
                {INDENT}    {{ None }}
                {INDENT}}}
            "}
        }
        "aarch64" => {
            // Check ALL features including neon — neon is NOT assumed baseline
            let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();
            let all_conditions = cfg_all_features(&check_features);

            formatdoc! {"
                {INDENT}#[inline]
                {INDENT}fn compiled_with() -> Option<bool> {{
                {INDENT}    #[cfg(all({all_conditions}{dct_guard}))]
                {INDENT}    {{ Some(true) }}
                {INDENT}    #[cfg(not(all({all_conditions}{dct_guard})))]
                {INDENT}    {{ None }}
                {INDENT}}}
            "}
        }
        "wasm" => {
            let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();
            let all_conditions = cfg_all_features(&check_features);
            if token.runtime_probe {
                // Even when +simd128,+relaxed-simd are enabled at compile time,
                // single-rounding FMA depends on the host WASM engine at runtime.
                // When those target features are NOT enabled at compile time, this
                // binary emits no relaxed-simd probe and can never summon the token.
                formatdoc! {"
                    {INDENT}#[inline]
                    {INDENT}fn compiled_with() -> Option<bool> {{
                    {INDENT}    #[cfg(all(target_arch = \"wasm32\", {all_conditions}))]
                    {INDENT}    {{ None }}
                    {INDENT}    #[cfg(not(all(target_arch = \"wasm32\", {all_conditions})))]
                    {INDENT}    {{ Some(false) }}
                    {INDENT}}}
                "}
            } else {
                // WASM has no runtime feature detection: unless `testable_dispatch`
                // suppresses compile-time guarantees on an enabled feature set,
                // availability is 100% determined at compile time via `cfg`.
                formatdoc! {"
                    {INDENT}#[inline]
                    {INDENT}fn compiled_with() -> Option<bool> {{
                    {INDENT}    #[cfg(all(target_arch = \"wasm32\", {all_conditions}{dct_guard}))]
                    {INDENT}    {{ Some(true) }}
                    {INDENT}    #[cfg(all(target_arch = \"wasm32\", {all_conditions}, feature = \"testable_dispatch\"))]
                    {INDENT}    {{ None }}
                    {INDENT}    #[cfg(not(all(target_arch = \"wasm32\", {all_conditions})))]
                    {INDENT}    {{ Some(false) }}
                    {INDENT}}}
                "}
            }
        }
        _ => unreachable!("unknown arch: {arch}"),
    }
}

fn gen_summon(reg: &Registry, token: &TokenDef, arch: &str) -> String {
    match arch {
        "x86" => gen_summon_x86(token),
        "aarch64" => gen_summon_aarch64(token),
        "wasm" => gen_summon_wasm(reg, token),
        _ => unreachable!("unknown arch: {arch}"),
    }
}

fn gen_summon_x86(token: &TokenDef) -> String {
    let dct_guard = ", not(feature = \"testable_dispatch\")";

    // Check ALL features including sse/sse2 (compile-time fast path still fires
    // with zero runtime cost on standard x86_64 targets where sse/sse2 are enabled).
    let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();
    let cache_name = cache_var_name(&token.name);
    let all_features = cfg_all_features(&check_features);
    let detect_fn_name = lower_snake(&token.name, "detect");

    formatdoc! {"
        {INDENT}#[inline(always)]
        {INDENT}fn summon() -> Option<Self> {{
        {INDENT}    // Compile-time fast path (suppressed by testable_dispatch)
        {INDENT}    #[cfg(all({all_features}{dct_guard}))]
        {INDENT}    {{
        {INDENT}        // SAFETY: every feature this token asserts is enabled at
        {INDENT}        // compile time (cfg(target_feature)), so the binary only
        {INDENT}        // runs on CPUs that have them.
        {INDENT}        Some(unsafe {{ Self::from_context() }})
        {INDENT}    }}

        {INDENT}    // Runtime path with caching
        {INDENT}    #[cfg(not(all({all_features}{dct_guard})))]
        {INDENT}    {{
        {INDENT}        match {cache_name}.load(Ordering::Acquire) {{
        {INDENT}            // SAFETY: 2 is only ever stored by {detect_fn_name}()
        {INDENT}            // after a positive runtime check of every feature.
        {INDENT}            2 => Some(unsafe {{ Self::from_context() }}),
        {INDENT}            1 => None,
        {INDENT}            _ => {detect_fn_name}(),
        {INDENT}        }}
        {INDENT}    }}
        {INDENT}}}
    "}
}

/// Generate the cold-path detection function for an x86 token.
///
/// This is `#[cold] #[inline(never)]` so summon()'s inline body stays tiny:
/// just an atomic u8 read + two-way branch. The full feature detection
/// (which calls into std_detect and probes CPUID) is outlined into this
/// function, which is called only on the first summon() for each token.
fn gen_summon_cold_x86(reg: &Registry, token: &TokenDef) -> String {
    let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();

    let name = &token.name;
    let cache_name = cache_var_name(name);
    let disabled_name = disabled_var_name(name);
    let detect_fn_name = lower_snake(name, "detect");
    let detect_expr = gen_x86_detect_expr(&check_features);
    let dct_guard = ", not(feature = \"testable_dispatch\")";
    let all_features = cfg_all_features(&check_features);
    let anc_disabled = gen_ancestor_disabled_expr(reg, token)
        .map(|e| format!(" || {e}"))
        .unwrap_or_default();

    formatdoc! {"
        #[cfg(not(all({all_features}{dct_guard})))]
        #[cold]
        #[inline(never)]
        fn {detect_fn_name}() -> Option<{name}> {{
            let available = {detect_expr};
            let pre_disabled = {disabled_name}.load(Ordering::Acquire){anc_disabled};
            // Publish only while the cache is still undetected (0): a
            // concurrent disable/enable writes the cache itself, and an
            // unconditional store here could overwrite it — resurrecting a
            // token that was disabled mid-detect (lost update: summon()
            // returning Some for a disabled token).
            let _ = {cache_name}.compare_exchange(
                0,
                if available && !pre_disabled {{ 2 }} else {{ 1 }},
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            // disable() stores the flag before the cache, so loading the
            // flag after the CAS observes every disable that could have
            // raced this probe.
            let disabled = {disabled_name}.load(Ordering::Acquire){anc_disabled};
            if available && !disabled {{
                // SAFETY: `available` — runtime detection just confirmed every
                // feature this token asserts is present on this CPU.
                Some(unsafe {{ {name}::from_context() }})
            }} else {{
                None
            }}
        }}

    "}
}

fn gen_summon_aarch64(token: &TokenDef) -> String {
    let dct_guard = ", not(feature = \"testable_dispatch\")";

    // Check ALL features including neon — neon is NOT assumed baseline
    let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();

    let cache_name = cache_var_name(&token.name);
    let all_features = cfg_all_features(&check_features);
    let detect_fn_name = lower_snake(&token.name, "detect");

    formatdoc! {"
        {INDENT}#[inline(always)]
        {INDENT}fn summon() -> Option<Self> {{
        {INDENT}    // Compile-time fast path (suppressed by testable_dispatch)
        {INDENT}    #[cfg(all({all_features}{dct_guard}))]
        {INDENT}    {{
        {INDENT}        // SAFETY: every feature this token asserts is enabled at
        {INDENT}        // compile time (cfg(target_feature)), so the binary only
        {INDENT}        // runs on CPUs that have them.
        {INDENT}        Some(unsafe {{ Self::from_context() }})
        {INDENT}    }}

        {INDENT}    // Runtime path with caching
        {INDENT}    #[cfg(not(all({all_features}{dct_guard})))]
        {INDENT}    {{
        {INDENT}        match {cache_name}.load(Ordering::Acquire) {{
        {INDENT}            // SAFETY: 2 is only ever stored by {detect_fn_name}()
        {INDENT}            // after a positive runtime check of every feature.
        {INDENT}            2 => Some(unsafe {{ Self::from_context() }}),
        {INDENT}            1 => None,
        {INDENT}            _ => {detect_fn_name}(),
        {INDENT}        }}
        {INDENT}    }}
        {INDENT}}}
    "}
}

/// Generate the cold-path detection function for an aarch64 token.
fn gen_summon_cold_aarch64(reg: &Registry, token: &TokenDef) -> String {
    let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();

    let name = &token.name;
    let cache_name = cache_var_name(name);
    let disabled_name = disabled_var_name(name);
    let detect_fn_name = lower_snake(name, "detect");
    let detect_expr = gen_aarch64_detect_expr(&check_features);
    let dct_guard = ", not(feature = \"testable_dispatch\")";
    let all_features = cfg_all_features(&check_features);
    let anc_disabled = gen_ancestor_disabled_expr(reg, token)
        .map(|e| format!(" || {e}"))
        .unwrap_or_default();

    formatdoc! {"
        #[cfg(not(all({all_features}{dct_guard})))]
        #[cold]
        #[inline(never)]
        fn {detect_fn_name}() -> Option<{name}> {{
            let available = {detect_expr};
            let pre_disabled = {disabled_name}.load(Ordering::Acquire){anc_disabled};
            // Publish only while the cache is still undetected (0): a
            // concurrent disable/enable writes the cache itself, and an
            // unconditional store here could overwrite it — resurrecting a
            // token that was disabled mid-detect (lost update: summon()
            // returning Some for a disabled token).
            let _ = {cache_name}.compare_exchange(
                0,
                if available && !pre_disabled {{ 2 }} else {{ 1 }},
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            // disable() stores the flag before the cache, so loading the
            // flag after the CAS observes every disable that could have
            // raced this probe.
            let disabled = {disabled_name}.load(Ordering::Acquire){anc_disabled};
            if available && !disabled {{
                // SAFETY: `available` — runtime detection just confirmed every
                // feature this token asserts is present on this CPU.
                Some(unsafe {{ {name}::from_context() }})
            }} else {{
                None
            }}
        }}

    "}
}

fn gen_summon_wasm(reg: &Registry, token: &TokenDef) -> String {
    let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();
    let all_conditions = cfg_all_features(&check_features);
    if token.runtime_probe {
        let cache_name = cache_var_name(&token.name);
        let detect_fn_name = lower_snake(&token.name, "detect");
        return formatdoc! {"
            {INDENT}#[inline(always)]
            {INDENT}fn summon() -> Option<Self> {{
            {INDENT}    #[cfg(all(target_arch = \"wasm32\", {all_conditions}))]
            {INDENT}    {{
            {INDENT}        match {cache_name}.load(Ordering::Acquire) {{
            {INDENT}            // 2 is only ever stored by {detect_fn_name}() after confirming
            {INDENT}            // single-rounding f32x4 and f64x2 FMA fusion across all lanes.
            {INDENT}            2 => Some(Self {{ _private: () }}),
            {INDENT}            1 => None,
            {INDENT}            _ => {detect_fn_name}(),
            {INDENT}        }}
            {INDENT}    }}
            {INDENT}    #[cfg(not(all(target_arch = \"wasm32\", {all_conditions})))]
            {INDENT}    {{
            {INDENT}        None
            {INDENT}    }}
            {INDENT}}}
        "};
    }
    let disabled_name = disabled_var_name(&token.name);
    let anc_disabled = gen_ancestor_disabled_expr(reg, token)
        .map(|e| format!(" || {e}"))
        .unwrap_or_default();
    formatdoc! {"
        {INDENT}#[inline(always)]
        {INDENT}fn summon() -> Option<Self> {{
        {INDENT}    #[cfg(all(target_arch = \"wasm32\", {all_conditions}))]
        {INDENT}    {{
        {INDENT}        #[cfg(feature = \"testable_dispatch\")]
        {INDENT}        if {disabled_name}.load(Ordering::Acquire){anc_disabled} {{
        {INDENT}            return None;
        {INDENT}        }}
        {INDENT}        // The required wasm features are compile-time enabled; a
        {INDENT}        // runtime that validated this module supports them (wasm has
        {INDENT}        // no runtime feature detection). wasm32 `#[target_feature]`
        {INDENT}        // functions are safe to call, so no `unsafe` is needed.
        {INDENT}        Some(Self::from_context())
        {INDENT}    }}
        {INDENT}    #[cfg(not(all(target_arch = \"wasm32\", {all_conditions})))]
        {INDENT}    {{
        {INDENT}        None
        {INDENT}    }}
        {INDENT}}}
    "}
}

/// Generate the cold-path runtime fusion probe for `Wasm128RelaxedFmaToken`.
///
/// Guarded by `#[cfg(all(target_arch = "wasm32", target_feature = "simd128", target_feature = "relaxed-simd"))]`
/// so strict `+simd128` WASM modules emit zero relaxed-SIMD opcodes and remain
/// valid under `WebAssembly.validate` on engines without Relaxed SIMD.
fn gen_summon_cold_wasm(reg: &Registry, token: &TokenDef) -> String {
    let check_features: Vec<&str> = token.features.iter().map(|s| s.as_str()).collect();
    let all_conditions = cfg_all_features(&check_features);
    let name = &token.name;
    let enable_features = token.features.join(",");
    let cache_name = cache_var_name(name);
    let disabled_name = disabled_var_name(name);
    let detect_fn_name = lower_snake(name, "detect");
    let anc_disabled = gen_ancestor_disabled_expr(reg, token)
        .map(|e| format!(" || {e}"))
        .unwrap_or_default();

    formatdoc! {"
        #[cfg(all(target_arch = \"wasm32\", {all_conditions}))]
        #[cold]
        #[inline(never)]
        #[target_feature(enable = \"{enable_features}\")]
        fn {detect_fn_name}() -> Option<{name}> {{
            use core::arch::wasm32;
            use core::hint::black_box;
            // (1 + 2^-12)^2 - (1 + 2^-11) == 2^-24 with single rounding; two roundings give 0.
            let x32 = wasm32::f32x4_splat(black_box(1.0_f32 + 1.0 / 4096.0));
            let c32 = wasm32::f32x4_splat(black_box(-(1.0_f32 + 1.0 / 2048.0)));
            let r32 = wasm32::f32x4_relaxed_madd(x32, x32, c32);
            let exp32 = 1.0_f32 / 16_777_216.0;
            let f32_fuses = wasm32::f32x4_extract_lane::<0>(r32) == exp32
                && wasm32::f32x4_extract_lane::<1>(r32) == exp32
                && wasm32::f32x4_extract_lane::<2>(r32) == exp32
                && wasm32::f32x4_extract_lane::<3>(r32) == exp32;
            // (1 + 2^-27)^2 - (1 + 2^-26) == 2^-54 with one rounding; two roundings give 0.
            let e64 = 1.0_f64 / 134_217_728.0;
            let x64 = wasm32::f64x2_splat(black_box(1.0_f64 + e64));
            let c64 = wasm32::f64x2_splat(black_box(-(1.0_f64 + 2.0 * e64)));
            let r64 = wasm32::f64x2_relaxed_madd(x64, x64, c64);
            let exp64 = e64 * e64;
            let f64_fuses = wasm32::f64x2_extract_lane::<0>(r64) == exp64
                && wasm32::f64x2_extract_lane::<1>(r64) == exp64;
            let available = f32_fuses && f64_fuses;
            let pre_disabled = {disabled_name}.load(Ordering::Acquire){anc_disabled};
            let _ = {cache_name}.compare_exchange(
                0,
                if available && !pre_disabled {{ 2 }} else {{ 1 }},
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            let disabled = {disabled_name}.load(Ordering::Acquire){anc_disabled};
            if available && !disabled {{
                // Runtime probe just confirmed single-rounding f32x4 and f64x2
                // relaxed_madd fusion across all lanes on this engine.
                Some({name} {{ _private: () }})
            }} else {{
                None
            }}
        }}

    "}
}

fn gen_extraction_methods(out: &mut String, reg: &Registry, token: &TokenDef) {
    // Combine explicit registry ancestors with strict feature-subset targets
    // (excluding runtime_probe tokens unless explicitly in the ancestor chain).
    let token_features: std::collections::BTreeSet<&str> =
        token.features.iter().map(|s| s.as_str()).collect();
    let ancestors = reg.collect_ancestors(token);

    let mut targets: Vec<&TokenDef> = reg
        .token
        .iter()
        .filter(|other| {
            other.name != token.name && other.arch == token.arch && {
                let is_ancestor = ancestors.iter().any(|a| a.name == other.name);
                let other_features: std::collections::BTreeSet<&str> =
                    other.features.iter().map(|s| s.as_str()).collect();
                let is_strict_subset = !other.runtime_probe
                    && other_features.is_subset(&token_features)
                    && other_features != token_features;
                is_ancestor || is_strict_subset
            }
        })
        .collect();

    if targets.is_empty() {
        return;
    }

    // Sort for deterministic output
    targets.sort_by_key(|t| &t.name);

    let name = &token.name;
    let token_display = token.display_name.as_deref().unwrap_or(name);

    out.push_str(&format!("\nimpl {name} {{\n"));

    for ancestor in &targets {
        let anc_name = &ancestor.name;
        let short = ancestor
            .short_name
            .as_deref()
            .unwrap_or("MISSING_SHORT_NAME");
        let anc_display = ancestor.display_name.as_deref().unwrap_or(anc_name);
        // On wasm32, sibling tokens live in the same module so direct struct
        // construction avoids cfg-dependent `from_context()` unsafety and
        // `unused_unsafe` warnings when extracting a weaker parent proof.
        let safe_wasm = token.arch == "wasm";

        let doc = formatdoc! {"
            {INDENT}/// Extract a {anc_name} — guaranteed because {token_display} implies {anc_display}.
            {INDENT}///
            {INDENT}/// Zero-cost: compiles away entirely.
            {INDENT}#[inline(always)]
        "};
        out.push_str(&doc);
        out.push_str(&if safe_wasm {
            formatdoc! {"
                {INDENT}pub fn {short}(self) -> {anc_name} {{
                {INDENT}    // Holding `self` proves {token_display}'s feature set, a superset
                {INDENT}    // of {anc_display}'s; constructing the parent proof directly avoids
                {INDENT}    // cfg-conditional `from_context()` unsafety across wasm builds.
                {INDENT}    let _ = self;
                {INDENT}    {anc_name} {{ _private: () }}
                {INDENT}}}
            "}
        } else {
            formatdoc! {"
                {INDENT}pub fn {short}(self) -> {anc_name} {{
                {INDENT}    // SAFETY: holding `self` proves this environment satisfies
                {INDENT}    // {token_display}'s full contract, a superset of {anc_display}'s.
                {INDENT}    unsafe {{ {anc_name}::from_context() }}
                {INDENT}}}
            "}
        });

        // Extraction aliases for this ancestor (e.g., .avx512() for X64V4Token)
        for alias_name in &ancestor.extraction_aliases {
            let body = if safe_wasm {
                formatdoc! {"
                    {INDENT}    self.{short}()
                "}
            } else {
                formatdoc! {"
                    {INDENT}    // SAFETY: identical to `.{short}()` — self's contract is a
                    {INDENT}    // registry-verified superset of {anc_display}'s.
                    {INDENT}    unsafe {{ {anc_name}::from_context() }}
                "}
            };
            out.push_str(&formatdoc! {"

                {INDENT}/// Get a {anc_name} (alias for `.{short}()`)
                {INDENT}#[inline(always)]
                {INDENT}pub fn {alias_name}(self) -> {anc_name} {{
                {body}{INDENT}}}
            "});
        }
    }

    out.push_str("}\n");
}

/// Generate disable/getter methods for a real token.
fn gen_disable_methods(
    out: &mut String,
    reg: &Registry,
    token: &TokenDef,
    all_tokens_in_file: &[&TokenDef],
) {
    let name = &token.name;
    let cache_name = cache_var_name(&token.name);
    let disabled_name = disabled_var_name(&token.name);

    // Collect descendants (tokens that have this token in their ancestor chain).
    let descendants: Vec<&TokenDef> = collect_descendants(reg, token);

    // Build cascade code
    let cascade_code = gen_cascade_code(&descendants, all_tokens_in_file);

    // Build descendant doc lines
    let descendant_docs = if !descendants.is_empty() {
        let mut lines = String::from("    ///\n    /// **Cascading:** Also affects descendants:\n");
        for desc in &descendants {
            lines.push_str(&format!("    /// - `{}`\n", desc.name));
        }
        lines
    } else {
        String::new()
    };

    let anc_disabled = gen_ancestor_disabled_expr(reg, token)
        .map(|e| format!(" || ({e})"))
        .unwrap_or_default();

    if token.runtime_probe {
        // Runtime-probed tokens (such as Wasm128RelaxedFmaToken) never elide
        // their runtime check into a compile-time constant, so process-wide
        // disabling always succeeds on the native architecture.
        out.push_str(&formatdoc! {"

            impl {name} {{
                /// Disable this token process-wide for testing and benchmarking.
                ///
                /// When disabled, `summon()` returns `None` even on engines where
                /// the runtime probe succeeds. Pass `false` to re-enable probing.
            {descendant_docs}    pub fn dangerously_disable_token_process_wide(disabled: bool) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {{
                    {disabled_name}.store(disabled, Ordering::Release);
                    let v = if disabled{anc_disabled} {{ 1 }} else {{ 0 }};
                    {cache_name}.store(v, Ordering::Release);
            {cascade_code}        Ok(())
                }}

                /// Check if this token has been manually disabled process-wide.
                #[inline(always)]
                pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {{
                    Ok({disabled_name}.load(Ordering::Acquire))
                }}
            }}
        "});
        return;
    }

    // For wasm non-probed tokens, availability is 100% compile-time determined
    // unless `testable_dispatch` is active (regardless of whether the wasm
    // features are enabled or disabled).
    // For x86/aarch64, compile-time guarantee holds when all features are enabled
    // and `testable_dispatch` is off.
    let (compile_time_cfg, runtime_cfg) = if token.arch == "wasm" {
        (
            "not(feature = \"testable_dispatch\")".to_string(),
            "feature = \"testable_dispatch\"".to_string(),
        )
    } else {
        let base_conditions = cfg_all_features_full(&token.features);
        let combined = format!("{base_conditions}, not(feature = \"testable_dispatch\")");
        (format!("all({combined})"), format!("not(all({combined}))"))
    };

    out.push_str(&formatdoc! {"

        impl {name} {{
            /// Disable this token process-wide for testing and benchmarking.
            ///
            /// When disabled, `summon()` will return `None` even if the CPU supports
            /// the required features.
            ///
            /// Returns `Err` when all required features are compile-time determined
            /// (e.g., via `-Ctarget-cpu=native`), since the compiler has already
            /// elided the runtime checks.
        {descendant_docs}    #[allow(clippy::needless_return)]
            pub fn dangerously_disable_token_process_wide(disabled: bool) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {{
                #[cfg({compile_time_cfg})]
                {{
                    let _ = disabled;
                    return Err(crate::tokens::CompileTimeGuaranteedError {{ token_name: Self::NAME, target_features: Self::TARGET_FEATURES, disable_flags: Self::DISABLE_TARGET_FEATURES }});
                }}
                #[cfg({runtime_cfg})]
                {{
                    {disabled_name}.store(disabled, Ordering::Release);
                    let v = if disabled{anc_disabled} {{ 1 }} else {{ 0 }};
                    {cache_name}.store(v, Ordering::Release);
        {cascade_code}            Ok(())
                }}
            }}

            /// Check if this token has been manually disabled process-wide.
            ///
            /// Returns `Err` when token availability is compile-time determined.
            #[inline(always)]
            #[allow(clippy::needless_return)]
            pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {{
                #[cfg({compile_time_cfg})]
                {{
                    return Err(crate::tokens::CompileTimeGuaranteedError {{ token_name: Self::NAME, target_features: Self::TARGET_FEATURES, disable_flags: Self::DISABLE_TARGET_FEATURES }});
                }}
                #[cfg({runtime_cfg})]
                {{
                    Ok({disabled_name}.load(Ordering::Acquire))
                }}
            }}
        }}
    "});
}

/// Generate cascade code for disabling descendant tokens.
fn gen_cascade_code(descendants: &[&TokenDef], all_tokens_in_file: &[&TokenDef]) -> String {
    let mut cascade = String::new();
    for desc in descendants {
        let desc_cache = cache_var_name(&desc.name);
        let desc_disabled = disabled_var_name(&desc.name);
        let desc_mod = file_module_for_token(desc);
        let in_same_file = all_tokens_in_file.iter().any(|t| t.name == desc.name);

        if in_same_file {
            cascade.push_str(&formatdoc! {"
                {INDENT}{INDENT}{INDENT}if disabled || v == 0 {{
                {INDENT}{INDENT}{INDENT}    {desc_disabled}.store(disabled, Ordering::Release);
                {INDENT}{INDENT}{INDENT}}}
                {INDENT}{INDENT}{INDENT}{desc_cache}.store(v, Ordering::Release);
            "});
        } else if let Some(cargo_feat) = &desc.cargo_feature {
            cascade.push_str(&formatdoc! {"
                {INDENT}{INDENT}{INDENT}#[cfg(feature = \"{cargo_feat}\")]
                {INDENT}{INDENT}{INDENT}{{
                {INDENT}{INDENT}{INDENT}    if disabled || v == 0 {{
                {INDENT}{INDENT}{INDENT}        super::{desc_mod}::{desc_disabled}.store(disabled, Ordering::Release);
                {INDENT}{INDENT}{INDENT}    }}
                {INDENT}{INDENT}{INDENT}    super::{desc_mod}::{desc_cache}.store(v, Ordering::Release);
                {INDENT}{INDENT}{INDENT}}}
            "});
        } else {
            cascade.push_str(&formatdoc! {"
                {INDENT}{INDENT}{INDENT}if disabled || v == 0 {{
                {INDENT}{INDENT}{INDENT}    super::{desc_mod}::{desc_disabled}.store(disabled, Ordering::Release);
                {INDENT}{INDENT}{INDENT}}}
                {INDENT}{INDENT}{INDENT}super::{desc_mod}::{desc_cache}.store(v, Ordering::Release);
            "});
        }
    }
    cascade
}

/// Collect all descendants (tokens that have this token in their ancestor chain).
fn collect_descendants<'a>(reg: &'a Registry, token: &'a TokenDef) -> Vec<&'a TokenDef> {
    let mut descendants = Vec::new();
    for other in &reg.token {
        if other.name == token.name {
            continue;
        }
        // Check if token is an ancestor of other
        let ancestors = reg.collect_ancestors(other);
        if ancestors.iter().any(|a| a.name == token.name) {
            descendants.push(other);
        }
    }
    descendants
}

// ============================================================================
// Stub token implementations (cross-platform)
// ============================================================================

fn arch_cap_types(arch: &str) -> (&'static str, &'static str, &'static str) {
    match arch {
        "x86" | "x86_64" => ("X64Arch", "X64CapSet", "X64Caps"),
        "aarch64" => ("ArmArch", "ArmCapSet", "ArmCaps"),
        "wasm" | "wasm32" => ("WasmArch", "WasmCapSet", "WasmCaps"),
        other => unreachable!("unknown arch for cap types: {other}"),
    }
}

fn gen_caps(reg: &Registry, major_version: u32) -> String {
    let mut out = String::with_capacity(8192);
    out.push_str(&formatdoc! {"
        //! Generated from token-registry.toml — DO NOT EDIT.
        //!
        //! Sealed architecture markers, type-level capability flags, and `And` proof tags.

        use crate::tokens::SimdToken;
        use core::marker::PhantomData;

        mod cap_sealed {{
            pub trait Sealed {{}}
        }}

        /// Sealed marker trait identifying a CPU architecture family.
        pub trait ArchMarker: cap_sealed::Sealed + Copy + Clone + Send + Sync + 'static {{}}

        /// Architecture marker for x86 / x86-64 tokens.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct X64Arch;
        impl cap_sealed::Sealed for X64Arch {{}}
        impl ArchMarker for X64Arch {{}}

        /// Architecture marker for AArch64 NEON tokens.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct ArmArch;
        impl cap_sealed::Sealed for ArmArch {{}}
        impl ArchMarker for ArmArch {{}}

        /// Architecture marker for WebAssembly SIMD tokens.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct WasmArch;
        impl cap_sealed::Sealed for WasmArch {{}}
        impl ArchMarker for WasmArch {{}}

        /// Architecture marker for [`ScalarToken`](crate::tokens::ScalarToken).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct ScalarArch;
        impl cap_sealed::Sealed for ScalarArch {{}}
        impl ArchMarker for ScalarArch {{}}

        /// Sealed type-level boolean flag for a single capability slot.
        pub trait Flag:
            cap_sealed::Sealed
            + Copy
            + Clone
            + core::fmt::Debug
            + PartialEq
            + Eq
            + core::hash::Hash
            + Send
            + Sync
            + 'static
        {{
            const ENABLED: bool;
            type Or<R: Flag>: Flag;
            type And<R: Flag>: Flag;
        }}

        /// Capability absent.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct No;
        impl cap_sealed::Sealed for No {{}}
        impl Flag for No {{
            const ENABLED: bool = false;
            type Or<R: Flag> = R;
            type And<R: Flag> = No;
        }}

        /// Capability present.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct Yes;
        impl cap_sealed::Sealed for Yes {{}}
        impl Flag for Yes {{
            const ENABLED: bool = true;
            type Or<R: Flag> = Yes;
            type And<R: Flag> = R;
        }}

        /// Capability set for [`ScalarToken`](crate::tokens::ScalarToken).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct ScalarCaps;

        /// Sealed type-level union of two capability sets from the same architecture.
        pub trait CapSetOr<Rhs>: cap_sealed::Sealed {{
            type Output: Copy + Clone + Send + Sync + 'static;
        }}

        impl cap_sealed::Sealed for ScalarCaps {{}}
        impl CapSetOr<ScalarCaps> for ScalarCaps {{
            type Output = ScalarCaps;
        }}
    "});

    for arch in ["x86", "aarch64", "wasm"] {
        let (_, cap_set, cap_trait) = arch_cap_types(arch);
        let slots = reg.cap_slots_for_arch(arch);
        let type_params = slots
            .iter()
            .map(|s| format!("{}: Flag", s.name))
            .collect::<Vec<_>>()
            .join(", ");
        let type_names = slots
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let assoc_types = slots
            .iter()
            .map(|s| format!("    type {}: Flag;\n", s.name))
            .collect::<String>();
        let and_assoc_types = slots
            .iter()
            .map(|s| {
                format!(
                    "    type {} = <Base::{} as Flag>::Or<Extra::{}>;\n",
                    s.name, s.name, s.name
                )
            })
            .collect::<String>();

        let lhs_params = slots
            .iter()
            .enumerate()
            .map(|(i, _)| format!("L{i}: Flag"))
            .collect::<Vec<_>>()
            .join(", ");
        let rhs_params = slots
            .iter()
            .enumerate()
            .map(|(i, _)| format!("R{i}: Flag"))
            .collect::<Vec<_>>()
            .join(", ");
        let lhs_names = (0..slots.len())
            .map(|i| format!("L{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let rhs_names = (0..slots.len())
            .map(|i| format!("R{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let or_exprs = (0..slots.len())
            .map(|i| format!("<L{i} as Flag>::Or<R{i}>"))
            .collect::<Vec<_>>()
            .join(", ");

        out.push_str(&formatdoc! {"

            #[allow(clippy::type_complexity)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            pub struct {cap_set}<{type_params}>(PhantomData<fn() -> ({type_names},)>);

            #[allow(clippy::type_complexity)]
            impl<{type_params}> cap_sealed::Sealed for {cap_set}<{type_names}> {{}}

            /// Sealed capability projection trait for `{arch}` tokens and conjunctions.
            pub trait {cap_trait}: crate::tokens::Sealed {{
            {assoc_types}}}

            impl<Base, Extra> {cap_trait} for crate::tokens::And<Base, Extra>
            where
                Base: crate::tokens::SameArch<Extra> + {cap_trait},
                Extra: SimdToken + {cap_trait},
            {{
            {and_assoc_types}}}

            #[allow(clippy::type_complexity)]
            impl<{lhs_params}, {rhs_params}> CapSetOr<{cap_set}<{rhs_names}>> for {cap_set}<{lhs_names}> {{
                type Output = {cap_set}<{or_exprs}>;
            }}
        "});
    }

    // Sound canonical token extractors guarded by exact capability slot requirements
    out.push_str(
        "\n// Canonical zero-cost token extractors (guarded by sealed capability slots)\n",
    );
    for token in &reg.token {
        let short = token
            .short_name
            .as_deref()
            .expect("every registry token needs short_name");
        let name = &token.name;
        let (_, cap_set, cap_trait) = arch_cap_types(&token.arch);
        let arch_slots = reg.cap_slots_for_arch(&token.arch);
        let token_caps = reg.token_cap_slots(token);
        let bindings = token_caps
            .iter()
            .map(|c| format!("{c} = Yes"))
            .collect::<Vec<_>>()
            .join(", ");
        let mut cap_generics = Vec::new();
        let mut cap_args = Vec::new();
        for (idx, slot) in arch_slots.iter().enumerate() {
            if token_caps.contains(slot.name.as_str()) {
                cap_args.push("Yes".to_string());
            } else {
                let param = format!("C{idx}");
                cap_generics.push(format!("{param}: Flag"));
                cap_args.push(param);
            }
        }
        let cap_fn_generics = if cap_generics.is_empty() {
            String::new()
        } else {
            format!("{}, ", cap_generics.join(", "))
        };
        let cap_args_str = cap_args.join(", ");
        out.push_str(&formatdoc! {"
            #[doc(hidden)]
            #[inline(always)]
            pub fn extract_{short}<T: SimdToken + {cap_trait}<{bindings}>>(_proof: T) -> super::{name} {{
                super::{name} {{ _private: () }}
            }}

            #[doc(hidden)]
            #[allow(clippy::type_complexity)]
            #[inline(always)]
            pub fn extract_{short}_caps<{cap_fn_generics}T: SimdToken<__Caps = {cap_set}<{cap_args_str}>>>(
                _proof: T,
            ) -> super::{name} {{
                super::{name} {{ _private: () }}
            }}
        "});
    }

    let scalar_tag = tier_tag("ScalarToken", major_version);
    let and_tag = tier_tag("And", major_version);
    out.push_str(&formatdoc! {"

        /// FNV-1a verification tag for [`And`](crate::tokens::And) conjunction types.
        #[doc(hidden)]
        pub const AND_ASSERT_TAG: u32 = 0x{and_tag:08X};

        impl crate::tokens::ScalarToken {{
            #[doc(hidden)]
            pub const __ARCHMAGE_TIER_TAG: u32 = 0x{scalar_tag:08X};

            #[doc(hidden)]
            pub const __ARCHMAGE_ASSERT_TIER_{scalar_tag:08X}: () =
                [()][!(Self::__ARCHMAGE_TIER_TAG == 0x{scalar_tag:08X}) as usize];
        }}

        impl<Base, Extra> crate::tokens::And<Base, Extra>
        where
            Base: crate::tokens::SameArch<Extra>,
            Extra: SimdToken,
        {{
            #[doc(hidden)]
            pub const __ARCHMAGE_TIER_TAG: u32 = 0x{and_tag:08X};

            #[doc(hidden)]
            pub const __ARCHMAGE_ASSERT_AND_{and_tag:08X}: () =
                [()][!(Self::__ARCHMAGE_TIER_TAG == 0x{and_tag:08X}) as usize];
        }}
    "});

    out
}

fn gen_stub_tokens(
    reg: &Registry,
    tokens: &[&TokenDef],
    deprecated_traits: &[&str],
    major_version: u32,
) -> String {
    let mut out = String::with_capacity(2048);

    out.push_str(&formatdoc! {"
        //! Generated from token-registry.toml — DO NOT EDIT.
        //!
        //! Stub tokens: `summon()` always returns `None`.

        use crate::tokens::{{SimdLeafToken, SimdToken}};
    "});

    // Collect all trait names (for compile-time verification)
    let trait_names: Vec<&str> = tokens
        .iter()
        .flat_map(|t| t.traits.iter().map(|s| s.as_str()))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    if !trait_names.is_empty() {
        let traits_list = trait_names.join(", ");
        out.push_str(&format!("use crate::tokens::{{{traits_list}}};\n"));
    }

    out.push('\n');

    // Generate struct + SimdToken impl for each token
    for token in tokens {
        gen_stub_token_struct(&mut out, reg, token);
        out.push('\n');
    }

    // Aliases
    for token in tokens {
        gen_aliases(&mut out, token);
    }

    // Tier tags — compile-time assertion constants (same tags as real tokens)
    gen_tier_tags(&mut out, tokens, major_version);

    // Trait verification (same as real — blanket impls in traits.rs apply to stubs too)
    gen_trait_impls(&mut out, tokens, deprecated_traits);

    out
}

fn gen_stub_token_struct(out: &mut String, reg: &Registry, token: &TokenDef) {
    let name = &token.name;
    let display = token.display_name.as_deref().unwrap_or(name);
    let dispatch_as = token
        .dispatch_as
        .as_deref()
        .expect("all registry tokens must have dispatch_as");

    // Feature flag strings — stubs carry the SAME strings as real tokens
    let (_, target_features, enable_flags, disable_flags) = feature_flag_strings(token);

    let (arch_marker, cap_set, cap_trait) = arch_cap_types(&token.arch);
    let slot_defs = reg.cap_slots_for_arch(&token.arch);
    let token_caps = reg.token_cap_slots(token);
    let slot_flags = slot_defs
        .iter()
        .map(|s| {
            if token_caps.contains(s.name.as_str()) {
                "super::caps::Yes"
            } else {
                "super::caps::No"
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    let mut cap_assoc_lines = String::new();
    for s in &slot_defs {
        let flag = if token_caps.contains(s.name.as_str()) {
            "super::caps::Yes"
        } else {
            "super::caps::No"
        };
        cap_assoc_lines.push_str(&format!("    type {} = {flag};\n", s.name));
    }

    let contract_bits_line = if token.runtime_probe {
        format!("{INDENT}const __ARCHMAGE_CONTRACT_BITS: u64 = 1;\n")
    } else {
        String::new()
    };

    out.push_str(&formatdoc! {"
        /// Stub for {display} token (not available on this architecture).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct {name} {{
            pub(in crate::tokens::generated) _private: (),
        }}

        impl crate::tokens::Sealed for {name} {{
            type Arch = super::caps::{arch_marker};
        }}

        impl super::caps::{cap_trait} for {name} {{
        {cap_assoc_lines}}}

        impl SimdToken for {name} {{
            type __Caps = super::caps::{cap_set}<{slot_flags}>;
        {contract_bits_line}    #[inline(always)]
            fn write_name(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                f.write_str(Self::NAME)
            }}

            #[inline(always)]
            fn write_target_features(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                f.write_str(Self::TARGET_FEATURES)
            }}

            #[inline(always)]
            fn has_target_feature(feature: &str) -> bool {{
                !feature.is_empty() && Self::TARGET_FEATURES.split(',').any(|f| f == feature)
            }}

            #[inline(always)]
            fn visit_target_features<F: FnMut(&'static str)>(f: &mut F) {{
                for feat in Self::TARGET_FEATURES.split(',') {{
                    if !feat.is_empty() {{
                        f(feat);
                    }}
                }}
            }}

            #[inline]
            fn compiled_with() -> Option<bool> {{
                Some(false) // Wrong architecture
            }}

            #[inline]
            fn summon() -> Option<Self> {{
                None // Not available on this architecture
            }}
        }}

        impl SimdLeafToken for {name} {{
            const NAME: &'static str = \"{display}\";
            const TARGET_FEATURES: &'static str = \"{target_features}\";
            const ENABLE_TARGET_FEATURES: &'static str = \"{enable_flags}\";
            const DISABLE_TARGET_FEATURES: &'static str = \"{disable_flags}\";
        }}

        impl {name} {{
            /// Human-readable name for diagnostics and error messages.
            pub const NAME: &'static str = \"{display}\";

            /// Comma-delimited target features asserted by this token.
            pub const TARGET_FEATURES: &'static str = \"{target_features}\";

            /// RUSTFLAGS to enable these features at compile time.
            pub const ENABLE_TARGET_FEATURES: &'static str = \"{enable_flags}\";

            /// RUSTFLAGS to disable these features at compile time.
            pub const DISABLE_TARGET_FEATURES: &'static str = \"{disable_flags}\";

            /// Returns the human-readable name for this token.
            #[inline(always)]
            pub fn name(&self) -> &'static str {{
                Self::NAME
            }}

            /// Append another token from the same architecture, forming a conjunction proof.
            #[inline(always)]
            pub const fn and<Extra: SimdToken>(self, extra: Extra) -> crate::tokens::And<Self, Extra>
            where
                Self: crate::tokens::SameArch<Extra>,
            {{
                crate::tokens::And::join(self, extra)
            }}

            /// This token is not available on this architecture.
            pub fn dangerously_disable_token_process_wide(_disabled: bool) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {{
                Err(crate::tokens::CompileTimeGuaranteedError {{ token_name: Self::NAME, target_features: Self::TARGET_FEATURES, disable_flags: Self::DISABLE_TARGET_FEATURES }})
            }}

            /// This token is not available on this architecture.
            pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {{
                Err(crate::tokens::CompileTimeGuaranteedError {{ token_name: Self::NAME, target_features: Self::TARGET_FEATURES, disable_flags: Self::DISABLE_TARGET_FEATURES }})
            }}
        }}

        impl core::fmt::Display for {name} {{
            #[inline(always)]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                <Self as SimdToken>::write_name(f)
            }}
        }}

        impl crate::tokens::IntoConcreteToken for {name} {{
            #[inline(always)]
            fn {dispatch_as}(self) -> Option<Self> {{
                Some(self)
            }}
        }}
    "});
}

// ============================================================================
// Aliases (shared between real and stub)
// ============================================================================

fn gen_aliases(out: &mut String, token: &TokenDef) {
    for alias in &token.aliases {
        out.push_str(&formatdoc! {"
            /// Type alias for [`{name}`].
            pub type {alias} = {name};

        ", name = token.name});
    }
}

// ============================================================================
// Trait verification (blanket impls generated in traits.rs)
// ============================================================================

fn gen_trait_impls(out: &mut String, tokens: &[&TokenDef], deprecated_trait_names: &[&str]) {
    let _ = deprecated_trait_names;
    let mut checks = Vec::new();
    for token in tokens {
        for trait_name in &token.traits {
            checks.push((&token.name, trait_name));
        }
    }
    if checks.is_empty() {
        return;
    }

    out.push_str(
        "\n// Compile-time check: capability blanket impls cover every token's declared traits.\n",
    );
    out.push_str("const _: () = {\n");
    for (token_name, trait_name) in checks {
        out.push_str(&format!(
            "    {{ const fn _assert<T: {trait_name}>() {{}} let _ = _assert::<{token_name}>; }}\n"
        ));
    }
    out.push_str("};\n");
}

// ============================================================================
// Tier tags — compile-time assertion constants
// ============================================================================

/// Generate tier tags and shared tier assertion constants for each token.
///
/// The expected tag is encoded in the assertion constant's name. Arcane refers
/// to it through the type in the signature, without requiring an archmage path.
fn gen_tier_tags(out: &mut String, tokens: &[&TokenDef], major_version: u32) {
    out.push('\n');
    for token in tokens {
        let name = &token.name;
        let tag = tier_tag(name, major_version);
        out.push_str(&formatdoc! {"
            impl {name} {{
                #[doc(hidden)]
                pub const __ARCHMAGE_TIER_TAG: u32 = 0x{tag:08X};

                #[doc(hidden)]
                pub const __ARCHMAGE_ASSERT_TIER_{tag:08X}: () =
                    [()][!(Self::__ARCHMAGE_TIER_TAG == 0x{tag:08X}) as usize];
            }}

        "});
    }
}

// ============================================================================
// Trait definitions
// ============================================================================

fn gen_traits(reg: &Registry) -> String {
    let mut out = String::with_capacity(4096);

    out.push_str(&formatdoc! {"
        //! Generated from token-registry.toml — DO NOT EDIT.
        //!
        //! Sealed capability marker traits with blanket impls for tokens and `And` conjunctions,
        //! plus [`IntoConcreteToken`] identity dispatch.

        use crate::tokens::{{And, SameArch, ScalarToken, SimdToken}};

    "});

    let deprecated_names: Vec<&str> = Vec::new();

    for trait_def in &reg.traits {
        gen_trait_def(&mut out, trait_def, &deprecated_names);
        out.push('\n');
    }

    // Generate IntoConcreteToken
    out.push_str(&formatdoc! {"
        /// Trait for compile-time dispatch via monomorphization.
        ///
        /// Each concrete leaf token implements this trait, returning `Some(self)` for
        /// its own type and `None` for others. Composite [`And<Base, Extra>`] tokens
        /// return `None` for all identity casts (they represent a conjunction of
        /// proofs rather than a single leaf type).
        pub trait IntoConcreteToken: SimdToken + Sized {{
    "});
    for token in &reg.token {
        let name = &token.name;
        let dispatch_as = token
            .dispatch_as
            .as_deref()
            .expect("all registry tokens must have dispatch_as");
        out.push_str(&formatdoc! {"
            {INDENT}/// Returns `Some(self)` if this is literally [`{name}`](super::{name}), `None` otherwise.
            {INDENT}#[inline(always)]
            {INDENT}fn {dispatch_as}(self) -> Option<super::{name}> {{
            {INDENT}    None
            {INDENT}}}

        "});
    }
    out.push_str(&formatdoc! {"
        {INDENT}/// Returns `Some(self)` if this is literally [`ScalarToken`], `None` otherwise.
        {INDENT}#[inline(always)]
        {INDENT}fn as_scalar(self) -> Option<ScalarToken> {{
        {INDENT}    None
        {INDENT}}}
        }}

        impl IntoConcreteToken for ScalarToken {{
            #[inline(always)]
            fn as_scalar(self) -> Option<ScalarToken> {{
                Some(self)
            }}
        }}

        impl<Base, Extra> IntoConcreteToken for And<Base, Extra>
        where
            Base: SameArch<Extra> + IntoConcreteToken,
            Extra: SimdToken,
        {{
            #[inline(always)]
            fn as_scalar(self) -> Option<ScalarToken> {{
                self.base().as_scalar()
            }}
        }}
    "});

    out
}

fn gen_trait_def(out: &mut String, trait_def: &TraitDef, deprecated_trait_names: &[&str]) {
    let _ = deprecated_trait_names;
    let doc_block = gen_doc_comment(&trait_def.doc);
    let name = &trait_def.name;
    let arch = trait_def
        .arch
        .as_deref()
        .expect("every registry trait must specify arch");
    let (_, _, cap_trait) = arch_cap_types(arch);
    let cap_bindings = trait_def
        .cap_req
        .iter()
        .map(|slot| format!("{slot} = super::caps::Yes"))
        .collect::<Vec<_>>()
        .join(", ");

    let bounds = if trait_def.parents.is_empty() {
        "SimdToken".to_string()
    } else {
        trait_def.parents.join(" + ")
    };

    out.push_str(&formatdoc! {"
        {doc_block}pub trait {name}: {bounds} {{}}

        impl<T> {name} for T where T: SimdToken + super::caps::{cap_trait}<{cap_bindings}> {{}}
    "});
}

// ============================================================================
// Module file
// ============================================================================

fn gen_mod_rs() -> String {
    formatdoc! {r#"
        //! Generated from token-registry.toml — DO NOT EDIT.
        //!
        //! cfg-gated module imports and re-exports for all token types.

        #[doc(hidden)]
        pub mod caps;

        mod traits;
        pub use traits::*;

        // x86_64: real implementations on x86_64, stubs elsewhere (including 32-bit x86)
        #[cfg(target_arch = "x86_64")]
        mod x86;
        #[cfg(target_arch = "x86_64")]
        pub use x86::*;

        #[cfg(not(target_arch = "x86_64"))]
        mod x86_stubs;
        #[cfg(not(target_arch = "x86_64"))]
        pub use x86_stubs::*;

        // aarch64: real implementations on aarch64, stubs elsewhere
        #[cfg(target_arch = "aarch64")]
        mod arm;
        #[cfg(target_arch = "aarch64")]
        pub use arm::*;

        #[cfg(not(target_arch = "aarch64"))]
        mod arm_stubs;
        #[cfg(not(target_arch = "aarch64"))]
        pub use arm_stubs::*;

        // wasm32: real implementations on wasm32, stubs elsewhere
        #[cfg(target_arch = "wasm32")]
        mod wasm;
        #[cfg(target_arch = "wasm32")]
        pub use wasm::*;

        #[cfg(not(target_arch = "wasm32"))]
        mod wasm_stubs;
        #[cfg(not(target_arch = "wasm32"))]
        pub use wasm_stubs::*;
    "#}
}

// ============================================================================
// Helpers
// ============================================================================

/// Indent constant for generated code inside impl blocks.
const INDENT: &str = "    ";

/// Compute the feature flag strings for a token:
/// (target_features, enable_flags, disable_flags).
///
/// All features are included (sse/sse2 are NOT stripped for x86).
/// For aarch64: includes all features (neon is NOT baseline).
/// For wasm: uses "simd128".
fn feature_flag_strings(token: &TokenDef) -> (&'static str, String, String, String) {
    let filtered: Vec<&str> = match token.arch.as_str() {
        "x86" => token.features.iter().map(|s| s.as_str()).collect(),
        "aarch64" => token.features.iter().map(|s| s.as_str()).collect(),
        "wasm" => token.features.iter().map(|s| s.as_str()).collect(),
        _ => vec![],
    };

    if filtered.is_empty() {
        return ("", String::new(), String::new(), String::new());
    }

    let target_features = filtered.join(",");
    let enable = format!(
        "-Ctarget-feature={}",
        filtered
            .iter()
            .map(|f| format!("+{f}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let disable = format!(
        "-Ctarget-feature={}",
        filtered
            .iter()
            .map(|f| format!("-{f}"))
            .collect::<Vec<_>>()
            .join(",")
    );

    // We return a &'static str "" placeholder — caller must use the Strings
    ("", target_features, enable, disable)
}

/// Determine which generated module file a token lives in.
fn file_module_for_token(token: &TokenDef) -> &str {
    match token.arch.as_str() {
        "x86" => "x86",
        "aarch64" => "arm",
        "wasm" => "wasm",
        _ => "unknown",
    }
}

/// Generate `target_feature = "feat1", target_feature = "feat2", ...` for cfg(all(...)).
fn cfg_all_features(features: &[&str]) -> String {
    features
        .iter()
        .map(|f| format!("target_feature = \"{f}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Generate cfg conditions from full (owned) feature list.
fn cfg_all_features_full(features: &[String]) -> String {
    features
        .iter()
        .map(|f| format!("target_feature = \"{f}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Generate x86 feature detection expression.
fn gen_x86_detect_expr(features: &[&str]) -> String {
    if features.len() == 1 {
        format!("crate::is_x86_feature_available!(\"{}\")", features[0])
    } else {
        let parts: Vec<String> = features
            .iter()
            .map(|f| format!("crate::is_x86_feature_available!(\"{f}\")"))
            .collect();
        parts.join("\n                        && ")
    }
}

/// Generate aarch64 feature detection expression.
fn gen_aarch64_detect_expr(features: &[&str]) -> String {
    if features.len() == 1 {
        format!("crate::is_aarch64_feature_available!(\"{}\")", features[0])
    } else {
        let parts: Vec<String> = features
            .iter()
            .map(|f| format!("crate::is_aarch64_feature_available!(\"{f}\")"))
            .collect();
        parts.join("\n                        && ")
    }
}

/// Generate a doc comment block from an optional doc string.
fn gen_doc_comment(doc: &Option<String>) -> String {
    match doc {
        Some(doc) => {
            let mut block = String::new();
            for line in doc.lines() {
                if line.is_empty() {
                    block.push_str("///\n");
                } else {
                    block.push_str(&format!("/// {line}\n"));
                }
            }
            block
        }
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::Registry;
    use std::path::Path;

    fn load_test_registry() -> Registry {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("token-registry.toml");
        Registry::load(&path).expect("Failed to load token-registry.toml")
    }

    #[test]
    fn generates_expected_file_set() {
        let reg = load_test_registry();
        let files = generate_token_files(&reg, 0);
        let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();

        // Must produce exactly these files
        let expected = [
            "caps.rs",
            "x86.rs",
            "arm.rs",
            "wasm.rs",
            "x86_stubs.rs",
            "arm_stubs.rs",
            "wasm_stubs.rs",
            "traits.rs",
            "mod.rs",
        ];
        for exp in &expected {
            assert!(
                names.contains(exp),
                "Missing expected file: {exp}. Got: {names:?}"
            );
        }
        assert_eq!(
            files.len(),
            expected.len(),
            "Unexpected file count. Got: {names:?}"
        );
        for (name, content) in &files {
            assert!(
                !content.contains("forge_token_dangerously"),
                "{name} must not contain removed forge_token_dangerously()"
            );
        }
        let wasm_code = &files.iter().find(|(n, _)| n == "wasm.rs").unwrap().1;
        let fma_section = wasm_code
            .split("pub struct Wasm128RelaxedFmaToken")
            .nth(1)
            .expect("wasm.rs defines Wasm128RelaxedFmaToken")
            .split("/// Type alias")
            .next()
            .unwrap();
        assert!(
            !fma_section.contains("fn from_context("),
            "Wasm128RelaxedFmaToken must not expose from_context()"
        );
    }

    #[test]
    fn x86_real_tokens_contain_all_x86_tokens() {
        let reg = load_test_registry();
        let files = generate_token_files(&reg, 0);
        let x86_code = &files.iter().find(|(n, _)| n == "x86.rs").unwrap().1;

        let x86_tokens: Vec<&str> = reg
            .token
            .iter()
            .filter(|t| t.arch == "x86")
            .map(|t| t.name.as_str())
            .collect();

        for token_name in &x86_tokens {
            assert!(
                x86_code.contains(&format!("pub struct {token_name}")),
                "x86.rs missing struct definition for {token_name}"
            );
            assert!(
                x86_code.contains(&format!("impl SimdToken for {token_name}")),
                "x86.rs missing SimdToken impl for {token_name}"
            );
        }
    }

    #[test]
    fn arm_real_tokens_contain_all_arm_tokens() {
        let reg = load_test_registry();
        let files = generate_token_files(&reg, 0);
        let arm_code = &files.iter().find(|(n, _)| n == "arm.rs").unwrap().1;

        let arm_tokens: Vec<&str> = reg
            .token
            .iter()
            .filter(|t| t.arch == "aarch64")
            .map(|t| t.name.as_str())
            .collect();

        for token_name in &arm_tokens {
            assert!(
                arm_code.contains(&format!("pub struct {token_name}")),
                "arm.rs missing struct definition for {token_name}"
            );
            assert!(
                arm_code.contains(&format!("impl SimdToken for {token_name}")),
                "arm.rs missing SimdToken impl for {token_name}"
            );
        }
    }

    #[test]
    fn stubs_return_none_for_summon() {
        let reg = load_test_registry();
        let files = generate_token_files(&reg, 0);

        for (name, code) in &files {
            if name.ends_with("_stubs.rs") {
                // Every stub's summon must return None
                assert!(
                    !code.contains("Some(Self"),
                    "{name} contains Some(Self — stubs must only return None"
                );
            }
        }
    }

    #[test]
    fn traits_file_contains_all_traits() {
        let reg = load_test_registry();
        let files = generate_token_files(&reg, 0);
        let traits_code = &files.iter().find(|(n, _)| n == "traits.rs").unwrap().1;

        for trait_def in &reg.traits {
            assert!(
                traits_code.contains(&format!("pub trait {}", trait_def.name)),
                "traits.rs missing trait: {}",
                trait_def.name
            );
        }
    }

    #[test]
    fn real_tokens_have_summon_and_compiled_with() {
        let reg = load_test_registry();
        let files = generate_token_files(&reg, 0);

        for (name, code) in &files {
            if name == "x86.rs" || name == "arm.rs" || name == "wasm.rs" {
                // Every real token file must implement summon() and compiled_with()
                let arch_tokens: Vec<&str> = reg
                    .token
                    .iter()
                    .filter(|t| match name.as_str() {
                        "x86.rs" => t.arch == "x86",
                        "arm.rs" => t.arch == "aarch64",
                        "wasm.rs" => t.arch == "wasm",
                        _ => false,
                    })
                    .map(|t| t.name.as_str())
                    .collect();

                for token_name in &arch_tokens {
                    assert!(
                        code.contains("fn summon()"),
                        "{name} missing summon() for {token_name}"
                    );
                    assert!(
                        code.contains("fn compiled_with()"),
                        "{name} missing compiled_with() for {token_name}"
                    );
                }
            }
        }
    }

    #[test]
    fn parent_extraction_methods_generated() {
        let reg = load_test_registry();
        let files = generate_token_files(&reg, 0);
        let x86_code = &files.iter().find(|(n, _)| n == "x86.rs").unwrap().1;

        // X64V3Token has parent X64V2Token — should generate extraction method
        let v3 = reg.find_token("X64V3Token").unwrap();
        assert!(
            !v3.parents.is_empty(),
            "X64V3Token should have parents for this test"
        );
        // The short_name of the parent becomes the extraction method
        let v2 = reg.find_token("X64V2Token").unwrap();
        let short = v2.short_name.as_deref().expect("X64V2Token has short_name");
        assert!(
            x86_code.contains(&format!("pub fn {short}(self)")),
            "x86.rs missing parent extraction method {short}() on X64V3Token",
        );
    }

    #[test]
    fn cache_variable_names_are_unique() {
        let reg = load_test_registry();

        let mut cache_names = std::collections::HashSet::new();
        for token in &reg.token {
            if token.arch == "any" {
                continue;
            }
            let name = cache_var_name(&token.name);
            assert!(
                cache_names.insert(name.clone()),
                "Duplicate cache variable: {name} (from {})",
                token.name
            );
        }
    }

    #[test]
    fn mod_rs_routes_all_architectures() {
        let reg = load_test_registry();
        let files = generate_token_files(&reg, 0);
        let mod_code = &files.iter().find(|(n, _)| n == "mod.rs").unwrap().1;

        // Must have cfg routing for all architectures
        assert!(
            mod_code.contains("target_arch = \"x86_64\""),
            "mod.rs missing x86_64 routing"
        );
        assert!(
            mod_code.contains("target_arch = \"aarch64\""),
            "mod.rs missing aarch64 routing"
        );
        assert!(
            mod_code.contains("target_arch = \"wasm32\""),
            "mod.rs missing wasm32 routing"
        );
    }
}
