//! Token registry — loads and validates `token-registry.toml`.
//!
//! This is the single source of truth for all token definitions, feature
//! sets, traits, width namespaces, magetypes file mappings, and polyfill
//! configurations.

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::{BTreeSet, HashSet};
use std::path::Path;

// ============================================================================
// Serde Structs
// ============================================================================

/// Top-level registry file.
#[derive(Debug, Deserialize)]
pub struct Registry {
    #[serde(default)]
    pub cap_slot: Vec<CapSlotDef>,
    pub token: Vec<TokenDef>,
    #[serde(rename = "trait")]
    pub traits: Vec<TraitDef>,
    pub width_namespace: Vec<WidthNamespace>,
    #[serde(default)]
    pub polyfill_w256: Vec<PolyfillW256>,
    #[serde(default)]
    pub polyfill_w512: Vec<PolyfillW512>,
}

/// An atomic capability slot for an architecture's `*Caps` trait.
#[derive(Debug, Deserialize)]
pub struct CapSlotDef {
    pub arch: String,
    pub name: String,
    pub features: Vec<String>,
    /// True when compile-time target features alone do not imply this slot
    /// (requires a runtime semantic contract via `extra_caps`).
    #[serde(default)]
    pub explicit_only: bool,
}

/// A token definition.
#[derive(Debug, Deserialize)]
pub struct TokenDef {
    pub name: String,
    pub arch: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub features: Vec<String>,
    /// Extra capability slots enabled by runtime contract (e.g. `"RelaxedFma"`).
    #[serde(default)]
    pub extra_caps: Vec<String>,
    pub traits: Vec<String>,
    #[serde(default)]
    pub cargo_feature: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub always_available: bool,
    /// SimdToken name value (human-readable).
    #[serde(default)]
    pub display_name: Option<String>,
    /// Extraction method name (e.g., "v2", "v3", "neon").
    #[serde(default)]
    pub short_name: Option<String>,
    /// Explicit dispatch ordering and exact-token extraction for this tier.
    #[serde(default)]
    pub dispatch_priority: Option<u32>,
    #[serde(default)]
    pub dispatch_as: Option<String>,
    /// Compatibility policy for legacy default tier sets only.
    #[serde(default)]
    pub legacy_dispatch_gate: Option<String>,
    /// Parent tokens in the hierarchy (for extraction method chain, DAG).
    #[serde(default)]
    pub parents: Vec<String>,
    /// Extra extraction method names (e.g., ["avx512"] for X64V4Token).
    #[serde(default)]
    pub extraction_aliases: Vec<String>,
    /// Doc comment for the struct.
    #[serde(default)]
    pub doc: Option<String>,
    /// Magetypes width namespace for this token (e.g., "v3", "v4", "neon").
    /// Used by `import_magetypes` parameter in `#[arcane]`/`#[rite]`.
    #[serde(default)]
    pub magetypes_namespace: Option<String>,
    /// True when availability requires a runtime semantic probe beyond
    /// compile-time `#[target_feature]` validation (e.g. `Wasm128RelaxedFmaToken`
    /// checking single-rounding fusion of `f32x4`/`f64x2` `relaxed_madd`).
    /// Omits `from_context()` completely and blocks static target-feature
    /// promotion without an explicit runtime proof.
    #[serde(default)]
    pub runtime_probe: bool,
    /// True when `magetypes` implements the full SIMD backend trait suite
    /// directly for this token (either natively or via parent delegation).
    #[serde(default)]
    pub has_magetypes_backend: bool,
}

