//! Generated from token-registry.toml — DO NOT EDIT.
//!
//! Regenerate with: cargo xtask generate

use crate::tokens::{HasWasm128, HasWasm128Relaxed, HasWasm128RelaxedFma};
use crate::tokens::{SimdLeafToken, SimdToken};
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

// Cache statics: 0 = unknown, 1 = unavailable, 2 = available
#[allow(dead_code)]
pub(super) static WASM128_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static WASM128_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static WASM128_RELAXED_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static WASM128_RELAXED_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static WASM128_RELAXED_FMA_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static WASM128_RELAXED_FMA_DISABLED: AtomicBool = AtomicBool::new(false);

/// Proof that WASM SIMD128 is available.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Wasm128Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for Wasm128Token {
    type Arch = super::caps::WasmArch;
}

impl super::caps::WasmCaps for Wasm128Token {
    type Simd128 = super::caps::Yes;
    type Relaxed = super::caps::No;
    type RelaxedFma = super::caps::No;
}

impl SimdToken for Wasm128Token {
    type __Caps = super::caps::WasmCapSet<super::caps::Yes, super::caps::No, super::caps::No>;
    #[inline(always)]
    fn write_name(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(Self::NAME)
    }

    #[inline(always)]
    fn write_target_features(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(Self::TARGET_FEATURES)
    }

    #[inline(always)]
    fn has_target_feature(feature: &str) -> bool {
        !feature.is_empty() && Self::TARGET_FEATURES.split(',').any(|f| f == feature)
    }

    #[inline(always)]
    fn visit_target_features<F: FnMut(&'static str)>(f: &mut F) {
        for feat in Self::TARGET_FEATURES.split(',') {
            if !feat.is_empty() {
                f(feat);
            }
        }
    }

    #[inline]
    fn compiled_with() -> Option<bool> {
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            feature = "testable_dispatch"
        ))]
        {
            None
        }
        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            Some(false)
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        {
            #[cfg(feature = "testable_dispatch")]
            if WASM128_DISABLED.load(Ordering::Acquire) {
                return None;
            }
            // The required wasm features are compile-time enabled; a
            // runtime that validated this module supports them (wasm has
            // no runtime feature detection). wasm32 `#[target_feature]`
            // functions are safe to call, so no `unsafe` is needed.
            Some(Self::from_context())
        }
        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            None
        }
    }
}

impl SimdLeafToken for Wasm128Token {
    const NAME: &'static str = "WASM SIMD128";
    const TARGET_FEATURES: &'static str = "simd128";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+simd128";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-simd128";
}

impl Wasm128Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "WASM SIMD128";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "simd128";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+simd128";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-simd128";

    /// Returns the human-readable name for this token.
    #[inline(always)]
    pub fn name(&self) -> &'static str {
        Self::NAME
    }

    /// Append another token from the same architecture, forming a conjunction proof.
    #[inline(always)]
    pub const fn and<Extra: SimdToken>(self, extra: Extra) -> crate::tokens::And<Self, Extra>
    where
        Self: crate::tokens::SameArch<Extra>,
    {
        crate::tokens::And::join(self, extra)
    }
}

