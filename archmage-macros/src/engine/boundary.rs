//! Feature-proof boundaries shared by all attribute frontends.
//! Parsing syntax is kept out of this module; the proof is authenticated before
//! emitting the single unsafe call.
use super::inline::InlinePolicy;
use crate::common::*;
use crate::token_discovery::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, FnArg, Type, parse_quote};

#[derive(Default)]
pub(crate) struct BoundaryOptions {
    /// Attune body policy; legacy frontends leave this unset.
    pub(crate) body_inline: Option<InlinePolicy>,
    /// Options spelled the same way in `#[rite]`: imports and `cfg(feature)`.
    pub(crate) shared: SharedOptions,
    /// Use `#[inline(always)]` instead of `#[inline]` for the inner function.
    /// Requires nightly Rust with `#![feature(target_feature_inline_always)]`.
    pub(crate) inline_always: bool,
    /// The concrete type to use for `self` receiver.
    /// When specified, `self`/`&self`/`&mut self` is transformed to `_self: Type`/`&Type`/`&mut Type`.
    /// Implies `nested = true`.
    pub(crate) self_type: Option<Type>,
    /// Use a nested inner function instead of a sibling function. Spelled
    /// `nested` or `in_trait`; implied by `_self = Type`. Required in trait
    /// impls, which cannot take the extra sibling item.
    pub(crate) nested: bool,
    /// The function is a receiver-less associated function in an inherent
    /// impl, so the wrapper calls its sibling as `Self::__arcane_fn`. An
    /// attribute macro cannot see the enclosing impl, so this is spelled out.
    pub(crate) in_impl: bool,
    /// Attributes for the forwarding wrapper only. In sibling mode the
    /// wrapper holds no author code, so lint levels here never reach the
    /// author's body; on wasm32 (one function) and in nested mode they do.
    pub(crate) wrapper_attrs: Vec<Attribute>,
}