/// A trait definition.
#[derive(Debug, Deserialize)]
pub struct TraitDef {
    pub name: String,
    /// Target architecture (e.g., "x86_64", "aarch64", "wasm32").
    /// Used by `import_intrinsics` to determine which `core::arch::` module to import.
    #[serde(default)]
    pub arch: Option<String>,
    /// Features required by this trait.
    #[serde(default)]
    pub features: Vec<String>,
    /// Capability slot names required by this trait's single blanket impl.
    #[serde(default)]
    pub cap_req: Vec<String>,
    /// True if this trait requires a runtime semantic contract (cannot be forged
    /// from compile-time `#[target_feature]` alone).
    #[serde(default)]
    pub runtime_probe: bool,
    #[serde(default)]
    pub parents: Vec<String>,
    /// Doc comment for the trait.
    #[serde(default)]
    pub doc: Option<String>,
    /// Magetypes width namespace for this trait (e.g., "v3", "neon").
    /// Used by `import_magetypes` parameter in `#[arcane]`/`#[rite]`.
    #[serde(default)]
    pub magetypes_namespace: Option<String>,
}

/// A width namespace for simd type re-exports.
#[derive(Debug, Deserialize)]
pub struct WidthNamespace {
    pub name: String,
    #[allow(dead_code)]
    pub arch: String,
    #[allow(dead_code)]
    pub width: u32,
    pub token: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub cargo_feature: Option<String>,
}

/// A polyfill_w256 platform configuration.
#[derive(Debug, Deserialize)]
pub struct PolyfillW256 {
    pub mod_name: String,
    #[allow(dead_code)]
    pub cfg: String,
    pub token: String,
    #[allow(dead_code)]
    pub w128_import: String,
}

/// A polyfill_w512 platform configuration.
#[derive(Debug, Deserialize)]
pub struct PolyfillW512 {
    pub mod_name: String,
    #[allow(dead_code)]
    pub cfg: String,
    pub token: String,
    #[allow(dead_code)]
    pub w256_import: String,
}

// ============================================================================
// Loading
// ============================================================================

impl Registry {
    /// Load and validate the registry from a TOML file.
    pub fn load(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let registry: Registry =
            toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;
        registry.validate()?;
        Ok(registry)
    }

    /// Look up a token by name (including aliases).
    pub fn find_token(&self, name: &str) -> Option<&TokenDef> {
        self.token
            .iter()
            .find(|t| t.name == name || t.aliases.iter().any(|a| a == name))
    }

    /// Get the feature set for a token or trait by name.
    ///
    /// For tokens, returns the token's features. For traits, returns the
    /// trait's features.
    #[allow(dead_code)] // exercised by tests; kept as registry API
    pub fn features_for(&self, name: &str) -> Option<Vec<&str>> {
        // Try tokens first (including aliases)
        if let Some(token) = self.find_token(name) {
            return Some(token.features.iter().map(|s| s.as_str()).collect());
        }
        // Try traits
        if let Some(trait_def) = self.traits.iter().find(|t| t.name == name)
            && !trait_def.features.is_empty()
        {
            return Some(trait_def.features.iter().map(|s| s.as_str()).collect());
        }
        None
    }

    /// Normalize trait arch (`"x86_64"`, `"wasm32"`) to slot/token arch (`"x86"`, `"wasm"`).
    pub fn normalize_arch(arch: &str) -> &str {
        match arch {
            "x86_64" => "x86",
            "wasm32" => "wasm",
            other => other,
        }
    }

    /// All capability slots defined for `arch` (accepts both `"x86"`/`"x86_64"` and `"wasm"`/`"wasm32"`).
    pub fn cap_slots_for_arch(&self, arch: &str) -> Vec<&CapSlotDef> {
        let norm = Self::normalize_arch(arch);
        self.cap_slot.iter().filter(|s| s.arch == norm).collect()
    }

    /// Capability slot names enabled by `token`.
    pub fn token_cap_slots(&self, token: &TokenDef) -> BTreeSet<&str> {
        let token_features: BTreeSet<&str> = token.features.iter().map(|s| s.as_str()).collect();
        self.cap_slots_for_arch(&token.arch)
            .into_iter()
            .filter(|slot| {
                token.extra_caps.contains(&slot.name)
                    || (!slot.explicit_only
                        && slot
                            .features
                            .iter()
                            .all(|f| token_features.contains(f.as_str())))
            })
            .map(|slot| slot.name.as_str())
            .collect()
    }