impl core::fmt::Display for Wasm128Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for Wasm128Token {
    #[inline(always)]
    fn as_wasm128(self) -> Option<Self> {
        Some(self)
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
impl Wasm128Token {
    /// Construct a WASM SIMD128 proof from the compile-time WASM target features.
    ///
    /// The required WASM features (`simd128`) are enabled at compile time,
    /// so the engine validates instruction support at module load time and no
    /// runtime detection is needed.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure every
    /// feature in this tier is available. Safe calls have that obligation
    /// discharged by the compiler's `#[target_feature]` check.
    #[inline]
    #[target_feature(enable = "simd128")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

#[cfg(all(target_arch = "wasm32", not(all(target_feature = "simd128"))))]
impl Wasm128Token {
    /// Construct a WASM SIMD128 proof without compile-time feature validation.
    ///
    /// The required WASM features (`simd128`) are NOT compile-time enabled
    /// in this build, so WebAssembly module load validation does not guarantee
    /// them and [`SimdToken::summon`](crate::SimdToken::summon) returns `None`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee every feature in `simd128` is supported
    /// by the executing WASM engine. Constructing this token without engine
    /// support is undefined behavior when executing instructions from this tier.
    #[inline(always)]
    pub unsafe fn from_context() -> Self {
        Self { _private: () }
    }
}

impl Wasm128Token {
    /// Disable this token process-wide for testing and benchmarking.
    ///
    /// When disabled, `summon()` will return `None` even if the CPU supports
    /// the required features.
    ///
    /// Returns `Err` when all required features are compile-time determined
    /// (e.g., via `-Ctarget-cpu=native`), since the compiler has already
    /// elided the runtime checks.
    ///
    /// **Cascading:** Also affects descendants:
    /// - `Wasm128RelaxedToken`
    /// - `Wasm128RelaxedFmaToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(not(feature = "testable_dispatch"))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(feature = "testable_dispatch")]
        {
            WASM128_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled { 1 } else { 0 };
            WASM128_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                WASM128_RELAXED_DISABLED.store(disabled, Ordering::Release);
            }
            WASM128_RELAXED_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                WASM128_RELAXED_FMA_DISABLED.store(disabled, Ordering::Release);
            }
            WASM128_RELAXED_FMA_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(not(feature = "testable_dispatch"))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(feature = "testable_dispatch")]
        {
            Ok(WASM128_DISABLED.load(Ordering::Acquire))
        }
    }
}

/// Proof that WASM Relaxed SIMD is available.
///
/// This token proves instruction availability, not single-rounding FMA.
/// Relaxed multiply-add may round once or twice, depending on the engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Wasm128RelaxedToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for Wasm128RelaxedToken {
    type Arch = super::caps::WasmArch;
}

impl super::caps::WasmCaps for Wasm128RelaxedToken {
    type Simd128 = super::caps::Yes;
    type Relaxed = super::caps::Yes;
    type RelaxedFma = super::caps::No;
}

impl SimdToken for Wasm128RelaxedToken {
    type __Caps = super::caps::WasmCapSet<super::caps::Yes, super::caps::Yes, super::caps::No>;
    #[inline(always)]
    fn write_name(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(Self::NAME)
    }

    #[inline(always)]
    fn write_target_features(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(Self::TARGET_FEATURES)
    }

    #[inline(always)]
    fn has_target_feature(feature: &str) -> bool {
        !feature.is_empty() && Self::TARGET_FEATURES.split(',').any(|f| f == feature)
    }

    #[inline(always)]
    fn visit_target_features<F: FnMut(&'static str)>(f: &mut F) {
        for feat in Self::TARGET_FEATURES.split(',') {
            if !feat.is_empty() {
                f(feat);
            }
        }
    }

    #[inline]
    fn compiled_with() -> Option<bool> {
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd",
            feature = "testable_dispatch"
        ))]
        {
            None
        }
        #[cfg(not(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd"
        )))]
        {
            Some(false)
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd"
        ))]
        {
            #[cfg(feature = "testable_dispatch")]
            if WASM128_RELAXED_DISABLED.load(Ordering::Acquire)
                || WASM128_DISABLED.load(Ordering::Acquire)
            {
                return None;
            }
            // The required wasm features are compile-time enabled; a
            // runtime that validated this module supports them (wasm has
            // no runtime feature detection). wasm32 `#[target_feature]`
            // functions are safe to call, so no `unsafe` is needed.
            Some(Self::from_context())
        }
        #[cfg(not(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd"
        )))]
        {
            None
        }
    }
}

