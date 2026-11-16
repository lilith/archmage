//! A feature-enabled function, independent of attribute spelling.
use super::inline::InlinePolicy;
use crate::common::*;
use crate::generated::{
    canonical_token_to_tier_suffix, token_requires_runtime_probe, token_to_arch, token_to_features,
    token_to_magetypes_namespace,
};
use crate::rite::DEFAULT_TIER_SENTINEL;
use crate::token_discovery::{SignatureProofs, discover_signature_proofs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, parse_quote};

/// Where a `#[rite]` variant gets its features from.
pub(crate) struct FeatureContext {
    /// Canonical token name (`X64V3Token`), or `None` for conjunctions/traits/default.
    pub(crate) token: Option<String>,
    /// The tier suffix (`v3`, `scalar`, `default`), when the tier is a single
    /// registry tier; it selects the nested `incant!` rewrite.
    pub(crate) suffix: Option<&'static str>,
    pub(crate) features: std::borrow::Cow<'static, [&'static str]>,
    pub(crate) target_arch: Option<&'static str>,
    pub(crate) magetypes_namespace: Option<&'static str>,
    /// The primary token parameter ident when deriving context from a signature.
    pub(crate) token_ident: Option<Ident>,
    /// Full signature proof metadata when derived from signature discovery.
    pub(crate) sig_proofs: Option<SignatureProofs>,
}

impl FeatureContext {
    /// A tier named in the attribute: `#[rite(v3)]`, or `default`.
    pub(crate) fn from_tier_token(tier_token: &str) -> Option<Self> {
        if tier_token == DEFAULT_TIER_SENTINEL {
            return Some(FeatureContext {
                token: None,
                suffix: Some("default"),
                features: std::borrow::Cow::Borrowed(&[]),
                target_arch: None,
                magetypes_namespace: None,
                token_ident: None,
                sig_proofs: None,
            });
        }
        Some(FeatureContext {
            token: Some(tier_token.to_string()),
            suffix: canonical_token_to_tier_suffix(tier_token),
            features: std::borrow::Cow::Borrowed(token_to_features(tier_token)?),
            target_arch: token_to_arch(tier_token),
            magetypes_namespace: token_to_magetypes_namespace(tier_token),
            token_ident: None,
            sig_proofs: None,
        })
    }

    /// Derive feature context from discovered signature proofs (concrete, `And`,
    /// tuple, multi-parameter conjunction, `impl Trait`, or bounded generic).
    pub(crate) fn from_signature_proofs(proofs: SignatureProofs) -> Self {
        let token = proofs.single_concrete_name().map(str::to_string);
        let suffix = proofs.single_tier_suffix();
        let features = std::borrow::Cow::Owned(proofs.features.clone());
        let target_arch = proofs.target_arch;
        let magetypes_namespace = proofs.magetypes_namespace;
        let token_ident = Some(proofs.primary_ident.clone());
        FeatureContext {
            token,
            suffix,
            features,
            target_arch,
            magetypes_namespace,
            token_ident,
            sig_proofs: Some(proofs),
        }
    }
}

fn signature_satisfies_runtime_probe(proofs: &SignatureProofs, required_token: &str) -> bool {
    if proofs
        .all_concrete_names
        .iter()
        .any(|name| name == required_token)
    {
        return true;
    }
    if required_token == "Wasm128RelaxedFmaToken"
        && proofs
            .all_trait_names
            .iter()
            .any(|name| name == "HasWasm128RelaxedFma")
    {
        return true;
    }
    false
}