    /// All token names including aliases.
    #[allow(dead_code)]
    pub fn all_token_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        for t in &self.token {
            names.push(t.name.as_str());
            for a in &t.aliases {
                names.push(a.as_str());
            }
        }
        names
    }

    /// All trait names.
    #[allow(dead_code)]
    pub fn all_trait_names(&self) -> Vec<&str> {
        self.traits.iter().map(|t| t.name.as_str()).collect()
    }

    /// BFS through parent DAG and collect all ancestors (deduplicated, stable order).
    pub fn collect_ancestors<'a>(&'a self, token: &'a TokenDef) -> Vec<&'a TokenDef> {
        let mut ancestors = Vec::new();
        let mut visited = HashSet::new();
        let mut queue: std::collections::VecDeque<&str> =
            token.parents.iter().map(|s| s.as_str()).collect();
        while let Some(name) = queue.pop_front() {
            if !visited.insert(name) {
                continue;
            }
            if let Some(parent) = self.token.iter().find(|t| t.name == name) {
                ancestors.push(parent);
                for gp in &parent.parents {
                    queue.push_back(gp.as_str());
                }
            }
        }
        ancestors
    }
}

// ============================================================================
// Validation
// ============================================================================

impl Registry {
    fn validate(&self) -> Result<()> {
        self.validate_no_duplicate_names()?;
        for token in &self.token {
            if token.dispatch_priority.is_some() != token.dispatch_as.is_some() {
                bail!(
                    "{} needs both dispatch_priority and dispatch_as",
                    token.name
                );
            }
            if token.dispatch_priority.is_some() && token.short_name.is_none() {
                bail!("{} needs a short_name for dispatch", token.name);
            }
        }
        self.validate_cap_slots()?;
        self.validate_trait_references()?;
        self.validate_token_trait_features()?;
        self.validate_width_namespace_tokens()?;
        self.validate_polyfill_tokens()?;
        self.validate_token_parents()?;
        Ok(())
    }

