//! Shared utilities for all proc-macros.

use proc_macro2::Ident;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Attribute, GenericParam, PathArguments, QSelf, Signature, Type,
    parse::{Parse, ParseStream},
    visit_mut::VisitMut,
};

/// Reject function shapes that cannot safely carry `#[target_feature]`.
pub(crate) fn validate_item_fn(item: &syn::ItemFn, macro_name: &str) -> syn::Result<()> {
    if item.sig.constness.is_some() {
        return Err(syn::Error::new_spanned(
            &item.sig,
            format!(
                "#[{macro_name}] cannot be used on a `const fn` \
                 (`#[target_feature]` functions cannot be const-evaluated)"
            ),
        ));
    }
    if item.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &item.sig,
            format!(
                "#[{macro_name}] cannot be used on an `async fn` \
                 (`#[target_feature]` is incompatible with async state machines)"
            ),
        ));
    }
    if item.sig.variadic.is_some() {
        return Err(syn::Error::new_spanned(
            &item.sig,
            format!("#[{macro_name}] cannot be used on a variadic function"),
        ));
    }
    if let Some(attr) = item.attrs.iter().find(|a| is_target_feature_attr(a)) {
        return Err(syn::Error::new_spanned(
            attr,
            format!(
                "#[{macro_name}] generates `#[target_feature]` from the token/tier; \
                 remove the manual `#[target_feature]` attribute"
            ),
        ));
    }
    Ok(())
}

fn is_target_feature_attr(attr: &Attribute) -> bool {
    if attr
        .path()
        .segments
        .last()
        .is_some_and(|seg| seg.ident == "target_feature")
    {
        return true;
    }
    if attr.path().is_ident("cfg_attr")
        && let syn::Meta::List(list) = &attr.meta
    {
        return tokens_contain_ident(&list.tokens, &["target_feature"]);
    }
    false
}

/// Replace all occurrences of the `Token` placeholder in a parsed function AST.
/// Preserves bare `Token` dispatch markers inside `incant!` / `dispatch_variant!`
/// argument lists while substituting `Token` in generic arguments and paths.
pub(crate) fn specialize_item_fn(
    item: &mut syn::ItemFn,
    token: &proc_macro2::TokenStream,
) -> syn::Result<()> {
    let token_path: syn::Path = syn::parse2(token.clone())?;
    let mut visitor = SpecializeTokenVisitor {
        token_path,
        token_tokens: token,
    };
    visitor.visit_item_fn_mut(item);
    Ok(())
}

struct SpecializeTokenVisitor<'a> {
    token_path: syn::Path,
    token_tokens: &'a proc_macro2::TokenStream,
}

impl SpecializeTokenVisitor<'_> {
    fn rewrite_path(&self, path: &mut syn::Path) {
        if path.leading_colon.is_none()
            && let Some(first) = path.segments.first()
            && first.ident == "Token"
            && first.arguments.is_empty()
        {
            let span = first.ident.span();
            let rest: Vec<_> = path.segments.iter().skip(1).cloned().collect();
            let mut replacement = self.token_path.clone();
            if let Some(last) = replacement.segments.last_mut() {
                last.ident.set_span(span);
            }
            path.leading_colon = replacement.leading_colon;
            path.segments = replacement.segments;
            for seg in rest {
                path.segments.push(seg);
            }
        }
    }
}

impl VisitMut for SpecializeTokenVisitor<'_> {
    fn visit_attribute_mut(&mut self, attr: &mut Attribute) {
        let tokens = attr.to_token_stream();
        if tokens_contain_ident(&tokens, &["Token"]) {
            let rewritten = replace_ident_in_tokens(tokens, "Token", self.token_tokens);
            if let Ok(parsed) = syn::parse::Parser::parse2(Attribute::parse_outer, rewritten)
                && let Some(first) = parsed.into_iter().next()
            {
                *attr = first;
            }
        }
    }

    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        syn::visit_mut::visit_path_mut(self, path);
        self.rewrite_path(path);
    }

    fn visit_macro_mut(&mut self, mac: &mut syn::Macro) {
        let tokens = std::mem::take(&mut mac.tokens);
        if mac
            .path
            .segments
            .last()
            .is_some_and(|s| s.ident == "incant" || s.ident == "dispatch_variant")
        {
            mac.tokens = rewrite_incant_macro_tokens(tokens, "Token", self.token_tokens);
        } else if tokens_contain_ident(&tokens, &["Token"]) {
            mac.tokens = replace_ident_in_tokens(tokens, "Token", self.token_tokens);
        } else {
            mac.tokens = tokens;
        }
    }
}

fn rewrite_incant_macro_tokens(
    tokens: proc_macro2::TokenStream,
    target: &str,
    replacement: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let parser = |input: ParseStream<'_>| -> syn::Result<proc_macro2::TokenStream> {
        let path: syn::Path = input.parse()?;
        let args;
        syn::parenthesized!(args in input);
        let args = args.parse_terminated(syn::Expr::parse, syn::Token![,])?;
        let rest: proc_macro2::TokenStream = input.parse()?;
        let path = replace_ident_in_tokens(path.to_token_stream(), target, replacement);
        let args = args.iter().map(|arg| {
            if matches!(arg, syn::Expr::Path(p) if p.qself.is_none() && p.path.is_ident("Token")) {
                arg.to_token_stream()
            } else {
                replace_ident_in_tokens(arg.to_token_stream(), target, replacement)
            }
        });
        let rest = replace_ident_in_tokens(rest, target, replacement);
        Ok(quote! { #path(#(#args),*) #rest })
    };
    use syn::parse::Parser;
    parser
        .parse2(tokens.clone())
        .unwrap_or_else(|_| replace_ident_in_tokens(tokens, target, replacement))
}

/// Substitute `Self` (type) and optionally `self` (value -> `_self`) throughout a
/// function's AST while respecting nested `impl`/`trait`/`mod` boundaries and
/// nested `fn` receiver scopes.
pub(crate) fn replace_self_in_item_fn(
    item: &mut syn::ItemFn,
    self_ty: &Type,
    self_ident: Option<&Ident>,
) {
    let mut replacer = SelfReplacer {
        self_ty,
        self_ident,
        replace_value: self_ident.is_some(),
    };
    replacer.visit_item_fn_mut_outer(item);
}

struct SelfReplacer<'a> {
    self_ty: &'a Type,
    self_ident: Option<&'a Ident>,
    replace_value: bool,
}