impl SimdLeafToken for Wasm128RelaxedToken {
    const NAME: &'static str = "WASM Relaxed SIMD";
    const TARGET_FEATURES: &'static str = "simd128,relaxed-simd";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+simd128,+relaxed-simd";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-simd128,-relaxed-simd";
}

impl Wasm128RelaxedToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "WASM Relaxed SIMD";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "simd128,relaxed-simd";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+simd128,+relaxed-simd";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-simd128,-relaxed-simd";

    /// Returns the human-readable name for this token.
    #[inline(always)]
    pub fn name(&self) -> &'static str {
        Self::NAME
    }

    /// Append another token from the same architecture, forming a conjunction proof.
    #[inline(always)]
    pub const fn and<Extra: SimdToken>(self, extra: Extra) -> crate::tokens::And<Self, Extra>
    where
        Self: crate::tokens::SameArch<Extra>,
    {
        crate::tokens::And::join(self, extra)
    }
}

impl core::fmt::Display for Wasm128RelaxedToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for Wasm128RelaxedToken {
    #[inline(always)]
    fn as_wasm128_relaxed(self) -> Option<Self> {
        Some(self)
    }
}

#[cfg(all(
    target_arch = "wasm32",
    target_feature = "simd128",
    target_feature = "relaxed-simd"
))]
impl Wasm128RelaxedToken {
    /// Construct a WASM Relaxed SIMD proof from the compile-time WASM target features.
    ///
    /// The required WASM features (`simd128,relaxed-simd`) are enabled at compile time,
    /// so the engine validates instruction support at module load time and no
    /// runtime detection is needed.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure every
    /// feature in this tier is available. Safe calls have that obligation
    /// discharged by the compiler's `#[target_feature]` check.
    #[inline]
    #[target_feature(enable = "simd128,relaxed-simd")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

#[cfg(all(
    target_arch = "wasm32",
    not(all(target_feature = "simd128", target_feature = "relaxed-simd"))
))]
impl Wasm128RelaxedToken {
    /// Construct a WASM Relaxed SIMD proof without compile-time feature validation.
    ///
    /// The required WASM features (`simd128,relaxed-simd`) are NOT compile-time enabled
    /// in this build, so WebAssembly module load validation does not guarantee
    /// them and [`SimdToken::summon`](crate::SimdToken::summon) returns `None`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee every feature in `simd128,relaxed-simd` is supported
    /// by the executing WASM engine. Constructing this token without engine
    /// support is undefined behavior when executing instructions from this tier.
    #[inline(always)]
    pub unsafe fn from_context() -> Self {
        Self { _private: () }
    }
}

impl Wasm128RelaxedToken {
    /// Extract a Wasm128Token — guaranteed because WASM Relaxed SIMD implies WASM SIMD128.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn wasm128(self) -> Wasm128Token {
        // Holding `self` proves WASM Relaxed SIMD's feature set, a superset
        // of WASM SIMD128's; constructing the parent proof directly avoids
        // cfg-conditional `from_context()` unsafety across wasm builds.
        let _ = self;
        Wasm128Token { _private: () }
    }
}

impl Wasm128RelaxedToken {
    /// Disable this token process-wide for testing and benchmarking.
    ///
    /// When disabled, `summon()` will return `None` even if the CPU supports
    /// the required features.
    ///
    /// Returns `Err` when all required features are compile-time determined
    /// (e.g., via `-Ctarget-cpu=native`), since the compiler has already
    /// elided the runtime checks.
    ///
    /// **Cascading:** Also affects descendants:
    /// - `Wasm128RelaxedFmaToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(not(feature = "testable_dispatch"))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(feature = "testable_dispatch")]
        {
            WASM128_RELAXED_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (WASM128_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            WASM128_RELAXED_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                WASM128_RELAXED_FMA_DISABLED.store(disabled, Ordering::Release);
            }
            WASM128_RELAXED_FMA_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(not(feature = "testable_dispatch"))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(feature = "testable_dispatch")]
        {
            Ok(WASM128_RELAXED_DISABLED.load(Ordering::Acquire))
        }
    }
}