/// Reject safe tokenless/wrong-token `#[attune]`/`#[rite]` functions on WASM
/// tiers whose contract requires a runtime probe beyond `#[target_feature]`.
pub(crate) fn runtime_probe_direct_error(
    sig: &syn::Signature,
    tier: &FeatureContext,
) -> Option<syn::Error> {
    if tier.target_arch != Some("wasm32") || matches!(sig.safety, syn::Safety::Unsafe(_)) {
        return None;
    }
    let token_name = tier.token.as_deref()?;
    if !token_requires_runtime_probe(token_name) {
        return None;
    }
    let has_matching_token = match discover_signature_proofs(sig) {
        Ok(Some(ref proofs)) => signature_satisfies_runtime_probe(proofs, token_name),
        Ok(None) => false,
        Err(err) => return Some(err),
    };
    if has_matching_token {
        return None;
    }
    let suffix = tier.suffix.unwrap_or(token_name);
    Some(syn::Error::new_spanned(
        &sig.ident,
        format!(
            "`{suffix}` requires a runtime fusion probe beyond compile-time `#[target_feature]`: \
             take a `{token_name}` parameter (e.g., `#[attune(_*_t, +{suffix})]` or `fn {}(_: {token_name})`) \
             or mark the function `unsafe fn`",
            sig.ident
        ),
    ))
}