    fn validate_cap_slots(&self) -> Result<()> {
        let mut seen = HashSet::new();
        for slot in &self.cap_slot {
            if !matches!(slot.arch.as_str(), "x86" | "aarch64" | "wasm") {
                bail!("cap_slot {} has invalid arch '{}'", slot.name, slot.arch);
            }
            if !seen.insert((slot.arch.as_str(), slot.name.as_str())) {
                bail!(
                    "Duplicate cap_slot '{}' for arch '{}'",
                    slot.name,
                    slot.arch
                );
            }
        }

        // Validate extra_caps on tokens
        for token in &self.token {
            let valid_slots: HashSet<&str> = self
                .cap_slots_for_arch(&token.arch)
                .into_iter()
                .map(|s| s.name.as_str())
                .collect();
            for cap in &token.extra_caps {
                if !valid_slots.contains(cap.as_str()) {
                    bail!(
                        "Token {} references unknown cap slot '{}' for arch '{}'",
                        token.name,
                        cap,
                        token.arch
                    );
                }
            }
        }

        // Validate traits' cap_req, parent cap_reqs, and feature consistency
        for trait_def in &self.traits {
            let arch = trait_def
                .arch
                .as_deref()
                .with_context(|| format!("Trait {} must specify arch", trait_def.name))?;
            let slots = self.cap_slots_for_arch(arch);
            let slot_map: std::collections::HashMap<&str, &CapSlotDef> =
                slots.into_iter().map(|s| (s.name.as_str(), s)).collect();

            let mut union_features: BTreeSet<&str> = BTreeSet::new();
            let mut has_explicit = false;
            for req in &trait_def.cap_req {
                let slot = slot_map.get(req.as_str()).with_context(|| {
                    format!(
                        "Trait {} requires unknown cap slot '{}' for arch '{}'",
                        trait_def.name, req, arch
                    )
                })?;
                if slot.explicit_only {
                    has_explicit = true;
                }
                for f in &slot.features {
                    union_features.insert(f.as_str());
                }
            }
            if has_explicit && !trait_def.runtime_probe {
                bail!(
                    "Trait {} requires an explicit_only cap slot and must set runtime_probe = true",
                    trait_def.name
                );
            }
            let declared_features: BTreeSet<&str> =
                trait_def.features.iter().map(|s| s.as_str()).collect();
            if union_features != declared_features {
                bail!(
                    "Trait {} features {:?} do not match union of cap_req slots {:?}",
                    trait_def.name,
                    declared_features,
                    union_features
                );
            }

            // Every parent trait's cap_req must be a subset of this trait's cap_req
            let my_caps: BTreeSet<&str> = trait_def.cap_req.iter().map(|s| s.as_str()).collect();
            for parent_name in &trait_def.parents {
                if let Some(parent) = self.traits.iter().find(|t| t.name == *parent_name) {
                    for p_cap in &parent.cap_req {
                        if !my_caps.contains(p_cap.as_str()) {
                            bail!(
                                "Trait {} inherits from {} but misses cap_req '{}'",
                                trait_def.name,
                                parent_name,
                                p_cap
                            );
                        }
                    }
                }
            }
        }

        // Verify token.traits equals exact set of arch traits whose cap_req <= token_cap_slots
        for token in &self.token {
            let token_caps = self.token_cap_slots(token);
            let declared_traits: BTreeSet<&str> = token.traits.iter().map(|s| s.as_str()).collect();
            let mut expected_traits: BTreeSet<&str> = BTreeSet::new();
            for trait_def in &self.traits {
                let trait_arch = trait_def
                    .arch
                    .as_deref()
                    .map(Self::normalize_arch)
                    .unwrap_or("");
                if trait_arch == token.arch
                    && trait_def
                        .cap_req
                        .iter()
                        .all(|req| token_caps.contains(req.as_str()))
                {
                    expected_traits.insert(trait_def.name.as_str());
                }
            }
            if declared_traits != expected_traits {
                bail!(
                    "Token {} declared traits {:?} != cap-slot implied traits {:?}",
                    token.name,
                    declared_traits,
                    expected_traits
                );
            }
        }

        Ok(())
    }

    fn validate_no_duplicate_names(&self) -> Result<()> {
        let mut seen = HashSet::new();
        for t in &self.token {
            if !seen.insert(&t.name) {
                bail!("Duplicate token name: {}", t.name);
            }
            for a in &t.aliases {
                if !seen.insert(a) {
                    bail!("Duplicate token alias: {} (on {})", a, t.name);
                }
            }
        }
        let mut trait_seen = HashSet::new();
        for t in &self.traits {
            if !trait_seen.insert(&t.name) {
                bail!("Duplicate trait name: {}", t.name);
            }
        }
        Ok(())
    }

    fn validate_trait_references(&self) -> Result<()> {
        let trait_names: HashSet<&str> = self.traits.iter().map(|t| t.name.as_str()).collect();

        // Tokens reference valid traits
        for token in &self.token {
            for trait_name in &token.traits {
                if !trait_names.contains(trait_name.as_str()) {
                    bail!(
                        "Token {} references unknown trait: {}",
                        token.name,
                        trait_name
                    );
                }
            }
        }

        // Trait parents reference valid traits
        for trait_def in &self.traits {
            for parent in &trait_def.parents {
                if !trait_names.contains(parent.as_str()) {
                    bail!(
                        "Trait {} references unknown parent: {}",
                        trait_def.name,
                        parent
                    );
                }
            }
        }
        Ok(())
    }

