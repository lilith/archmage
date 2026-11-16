//! Token parameter, conjunction, and capability discovery across function signatures.

use crate::common::{
    gen_and_token_assertion, gen_concrete_token_assertion, gen_tier_trait_assertion,
    gen_type_tier_trait_assertion,
};
use crate::generated::{
    canonical_token_to_tier_suffix, token_requires_runtime_probe, token_to_arch, token_to_features,
    token_to_features_csv, token_to_magetypes_namespace, trait_requires_runtime_probe,
    trait_to_arch, trait_to_features, trait_to_magetypes_namespace,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;
use syn::{GenericArgument, Ident, PathArguments, PredicateType, Signature, Type, TypeParamBound};

/// Archmage marker traits that carry no CPU target features of their own.
pub(crate) const FEATURELESS_TRAIT_NAMES: &[&str] = &[
    "SimdToken",
    "SimdLeafToken",
    "SameArch",
    "IntoConcreteToken",
];

/// Find the first featureless archmage marker trait in `trait_names`.
pub(crate) fn find_featureless_trait(trait_names: &[String]) -> Option<&'static str> {
    for name in trait_names {
        if let Some(&hit) = FEATURELESS_TRAIT_NAMES.iter().find(|&&f| f == name) {
            return Some(hit);
        }
    }
    None
}

/// What kind of token type a parameter directly has.
///
/// Kept for legacy callers (`attune::parent`, `magetypes`) that only inspect a
/// single parameter's outer shape.
#[derive(Debug)]
pub(crate) enum TokenTypeInfo {
    /// Concrete token type (e.g., `X64V3Token`, `NeonToken`, `Wasm128Token`).
    Concrete(String),
    /// `impl Trait` with one or more trait bounds (e.g., `impl HasX64V2 + HasNeon`).
    ImplTrait(Vec<String>),
    /// Bounded generic type parameter (e.g., `T` from `fn foo<T: HasX64V2>(token: T)`).
    Generic(String),
}

fn peel_paren_group(mut ty: &Type) -> &Type {
    loop {
        match ty {
            Type::Paren(p) => ty = &p.elem,
            Type::Group(g) => ty = &g.elem,
            _ => return ty,
        }
    }
}

fn extract_traits_from_bounds<'a>(
    bounds: impl IntoIterator<Item = &'a TypeParamBound>,
) -> Vec<String> {
    let mut traits = Vec::new();
    for bound in bounds {
        if let TypeParamBound::Trait(trait_bound) = bound
            && let Some(segment) = trait_bound.path.segments.last()
        {
            traits.push(segment.ident.to_string());
        }
    }
    traits
}

/// Extract token type info from a single parameter type (unwrapping references).
pub(crate) fn extract_token_type_info(ty: &Type) -> Option<TokenTypeInfo> {
    let ty = peel_paren_group(ty);
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => {
            let last = type_path.path.segments.last()?;
            let name = last.ident.to_string();
            if last.arguments.is_empty() && token_to_features(&name).is_some() {
                Some(TokenTypeInfo::Concrete(name))
            } else if type_path.path.segments.len() == 1 && last.arguments.is_empty() {
                Some(TokenTypeInfo::Generic(name))
            } else {
                None
            }
        }
        Type::Reference(type_ref) => extract_token_type_info(&type_ref.elem),
        Type::ImplTrait(impl_trait) => {
            let traits = extract_traits_from_bounds(&impl_trait.bounds);
            if traits.is_empty() {
                None
            } else {
                Some(TokenTypeInfo::ImplTrait(traits))
            }
        }
        _ => None,
    }
}