/// Proof that WASM Relaxed SIMD is available AND both `f32x4.relaxed_madd` and `f64x2.relaxed_madd` fuse to single-rounding FMA on the executing engine.
///
/// Can only be obtained via `summon()`, which runs and caches the runtime fusion probe; there is no `from_context()` constructor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Wasm128RelaxedFmaToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for Wasm128RelaxedFmaToken {
    type Arch = super::caps::WasmArch;
}

impl super::caps::WasmCaps for Wasm128RelaxedFmaToken {
    type Simd128 = super::caps::Yes;
    type Relaxed = super::caps::Yes;
    type RelaxedFma = super::caps::Yes;
}

impl SimdToken for Wasm128RelaxedFmaToken {
    type __Caps = super::caps::WasmCapSet<super::caps::Yes, super::caps::Yes, super::caps::Yes>;
    const __ARCHMAGE_CONTRACT_BITS: u64 = 1;
    #[inline(always)]
    fn write_name(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(Self::NAME)
    }

    #[inline(always)]
    fn write_target_features(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(Self::TARGET_FEATURES)
    }

    #[inline(always)]
    fn has_target_feature(feature: &str) -> bool {
        !feature.is_empty() && Self::TARGET_FEATURES.split(',').any(|f| f == feature)
    }

    #[inline(always)]
    fn visit_target_features<F: FnMut(&'static str)>(f: &mut F) {
        for feat in Self::TARGET_FEATURES.split(',') {
            if !feat.is_empty() {
                f(feat);
            }
        }
    }

    #[inline]
    fn compiled_with() -> Option<bool> {
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd"
        ))]
        {
            None
        }
        #[cfg(not(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd"
        )))]
        {
            Some(false)
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd"
        ))]
        {
            match WASM128_RELAXED_FMA_CACHE.load(Ordering::Acquire) {
                // 2 is only ever stored by wasm128_relaxed_fma_detect() after confirming
                // single-rounding f32x4 and f64x2 FMA fusion across all lanes.
                2 => Some(Self { _private: () }),
                1 => None,
                _ => wasm128_relaxed_fma_detect(),
            }
        }
        #[cfg(not(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            target_feature = "relaxed-simd"
        )))]
        {
            None
        }
    }
}

impl SimdLeafToken for Wasm128RelaxedFmaToken {
    const NAME: &'static str = "WASM Relaxed FMA";
    const TARGET_FEATURES: &'static str = "simd128,relaxed-simd";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+simd128,+relaxed-simd";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-simd128,-relaxed-simd";
}

impl Wasm128RelaxedFmaToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "WASM Relaxed FMA";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "simd128,relaxed-simd";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+simd128,+relaxed-simd";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-simd128,-relaxed-simd";

    /// Returns the human-readable name for this token.
    #[inline(always)]
    pub fn name(&self) -> &'static str {
        Self::NAME
    }

    /// Append another token from the same architecture, forming a conjunction proof.
    #[inline(always)]
    pub const fn and<Extra: SimdToken>(self, extra: Extra) -> crate::tokens::And<Self, Extra>
    where
        Self: crate::tokens::SameArch<Extra>,
    {
        crate::tokens::And::join(self, extra)
    }
}

impl core::fmt::Display for Wasm128RelaxedFmaToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for Wasm128RelaxedFmaToken {
    #[inline(always)]
    fn as_wasm128_relaxed_fma(self) -> Option<Self> {
        Some(self)
    }
}