    fn validate_token_trait_features(&self) -> Result<()> {
        // For each token, verify its features are a superset of each claimed trait's features
        for token in &self.token {
            let token_features: BTreeSet<&str> =
                token.features.iter().map(|s| s.as_str()).collect();

            for trait_name in &token.traits {
                if let Some(trait_def) = self.traits.iter().find(|t| t.name == *trait_name) {
                    let trait_features: Vec<&str> = if !trait_def.features.is_empty() {
                        trait_def.features.iter().map(|s| s.as_str()).collect()
                    } else {
                        continue;
                    };

                    for f in &trait_features {
                        if !token_features.contains(f) {
                            bail!(
                                "Token {} claims trait {} but is missing feature '{}'",
                                token.name,
                                trait_name,
                                f
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_width_namespace_tokens(&self) -> Result<()> {
        for ns in &self.width_namespace {
            if self.find_token(&ns.token).is_none() {
                bail!(
                    "Width namespace '{}' references unknown token: {}",
                    ns.name,
                    ns.token
                );
            }
        }
        Ok(())
    }

    fn validate_polyfill_tokens(&self) -> Result<()> {
        for p in &self.polyfill_w256 {
            if self.find_token(&p.token).is_none() {
                bail!(
                    "Polyfill w256 '{}' references unknown token: {}",
                    p.mod_name,
                    p.token
                );
            }
        }
        for p in &self.polyfill_w512 {
            if self.find_token(&p.token).is_none() {
                bail!(
                    "Polyfill w512 '{}' references unknown token: {}",
                    p.mod_name,
                    p.token
                );
            }
        }
        Ok(())
    }

    fn validate_token_parents(&self) -> Result<()> {
        let token_names: HashSet<&str> = self.token.iter().map(|t| t.name.as_str()).collect();

        for token in &self.token {
            for parent in &token.parents {
                if !token_names.contains(parent.as_str()) {
                    bail!("Token {} references unknown parent: {}", token.name, parent);
                }
                // Parent must be same arch
                if let Some(parent_def) = self.token.iter().find(|t| t.name == *parent)
                    && parent_def.arch != token.arch
                {
                    bail!(
                        "Token {} (arch={}) has parent {} (arch={}) — must be same arch",
                        token.name,
                        token.arch,
                        parent,
                        parent_def.arch
                    );
                }
            }

            // Cycle detection: BFS from this token through parents must not revisit itself
            let mut visited = HashSet::new();
            let mut queue: std::collections::VecDeque<&str> =
                token.parents.iter().map(|s| s.as_str()).collect();
            while let Some(name) = queue.pop_front() {
                if name == token.name {
                    bail!("Token {} has a cycle in its parent hierarchy", token.name);
                }
                if !visited.insert(name) {
                    continue;
                }
                if let Some(ancestor) = self.token.iter().find(|t| t.name == name) {
                    for gp in &ancestor.parents {
                        queue.push_back(gp.as_str());
                    }
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Display
// ============================================================================

impl std::fmt::Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Token Registry:")?;
        writeln!(f, "  Tokens: {}", self.token.len())?;
        for t in &self.token {
            let aliases = if t.aliases.is_empty() {
                String::new()
            } else {
                format!(" (aliases: {})", t.aliases.join(", "))
            };
            writeln!(
                f,
                "    {} [{}] — {} features, {} traits{}",
                t.name,
                t.arch,
                t.features.len(),
                t.traits.len(),
                aliases,
            )?;
        }
        writeln!(f, "  Traits: {}", self.traits.len())?;
        for t in &self.traits {
            writeln!(f, "    {}", t.name)?;
        }
        writeln!(f, "  Width namespaces: {}", self.width_namespace.len())?;
        writeln!(
            f,
            "  Polyfill platforms: {} w256 + {} w512",
            self.polyfill_w256.len(),
            self.polyfill_w512.len()
        )?;
        Ok(())
    }
}

// ============================================================================
// Tier Tags
// ============================================================================

/// FNV-1a hash of token name seeded with major version.
///
/// Produces a unique tag for each token struct name, used for compile-time
/// assertion that a concrete token type is genuinely the expected archmage
/// type (not shadowed or aliased).
pub fn tier_tag(token_name: &str, major_version: u32) -> u32 {
    let mut hash: u32 = 0x811c_9dc5 ^ major_version.wrapping_mul(0x0100_0193);
    for byte in token_name.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

// ============================================================================
// Code Generation
// ============================================================================

mod macros;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_registry() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("token-registry.toml");
        let registry = Registry::load(&path).expect("Failed to load token-registry.toml");

        // Basic counts
        assert_eq!(registry.cap_slot.len(), 33, "Expected 33 cap slots");
        assert_eq!(registry.token.len(), 38, "Expected 38 tokens");
        assert_eq!(registry.traits.len(), 38, "Expected 38 traits");
        assert_eq!(
            registry.width_namespace.len(),
            4,
            "Expected 4 width namespaces"
        );
        // Spot-check Wasm128RelaxedFmaToken
        let wasm_fma = registry
            .find_token("Wasm128RelaxedFmaToken")
            .expect("Wasm128RelaxedFmaToken not found");
        assert!(wasm_fma.runtime_probe);
        assert!(wasm_fma.has_magetypes_backend);
        assert_eq!(wasm_fma.parents, vec!["Wasm128RelaxedToken"]);
        // Spot-check X64V3Token
        let v3 = registry
            .find_token("X64V3Token")
            .expect("X64V3Token not found");
        assert!(v3.features.contains(&"avx2".to_string()));
        assert!(v3.features.contains(&"fma".to_string()));
        assert!(v3.features.contains(&"f16c".to_string()));
        assert!(v3.features.contains(&"lzcnt".to_string()));
        assert_eq!(v3.features.len(), 16);

        let v3_gfni_crypto = registry
            .find_token("X64V3GfniCryptoToken")
            .expect("X64V3GfniCryptoToken not found");
        assert!(v3_gfni_crypto.features.contains(&"gfni".to_string()));
        assert!(
            v3_gfni_crypto
                .parents
                .contains(&"X64V3CryptoToken".to_string())
        );
        assert!(v3_gfni_crypto.parents.contains(&"X64GfniToken".to_string()));

        let v3_vnni = registry
            .find_token("X64V3VnniToken")
            .expect("X64V3VnniToken not found");
        assert!(v3_vnni.features.contains(&"avxvnni".to_string()));
        assert!(v3_vnni.features.contains(&"adx".to_string()));
        assert!(v3_vnni.features.contains(&"sha".to_string()));
        assert!(
            v3_vnni
                .parents
                .contains(&"X64V3GfniCryptoToken".to_string())
        );

        let v3_vnni2 = registry
            .find_token("X64V3Vnni2Token")
            .expect("X64V3Vnni2Token not found");
        assert!(v3_vnni2.features.contains(&"avxvnniint8".to_string()));
        assert!(v3_vnni2.features.contains(&"avxneconvert".to_string()));
        assert!(v3_vnni2.features.contains(&"avxifma".to_string()));
        assert!(v3_vnni2.features.contains(&"adx".to_string()));
        assert!(v3_vnni2.features.contains(&"sha".to_string()));
        assert!(v3_vnni2.parents.contains(&"X64V3VnniToken".to_string()));

        // Orthogonal clumps
        let sha512 = registry
            .find_token("X64Sha512Token")
            .expect("X64Sha512Token");
        assert!(sha512.features.contains(&"avx".to_string()));
        assert!(sha512.features.contains(&"sha512".to_string()));
        let bf16 = registry
            .find_token("X64Avx512Bf16Token")
            .expect("X64Avx512Bf16Token");
        assert_eq!(bf16.parents, vec!["X64V4Token"]);
        let i8mm = registry.find_token("NeonI8mmToken").expect("NeonI8mmToken");
        assert_eq!(i8mm.features, vec!["neon", "i8mm"]);

        // Non-deprecated aliases kept cleanly in 0.10
        assert!(registry.find_token("Desktop64").is_some());
        assert!(registry.find_token("Avx2FmaToken").is_none());
        assert!(registry.find_token("Sse2Token").is_none());
        assert!(registry.find_token("Arm64").is_some());
        assert!(registry.find_token("Server64").is_some());

        // Spot-check NeonCrcToken
        let crc = registry
            .find_token("NeonCrcToken")
            .expect("NeonCrcToken not found");
        assert_eq!(crc.features, vec!["neon", "crc"]);
    }

    fn load_test_registry() -> Registry {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("token-registry.toml");
        Registry::load(&path).expect("Failed to load token-registry.toml")
    }

    #[test]
    fn macro_registry_contains_all_tokens() {
        let registry = load_test_registry();
        let output = registry.generate_macro_registry(0);

        // Every token should appear in the generated registry
        for token in &registry.token {
            assert!(
                output.contains(&token.name),
                "Token {} missing from macro registry output",
                token.name
            );
            // Every alias should also appear
            for alias in &token.aliases {
                assert!(
                    output.contains(alias),
                    "Alias {} for {} missing from macro registry",
                    alias,
                    token.name
                );
            }
        }
    }

    #[test]
    fn macro_registry_contains_features() {
        let registry = load_test_registry();
        let output = registry.generate_macro_registry(0);

        // Key features should appear in the output
        for feature in &["avx2", "fma", "neon", "simd128"] {
            assert!(
                output.contains(feature),
                "Feature {feature} missing from macro registry output",
            );
        }
    }

    #[test]
    fn token_hierarchy_is_consistent() {
        let registry = load_test_registry();

        // Every parent must exist
        for token in &registry.token {
            for parent_name in &token.parents {
                assert!(
                    registry.find_token(parent_name).is_some(),
                    "Token {} declares parent {} which doesn't exist",
                    token.name,
                    parent_name
                );
            }
        }

        // Every child's features must be a superset of parent's features
        for token in &registry.token {
            for parent_name in &token.parents {
                if let Some(parent) = registry.find_token(parent_name) {
                    for parent_feature in &parent.features {
                        assert!(
                            token.features.contains(parent_feature),
                            "Token {} missing parent feature {} from {}",
                            token.name,
                            parent_feature,
                            parent_name
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn all_traits_referenced_by_tokens_exist() {
        let registry = load_test_registry();
        let trait_names: Vec<&str> = registry.traits.iter().map(|t| t.name.as_str()).collect();

        for token in &registry.token {
            for trait_name in &token.traits {
                assert!(
                    trait_names.contains(&trait_name.as_str()),
                    "Token {} references trait {} which isn't defined",
                    token.name,
                    trait_name
                );
            }
        }
    }

    #[test]
    fn features_for_returns_correct_data() {
        let registry = load_test_registry();

        // X64V3Token should include inherited features
        let v3_features = registry.features_for("X64V3Token").unwrap();
        assert!(v3_features.contains(&"avx2"));
        assert!(v3_features.contains(&"fma"));
        assert!(v3_features.contains(&"sse2")); // inherited from V1

        // Alias should work too
        let desktop = registry.features_for("Desktop64").unwrap();
        assert_eq!(v3_features, desktop);

        // Nonexistent token
        assert!(registry.features_for("FakeToken").is_none());
    }

    #[test]
    fn arch_grouping_covers_all_tokens() {
        let registry = load_test_registry();
        let valid_arches = ["x86", "aarch64", "wasm"];

        for token in &registry.token {
            if token.name == "ScalarToken" {
                continue; // ScalarToken has arch "any"
            }
            assert!(
                valid_arches.contains(&token.arch.as_str()) || token.arch == "any",
                "Token {} has unexpected arch: {}",
                token.name,
                token.arch
            );
        }
    }
}
