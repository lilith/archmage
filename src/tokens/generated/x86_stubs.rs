//! Generated from token-registry.toml — DO NOT EDIT.
//!
//! Stub tokens: `summon()` always returns `None`.

use crate::tokens::{
    HasAdx, HasAvx512Bf16, HasAvx512Fp16, HasAvxIfma, HasAvxNeConvert, HasAvxVnni, HasAvxVnniInt8,
    HasAvxVnniInt16, HasGfni, HasSha, HasSha512, HasX64Crypto, HasX64V1, HasX64V2, HasX64V3,
    HasX64V3Crypto, HasX64V3GfniCrypto, HasX64V3Vnni, HasX64V3Vnni2, HasX64V4, HasX64V4x,
};
use crate::tokens::{SimdLeafToken, SimdToken};

/// Stub for x86-64-v1 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V1Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V1Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V1Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V1Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V1Token {
    const NAME: &'static str = "x86-64-v1";
    const TARGET_FEATURES: &'static str = "sse,sse2";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2";
}

impl X64V1Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64-v1";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2";

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

impl core::fmt::Display for X64V1Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V1Token {
    #[inline(always)]
    fn as_x64v1(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64-v2 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V2Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V2Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V2Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V2Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V2Token {
    const NAME: &'static str = "x86-64-v2";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b";
    const ENABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b";
    const DISABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b";
}

impl X64V2Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64-v2";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b";

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

impl core::fmt::Display for X64V2Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V2Token {
    #[inline(always)]
    fn as_x64v2(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 Crypto token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64CryptoToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64CryptoToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64CryptoToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64CryptoToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64CryptoToken {
    const NAME: &'static str = "x86-64 Crypto";
    const TARGET_FEATURES: &'static str =
        "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,pclmulqdq,aes";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+pclmulqdq,+aes";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-pclmulqdq,-aes";
}

impl X64CryptoToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 Crypto";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str =
        "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,pclmulqdq,aes";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+pclmulqdq,+aes";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-pclmulqdq,-aes";

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

impl core::fmt::Display for X64CryptoToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64CryptoToken {
    #[inline(always)]
    fn as_x64_crypto(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64-v3 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V3Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V3Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V3Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V3Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V3Token {
    const NAME: &'static str = "x86-64-v3";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe";
}

impl X64V3Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64-v3";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe";

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

impl core::fmt::Display for X64V3Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V3Token {
    #[inline(always)]
    fn as_x64v3(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64-v3 Crypto token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V3CryptoToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V3CryptoToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V3CryptoToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::Yes;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V3CryptoToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V3CryptoToken {
    const NAME: &'static str = "x86-64-v3 Crypto";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+vpclmulqdq,+vaes";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-vpclmulqdq,-vaes";
}

impl X64V3CryptoToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64-v3 Crypto";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+vpclmulqdq,+vaes";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-vpclmulqdq,-vaes";

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

impl core::fmt::Display for X64V3CryptoToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V3CryptoToken {
    #[inline(always)]
    fn as_x64v3_crypto(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64-v3 GFNI Crypto token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V3GfniCryptoToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V3GfniCryptoToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V3GfniCryptoToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::Yes;
    type Gfni = super::caps::Yes;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V3GfniCryptoToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V3GfniCryptoToken {
    const NAME: &'static str = "x86-64-v3 GFNI Crypto";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+vpclmulqdq,+vaes,+gfni";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-vpclmulqdq,-vaes,-gfni";
}

impl X64V3GfniCryptoToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64-v3 GFNI Crypto";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+vpclmulqdq,+vaes,+gfni";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-vpclmulqdq,-vaes,-gfni";

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

impl core::fmt::Display for X64V3GfniCryptoToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V3GfniCryptoToken {
    #[inline(always)]
    fn as_x64v3_gfni_crypto(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64-v3 VNNI token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V3VnniToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V3VnniToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V3VnniToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::Yes;
    type Gfni = super::caps::Yes;
    type Sha = super::caps::Yes;
    type Adx = super::caps::Yes;
    type AvxVnni = super::caps::Yes;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V3VnniToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V3VnniToken {
    const NAME: &'static str = "x86-64-v3 VNNI";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni,avxvnni,adx,sha";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+vpclmulqdq,+vaes,+gfni,+avxvnni,+adx,+sha";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-vpclmulqdq,-vaes,-gfni,-avxvnni,-adx,-sha";
}

impl X64V3VnniToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64-v3 VNNI";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni,avxvnni,adx,sha";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+vpclmulqdq,+vaes,+gfni,+avxvnni,+adx,+sha";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-vpclmulqdq,-vaes,-gfni,-avxvnni,-adx,-sha";

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

impl core::fmt::Display for X64V3VnniToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V3VnniToken {
    #[inline(always)]
    fn as_x64v3_vnni(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64-v3 VNNI-2 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V3Vnni2Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V3Vnni2Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V3Vnni2Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::Yes;
    type Gfni = super::caps::Yes;
    type Sha = super::caps::Yes;
    type Adx = super::caps::Yes;
    type AvxVnni = super::caps::Yes;
    type AvxIfma = super::caps::Yes;
    type AvxVnniInt8 = super::caps::Yes;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::Yes;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V3Vnni2Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V3Vnni2Token {
    const NAME: &'static str = "x86-64-v3 VNNI-2";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni,avxvnni,adx,sha,avxvnniint8,avxneconvert,avxifma";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+vpclmulqdq,+vaes,+gfni,+avxvnni,+adx,+sha,+avxvnniint8,+avxneconvert,+avxifma";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-vpclmulqdq,-vaes,-gfni,-avxvnni,-adx,-sha,-avxvnniint8,-avxneconvert,-avxifma";
}

impl X64V3Vnni2Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64-v3 VNNI-2";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni,avxvnni,adx,sha,avxvnniint8,avxneconvert,avxifma";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+vpclmulqdq,+vaes,+gfni,+avxvnni,+adx,+sha,+avxvnniint8,+avxneconvert,+avxifma";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-vpclmulqdq,-vaes,-gfni,-avxvnni,-adx,-sha,-avxvnniint8,-avxneconvert,-avxifma";

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

impl core::fmt::Display for X64V3Vnni2Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V3Vnni2Token {
    #[inline(always)]
    fn as_x64v3_vnni2(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for AVX-512 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V4Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V4Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V4Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::Yes;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V4Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V4Token {
    const NAME: &'static str = "AVX-512";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl";
}

impl X64V4Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "AVX-512";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl";

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

impl core::fmt::Display for X64V4Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V4Token {
    #[inline(always)]
    fn as_x64v4(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for AVX-512FP16 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Avx512Fp16Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for Avx512Fp16Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for Avx512Fp16Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::Yes;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::Yes;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for Avx512Fp16Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
    >;
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

impl SimdLeafToken for Avx512Fp16Token {
    const NAME: &'static str = "AVX-512FP16";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512fp16";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl,+avx512fp16";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl,-avx512fp16";
}

impl Avx512Fp16Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "AVX-512FP16";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512fp16";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl,+avx512fp16";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl,-avx512fp16";

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

impl core::fmt::Display for Avx512Fp16Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for Avx512Fp16Token {
    #[inline(always)]
    fn as_avx512_fp16(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for AVX-512 BF16 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64Avx512Bf16Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64Avx512Bf16Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64Avx512Bf16Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::Yes;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::Yes;
}

impl SimdToken for X64Avx512Bf16Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
    >;
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

impl SimdLeafToken for X64Avx512Bf16Token {
    const NAME: &'static str = "AVX-512 BF16";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512bf16";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl,+avx512bf16";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl,-avx512bf16";
}

impl X64Avx512Bf16Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "AVX-512 BF16";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512bf16";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl,+avx512bf16";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl,-avx512bf16";

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

impl core::fmt::Display for X64Avx512Bf16Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64Avx512Bf16Token {
    #[inline(always)]
    fn as_x64_avx512_bf16(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64-v4x token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64V4xToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64V4xToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64V4xToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::Yes;
    type Crypto128 = super::caps::Yes;
    type V3 = super::caps::Yes;
    type VpclmulVaes = super::caps::Yes;
    type Gfni = super::caps::Yes;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::Yes;
    type V4x = super::caps::Yes;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64V4xToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64V4xToken {
    const NAME: &'static str = "x86-64-v4x";
    const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512vpopcntdq,avx512ifma,avx512vbmi,avx512vbmi2,avx512bitalg,avx512vnni,vpclmulqdq,gfni,vaes";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl,+avx512vpopcntdq,+avx512ifma,+avx512vbmi,+avx512vbmi2,+avx512bitalg,+avx512vnni,+vpclmulqdq,+gfni,+vaes";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl,-avx512vpopcntdq,-avx512ifma,-avx512vbmi,-avx512vbmi2,-avx512bitalg,-avx512vnni,-vpclmulqdq,-gfni,-vaes";
}

impl X64V4xToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64-v4x";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512vpopcntdq,avx512ifma,avx512vbmi,avx512vbmi2,avx512bitalg,avx512vnni,vpclmulqdq,gfni,vaes";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt,+cmpxchg16b,+avx,+avx2,+fma,+bmi1,+bmi2,+f16c,+lzcnt,+movbe,+pclmulqdq,+aes,+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl,+avx512vpopcntdq,+avx512ifma,+avx512vbmi,+avx512vbmi2,+avx512bitalg,+avx512vnni,+vpclmulqdq,+gfni,+vaes";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-popcnt,-cmpxchg16b,-avx,-avx2,-fma,-bmi1,-bmi2,-f16c,-lzcnt,-movbe,-pclmulqdq,-aes,-avx512f,-avx512bw,-avx512cd,-avx512dq,-avx512vl,-avx512vpopcntdq,-avx512ifma,-avx512vbmi,-avx512vbmi2,-avx512bitalg,-avx512vnni,-vpclmulqdq,-gfni,-vaes";

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

impl core::fmt::Display for X64V4xToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64V4xToken {
    #[inline(always)]
    fn as_x64v4x(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 GFNI token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64GfniToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64GfniToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64GfniToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::Yes;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64GfniToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64GfniToken {
    const NAME: &'static str = "x86-64 GFNI";
    const TARGET_FEATURES: &'static str = "sse,sse2,gfni";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+gfni";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-gfni";
}

impl X64GfniToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 GFNI";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,gfni";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+gfni";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-gfni";

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

impl core::fmt::Display for X64GfniToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64GfniToken {
    #[inline(always)]
    fn as_x64_gfni(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 SHA token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64ShaToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64ShaToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64ShaToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::Yes;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64ShaToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64ShaToken {
    const NAME: &'static str = "x86-64 SHA";
    const TARGET_FEATURES: &'static str = "sse,sse2,sha";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sha";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sha";
}

impl X64ShaToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 SHA";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,sha";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+sha";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-sha";

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

impl core::fmt::Display for X64ShaToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64ShaToken {
    #[inline(always)]
    fn as_x64_sha(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 ADX token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64AdxToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64AdxToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64AdxToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::Yes;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64AdxToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64AdxToken {
    const NAME: &'static str = "x86-64 ADX";
    const TARGET_FEATURES: &'static str = "sse,sse2,adx";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+adx";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-adx";
}

impl X64AdxToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 ADX";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,adx";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+adx";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-adx";

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

impl core::fmt::Display for X64AdxToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64AdxToken {
    #[inline(always)]
    fn as_x64_adx(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 AVX-VNNI token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64AvxVnniToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64AvxVnniToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64AvxVnniToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::Yes;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64AvxVnniToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64AvxVnniToken {
    const NAME: &'static str = "x86-64 AVX-VNNI";
    const TARGET_FEATURES: &'static str = "sse,sse2,avxvnni";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxvnni";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxvnni";
}

impl X64AvxVnniToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 AVX-VNNI";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,avxvnni";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxvnni";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxvnni";

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

impl core::fmt::Display for X64AvxVnniToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64AvxVnniToken {
    #[inline(always)]
    fn as_x64_avx_vnni(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 AVX-IFMA token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64AvxIfmaToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64AvxIfmaToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64AvxIfmaToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::Yes;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64AvxIfmaToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64AvxIfmaToken {
    const NAME: &'static str = "x86-64 AVX-IFMA";
    const TARGET_FEATURES: &'static str = "sse,sse2,avxifma";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxifma";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxifma";
}

impl X64AvxIfmaToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 AVX-IFMA";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,avxifma";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxifma";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxifma";

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

impl core::fmt::Display for X64AvxIfmaToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64AvxIfmaToken {
    #[inline(always)]
    fn as_x64_avx_ifma(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 AVX-VNNI-INT8 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64AvxVnniInt8Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64AvxVnniInt8Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64AvxVnniInt8Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::Yes;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64AvxVnniInt8Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64AvxVnniInt8Token {
    const NAME: &'static str = "x86-64 AVX-VNNI-INT8";
    const TARGET_FEATURES: &'static str = "sse,sse2,avxvnniint8";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxvnniint8";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxvnniint8";
}

impl X64AvxVnniInt8Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 AVX-VNNI-INT8";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,avxvnniint8";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxvnniint8";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxvnniint8";

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

impl core::fmt::Display for X64AvxVnniInt8Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64AvxVnniInt8Token {
    #[inline(always)]
    fn as_x64_avx_vnni_int8(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 AVX-VNNI-INT16 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64AvxVnniInt16Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64AvxVnniInt16Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64AvxVnniInt16Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::Yes;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64AvxVnniInt16Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64AvxVnniInt16Token {
    const NAME: &'static str = "x86-64 AVX-VNNI-INT16";
    const TARGET_FEATURES: &'static str = "sse,sse2,avxvnniint16";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxvnniint16";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxvnniint16";
}

impl X64AvxVnniInt16Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 AVX-VNNI-INT16";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,avxvnniint16";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxvnniint16";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxvnniint16";

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

impl core::fmt::Display for X64AvxVnniInt16Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64AvxVnniInt16Token {
    #[inline(always)]
    fn as_x64_avx_vnni_int16(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 AVX-NE-CONVERT token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64AvxNeConvertToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64AvxNeConvertToken {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64AvxNeConvertToken {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::Yes;
    type Sha512 = super::caps::No;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64AvxNeConvertToken {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64AvxNeConvertToken {
    const NAME: &'static str = "x86-64 AVX-NE-CONVERT";
    const TARGET_FEATURES: &'static str = "sse,sse2,avxneconvert";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxneconvert";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxneconvert";
}

impl X64AvxNeConvertToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 AVX-NE-CONVERT";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,avxneconvert";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avxneconvert";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avxneconvert";

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

impl core::fmt::Display for X64AvxNeConvertToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64AvxNeConvertToken {
    #[inline(always)]
    fn as_x64_avx_ne_convert(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for x86-64 SHA512 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64Sha512Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for X64Sha512Token {
    type Arch = super::caps::X64Arch;
}

impl super::caps::X64Caps for X64Sha512Token {
    type V1 = super::caps::Yes;
    type V2 = super::caps::No;
    type Crypto128 = super::caps::No;
    type V3 = super::caps::No;
    type VpclmulVaes = super::caps::No;
    type Gfni = super::caps::No;
    type Sha = super::caps::No;
    type Adx = super::caps::No;
    type AvxVnni = super::caps::No;
    type AvxIfma = super::caps::No;
    type AvxVnniInt8 = super::caps::No;
    type AvxVnniInt16 = super::caps::No;
    type AvxNeConvert = super::caps::No;
    type Sha512 = super::caps::Yes;
    type V4 = super::caps::No;
    type V4x = super::caps::No;
    type Avx512Fp16 = super::caps::No;
    type Avx512Bf16 = super::caps::No;
}

impl SimdToken for X64Sha512Token {
    type __Caps = super::caps::X64CapSet<
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::Yes,
        super::caps::No,
        super::caps::No,
        super::caps::No,
        super::caps::No,
    >;
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

impl SimdLeafToken for X64Sha512Token {
    const NAME: &'static str = "x86-64 SHA512";
    const TARGET_FEATURES: &'static str = "sse,sse2,avx,sha512";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avx,+sha512";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avx,-sha512";
}

impl X64Sha512Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "x86-64 SHA512";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "sse,sse2,avx,sha512";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+sse,+sse2,+avx,+sha512";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-sse,-sse2,-avx,-sha512";

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

impl core::fmt::Display for X64Sha512Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for X64Sha512Token {
    #[inline(always)]
    fn as_x64_sha512(self) -> Option<Self> {
        Some(self)
    }
}

/// Type alias for [`X64V3Token`].
pub type Desktop64 = X64V3Token;

/// Type alias for [`X64V4Token`].
pub type Avx512Token = X64V4Token;

/// Type alias for [`X64V4Token`].
pub type Server64 = X64V4Token;

/// Type alias for [`X64V4xToken`].
pub type Avx512ModernToken = X64V4xToken;

impl X64V1Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x510E0DCD;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_510E0DCD: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x510E0DCD) as usize];
}

impl X64V2Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x8CCD97C6;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_8CCD97C6: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x8CCD97C6) as usize];
}

impl X64CryptoToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xF5F0FBF5;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_F5F0FBF5: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xF5F0FBF5) as usize];
}

impl X64V3Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xF38B284B;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_F38B284B: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xF38B284B) as usize];
}

impl X64V3CryptoToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x01EAE708;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_01EAE708: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x01EAE708) as usize];
}

impl X64V3GfniCryptoToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x49A7BE90;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_49A7BE90: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x49A7BE90) as usize];
}

impl X64V3VnniToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x256B958C;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_256B958C: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x256B958C) as usize];
}

impl X64V3Vnni2Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x82D823D4;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_82D823D4: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x82D823D4) as usize];
}

impl X64V4Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xFE1B900C;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_FE1B900C: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xFE1B900C) as usize];
}

impl Avx512Fp16Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x2F39FFC0;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_2F39FFC0: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x2F39FFC0) as usize];
}

impl X64Avx512Bf16Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x90F5AF32;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_90F5AF32: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x90F5AF32) as usize];
}

impl X64V4xToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x8F63232A;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_8F63232A: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x8F63232A) as usize];
}

impl X64GfniToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x2713339E;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_2713339E: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x2713339E) as usize];
}

impl X64ShaToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x87E0FD2A;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_87E0FD2A: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x87E0FD2A) as usize];
}

impl X64AdxToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x6090ECC5;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_6090ECC5: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x6090ECC5) as usize];
}