impl Wasm128RelaxedFmaToken {
    /// Extract a Wasm128RelaxedToken — guaranteed because WASM Relaxed FMA implies WASM Relaxed SIMD.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn wasm128_relaxed(self) -> Wasm128RelaxedToken {
        // Holding `self` proves WASM Relaxed FMA's feature set, a superset
        // of WASM Relaxed SIMD's; constructing the parent proof directly avoids
        // cfg-conditional `from_context()` unsafety across wasm builds.
        let _ = self;
        Wasm128RelaxedToken { _private: () }
    }
    /// Extract a Wasm128Token — guaranteed because WASM Relaxed FMA implies WASM SIMD128.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn wasm128(self) -> Wasm128Token {
        // Holding `self` proves WASM Relaxed FMA's feature set, a superset
        // of WASM SIMD128's; constructing the parent proof directly avoids
        // cfg-conditional `from_context()` unsafety across wasm builds.
        let _ = self;
        Wasm128Token { _private: () }
    }
}

impl Wasm128RelaxedFmaToken {
    /// Disable this token process-wide for testing and benchmarking.
    ///
    /// When disabled, `summon()` returns `None` even on engines where
    /// the runtime probe succeeds. Pass `false` to re-enable probing.
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        WASM128_RELAXED_FMA_DISABLED.store(disabled, Ordering::Release);
        let v = if disabled
            || (WASM128_RELAXED_DISABLED.load(Ordering::Acquire)
                || WASM128_DISABLED.load(Ordering::Acquire))
        {
            1
        } else {
            0
        };
        WASM128_RELAXED_FMA_CACHE.store(v, Ordering::Release);
        Ok(())
    }

    /// Check if this token has been manually disabled process-wide.
    #[inline(always)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        Ok(WASM128_RELAXED_FMA_DISABLED.load(Ordering::Acquire))
    }
}
#[cfg(all(
    target_arch = "wasm32",
    target_feature = "simd128",
    target_feature = "relaxed-simd"
))]
#[cold]
#[inline(never)]
#[target_feature(enable = "simd128,relaxed-simd")]
fn wasm128_relaxed_fma_detect() -> Option<Wasm128RelaxedFmaToken> {
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
    let pre_disabled = WASM128_RELAXED_FMA_DISABLED.load(Ordering::Acquire)
        || WASM128_RELAXED_DISABLED.load(Ordering::Acquire)
        || WASM128_DISABLED.load(Ordering::Acquire);
    let _ = WASM128_RELAXED_FMA_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    let disabled = WASM128_RELAXED_FMA_DISABLED.load(Ordering::Acquire)
        || WASM128_RELAXED_DISABLED.load(Ordering::Acquire)
        || WASM128_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // Runtime probe just confirmed single-rounding f32x4 and f64x2
        // relaxed_madd fusion across all lanes on this engine.
        Some(Wasm128RelaxedFmaToken { _private: () })
    } else {
        None
    }
}

impl Wasm128Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x1E0DF26B;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_1E0DF26B: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x1E0DF26B) as usize];
}

impl Wasm128RelaxedToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x821D5452;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_821D5452: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x821D5452) as usize];
}

impl Wasm128RelaxedFmaToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x735B647C;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_735B647C: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x735B647C) as usize];
}

// Compile-time check: capability blanket impls cover every token's declared traits.
const _: () = {
    {
        const fn _assert<T: HasWasm128>() {}
        let _ = _assert::<Wasm128Token>;
    }
    {
        const fn _assert<T: HasWasm128>() {}
        let _ = _assert::<Wasm128RelaxedToken>;
    }
    {
        const fn _assert<T: HasWasm128Relaxed>() {}
        let _ = _assert::<Wasm128RelaxedToken>;
    }
    {
        const fn _assert<T: HasWasm128>() {}
        let _ = _assert::<Wasm128RelaxedFmaToken>;
    }
    {
        const fn _assert<T: HasWasm128Relaxed>() {}
        let _ = _assert::<Wasm128RelaxedFmaToken>;
    }
    {
        const fn _assert<T: HasWasm128RelaxedFma>() {}
        let _ = _assert::<Wasm128RelaxedFmaToken>;
    }
};