/// Shared implementation for `#[arcane]` / sibling boundary macros.
pub(crate) fn expand(
    mut input_fn: syn::ItemFn,
    macro_name: &str,
    args: BoundaryOptions,
) -> TokenStream {
    if let Err(err) = validate_item_fn(&input_fn, macro_name) {
        return err.to_compile_error();
    }

    // Check for self receiver
    let has_self_receiver = input_fn
        .sig
        .inputs
        .first()
        .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));

    // Nested mode is required when _self = Type is used (for Self replacement in nested fn).
    // In sibling mode, self/Self work naturally since both fns live in the same impl scope.
    // However, if there's a self receiver in nested mode, we still need _self = Type.
    if has_self_receiver && args.nested && args.self_type.is_none() {
        let msg = format!(
            "{} with self receiver in nested mode requires `_self = Type` argument.\n\
             Example: #[{}(in_trait, _self = MyType)]\n\
             The body may keep using `self`; it is renamed to `_self` for the inner function.\n\
             \n\
             Alternatively, remove `nested`/`in_trait` to use sibling expansion (default), \
             which handles self/Self naturally in inherent impls.",
            macro_name, macro_name
        );
        return syn::Error::new_spanned(&input_fn.sig, msg).to_compile_error();
    }

    let (mut proofs, from_self_receiver) = match discover_signature_proofs(&input_fn.sig) {
        Ok(Some(proofs)) => (proofs, false),
        Ok(None) => {
            let receiver_proofs = if has_self_receiver && let Some(self_ty) = &args.self_type {
                let mut receiver_sig = input_fn.sig.clone();
                if let Some(FnArg::Receiver(receiver)) = receiver_sig.inputs.iter().next().cloned()
                {
                    receiver_sig.inputs.clear();
                    receiver_sig
                        .inputs
                        .push(nested_self_param(&receiver, self_ty));
                } else {
                    receiver_sig.inputs.clear();
                    receiver_sig.inputs.push(syn::parse_quote!(_self: #self_ty));
                }
                discover_signature_proofs(&receiver_sig)
            } else {
                Ok(None)
            };
            match receiver_proofs {
                Ok(Some(proofs)) => (proofs, true),
                Ok(None) => {
                    return missing_token_error(
                        &input_fn.sig,
                        macro_name,
                        &format!(
                            ". Supported forms:\n\
                             - Concrete: `token: X64V3Token` or `And<X64V3Token, X64GfniToken>`\n\
                             - impl Trait: `token: impl HasX64V2`\n\
                             - Generic: `fn foo<T: HasX64V2>(token: T, ...)`\n\
                             - With self: `#[{macro_name}(_self = Type)] fn method(&self, token: impl HasNeon, ...)`"
                        ),
                    );
                }
                Err(err) => return err.to_compile_error(),
            }
        }
        Err(err) => return err.to_compile_error(),
    };
    if from_self_receiver {
        proofs.mark_receiver_source();
    }

    // import_intrinsics with AVX-512 features needs archmage's avx512 feature
    // (propagated to archmage-macros) for the 512-bit safe memory wrappers.
    if let Some(err) = avx512_import_error(
        &input_fn.sig,
        args.shared.import_intrinsics,
        &proofs.features,
        proofs.single_concrete_name().unwrap_or("an AVX-512 token"),
    ) {
        return err;
    }

    // Prepend import statements to body if requested
    prepend_to_block(
        &mut input_fn.block,
        generate_imports(
            proofs.target_arch,
            proofs.magetypes_namespace,
            args.shared.import_intrinsics,
            args.shared.import_magetypes,
        ),
    );

    // Only the forwarding wrapper needs identifiers for every argument. Keep
    // the user's patterns on the feature body; name wildcard proof parameters
    // so contextual dispatch can use them.
    let wrapper_sig = {
        let mut sig = input_fn.sig.clone();
        rename_non_ident_params(&mut sig);
        crate::common::rename_underscore_params(&mut sig);
        sig
    };
    let mut primary_token_ident = proofs.primary_ident.clone();
    if let Some(ident) = rename_wildcard_token(&mut input_fn.sig) {
        primary_token_ident = ident;
    }

    // Rewrite incant!() calls in the body to direct tier calls or contextual calls.
    if let Some(tier_suffix) = proofs.single_tier_suffix()
        && let Some(tier) = crate::tiers::find_tier(tier_suffix)
    {
        let ctx = crate::rewrite::CallerContext {
            tier_suffix,
            target_arch: tier.target_arch,
            token_ident: primary_token_ident,
            has_token: true,
            derive_token: false,
        };
        let rewritten = crate::rewrite::rewrite_incant_in_function(
            block_inner_tokens(&input_fn.block),
            &ctx,
            Some(&input_fn.sig),
        );
        if let Err(err) = set_block_inner_tokens(&mut input_fn.block, rewritten) {
            return err.to_compile_error();
        }
    } else {
        let rewritten = crate::attune::call::rewrite_context(
            block_inner_tokens(&input_fn.block),
            crate::attune::call::Context {
                features: &proofs.features,
                target_arch: proofs.target_arch,
            },
            Some(&input_fn.sig),
        );
        if let Err(err) = set_block_inner_tokens(&mut input_fn.block, rewritten) {
            return err.to_compile_error();
        }
    }

    // Build a single target_feature attribute with all features comma-joined
    let features_csv = features_csv(proofs.single_concrete_name(), &proofs.features);
    if args.body_inline == Some(InlinePolicy::Always) && !proofs.features.is_empty() {
        return syn::Error::new_spanned(
            &input_fn.sig.ident,
            "inline(always) on target-feature bodies requires nightly; use inline(hint) or inline(never)",
        )
        .to_compile_error();
    }
    // The operation's visibility determines the body policy. Native lowering
    // makes the implementation private without changing its inlining needs.
    let inline_attr = args
        .body_inline
        .unwrap_or(if args.inline_always {
            InlinePolicy::Always
        } else {
            InlinePolicy::Hint
        })
        .attribute(&input_fn.vis);

    // Scalar has no instruction-set boundary. Preserve its signature and body
    // without emitting the invalid #[target_feature(enable = "")], while still
    // verifying token/And type tags so local shadow structs cannot spoof them.
    // When `_self = Type` is used (`args.nested`), route through `emit_boundary`
    // (with safe_callee) so `self` is constrained to `#self_ty` and `Self` is substituted.
    if features_csv.is_empty() {
        if args.nested && args.self_type.is_some() {
            let cfg_guard = gen_cfg_guard(None, args.shared.cfg_feature.as_deref());
            drop_attrs_equal_to(&mut input_fn.attrs, &cfg_guard);
            let proof_assertions = proofs.gen_assertions(&wrapper_sig);
            let callee = format!("__simd_inner_{}", input_fn.sig.ident);
            if let Some(err) = reserved_param_error(&input_fn.sig, &callee, macro_name)
                .or_else(|| reserved_param_error(&wrapper_sig, &callee, macro_name))
            {
                return err;
            }
            let parts = BoundaryParts {
                cfg_guard,
                target_feature_attrs: Vec::new(),
                inline_attr,
                proof_assertions,
                wrapper_sig,
                safe_callee: true,
            };
            return emit_boundary(input_fn, &args, parts);
        }
        let rebinds =
            normalize_single_fn_token_params(&mut input_fn.sig, &proofs.token_param_indices);
        let assertions = proofs.gen_assertions(&input_fn.sig);
        prepend_to_block(
            &mut input_fn.block,
            quote! {
                #assertions
                #(#rebinds)*
            },
        );
        let vis = &input_fn.vis;
        let sig = &input_fn.sig;
        let attrs = filter_inline_attrs(&input_fn.attrs);
        let body = block_inner_tokens(&input_fn.block);
        let cfg = gen_cfg_guard(None, args.shared.cfg_feature.as_deref());
        return quote! { #cfg #(#attrs)* #inline_attr #vis #sig { #body } };
    }
    let target_feature_attrs: Vec<Attribute> =
        vec![parse_quote!(#[target_feature(enable = #features_csv)])];

    // On wasm32, #[target_feature(enable = "simd128")] functions are safe (Rust 1.54+).
    // Authenticate all concrete, `And`, and `Has*` trait proofs unconditionally so
    // shadowed types or traits can never bypass proof checking on wasm32.
    // When `_self = Type` is used, route through `emit_boundary` (with safe_callee)
    // so `self` is checked against `#self_ty`, `Self` is substituted, and shadowed
    // traits / unrelated structs cannot slip past `_self = ...`.
    if proofs.target_arch == Some("wasm32") {
        if args.nested && args.self_type.is_some() {
            let cfg_guard = gen_target_cfg_guard(
                Some("wasm32"),
                args.shared.cfg_feature.as_deref(),
                &proofs.features,
            );
            drop_attrs_equal_to(&mut input_fn.attrs, &cfg_guard);
            drop_attrs_equal_to(
                &mut input_fn.attrs,
                &gen_cfg_guard(Some("wasm32"), args.shared.cfg_feature.as_deref()),
            );
            let proof_assertions = proofs.gen_assertions(&wrapper_sig);
            let callee = format!("__simd_inner_{}", input_fn.sig.ident);
            if let Some(err) = reserved_param_error(&input_fn.sig, &callee, macro_name)
                .or_else(|| reserved_param_error(&wrapper_sig, &callee, macro_name))
            {
                return err;
            }
            let parts = BoundaryParts {
                cfg_guard,
                target_feature_attrs,
                inline_attr,
                proof_assertions,
                wrapper_sig,
                safe_callee: true,
            };
            return emit_boundary(input_fn, &args, parts);
        }
        let rebinds =
            normalize_single_fn_token_params(&mut input_fn.sig, &proofs.token_param_indices);
        let assertions = proofs.gen_assertions(&input_fn.sig);
        let prefix = quote! {
            #assertions
            #(#rebinds)*
        };
        prepend_to_block(&mut input_fn.block, prefix);
        return arcane_impl_wasm_safe(
            input_fn,
            &args,
            &proofs.features,
            target_feature_attrs,
            inline_attr,
        );
    }

    let cfg_guard = gen_cfg_guard(proofs.target_arch, args.shared.cfg_feature.as_deref());
    drop_attrs_equal_to(&mut input_fn.attrs, &cfg_guard);
    let proof_assertions = proofs.gen_assertions(&wrapper_sig);
    let parts = BoundaryParts {
        cfg_guard,
        target_feature_attrs,
        inline_attr,
        proof_assertions,
        wrapper_sig,
        safe_callee: false,
    };
    let callee = if args.nested {
        format!("__simd_inner_{}", input_fn.sig.ident)
    } else {
        format!("__arcane_{}", input_fn.sig.ident)
    };
    if let Some(err) = reserved_param_error(&input_fn.sig, &callee, macro_name)
        .or_else(|| reserved_param_error(&parts.wrapper_sig, &callee, macro_name))
    {
        return err;
    }
    emit_boundary(input_fn, &args, parts)
}

/// Reject a parameter named like the generated callee, so nothing a caller
/// writes can stand between the wrapper's `unsafe` call and that function.
pub(crate) fn reserved_param_error(
    sig: &syn::Signature,
    callee: &str,
    macro_name: &str,
) -> Option<TokenStream> {
    sig.inputs.iter().find_map(|arg| match arg {
        FnArg::Typed(pat_type) => match pat_type.pat.as_ref() {
            syn::Pat::Ident(pat) if pat.ident == callee => Some(
                syn::Error::new_spanned(
                    &pat.ident,
                    format!(
                        "parameter `{callee}` shadows the function #[{macro_name}] generates for \
                         `{}` and would receive its `unsafe` call; rename the parameter",
                        sig.ident
                    ),
                )
                .to_compile_error(),
            ),
            _ => None,
        },
        FnArg::Receiver(_) => None,
    })
}

/// The pieces every boundary expansion emits: the cfg guard on both halves,
/// the attributes of the feature-enabled half, and the assertions that
/// authenticate every token proof before calling the body.
struct BoundaryParts {
    cfg_guard: TokenStream,
    target_feature_attrs: Vec<Attribute>,
    inline_attr: Option<Attribute>,
    proof_assertions: TokenStream,
    wrapper_sig: syn::Signature,
    safe_callee: bool,
}

/// WASM-safe expansion: emits rite-style output (no unsafe wrapper).
pub(crate) fn arcane_impl_wasm_safe(
    mut input_fn: syn::ItemFn,
    args: &BoundaryOptions,
    features: &[&str],
    target_feature_attrs: Vec<Attribute>,
    inline_attr: Option<Attribute>,
) -> TokenStream {
    let cfg_guard =
        gen_target_cfg_guard(Some("wasm32"), args.shared.cfg_feature.as_deref(), features);
    drop_attrs_equal_to(&mut input_fn.attrs, &cfg_guard);
    drop_attrs_equal_to(
        &mut input_fn.attrs,
        &gen_cfg_guard(Some("wasm32"), args.shared.cfg_feature.as_deref()),
    );
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let attrs = &input_fn.attrs;

    let body = block_inner_tokens(&input_fn.block);

    // Prepend target_feature + inline attrs, filtering user #[inline] to avoid duplicates
    let mut new_attrs = target_feature_attrs;
    new_attrs.extend(inline_attr);
    // One function is both wrapper and body here.
    new_attrs.extend(args.wrapper_attrs.iter().cloned());
    for attr in filter_inline_attrs(attrs) {
        new_attrs.push(attr.clone());
    }

    quote! {
        #cfg_guard
        #(#new_attrs)*
        #vis #sig {
            #body
        }
    }
}

/// One boundary emitter for sibling and nested placement.
fn emit_boundary(
    mut input: syn::ItemFn,
    options: &BoundaryOptions,
    parts: BoundaryParts,
) -> TokenStream {
    let BoundaryParts {
        cfg_guard,
        target_feature_attrs,
        inline_attr,
        proof_assertions,
        wrapper_sig,
        safe_callee,
    } = parts;
    let generics = input.sig.generics.clone();
    let vis = input.vis.clone();
    let name = if options.nested {
        format_ident!("__simd_inner_{}", input.sig.ident)
    } else {
        format_ident!("__arcane_{}", input.sig.ident)
    };
    let unsafety = input.sig.safety.clone();
    let attrs = if options.nested {
        filter_inline_attrs(&input.attrs)
            .into_iter()
            .cloned()
            .collect()
    } else {
        expect_as_allow(filter_inline_attrs(&input.attrs))
    };
    let body_attrs = body_attrs(&input.attrs, !options.nested);
    let has_self = input
        .sig
        .inputs
        .iter()
        .any(|arg| matches!(arg, FnArg::Receiver(_)));
    let self_ident = format_ident!("self");
    let forwarded: Vec<_> = wrapper_sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Receiver(_) => options.nested.then_some(&self_ident),
            FnArg::Typed(param) => match param.pat.as_ref() {
                syn::Pat::Ident(pat) => Some(&pat.ident),
                _ => None,
            },
        })
        .collect();

    if options.nested
        && let Some(ty) = &options.self_type
    {
        for arg in input.sig.inputs.iter_mut() {
            if let FnArg::Receiver(receiver) = arg {
                *arg = nested_self_param(receiver, ty);
            }
        }
        replace_self_in_item_fn(&mut input, ty, Some(&format_ident!("_self")));
    }

    let inner_sig = &input.sig;
    let turbofish = build_turbofish(&inner_sig.generics);
    let params = &inner_sig.inputs;
    let output = &inner_sig.output;
    let where_clause = &inner_sig.generics.where_clause;
    let body = block_inner_tokens(&input.block);

    let call = if options.nested {
        quote!(#name #turbofish(#(#forwarded),*))
    } else if has_self {
        quote!(self.#name #turbofish(#(#forwarded),*))
    } else if options.in_impl {
        quote!(Self::#name #turbofish(#(#forwarded),*))
    } else {
        quote!(#name #turbofish(#(#forwarded),*))
    };

    let before = if options.nested {
        quote!()
    } else {
        quote!(#[doc(hidden)] #(#body_attrs)*)
    };
    let after = if options.nested {
        quote!(#(#body_attrs)*)
    } else {
        quote!()
    };
    let inner = quote! {
        #before #(#target_feature_attrs)* #inline_attr #after
        #unsafety fn #name #generics (#params) #output #where_clause { #body }
    };
    let call_stmt = if safe_callee && !matches!(unsafety, syn::Safety::Unsafe(_)) {
        call
    } else {
        proof_call(call)
    };
    let sibling = (!options.nested).then_some(&inner);
    let sibling_cfg = (!options.nested).then_some(&cfg_guard);
    let nested = options.nested.then_some(&inner);
    let wrapper_attrs = &options.wrapper_attrs;
    quote! {
        #sibling_cfg #sibling
        #cfg_guard #(#attrs)* #(#wrapper_attrs)* #[inline(always)]
        #vis #wrapper_sig {
            #nested #proof_assertions #call_stmt
        }
    }
}

/// The only feature-boundary call emitted for native proof entries. Callers
/// must authenticate the supplied proof and resolve a body generated in the
/// same scope before reaching this function.
pub(crate) fn proof_call(call: TokenStream) -> TokenStream {
    // SAFETY: the enclosing proof entry establishes the callee's feature set.
    quote!(unsafe { #call })
}

#[cfg(test)]
mod assertion_tests {
    use super::*;
    use crate::arcane::ArcaneArgs;

    #[test]
    fn concrete_checks_are_a_single_shared_lookup() {
        let ty: Type = parse_quote!(renamed::X64V3Token);
        let check = gen_concrete_token_assertion("X64V3Token", &ty);
        assert_eq!(
            check.to_string(),
            quote!(let _: () = <renamed::X64V3Token>::__ARCHMAGE_ASSERT_TIER_F38B284B;).to_string()
        );
    }

    #[test]
    fn and_checks_emit_and_tag_and_component_checks() {
        let sig: syn::Signature =
            parse_quote!(fn f(t: archmage::And<renamed::X64V3Token, X64GfniToken>));
        let proofs = discover_signature_proofs(&sig).unwrap().unwrap();
        let assertions = proofs.gen_assertions(&sig);
        let s = assertions.to_string();
        assert!(s.contains("__ARCHMAGE_ASSERT_AND_71B5ED46"));
        assert!(s.contains("__ARCHMAGE_ASSERT_TIER_F38B284B"));
        assert!(s.contains("__ARCHMAGE_ASSERT_TIER_2713339E"));
    }

    #[test]
    fn no_shared_opt_in() {
        assert!(syn::parse_str::<ArcaneArgs>("shared").is_err());
    }

    #[test]
    fn in_impl_and_nested_are_exclusive() {
        assert!(syn::parse_str::<ArcaneArgs>("in_impl, nested").is_err());
        assert!(syn::parse_str::<ArcaneArgs>("in_impl").unwrap().in_impl);
        assert!(syn::parse_str::<ArcaneArgs>("in_trait").unwrap().nested);
    }
}