impl SelfReplacer<'_> {
    fn visit_item_fn_mut_outer(&mut self, item: &mut syn::ItemFn) {
        self.visit_signature_mut(&mut item.sig);
        self.visit_block_mut(&mut item.block);
    }

    fn rewrite_self_path(
        &self,
        qself: &mut Option<QSelf>,
        path: &mut syn::Path,
        expr_pos: bool,
    ) -> bool {
        if qself.is_some()
            || path.leading_colon.is_some()
            || !path
                .segments
                .first()
                .is_some_and(|s| s.ident == "Self" && s.arguments.is_empty())
        {
            return false;
        }
        if let Type::Path(rtp) = self.self_ty
            && rtp.qself.is_none()
        {
            let rest: Vec<_> = path.segments.iter().skip(1).cloned().collect();
            path.leading_colon = rtp.path.leading_colon;
            path.segments = rtp.path.segments.clone();
            if expr_pos {
                for seg in &mut path.segments {
                    if let PathArguments::AngleBracketed(ab) = &mut seg.arguments {
                        ab.colon2_token = Some(Default::default());
                    }
                }
            }
            for seg in rest {
                path.segments.push(seg);
            }
            return true;
        }
        if path.segments.len() > 1 {
            let rest: Vec<_> = path.segments.iter().skip(1).cloned().collect();
            *qself = Some(QSelf {
                lt_token: Default::default(),
                ty: Box::new(self.self_ty.clone()),
                position: 0,
                as_token: None,
                gt_token: Default::default(),
            });
            path.leading_colon = Some(Default::default());
            path.segments = rest.into_iter().collect();
            return true;
        }
        false
    }
}

impl VisitMut for SelfReplacer<'_> {
    fn visit_item_impl_mut(&mut self, _: &mut syn::ItemImpl) {
        // Nested impl introduces its own Self and self bindings.
    }

    fn visit_item_trait_mut(&mut self, _: &mut syn::ItemTrait) {
        // Nested trait introduces its own Self binding.
    }

    fn visit_item_mod_mut(&mut self, _: &mut syn::ItemMod) {
        // Nested mod is a separate scope.
    }

    fn visit_item_fn_mut(&mut self, item: &mut syn::ItemFn) {
        // Nested free/inner fn cannot capture outer `self` value, though it
        // still sees enclosing `Self` if inside an impl block.
        let prev = self.replace_value;
        self.replace_value = false;
        syn::visit_mut::visit_item_fn_mut(self, item);
        self.replace_value = prev;
    }

    fn visit_type_mut(&mut self, ty: &mut Type) {
        syn::visit_mut::visit_type_mut(self, ty);
        if let Type::Path(tp) = ty
            && tp.qself.is_none()
            && tp.path.leading_colon.is_none()
            && tp
                .path
                .segments
                .first()
                .is_some_and(|s| s.ident == "Self" && s.arguments.is_empty())
        {
            if tp.path.segments.len() == 1 {
                *ty = self.self_ty.clone();
            } else {
                self.rewrite_self_path(&mut tp.qself, &mut tp.path, false);
            }
        }
    }

    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        syn::visit_mut::visit_expr_mut(self, expr);
        match expr {
            syn::Expr::Path(ep) => {
                if self.replace_value
                    && ep.qself.is_none()
                    && ep.path.leading_colon.is_none()
                    && ep.path.segments.len() == 1
                    && ep.path.segments[0].ident == "self"
                    && ep.path.segments[0].arguments.is_empty()
                    && let Some(target_ident) = self.self_ident
                {
                    let span = ep.path.segments[0].ident.span();
                    let mut replacement = target_ident.clone();
                    replacement.set_span(span);
                    ep.path.segments[0].ident = replacement;
                    return;
                }
                self.rewrite_self_path(&mut ep.qself, &mut ep.path, true);
            }
            syn::Expr::Struct(es) => {
                let mut qself = None;
                self.rewrite_self_path(&mut qself, &mut es.path, true);
            }
            _ => {}
        }
    }

    fn visit_pat_mut(&mut self, pat: &mut syn::Pat) {
        syn::visit_mut::visit_pat_mut(self, pat);
        match pat {
            syn::Pat::Path(pp) => {
                self.rewrite_self_path(&mut pp.qself, &mut pp.path, true);
            }
            syn::Pat::Struct(ps) => {
                let mut qself = None;
                self.rewrite_self_path(&mut qself, &mut ps.path, true);
            }
            syn::Pat::TupleStruct(pts) => {
                let mut qself = None;
                self.rewrite_self_path(&mut qself, &mut pts.path, true);
            }
            _ => {}
        }
    }

    fn visit_macro_mut(&mut self, mac: &mut syn::Macro) {
        let tokens = std::mem::take(&mut mac.tokens);
        if tokens_contain_ident(&tokens, &["self", "Self"]) {
            let mut out = replace_self_in_tokens(tokens, self.self_ty);
            if self.replace_value
                && let Some(target_ident) = self.self_ident
            {
                out = replace_self_value_in_tokens(out, target_ident);
            }
            mac.tokens = out;
        } else {
            mac.tokens = tokens;
        }
    }
}

/// Preserve the parsed node when it has no placeholder. This also covers
/// `Token` in generic bounds, return types, attributes and visibility paths.
pub(crate) fn specialize_syntax<T: syn::parse::Parse + ToTokens>(
    syntax: &mut T,
    token: &proc_macro2::TokenStream,
) -> syn::Result<()> {
    let original = syntax.to_token_stream();
    if tokens_contain_ident(&original, &["Token"]) {
        *syntax = syn::parse2(replace_ident_in_tokens(original, "Token", token))?;
    }
    Ok(())
}

/// Filter out `#[inline]`, `#[inline(always)]`, `#[inline(never)]` from attributes.
pub(crate) fn filter_inline_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
    attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("inline"))
        .collect()
}