/// Find trait bounds for a generic type parameter in `sig` (both inline and `where` clause).
pub(crate) fn find_generic_bounds(sig: &Signature, type_name: &str) -> Option<Vec<String>> {
    let mut bounds = Vec::new();

    for param in &sig.generics.params {
        if let syn::GenericParam::Type(type_param) = param
            && type_param.ident == type_name
        {
            bounds.extend(extract_traits_from_bounds(&type_param.bounds));
        }
    }

    if let Some(where_clause) = &sig.generics.where_clause {
        for predicate in &where_clause.predicates {
            if let syn::WherePredicate::Type(PredicateType {
                bounded_ty: Type::Path(type_path),
                bounds: pred_bounds,
                ..
            }) = predicate
                && type_path.qself.is_none()
                && let Some(ident) = type_path.path.get_ident()
                && ident == type_name
            {
                bounds.extend(extract_traits_from_bounds(pred_bounds));
            }
        }
    }

    if bounds.is_empty() {
        None
    } else {
        Some(bounds)
    }
}

/// Map a set of trait names to the union of their target features.
///
/// Ignores known featureless/std marker traits when combined with archmage
/// traits, and returns `None` if no known archmage capability trait is present
/// or if traits from conflicting architectures are mixed.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn traits_to_features(trait_names: &[String]) -> Option<Cow<'static, [&'static str]>> {
    let known: Vec<&str> = trait_names
        .iter()
        .map(String::as_str)
        .filter(|name| trait_to_features(name).is_some())
        .collect();

    if known.is_empty() {
        return None;
    }

    // Reject conflicting architectures within known traits.
    let mut arch: Option<&'static str> = None;
    for name in &known {
        let a = trait_to_arch(name)?;
        match arch {
            Some(prev) if prev != a => return None,
            None => arch = Some(a),
            _ => {}
        }
    }

    if known.iter().all(|&n| n == known[0])
        && let Some(features) = trait_to_features(known[0])
    {
        return Some(Cow::Borrowed(features));
    }

    let mut combined: Vec<&'static str> = Vec::new();
    for name in &known {
        if let Some(features) = trait_to_features(name) {
            for &feat in features {
                if !combined.contains(&feat) {
                    combined.push(feat);
                }
            }
        }
    }

    if combined.is_empty() {
        None
    } else {
        Some(Cow::Owned(combined))
    }
}

/// Architecture classification of a token proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArchClass {
    Scalar,
    Target(&'static str),
}

impl std::fmt::Display for ArchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchClass::Scalar => write!(f, "Scalar"),
            ArchClass::Target(arch) => write!(f, "{arch}"),
        }
    }
}

/// One step projecting from a typed function parameter into a token leaf.
#[derive(Clone, Debug)]
pub(crate) enum PlaceStep {
    Deref,
    TupleIndex(usize),
}

/// Location of a value-backed proof inside a function parameter.
#[derive(Clone, Debug)]
pub(crate) struct ParamPlace {
    pub(crate) is_receiver: bool,
    /// Index among typed (`FnArg::Typed`) parameters in `sig.inputs`.
    pub(crate) param_idx: usize,
    pub(crate) steps: Vec<PlaceStep>,
}

impl ParamPlace {
    /// Render the place expression referencing the normalized parameter binding in `sig`.
    pub(crate) fn borrow_expr(&self, sig: &Signature) -> Option<TokenStream> {
        let mut expr = if self.is_receiver {
            match sig.inputs.first() {
                Some(syn::FnArg::Receiver(_)) => quote!(self),
                _ => return None,
            }
        } else {
            let ident = sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Typed(pt) => Some(pt),
                    syn::FnArg::Receiver(_) => None,
                })
                .nth(self.param_idx)
                .and_then(|pt| match pt.pat.as_ref() {
                    syn::Pat::Ident(pi) => Some(pi.ident.clone()),
                    _ => None,
                })?;
            quote!(#ident)
        };

