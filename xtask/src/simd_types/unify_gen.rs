//! Unified canonical-backend code generation (`issue #175`).
//!
//! Collapses all per-token backend delegation into:
//! - `backends/ops.rs`: crate-private `*Ops` mirror traits implemented only by
//!   the canonical base tokens (`ScalarToken`, `X64V3Token`, `X64V4Token`,
//!   `NeonToken`, `Wasm128Token`), plus `NarrowOps` and `W512Ops`.
//! - `backends/sealed.rs`: `Sealed` supertrait mapping all 21 tokens (all 20
//!   registry tokens on their native architectures + `ScalarToken`) to their
//!   `Narrow` / `W512` canonical bases and hosting override hooks for AVX-512VL
//!   pixel packing, WASM `relaxed-simd` FMA/min/max/cvt/q15/dot, fast
//!   `to_u32_fast`, `dot_i8_i7_add`, `F32x16FromHalves`, and `F16Convert`.
//! - `impls/delegated.rs`: single blanket `impl<T: SimdToken + Sealed + Copy + 'static>`
//!   blocks for all 30 `*Backend`, 3 `*Convert`, and 10 `*Bitcast` traits.

use std::collections::{BTreeMap, HashSet};

use indoc::formatdoc;
use proc_macro2::{Group, Ident, Spacing, TokenStream, TokenTree};
use quote::{ToTokens, format_ident, quote};
use syn::{FnArg, Item, Pat, TraitItem};

/// Exact list of the 43 public backend/convert/bitcast trait names unified
/// through `*Ops` + `Sealed` + blanket impls (`PopcntBackend` stays on `X64V4xToken`).
pub(crate) fn is_unified_trait(name: &str) -> bool {
    matches!(
        name,
        "F32x4Backend"
            | "F32x8Backend"
            | "F32x16Backend"
            | "F64x2Backend"
            | "F64x4Backend"
            | "F64x8Backend"
            | "I8x16Backend"
            | "I8x32Backend"
            | "I8x64Backend"
            | "U8x16Backend"
            | "U8x32Backend"
            | "U8x64Backend"
            | "I16x8Backend"
            | "I16x16Backend"
            | "I16x32Backend"
            | "U16x8Backend"
            | "U16x16Backend"
            | "U16x32Backend"
            | "I32x4Backend"
            | "I32x8Backend"
            | "I32x16Backend"
            | "U32x4Backend"
            | "U32x8Backend"
            | "U32x16Backend"
            | "I64x2Backend"
            | "I64x4Backend"
            | "I64x8Backend"
            | "U64x2Backend"
            | "U64x4Backend"
            | "U64x8Backend"
            | "F32x4Convert"
            | "F32x8Convert"
            | "F32x16Convert"
            | "F64x2Convert"
            | "F64x4Convert"
            | "U32x4Bitcast"
            | "U32x8Bitcast"
            | "I64x2Bitcast"
            | "I64x4Bitcast"
            | "I8x16Bitcast"
            | "I8x32Bitcast"
            | "I16x8Bitcast"
            | "I16x16Bitcast"
            | "U64x2Bitcast"
            | "U64x4Bitcast"
    )
}

/// Map a public backend/convert/bitcast trait name to its internal `*Ops` counterpart.
pub(crate) fn ops_trait_name(public_name: &str) -> String {
    if let Some(stem) = public_name.strip_suffix("Backend") {
        format!("{stem}Ops")
    } else if public_name.ends_with("Convert") || public_name.ends_with("Bitcast") {
        format!("{public_name}Ops")
    } else {
        panic!("unexpected unified trait name: {public_name}");
    }
}

/// AVX-512VL pixel-packing overrides: `(trait_name, method_name, override_fn)`.
const V4_F32_OVERRIDES: &[(&str, &str, &str)] = &[
    ("F32x4Backend", "to_u8_bytes", "to_u8_bytes_x4_v4"),
    ("F32x4Backend", "store_rgba_bytes", "store_rgba_bytes_x4_v4"),
    ("F32x4Backend", "store_rgb_bytes", "store_rgb_bytes_x4_v4"),
    ("F32x8Backend", "to_u8_bytes", "to_u8_bytes_x8_v4"),
];

/// WASM `relaxed-simd` overrides: `(trait_name, method_name, override_fn)`.
const WASM_FMA_OVERRIDES: &[(&str, &str, &str)] = &[
    ("F32x4Backend", "mul_add", "mul_add_f32x4"),
    ("F32x4Backend", "mul_sub", "mul_sub_f32x4"),
    ("F32x4Backend", "mul_add_portable", "mul_add_f32x4"),
    ("F32x4Backend", "mul_sub_portable", "mul_sub_f32x4"),
    ("F32x4Backend", "min_fast", "min_fast_f32x4"),
    ("F32x4Backend", "max_fast", "max_fast_f32x4"),
    ("F32x4Backend", "mul_add_lane_relaxed", "mul_add_lane_f32x4"),
    ("F32x8Backend", "mul_add", "mul_add_f32x8"),
    ("F32x8Backend", "mul_sub", "mul_sub_f32x8"),
    ("F32x8Backend", "mul_add_portable", "mul_add_f32x8"),
    ("F32x8Backend", "mul_sub_portable", "mul_sub_f32x8"),
    ("F32x8Backend", "min_fast", "min_fast_f32x8"),
    ("F32x8Backend", "max_fast", "max_fast_f32x8"),
    ("F32x8Backend", "mul_add_lane_relaxed", "mul_add_lane_f32x8"),
    ("F32x16Backend", "mul_add", "mul_add_f32x16"),
    ("F32x16Backend", "mul_sub", "mul_sub_f32x16"),
    ("F32x16Backend", "mul_add_portable", "mul_add_f32x16"),
    ("F32x16Backend", "mul_sub_portable", "mul_sub_f32x16"),
    ("F32x16Backend", "min_fast", "min_fast_f32x16"),
    ("F32x16Backend", "max_fast", "max_fast_f32x16"),
    ("F64x2Backend", "mul_add", "mul_add_f64x2"),
    ("F64x2Backend", "mul_sub", "mul_sub_f64x2"),
    ("F64x2Backend", "mul_add_portable", "mul_add_f64x2"),
    ("F64x2Backend", "mul_sub_portable", "mul_sub_f64x2"),
    ("F64x2Backend", "min_fast", "min_fast_f64x2"),
    ("F64x2Backend", "max_fast", "max_fast_f64x2"),
    ("F64x4Backend", "mul_add", "mul_add_f64x4"),
    ("F64x4Backend", "mul_sub", "mul_sub_f64x4"),
    ("F64x4Backend", "mul_add_portable", "mul_add_f64x4"),
    ("F64x4Backend", "mul_sub_portable", "mul_sub_f64x4"),
    ("F64x4Backend", "min_fast", "min_fast_f64x4"),
    ("F64x4Backend", "max_fast", "max_fast_f64x4"),
    ("F64x8Backend", "mul_add", "mul_add_f64x8"),
    ("F64x8Backend", "mul_sub", "mul_sub_f64x8"),
    ("F64x8Backend", "mul_add_portable", "mul_add_f64x8"),
    ("F64x8Backend", "mul_sub_portable", "mul_sub_f64x8"),
    ("F64x8Backend", "min_fast", "min_fast_f64x8"),
    ("F64x8Backend", "max_fast", "max_fast_f64x8"),
    (
        "F32x4Convert",
        "convert_f32_to_i32_fast",
        "convert_f32_to_i32_fast_x4",
    ),
    (
        "F32x8Convert",
        "convert_f32_to_i32_fast",
        "convert_f32_to_i32_fast_x8",
    ),
    (
        "F32x16Convert",
        "convert_f32_to_i32_fast",
        "convert_f32_to_i32_fast_x16",
    ),
    ("I16x8Backend", "mul_hrs_q15", "mul_hrs_q15_x8"),
    ("I16x16Backend", "mul_hrs_q15", "mul_hrs_q15_x16"),
    ("I16x32Backend", "mul_hrs_q15", "mul_hrs_q15_x32"),
];

/// Generate `(backends/ops.rs, backends/sealed.rs, impls/delegated.rs)` from
/// `(trait_name, trait_file_src, is_w512)` tuples for all 45 unified traits.
pub(crate) fn generate_ops_and_sealed_and_delegated(
    specs: &[(String, String, bool)],
) -> (String, String, String) {
    assert_eq!(specs.len(), 45, "expected all 45 unified traits");

    let mut parsed_files: BTreeMap<String, syn::File> = BTreeMap::new();
    for (_, src, _) in specs {
        parsed_files
            .entry(src.clone())
            .or_insert_with(|| syn::parse_file(src).expect("valid generated backend trait file"));
    }

    let traits: Vec<(String, syn::ItemTrait, bool)> = specs
        .iter()
        .map(|(name, src, is_w512)| {
            let tr = find_trait(&parsed_files[src], name);
            (name.clone(), tr, *is_w512)
        })
        .collect();

    let ops_rs = generate_ops_rs(&traits);
    let (sealed_rs, v4_matched, wasm_matched) = generate_sealed_rs(&traits);
    assert_all_overrides_matched(&v4_matched, V4_F32_OVERRIDES);
    assert_all_overrides_matched(&wasm_matched, WASM_FMA_OVERRIDES);
    let delegated_rs = generate_delegated_rs(&traits);

    (ops_rs, sealed_rs, delegated_rs)
}

/// Convert a concrete base implementation file (`scalar.rs`, `x86_v3.rs`,
/// `x86_v4.rs`, `arm_neon.rs`, `wasm128.rs`) from public `*Backend`/`*Convert`/
/// `*Bitcast` traits to crate-private `*Ops` traits while leaving `*PopcntBackend`
/// on `X64V4xToken` untouched.
pub(crate) fn to_ops_impls(src: &str) -> String {
    let keep_popcnt = src.contains("PopcntBackend");
    let mut out = String::with_capacity(src.len());
    for line in src.split_inclusive('\n') {
        if let Some(use_idx) = line.find("use crate::simd::backends::*;") {
            let prefix = &line[..use_idx];
            if prefix.trim_start().is_empty() {
                if keep_popcnt {
                    out.push_str(prefix);
                    out.push_str("use crate::simd::backends::*;\n");
                }
                out.push_str(prefix);
                out.push_str("use crate::simd::backends::ops::*;\n");
                continue;
            }
        }
        // Split at line comments so doc comments & safety justifications are untouched.
        let (code, comment) = match line.find("//") {
            Some(pos) => (&line[..pos], &line[pos..]),
            None => (line, ""),
        };
        replace_unified_idents(code, &mut out);
        out.push_str(comment);
    }
    out
}