/// Check if an attribute is a lint-control attribute.
pub(crate) fn is_lint_attr(attr: &Attribute) -> bool {
    let path = attr.path();
    path.is_ident("allow")
        || path.is_ident("expect")
        || path.is_ident("deny")
        || path.is_ident("warn")
        || path.is_ident("forbid")
}

/// The attributes for the half of a boundary expansion that holds the user's
/// body (the `__arcane_` sibling, or the nested inner fn): the lint levels,
/// which must govern the body's code (#17), and `#[track_caller]`, which must
/// sit on every function between a panic in the body and the caller it names.
/// In nested mode the inner fn sits inside the wrapper, so the wrapper's lint
/// levels already cover it and `lint_levels` is false.
pub(crate) fn body_attrs(attrs: &[Attribute], lint_levels: bool) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| (lint_levels && is_lint_attr(attr)) || attr.path().is_ident("track_caller"))
        .cloned()
        .collect()
}

/// The attributes for the wrapper or dispatcher that only forwards its
/// arguments, with each `#[expect(..)]` turned into `#[allow(..)]`. The
/// forwarding function uses every parameter and has no body of its own, so a
/// copy of the expectation there would be unfulfilled; the body half keeps
/// the `#[expect]`, which is fulfilled or reported there.
pub(crate) fn expect_as_allow<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
) -> Vec<Attribute> {
    attrs
        .into_iter()
        .map(|attr| {
            let mut attr = attr.clone();
            if attr.path().is_ident("expect")
                && let syn::Meta::List(list) = &mut attr.meta
                && let Some(last) = list.path.segments.last_mut()
            {
                last.ident = Ident::new("allow", last.ident.span());
            }
            attr
        })
        .collect()
}

/// Generate a cfg guard combining target_arch and an optional feature gate.
pub(crate) fn gen_cfg_guard(
    target_arch: Option<&str>,
    cfg_feature: Option<&str>,
) -> proc_macro2::TokenStream {
    gen_target_cfg_guard::<&str>(target_arch, cfg_feature, &[])
}