/// Emit one `#[target_feature]` + `#[inline]` function for `tier`, with
/// imports, nested `incant!` rewriting, all proof assertions, and the cfg guard.
/// Both the single- and multi-tier forms go through here.
pub(crate) fn emit(
    mut variant_fn: syn::ItemFn,
    options: &SharedOptions,
    inline: Option<InlinePolicy>,
    preserve_inline: bool,
    tier: FeatureContext,
) -> Result<TokenStream, TokenStream> {
    if let Err(err) = validate_item_fn(&variant_fn, "rite") {
        return Err(err.to_compile_error());
    }
    if let Some(err) = runtime_probe_direct_error(&variant_fn.sig, &tier) {
        return Err(err.to_compile_error());
    }
    let token_desc = tier.token.as_deref().unwrap_or("an AVX-512 token");
    if let Some(err) = avx512_import_error(
        &variant_fn.sig,
        options.import_intrinsics,
        &tier.features,
        token_desc,
    ) {
        return Err(err);
    }

    // Always inspect signature proofs, even in explicit-tier mode (`#[rite(v3)]`),
    // so signature cross-arch mixes fail closed and token/trait/`And` assertions
    // are emitted unconditionally. (An explicit tier may still sit alongside a
    // single fallback/parent token parameter such as `#[rite(v3, scalar)] fn f(t: ScalarToken)`
    // or `#[attune(scalar)] fn parent(t: X64V3Token)`.)
    let explicit_tier_mode = tier.token_ident.is_none();
    let sig_proofs = match tier.sig_proofs {
        Some(proofs) => Some(proofs),
        None => match discover_signature_proofs(&variant_fn.sig) {
            Ok(Some(proofs)) => {
                if tier.target_arch == Some("wasm32")
                    && !matches!(variant_fn.sig.safety, syn::Safety::Unsafe(_))
                    && proofs.requires_runtime_probe()
                    && tier.token.as_deref().is_some_and(|tok| {
                        token_requires_runtime_probe(tok)
                            && !signature_satisfies_runtime_probe(&proofs, tok)
                    })
                {
                    return Err(syn::Error::new_spanned(
                        &variant_fn.sig,
                        "signature mixes wasm32 runtime-probed proof constraints inconsistently",
                    )
                    .to_compile_error());
                }
                Some(proofs)
            }
            Ok(None) => None,
            Err(err) => return Err(err.to_compile_error()),
        },
    };

    // Normalize non-plain token parameters (wildcards, tuples, `mut`, `ref`, `@`)
    // so bound assertions can borrow/project them before rebinding for the body.
    let mut active_token_ident = tier.token_ident.clone();
    let rebinds = if let Some(ref proofs) = sig_proofs {
        let rebinds =
            normalize_single_fn_token_params(&mut variant_fn.sig, &proofs.token_param_indices);
        if active_token_ident.is_none() {
            active_token_ident = Some(proofs.primary_ident.clone());
        }
        // Refresh wildcard ident after normalization (`_: X64V3Token` -> `__archmage_token`).
        if let Some(&first_idx) = proofs.token_param_indices.first()
            && let Some(ident) = variant_fn
                .sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Typed(pt) => Some(pt),
                    syn::FnArg::Receiver(_) => None,
                })
                .nth(first_idx)
                .and_then(|pt| match pt.pat.as_ref() {
                    syn::Pat::Ident(pi) => Some(pi.ident.clone()),
                    _ => None,
                })
        {
            active_token_ident = Some(ident);
        }
        rebinds
    } else {
        Vec::new()
    };

    // Rewrite incant!() calls in the body to direct tier calls.
    if let Some(tier_suffix) = tier.suffix
        && let Some(resolved) = crate::tiers::find_tier(tier_suffix)
    {
        let _ = explicit_tier_mode;
        let ctx = match &active_token_ident {
            Some(ident)
                if sig_proofs
                    .as_ref()
                    .is_some_and(|p| p.single_tier_suffix().is_some()) =>
            {
                crate::rewrite::CallerContext {
                    tier_suffix,
                    target_arch: resolved.target_arch,
                    token_ident: ident.clone(),
                    has_token: true,
                    derive_token: false,
                }
            }
            _ => crate::rewrite::CallerContext {
                tier_suffix,
                target_arch: resolved.target_arch,
                token_ident: quote::format_ident!("_"),
                has_token: false,
                derive_token: true,
            },
        };
        let rewritten = crate::rewrite::rewrite_incant_in_function(
            block_inner_tokens(&variant_fn.block),
            &ctx,
            Some(&variant_fn.sig),
        );
        if let Err(err) = set_block_inner_tokens(&mut variant_fn.block, rewritten) {
            return Err(err.to_compile_error());
        }
    }

    if tier.suffix.is_none() {
        let rewritten = crate::attune::call::rewrite_context(
            block_inner_tokens(&variant_fn.block),
            crate::attune::call::Context {
                features: &tier.features,
                target_arch: tier.target_arch,
            },
            Some(&variant_fn.sig),
        );
        if let Err(err) = set_block_inner_tokens(&mut variant_fn.block, rewritten) {
            return Err(err.to_compile_error());
        }
    }

    // Build the attribute list. Scalar and default tiers have no features —
    // emit only `#[inline]` without `#[target_feature]` (enable="" is an error).
    let mut new_attrs: Vec<Attribute> = Vec::new();
    if !tier.features.is_empty() {
        let features_csv =
            crate::token_discovery::features_csv(tier.token.as_deref(), &tier.features);
        new_attrs.push(parse_quote!(#[target_feature(enable = #features_csv)]));
    }
    if let Some(policy) = inline {
        new_attrs.extend(policy.attribute(&variant_fn.vis));
    } else if !preserve_inline || !variant_fn.attrs.iter().any(|a| a.path().is_ident("inline")) {
        new_attrs.push(parse_quote!(#[inline]));
    }
    for attr in variant_fn
        .attrs
        .into_iter()
        .filter(|attr| (preserve_inline && inline.is_none()) || !attr.path().is_ident("inline"))
    {
        new_attrs.push(attr);
    }
    variant_fn.attrs = new_attrs;

    let body_imports = generate_imports(
        tier.target_arch,
        tier.magetypes_namespace,
        options.import_intrinsics,
        options.import_magetypes,
    );
    let proof_assertions = sig_proofs
        .as_ref()
        .map(|p| p.gen_assertions(&variant_fn.sig))
        .unwrap_or_default();
    prepend_to_block(
        &mut variant_fn.block,
        quote! { #proof_assertions #(#rebinds)* #body_imports },
    );

    let cfg_guard = gen_target_cfg_guard(
        tier.target_arch,
        options.cfg_feature.as_deref(),
        &tier.features,
    );
    drop_attrs_equal_to(&mut variant_fn.attrs, &cfg_guard);
    drop_attrs_equal_to(
        &mut variant_fn.attrs,
        &gen_cfg_guard(tier.target_arch, options.cfg_feature.as_deref()),
    );
    let vis = &variant_fn.vis;
    let sig = &variant_fn.sig;
    let attrs = &variant_fn.attrs;
    let body = block_inner_tokens(&variant_fn.block);
    Ok(quote! {
        #cfg_guard
        #(#attrs)*
        #vis #sig {
            #body
        }
    })
}