        let mut wrap_for_postfix = false;
        for step in &self.steps {
            match step {
                PlaceStep::Deref => {
                    expr = quote!(*#expr);
                    wrap_for_postfix = true;
                }
                PlaceStep::TupleIndex(i) => {
                    let idx = syn::Index::from(*i);
                    expr = if wrap_for_postfix {
                        quote!((#expr).#idx)
                    } else {
                        quote!(#expr.#idx)
                    };
                    wrap_for_postfix = false;
                }
            }
        }
        Some(expr)
    }
}

/// Compile-time assertion for a trait-bounded proof (`impl Has*` or `T: Has*`).
#[derive(Clone)]
pub(crate) enum BoundAssertion {
    ByRef {
        place: ParamPlace,
        traits: Vec<String>,
    },
    ByType {
        ty: Box<Type>,
        traits: Vec<String>,
    },
}

/// Metadata preserved when the entire proof is a single concrete token type.
#[derive(Clone, Debug)]
pub(crate) struct SingleConcreteToken {
    pub(crate) type_name: String,
    pub(crate) direct_value_param: bool,
}

/// Complete capability proof extracted from a function signature.
#[derive(Clone)]
pub(crate) struct SignatureProofs {
    /// Primary parameter ident (from first proof param, or `__archmage_token` if wildcard/destructured).
    pub(crate) primary_ident: Ident,
    /// Union of required target features, deduplicated in encounter order.
    pub(crate) features: Vec<&'static str>,
    /// Target architecture (`Some("x86_64" | "aarch64" | "wasm32")` or `None` for scalar-only).
    pub(crate) target_arch: Option<&'static str>,
    /// True when the signature uses `ScalarToken`.
    pub(crate) is_scalar: bool,
    /// Combined magetypes namespace (`"v3"`, `"v4"`, `"v4x"`, `"neon"`, `"wasm128"`, `"scalar"`).
    pub(crate) magetypes_namespace: Option<&'static str>,
    /// Single concrete token metadata when there is only one simple concrete proof.
    pub(crate) single_concrete: Option<SingleConcreteToken>,
    /// Every concrete token leaf `(type_name, stripped_type)` for `__ARCHMAGE_ASSERT_TIER_*`.
    pub(crate) concrete_assertions: Vec<(String, Type)>,
    /// Every `And<Base, Extra>` type for `__ARCHMAGE_ASSERT_AND_*`.
    pub(crate) and_assertions: Vec<Type>,
    /// Every `impl Has*` or `T: Has*` bound requiring `::archmage::Has*` checking.
    pub(crate) bound_assertions: Vec<BoundAssertion>,
    /// Unique concrete leaf token names (for runtime-probe enforcement).
    pub(crate) all_concrete_names: Vec<String>,
    /// Unique capability trait names (for runtime-probe enforcement).
    pub(crate) all_trait_names: Vec<String>,
    /// Sorted, deduplicated indices among `FnArg::Typed` inputs that contain token proofs.
    pub(crate) token_param_indices: Vec<usize>,
}

impl SignatureProofs {
    /// Tier suffix for single concrete token proofs (preserving `incant!` contextual dispatch).
    pub(crate) fn single_tier_suffix(&self) -> Option<&'static str> {
        self.single_concrete
            .as_ref()
            .filter(|c| c.direct_value_param)
            .and_then(|c| canonical_token_to_tier_suffix(&c.type_name))
    }

    /// Single concrete token type name, if applicable (for `features_csv`).
    pub(crate) fn single_concrete_name(&self) -> Option<&str> {
        self.single_concrete.as_ref().map(|c| c.type_name.as_str())
    }

    /// True if any concrete leaf token or trait bound requires a runtime semantic probe.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn requires_runtime_probe(&self) -> bool {
        self.all_concrete_names
            .iter()
            .any(|name| token_requires_runtime_probe(name))
            || self
                .all_trait_names
                .iter()
                .any(|name| trait_requires_runtime_probe(name))
    }

    /// Re-target parameter-based trait assertions to the `self` receiver when
    /// proofs were discovered from `_self = ...` rather than a typed parameter.
    pub(crate) fn mark_receiver_source(&mut self) {
        for bound in &mut self.bound_assertions {
            if let BoundAssertion::ByRef { place, .. } = bound {
                place.is_receiver = true;
            }
        }
    }

    /// Generate all compile-time tag, `And`, and `::archmage::Has*` assertions for `sig`.
    pub(crate) fn gen_assertions(&self, sig: &Signature) -> TokenStream {
        let mut out = TokenStream::new();
        for and_ty in &self.and_assertions {
            out.extend(gen_and_token_assertion(and_ty));
        }
        for (name, ty) in &self.concrete_assertions {
            out.extend(gen_concrete_token_assertion(name, ty));
        }
        for bound in &self.bound_assertions {
            match bound {
                BoundAssertion::ByRef { place, traits } => {
                    if let Some(borrow_expr) = place.borrow_expr(sig) {
                        out.extend(gen_tier_trait_assertion(traits, &borrow_expr));
                    }
                }
                BoundAssertion::ByType { ty, traits } => {
                    out.extend(gen_type_tier_trait_assertion(traits, ty));
                }
            }
        }
        out
    }
}

#[derive(Clone)]
struct ProofAccumulator {
    arch: Option<ArchClass>,
    features: Vec<&'static str>,
    magetypes_namespace: Option<&'static str>,
    concrete_assertions: Vec<(String, Type)>,
    and_assertions: Vec<Type>,
    bound_assertions: Vec<BoundAssertion>,
    all_concrete_names: Vec<String>,
    all_trait_names: Vec<String>,
    token_param_indices: Vec<usize>,
    primary_ident: Option<Ident>,
    first_leaf_had_wrapper: bool,
    leaf_count: usize,
}

impl ProofAccumulator {
    fn new() -> Self {
        Self {
            arch: None,
            features: Vec::new(),
            magetypes_namespace: None,
            concrete_assertions: Vec::new(),
            and_assertions: Vec::new(),
            bound_assertions: Vec::new(),
            all_concrete_names: Vec::new(),
            all_trait_names: Vec::new(),
            token_param_indices: Vec::new(),
            primary_ident: None,
            first_leaf_had_wrapper: false,
            leaf_count: 0,
        }
    }