/// Generate a cfg guard combining `target_arch`, WASM compile-time
/// `target_feature` predicates (since WebAssembly validates the whole module at
/// load time and has no runtime instruction-set detection), and an optional
/// Cargo `feature` gate.
pub(crate) fn gen_target_cfg_guard<S: AsRef<str>>(
    target_arch: Option<&str>,
    cfg_feature: Option<&str>,
    target_features: &[S],
) -> proc_macro2::TokenStream {
    let mut preds = Vec::new();
    if let Some(arch) = target_arch {
        preds.push(quote! { target_arch = #arch });
        if arch == "wasm32" {
            for tf in target_features {
                let tf = tf.as_ref();
                preds.push(quote! { target_feature = #tf });
            }
        }
    }
    if let Some(feat) = cfg_feature {
        preds.push(quote! { feature = #feat });
    }
    match preds.as_slice() {
        [] => quote! {},
        [only] => quote! { #[cfg(#only)] },
        many => quote! { #[cfg(all(#(#many),*))] },
    }
}

/// Required target features for a registered tier name (used for WASM compile-time
/// `cfg(target_feature = ...)` gating).
pub(crate) fn tier_wasm_features(tier_name: &str) -> &'static [&'static str] {
    crate::generated::tier_to_canonical_token(tier_name)
        .and_then(crate::generated::token_to_features)
        .unwrap_or(&[])
}

/// Drop from `attrs` any `#[cfg]` equal to the guard the macro emits on both
/// halves of its output (a `#[magetypes]` variant arrives with its
/// `#[cfg(target_arch = ...)]` already on it), so the wrapper, which copies
/// the input's attributes, is not gated twice.
pub(crate) fn drop_attrs_equal_to(attrs: &mut Vec<Attribute>, guard: &proc_macro2::TokenStream) {
    let wanted = guard.to_string();
    if wanted.is_empty() {
        return;
    }
    attrs.retain(|a| a.to_token_stream().to_string() != wanted);
}

/// Build a turbofish token stream from a function's generics.
pub(crate) fn build_turbofish(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let params: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(tp) => Some(&tp.ident),
            GenericParam::Const(cp) => Some(&cp.ident),
            GenericParam::Lifetime(_) => None,
        })
        .collect();
    if params.is_empty() {
        quote! {}
    } else {
        quote! { ::<#(#params),*> }
    }
}

/// Conservative token-level presence check. Literals and partial identifier
/// matches do not count. A false positive only costs a rewrite pass; a false
/// negative would change expansion, so all delimiter kinds are traversed.
pub(crate) fn tokens_contain_ident(tokens: &proc_macro2::TokenStream, names: &[&str]) -> bool {
    tokens.clone().into_iter().any(|token| match token {
        proc_macro2::TokenTree::Ident(id) => names.iter().any(|name| id == *name),
        proc_macro2::TokenTree::Group(group) => tokens_contain_ident(&group.stream(), names),
        _ => false,
    })
}

/// Marker for a dispatch-macro invocation that a body rewriter already
/// expanded. Rewriters emit `<the caller's macro path>!(@__archmage_expanded
/// tokens)` instead of splicing `tokens` directly, so the caller's import of
/// that macro is used (no `unused_imports`) and the result keeps bang-macro
/// expression parsing (`incant!(f(x)) + 1` at statement start). A path cannot
/// begin with `@`, so the marker never collides with user input.
const EXPANDED_MARKER: &str = "__archmage_expanded";

/// The already-expanded tokens of a marked invocation, if `input` is one.
pub(crate) fn expanded_passthrough(
    input: &proc_macro2::TokenStream,
) -> Option<proc_macro2::TokenStream> {
    let mut tokens = input.clone().into_iter();
    match (tokens.next(), tokens.next()) {
        (Some(proc_macro2::TokenTree::Punct(at)), Some(proc_macro2::TokenTree::Ident(marker)))
            if at.as_char() == '@' && marker == EXPANDED_MARKER =>
        {
            Some(tokens.collect())
        }
        _ => None,
    }
}

/// Re-emit a rewritten invocation through the caller's own macro path and
/// bang (`path_and_bang`, copied verbatim) and the caller's delimiter: see
/// [`EXPANDED_MARKER`]. Keeping the delimiter keeps statement parsing as
/// written (`attuned!{ f(x) }` needs no trailing `;`, `attuned!(f(x))` does).
pub(crate) fn wrap_expanded(
    path_and_bang: impl IntoIterator<Item = proc_macro2::TokenTree>,
    original: &proc_macro2::Group,
    expanded: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let span = original.span();
    let marker = Ident::new(EXPANDED_MARKER, span);
    let mut group = proc_macro2::Group::new(original.delimiter(), quote!(@ #marker #expanded));
    group.set_span(span);
    let mut out: proc_macro2::TokenStream = path_and_bang.into_iter().collect();
    out.extend([proc_macro2::TokenTree::Group(group)]);
    out
}

/// Replace all occurrences of a named identifier in a token stream.
///
/// Recurses into groups (braces, parens, brackets). Each matching `Ident` is
/// replaced with `replacement` — which can be multiple tokens (e.g., a path
/// like `archmage::X64V3Token`). Non-matching tokens pass through unchanged.
pub(crate) fn replace_ident_in_tokens(
    tokens: proc_macro2::TokenStream,
    target: &str,
    replacement: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut result = proc_macro2::TokenStream::new();
    let mut tokens = tokens.into_iter().peekable();
    while let Some(tt) = tokens.next() {
        // A bare Token in an incant argument list is syntax, not a type.
        // Preserve only that marker; substitute Token in generic arguments,
        // type paths, and nested expressions normally.
        if target == "Token"
            && matches!(&tt, proc_macro2::TokenTree::Ident(id) if id == "incant" || id == "dispatch_variant")
            && matches!(tokens.peek(), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '!')
        {
            result.extend([tt, tokens.next().unwrap()]);
            if let Some(proc_macro2::TokenTree::Group(group)) = tokens.peek() {
                let group = group.clone();
                tokens.next();
                let parser = |input: ParseStream<'_>| -> syn::Result<proc_macro2::TokenStream> {
                    let path: syn::Path = input.parse()?;
                    let args;
                    syn::parenthesized!(args in input);
                    let args = args.parse_terminated(syn::Expr::parse, syn::Token![,])?;
                    let rest: proc_macro2::TokenStream = input.parse()?;
                    let path = replace_ident_in_tokens(path.to_token_stream(), target, replacement);
                    let args = args.iter().map(|arg| {
                        if matches!(arg, syn::Expr::Path(p) if p.path.is_ident("Token")) {
                            arg.to_token_stream()
                        } else {
                            replace_ident_in_tokens(arg.to_token_stream(), target, replacement)
                        }
                    });
                    let rest = replace_ident_in_tokens(rest, target, replacement);
                    Ok(quote! { #path(#(#args),*) #rest })
                };
                use syn::parse::{Parse, Parser};
                let replaced = parser
                    .parse2(group.stream())
                    .unwrap_or_else(|_| group.stream());
                let mut new_group = proc_macro2::Group::new(group.delimiter(), replaced);
                new_group.set_span(group.span());
                result.extend([proc_macro2::TokenTree::Group(new_group)]);
            }
            continue;
        }
        match tt {
            proc_macro2::TokenTree::Ident(ref ident) if *ident == target => {
                result.extend(replacement.clone());
            }
            proc_macro2::TokenTree::Group(group) => {
                let stream = group.stream();
                if !tokens_contain_ident(&stream, &[target]) {
                    result.extend(std::iter::once(proc_macro2::TokenTree::Group(group)));
                    continue;
                }
                let new_stream = replace_ident_in_tokens(stream, target, replacement);
                let mut new_group = proc_macro2::Group::new(group.delimiter(), new_stream);
                new_group.set_span(group.span());
                result.extend(std::iter::once(proc_macro2::TokenTree::Group(new_group)));
            }
            other => {
                result.extend(std::iter::once(other));
            }
        }
    }
    result
}

/// Replace all `Self` identifier tokens with a concrete type in a token stream.
pub(crate) fn replace_self_in_tokens(
    tokens: proc_macro2::TokenStream,
    replacement: &Type,
) -> proc_macro2::TokenStream {
    replace_ident_in_tokens(tokens, "Self", &replacement.to_token_stream())
}

/// Generate import statements for intrinsics and/or magetypes.
pub(crate) fn generate_imports(
    target_arch: Option<&str>,
    magetypes_namespace: Option<&str>,
    import_intrinsics: bool,
    import_magetypes: bool,
) -> proc_macro2::TokenStream {
    let mut imports = proc_macro2::TokenStream::new();

    if import_intrinsics && let Some(arch) = target_arch {
        let arch_ident = quote::format_ident!("{}", arch);
        imports.extend(quote! {
            #[allow(unused_imports)]
            use archmage::intrinsics::#arch_ident::*;
        });
    }

    if import_magetypes && let Some(ns) = magetypes_namespace {
        let ns_ident = quote::format_ident!("{}", ns);
        imports.extend(quote! {
            #[allow(unused_imports)]
            use magetypes::simd::#ns_ident::*;
            #[allow(unused_imports)]
            use magetypes::simd::backends::*;
        });
    }

    imports
}

pub(crate) fn is_bare_ident(expr: &syn::Expr, name: &str) -> bool {
    match expr {
        syn::Expr::Path(p) => {
            p.qself.is_none() && p.path.segments.len() == 1 && p.path.segments[0].ident == name
        }
        _ => false,
    }
}

/// Detect how the token is placed in incant! arguments.
///
/// Returns the kind of token placement found:
/// - `TokenPlacement::Explicit(index)`: arg at `index` is the `Token` marker
/// - `TokenPlacement::Variable(index)`: arg at `index` is the caller's token variable
/// - `TokenPlacement::None`: no token in args (will be prepended, deprecated)
pub(crate) enum TokenPlacement {
    /// `Token` marker found in args
    Explicit,
    /// Caller's token variable name found at this arg index
    Variable(usize),
    /// No token in args — will prepend (deprecated)
    None,
}

/// Find where the token is placed in incant! arguments.
///
/// Checks for:
/// 1. `Token` marker (explicit placeholder for summon mode)
/// 2. A bare ident matching `caller_token_ident` (the actual variable)
///
/// Falls back to `None` (token will be prepended).
pub(crate) fn find_token_placement(
    args: &[syn::Expr],
    caller_token_ident: Option<&str>,
) -> TokenPlacement {
    for arg in args.iter() {
        if is_bare_ident(arg, "Token") {
            return TokenPlacement::Explicit;
        }
    }
    if let Some(ident_name) = caller_token_ident {
        for (i, arg) in args.iter().enumerate() {
            if is_bare_ident(arg, ident_name) {
                return TokenPlacement::Variable(i);
            }
        }
    }
    TokenPlacement::None
}

/// Build call arguments with the token in the correct position.
///
/// Handles three cases:
/// - `Token` marker in args: replace with `token_expr`
/// - Caller's token variable in args: replace with `token_expr` (may downcast)
/// - Neither: prepend `token_expr` (backward compat, deprecated)
pub(crate) fn build_call_args(
    args: &[syn::Expr],
    token_expr: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    build_call_args_with_ident(args, token_expr, None)
}

/// Build call arguments, also checking for the caller's token variable name.
pub(crate) fn build_call_args_with_ident(
    args: &[syn::Expr],
    token_expr: &proc_macro2::TokenStream,
    caller_token_ident: Option<&str>,
) -> proc_macro2::TokenStream {
    match find_token_placement(args, caller_token_ident) {
        TokenPlacement::Explicit => {
            // Replace Token marker with token expression
            let replaced = args
                .iter()
                .map(|arg| replace_ident_in_tokens(arg.to_token_stream(), "Token", token_expr));
            quote! { #(#replaced),* }
        }
        TokenPlacement::Variable(idx) => {
            // Replace the caller's token variable with the target token expression
            let replaced = args.iter().enumerate().map(|(i, arg)| {
                if i == idx {
                    token_expr.clone()
                } else {
                    arg.to_token_stream()
                }
            });
            quote! { #(#replaced),* }
        }
        TokenPlacement::None => {
            // No token in args — prepend (backward compat)
            quote! { #token_expr, #(#args),* }
        }
    }
}

/// Build call arguments for scalar fallback.
///
/// If `Token` marker or caller's token variable is in args, replace with `ScalarToken`.
/// If neither, prepend `ScalarToken`.
pub(crate) fn build_scalar_call_args(args: &[syn::Expr]) -> proc_macro2::TokenStream {
    let scalar = quote! { archmage::ScalarToken };
    build_call_args(args, &scalar)
}

/// Suffix the last segment of a path: `process` → `process_v3`.
pub(crate) fn suffix_path(path: &syn::Path, suffix: &str) -> syn::Path {
    let mut suffixed = path.clone();
    if let Some(last) = suffixed.segments.last_mut() {
        last.ident = quote::format_ident!("{}_{}", last.ident, suffix);
    }
    suffixed
}

/// Emit the tier-trait identity assertion for a trait or generic token bound.
///
/// `#[arcane]`/`#[rite]` resolve a tier trait **by name**: seeing `HasX64V2` in
/// the signature is what selects the SSE4.2 `#[target_feature]` list. On its own
/// that proves nothing, because a downstream crate can declare
///
/// ```ignore
/// pub trait HasX64V2 {}
/// impl HasX64V2 for NotAToken {}
/// ```
///
/// and receive the whole feature set with no token, no sealing and no runtime
/// detection — in ordinary safe code, with no diagnostic. On a CPU without the
/// features that is SIGILL.
///
/// A concrete token is already covered: the wrapper asserts its generated
/// `__ARCHMAGE_ASSERT_TIER_<tag>` const, which a same-named local struct does
/// not have. This is the equivalent for the bound case. It re-states each tier
/// trait through an **absolute** `::archmage::` path, which a local trait cannot
/// shadow, and requires the token value's type to satisfy it:
///
/// ```ignore
/// const fn __archmage_assert_tier_trait<__T: ?Sized + ::archmage::HasX64V2>(_: &__T) {}
/// __archmage_assert_tier_trait(&token);
/// ```
///
/// Because archmage's tier traits are sealed through `SimdToken`, satisfying the
/// bound is only possible for a genuine token of that tier or stronger. A
/// same-named local trait fails with rustc's own diagnostic, which names both:
/// "`impl HasX64V2` implements similarly named trait `HasX64V2`, but not
/// `archmage::HasX64V2`". A real but *weaker* token fails too, closing the
/// trait-bound analogue of the token-aliasing hole.
///
/// It is a `const fn` so it provably has no runtime body, and its parameter is
/// taken by reference so the form works for `impl Trait` in argument position,
/// where the type cannot be named. All bounds are emitted on one helper, so a
/// multi-trait bound (`impl HasNeon + HasNeonAes`) is checked as the same union
/// of tiers the `#[target_feature]` list was built from.
///
/// Returns nothing for a concrete token (`tier_traits` empty) — that path keeps
/// its cheaper const assertion.
/// Returns `true` if `ty` contains an `impl Trait` position anywhere inside it,
/// which Rust forbids in `let pat: ty = ...;` annotations.
pub(crate) fn contains_impl_trait(ty: &Type) -> bool {
    match ty {
        Type::ImplTrait(_) => true,
        Type::Reference(r) => contains_impl_trait(&r.elem),
        Type::Ptr(p) => contains_impl_trait(&p.elem),
        Type::Tuple(t) => t.elems.iter().any(contains_impl_trait),
        Type::Array(a) => contains_impl_trait(&a.elem),
        Type::Slice(s) => contains_impl_trait(&s.elem),
        Type::Paren(p) => contains_impl_trait(&p.elem),
        Type::Group(g) => contains_impl_trait(&g.elem),
        Type::Path(p) => p.path.segments.iter().any(|seg| match &seg.arguments {
            syn::PathArguments::AngleBracketed(ab) => ab.args.iter().any(|arg| {
                matches!(arg, syn::GenericArgument::Type(inner) if contains_impl_trait(inner))
            }),
            syn::PathArguments::Parenthesized(pa) => {
                pa.inputs.iter().any(|arg| contains_impl_trait(&arg.ty))
                    || matches!(&pa.output, syn::ReturnType::Type(_, out) if contains_impl_trait(out))
            }
            syn::PathArguments::None => false,
        }),
        _ => false,
    }
}

/// Emit the compile-time tag assertion for a concrete token type.
pub(crate) fn gen_concrete_token_assertion(
    token_name: &str,
    ty: &Type,
) -> proc_macro2::TokenStream {
    if let Some(expected_tag) = crate::generated::expected_tier_tag(token_name) {
        let assertion = format_ident!("__ARCHMAGE_ASSERT_TIER_{:08X}", expected_tag);
        quote! { let _: () = <#ty>::#assertion; }
    } else {
        quote! {}
    }
}

/// Emit the compile-time tag assertion for `archmage::And<Base, Extra>`.
pub(crate) fn gen_and_token_assertion(ty: &Type) -> proc_macro2::TokenStream {
    let assertion = format_ident!(
        "__ARCHMAGE_ASSERT_AND_{:08X}",
        crate::generated::AND_ASSERT_TAG
    );
    quote! { let _: () = <#ty>::#assertion; }
}

/// Emit a compile-time verification for a tier-trait-bounded value parameter.
pub(crate) fn gen_tier_trait_assertion<T: ToTokens>(
    tier_traits: &[String],
    borrow_target: &T,
) -> proc_macro2::TokenStream {
    if tier_traits.is_empty() {
        return quote! {};
    }
    let bounds = tier_traits.iter().map(|name| {
        let ident = format_ident!("{}", name);
        quote! { + ::archmage::#ident }
    });
    quote! {
        {
            #[inline(always)]
            const fn __archmage_assert_tier_trait<__T: ?Sized #(#bounds)*>(_: &__T) {}
            __archmage_assert_tier_trait(&#borrow_target);
        }
    }
}

/// Emit a compile-time verification for a bounded generic type parameter `T`
/// when it is checked directly by type (for example inside `And<Base, T>`).
pub(crate) fn gen_type_tier_trait_assertion(
    tier_traits: &[String],
    ty: &Type,
) -> proc_macro2::TokenStream {
    if tier_traits.is_empty() {
        return quote! {};
    }
    let bounds = tier_traits.iter().map(|name| {
        let ident = format_ident!("{}", name);
        quote! { + ::archmage::#ident }
    });
    quote! {
        {
            #[inline(always)]
            const fn __archmage_assert_tier_trait<__T: ?Sized #(#bounds)*>() {}
            __archmage_assert_tier_trait::<#ty>();
        }
    }
}

// ============================================================================
// Helpers shared by #[arcane] and #[rite]
// ============================================================================

/// Options that `#[arcane]` and `#[rite]` spell the same way.
///
/// Each macro's parser calls [`parse_shared_option`] first and handles only
/// its own options itself, so the two accept identical spellings for the
/// shared ones and give the same diagnostics.
#[derive(Default)]
pub(crate) struct SharedOptions {
    /// Inject `use archmage::intrinsics::{arch}::*;` (includes safe memory ops).
    pub(crate) import_intrinsics: bool,
    /// Inject `use magetypes::simd::{ns}::*;`, `use magetypes::simd::generic::*;`,
    /// and `use magetypes::simd::backends::*;`.
    pub(crate) import_magetypes: bool,
    /// Additional cargo feature gate: `cfg(avx512)` adds `feature = "avx512"`
    /// to the generated `#[cfg(...)]`.
    pub(crate) cfg_feature: Option<String>,
}

/// Parse one of the shared options. Returns `Ok(true)` when `ident` named one.
pub(crate) fn parse_shared_option(
    ident: &Ident,
    input: ParseStream<'_>,
    opts: &mut SharedOptions,
) -> syn::Result<bool> {
    match ident.to_string().as_str() {
        "import_intrinsics" => opts.import_intrinsics = true,
        "import_magetypes" => opts.import_magetypes = true,
        "cfg" => {
            let content;
            syn::parenthesized!(content in input);
            let feat: Ident = content.parse()?;
            opts.cfg_feature = Some(feat.to_string());
        }
        "stub" => {
            return Err(syn::Error::new(
                ident.span(),
                "`stub` has been removed. Use `incant!` for cross-arch dispatch \
                 instead — it cfg-gates each architecture automatically.\n\
                 \n\
                 Before: #[arcane(stub)] fn process(token: X64V3Token, ...) { ... }\n\
                 After:  #[arcane] fn process_v3(token: X64V3Token, ...) { ... }\n\
                 \x20       fn dispatch(...) { incant!(process(...)) }",
            ));
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/// The error for `import_intrinsics` with AVX-512 features when archmage was
/// built without its `avx512` feature: the 512-bit safe memory wrappers are
/// missing, so `_mm512_loadu_ps` would resolve to the pointer-taking intrinsic.
#[cfg(not(feature = "avx512"))]
pub(crate) fn avx512_import_error(
    sig: &Signature,
    import_intrinsics: bool,
    features: &[&str],
    token_desc: &str,
) -> Option<proc_macro2::TokenStream> {
    if !import_intrinsics || !features.iter().any(|f| f.starts_with("avx512")) {
        return None;
    }
    let msg = format!(
        "Using {token_desc} with `import_intrinsics` requires the `avx512` feature.\n\
         \n\
         Add to your Cargo.toml:\n\
         \x20 archmage = {{ version = \"...\", features = [\"avx512\"] }}\n\
         \n\
         Without it, 512-bit safe memory ops (_mm512_loadu_ps etc.) are not available.\n\
         If you only need value intrinsics (no memory ops), remove `import_intrinsics`."
    );
    Some(syn::Error::new_spanned(sig, msg).to_compile_error())
}

#[cfg(feature = "avx512")]
pub(crate) fn avx512_import_error(
    _sig: &Signature,
    _import_intrinsics: bool,
    _features: &[&str],
    _token_desc: &str,
) -> Option<proc_macro2::TokenStream> {
    None
}

/// The error when a signature has no usable token parameter. A featureless
/// bound such as `SimdToken` gets its own explanation; `forms` lists the
/// macro's accepted spellings.
pub(crate) fn missing_token_error(
    sig: &Signature,
    macro_name: &str,
    forms: &str,
) -> proc_macro2::TokenStream {
    if let Some(trait_name) = crate::token_discovery::diagnose_featureless_token(sig) {
        let msg = format!(
            "`{trait_name}` cannot be used as a token bound in #[{macro_name}] \
             because it doesn't specify any CPU features.\n\
             \n\
             #[{macro_name}] needs concrete features to generate #[target_feature]. \
             Use a concrete token or a feature trait:\n\
             \n\
             Concrete tokens: X64V3Token, Desktop64, NeonToken, Arm64V2Token, ...\n\
             Feature traits:  impl HasX64V2, impl HasNeon, impl HasArm64V3, ...{}",
            if macro_name == "rite" {
                "\nTier names:      #[rite(v3)], #[rite(neon)], #[rite(v4)], ..."
            } else {
                ""
            }
        );
        return syn::Error::new_spanned(sig, msg).to_compile_error();
    }
    let msg = format!("{macro_name} requires a token parameter{forms}");
    syn::Error::new_spanned(sig, msg).to_compile_error()
}

/// Rename every non-identifier parameter pattern to a generated identifier,
/// so the parameter can be forwarded by name, and return the statements that
/// re-bind the original pattern inside the body.
///
/// `_: T` becomes `__archmage_arg_0: T` with no re-binding (a wildcard binds
/// nothing). `(a, b): (T, U)` becomes `__archmage_arg_1: (T, U)` plus
/// `let (a, b): (T, U) = __archmage_arg_1;`. The type annotation is left off
/// when the type is `impl Trait`, which a `let` cannot name.
/// True iff `pat` is a plain identifier with no `ref`, `mut`, or `@ subpat` modifiers.
pub(crate) fn is_plain_ident(pat: &syn::Pat) -> bool {
    matches!(
        pat,
        syn::Pat::Ident(p)
            if p.by_ref.is_none() && p.mutability.is_none() && p.subpat.is_none()
    )
}

/// Replace any non-plain-identifier parameter patterns (`_: T`, `(a, b): (T, U)`,
/// `ref x: T`, `mut x: T`, `x @ pat: T`) with generated identifiers
/// (`__archmage_arg_0`, `__archmage_arg_1`, ...) so the parameter can be
/// forwarded by name, and return the statements that re-bind the original
/// pattern inside the body.
pub(crate) fn rename_non_ident_params(sig: &mut Signature) -> Vec<proc_macro2::TokenStream> {
    let mut rebinds = Vec::new();
    let mut counter = 0u32;
    for arg in &mut sig.inputs {
        let syn::FnArg::Typed(pat_type) = arg else {
            continue;
        };
        if is_plain_ident(pat_type.pat.as_ref()) {
            continue;
        }
        let generated = format_ident!("__archmage_arg_{}", counter);
        counter += 1;
        let original_pat = pat_type.pat.clone();
        let ty = &pat_type.ty;
        if !matches!(original_pat.as_ref(), syn::Pat::Wild(_)) {
            rebinds.push(if contains_impl_trait(ty) {
                quote! { let #original_pat = #generated; }
            } else {
                quote! { let #original_pat: #ty = #generated; }
            });
        }
        *pat_type.pat = syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: generated,
            subpat: None,
        });
    }
    rebinds
}

/// Forwarding layers (wrappers, proof wrappers, dispatchers) use every
/// parameter, so an author's `_name` (conventionally "unused") would trip
/// `clippy::used_underscore_binding` in them. Their bodies are generated, so
/// such a parameter takes a reserved name instead (`_t` becomes
/// `__archmage_t`; clippy exempts `__`-prefixed names). No rebinding: only
/// the forwarding call reads it. Feature bodies keep the author's names.
pub(crate) fn rename_underscore_params(sig: &mut Signature) {
    for arg in &mut sig.inputs {
        if let syn::FnArg::Typed(pat_type) = arg
            && let syn::Pat::Ident(pat) = pat_type.pat.as_mut()
        {
            let name = pat.ident.to_string();
            if name.starts_with('_') && !name.starts_with("__") && name.len() > 1 {
                pat.ident = format_ident!("__archmage{}", name, span = pat.ident.span());
            }
        }
    }
}

/// Give a wildcard token parameter (`_: X64V3Token`) the name
/// `find_token_param` synthesizes for it, so the nested dispatch rewrite and
/// the tier assertions can refer to it; every other pattern is left as
/// written. Returns the name when something was renamed.
pub(crate) fn rename_wildcard_token(sig: &mut Signature) -> Option<Ident> {
    let template = sig.clone();
    for arg in &mut sig.inputs {
        let syn::FnArg::Typed(pat_type) = arg else {
            continue;
        };
        if !matches!(pat_type.pat.as_ref(), syn::Pat::Wild(_)) {
            continue;
        }
        // Probe this parameter alone, with the function's generics so a
        // `T: HasX64V2` bound still resolves.
        let probe = Signature {
            inputs: std::iter::once(syn::FnArg::Typed(pat_type.clone())).collect(),
            ..template.clone()
        };
        let Ok(Some(proofs)) = crate::token_discovery::discover_signature_proofs(&probe) else {
            continue;
        };
        let ident = proofs.primary_ident;
        *pat_type.pat = syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: ident.clone(),
            subpat: None,
        });
        return Some(ident);
    }
    None
}

/// Normalize non-plain parameter patterns for single-function emitters
/// (`#[rite]`, `wasm32` `#[arcane]`, `ScalarToken` `#[arcane]`) on the
/// parameters indexed in `param_indices`, so bound assertions can borrow or
/// project them safely before restoring the author's bindings inside the body.
///
/// Single wildcard token parameters (`_: X64V3Token`) still become `__archmage_token`
/// (with no extra rebind) to match historical single-token snapshots.
pub(crate) fn normalize_single_fn_token_params(
    sig: &mut Signature,
    param_indices: &[usize],
) -> Vec<proc_macro2::TokenStream> {
    let mut rebinds = Vec::new();
    let mut typed_idx = 0usize;
    let mut counter = 0u32;
    let single_wildcard = param_indices.len() == 1
        && sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(pt) => Some(pt),
                syn::FnArg::Receiver(_) => None,
            })
            .nth(param_indices[0])
            .is_some_and(|pt| matches!(pt.pat.as_ref(), syn::Pat::Wild(_)));

    for arg in &mut sig.inputs {
        let syn::FnArg::Typed(pat_type) = arg else {
            continue;
        };
        let idx = typed_idx;
        typed_idx += 1;
        if !param_indices.contains(&idx) || is_plain_ident(pat_type.pat.as_ref()) {
            continue;
        }
        let generated = if single_wildcard && matches!(pat_type.pat.as_ref(), syn::Pat::Wild(_)) {
            format_ident!("__archmage_token")
        } else {
            let ident = format_ident!("__archmage_arg_{}", counter);
            counter += 1;
            ident
        };
        let original_pat = pat_type.pat.clone();
        let ty = &pat_type.ty;
        if !matches!(original_pat.as_ref(), syn::Pat::Wild(_)) {
            rebinds.push(if contains_impl_trait(ty) {
                quote! { let #original_pat = #generated; }
            } else {
                quote! { let #original_pat: #ty = #generated; }
            });
        }
        *pat_type.pat = syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: generated,
            subpat: None,
        });
    }
    rebinds
}

/// Prepend statements to a function body block.
pub(crate) fn prepend_to_block(block: &mut syn::Block, prefix: proc_macro2::TokenStream) {
    if prefix.is_empty() {
        return;
    }
    let stmts: Vec<syn::Stmt> = syn::parse2::<syn::Block>(quote! { { #prefix } })
        .map(|b| b.stmts)
        .unwrap_or_else(|_| vec![syn::Stmt::Item(syn::Item::Verbatim(prefix))]);
    let existing = std::mem::take(&mut block.stmts);
    block.stmts = stmts;
    block.stmts.extend(existing);
}

/// Extract the statement tokens inside a `syn::Block` (without outer braces).
pub(crate) fn block_inner_tokens(block: &syn::Block) -> proc_macro2::TokenStream {
    let stmts = &block.stmts;
    quote! { #(#stmts)* }
}

/// Replace the statements inside a `syn::Block` by parsing `inner` within braces.
pub(crate) fn set_block_inner_tokens(
    block: &mut syn::Block,
    inner: proc_macro2::TokenStream,
) -> syn::Result<()> {
    let parsed: syn::Block = syn::parse2(quote! { { #inner } })?;
    block.stmts = parsed.stmts;
    Ok(())
}

/// The `_self` parameter a nested function takes in place of a receiver,
/// with the receiver's own shape kept: `&'a self` becomes `_self: &'a Type`,
/// `&mut self` becomes `_self: &mut Type`, `self: Box<Self>` becomes
/// `_self: Box<Type>`, and `self` / `mut self` become `_self: Type`.
pub(crate) fn nested_self_param(receiver: &syn::Receiver, self_ty: &Type) -> syn::FnArg {
    let ty: proc_macro2::TokenStream = match &receiver.kind {
        syn::ReceiverKind::Value => quote!(#self_ty),
        syn::ReceiverKind::Reference(_, lifetime, mutability) => {
            quote!(& #lifetime #mutability #self_ty)
        }
        syn::ReceiverKind::Typed(_, ty) => replace_self_in_tokens(ty.to_token_stream(), self_ty),
        // `#[non_exhaustive]`: a receiver shape syn adds later is passed by value.
        _ => quote!(#self_ty),
    };
    syn::parse_quote!(_self: #ty)
}

/// Replace the value `self` with another identifier, leaving `self::` paths
/// alone. Used when a method body moves into a nested function that receives
/// the receiver as `_self`.
pub(crate) fn replace_self_value_in_tokens(
    tokens: proc_macro2::TokenStream,
    replacement: &Ident,
) -> proc_macro2::TokenStream {
    let mut result = proc_macro2::TokenStream::new();
    let mut tokens = tokens.into_iter().peekable();
    // A nested `impl` or `trait` item has receivers of its own: once one of
    // those keywords is seen, the next brace group is that item's body and is
    // copied untouched. Closures and blocks still refer to the outer `self`
    // and are rewritten.
    let mut in_item_header = false;
    while let Some(tt) = tokens.next() {
        match tt {
            proc_macro2::TokenTree::Ident(ref ident) if *ident == "impl" || *ident == "trait" => {
                in_item_header = true;
                result.extend([tt]);
            }
            proc_macro2::TokenTree::Ident(ref ident) if *ident == "self" => {
                let is_path = matches!(tokens.peek(), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == ':');
                if is_path {
                    result.extend([tt]);
                } else {
                    let mut replaced = replacement.clone();
                    replaced.set_span(ident.span());
                    result.extend([proc_macro2::TokenTree::Ident(replaced)]);
                }
            }
            proc_macro2::TokenTree::Group(group)
                if in_item_header && group.delimiter() == proc_macro2::Delimiter::Brace =>
            {
                in_item_header = false;
                result.extend([proc_macro2::TokenTree::Group(group)]);
            }
            proc_macro2::TokenTree::Group(group) => {
                let inner = replace_self_value_in_tokens(group.stream(), replacement);
                let mut new_group = proc_macro2::Group::new(group.delimiter(), inner);
                new_group.set_span(group.span());
                result.extend([proc_macro2::TokenTree::Group(new_group)]);
            }
            other => result.extend([other]),
        }
    }
    result
}

#[cfg(test)]
mod dispatch_marker_tests {
    use super::*;

    #[test]
    fn substitutes_types_but_preserves_dispatch_markers() {
        let source = quote! {
            fn f(t: Token) {
                archmage::incant!(callee::<Token>(Token, Token::from_context(), wrap::<Token>()), [scalar]);
                dispatch_variant!(other(value, Token) with t);
            }
        };
        let expected = quote! {
            fn f(t: archmage::ScalarToken) {
                archmage::incant!(callee::<archmage::ScalarToken>(Token, archmage::ScalarToken::from_context(), wrap::<archmage::ScalarToken>()), [scalar]);
                dispatch_variant!(other(value, Token) with t);
            }
        };
        assert_eq!(
            replace_ident_in_tokens(source, "Token", &quote!(archmage::ScalarToken)).to_string(),
            expected.to_string()
        );
    }
}
