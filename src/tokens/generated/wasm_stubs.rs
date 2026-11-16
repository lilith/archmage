//! Generated from token-registry.toml — DO NOT EDIT.
//!
//! Stub tokens: `summon()` always returns `None`.

use crate::tokens::{HasWasm128, HasWasm128Relaxed, HasWasm128RelaxedFma};
use crate::tokens::{SimdLeafToken, SimdToken};

/// Stub for WASM SIMD128 token (not available on this architecture).
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
        Some(false) // Wrong architecture
    }

    #[inline]
    fn summon() -> Option<Self> {
        None // Not available on this architecture
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

    /// This token is not available on this architecture.
    pub fn dangerously_disable_token_process_wide(
        _disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        Err(crate::tokens::CompileTimeGuaranteedError {
            token_name: Self::NAME,
            target_features: Self::TARGET_FEATURES,
            disable_flags: Self::DISABLE_TARGET_FEATURES,
        })
    }

    /// This token is not available on this architecture.
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        Err(crate::tokens::CompileTimeGuaranteedError {
            token_name: Self::NAME,
            target_features: Self::TARGET_FEATURES,
            disable_flags: Self::DISABLE_TARGET_FEATURES,
        })
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

/// Stub for WASM Relaxed SIMD token (not available on this architecture).
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
        Some(false) // Wrong architecture
    }

    #[inline]
    fn summon() -> Option<Self> {
        None // Not available on this architecture
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

    /// This token is not available on this architecture.
    pub fn dangerously_disable_token_process_wide(
        _disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        Err(crate::tokens::CompileTimeGuaranteedError {
            token_name: Self::NAME,
            target_features: Self::TARGET_FEATURES,
            disable_flags: Self::DISABLE_TARGET_FEATURES,
        })
    }

    /// This token is not available on this architecture.
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        Err(crate::tokens::CompileTimeGuaranteedError {
            token_name: Self::NAME,
            target_features: Self::TARGET_FEATURES,
            disable_flags: Self::DISABLE_TARGET_FEATURES,
        })
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

/// Stub for WASM Relaxed FMA token (not available on this architecture).
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
        Some(false) // Wrong architecture
    }

    #[inline]
    fn summon() -> Option<Self> {
        None // Not available on this architecture
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

    /// This token is not available on this architecture.
    pub fn dangerously_disable_token_process_wide(
        _disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        Err(crate::tokens::CompileTimeGuaranteedError {
            token_name: Self::NAME,
            target_features: Self::TARGET_FEATURES,
            disable_flags: Self::DISABLE_TARGET_FEATURES,
        })
    }

    /// This token is not available on this architecture.
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        Err(crate::tokens::CompileTimeGuaranteedError {
            token_name: Self::NAME,
            target_features: Self::TARGET_FEATURES,
            disable_flags: Self::DISABLE_TARGET_FEATURES,
        })
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