    fn merge_arch(&mut self, incoming: ArchClass, span_ty: &Type) -> syn::Result<()> {
        match self.arch {
            Some(prev) if prev != incoming => Err(syn::Error::new_spanned(
                span_ty,
                format!(
                    "cannot combine SIMD tokens from different architectures ({prev} and {incoming}) in the same function"
                ),
            )),
            None => {
                self.arch = Some(incoming);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn merge_features(&mut self, feats: &[&'static str]) {
        for &feat in feats {
            if !self.features.contains(&feat) {
                self.features.push(feat);
            }
        }
    }

    fn merge_namespace(&mut self, next: Option<&'static str>) {
        self.magetypes_namespace = match (self.magetypes_namespace, next) {
            (Some("v4x"), _) | (_, Some("v4x")) => Some("v4x"),
            (Some("v4"), _) | (_, Some("v4")) => Some("v4"),
            (Some("v3"), _) | (_, Some("v3")) => Some("v3"),
            (cur, nxt) => cur.or(nxt),
        };
    }

    fn push_concrete_name(&mut self, name: &str) {
        if !self.all_concrete_names.iter().any(|n| n == name) {
            self.all_concrete_names.push(name.to_string());
        }
    }

    fn push_trait_names(&mut self, names: &[String]) {
        for name in names {
            if !self.all_trait_names.iter().any(|n| n == name) {
                self.all_trait_names.push(name.clone());
            }
        }
    }
}

fn path_idents(path: &syn::Path) -> Vec<String> {
    path.segments.iter().map(|s| s.ident.to_string()).collect()
}

fn is_and_path(path: &syn::Path) -> Option<(&Type, &Type)> {
    let segs = path_idents(path);
    let refs: Vec<&str> = segs.iter().map(|s| s.as_str()).collect();
    let is_candidate = if path.leading_colon.is_some() {
        matches!(
            refs.as_slice(),
            ["archmage", "And"] | ["archmage", "tokens", "And"]
        )
    } else {
        matches!(
            refs.as_slice(),
            ["And"] | ["archmage", "And"] | ["archmage", "tokens", "And"]
        )
    };
    if !is_candidate {
        return None;
    }

    let last = path.segments.last()?;
    let PathArguments::AngleBracketed(ab) = &last.arguments else {
        return None;
    };
    let mut type_args = ab.args.iter().filter_map(|arg| match arg {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    });
    let left = type_args.next()?;
    let right = type_args.next()?;
    if ab.args.len() != 2 || type_args.next().is_some() {
        return None;
    }
    Some((left, right))
}

fn is_explicit_archmage_and(path: &syn::Path) -> bool {
    let segs = path_idents(path);
    let refs: Vec<&str> = segs.iter().map(|s| s.as_str()).collect();
    matches!(
        refs.as_slice(),
        ["archmage", "And"] | ["archmage", "tokens", "And"]
    )
}

fn arch_for_concrete_token(name: &str) -> ArchClass {
    if name == "ScalarToken" {
        ArchClass::Scalar
    } else if let Some(arch) = token_to_arch(name) {
        ArchClass::Target(arch)
    } else {
        ArchClass::Scalar
    }
}

fn discover_in_type(
    ty: &Type,
    sig: &Signature,
    param_idx: usize,
    steps: Vec<PlaceStep>,
    in_and: bool,
    acc: &mut ProofAccumulator,
) -> syn::Result<bool> {
    let ty = peel_paren_group(ty);
    match ty {
        Type::Reference(type_ref) if !in_and => {
            let mut next_steps = steps;
            next_steps.push(PlaceStep::Deref);
            discover_in_type(&type_ref.elem, sig, param_idx, next_steps, false, acc)
        }
        Type::Tuple(tuple) if !in_and => {
            if tuple.elems.is_empty() {
                return Ok(false);
            }
            let mut any_found = false;
            for (i, elem) in tuple.elems.iter().enumerate() {
                let mut next_steps = steps.clone();
                next_steps.push(PlaceStep::TupleIndex(i));
                any_found |= discover_in_type(elem, sig, param_idx, next_steps, false, acc)?;
            }
            Ok(any_found)
        }
        Type::Path(type_path) if type_path.qself.is_none() => {
            let Some(last) = type_path.path.segments.last() else {
                return Ok(false);
            };

            if let Some((left_ty, right_ty)) = is_and_path(&type_path.path) {
                let mut scratch = acc.clone();
                let left_ok =
                    discover_in_type(left_ty, sig, param_idx, steps.clone(), true, &mut scratch)?;
                let right_ok =
                    discover_in_type(right_ty, sig, param_idx, steps, true, &mut scratch)?;

                if left_ok && right_ok {
                    *acc = scratch;
                    acc.and_assertions.push(ty.clone());
                    return Ok(true);
                }
                if left_ok || right_ok || in_and || is_explicit_archmage_and(&type_path.path) {
                    return Err(syn::Error::new_spanned(
                        ty,
                        "`archmage::And<Base, Extra>` requires both type arguments to be valid \
                         archmage tokens or capability-bounded generic tokens",
                    ));
                }
                return Ok(false);
            }

            if !last.arguments.is_empty() {
                return Ok(false);
            }

            let name = last.ident.to_string();
            if let Some(features) = token_to_features(&name) {
                let incoming_arch = arch_for_concrete_token(&name);
                acc.merge_arch(incoming_arch, ty)?;
                acc.merge_features(features);
                acc.merge_namespace(token_to_magetypes_namespace(&name));
                acc.push_concrete_name(&name);
                acc.concrete_assertions.push((name, ty.clone()));
                if acc.leaf_count == 0 {
                    acc.first_leaf_had_wrapper = !steps.is_empty();
                }
                acc.leaf_count += 1;
                return Ok(true);
            }

            if type_path.path.segments.len() == 1
                && let Some(bounds) = find_generic_bounds(sig, &name)
            {
                return record_trait_bounds(&bounds, Some(ty), param_idx, steps, in_and, acc, ty);
            }

            Ok(false)
        }
        Type::ImplTrait(impl_trait) if !in_and => {
            let bounds = extract_traits_from_bounds(&impl_trait.bounds);
            if bounds.is_empty() {
                return Ok(false);
            }
            record_trait_bounds(&bounds, None, param_idx, steps, false, acc, ty)
        }
        _ => Ok(false),
    }
}

fn record_trait_bounds(
    bounds: &[String],
    generic_ty: Option<&Type>,
    param_idx: usize,
    steps: Vec<PlaceStep>,
    in_and: bool,
    acc: &mut ProofAccumulator,
    span_ty: &Type,
) -> syn::Result<bool> {
    let arch_traits: Vec<String> = bounds
        .iter()
        .filter(|name| trait_to_features(name).is_some())
        .cloned()
        .collect();

    if arch_traits.is_empty() {
        return Ok(false);
    }

    for trait_name in &arch_traits {
        let Some(arch) = trait_to_arch(trait_name) else {
            return Ok(false);
        };
        acc.merge_arch(ArchClass::Target(arch), span_ty)?;
        if let Some(features) = trait_to_features(trait_name) {
            acc.merge_features(features);
        }
        acc.merge_namespace(trait_to_magetypes_namespace(trait_name));
    }

    acc.push_trait_names(&arch_traits);
    if !in_and && steps.is_empty() {
        acc.bound_assertions.push(BoundAssertion::ByRef {
            place: ParamPlace {
                is_receiver: false,
                param_idx,
                steps,
            },
            traits: arch_traits,
        });
    } else if let Some(ty) = generic_ty {
        acc.bound_assertions.push(BoundAssertion::ByType {
            ty: Box::new(ty.clone()),
            traits: arch_traits,
        });
    } else {
        acc.bound_assertions.push(BoundAssertion::ByRef {
            place: ParamPlace {
                is_receiver: false,
                param_idx,
                steps,
            },
            traits: arch_traits,
        });
    }
    if acc.leaf_count == 0 {
        acc.first_leaf_had_wrapper = true;
    }
    acc.leaf_count += 1;
    Ok(true)
}

/// Discover all SIMD capability proofs in `sig`.
///
/// Supports concrete tokens, `&T`/`&mut T`, tuples `(A, B)`, `And<Base, Extra>`,
/// multi-parameter conjunctions, `impl Has*`, and generic parameters bounded in
/// inline bounds or `where` clauses. Rejects cross-architecture or scalar+arch
/// combinations and invalid `And<Base, Extra>` arguments at macro expansion time.
pub(crate) fn discover_signature_proofs(sig: &Signature) -> syn::Result<Option<SignatureProofs>> {
    let mut acc = ProofAccumulator::new();
    let mut typed_idx = 0usize;

    for arg in &sig.inputs {
        let syn::FnArg::Typed(pat_type) = arg else {
            continue;
        };
        let idx = typed_idx;
        typed_idx += 1;

        if discover_in_type(&pat_type.ty, sig, idx, Vec::new(), false, &mut acc)? {
            acc.token_param_indices.push(idx);
            if acc.primary_ident.is_none() {
                acc.primary_ident = match pat_type.pat.as_ref() {
                    syn::Pat::Ident(pat_ident) => Some(pat_ident.ident.clone()),
                    _ => Some(format_ident!("__archmage_token")),
                };
            }
        }
    }

    let Some(arch_class) = acc.arch else {
        return Ok(None);
    };

    let (is_scalar, target_arch) = match arch_class {
        ArchClass::Scalar => (true, None),
        ArchClass::Target(arch) => (false, Some(arch)),
    };

    let single_concrete = if acc.concrete_assertions.len() == 1
        && acc.and_assertions.is_empty()
        && acc.bound_assertions.is_empty()
        && acc.token_param_indices.len() == 1
    {
        Some(SingleConcreteToken {
            type_name: acc.concrete_assertions[0].0.clone(),
            direct_value_param: !acc.first_leaf_had_wrapper,
        })
    } else {
        None
    };

    Ok(Some(SignatureProofs {
        primary_ident: acc
            .primary_ident
            .unwrap_or_else(|| format_ident!("__archmage_token")),
        features: acc.features,
        target_arch,
        is_scalar,
        magetypes_namespace: acc.magetypes_namespace,
        single_concrete,
        concrete_assertions: acc.concrete_assertions,
        and_assertions: acc.and_assertions,
        bound_assertions: acc.bound_assertions,
        all_concrete_names: acc.all_concrete_names,
        all_trait_names: acc.all_trait_names,
        token_param_indices: acc.token_param_indices,
    }))
}

fn diagnose_featureless_in_type(ty: &Type, sig: &Signature) -> Option<&'static str> {
    let ty = peel_paren_group(ty);
    match ty {
        Type::Reference(type_ref) => diagnose_featureless_in_type(&type_ref.elem, sig),
        Type::Tuple(tuple) => tuple
            .elems
            .iter()
            .find_map(|elem| diagnose_featureless_in_type(elem, sig)),
        Type::ImplTrait(impl_trait) => {
            let names = extract_traits_from_bounds(&impl_trait.bounds);
            find_featureless_trait(&names)
        }
        Type::Path(tp) if tp.qself.is_none() => {
            if let Some((l, r)) = is_and_path(&tp.path) {
                return diagnose_featureless_in_type(l, sig)
                    .or_else(|| diagnose_featureless_in_type(r, sig));
            }
            let ident = tp.path.get_ident()?;
            let name = ident.to_string();
            let bounds = find_generic_bounds(sig, &name)?;
            find_featureless_trait(&bounds)
        }
        _ => None,
    }
}

/// Diagnose why `discover_signature_proofs` found no capability token.
pub(crate) fn diagnose_featureless_token(sig: &Signature) -> Option<&'static str> {
    for arg in &sig.inputs {
        if let syn::FnArg::Typed(syn::PatType { ty, .. }) = arg
            && let Some(hit) = diagnose_featureless_in_type(ty, sig)
        {
            return Some(hit);
        }
    }
    None
}

/// Legacy single-parameter view kept for callers that only query simple token existence.
#[allow(dead_code)]
pub(crate) struct TokenParamInfo {
    pub(crate) ident: Ident,
    pub(crate) features: Cow<'static, [&'static str]>,
    pub(crate) target_arch: Option<&'static str>,
    pub(crate) token_type_name: Option<String>,
    pub(crate) magetypes_namespace: Option<&'static str>,
    pub(crate) token_type: Option<Type>,
    pub(crate) tier_traits: Vec<String>,
}

/// Find the token parameter in a function signature (delegates to `discover_signature_proofs`).
#[allow(dead_code)]
pub(crate) fn find_token_param(sig: &Signature) -> Option<TokenParamInfo> {
    let Ok(Some(proofs)) = discover_signature_proofs(sig) else {
        return None;
    };
    let (token_type_name, token_type) = match proofs.concrete_assertions.first() {
        Some((name, ty))
            if proofs.and_assertions.is_empty() && proofs.bound_assertions.is_empty() =>
        {
            (Some(name.clone()), Some(ty.clone()))
        }
        _ => (None, None),
    };
    let tier_traits = proofs.all_trait_names.clone();
    Some(TokenParamInfo {
        ident: proofs.primary_ident,
        features: Cow::Owned(proofs.features),
        target_arch: proofs.target_arch,
        token_type_name,
        magetypes_namespace: proofs.magetypes_namespace,
        token_type,
        tier_traits,
    })
}

/// Build the comma-separated `#[target_feature(enable = ...)]` string.
pub(crate) fn features_csv(token_name: Option<&str>, features: &[&str]) -> Cow<'static, str> {
    if let Some(name) = token_name
        && let Some(csv) = token_to_features_csv(name)
    {
        return Cow::Borrowed(csv);
    }
    Cow::Owned(features.join(","))
}