impl X64AvxVnniToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x79810440;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_79810440: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x79810440) as usize];
}

impl X64AvxIfmaToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x998C2B98;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_998C2B98: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x998C2B98) as usize];
}

impl X64AvxVnniInt8Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x8C8D6359;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_8C8D6359: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x8C8D6359) as usize];
}

impl X64AvxVnniInt16Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x6F320E60;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_6F320E60: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x6F320E60) as usize];
}

impl X64AvxNeConvertToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x220DA18B;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_220DA18B: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x220DA18B) as usize];
}

impl X64Sha512Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xEADE7158;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_EADE7158: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xEADE7158) as usize];
}

// Compile-time check: capability blanket impls cover every token's declared traits.
const _: () = {
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V1Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V2Token>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64V2Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64CryptoToken>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64CryptoToken>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<X64CryptoToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V3Token>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64V3Token>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<X64V3Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V3CryptoToken>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64V3CryptoToken>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<X64V3CryptoToken>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<X64V3CryptoToken>;
    }
    {
        const fn _assert<T: HasX64V3Crypto>() {}
        let _ = _assert::<X64V3CryptoToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V3GfniCryptoToken>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64V3GfniCryptoToken>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<X64V3GfniCryptoToken>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<X64V3GfniCryptoToken>;
    }
    {
        const fn _assert<T: HasX64V3Crypto>() {}
        let _ = _assert::<X64V3GfniCryptoToken>;
    }
    {
        const fn _assert<T: HasGfni>() {}
        let _ = _assert::<X64V3GfniCryptoToken>;
    }
    {
        const fn _assert<T: HasX64V3GfniCrypto>() {}
        let _ = _assert::<X64V3GfniCryptoToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasX64V3Crypto>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasGfni>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasX64V3GfniCrypto>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasAvxVnni>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasAdx>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasSha>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasX64V3Vnni>() {}
        let _ = _assert::<X64V3VnniToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasX64V3Crypto>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasGfni>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasX64V3GfniCrypto>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasAvxVnni>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasAdx>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasSha>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasX64V3Vnni>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasAvxVnniInt8>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasAvxNeConvert>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasAvxIfma>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasX64V3Vnni2>() {}
        let _ = _assert::<X64V3Vnni2Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V4Token>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64V4Token>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<X64V4Token>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<X64V4Token>;
    }
    {
        const fn _assert<T: HasX64V4>() {}
        let _ = _assert::<X64V4Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<Avx512Fp16Token>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<Avx512Fp16Token>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<Avx512Fp16Token>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<Avx512Fp16Token>;
    }
    {
        const fn _assert<T: HasX64V4>() {}
        let _ = _assert::<Avx512Fp16Token>;
    }
    {
        const fn _assert<T: HasAvx512Fp16>() {}
        let _ = _assert::<Avx512Fp16Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64Avx512Bf16Token>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64Avx512Bf16Token>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<X64Avx512Bf16Token>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<X64Avx512Bf16Token>;
    }
    {
        const fn _assert<T: HasX64V4>() {}
        let _ = _assert::<X64Avx512Bf16Token>;
    }
    {
        const fn _assert<T: HasAvx512Bf16>() {}
        let _ = _assert::<X64Avx512Bf16Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasX64V2>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasX64Crypto>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasX64V3>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasX64V3Crypto>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasGfni>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasX64V3GfniCrypto>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasX64V4>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasX64V4x>() {}
        let _ = _assert::<X64V4xToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64GfniToken>;
    }
    {
        const fn _assert<T: HasGfni>() {}
        let _ = _assert::<X64GfniToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64ShaToken>;
    }
    {
        const fn _assert<T: HasSha>() {}
        let _ = _assert::<X64ShaToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64AdxToken>;
    }
    {
        const fn _assert<T: HasAdx>() {}
        let _ = _assert::<X64AdxToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64AvxVnniToken>;
    }
    {
        const fn _assert<T: HasAvxVnni>() {}
        let _ = _assert::<X64AvxVnniToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64AvxIfmaToken>;
    }
    {
        const fn _assert<T: HasAvxIfma>() {}
        let _ = _assert::<X64AvxIfmaToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64AvxVnniInt8Token>;
    }
    {
        const fn _assert<T: HasAvxVnniInt8>() {}
        let _ = _assert::<X64AvxVnniInt8Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64AvxVnniInt16Token>;
    }
    {
        const fn _assert<T: HasAvxVnniInt16>() {}
        let _ = _assert::<X64AvxVnniInt16Token>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64AvxNeConvertToken>;
    }
    {
        const fn _assert<T: HasAvxNeConvert>() {}
        let _ = _assert::<X64AvxNeConvertToken>;
    }
    {
        const fn _assert<T: HasX64V1>() {}
        let _ = _assert::<X64Sha512Token>;
    }
    {
        const fn _assert<T: HasSha512>() {}
        let _ = _assert::<X64Sha512Token>;
    }
};
