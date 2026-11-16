//! Generated from token-registry.toml — DO NOT EDIT.
//!
//! Stub tokens: `summon()` always returns `None`.

use crate::tokens::{
    HasArm64V2, HasArm64V3, HasNeon, HasNeonAes, HasNeonBf16, HasNeonCrc, HasNeonDotProd,
    HasNeonFcma, HasNeonFhm, HasNeonFp16, HasNeonI8mm, HasNeonRdm, HasNeonSha2, HasNeonSha3,
};
use crate::tokens::{SimdLeafToken, SimdToken};

/// Stub for NEON token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonToken {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonToken {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonToken {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonToken {
    const NAME: &'static str = "NEON";
    const TARGET_FEATURES: &'static str = "neon";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon";
}

impl NeonToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon";

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

impl core::fmt::Display for NeonToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonToken {
    #[inline(always)]
    fn as_neon(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+AES token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonAesToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonAesToken {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonAesToken {
    type Neon = super::caps::Yes;
    type Aes = super::caps::Yes;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonAesToken {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonAesToken {
    const NAME: &'static str = "NEON+AES";
    const TARGET_FEATURES: &'static str = "neon,aes";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+aes";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-aes";
}

impl NeonAesToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+AES";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,aes";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+aes";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-aes";

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

impl core::fmt::Display for NeonAesToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonAesToken {
    #[inline(always)]
    fn as_neon_aes(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+SHA2 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonSha2Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonSha2Token {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonSha2Token {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::Yes;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonSha2Token {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonSha2Token {
    const NAME: &'static str = "NEON+SHA2";
    const TARGET_FEATURES: &'static str = "neon,sha2";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+sha2";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-sha2";
}

impl NeonSha2Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+SHA2";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,sha2";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+sha2";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-sha2";

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

impl core::fmt::Display for NeonSha2Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonSha2Token {
    #[inline(always)]
    fn as_neon_sha2(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+SHA3 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonSha3Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonSha3Token {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonSha3Token {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::Yes;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonSha3Token {
    type __Caps = super::caps::ArmCapSet<
        super::caps::Yes,
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

impl SimdLeafToken for NeonSha3Token {
    const NAME: &'static str = "NEON+SHA3";
    const TARGET_FEATURES: &'static str = "neon,sha3";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+sha3";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-sha3";
}

impl NeonSha3Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+SHA3";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,sha3";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+sha3";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-sha3";

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

impl core::fmt::Display for NeonSha3Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonSha3Token {
    #[inline(always)]
    fn as_neon_sha3(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+CRC token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonCrcToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonCrcToken {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonCrcToken {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::Yes;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonCrcToken {
    type __Caps = super::caps::ArmCapSet<
        super::caps::Yes,
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

impl SimdLeafToken for NeonCrcToken {
    const NAME: &'static str = "NEON+CRC";
    const TARGET_FEATURES: &'static str = "neon,crc";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+crc";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-crc";
}

impl NeonCrcToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+CRC";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,crc";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+crc";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-crc";

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

impl core::fmt::Display for NeonCrcToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonCrcToken {
    #[inline(always)]
    fn as_neon_crc(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+RDM token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonRdmToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonRdmToken {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonRdmToken {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::Yes;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonRdmToken {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonRdmToken {
    const NAME: &'static str = "NEON+RDM";
    const TARGET_FEATURES: &'static str = "neon,rdm";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+rdm";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-rdm";
}

impl NeonRdmToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+RDM";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,rdm";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+rdm";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-rdm";

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

impl core::fmt::Display for NeonRdmToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonRdmToken {
    #[inline(always)]
    fn as_neon_rdm(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+DotProd token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonDotProdToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonDotProdToken {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonDotProdToken {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::Yes;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonDotProdToken {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonDotProdToken {
    const NAME: &'static str = "NEON+DotProd";
    const TARGET_FEATURES: &'static str = "neon,dotprod";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+dotprod";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-dotprod";
}

impl NeonDotProdToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+DotProd";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,dotprod";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+dotprod";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-dotprod";

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

impl core::fmt::Display for NeonDotProdToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonDotProdToken {
    #[inline(always)]
    fn as_neon_dotprod(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+FP16 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonFp16Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonFp16Token {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonFp16Token {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::Yes;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonFp16Token {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonFp16Token {
    const NAME: &'static str = "NEON+FP16";
    const TARGET_FEATURES: &'static str = "neon,fp16";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+fp16";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-fp16";
}

impl NeonFp16Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+FP16";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,fp16";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+fp16";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-fp16";

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

impl core::fmt::Display for NeonFp16Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonFp16Token {
    #[inline(always)]
    fn as_neon_fp16(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+FCMA token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonFcmaToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonFcmaToken {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonFcmaToken {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::Yes;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonFcmaToken {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonFcmaToken {
    const NAME: &'static str = "NEON+FCMA";
    const TARGET_FEATURES: &'static str = "neon,fcma";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+fcma";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-fcma";
}

impl NeonFcmaToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+FCMA";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,fcma";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+fcma";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-fcma";

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

impl core::fmt::Display for NeonFcmaToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonFcmaToken {
    #[inline(always)]
    fn as_neon_fcma(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+I8MM token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonI8mmToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonI8mmToken {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonI8mmToken {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::Yes;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonI8mmToken {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonI8mmToken {
    const NAME: &'static str = "NEON+I8MM";
    const TARGET_FEATURES: &'static str = "neon,i8mm";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+i8mm";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-i8mm";
}

impl NeonI8mmToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+I8MM";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,i8mm";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+i8mm";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-i8mm";

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

impl core::fmt::Display for NeonI8mmToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonI8mmToken {
    #[inline(always)]
    fn as_neon_i8mm(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+BF16 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonBf16Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonBf16Token {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonBf16Token {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::No;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::Yes;
}

impl SimdToken for NeonBf16Token {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for NeonBf16Token {
    const NAME: &'static str = "NEON+BF16";
    const TARGET_FEATURES: &'static str = "neon,bf16";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+bf16";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-bf16";
}

impl NeonBf16Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+BF16";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,bf16";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+bf16";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-bf16";

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

impl core::fmt::Display for NeonBf16Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonBf16Token {
    #[inline(always)]
    fn as_neon_bf16(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for NEON+FHM token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeonFhmToken {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for NeonFhmToken {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for NeonFhmToken {
    type Neon = super::caps::Yes;
    type Aes = super::caps::No;
    type Sha2 = super::caps::No;
    type Sha3 = super::caps::No;
    type Crc = super::caps::No;
    type Rdm = super::caps::No;
    type DotProd = super::caps::No;
    type Fp16 = super::caps::Yes;
    type Fhm = super::caps::Yes;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for NeonFhmToken {
    type __Caps = super::caps::ArmCapSet<
        super::caps::Yes,
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

impl SimdLeafToken for NeonFhmToken {
    const NAME: &'static str = "NEON+FHM";
    const TARGET_FEATURES: &'static str = "neon,fp16,fhm";
    const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+fp16,+fhm";
    const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-fp16,-fhm";
}

impl NeonFhmToken {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "NEON+FHM";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,fp16,fhm";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=+neon,+fp16,+fhm";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str = "-Ctarget-feature=-neon,-fp16,-fhm";

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

impl core::fmt::Display for NeonFhmToken {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for NeonFhmToken {
    #[inline(always)]
    fn as_neon_fhm(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for Arm64-v2 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Arm64V2Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for Arm64V2Token {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for Arm64V2Token {
    type Neon = super::caps::Yes;
    type Aes = super::caps::Yes;
    type Sha2 = super::caps::Yes;
    type Sha3 = super::caps::No;
    type Crc = super::caps::Yes;
    type Rdm = super::caps::Yes;
    type DotProd = super::caps::Yes;
    type Fp16 = super::caps::Yes;
    type Fhm = super::caps::No;
    type Fcma = super::caps::No;
    type I8mm = super::caps::No;
    type Bf16 = super::caps::No;
}

impl SimdToken for Arm64V2Token {
    type __Caps = super::caps::ArmCapSet<
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::No,
        super::caps::Yes,
        super::caps::Yes,
        super::caps::Yes,
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

impl SimdLeafToken for Arm64V2Token {
    const NAME: &'static str = "Arm64-v2";
    const TARGET_FEATURES: &'static str = "neon,crc,rdm,dotprod,fp16,aes,sha2";
    const ENABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=+neon,+crc,+rdm,+dotprod,+fp16,+aes,+sha2";
    const DISABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=-neon,-crc,-rdm,-dotprod,-fp16,-aes,-sha2";
}

impl Arm64V2Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "Arm64-v2";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str = "neon,crc,rdm,dotprod,fp16,aes,sha2";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=+neon,+crc,+rdm,+dotprod,+fp16,+aes,+sha2";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=-neon,-crc,-rdm,-dotprod,-fp16,-aes,-sha2";

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

impl core::fmt::Display for Arm64V2Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for Arm64V2Token {
    #[inline(always)]
    fn as_arm_v2(self) -> Option<Self> {
        Some(self)
    }
}

/// Stub for Arm64-v3 token (not available on this architecture).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Arm64V3Token {
    pub(in crate::tokens::generated) _private: (),
}

impl crate::tokens::Sealed for Arm64V3Token {
    type Arch = super::caps::ArmArch;
}

impl super::caps::ArmCaps for Arm64V3Token {
    type Neon = super::caps::Yes;
    type Aes = super::caps::Yes;
    type Sha2 = super::caps::Yes;
    type Sha3 = super::caps::Yes;
    type Crc = super::caps::Yes;
    type Rdm = super::caps::Yes;
    type DotProd = super::caps::Yes;
    type Fp16 = super::caps::Yes;
    type Fhm = super::caps::Yes;
    type Fcma = super::caps::Yes;
    type I8mm = super::caps::Yes;
    type Bf16 = super::caps::Yes;
}

impl SimdToken for Arm64V3Token {
    type __Caps = super::caps::ArmCapSet<
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

impl SimdLeafToken for Arm64V3Token {
    const NAME: &'static str = "Arm64-v3";
    const TARGET_FEATURES: &'static str =
        "neon,crc,rdm,dotprod,fp16,aes,sha2,fhm,fcma,sha3,i8mm,bf16";
    const ENABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=+neon,+crc,+rdm,+dotprod,+fp16,+aes,+sha2,+fhm,+fcma,+sha3,+i8mm,+bf16";
    const DISABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=-neon,-crc,-rdm,-dotprod,-fp16,-aes,-sha2,-fhm,-fcma,-sha3,-i8mm,-bf16";
}

impl Arm64V3Token {
    /// Human-readable name for diagnostics and error messages.
    pub const NAME: &'static str = "Arm64-v3";

    /// Comma-delimited target features asserted by this token.
    pub const TARGET_FEATURES: &'static str =
        "neon,crc,rdm,dotprod,fp16,aes,sha2,fhm,fcma,sha3,i8mm,bf16";

    /// RUSTFLAGS to enable these features at compile time.
    pub const ENABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=+neon,+crc,+rdm,+dotprod,+fp16,+aes,+sha2,+fhm,+fcma,+sha3,+i8mm,+bf16";

    /// RUSTFLAGS to disable these features at compile time.
    pub const DISABLE_TARGET_FEATURES: &'static str =
        "-Ctarget-feature=-neon,-crc,-rdm,-dotprod,-fp16,-aes,-sha2,-fhm,-fcma,-sha3,-i8mm,-bf16";

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

impl core::fmt::Display for Arm64V3Token {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as SimdToken>::write_name(f)
    }
}

impl crate::tokens::IntoConcreteToken for Arm64V3Token {
    #[inline(always)]
    fn as_arm_v3(self) -> Option<Self> {
        Some(self)
    }
}

/// Type alias for [`NeonToken`].
pub type Arm64 = NeonToken;

impl NeonToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x72CB52B2;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_72CB52B2: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x72CB52B2) as usize];
}

impl NeonAesToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x8C16863D;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_8C16863D: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x8C16863D) as usize];
}

impl NeonSha2Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x2D17C59A;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_2D17C59A: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x2D17C59A) as usize];
}

impl NeonSha3Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x8215198F;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_8215198F: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x8215198F) as usize];
}

impl NeonCrcToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x5C2B1B4E;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_5C2B1B4E: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x5C2B1B4E) as usize];
}

impl NeonRdmToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x4EAE741F;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_4EAE741F: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x4EAE741F) as usize];
}

impl NeonDotProdToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xC5BAFACA;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_C5BAFACA: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xC5BAFACA) as usize];
}

impl NeonFp16Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x0B5D99DD;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_0B5D99DD: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x0B5D99DD) as usize];
}

impl NeonFcmaToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x1651125D;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_1651125D: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x1651125D) as usize];
}

impl NeonI8mmToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x5CCC31B7;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_5CCC31B7: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x5CCC31B7) as usize];
}

impl NeonBf16Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xBD1175BF;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_BD1175BF: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xBD1175BF) as usize];
}

impl NeonFhmToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x31BD919F;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_31BD919F: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x31BD919F) as usize];
}

impl Arm64V2Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xB0231590;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_B0231590: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xB0231590) as usize];
}

impl Arm64V3Token {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xB2F6E2D5;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_B2F6E2D5: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xB2F6E2D5) as usize];
}

// Compile-time check: capability blanket impls cover every token's declared traits.
const _: () = {
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonToken>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonAesToken>;
    }
    {
        const fn _assert<T: HasNeonAes>() {}
        let _ = _assert::<NeonAesToken>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonSha2Token>;
    }
    {
        const fn _assert<T: HasNeonSha2>() {}
        let _ = _assert::<NeonSha2Token>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonSha3Token>;
    }
    {
        const fn _assert<T: HasNeonSha3>() {}
        let _ = _assert::<NeonSha3Token>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonCrcToken>;
    }
    {
        const fn _assert<T: HasNeonCrc>() {}
        let _ = _assert::<NeonCrcToken>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonRdmToken>;
    }
    {
        const fn _assert<T: HasNeonRdm>() {}
        let _ = _assert::<NeonRdmToken>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonDotProdToken>;
    }
    {
        const fn _assert<T: HasNeonDotProd>() {}
        let _ = _assert::<NeonDotProdToken>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonFp16Token>;
    }
    {
        const fn _assert<T: HasNeonFp16>() {}
        let _ = _assert::<NeonFp16Token>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonFcmaToken>;
    }
    {
        const fn _assert<T: HasNeonFcma>() {}
        let _ = _assert::<NeonFcmaToken>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonI8mmToken>;
    }
    {
        const fn _assert<T: HasNeonI8mm>() {}
        let _ = _assert::<NeonI8mmToken>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonBf16Token>;
    }
    {
        const fn _assert<T: HasNeonBf16>() {}
        let _ = _assert::<NeonBf16Token>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<NeonFhmToken>;
    }
    {
        const fn _assert<T: HasNeonFp16>() {}
        let _ = _assert::<NeonFhmToken>;
    }
    {
        const fn _assert<T: HasNeonFhm>() {}
        let _ = _assert::<NeonFhmToken>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<Arm64V2Token>;
    }
    {
        const fn _assert<T: HasNeonAes>() {}
        let _ = _assert::<Arm64V2Token>;
    }
    {
        const fn _assert<T: HasNeonSha2>() {}
        let _ = _assert::<Arm64V2Token>;
    }
    {
        const fn _assert<T: HasNeonCrc>() {}
        let _ = _assert::<Arm64V2Token>;
    }
    {
        const fn _assert<T: HasNeonRdm>() {}
        let _ = _assert::<Arm64V2Token>;
    }
    {
        const fn _assert<T: HasNeonDotProd>() {}
        let _ = _assert::<Arm64V2Token>;
    }
    {
        const fn _assert<T: HasNeonFp16>() {}
        let _ = _assert::<Arm64V2Token>;
    }
    {
        const fn _assert<T: HasArm64V2>() {}
        let _ = _assert::<Arm64V2Token>;
    }
    {
        const fn _assert<T: HasNeon>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonAes>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonSha2>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonSha3>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonCrc>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonRdm>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonDotProd>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonFp16>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonFhm>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonFcma>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonI8mm>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasNeonBf16>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasArm64V2>() {}
        let _ = _assert::<Arm64V3Token>;
    }
    {
        const fn _assert<T: HasArm64V3>() {}
        let _ = _assert::<Arm64V3Token>;
    }
};