fn replace_unified_idents(code: &str, out: &mut String) {
    let bytes = code.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if is_ident_start(bytes[i]) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let ident = &code[start..i];
            if is_unified_trait(ident) {
                out.push_str(&ops_trait_name(ident));
            } else {
                out.push_str(ident);
            }
        } else {
            let ch = code[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn find_trait(file: &syn::File, name: &str) -> syn::ItemTrait {
    for item in &file.items {
        if let Item::Trait(tr) = item
            && tr.ident == name
        {
            return tr.clone();
        }
    }
    panic!("trait `{name}` not found in parsed backend file");
}

fn rewrite_unified_stream(tokens: TokenStream, map_to_ops: bool) -> TokenStream {
    let tts: Vec<TokenTree> = tokens.into_iter().collect();
    let mut out: Vec<TokenTree> = Vec::with_capacity(tts.len());
    let mut i = 0;
    while i < tts.len() {
        if i + 3 < tts.len()
            && let TokenTree::Ident(sup) = &tts[i]
            && sup == "super"
            && let TokenTree::Punct(p1) = &tts[i + 1]
            && p1.as_char() == ':'
            && p1.spacing() == Spacing::Joint
            && let TokenTree::Punct(p2) = &tts[i + 2]
            && p2.as_char() == ':'
            && let TokenTree::Ident(id) = &tts[i + 3]
            && is_unified_trait(&id.to_string())
        {
            i += 3;
            continue;
        }
        match &tts[i] {
            TokenTree::Group(g) => {
                let mut g2 = Group::new(
                    g.delimiter(),
                    rewrite_unified_stream(g.stream(), map_to_ops),
                );
                g2.set_span(g.span());
                out.push(TokenTree::Group(g2));
            }
            TokenTree::Ident(id) if map_to_ops && is_unified_trait(&id.to_string()) => {
                out.push(TokenTree::Ident(Ident::new(
                    &ops_trait_name(&id.to_string()),
                    id.span(),
                )));
            }
            other => out.push(other.clone()),
        }
        i += 1;
    }
    out.into_iter().collect()
}

fn map_tokens_to_ops(tokens: TokenStream) -> TokenStream {
    rewrite_unified_stream(tokens, true)
}

fn strip_super_on_unified(tokens: TokenStream) -> TokenStream {
    rewrite_unified_stream(tokens, false)
}

fn generate_ops_rs(traits: &[(String, syn::ItemTrait, bool)]) -> String {
    let mut blocks = Vec::new();
    let mut narrow_ops_bounds = Vec::new();
    let mut w512_ops_bounds = Vec::new();

    for (name, tr, is_w512) in traits {
        let ops_name = ops_trait_name(name);
        let ops_ident = format_ident!("{ops_name}");
        if *is_w512 {
            w512_ops_bounds.push(quote!(#ops_ident));
        } else {
            narrow_ops_bounds.push(quote!(#ops_ident));
        }

        let mut super_tokens: Vec<TokenStream> = Vec::new();
        for bound in &tr.supertraits {
            match bound {
                syn::TypeParamBound::Lifetime(lt) => super_tokens.push(quote!(#lt)),
                syn::TypeParamBound::Trait(tb) => {
                    let last = tb.path.segments.last().map(|s| s.ident.to_string());
                    match last.as_deref() {
                        Some("SimdToken") | Some("Sealed") | Some("Copy") => {}
                        Some(other) if is_unified_trait(other) => {
                            let mapped = format_ident!("{}", ops_trait_name(other));
                            super_tokens.push(quote!(#mapped));
                        }
                        _ => super_tokens.push(quote!(#tb)),
                    }
                }
                other => super_tokens.push(quote!(#other)),
            }
        }
        super_tokens.push(quote!(archmage::SimdToken));
        super_tokens.push(quote!(Copy));

        let mut item_streams: Vec<TokenStream> = Vec::new();
        for item in &tr.items {
            match item {
                TraitItem::Type(ty) => {
                    let attrs: Vec<_> = ty
                        .attrs
                        .iter()
                        .filter(|a| !a.path().is_ident("doc"))
                        .collect();
                    let ident = &ty.ident;
                    let bounds = &ty.bounds;
                    item_streams.push(quote! { #(#attrs)* type #ident: #bounds; });
                }
                TraitItem::Fn(m) => {
                    let attrs: Vec<_> = m
                        .attrs
                        .iter()
                        .filter(|a| !a.path().is_ident("doc") && !a.path().is_ident("inline"))
                        .collect();
                    let sig_ts = map_tokens_to_ops(m.sig.to_token_stream());
                    if let Some(default) = &m.default {
                        let default_ts = map_tokens_to_ops(default.to_token_stream());
                        item_streams.push(quote! {
                            #(#attrs)*
                            #[inline(always)]
                            #sig_ts #default_ts
                        });
                    } else {
                        item_streams.push(quote! {
                            #(#attrs)*
                            #sig_ts;
                        });
                    }
                }
                _ => panic!("unexpected trait item in {name}"),
            }
        }

        let cfg_attr = if *is_w512 {
            quote!(#[cfg(feature = "w512")])
        } else {
            quote!()
        };

        blocks.push(
            quote! {
                #cfg_attr
                pub trait #ops_ident: #(#super_tokens)+* {
                    #(#item_streams)*
                }
            }
            .to_string(),
        );
    }

    let narrow_bundle = quote! {
        pub trait NarrowOps:
            #(#narrow_ops_bounds)+*
            + crate::simd::generic::cross_width::NarrowCrossWidthOps
            + crate::simd::generic::fast_ops::NarrowToU32Ops
            + crate::simd::generic::quant_ops::NarrowDotI8Ops
            + archmage::SimdToken
            + Copy
            + 'static
        {}
        impl<T> NarrowOps for T where
            T: #(#narrow_ops_bounds)+*
                + crate::simd::generic::cross_width::NarrowCrossWidthOps
                + crate::simd::generic::fast_ops::NarrowToU32Ops
                + crate::simd::generic::quant_ops::NarrowDotI8Ops
                + archmage::SimdToken
                + Copy
                + 'static
        {}
    }
    .to_string();

    let w512_bundle = quote! {
        #[cfg(feature = "w512")]
        pub trait W512Ops:
            #(#w512_ops_bounds)+*
            + crate::simd::generic::fast_ops::W512ToU32Ops
            + crate::simd::generic::quant_ops::W512DotI8Ops
            + archmage::SimdToken
            + Copy
            + 'static
        {}
        #[cfg(feature = "w512")]
        impl<T> W512Ops for T where
            T: #(#w512_ops_bounds)+*
                + crate::simd::generic::fast_ops::W512ToU32Ops
                + crate::simd::generic::quant_ops::W512DotI8Ops
                + archmage::SimdToken
                + Copy
                + 'static
        {}
    }
    .to_string();

    let body = blocks.join("\n\n");
    formatdoc! {r#"
        //! Crate-private canonical backend operation traits.
        //!
        //! Implemented only by the five canonical base tokens (`ScalarToken`,
        //! `X64V3Token`, `X64V4Token`, `NeonToken`, `Wasm128Token`). Public
        //! `*Backend` / `*Convert` / `*Bitcast` traits delegate to these via
        //! blanket impls over `SimdToken + Sealed`.
        //!
        //! **Auto-generated** by `cargo xtask generate` - do not edit manually.

        #![allow(dead_code)]

        {narrow_bundle}

        {w512_bundle}

        {body}
    "#}
}

/// Rewrite a trait method signature for use as a hook method on `Sealed`.
/// Replaces `Self::Repr` with `<Self::{proj} as ops::{ops_trait}>::Repr`
/// and `<Self as {OtherTrait}>` with `<Self::{proj} as ops::{OtherTraitOps}>`.
fn map_sig_for_sealed(tokens: TokenStream, proj: &Ident, default_ops: &Ident) -> TokenStream {
    let tts: Vec<TokenTree> = tokens.into_iter().collect();
    let mut out = Vec::with_capacity(tts.len());
    let mut i = 0;
    while i < tts.len() {
        // Detect `Self :: Repr`
        if i + 2 < tts.len()
            && let TokenTree::Ident(id) = &tts[i]
            && id == "Self"
            && let TokenTree::Punct(p1) = &tts[i + 1]
            && p1.as_char() == ':'
            && p1.spacing() == Spacing::Joint
            && let TokenTree::Punct(p2) = &tts[i + 2]
            && p2.as_char() == ':'
            && i + 3 < tts.len()
            && let TokenTree::Ident(rhs) = &tts[i + 3]
            && rhs == "Repr"
        {
            let span = id.span();
            let q = quote::quote_spanned!(span => <Self::#proj as crate::simd::backends::ops::#default_ops>::Repr);
            out.extend(q);
            i += 4;
            continue;
        }
        // Detect `< Self as TraitName >` (inside `TokenStream` directly)
        if let TokenTree::Punct(lt) = &tts[i]
            && lt.as_char() == '<'
            && i + 4 < tts.len()
            && let TokenTree::Ident(self_id) = &tts[i + 1]
            && self_id == "Self"
            && let TokenTree::Ident(as_id) = &tts[i + 2]
            && as_id == "as"
            && let TokenTree::Ident(tr_id) = &tts[i + 3]
            && is_unified_trait(&tr_id.to_string())
            && let TokenTree::Punct(gt) = &tts[i + 4]
            && gt.as_char() == '>'
        {
            let mapped = format_ident!("{}", ops_trait_name(&tr_id.to_string()));
            let q = quote! { <Self::#proj as crate::simd::backends::ops::#mapped> };
            out.extend(q);
            i += 5;
            continue;
        }
        match &tts[i] {
            TokenTree::Group(g) => {
                let mut g2 = Group::new(
                    g.delimiter(),
                    map_sig_for_sealed(g.stream(), proj, default_ops),
                );
                g2.set_span(g.span());
                out.push(TokenTree::Group(g2));
            }
            other => out.push(other.clone()),
        }
        i += 1;
    }
    out.into_iter().collect()
}

fn hook_method_sig_and_call(
    m: &syn::TraitItemFn,
    hook_name: &str,
    proj: &Ident,
    default_ops: &Ident,
) -> (TokenStream, TokenStream) {
    let mut sig = m.sig.clone();
    sig.ident = format_ident!("{hook_name}");
    let sig_tokens = map_sig_for_sealed(sig.to_token_stream(), proj, default_ops);

    let call_args: Vec<TokenStream> = m
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_ty) => match &*pat_ty.pat {
                Pat::Ident(pi) => {
                    let id = &pi.ident;
                    Some(quote!(#id))
                }
                _ => panic!("expected simple ident arg in `{}`", m.sig.ident),
            },
        })
        .collect();
    let generic_args = generic_turbofish(&m.sig.generics);
    let hook_ident = format_ident!("{hook_name}");
    let call_expr = quote! { self.#hook_ident #generic_args (#(#call_args),*) };
    (sig_tokens, call_expr)
}

fn generic_turbofish(generics: &syn::Generics) -> TokenStream {
    let params: Vec<TokenStream> = generics
        .params
        .iter()
        .filter_map(|p| match p {
            syn::GenericParam::Const(c) => {
                let id = &c.ident;
                Some(quote!(#id))
            }
            syn::GenericParam::Type(t) => {
                let id = &t.ident;
                Some(quote!(#id))
            }
            syn::GenericParam::Lifetime(_) => None,
        })
        .collect();
    if params.is_empty() {
        quote!()
    } else {
        quote!(::<#(#params),*>)
    }
}

fn v4_hook_ident(trait_name: &str, method_name: &str) -> String {
    let tag = match trait_name {
        "F32x4Backend" => "f32x4",
        "F32x8Backend" => "f32x8",
        _ => panic!("unexpected V4 override trait `{trait_name}`"),
    };
    format!("v4_{tag}_{method_name}")
}

fn wasm_hook_ident(trait_name: &str, method_name: &str) -> String {
    let tag = match trait_name {
        "F32x4Backend" => "f32x4",
        "F32x8Backend" => "f32x8",
        "F32x16Backend" => "f32x16",
        "F64x2Backend" => "f64x2",
        "F64x4Backend" => "f64x4",
        "F64x8Backend" => "f64x8",
        "F32x4Convert" => "f32x4_cvt",
        "F32x8Convert" => "f32x8_cvt",
        "F32x16Convert" => "f32x16_cvt",
        "I16x8Backend" => "i16x8",
        "I16x16Backend" => "i16x16",
        "I16x32Backend" => "i16x32",
        _ => panic!("unexpected WASM FMA override trait `{trait_name}`"),
    };
    format!("wasm_fma_{tag}_{method_name}")
}

fn sig_with_proof_arg(tokens: TokenStream) -> TokenStream {
    let mut tts: Vec<TokenTree> = tokens.into_iter().collect();
    for tt in &mut tts {
        if let TokenTree::Group(g) = tt
            && g.delimiter() == proc_macro2::Delimiter::Parenthesis
        {
            let inner: Vec<TokenTree> = g.stream().into_iter().collect();
            let mut new_inner: Vec<TokenTree> = Vec::new();
            let mut replaced = false;
            for item in inner {
                if !replaced
                    && let TokenTree::Ident(id) = &item
                    && id == "self"
                {
                    new_inner.extend(quote!(proof: Self::Proof));
                    replaced = true;
                } else {
                    new_inner.push(item);
                }
            }
            let mut g2 = Group::new(g.delimiter(), new_inner.into_iter().collect());
            g2.set_span(g.span());
            *tt = TokenTree::Group(g2);
            break;
        }
    }
    tts.into_iter().collect()
}

type MethodKeySet = HashSet<(String, String)>;

fn generate_sealed_rs(
    traits: &[(String, syn::ItemTrait, bool)],
) -> (String, MethodKeySet, MethodKeySet) {
    let mut v4_matched = HashSet::new();
    let mut wasm_matched = HashSet::new();

    let mut sealed_hook_decls: Vec<TokenStream> = Vec::new();
    let mut sealed_hook_impls: Vec<TokenStream> = Vec::new();
    let mut cap_hook_defaults: Vec<TokenStream> = Vec::new();
    let mut v4_cap_override_methods: Vec<TokenStream> = Vec::new();
    let mut v4_cap_delegate_methods: Vec<TokenStream> = Vec::new();
    let mut wasm_cap_override_methods: Vec<TokenStream> = Vec::new();

    let trait_map: BTreeMap<&str, (&syn::ItemTrait, bool)> = traits
        .iter()
        .map(|(n, tr, w)| (n.as_str(), (tr, *w)))
        .collect();

    for &(tname, mname, helper) in V4_F32_OVERRIDES {
        let (tr, is_w512) = trait_map[tname];
        let m = find_fn(tr, mname);
        v4_matched.insert((tname.to_string(), mname.to_string()));
        let proj = format_ident!("{}", if is_w512 { "W512" } else { "Narrow" });
        let proj_fn = format_ident!("{}", if is_w512 { "w512" } else { "narrow" });
        let ops_id = format_ident!("{}", ops_trait_name(tname));
        let hook = v4_hook_ident(tname, mname);
        let (sig_ts, _) = hook_method_sig_and_call(m, &hook, &proj, &ops_id);
        let cap_sig_ts = sig_with_proof_arg(sig_ts.clone());
        let args = extract_arg_idents(&m.sig);
        let orig_fn = &m.sig.ident;
        let hook_ident = format_ident!("{hook}");
        let tf = generic_turbofish(&m.sig.generics);
        let helper_ident = format_ident!("{helper}");

        sealed_hook_decls.push(quote! {
            #sig_ts;
        });
        sealed_hook_impls.push(quote! {
            #[inline(always)]
            #sig_ts {
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::#hook_ident #tf (
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    #(#args),*
                )
            }
        });
        cap_hook_defaults.push(quote! {
            #[inline(always)]
            #cap_sig_ts {
                <Self::#proj as crate::simd::backends::ops::#ops_id>::#orig_fn #tf (Self::#proj_fn(proof), #(#args),*)
            }
        });
        v4_cap_override_methods.push(quote! {
            #[inline(always)]
            #cap_sig_ts {
                crate::simd::impls::x86_v4_f32_overrides::#helper_ident #tf (proof, #(#args),*)
            }
        });
        v4_cap_delegate_methods.push(quote! {
            #[inline(always)]
            #cap_sig_ts {
                <X64V4Base as SealedBase>::#hook_ident #tf (archmage::caps::extract_v4_caps(proof), #(#args),*)
            }
        });
    }

    for &(tname, mname, helper) in WASM_FMA_OVERRIDES {
        let (tr, is_w512) = trait_map[tname];
        let m = find_fn(tr, mname);
        wasm_matched.insert((tname.to_string(), mname.to_string()));
        let proj = format_ident!("{}", if is_w512 { "W512" } else { "Narrow" });
        let proj_fn = format_ident!("{}", if is_w512 { "w512" } else { "narrow" });
        let ops_id = format_ident!("{}", ops_trait_name(tname));
        let hook = wasm_hook_ident(tname, mname);
        let (sig_ts, _) = hook_method_sig_and_call(m, &hook, &proj, &ops_id);
        let cap_sig_ts = sig_with_proof_arg(sig_ts.clone());
        let args = extract_arg_idents(&m.sig);
        let orig_fn = &m.sig.ident;
        let hook_ident = format_ident!("{hook}");
        let tf = generic_turbofish(&m.sig.generics);
        let helper_ident = format_ident!("{helper}");
        let cfg_w512 = if is_w512 {
            quote!(#[cfg(feature = "w512")])
        } else {
            quote!()
        };

        sealed_hook_decls.push(quote! {
            #cfg_w512
            #sig_ts;
        });
        sealed_hook_impls.push(quote! {
            #cfg_w512
            #[inline(always)]
            #sig_ts {
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::#hook_ident #tf (
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    #(#args),*
                )
            }
        });
        cap_hook_defaults.push(quote! {
            #cfg_w512
            #[inline(always)]
            #cap_sig_ts {
                <Self::#proj as crate::simd::backends::ops::#ops_id>::#orig_fn #tf (Self::#proj_fn(proof), #(#args),*)
            }
        });
        wasm_cap_override_methods.push(quote! {
            #cfg_w512
            #[inline(always)]
            #cap_sig_ts {
                crate::simd::impls::wasm128_fma_overrides::#helper_ident #tf (proof, #(#args),*)
            }
        });
    }

    let sealed_hook_decls_str: String = sealed_hook_decls
        .into_iter()
        .map(|t| t.to_string())
        .collect();
    let sealed_hook_impls_str: String = sealed_hook_impls
        .into_iter()
        .map(|t| t.to_string())
        .collect();
    let cap_hook_defaults_str: String = cap_hook_defaults
        .into_iter()
        .map(|t| t.to_string())
        .collect();
    let v4_cap_overrides_str: String = v4_cap_override_methods
        .into_iter()
        .map(|t| t.to_string())
        .collect();
    let v4_cap_delegates_str: String = v4_cap_delegate_methods
        .into_iter()
        .map(|t| t.to_string())
        .collect();
    let wasm_cap_overrides_str: String = wasm_cap_override_methods
        .into_iter()
        .map(|t| t.to_string())
        .collect();

    let cap_impls = build_sealed_token_impls(
        &v4_cap_overrides_str,
        &v4_cap_delegates_str,
        &wasm_cap_overrides_str,
    );

    let out = formatdoc! {r#"
        //! Marker trait sealing all SIMD backend traits to validated archmage tokens
        //! and routing all tokens and `And` conjunctions through canonical `*Ops` traits.
        //!
        //! **Auto-generated** by `cargo xtask generate` - do not edit manually.

        /// Crate-internal supertrait seal for `MagetypesToken`.
        pub trait MagetypesSeal {{}}

        mod seal_cap {{
            pub trait SealedBase {{
                type Narrow: crate::simd::backends::ops::NarrowOps;
                #[cfg(feature = "w512")]
                type W512: crate::simd::backends::ops::W512Ops;
                type MagetypesBackend: archmage::SimdToken + archmage::IntoConcreteToken;
                type Proof: Copy;
                const IS_SCALAR_BACKED: bool;
                type SimdF32<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdF64<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdI8<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdI16<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdI32<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdI64<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdU8<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdU16<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdU32<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                type SimdU64<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
                const SIMD_F32_LANES: usize;
                const SIMD_F64_LANES: usize;
                const SIMD_I32_LANES: usize;

                fn narrow(proof: Self::Proof) -> Self::Narrow;
                #[cfg(feature = "w512")]
                fn w512(proof: Self::Proof) -> Self::W512;
                fn magetypes_backend(proof: Self::Proof) -> Self::MagetypesBackend;

                {cap_hook_defaults_str}

                #[inline(always)]
                fn f32x8_from_halves(
                    proof: Self::Proof,
                    lo: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
                    hi: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {{
                    <Self::Narrow as crate::simd::generic::cross_width::NarrowCrossWidthOps>::from_halves_x8(
                        Self::narrow(proof),
                        lo,
                        hi,
                    )
                }}

                #[inline(always)]
                fn f32x8_low(
                    proof: Self::Proof,
                    wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {{
                    <Self::Narrow as crate::simd::generic::cross_width::NarrowCrossWidthOps>::low_x8(
                        Self::narrow(proof),
                        wide,
                    )
                }}

                #[inline(always)]
                fn f32x8_high(
                    proof: Self::Proof,
                    wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {{
                    <Self::Narrow as crate::simd::generic::cross_width::NarrowCrossWidthOps>::high_x8(
                        Self::narrow(proof),
                        wide,
                    )
                }}

                #[inline(always)]
                fn mul_add_lane4_relaxed_x8<const LANE: i32>(
                    proof: Self::Proof,
                    a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                    v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
                    acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {{
                    <Self::Narrow as crate::simd::generic::cross_width::NarrowCrossWidthOps>::mul_add_lane4_relaxed_x8::<LANE>(
                        Self::narrow(proof),
                        a,
                        v_in,
                        acc,
                    )
                }}

                #[cfg(feature = "w512")]
                fn f32x16_from_halves(
                    proof: Self::Proof,
                    lo: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                    hi: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;

                #[cfg(feature = "w512")]
                fn f32x16_low(
                    proof: Self::Proof,
                    wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

                #[cfg(feature = "w512")]
                fn f32x16_high(
                    proof: Self::Proof,
                    wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

                #[inline(always)]
                fn f32x4_to_u32_fast(
                    proof: Self::Proof,
                    a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::U32x4Ops>::Repr {{
                    <Self::Narrow as crate::simd::generic::fast_ops::NarrowToU32Ops>::f32x4_to_u32_fast(
                        Self::narrow(proof),
                        a,
                    )
                }}

                #[inline(always)]
                fn f32x8_to_u32_fast(
                    proof: Self::Proof,
                    a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::U32x8Ops>::Repr {{
                    <Self::Narrow as crate::simd::generic::fast_ops::NarrowToU32Ops>::f32x8_to_u32_fast(
                        Self::narrow(proof),
                        a,
                    )
                }}

                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_to_u32_fast(
                    proof: Self::Proof,
                    a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
                ) -> <Self::W512 as crate::simd::backends::ops::U32x16Ops>::Repr {{
                    <Self::W512 as crate::simd::generic::fast_ops::W512ToU32Ops>::f32x16_to_u32_fast(
                        Self::w512(proof),
                        a,
                    )
                }}

                #[inline(always)]
                fn i32x4_dot_i8_i7_add(
                    proof: Self::Proof,
                    acc: <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr,
                    a: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
                    b: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr {{
                    <Self::Narrow as crate::simd::generic::quant_ops::NarrowDotI8Ops>::dot_i8_i7_add_x4(
                        Self::narrow(proof),
                        acc,
                        a,
                        b,
                    )
                }}

                #[inline(always)]
                fn i32x8_dot_i8_i7_add(
                    proof: Self::Proof,
                    acc: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
                    a: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
                    b: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {{
                    <Self::Narrow as crate::simd::generic::quant_ops::NarrowDotI8Ops>::dot_i8_i7_add_x8(
                        Self::narrow(proof),
                        acc,
                        a,
                        b,
                    )
                }}

                #[cfg(feature = "w512")]
                #[inline(always)]
                fn i32x16_dot_i8_i7_add(
                    proof: Self::Proof,
                    acc: <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr,
                    a: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
                    b: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
                ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr {{
                    <Self::W512 as crate::simd::generic::quant_ops::W512DotI8Ops>::dot_i8_i7_add_x16(
                        Self::w512(proof),
                        acc,
                        a,
                        b,
                    )
                }}

                fn f16_to_f32_into_ops(proof: Self::Proof, input: &[u16], output: &mut [f32]);

                fn f32_to_f16_into_ops(proof: Self::Proof, input: &[f32], output: &mut [u16]);

                #[cfg(feature = "w512")]
                fn f64x8_convert_to_i32(
                    proof: Self::Proof,
                    a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

                #[cfg(feature = "w512")]
                fn f64x8_convert_to_i32_saturating(
                    proof: Self::Proof,
                    a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

                #[cfg(feature = "w512")]
                fn f64x8_convert_to_i32_round(
                    proof: Self::Proof,
                    a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
                ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

                #[cfg(feature = "w512")]
                fn f64x8_convert_from_i32(
                    proof: Self::Proof,
                    a: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
                ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
            }}

            pub trait SealedFromCaps<
                T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
            > {{
                type Base: SealedBase;
                fn into_proof(token: T) -> <Self::Base as SealedBase>::Proof;
            }}
        }}

        /// Seals backend traits so only validated archmage tokens can implement them.
        ///
        /// Also supplies the canonical `Narrow` / `W512` projections and optional
        /// hardware override hooks (AVX-512VL pixel packing, WASM `relaxed-simd`
        /// single-rounding ops, AVX-512F `to_u32_fast` / `dot_i8` / `f16` slice,
        /// `F32x16FromHalves`).
        pub trait Sealed: crate::simd::generic::MagetypesToken + Copy + 'static {{
            /// Canonical 128/256-bit backend token implementing [`super::ops::NarrowOps`].
            type Narrow: crate::simd::backends::ops::NarrowOps;

            /// Canonical 512-bit backend token implementing [`super::ops::W512Ops`].
            #[cfg(feature = "w512")]
            type W512: crate::simd::backends::ops::W512Ops;

            /// Project `self` to its canonical narrow backend proof.
            fn narrow(self) -> Self::Narrow;

            /// Project `self` to its canonical 512-bit backend proof.
            #[cfg(feature = "w512")]
            fn w512(self) -> Self::W512;

            {sealed_hook_decls_str}

            fn f32x8_from_halves(
                self,
                lo: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
                hi: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

            fn f32x8_low(
                self,
                wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;

            fn f32x8_high(
                self,
                wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;

            fn mul_add_lane4_relaxed_x8<const LANE: i32>(
                self,
                a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
                acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

            #[cfg(feature = "w512")]
            fn f32x16_from_halves(
                self,
                lo: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                hi: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;

            #[cfg(feature = "w512")]
            fn f32x16_low(
                self,
                wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

            #[cfg(feature = "w512")]
            fn f32x16_high(
                self,
                wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

            fn f32x4_to_u32_fast(
                self,
                a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::U32x4Ops>::Repr;

            fn f32x8_to_u32_fast(
                self,
                a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::U32x8Ops>::Repr;

            #[cfg(feature = "w512")]
            fn f32x16_to_u32_fast(
                self,
                a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            ) -> <Self::W512 as crate::simd::backends::ops::U32x16Ops>::Repr;

            fn i32x4_dot_i8_i7_add(
                self,
                acc: <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr,
                a: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
                b: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr;

            fn i32x8_dot_i8_i7_add(
                self,
                acc: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
                a: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
                b: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

            #[cfg(feature = "w512")]
            fn i32x16_dot_i8_i7_add(
                self,
                acc: <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr,
                a: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
                b: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
            ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr;

            fn f16_to_f32_into_ops(self, input: &[u16], output: &mut [f32]);

            fn f32_to_f16_into_ops(self, input: &[f32], output: &mut [u16]);

            #[cfg(feature = "w512")]
            fn f64x8_convert_to_i32(
                self,
                a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

            #[cfg(feature = "w512")]
            fn f64x8_convert_to_i32_saturating(
                self,
                a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

            #[cfg(feature = "w512")]
            fn f64x8_convert_to_i32_round(
                self,
                a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

            #[cfg(feature = "w512")]
            fn f64x8_convert_from_i32(
                self,
                a: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
            ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
        }}

        impl<T> MagetypesSeal for T
        where
            T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
            <T as archmage::SimdToken>::__Caps: seal_cap::SealedFromCaps<T>,
        {{}}

        impl<T> crate::simd::generic::MagetypesToken for T
        where
            T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
            <T as archmage::SimdToken>::__Caps: seal_cap::SealedFromCaps<T>,
        {{
            type MagetypesBackend = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::MagetypesBackend;
            const __IS_SCALAR_BACKED: bool = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::IS_SCALAR_BACKED;

            #[inline(always)]
            fn magetypes(self) -> Self::MagetypesBackend {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::magetypes_backend(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                )
            }}
        }}

        impl<T> crate::SimdTypes for T
        where
            T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
            <T as archmage::SimdToken>::__Caps: seal_cap::SealedFromCaps<T>,
        {{
            type F32 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdF32<T>;
            type F64 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdF64<T>;
            type I8 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdI8<T>;
            type I16 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdI16<T>;
            type I32 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdI32<T>;
            type I64 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdI64<T>;
            type U8 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdU8<T>;
            type U16 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdU16<T>;
            type U32 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdU32<T>;
            type U64 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdU64<T>;
            const F32_LANES: usize = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SIMD_F32_LANES;
            const F64_LANES: usize = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SIMD_F64_LANES;
            const I32_LANES: usize = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SIMD_I32_LANES;
        }}

        impl<T> Sealed for T
        where
            T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
            <T as archmage::SimdToken>::__Caps: seal_cap::SealedFromCaps<T>,
        {{
            type Narrow = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::Narrow;
            #[cfg(feature = "w512")]
            type W512 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::W512;

            #[inline(always)]
            fn narrow(self) -> Self::Narrow {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::narrow(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn w512(self) -> Self::W512 {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::w512(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                )
            }}

            {sealed_hook_impls_str}

            #[inline(always)]
            fn f32x8_from_halves(
                self,
                lo: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
                hi: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x8_from_halves(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    lo,
                    hi,
                )
            }}

            #[inline(always)]
            fn f32x8_low(
                self,
                wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x8_low(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    wide,
                )
            }}

            #[inline(always)]
            fn f32x8_high(
                self,
                wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x8_high(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    wide,
                )
            }}

            #[inline(always)]
            fn mul_add_lane4_relaxed_x8<const LANE: i32>(
                self,
                a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
                acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::mul_add_lane4_relaxed_x8::<LANE>(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    a,
                    v_in,
                    acc,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn f32x16_from_halves(
                self,
                lo: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
                hi: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x16_from_halves(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    lo,
                    hi,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn f32x16_low(
                self,
                wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x16_low(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    wide,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn f32x16_high(
                self,
                wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x16_high(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    wide,
                )
            }}

            #[inline(always)]
            fn f32x4_to_u32_fast(
                self,
                a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::U32x4Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x4_to_u32_fast(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    a,
                )
            }}

            #[inline(always)]
            fn f32x8_to_u32_fast(
                self,
                a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::U32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x8_to_u32_fast(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    a,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn f32x16_to_u32_fast(
                self,
                a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            ) -> <Self::W512 as crate::simd::backends::ops::U32x16Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x16_to_u32_fast(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    a,
                )
            }}

            #[inline(always)]
            fn i32x4_dot_i8_i7_add(
                self,
                acc: <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr,
                a: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
                b: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::i32x4_dot_i8_i7_add(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    acc,
                    a,
                    b,
                )
            }}

            #[inline(always)]
            fn i32x8_dot_i8_i7_add(
                self,
                acc: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
                a: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
                b: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::i32x8_dot_i8_i7_add(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    acc,
                    a,
                    b,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn i32x16_dot_i8_i7_add(
                self,
                acc: <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr,
                a: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
                b: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
            ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::i32x16_dot_i8_i7_add(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    acc,
                    a,
                    b,
                )
            }}

            #[inline(always)]
            fn f16_to_f32_into_ops(self, input: &[u16], output: &mut [f32]) {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f16_to_f32_into_ops(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    input,
                    output,
                )
            }}

            #[inline(always)]
            fn f32_to_f16_into_ops(self, input: &[f32], output: &mut [u16]) {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32_to_f16_into_ops(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    input,
                    output,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn f64x8_convert_to_i32(
                self,
                a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f64x8_convert_to_i32(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    a,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn f64x8_convert_to_i32_saturating(
                self,
                a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f64x8_convert_to_i32_saturating(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    a,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn f64x8_convert_to_i32_round(
                self,
                a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f64x8_convert_to_i32_round(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    a,
                )
            }}

            #[cfg(feature = "w512")]
            #[inline(always)]
            fn f64x8_convert_from_i32(
                self,
                a: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
            ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {{
                <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f64x8_convert_from_i32(
                    <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
                    a,
                )
            }}
        }}

        {cap_impls}
    "#};

    (out, v4_matched, wasm_matched)
}

fn emit_cap_from_impl(
    flag_params: &[&str],
    cap_ty: &str,
    base_ty: &str,
    proof_ty: &str,
    into_expr: &str,
) -> String {
    let params = if flag_params.is_empty() {
        String::new()
    } else {
        let mut s = String::new();
        for p in flag_params {
            s.push_str(", ");
            s.push_str(p);
            s.push_str(": archmage::caps::Flag");
        }
        s
    };
    formatdoc! {r#"
        #[allow(clippy::type_complexity)]
        impl<T{params}> crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for {cap_ty}
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {{
            type Base = {base_ty};
            #[inline(always)]
            fn into_proof(token: T) -> {proof_ty} {{
                {into_expr}
            }}
        }}
    "#}
}

#[allow(clippy::too_many_lines)]
fn build_sealed_token_impls(
    v4_cap_overrides: &str,
    v4_cap_delegates: &str,
    wasm_cap_overrides: &str,
) -> String {
    let scalar_caps_impl = emit_cap_from_impl(
        &[],
        "archmage::caps::ScalarCaps",
        "crate::simd::backends::sealed::seal_base_impls::ScalarBase",
        "archmage::ScalarToken",
        "let _ = token; archmage::ScalarToken",
    );
    let x64_scalar_1 = emit_cap_from_impl(
        &[
            "C0", "C1", "C2", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13",
            "C14", "C15", "C16", "C17",
        ],
        "archmage::caps::X64CapSet<C0, C1, C2, archmage::caps::No, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::ScalarBase",
        "archmage::ScalarToken",
        "let _ = token; archmage::ScalarToken",
    );
    let x64_scalar_2 = emit_cap_from_impl(
        &[
            "C1", "C2", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C14",
            "C15", "C16", "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::No, C1, C2, archmage::caps::Yes, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::ScalarBase",
        "archmage::ScalarToken",
        "let _ = token; archmage::ScalarToken",
    );
    let x64_scalar_3 = emit_cap_from_impl(
        &[
            "C2", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C14", "C15",
            "C16", "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::Yes, archmage::caps::No, C2, archmage::caps::Yes, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::ScalarBase",
        "archmage::ScalarToken",
        "let _ = token; archmage::ScalarToken",
    );
    let x64_v3_no512 = emit_cap_from_impl(
        &[
            "C2", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C14", "C15",
            "C16", "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::Yes, archmage::caps::Yes, C2, archmage::caps::Yes, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::X64V3Base",
        "archmage::X64V3Token",
        "archmage::caps::extract_v3_caps(token)",
    );
    let x64_v3_512_a = emit_cap_from_impl(
        &[
            "C2", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C15", "C16",
            "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::Yes, archmage::caps::Yes, C2, archmage::caps::Yes, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, archmage::caps::No, C15, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::X64V3Base",
        "archmage::X64V3Token",
        "archmage::caps::extract_v3_caps(token)",
    );
    let x64_v3_512_b = emit_cap_from_impl(
        &[
            "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C15", "C16", "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::Yes, archmage::caps::Yes, archmage::caps::No, archmage::caps::Yes, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, archmage::caps::Yes, C15, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::X64V3Base",
        "archmage::X64V3Token",
        "archmage::caps::extract_v3_caps(token)",
    );
    let x64_v4_a = emit_cap_from_impl(
        &[
            "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C16", "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, archmage::caps::Yes, archmage::caps::No, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::X64V4Base",
        "archmage::X64V4Token",
        "archmage::caps::extract_v4_caps(token)",
    );
    let x64_v4_b = emit_cap_from_impl(
        &[
            "C4", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C16", "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, C4, archmage::caps::No, C6, C7, C8, C9, C10, C11, C12, C13, archmage::caps::Yes, archmage::caps::Yes, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::X64V4Base",
        "archmage::X64V4Token",
        "archmage::caps::extract_v4_caps(token)",
    );
    let x64_v4_c = emit_cap_from_impl(
        &[
            "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C16", "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, archmage::caps::No, archmage::caps::Yes, C6, C7, C8, C9, C10, C11, C12, C13, archmage::caps::Yes, archmage::caps::Yes, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::X64V4Base",
        "archmage::X64V4Token",
        "archmage::caps::extract_v4_caps(token)",
    );
    let x64_v4x = emit_cap_from_impl(
        &[
            "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C16", "C17",
        ],
        "archmage::caps::X64CapSet<archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes, C6, C7, C8, C9, C10, C11, C12, C13, archmage::caps::Yes, archmage::caps::Yes, C16, C17>",
        "crate::simd::backends::sealed::seal_base_impls::X64V4xBase",
        "archmage::X64V4xToken",
        "archmage::caps::extract_v4x_caps(token)",
    );
    let arm_scalar = emit_cap_from_impl(
        &[
            "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11",
        ],
        "archmage::caps::ArmCapSet<archmage::caps::No, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11>",
        "crate::simd::backends::sealed::seal_base_impls::ScalarBase",
        "archmage::ScalarToken",
        "let _ = token; archmage::ScalarToken",
    );
    let arm_neon = emit_cap_from_impl(
        &[
            "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11",
        ],
        "archmage::caps::ArmCapSet<archmage::caps::Yes, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11>",
        "crate::simd::backends::sealed::seal_base_impls::NeonBase",
        "archmage::NeonToken",
        "archmage::caps::extract_neon_caps(token)",
    );
    let wasm_scalar = emit_cap_from_impl(
        &["C1", "C2"],
        "archmage::caps::WasmCapSet<archmage::caps::No, C1, C2>",
        "crate::simd::backends::sealed::seal_base_impls::ScalarBase",
        "archmage::ScalarToken",
        "let _ = token; archmage::ScalarToken",
    );
    let wasm_non_relaxed = emit_cap_from_impl(
        &["C1", "C2"],
        "archmage::caps::WasmCapSet<archmage::caps::Yes, C1, C2>",
        "crate::simd::backends::sealed::seal_base_impls::Wasm128Base",
        "archmage::Wasm128Token",
        "archmage::caps::extract_wasm128_caps(token)",
    );
    let wasm_relaxed_a = emit_cap_from_impl(
        &["C1"],
        "archmage::caps::WasmCapSet<archmage::caps::Yes, C1, archmage::caps::No>",
        "crate::simd::backends::sealed::seal_base_impls::Wasm128Base",
        "archmage::Wasm128Token",
        "archmage::caps::extract_wasm128_caps(token)",
    );
    let wasm_relaxed_b = emit_cap_from_impl(
        &[],
        "archmage::caps::WasmCapSet<archmage::caps::Yes, archmage::caps::No, archmage::caps::Yes>",
        "crate::simd::backends::sealed::seal_base_impls::Wasm128Base",
        "archmage::Wasm128Token",
        "archmage::caps::extract_wasm128_caps(token)",
    );
    let wasm_relaxed_fma = emit_cap_from_impl(
        &[],
        "archmage::caps::WasmCapSet<archmage::caps::Yes, archmage::caps::Yes, archmage::caps::Yes>",
        "crate::simd::backends::sealed::seal_base_impls::Wasm128RelaxedFmaBase",
        "archmage::Wasm128RelaxedFmaToken",
        "archmage::caps::extract_wasm128_relaxed_fma_caps(token)",
    );

    formatdoc! {r#"
        mod seal_base_impls {{
            use super::seal_cap::SealedBase;

            macro_rules! impl_simd_types_assoc {{
                (scalar) => {{
                    type SimdF32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = f32;
                    type SimdF64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = f64;
                    type SimdI8<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = i8;
                    type SimdI16<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = i16;
                    type SimdI32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = i32;
                    type SimdI64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = i64;
                    type SimdU8<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = u8;
                    type SimdU16<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = u16;
                    type SimdU32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = u32;
                    type SimdU64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = u64;
                    const SIMD_F32_LANES: usize = 1;
                    const SIMD_F64_LANES: usize = 1;
                    const SIMD_I32_LANES: usize = 1;
                }};
                ($f32:ident, $f64:ident, $i8:ident, $i16:ident, $i32:ident, $i64:ident, $u8:ident, $u16:ident, $u32:ident, $u64:ident, $l32:literal, $l64:literal) => {{
                    type SimdF32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$f32<Token>;
                    type SimdF64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$f64<Token>;
                    type SimdI8<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$i8<Token>;
                    type SimdI16<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$i16<Token>;
                    type SimdI32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$i32<Token>;
                    type SimdI64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$i64<Token>;
                    type SimdU8<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$u8<Token>;
                    type SimdU16<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$u16<Token>;
                    type SimdU32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$u32<Token>;
                    type SimdU64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = crate::simd::generic::$u64<Token>;
                    const SIMD_F32_LANES: usize = $l32;
                    const SIMD_F64_LANES: usize = $l64;
                    const SIMD_I32_LANES: usize = $l32;
                }};
            }}

            pub struct ScalarBase;

            impl SealedBase for ScalarBase {{
                type Narrow = archmage::ScalarToken;
                #[cfg(feature = "w512")]
                type W512 = archmage::ScalarToken;
                type MagetypesBackend = archmage::ScalarToken;
                type Proof = archmage::ScalarToken;
                const IS_SCALAR_BACKED: bool = true;
                impl_simd_types_assoc!(scalar);

                #[inline(always)]
                fn narrow(proof: archmage::ScalarToken) -> Self::Narrow {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn w512(proof: archmage::ScalarToken) -> Self::W512 {{
                    proof
                }}
                #[inline(always)]
                fn magetypes_backend(proof: archmage::ScalarToken) -> Self::MagetypesBackend {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_from_halves(_proof: archmage::ScalarToken, lo: [f32; 8], hi: [f32; 8]) -> [f32; 16] {{
                    crate::simd::generic::cross_width::scalar_f32x16_from_halves(lo, hi)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_low(_proof: archmage::ScalarToken, wide: [f32; 16]) -> [f32; 8] {{
                    crate::simd::generic::cross_width::scalar_f32x16_low(wide)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_high(_proof: archmage::ScalarToken, wide: [f32; 16]) -> [f32; 8] {{
                    crate::simd::generic::cross_width::scalar_f32x16_high(wide)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32(_proof: archmage::ScalarToken, a: [f64; 8]) -> [i32; 8] {{
                    crate::simd::impls::scalar::scalar_f64x8_to_i32(a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_saturating(_proof: archmage::ScalarToken, a: [f64; 8]) -> [i32; 8] {{
                    crate::simd::impls::scalar::scalar_f64x8_to_i32(a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_round(_proof: archmage::ScalarToken, a: [f64; 8]) -> [i32; 8] {{
                    crate::simd::impls::scalar::scalar_f64x8_to_i32_round(a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_from_i32(_proof: archmage::ScalarToken, a: [i32; 8]) -> [f64; 8] {{
                    crate::simd::impls::scalar::scalar_f64x8_from_i32(a)
                }}
                #[inline(always)]
                fn f16_to_f32_into_ops(proof: archmage::ScalarToken, input: &[u16], output: &mut [f32]) {{
                    crate::simd::generic::convert_f16::f16_to_f32_slice_soft(proof, input, output);
                }}
                #[inline(always)]
                fn f32_to_f16_into_ops(proof: archmage::ScalarToken, input: &[f32], output: &mut [u16]) {{
                    crate::simd::generic::convert_f16::f32_to_f16_slice_soft(proof, input, output);
                }}
            }}

            #[cfg(target_arch = "x86_64")]
            pub struct X64V3Base;

            #[cfg(target_arch = "x86_64")]
            impl SealedBase for X64V3Base {{
                type Narrow = archmage::X64V3Token;
                #[cfg(feature = "w512")]
                type W512 = archmage::X64V3Token;
                type MagetypesBackend = archmage::X64V3Token;
                type Proof = archmage::X64V3Token;
                const IS_SCALAR_BACKED: bool = false;
                impl_simd_types_assoc!(f32x8, f64x4, i8x32, i16x16, i32x8, i64x4, u8x32, u16x16, u32x8, u64x4, 8, 4);

                #[inline(always)]
                fn narrow(proof: archmage::X64V3Token) -> Self::Narrow {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn w512(proof: archmage::X64V3Token) -> Self::W512 {{
                    proof
                }}
                #[inline(always)]
                fn magetypes_backend(proof: archmage::X64V3Token) -> Self::MagetypesBackend {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_from_halves(
                    _proof: archmage::X64V3Token,
                    lo: core::arch::x86_64::__m256,
                    hi: core::arch::x86_64::__m256,
                ) -> [core::arch::x86_64::__m256; 2] {{
                    [lo, hi]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_low(
                    _proof: archmage::X64V3Token,
                    wide: [core::arch::x86_64::__m256; 2],
                ) -> core::arch::x86_64::__m256 {{
                    wide[0]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_high(
                    _proof: archmage::X64V3Token,
                    wide: [core::arch::x86_64::__m256; 2],
                ) -> core::arch::x86_64::__m256 {{
                    wide[1]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32(
                    proof: archmage::X64V3Token,
                    a: [core::arch::x86_64::__m256d; 2],
                ) -> core::arch::x86_64::__m256i {{
                    crate::simd::impls::x86_v3::v3_f64x8_to_i32(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_saturating(
                    proof: archmage::X64V3Token,
                    a: [core::arch::x86_64::__m256d; 2],
                ) -> core::arch::x86_64::__m256i {{
                    crate::simd::impls::x86_v3::v3_f64x8_to_i32_saturating(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_round(
                    proof: archmage::X64V3Token,
                    a: [core::arch::x86_64::__m256d; 2],
                ) -> core::arch::x86_64::__m256i {{
                    crate::simd::impls::x86_v3::v3_f64x8_to_i32_round(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_from_i32(
                    proof: archmage::X64V3Token,
                    a: core::arch::x86_64::__m256i,
                ) -> [core::arch::x86_64::__m256d; 2] {{
                    crate::simd::impls::x86_v3::v3_f64x8_from_i32(proof, a)
                }}
                #[inline(always)]
                fn f16_to_f32_into_ops(proof: archmage::X64V3Token, input: &[u16], output: &mut [f32]) {{
                    crate::simd::generic::convert_f16::f16c_decode_select(proof, input, output);
                }}
                #[inline(always)]
                fn f32_to_f16_into_ops(proof: archmage::X64V3Token, input: &[f32], output: &mut [u16]) {{
                    crate::simd::generic::convert_f16::f16c_encode_select(proof, input, output);
                }}
            }}

            #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
            pub struct X64V4Base;

            #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
            impl SealedBase for X64V4Base {{
                type Narrow = archmage::X64V3Token;
                #[cfg(feature = "w512")]
                type W512 = archmage::X64V4Token;
                type MagetypesBackend = archmage::X64V4Token;
                type Proof = archmage::X64V4Token;
                const IS_SCALAR_BACKED: bool = false;
                impl_simd_types_assoc!(f32x16, f64x8, i8x64, i16x32, i32x16, i64x8, u8x64, u16x32, u32x16, u64x8, 16, 8);

                #[inline(always)]
                fn narrow(proof: archmage::X64V4Token) -> Self::Narrow {{
                    archmage::caps::extract_v3_caps(proof)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn w512(proof: archmage::X64V4Token) -> Self::W512 {{
                    proof
                }}
                #[inline(always)]
                fn magetypes_backend(proof: archmage::X64V4Token) -> Self::MagetypesBackend {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_from_halves(
                    proof: archmage::X64V4Token,
                    lo: core::arch::x86_64::__m256,
                    hi: core::arch::x86_64::__m256,
                ) -> core::arch::x86_64::__m512 {{
                    crate::simd::generic::cross_width::v4_f32x16_from_halves(proof, lo, hi)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_low(
                    proof: archmage::X64V4Token,
                    wide: core::arch::x86_64::__m512,
                ) -> core::arch::x86_64::__m256 {{
                    crate::simd::generic::cross_width::v4_f32x16_low(proof, wide)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_high(
                    proof: archmage::X64V4Token,
                    wide: core::arch::x86_64::__m512,
                ) -> core::arch::x86_64::__m256 {{
                    crate::simd::generic::cross_width::v4_f32x16_high(proof, wide)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32(
                    proof: archmage::X64V4Token,
                    a: core::arch::x86_64::__m512d,
                ) -> core::arch::x86_64::__m256i {{
                    crate::simd::impls::x86_v4::v4_f64x8_to_i32(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_saturating(
                    proof: archmage::X64V4Token,
                    a: core::arch::x86_64::__m512d,
                ) -> core::arch::x86_64::__m256i {{
                    crate::simd::impls::x86_v4::v4_f64x8_to_i32_saturating(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_round(
                    proof: archmage::X64V4Token,
                    a: core::arch::x86_64::__m512d,
                ) -> core::arch::x86_64::__m256i {{
                    crate::simd::impls::x86_v4::v4_f64x8_to_i32_round(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_from_i32(
                    proof: archmage::X64V4Token,
                    a: core::arch::x86_64::__m256i,
                ) -> core::arch::x86_64::__m512d {{
                    crate::simd::impls::x86_v4::v4_f64x8_from_i32(proof, a)
                }}
                {v4_cap_overrides}
                #[inline(always)]
                fn f32x4_to_u32_fast(
                    proof: archmage::X64V4Token,
                    a: core::arch::x86_64::__m128,
                ) -> core::arch::x86_64::__m128i {{
                    crate::simd::generic::fast_ops::v4_f32x4_to_u32_fast(proof, a)
                }}
                #[inline(always)]
                fn f32x8_to_u32_fast(
                    proof: archmage::X64V4Token,
                    a: core::arch::x86_64::__m256,
                ) -> core::arch::x86_64::__m256i {{
                    crate::simd::generic::fast_ops::v4_f32x8_to_u32_fast(proof, a)
                }}
                #[inline(always)]
                fn f16_to_f32_into_ops(proof: archmage::X64V4Token, input: &[u16], output: &mut [f32]) {{
                    crate::simd::generic::convert_f16::v4_f16_to_f32_into(proof, input, output);
                }}
                #[inline(always)]
                fn f32_to_f16_into_ops(proof: archmage::X64V4Token, input: &[f32], output: &mut [u16]) {{
                    crate::simd::generic::convert_f16::v4_f32_to_f16_into(proof, input, output);
                }}
            }}

            #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
            pub struct X64V4xBase;

            #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
            impl SealedBase for X64V4xBase {{
                type Narrow = archmage::X64V3Token;
                #[cfg(feature = "w512")]
                type W512 = archmage::X64V4Token;
                type MagetypesBackend = archmage::X64V4xToken;
                type Proof = archmage::X64V4xToken;
                const IS_SCALAR_BACKED: bool = false;
                impl_simd_types_assoc!(f32x16, f64x8, i8x64, i16x32, i32x16, i64x8, u8x64, u16x32, u32x16, u64x8, 16, 8);

                #[inline(always)]
                fn narrow(proof: archmage::X64V4xToken) -> Self::Narrow {{
                    archmage::caps::extract_v3_caps(proof)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn w512(proof: archmage::X64V4xToken) -> Self::W512 {{
                    archmage::caps::extract_v4_caps(proof)
                }}
                #[inline(always)]
                fn magetypes_backend(proof: archmage::X64V4xToken) -> Self::MagetypesBackend {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_from_halves(
                    proof: archmage::X64V4xToken,
                    lo: core::arch::x86_64::__m256,
                    hi: core::arch::x86_64::__m256,
                ) -> core::arch::x86_64::__m512 {{
                    <X64V4Base as SealedBase>::f32x16_from_halves(
                        archmage::caps::extract_v4_caps(proof),
                        lo,
                        hi,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_low(
                    proof: archmage::X64V4xToken,
                    wide: core::arch::x86_64::__m512,
                ) -> core::arch::x86_64::__m256 {{
                    <X64V4Base as SealedBase>::f32x16_low(
                        archmage::caps::extract_v4_caps(proof),
                        wide,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_high(
                    proof: archmage::X64V4xToken,
                    wide: core::arch::x86_64::__m512,
                ) -> core::arch::x86_64::__m256 {{
                    <X64V4Base as SealedBase>::f32x16_high(
                        archmage::caps::extract_v4_caps(proof),
                        wide,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32(
                    proof: archmage::X64V4xToken,
                    a: core::arch::x86_64::__m512d,
                ) -> core::arch::x86_64::__m256i {{
                    <X64V4Base as SealedBase>::f64x8_convert_to_i32(
                        archmage::caps::extract_v4_caps(proof),
                        a,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_saturating(
                    proof: archmage::X64V4xToken,
                    a: core::arch::x86_64::__m512d,
                ) -> core::arch::x86_64::__m256i {{
                    <X64V4Base as SealedBase>::f64x8_convert_to_i32_saturating(
                        archmage::caps::extract_v4_caps(proof),
                        a,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_round(
                    proof: archmage::X64V4xToken,
                    a: core::arch::x86_64::__m512d,
                ) -> core::arch::x86_64::__m256i {{
                    <X64V4Base as SealedBase>::f64x8_convert_to_i32_round(
                        archmage::caps::extract_v4_caps(proof),
                        a,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_from_i32(
                    proof: archmage::X64V4xToken,
                    a: core::arch::x86_64::__m256i,
                ) -> core::arch::x86_64::__m512d {{
                    <X64V4Base as SealedBase>::f64x8_convert_from_i32(
                        archmage::caps::extract_v4_caps(proof),
                        a,
                    )
                }}
                {v4_cap_delegates}
                #[inline(always)]
                fn f32x4_to_u32_fast(
                    proof: archmage::X64V4xToken,
                    a: core::arch::x86_64::__m128,
                ) -> core::arch::x86_64::__m128i {{
                    <X64V4Base as SealedBase>::f32x4_to_u32_fast(
                        archmage::caps::extract_v4_caps(proof),
                        a,
                    )
                }}
                #[inline(always)]
                fn f32x8_to_u32_fast(
                    proof: archmage::X64V4xToken,
                    a: core::arch::x86_64::__m256,
                ) -> core::arch::x86_64::__m256i {{
                    <X64V4Base as SealedBase>::f32x8_to_u32_fast(
                        archmage::caps::extract_v4_caps(proof),
                        a,
                    )
                }}
                #[inline(always)]
                fn f16_to_f32_into_ops(proof: archmage::X64V4xToken, input: &[u16], output: &mut [f32]) {{
                    <X64V4Base as SealedBase>::f16_to_f32_into_ops(
                        archmage::caps::extract_v4_caps(proof),
                        input,
                        output,
                    );
                }}
                #[inline(always)]
                fn f32_to_f16_into_ops(proof: archmage::X64V4xToken, input: &[f32], output: &mut [u16]) {{
                    <X64V4Base as SealedBase>::f32_to_f16_into_ops(
                        archmage::caps::extract_v4_caps(proof),
                        input,
                        output,
                    );
                }}
                #[inline(always)]
                fn i32x4_dot_i8_i7_add(
                    proof: archmage::X64V4xToken,
                    acc: core::arch::x86_64::__m128i,
                    a: core::arch::x86_64::__m128i,
                    b: core::arch::x86_64::__m128i,
                ) -> core::arch::x86_64::__m128i {{
                    crate::simd::generic::quant_ops::v4x_i32x4_dot_i8(proof, acc, a, b)
                }}
                #[inline(always)]
                fn i32x8_dot_i8_i7_add(
                    proof: archmage::X64V4xToken,
                    acc: core::arch::x86_64::__m256i,
                    a: core::arch::x86_64::__m256i,
                    b: core::arch::x86_64::__m256i,
                ) -> core::arch::x86_64::__m256i {{
                    crate::simd::generic::quant_ops::v4x_i32x8_dot_i8(proof, acc, a, b)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn i32x16_dot_i8_i7_add(
                    proof: archmage::X64V4xToken,
                    acc: core::arch::x86_64::__m512i,
                    a: core::arch::x86_64::__m512i,
                    b: core::arch::x86_64::__m512i,
                ) -> core::arch::x86_64::__m512i {{
                    crate::simd::generic::quant_ops::v4x_i32x16_dot_i8(proof, acc, a, b)
                }}
            }}

            #[cfg(target_arch = "aarch64")]
            pub struct NeonBase;

            #[cfg(target_arch = "aarch64")]
            impl SealedBase for NeonBase {{
                type Narrow = archmage::NeonToken;
                #[cfg(feature = "w512")]
                type W512 = archmage::NeonToken;
                type MagetypesBackend = archmage::NeonToken;
                type Proof = archmage::NeonToken;
                const IS_SCALAR_BACKED: bool = false;
                impl_simd_types_assoc!(f32x4, f64x2, i8x16, i16x8, i32x4, i64x2, u8x16, u16x8, u32x4, u64x2, 4, 2);

                #[inline(always)]
                fn narrow(proof: archmage::NeonToken) -> Self::Narrow {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn w512(proof: archmage::NeonToken) -> Self::W512 {{
                    proof
                }}
                #[inline(always)]
                fn magetypes_backend(proof: archmage::NeonToken) -> Self::MagetypesBackend {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_from_halves(
                    _proof: archmage::NeonToken,
                    lo: [core::arch::aarch64::float32x4_t; 2],
                    hi: [core::arch::aarch64::float32x4_t; 2],
                ) -> [core::arch::aarch64::float32x4_t; 4] {{
                    [lo[0], lo[1], hi[0], hi[1]]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_low(
                    _proof: archmage::NeonToken,
                    wide: [core::arch::aarch64::float32x4_t; 4],
                ) -> [core::arch::aarch64::float32x4_t; 2] {{
                    [wide[0], wide[1]]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_high(
                    _proof: archmage::NeonToken,
                    wide: [core::arch::aarch64::float32x4_t; 4],
                ) -> [core::arch::aarch64::float32x4_t; 2] {{
                    [wide[2], wide[3]]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32(
                    proof: archmage::NeonToken,
                    a: [core::arch::aarch64::float64x2_t; 4],
                ) -> [core::arch::aarch64::int32x4_t; 2] {{
                    crate::simd::impls::arm_neon::neon_f64x8_to_i32(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_saturating(
                    proof: archmage::NeonToken,
                    a: [core::arch::aarch64::float64x2_t; 4],
                ) -> [core::arch::aarch64::int32x4_t; 2] {{
                    crate::simd::impls::arm_neon::neon_f64x8_to_i32(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_round(
                    proof: archmage::NeonToken,
                    a: [core::arch::aarch64::float64x2_t; 4],
                ) -> [core::arch::aarch64::int32x4_t; 2] {{
                    crate::simd::impls::arm_neon::neon_f64x8_to_i32_round(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_from_i32(
                    proof: archmage::NeonToken,
                    a: [core::arch::aarch64::int32x4_t; 2],
                ) -> [core::arch::aarch64::float64x2_t; 4] {{
                    crate::simd::impls::arm_neon::neon_f64x8_from_i32(proof, a)
                }}
                #[inline(always)]
                fn f16_to_f32_into_ops(proof: archmage::NeonToken, input: &[u16], output: &mut [f32]) {{
                    crate::simd::generic::convert_f16::neon_f16_decode(proof, input, output);
                }}
                #[inline(always)]
                fn f32_to_f16_into_ops(proof: archmage::NeonToken, input: &[f32], output: &mut [u16]) {{
                    crate::simd::generic::convert_f16::neon_f16_encode(proof, input, output);
                }}
            }}

            #[cfg(target_arch = "wasm32")]
            pub struct Wasm128Base;

            #[cfg(target_arch = "wasm32")]
            impl SealedBase for Wasm128Base {{
                type Narrow = archmage::Wasm128Token;
                #[cfg(feature = "w512")]
                type W512 = archmage::Wasm128Token;
                type MagetypesBackend = archmage::Wasm128Token;
                type Proof = archmage::Wasm128Token;
                const IS_SCALAR_BACKED: bool = false;
                impl_simd_types_assoc!(f32x4, f64x2, i8x16, i16x8, i32x4, i64x2, u8x16, u16x8, u32x4, u64x2, 4, 2);

                #[inline(always)]
                fn narrow(proof: archmage::Wasm128Token) -> Self::Narrow {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn w512(proof: archmage::Wasm128Token) -> Self::W512 {{
                    proof
                }}
                #[inline(always)]
                fn magetypes_backend(proof: archmage::Wasm128Token) -> Self::MagetypesBackend {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_from_halves(
                    _proof: archmage::Wasm128Token,
                    lo: [core::arch::wasm32::v128; 2],
                    hi: [core::arch::wasm32::v128; 2],
                ) -> [core::arch::wasm32::v128; 4] {{
                    [lo[0], lo[1], hi[0], hi[1]]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_low(
                    _proof: archmage::Wasm128Token,
                    wide: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    [wide[0], wide[1]]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_high(
                    _proof: archmage::Wasm128Token,
                    wide: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    [wide[2], wide[3]]
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32(
                    proof: archmage::Wasm128Token,
                    a: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    crate::simd::impls::wasm128::wasm_f64x8_to_i32(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_saturating(
                    proof: archmage::Wasm128Token,
                    a: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    crate::simd::impls::wasm128::wasm_f64x8_to_i32(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_round(
                    proof: archmage::Wasm128Token,
                    a: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    crate::simd::impls::wasm128::wasm_f64x8_to_i32_round(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_from_i32(
                    proof: archmage::Wasm128Token,
                    a: [core::arch::wasm32::v128; 2],
                ) -> [core::arch::wasm32::v128; 4] {{
                    crate::simd::impls::wasm128::wasm_f64x8_from_i32(proof, a)
                }}
                #[inline(always)]
                fn f16_to_f32_into_ops(proof: archmage::Wasm128Token, input: &[u16], output: &mut [f32]) {{
                    crate::simd::generic::convert_f16::f16_to_f32_slice_soft(proof, input, output);
                }}
                #[inline(always)]
                fn f32_to_f16_into_ops(proof: archmage::Wasm128Token, input: &[f32], output: &mut [u16]) {{
                    crate::simd::generic::convert_f16::f32_to_f16_slice_soft(proof, input, output);
                }}
            }}

            #[cfg(all(target_arch = "wasm32", target_feature = "simd128", target_feature = "relaxed-simd"))]
            pub struct Wasm128RelaxedFmaBase;

            #[cfg(all(target_arch = "wasm32", target_feature = "simd128", target_feature = "relaxed-simd"))]
            impl SealedBase for Wasm128RelaxedFmaBase {{
                type Narrow = archmage::Wasm128Token;
                #[cfg(feature = "w512")]
                type W512 = archmage::Wasm128Token;
                type MagetypesBackend = archmage::Wasm128RelaxedFmaToken;
                type Proof = archmage::Wasm128RelaxedFmaToken;
                const IS_SCALAR_BACKED: bool = false;
                impl_simd_types_assoc!(f32x4, f64x2, i8x16, i16x8, i32x4, i64x2, u8x16, u16x8, u32x4, u64x2, 4, 2);

                #[inline(always)]
                fn narrow(proof: archmage::Wasm128RelaxedFmaToken) -> Self::Narrow {{
                    archmage::caps::extract_wasm128_caps(proof)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn w512(proof: archmage::Wasm128RelaxedFmaToken) -> Self::W512 {{
                    archmage::caps::extract_wasm128_caps(proof)
                }}
                #[inline(always)]
                fn magetypes_backend(proof: archmage::Wasm128RelaxedFmaToken) -> Self::MagetypesBackend {{
                    proof
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_from_halves(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    lo: [core::arch::wasm32::v128; 2],
                    hi: [core::arch::wasm32::v128; 2],
                ) -> [core::arch::wasm32::v128; 4] {{
                    <Wasm128Base as SealedBase>::f32x16_from_halves(
                        archmage::caps::extract_wasm128_caps(proof),
                        lo,
                        hi,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_low(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    wide: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    <Wasm128Base as SealedBase>::f32x16_low(
                        archmage::caps::extract_wasm128_caps(proof),
                        wide,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_high(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    wide: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    <Wasm128Base as SealedBase>::f32x16_high(
                        archmage::caps::extract_wasm128_caps(proof),
                        wide,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    a: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    <Wasm128Base as SealedBase>::f64x8_convert_to_i32(
                        archmage::caps::extract_wasm128_caps(proof),
                        a,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_saturating(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    a: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    <Wasm128Base as SealedBase>::f64x8_convert_to_i32_saturating(
                        archmage::caps::extract_wasm128_caps(proof),
                        a,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_to_i32_round(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    a: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 2] {{
                    <Wasm128Base as SealedBase>::f64x8_convert_to_i32_round(
                        archmage::caps::extract_wasm128_caps(proof),
                        a,
                    )
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f64x8_convert_from_i32(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    a: [core::arch::wasm32::v128; 2],
                ) -> [core::arch::wasm32::v128; 4] {{
                    <Wasm128Base as SealedBase>::f64x8_convert_from_i32(
                        archmage::caps::extract_wasm128_caps(proof),
                        a,
                    )
                }}
                #[inline(always)]
                fn f16_to_f32_into_ops(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    input: &[u16],
                    output: &mut [f32],
                ) {{
                    <Wasm128Base as SealedBase>::f16_to_f32_into_ops(
                        archmage::caps::extract_wasm128_caps(proof),
                        input,
                        output,
                    );
                }}
                #[inline(always)]
                fn f32_to_f16_into_ops(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    input: &[f32],
                    output: &mut [u16],
                ) {{
                    <Wasm128Base as SealedBase>::f32_to_f16_into_ops(
                        archmage::caps::extract_wasm128_caps(proof),
                        input,
                        output,
                    );
                }}
                {wasm_cap_overrides}
                #[inline(always)]
                fn mul_add_lane4_relaxed_x8<const LANE: i32>(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    a: [core::arch::wasm32::v128; 2],
                    v_in: core::arch::wasm32::v128,
                    acc: [core::arch::wasm32::v128; 2],
                ) -> [core::arch::wasm32::v128; 2] {{
                    crate::simd::generic::cross_width::wasm_fma_mul_add_lane4_relaxed::<LANE>(
                        proof, a, v_in, acc,
                    )
                }}
                #[inline(always)]
                fn f32x4_to_u32_fast(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    a: core::arch::wasm32::v128,
                ) -> core::arch::wasm32::v128 {{
                    crate::simd::generic::fast_ops::wasm_fma_f32x4_to_u32_fast(proof, a)
                }}
                #[inline(always)]
                fn f32x8_to_u32_fast(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    a: [core::arch::wasm32::v128; 2],
                ) -> [core::arch::wasm32::v128; 2] {{
                    crate::simd::generic::fast_ops::wasm_fma_f32x8_to_u32_fast(proof, a)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn f32x16_to_u32_fast(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    a: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 4] {{
                    crate::simd::generic::fast_ops::wasm_fma_f32x16_to_u32_fast(proof, a)
                }}
                #[inline(always)]
                fn i32x4_dot_i8_i7_add(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    acc: core::arch::wasm32::v128,
                    a: core::arch::wasm32::v128,
                    b: core::arch::wasm32::v128,
                ) -> core::arch::wasm32::v128 {{
                    crate::simd::generic::quant_ops::wasm_fma_i32x4_dot_i8(proof, acc, a, b)
                }}
                #[inline(always)]
                fn i32x8_dot_i8_i7_add(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    acc: [core::arch::wasm32::v128; 2],
                    a: [core::arch::wasm32::v128; 2],
                    b: [core::arch::wasm32::v128; 2],
                ) -> [core::arch::wasm32::v128; 2] {{
                    crate::simd::generic::quant_ops::wasm_fma_i32x8_dot_i8(proof, acc, a, b)
                }}
                #[cfg(feature = "w512")]
                #[inline(always)]
                fn i32x16_dot_i8_i7_add(
                    proof: archmage::Wasm128RelaxedFmaToken,
                    acc: [core::arch::wasm32::v128; 4],
                    a: [core::arch::wasm32::v128; 4],
                    b: [core::arch::wasm32::v128; 4],
                ) -> [core::arch::wasm32::v128; 4] {{
                    crate::simd::generic::quant_ops::wasm_fma_i32x16_dot_i8(proof, acc, a, b)
                }}
            }}
        }}

        {scalar_caps_impl}

        #[cfg(target_arch = "x86_64")]
        mod x64_cap_sealed {{
            {x64_scalar_1}
            {x64_scalar_2}
            {x64_scalar_3}
            #[cfg(not(feature = "avx512"))]
            mod no_avx512_cap_sealed {{
                {x64_v3_no512}
            }}
            #[cfg(feature = "avx512")]
            mod avx512_cap_sealed {{
                {x64_v3_512_a}
                {x64_v3_512_b}
                {x64_v4_a}
                {x64_v4_b}
                {x64_v4_c}
                {x64_v4x}
            }}
        }}

        #[cfg(target_arch = "aarch64")]
        mod arm_cap_sealed {{
            {arm_scalar}
            {arm_neon}
        }}

        #[cfg(target_arch = "wasm32")]
        mod wasm_cap_sealed {{
            {wasm_scalar}
            #[cfg(not(all(target_feature = "simd128", target_feature = "relaxed-simd")))]
            mod wasm_base_cap_sealed {{
                {wasm_non_relaxed}
            }}
            #[cfg(all(target_feature = "simd128", target_feature = "relaxed-simd"))]
            mod wasm_relaxed_cap_sealed {{
                {wasm_relaxed_a}
                {wasm_relaxed_b}
                {wasm_relaxed_fma}
            }}
        }}
    "#}
}

fn find_fn<'a>(tr: &'a syn::ItemTrait, name: &str) -> &'a syn::TraitItemFn {
    for item in &tr.items {
        if let TraitItem::Fn(m) = item
            && m.sig.ident == name
        {
            return m;
        }
    }
    panic!("method `{name}` not found in trait `{}`", tr.ident);
}

fn extract_arg_idents(sig: &syn::Signature) -> Vec<Ident> {
    sig.inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_ty) => match &*pat_ty.pat {
                Pat::Ident(pi) => Some(pi.ident.clone()),
                _ => panic!("expected simple ident arg in `{}`", sig.ident),
            },
        })
        .collect()
}

fn generate_delegated_rs(traits: &[(String, syn::ItemTrait, bool)]) -> String {
    let mut blocks = Vec::new();
    let v4_map: BTreeMap<(&str, &str), &str> = V4_F32_OVERRIDES
        .iter()
        .map(|&(t, m, h)| ((t, m), h))
        .collect();
    let wasm_map: BTreeMap<(&str, &str), &str> = WASM_FMA_OVERRIDES
        .iter()
        .map(|&(t, m, h)| ((t, m), h))
        .collect();

    for (name, tr, is_w512) in traits {
        let trait_ident = format_ident!("{name}");
        let ops_ident = format_ident!("{}", ops_trait_name(name));
        let proj = format_ident!("{}", if *is_w512 { "W512" } else { "Narrow" });
        let proj_fn = format_ident!("{}", if *is_w512 { "w512" } else { "narrow" });

        let mut items: Vec<TokenStream> = Vec::new();
        for item in &tr.items {
            match item {
                TraitItem::Type(ty) => {
                    assert_eq!(ty.ident, "Repr", "unexpected associated type in {name}");
                    items.push(quote! {
                        type Repr = <T::#proj as crate::simd::backends::ops::#ops_ident>::Repr;
                    });
                }
                TraitItem::Fn(m) => {
                    let raw_sig = &m.sig;
                    let mname = raw_sig.ident.to_string();
                    let args = extract_arg_idents(raw_sig);
                    let tf = generic_turbofish(&raw_sig.generics);
                    // Drop method-level `where Self: super::<Sibling>Backend` bounds in
                    // delegated.rs so rustc #152409 does not shadow `<T::Narrow as *Ops>::Repr`;
                    // all sibling *Backend traits already hold unconditionally on `T: SimdToken + Sealed`.
                    let mut clean_sig = raw_sig.clone();
                    clean_sig.generics.where_clause = None;
                    let sig = strip_super_on_unified(clean_sig.to_token_stream());
                    let body = if v4_map.contains_key(&(name.as_str(), mname.as_str())) {
                        let hook = format_ident!("{}", v4_hook_ident(name, &mname));
                        quote! { self.#hook #tf (#(#args),*) }
                    } else if wasm_map.contains_key(&(name.as_str(), mname.as_str())) {
                        let hook = format_ident!("{}", wasm_hook_ident(name, &mname));
                        quote! { self.#hook #tf (#(#args),*) }
                    } else {
                        let orig = &raw_sig.ident;
                        quote! {
                            <T::#proj as crate::simd::backends::ops::#ops_ident>::#orig #tf (
                                self.#proj_fn(),
                                #(#args),*
                            )
                        }
                    };
                    items.push(quote! {
                        #[inline(always)]
                        #sig {
                            #body
                        }
                    });
                }
                _ => panic!("unexpected trait item in {name}"),
            }
        }

        let cfg_attr = if *is_w512 {
            quote!(#[cfg(feature = "w512")])
        } else {
            quote!()
        };

        blocks.push(
            quote! {
                #cfg_attr
                impl<T: archmage::SimdToken + crate::simd::backends::sealed::Sealed + Copy + 'static>
                    crate::simd::backends::#trait_ident for T
                {
                    #(#items)*
                }
            }
            .to_string(),
        );
    }

    // `F64x8Convert` bridges W512 (`F64x8Backend`) and Narrow (`I32x8Backend`),
    // so it routes through `Sealed::f64x8_convert_*` hooks rather than a single
    // `*Ops` projection.
    blocks.push(
        quote! {
            #[cfg(feature = "w512")]
            impl<T: archmage::SimdToken + crate::simd::backends::sealed::Sealed + Copy + 'static>
                crate::simd::backends::F64x8Convert for T
            {
                #[inline(always)]
                fn convert_f64_to_i32(
                    self,
                    a: <Self as crate::simd::backends::F64x8Backend>::Repr,
                ) -> <Self as crate::simd::backends::I32x8Backend>::Repr {
                    self.f64x8_convert_to_i32(a)
                }

                #[inline(always)]
                fn convert_f64_to_i32_saturating(
                    self,
                    a: <Self as crate::simd::backends::F64x8Backend>::Repr,
                ) -> <Self as crate::simd::backends::I32x8Backend>::Repr {
                    self.f64x8_convert_to_i32_saturating(a)
                }

                #[inline(always)]
                fn convert_f64_to_i32_round(
                    self,
                    a: <Self as crate::simd::backends::F64x8Backend>::Repr,
                ) -> <Self as crate::simd::backends::I32x8Backend>::Repr {
                    self.f64x8_convert_to_i32_round(a)
                }

                #[inline(always)]
                fn convert_i32_to_f64(
                    self,
                    a: <Self as crate::simd::backends::I32x8Backend>::Repr,
                ) -> <Self as crate::simd::backends::F64x8Backend>::Repr {
                    self.f64x8_convert_from_i32(a)
                }
            }
        }
        .to_string(),
    );

    let body = blocks.join("\n\n");
    formatdoc! {r#"
        //! Blanket implementations of all 45 public backend, conversion, and bitcast
        //! traits (plus `F64x8Convert`) over every sealed archmage token.
        //!
        //! Routes through `T::Narrow` / `T::W512` (`backends/ops.rs`) and optional
        //! hardware overrides on `Sealed`.
        //!
        //! **Auto-generated** by `cargo xtask generate` - do not edit manually.
        //!
        //! # Safety (audit contract — checked backend boundaries)
        //!
        //! `Sealed` is crate-private and implemented only for archmage tokens whose
        //! `Narrow` / `W512` projections invoke `.v3()` / `.v4()` / `.neon()` /
        //! `.wasm128()` safe extractors or `ScalarToken`. Hardware override hooks
        //! forward to concrete `#[arcane]` / target-feature-proven functions.
        //! Every method delegates safely without `unsafe` or raw pointers.

        #![allow(clippy::too_many_arguments, unused_imports)]

        use crate::simd::backends::*;
        use crate::simd::backends::sealed::Sealed;

        {body}
    "#}
}

fn assert_all_overrides_matched(
    matched: &HashSet<(String, String)>,
    overrides: &[(&str, &str, &str)],
) {
    for &(trait_name, method_name, helper) in overrides {
        assert!(
            matched.contains(&(trait_name.to_string(), method_name.to_string())),
            "override ({trait_name}, {method_name} -> {helper}) did not match any parsed method; \
             method renamed or removed?"
        );
    }
}
