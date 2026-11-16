//! SIMD capability tokens
//!
//! Tokens are zero-sized proof types that demonstrate a CPU feature set is
//! available. They are obtained via `summon()`, safely composed with `.and(...)`
//! or [`And::join`], and passed to `#[arcane]` / `#[rite]` functions.
//!
//! ## Token Availability
//!
//! Use `compiled_with()` to check compile-time availability:
//! - `Some(true)` — Binary was compiled with these features enabled (use `summon().unwrap()`)
//! - `Some(false)` — Wrong architecture or compile-time constraint makes this token unavailable
//! - `None` — Might be available, call `summon()` to check at runtime
//!
//! ## Composing Tokens (`And<Base, Extra>`)
//!
//! Orthogonal hardware extensions (such as `X64V3Token` + `X64GfniToken` +
//! `X64AvxVnniToken`) compose at zero runtime cost via `.and(...)` / [`And`]:
//!
//! ```rust,ignore
//! use archmage::{SimdToken, X64GfniToken, X64V3Token};
//!
//! if let Some(token) = X64V3Token::summon().zip(X64GfniToken::summon()).map(|(v3, g)| v3.and(g)) {
//!     // `token` satisfies both `HasX64V3` and `HasGfni`!
//! }
//! ```
//!
//! Cross-architecture combinations (`X64` + `Arm`, `X64` + `Scalar`, etc.) are
//! rejected at compile time via [`SameArch`].
//!
//! ## Cross-Architecture Design
//!
//! All token types exist on all architectures for easier cross-platform code.
//! On unsupported architectures, `summon()` returns `None` and `compiled_with()`
//! returns `Some(false)`.
//!
//! ## Disabling Tokens
//!
//! For testing and benchmarking, tokens can be disabled process-wide:
//!
//! ```rust,ignore
//! // Force scalar fallback for benchmarking
//! X64V3Token::dangerously_disable_token_process_wide(true)?;
//! // All summon() calls now return None (cascades to V3Crypto, V4, V4x, Fp16, Avx512Bf16)
//! assert!(X64V3Token::summon().is_none());
//! ```
//!
//! Disabling returns `Err(CompileTimeGuaranteedError)` when the features are
//! compile-time enabled (e.g., via `-Ctarget-cpu=native`), since the compiler
//! has already elided the runtime checks.

mod sealed {
    /// Sealed trait preventing external implementations of [`SimdToken`](super::SimdToken)
    /// and [`SameArch`](super::SameArch).
    pub trait Sealed {
        #[doc(hidden)]
        type Arch: super::caps::ArchMarker;
    }
}

// Sealed is pub(crate) — external crates cannot implement SimdToken or capability traits.
pub(crate) use sealed::Sealed;

// With std, `Vec` comes from the prelude; importing it there is redundant.
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Supertrait guaranteeing type-level capability union for same-architecture tokens.
#[doc(hidden)]
pub trait SameArchCaps<Other: SimdToken>: Sealed {
    #[doc(hidden)]
    type __JoinedCaps: Copy + Clone + Send + Sync + 'static;
}

impl<A, B> SameArchCaps<B> for A
where
    A: SimdToken,
    B: SimdToken,
    A::__Caps: caps::CapSetOr<B::__Caps>,
{
    type __JoinedCaps = <A::__Caps as caps::CapSetOr<B::__Caps>>::Output;
}

/// Marker trait proving two tokens belong to the same CPU architecture family.
///
/// Cross-architecture combinations (e.g., `X64V3Token` with `NeonToken`, or
/// `X64V3Token` with `ScalarToken`) do not implement `SameArch` and fail at
/// compile time.
pub trait SameArch<Other: SimdToken>: SimdToken + SameArchCaps<Other> {}

impl<A, B> SameArch<B> for A
where
    A: SimdToken,
    B: SimdToken + Sealed<Arch = <A as Sealed>::Arch>,
    A::__Caps: caps::CapSetOr<B::__Caps>,
{
}

