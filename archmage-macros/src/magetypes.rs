//! `#[magetypes]` — generate per-tier function variants via text substitution.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::common::*;
use crate::tiers::*;

/// Generate per-tier variants of the input function.
///
/// When `rite_flag` is false (default), non-fallback variants are wrapped
/// with `#[archmage::arcane]` (safe outer wrapper + `#[target_feature]`
/// inner via trampoline). When true, variants are annotated with
/// `#[archmage::rite(import_intrinsics)]` — direct `#[target_feature]` +
/// `#[inline]`, no trampoline, no optimization boundary. The rite form is
/// only safe to call from matching-feature contexts (another `#[arcane]`,
/// `#[rite]`, or `#[magetypes]`-generated variant, or via `incant!`
/// rewriting inside such a context). Standalone `incant!` dispatch at a
/// public boundary is NOT supported for rite-flavored magetypes because the
/// non-tier dispatcher can't safely call a bare `#[target_feature]` fn.
///
/// `defines` is a list of magetypes type names (e.g. `["f32x8", "u16x16"]`)
/// to inject as local type aliases at the top of each variant's body:
///
/// ```text
/// type f32x8 = ::magetypes::simd::generic::f32x8<Token>;
/// ```
///
/// The alias's `Token` is substituted to the concrete token type for each
/// tier (same as the rest of the body). This eliminates the boilerplate
/// `type f32x8 = GenericF32x8<Token>;` line users would otherwise write
/// inside every `#[magetypes]` function body.
pub(crate) fn magetypes_impl(
    mut input_fn: syn::ItemFn,
    tiers: &[ResolvedTier],
    rite_flag: bool,
    in_impl: bool,
    defines: &[String],
) -> TokenStream {
    if let Err(err) = validate_item_fn(&input_fn, "magetypes") {
        return err.to_compile_error();
    }
    // Propagate ordinary attributes once; these macro attributes are consumed
    // or supplied per tier below.
    input_fn.attrs.retain(|attr| {
        !["arcane", "rite", "magetypes"]
            .iter()
            .any(|name| attr.path().is_ident(name))
    });
    let fn_name = &input_fn.sig.ident;

    // Dispatch presence is independent of the tier. Scan the original body
    // once, before making variants (the define preamble contains no dispatch).
    let has_dispatch = tokens_contain_ident(
        &block_inner_tokens(&input_fn.block),
        &["incant", "dispatch_variant", "attuned", "reattune"],
    );
    // Token is still a placeholder here. Use the same type inspection as the
    // other tier macros, also recognizing that placeholder before substitution.
    // Do not mistake a vector parameter such as V<Token> for a token parameter.
    let tokenless_rite = rite_flag
        && has_dispatch
        && crate::token_discovery::discover_signature_proofs(&input_fn.sig)
            .ok()
            .flatten()
            .is_none()
        && !input_fn.sig.inputs.iter().any(|arg| {
            let syn::FnArg::Typed(param) = arg else {
                return false;
            };
            matches!(
                crate::token_discovery::extract_token_type_info(&param.ty),
                Some(crate::token_discovery::TokenTypeInfo::Generic(name)) if name == "Token"
            )
        });
    let mut variants = Vec::with_capacity(tiers.len());

    for tier in tiers {
        // Clone and rename at the AST level (no string surgery)
        let mut variant_fn = input_fn.clone();
        variant_fn.sig.ident = quote::format_ident!("{}_{}", fn_name, tier.suffix);

        // Prepend the `define(...)` type aliases to the body. Sub-tier tokens
        // (`v3_vnni`, `arm_v2`, `arm_v3`, `wasm128_relaxed`) resolve to their
        // base representation token (`X64V3Token`, `NeonToken`, `Wasm128Token`),
        // matching `#[attune(..., define(...))]`.
        if !defines.is_empty() {
            let define_token: TokenStream = if tier.token_path.is_empty() {
                quote!(Token)
            } else {
                tier.define_backend_token_path()
                    .parse()
                    .expect("tier define_backend_token_path must be valid tokens")
            };
            let aliases = defines.iter().map(|name| {
                let ident = quote::format_ident!("{name}");
                quote! {
                    #[allow(non_camel_case_types, dead_code)]
                    type #ident = ::magetypes::simd::generic::#ident<#define_token>;
                }
            });
            prepend_to_block(&mut variant_fn.block, quote! { #(#aliases)* });
        }

        // SIMD variants receive the remaining call rewrite from arcane/rite.
        // Scalar/default variants have no wrapper: tokenless rite fallbacks
        // must also select covered callees here. Tokenful and ordinary boundary
        // fallbacks retain runtime dispatch.
        if has_dispatch {
            let ctx = crate::rewrite::CallerContext {
                tier_suffix: tier.suffix,
                target_arch: tier.target_arch,
                token_ident: quote::format_ident!("_"),
                has_token: false,
                derive_token: tokenless_rite && tier.target_arch.is_none(),
            };
            let rewritten =
                crate::rewrite::rewrite_incant_in_body(block_inner_tokens(&variant_fn.block), &ctx);
            if let Err(err) = set_block_inner_tokens(&mut variant_fn.block, rewritten) {
                return err.to_compile_error();
            }
        }

        let cfg_guard = tier.variant_cfg_guard();
        let token = (!tier.token_path.is_empty()).then(|| {
            tier.token_path
                .parse::<TokenStream>()
                .expect("tier token_path must be valid tokens")
        });

        // Fallbacks need no feature emitter. Substitute their output directly,
        // without reparsing syntax that no later stage will inspect.
        if tier.is_fallback() {
            let mut function = variant_fn.to_token_stream();
            if let Some(token) = &token {
                function = replace_ident_in_tokens(function, "Token", token);
            }
            variants.push(quote!(#cfg_guard #function));
            continue;
        }

        // Specialize the placeholder `Token` across the `syn::ItemFn` AST.
        if let Some(token) = &token
            && let Err(error) = specialize_item_fn(&mut variant_fn, token)
        {
            return error.to_compile_error();
        }
        if tier.allow_unexpected_cfg {
            variant_fn
                .attrs
                .insert(0, syn::parse_quote!(#[allow(unexpected_cfgs)]));
        }
        let lowered = if rite_flag {
            let token = crate::generated::tier_to_canonical_token(tier.name)
                .expect("resolved tier has a canonical token");
            let context = crate::engine::feature::FeatureContext::from_tier_token(token)
                .expect("registered feature context");
            crate::engine::feature::emit(
                variant_fn,
                &SharedOptions {
                    import_intrinsics: true,
                    cfg_feature: tier.feature_gate.clone(),
                    ..Default::default()
                },
                None,
                false,
                context,
            )
            .unwrap_or_else(|diagnostic| diagnostic)
        } else {
            crate::engine::boundary::expand(
                variant_fn,
                "arcane",
                crate::engine::boundary::BoundaryOptions {
                    in_impl,
                    shared: SharedOptions {
                        cfg_feature: tier.feature_gate.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
        };
        // An unavailable variant must not expose a validation diagnostic
        // either. Successful emitters guard every item themselves.
        variants.push(if tokens_contain_ident(&lowered, &["compile_error"]) {
            quote!(#cfg_guard #lowered)
        } else {
            lowered
        });
    }

    let output = quote! {
        #(#variants)*
    };

    output
}

#[cfg(test)]
mod specialization_tests {
    use super::*;
    use quote::ToTokens;

    #[test]
    fn substitution_preserves_all_function_parts_and_dispatch_markers() {
        let mut function: syn::ItemFn = syn::parse_quote! {
            #[some_attribute(Token, "Token")]
            pub(in Token) fn kernel<T: Into<Token>>(proof: Token, x: T) -> Option<Token>
            where Token: Copy
            {
                let _: Option<Token> = None;
                incant!(helper::<Token>(Token, Some(Token::from_context())), [scalar]);
                stringify!("Token", Token)
            }
        };
        specialize_item_fn(&mut function, &quote!(archmage::ScalarToken)).unwrap();
        let expected: syn::ItemFn = syn::parse_quote! {
            #[some_attribute(archmage::ScalarToken, "Token")]
            pub(in archmage::ScalarToken) fn kernel<T: Into<archmage::ScalarToken>>(
                proof: archmage::ScalarToken, x: T
            ) -> Option<archmage::ScalarToken>
            where archmage::ScalarToken: Copy
            {
                let _: Option<archmage::ScalarToken> = None;
                incant!(helper::<archmage::ScalarToken>(Token, Some(archmage::ScalarToken::from_context())), [scalar]);
                stringify!("Token", archmage::ScalarToken)
            }
        };
        assert_eq!(
            function.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