/// Marker trait for SIMD capability tokens (both concrete leaves and [`And`] conjunctions).
///
/// All tokens implement this trait, enabling generic code over different
/// SIMD feature levels and composable conjunction proofs.
///
/// This trait is **sealed** — it cannot be implemented outside of archmage.
/// Only tokens created by `summon()`, `from_context()`, extraction, or
/// `.and(...)` / [`And::join`] are valid.
pub trait SimdToken:
    Sealed
    + Copy
    + Clone
    + core::fmt::Debug
    + core::fmt::Display
    + PartialEq
    + Eq
    + core::hash::Hash
    + Send
    + Sync
    + 'static
{
    /// Type-level capability set encoding every ISA slot proven by this token.
    #[doc(hidden)]
    type __Caps: Copy + Clone + Send + Sync + 'static;

    /// Bitmask of non-target-feature runtime proof contracts (e.g., bit 0 for
    /// WASM single-rounding FMA fusion on `Wasm128RelaxedFmaToken`).
    #[doc(hidden)]
    const __ARCHMAGE_CONTRACT_BITS: u64 = 0;

    /// Format the human-readable name of this token.
    fn write_name(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;

    /// Format comma-delimited deduplicated LLVM target features asserted by this token.
    fn write_target_features(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;

    /// Returns `true` if `feature` is part of this token's target feature set.
    fn has_target_feature(feature: &str) -> bool;

    /// Invoke `f` once for each unique LLVM target feature asserted by this token.
    fn visit_target_features<F: FnMut(&'static str)>(f: &mut F);

    /// Check if this binary was compiled with the required target features enabled.
    ///
    /// Returns:
    /// - `Some(true)` — Features are compile-time enabled and runtime contracts are satisfied statically
    /// - `Some(false)` — Wrong architecture or missing compile-time prerequisite
    /// - `None` — Might be available, call `summon()` to check at runtime
    fn compiled_with() -> Option<bool>;

    /// Attempt to create a token with runtime feature detection (and any required runtime probes).
    ///
    /// Returns `Some(token)` if the CPU/engine supports every required capability, `None` otherwise.
    fn summon() -> Option<Self>;

    /// Attempt to create a token with runtime feature detection.
    ///
    /// This is an alias for [`summon()`](Self::summon).
    #[inline(always)]
    fn attempt() -> Option<Self> {
        Self::summon()
    }
}

/// Extension of [`SimdToken`] for atomic leaf tokens with static `&'static str` metadata.
///
/// Composite [`And<Base, Extra>`] tokens implement [`SimdToken`] rather than `SimdLeafToken`
/// so arbitrary trees compose without requiring nightly const string allocation.
pub trait SimdLeafToken: SimdToken {
    /// Human-readable name for diagnostics and error messages.
    const NAME: &'static str;

    /// Comma-delimited LLVM target features asserted by this token.
    const TARGET_FEATURES: &'static str;

    /// RUSTFLAGS to enable these features at compile time.
    const ENABLE_TARGET_FEATURES: &'static str;

    /// RUSTFLAGS to disable these features at compile time.
    const DISABLE_TARGET_FEATURES: &'static str;

    /// Returns the human-readable name for this leaf token.
    #[inline(always)]
    fn name(&self) -> &'static str {
        Self::NAME
    }
}

// All token types, traits, capability sets, and aliases are generated from token-registry.toml.
mod generated;
pub use generated::*;

/// Conjunction proof holding both `Base` and `Extra` capability proofs from the same architecture.
///
/// Construct via [`token.and(extra)`](X64V3Token::and) or [`And::join`].
/// Automatically inherits every [`Has*`](HasX64V3) capability marker satisfied by
/// the union of `Base` and `Extra`'s capabilities, with zero runtime overhead (`#[repr(transparent)]`-like ZST).
///
/// `And<Base, Extra>` intentionally has **no** `from_context()` constructor:
/// each constituent proof must already be obtained legitimately, preserving runtime
/// probe contracts (such as [`Wasm128RelaxedFmaToken`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct And<Base, Extra>
where
    Base: SameArch<Extra>,
    Extra: SimdToken,
{
    base: Base,
    extra: Extra,
}

impl<Base, Extra> And<Base, Extra>
where
    Base: SameArch<Extra>,
    Extra: SimdToken,
{
    /// Combine two same-architecture capability tokens into their conjunction proof.
    #[inline(always)]
    pub const fn join(base: Base, extra: Extra) -> Self {
        Self { base, extra }
    }

    /// Append another same-architecture capability token to this conjunction.
    #[inline(always)]
    pub const fn and<Next: SimdToken>(self, extra: Next) -> And<Self, Next>
    where
        Self: SameArch<Next>,
    {
        And::join(self, extra)
    }

    /// Extract the left constituent proof.
    #[inline(always)]
    pub const fn base(&self) -> Base {
        self.base
    }

    /// Extract the right constituent proof.
    #[inline(always)]
    pub const fn extra(&self) -> Extra {
        self.extra
    }
}

impl<Base, Extra> Sealed for And<Base, Extra>
where
    Base: SameArch<Extra>,
    Extra: SimdToken,
{
    type Arch = <Base as Sealed>::Arch;
}

impl<Base, Extra> SimdToken for And<Base, Extra>
where
    Base: SameArch<Extra>,
    Extra: SimdToken,
{
    type __Caps = <Base as SameArchCaps<Extra>>::__JoinedCaps;

    const __ARCHMAGE_CONTRACT_BITS: u64 =
        Base::__ARCHMAGE_CONTRACT_BITS | Extra::__ARCHMAGE_CONTRACT_BITS;

    #[inline(always)]
    fn write_name(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Base::write_name(f)?;
        f.write_str(" + ")?;
        Extra::write_name(f)
    }

    fn write_target_features(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;
        let mut res = Ok(());
        Self::visit_target_features(&mut |feat| {
            if res.is_err() {
                return;
            }
            if !first {
                res = f.write_str(",");
                if res.is_err() {
                    return;
                }
            }
            first = false;
            res = f.write_str(feat);
        });
        res
    }

    #[inline]
    fn has_target_feature(feature: &str) -> bool {
        Base::has_target_feature(feature) || Extra::has_target_feature(feature)
    }

    #[inline]
    fn visit_target_features<F: FnMut(&'static str)>(f: &mut F) {
        Base::visit_target_features(f);
        Extra::visit_target_features(&mut |feat| {
            if !Base::has_target_feature(feat) {
                f(feat);
            }
        });
    }

    #[inline]
    fn compiled_with() -> Option<bool> {
        match (Base::compiled_with(), Extra::compiled_with()) {
            (Some(false), _) | (_, Some(false)) => Some(false),
            (Some(true), Some(true)) => Some(true),
            _ => None,
        }
    }

    #[inline]
    fn summon() -> Option<Self> {
        Some(Self {
            base: Base::summon()?,
            extra: Extra::summon()?,
        })
    }
}

impl<Base, Extra> core::fmt::Display for And<Base, Extra>
where
    Base: SameArch<Extra>,
    Extra: SimdToken,
{
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

/// Scalar fallback token — always available on all platforms.
///
/// Use this for fallback paths when no SIMD is available. `ScalarToken`
/// provides type-level proof that "we've given up on SIMD" and allows
/// consistent API shapes in dispatch code.
///
/// # Example
///
/// ```rust
/// use archmage::{ScalarToken, SimdToken, SimdLeafToken};
///
/// // Always succeeds
/// let token = ScalarToken::summon().unwrap();
///
/// // Or use the shorthand
/// let token = ScalarToken;
/// assert_eq!(token.name(), "Scalar");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScalarToken;

impl Sealed for ScalarToken {
    type Arch = caps::ScalarArch;
}

impl SimdToken for ScalarToken {
    type __Caps = caps::ScalarCaps;

    #[inline(always)]
    fn write_name(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(Self::NAME)
    }

    #[inline(always)]
    fn write_target_features(_f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }

    #[inline(always)]
    fn has_target_feature(_feature: &str) -> bool {
        false
    }

    #[inline(always)]
    fn visit_target_features<F: FnMut(&'static str)>(_f: &mut F) {}

    /// Always returns `Some(true)` — scalar fallback is always available.
    #[inline(always)]
    fn compiled_with() -> Option<bool> {
        Some(true)
    }

    /// Always returns `Some(ScalarToken)`.
    #[inline(always)]
    fn summon() -> Option<Self> {
        Some(Self)
    }
}

impl SimdLeafToken for ScalarToken {
    const NAME: &'static str = "Scalar";
    const TARGET_FEATURES: &'static str = "";
    const ENABLE_TARGET_FEATURES: &'static str = "";
    const DISABLE_TARGET_FEATURES: &'static str = "";
}

impl ScalarToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "Scalar";

    /// Comma-delimited target features (empty for scalar).
    pub const TARGET_FEATURES: &'static str = "";

    /// RUSTFLAGS to enable these features (empty for scalar).
    pub const ENABLE_TARGET_FEATURES: &'static str = "";

    /// RUSTFLAGS to disable these features (empty for scalar).
    pub const DISABLE_TARGET_FEATURES: &'static str = "";

    /// Returns `"Scalar"`.
    #[inline(always)]
    pub fn name(&self) -> &'static str {
        Self::NAME
    }

    /// Append another scalar token (trivially returns `And<ScalarToken, Extra>`).
    #[inline(always)]
    pub const fn and<Extra: SimdToken>(self, extra: Extra) -> And<Self, Extra>
    where
        Self: SameArch<Extra>,
    {
        And::join(self, extra)
    }

    /// Construct a scalar proof from the caller's feature context.
    ///
    /// `ScalarToken` asserts no CPU features, so it carries no
    /// `#[target_feature]` gate and every context can construct it.
    #[inline(always)]
    pub const fn from_context() -> Self {
        Self
    }

    /// Scalar tokens cannot be disabled (they are always available).
    ///
    /// Always returns `Err(CompileTimeGuaranteedError)` because `ScalarToken`
    /// is unconditionally available — there is no runtime check to bypass.
    pub fn dangerously_disable_token_process_wide(
        _disabled: bool,
    ) -> Result<(), CompileTimeGuaranteedError> {
        Err(CompileTimeGuaranteedError {
            token_name: Self::NAME,
            target_features: Self::TARGET_FEATURES,
            disable_flags: Self::DISABLE_TARGET_FEATURES,
        })
    }

    /// Scalar tokens cannot be disabled (they are always available).
    ///
    /// Always returns `Err(CompileTimeGuaranteedError)`.
    pub fn manually_disabled() -> Result<bool, CompileTimeGuaranteedError> {
        Err(CompileTimeGuaranteedError {
            token_name: Self::NAME,
            target_features: Self::TARGET_FEATURES,
            disable_flags: Self::DISABLE_TARGET_FEATURES,
        })
    }
}

impl core::fmt::Display for ScalarToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

/// Error returned when attempting to disable a token whose features are
/// compile-time enabled.
///
/// When all required features are enabled via `-Ctarget-cpu` or
/// `-Ctarget-feature`, the compiler has already elided the runtime
/// detection — `summon()` unconditionally returns `Some`. Disabling
/// the token would have no effect and silently produce incorrect behavior.
///
/// # Resolution
///
/// To use `dangerously_disable_token_process_wide`, compile without
/// the target features enabled. For example:
/// - Remove `-Ctarget-cpu=native` from `RUSTFLAGS`
/// - Use the [`disable_flags`](Self::disable_flags) string to disable specific features
#[derive(Debug, Clone)]
pub struct CompileTimeGuaranteedError {
    /// The token type that cannot be disabled.
    pub token_name: &'static str,
    /// Comma-delimited target features (e.g., `"avx2,fma,bmi1,bmi2,f16c,lzcnt"`).
    /// Empty for ScalarToken.
    pub target_features: &'static str,
    /// RUSTFLAGS to disable these features (e.g., `"-Ctarget-feature=-avx2,-fma,..."`).
    /// Empty for ScalarToken.
    pub disable_flags: &'static str,
}

impl core::fmt::Display for CompileTimeGuaranteedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.target_features.is_empty() {
            write!(f, "Cannot disable {} — always available.", self.token_name)
        } else {
            write!(
                f,
                "Cannot disable {} — features [{}] are compile-time enabled.\n\n\
                 To allow runtime disable, recompile with RUSTFLAGS:\n  \
                 \"{}\"\n\n\
                 Or remove `-Ctarget-cpu` from RUSTFLAGS entirely.",
                self.token_name, self.target_features, self.disable_flags
            )
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CompileTimeGuaranteedError {}

/// Error returned when [`dangerously_disable_tokens_except_wasm`] fails to
/// disable one or more tokens because their features are compile-time enabled.
///
/// Contains the individual [`CompileTimeGuaranteedError`] for each token that
/// could not be disabled.
#[derive(Debug, Clone)]
pub struct DisableAllSimdError {
    /// The individual errors for each token that could not be disabled.
    pub errors: Vec<CompileTimeGuaranteedError>,
}

impl core::fmt::Display for DisableAllSimdError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed to disable {} token(s):", self.errors.len())?;
        for err in &self.errors {
            write!(f, "\n  - {}", err.token_name)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DisableAllSimdError {}

/// Disable all SIMD tokens process-wide (except WASM, which is always compile-time
/// except for the runtime-probed [`Wasm128RelaxedFmaToken`]).
///
/// Calls `dangerously_disable_token_process_wide(disabled)` on every root
/// non-baseline token on the current architecture:
/// - **x86/x86_64:** `X64V2Token` (cascades through V3/V4 hierarchies) plus
///   the orthogonal extension clumps (`X64GfniToken`, `X64ShaToken`,
///   `X64AdxToken`, `X64AvxVnniToken`, `X64AvxIfmaToken`,
///   `X64AvxVnniInt8Token`, `X64AvxVnniInt16Token`, `X64AvxNeConvertToken`,
///   `X64Sha512Token`).
/// - **AArch64:** `NeonToken` (cascades to all NEON extension tokens and `Arm64V2`/`Arm64V3`).
///
/// Pass `disabled = false` to re-enable all tokens.
pub fn dangerously_disable_tokens_except_wasm(
    #[allow(unused)] disabled: bool,
) -> Result<(), DisableAllSimdError> {
    #[allow(unused_mut)]
    let mut errors = Vec::new();

    #[cfg(target_arch = "x86_64")]
    {
        for res in [
            X64V2Token::dangerously_disable_token_process_wide(disabled),
            X64GfniToken::dangerously_disable_token_process_wide(disabled),
            X64ShaToken::dangerously_disable_token_process_wide(disabled),
            X64AdxToken::dangerously_disable_token_process_wide(disabled),
            X64AvxVnniToken::dangerously_disable_token_process_wide(disabled),
            X64AvxIfmaToken::dangerously_disable_token_process_wide(disabled),
            X64AvxVnniInt8Token::dangerously_disable_token_process_wide(disabled),
            X64AvxVnniInt16Token::dangerously_disable_token_process_wide(disabled),
            X64AvxNeConvertToken::dangerously_disable_token_process_wide(disabled),
            X64Sha512Token::dangerously_disable_token_process_wide(disabled),
        ] {
            if let Err(e) = res {
                errors.push(e);
            }
        }
    }

    #[cfg(target_arch = "aarch64")]
    if let Err(e) = NeonToken::dangerously_disable_token_process_wide(disabled) {
        errors.push(e);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(DisableAllSimdError { errors })
    }
}
