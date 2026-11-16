//! Generated from token-registry.toml — DO NOT EDIT.
//!
//! Regenerate with: cargo xtask generate

use crate::tokens::{
    HasArm64V2, HasArm64V3, HasNeon, HasNeonAes, HasNeonBf16, HasNeonCrc, HasNeonDotProd,
    HasNeonFcma, HasNeonFhm, HasNeonFp16, HasNeonI8mm, HasNeonRdm, HasNeonSha2, HasNeonSha3,
};
use crate::tokens::{SimdLeafToken, SimdToken};
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

// Cache statics: 0 = unknown, 1 = unavailable, 2 = available
#[allow(dead_code)]
pub(super) static NEON_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_AES_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_AES_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_SHA2_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_SHA2_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_SHA3_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_SHA3_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_CRC_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_CRC_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_RDM_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_RDM_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_DOT_PROD_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_DOT_PROD_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_FP16_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_FP16_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_FCMA_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_FCMA_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_I8MM_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_I8MM_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_BF16_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_BF16_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static NEON_FHM_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static NEON_FHM_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static ARM64_V2_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static ARM64_V2_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static ARM64_V3_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static ARM64_V3_DISABLED: AtomicBool = AtomicBool::new(false);

/// Proof that NEON is available.
///
/// NEON is available on virtually all AArch64 processors, but requires
/// runtime detection via `summon()` unless compiled with `-Ctarget-feature=+neon`.
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
        #[cfg(all(target_feature = "neon", not(feature = "testable_dispatch")))]
        {
            Some(true)
        }
        #[cfg(not(all(target_feature = "neon", not(feature = "testable_dispatch"))))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(target_feature = "neon", not(feature = "testable_dispatch")))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(target_feature = "neon", not(feature = "testable_dispatch"))))]
        {
            match NEON_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_detect(),
            }
        }
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

impl NeonToken {
    /// Construct a NEON proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonToken {
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
    /// - `NeonAesToken`
    /// - `NeonSha2Token`
    /// - `NeonSha3Token`
    /// - `NeonCrcToken`
    /// - `NeonRdmToken`
    /// - `NeonDotProdToken`
    /// - `NeonFp16Token`
    /// - `NeonFcmaToken`
    /// - `NeonI8mmToken`
    /// - `NeonBf16Token`
    /// - `NeonFhmToken`
    /// - `Arm64V2Token`
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(target_feature = "neon", not(feature = "testable_dispatch")))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(target_feature = "neon", not(feature = "testable_dispatch"))))]
        {
            NEON_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled { 1 } else { 0 };
            NEON_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_AES_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_AES_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_SHA2_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_SHA2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_SHA3_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_SHA3_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_CRC_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_CRC_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_RDM_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_RDM_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_DOT_PROD_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_DOT_PROD_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_FP16_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_FP16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_FCMA_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_FCMA_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_I8MM_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_I8MM_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_BF16_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_BF16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_FHM_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_FHM_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V2_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(target_feature = "neon", not(feature = "testable_dispatch")))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(target_feature = "neon", not(feature = "testable_dispatch"))))]
        {
            Ok(NEON_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(target_feature = "neon", not(feature = "testable_dispatch"))))]
#[cold]
#[inline(never)]
fn neon_detect() -> Option<NeonToken> {
    let available = crate::is_aarch64_feature_available!("neon");
    let pre_disabled = NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonToken::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + AES is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_AES_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_aes_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_aes_detect(),
            }
        }
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

impl NeonAesToken {
    /// Construct a NEON+AES proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,aes")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonAesToken {
    /// Extract a NeonToken — guaranteed because NEON+AES implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+AES's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonAesToken {
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
    /// - `Arm64V2Token`
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_AES_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_AES_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V2_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_AES_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "aes",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_aes_detect() -> Option<NeonAesToken> {
    let available =
        crate::is_aarch64_feature_available!("neon") && crate::is_aarch64_feature_available!("aes");
    let pre_disabled =
        NEON_AES_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_AES_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_AES_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonAesToken::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + SHA-256 (`sha2`) is available on AArch64.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_SHA2_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_sha2_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_sha2_detect(),
            }
        }
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

impl NeonSha2Token {
    /// Construct a NEON+SHA2 proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,sha2")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonSha2Token {
    /// Extract a NeonToken — guaranteed because NEON+SHA2 implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+SHA2's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonSha2Token {
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
    /// - `Arm64V2Token`
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_SHA2_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_SHA2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V2_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_SHA2_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "sha2",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_sha2_detect() -> Option<NeonSha2Token> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("sha2");
    let pre_disabled =
        NEON_SHA2_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_SHA2_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_SHA2_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonSha2Token::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + SHA3 is available.
///
/// SHA3 extension is available on ARMv8.2-A and later.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "sha3",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "sha3",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "sha3",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "sha3",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_SHA3_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_sha3_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_sha3_detect(),
            }
        }
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

impl NeonSha3Token {
    /// Construct a NEON+SHA3 proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,sha3")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonSha3Token {
    /// Extract a NeonToken — guaranteed because NEON+SHA3 implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+SHA3's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonSha3Token {
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
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "sha3",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "sha3",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_SHA3_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_SHA3_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "sha3",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "sha3",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_SHA3_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "sha3",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_sha3_detect() -> Option<NeonSha3Token> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("sha3");
    let pre_disabled =
        NEON_SHA3_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_SHA3_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_SHA3_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonSha3Token::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + CRC is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_CRC_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_crc_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_crc_detect(),
            }
        }
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

impl NeonCrcToken {
    /// Construct a NEON+CRC proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,crc")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonCrcToken {
    /// Extract a NeonToken — guaranteed because NEON+CRC implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+CRC's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonCrcToken {
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
    /// - `Arm64V2Token`
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_CRC_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_CRC_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V2_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_CRC_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "crc",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_crc_detect() -> Option<NeonCrcToken> {
    let available =
        crate::is_aarch64_feature_available!("neon") && crate::is_aarch64_feature_available!("crc");
    let pre_disabled =
        NEON_CRC_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_CRC_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_CRC_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonCrcToken::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + RDM (rounding doubling multiply-accumulate, ARMv8.1-A) is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "rdm",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "rdm",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "rdm",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "rdm",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_RDM_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_rdm_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_rdm_detect(),
            }
        }
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

impl NeonRdmToken {
    /// Construct a NEON+RDM proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,rdm")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonRdmToken {
    /// Extract a NeonToken — guaranteed because NEON+RDM implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+RDM's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonRdmToken {
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
    /// - `Arm64V2Token`
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "rdm",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "rdm",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_RDM_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_RDM_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V2_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "rdm",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "rdm",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_RDM_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "rdm",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_rdm_detect() -> Option<NeonRdmToken> {
    let available =
        crate::is_aarch64_feature_available!("neon") && crate::is_aarch64_feature_available!("rdm");
    let pre_disabled =
        NEON_RDM_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_RDM_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_RDM_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonRdmToken::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + DotProd (`udot`/`sdot`, ARMv8.2-A) is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "dotprod",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "dotprod",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "dotprod",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "dotprod",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_DOT_PROD_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_dot_prod_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_dot_prod_detect(),
            }
        }
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

impl NeonDotProdToken {
    /// Construct a NEON+DotProd proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,dotprod")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonDotProdToken {
    /// Extract a NeonToken — guaranteed because NEON+DotProd implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+DotProd's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonDotProdToken {
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
    /// - `Arm64V2Token`
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "dotprod",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "dotprod",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_DOT_PROD_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_DOT_PROD_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V2_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "dotprod",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "dotprod",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_DOT_PROD_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "dotprod",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_dot_prod_detect() -> Option<NeonDotProdToken> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("dotprod");
    let pre_disabled =
        NEON_DOT_PROD_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_DOT_PROD_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_DOT_PROD_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonDotProdToken::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + FP16 vector arithmetic (`fp16`, ARMv8.2-A) is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fp16",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fp16",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fp16",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fp16",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_FP16_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_fp16_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_fp16_detect(),
            }
        }
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

impl NeonFp16Token {
    /// Construct a NEON+FP16 proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,fp16")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonFp16Token {
    /// Extract a NeonToken — guaranteed because NEON+FP16 implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+FP16's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonFp16Token {
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
    /// - `NeonFhmToken`
    /// - `Arm64V2Token`
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fp16",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fp16",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_FP16_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_FP16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                NEON_FHM_DISABLED.store(disabled, Ordering::Release);
            }
            NEON_FHM_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V2_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fp16",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fp16",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_FP16_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "fp16",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_fp16_detect() -> Option<NeonFp16Token> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("fp16");
    let pre_disabled =
        NEON_FP16_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_FP16_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_FP16_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonFp16Token::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + FCMA (complex multiply-add, ARMv8.3-A) is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fcma",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fcma",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fcma",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fcma",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_FCMA_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_fcma_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_fcma_detect(),
            }
        }
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

impl NeonFcmaToken {
    /// Construct a NEON+FCMA proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,fcma")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonFcmaToken {
    /// Extract a NeonToken — guaranteed because NEON+FCMA implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+FCMA's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonFcmaToken {
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
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fcma",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fcma",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_FCMA_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_FCMA_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fcma",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fcma",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_FCMA_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "fcma",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_fcma_detect() -> Option<NeonFcmaToken> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("fcma");
    let pre_disabled =
        NEON_FCMA_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_FCMA_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_FCMA_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonFcmaToken::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + I8MM (`vmmla` / `usdot` / `sudot`, ARMv8.6-A) is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "i8mm",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "i8mm",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "i8mm",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "i8mm",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_I8MM_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_i8mm_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_i8mm_detect(),
            }
        }
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

impl NeonI8mmToken {
    /// Construct a NEON+I8MM proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,i8mm")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonI8mmToken {
    /// Extract a NeonToken — guaranteed because NEON+I8MM implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+I8MM's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonI8mmToken {
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
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "i8mm",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "i8mm",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_I8MM_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_I8MM_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "i8mm",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "i8mm",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_I8MM_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "i8mm",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_i8mm_detect() -> Option<NeonI8mmToken> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("i8mm");
    let pre_disabled =
        NEON_I8MM_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_I8MM_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_I8MM_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonI8mmToken::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + BF16 (`vbfdot` / `vbfmmla`, ARMv8.6-A) is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_BF16_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_bf16_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_bf16_detect(),
            }
        }
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

impl NeonBf16Token {
    /// Construct a NEON+BF16 proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,bf16")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonBf16Token {
    /// Extract a NeonToken — guaranteed because NEON+BF16 implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+BF16's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonBf16Token {
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
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_BF16_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (NEON_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            NEON_BF16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_BF16_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "bf16",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_bf16_detect() -> Option<NeonBf16Token> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("bf16");
    let pre_disabled =
        NEON_BF16_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_BF16_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        NEON_BF16_DISABLED.load(Ordering::Acquire) || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonBf16Token::from_context() })
    } else {
        None
    }
}

/// Proof that NEON + FP16 + FHM (widening FP16-to-FP32 multiply-accumulate, ARMv8.4-A) is available.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fp16",
            target_feature = "fhm",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fp16",
            target_feature = "fhm",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fp16",
            target_feature = "fhm",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fp16",
            target_feature = "fhm",
            not(feature = "testable_dispatch")
        )))]
        {
            match NEON_FHM_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by neon_fhm_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => neon_fhm_detect(),
            }
        }
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

impl NeonFhmToken {
    /// Construct a NEON+FHM proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,fp16,fhm")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl NeonFhmToken {
    /// Extract a NeonFp16Token — guaranteed because NEON+FHM implies NEON+FP16.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_fp16(self) -> NeonFp16Token {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+FHM's full contract, a superset of NEON+FP16's.
        unsafe { NeonFp16Token::from_context() }
    }
    /// Extract a NeonToken — guaranteed because NEON+FHM implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // NEON+FHM's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl NeonFhmToken {
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
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fp16",
            target_feature = "fhm",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fp16",
            target_feature = "fhm",
            not(feature = "testable_dispatch")
        )))]
        {
            NEON_FHM_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (NEON_FP16_DISABLED.load(Ordering::Acquire)
                    || NEON_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            NEON_FHM_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "fp16",
            target_feature = "fhm",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "fp16",
            target_feature = "fhm",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(NEON_FHM_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "fp16",
    target_feature = "fhm",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn neon_fhm_detect() -> Option<NeonFhmToken> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("fp16")
        && crate::is_aarch64_feature_available!("fhm");
    let pre_disabled = NEON_FHM_DISABLED.load(Ordering::Acquire)
        || NEON_FP16_DISABLED.load(Ordering::Acquire)
        || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = NEON_FHM_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = NEON_FHM_DISABLED.load(Ordering::Acquire)
        || NEON_FP16_DISABLED.load(Ordering::Acquire)
        || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { NeonFhmToken::from_context() })
    } else {
        None
    }
}

/// Proof that the Arm64-v2 feature set is available.
///
/// Arm64-v2 is archmage's second ARM tier, covering: NEON, CRC, RDM, DotProd,
/// FP16, AES, SHA2. This targets the broadest modern ARM baseline.
///
/// Available on: Cortex-A55+, Apple M1+, Graviton 2+, all post-2017 ARM chips.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        )))]
        {
            match ARM64_V2_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by arm64_v2_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => arm64_v2_detect(),
            }
        }
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

impl Arm64V2Token {
    /// Construct a Arm64-v2 proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,crc,rdm,dotprod,fp16,aes,sha2")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl Arm64V2Token {
    /// Extract a NeonAesToken — guaranteed because Arm64-v2 implies NEON+AES.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_aes(self) -> NeonAesToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v2's full contract, a superset of NEON+AES's.
        unsafe { NeonAesToken::from_context() }
    }
    /// Extract a NeonCrcToken — guaranteed because Arm64-v2 implies NEON+CRC.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_crc(self) -> NeonCrcToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v2's full contract, a superset of NEON+CRC's.
        unsafe { NeonCrcToken::from_context() }
    }
    /// Extract a NeonDotProdToken — guaranteed because Arm64-v2 implies NEON+DotProd.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_dotprod(self) -> NeonDotProdToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v2's full contract, a superset of NEON+DotProd's.
        unsafe { NeonDotProdToken::from_context() }
    }
    /// Extract a NeonFp16Token — guaranteed because Arm64-v2 implies NEON+FP16.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_fp16(self) -> NeonFp16Token {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v2's full contract, a superset of NEON+FP16's.
        unsafe { NeonFp16Token::from_context() }
    }
    /// Extract a NeonRdmToken — guaranteed because Arm64-v2 implies NEON+RDM.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_rdm(self) -> NeonRdmToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v2's full contract, a superset of NEON+RDM's.
        unsafe { NeonRdmToken::from_context() }
    }
    /// Extract a NeonSha2Token — guaranteed because Arm64-v2 implies NEON+SHA2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_sha2(self) -> NeonSha2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v2's full contract, a superset of NEON+SHA2's.
        unsafe { NeonSha2Token::from_context() }
    }
    /// Extract a NeonToken — guaranteed because Arm64-v2 implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v2's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl Arm64V2Token {
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
    /// - `Arm64V3Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        )))]
        {
            ARM64_V2_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (NEON_CRC_DISABLED.load(Ordering::Acquire)
                    || NEON_RDM_DISABLED.load(Ordering::Acquire)
                    || NEON_DOT_PROD_DISABLED.load(Ordering::Acquire)
                    || NEON_FP16_DISABLED.load(Ordering::Acquire)
                    || NEON_AES_DISABLED.load(Ordering::Acquire)
                    || NEON_SHA2_DISABLED.load(Ordering::Acquire)
                    || NEON_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            ARM64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(ARM64_V2_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "crc",
    target_feature = "rdm",
    target_feature = "dotprod",
    target_feature = "fp16",
    target_feature = "aes",
    target_feature = "sha2",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn arm64_v2_detect() -> Option<Arm64V2Token> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("crc")
        && crate::is_aarch64_feature_available!("rdm")
        && crate::is_aarch64_feature_available!("dotprod")
        && crate::is_aarch64_feature_available!("fp16")
        && crate::is_aarch64_feature_available!("aes")
        && crate::is_aarch64_feature_available!("sha2");
    let pre_disabled = ARM64_V2_DISABLED.load(Ordering::Acquire)
        || NEON_CRC_DISABLED.load(Ordering::Acquire)
        || NEON_RDM_DISABLED.load(Ordering::Acquire)
        || NEON_DOT_PROD_DISABLED.load(Ordering::Acquire)
        || NEON_FP16_DISABLED.load(Ordering::Acquire)
        || NEON_AES_DISABLED.load(Ordering::Acquire)
        || NEON_SHA2_DISABLED.load(Ordering::Acquire)
        || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = ARM64_V2_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = ARM64_V2_DISABLED.load(Ordering::Acquire)
        || NEON_CRC_DISABLED.load(Ordering::Acquire)
        || NEON_RDM_DISABLED.load(Ordering::Acquire)
        || NEON_DOT_PROD_DISABLED.load(Ordering::Acquire)
        || NEON_FP16_DISABLED.load(Ordering::Acquire)
        || NEON_AES_DISABLED.load(Ordering::Acquire)
        || NEON_SHA2_DISABLED.load(Ordering::Acquire)
        || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { Arm64V2Token::from_context() })
    } else {
        None
    }
}

/// Proof that the full modern ARM SIMD feature set is available (Arm64-v3).
///
/// Arm64-v3 adds FHM, FCMA, SHA3, I8MM, and BF16 over Arm64-v2.
/// Available on: Cortex-A510+, Apple M2+, Snapdragon X, Graviton 3+, Cobalt 100.
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
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            target_feature = "fhm",
            target_feature = "fcma",
            target_feature = "sha3",
            target_feature = "i8mm",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            target_feature = "fhm",
            target_feature = "fcma",
            target_feature = "sha3",
            target_feature = "i8mm",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            None
        }
    }

    #[inline(always)]
    fn summon() -> Option<Self> {
        // Compile-time fast path (suppressed by testable_dispatch)
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            target_feature = "fhm",
            target_feature = "fcma",
            target_feature = "sha3",
            target_feature = "i8mm",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            // SAFETY: every feature this token asserts is enabled at
            // compile time (cfg(target_feature)), so the binary only
            // runs on CPUs that have them.
            Some(unsafe { Self::from_context() })
        }

        // Runtime path with caching
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            target_feature = "fhm",
            target_feature = "fcma",
            target_feature = "sha3",
            target_feature = "i8mm",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            match ARM64_V3_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by arm64_v3_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => arm64_v3_detect(),
            }
        }
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

impl Arm64V3Token {
    /// Construct a Arm64-v3 proof from the caller's statically proven
    /// feature context.
    ///
    /// A safe call requires a caller whose `#[target_feature]` attribute
    /// enables every feature of this tier; a superset is sufficient. Outside
    /// such a context rustc requires an `unsafe` block at the call site.
    ///
    /// Features enabled globally (`-C target-feature`, `-C target-cpu`) do
    /// not satisfy this — rustc requires them on the caller's own attribute.
    ///
    /// No runtime detection happens here, so this also bypasses
    /// process-wide token disabling (including `testable_dispatch`):
    /// the caller's feature context is already the proof. Use
    /// [`SimdToken::summon`](crate::SimdToken::summon) when the
    /// features have to be detected at runtime.
    ///
    /// Being a `#[target_feature]` function, this cannot be coerced to
    /// a safe function pointer — there would be no call site left for
    /// rustc to check.
    ///
    /// # Safety
    ///
    /// When called through an `unsafe` block, the caller must ensure
    /// every feature in this tier is available on the executing CPU.
    /// Safe calls have that obligation discharged by the compiler.
    #[inline]
    #[target_feature(enable = "neon,crc,rdm,dotprod,fp16,aes,sha2,fhm,fcma,sha3,i8mm,bf16")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl Arm64V3Token {
    /// Extract a Arm64V2Token — guaranteed because Arm64-v3 implies Arm64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn arm_v2(self) -> Arm64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of Arm64-v2's.
        unsafe { Arm64V2Token::from_context() }
    }
    /// Extract a NeonAesToken — guaranteed because Arm64-v3 implies NEON+AES.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_aes(self) -> NeonAesToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+AES's.
        unsafe { NeonAesToken::from_context() }
    }
    /// Extract a NeonBf16Token — guaranteed because Arm64-v3 implies NEON+BF16.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_bf16(self) -> NeonBf16Token {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+BF16's.
        unsafe { NeonBf16Token::from_context() }
    }
    /// Extract a NeonCrcToken — guaranteed because Arm64-v3 implies NEON+CRC.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_crc(self) -> NeonCrcToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+CRC's.
        unsafe { NeonCrcToken::from_context() }
    }
    /// Extract a NeonDotProdToken — guaranteed because Arm64-v3 implies NEON+DotProd.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_dotprod(self) -> NeonDotProdToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+DotProd's.
        unsafe { NeonDotProdToken::from_context() }
    }
    /// Extract a NeonFcmaToken — guaranteed because Arm64-v3 implies NEON+FCMA.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_fcma(self) -> NeonFcmaToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+FCMA's.
        unsafe { NeonFcmaToken::from_context() }
    }
    /// Extract a NeonFhmToken — guaranteed because Arm64-v3 implies NEON+FHM.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_fhm(self) -> NeonFhmToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+FHM's.
        unsafe { NeonFhmToken::from_context() }
    }
    /// Extract a NeonFp16Token — guaranteed because Arm64-v3 implies NEON+FP16.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_fp16(self) -> NeonFp16Token {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+FP16's.
        unsafe { NeonFp16Token::from_context() }
    }
    /// Extract a NeonI8mmToken — guaranteed because Arm64-v3 implies NEON+I8MM.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_i8mm(self) -> NeonI8mmToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+I8MM's.
        unsafe { NeonI8mmToken::from_context() }
    }
    /// Extract a NeonRdmToken — guaranteed because Arm64-v3 implies NEON+RDM.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_rdm(self) -> NeonRdmToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+RDM's.
        unsafe { NeonRdmToken::from_context() }
    }
    /// Extract a NeonSha2Token — guaranteed because Arm64-v3 implies NEON+SHA2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_sha2(self) -> NeonSha2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+SHA2's.
        unsafe { NeonSha2Token::from_context() }
    }
    /// Extract a NeonSha3Token — guaranteed because Arm64-v3 implies NEON+SHA3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon_sha3(self) -> NeonSha3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON+SHA3's.
        unsafe { NeonSha3Token::from_context() }
    }
    /// Extract a NeonToken — guaranteed because Arm64-v3 implies NEON.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn neon(self) -> NeonToken {
        // SAFETY: holding `self` proves this environment satisfies
        // Arm64-v3's full contract, a superset of NEON's.
        unsafe { NeonToken::from_context() }
    }
}

impl Arm64V3Token {
    /// Disable this token process-wide for testing and benchmarking.
    ///
    /// When disabled, `summon()` will return `None` even if the CPU supports
    /// the required features.
    ///
    /// Returns `Err` when all required features are compile-time determined
    /// (e.g., via `-Ctarget-cpu=native`), since the compiler has already
    /// elided the runtime checks.
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            target_feature = "fhm",
            target_feature = "fcma",
            target_feature = "sha3",
            target_feature = "i8mm",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            let _ = disabled;
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            target_feature = "fhm",
            target_feature = "fcma",
            target_feature = "sha3",
            target_feature = "i8mm",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            ARM64_V3_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (ARM64_V2_DISABLED.load(Ordering::Acquire)
                    || NEON_FHM_DISABLED.load(Ordering::Acquire)
                    || NEON_FCMA_DISABLED.load(Ordering::Acquire)
                    || NEON_SHA3_DISABLED.load(Ordering::Acquire)
                    || NEON_I8MM_DISABLED.load(Ordering::Acquire)
                    || NEON_BF16_DISABLED.load(Ordering::Acquire)
                    || NEON_CRC_DISABLED.load(Ordering::Acquire)
                    || NEON_RDM_DISABLED.load(Ordering::Acquire)
                    || NEON_DOT_PROD_DISABLED.load(Ordering::Acquire)
                    || NEON_FP16_DISABLED.load(Ordering::Acquire)
                    || NEON_AES_DISABLED.load(Ordering::Acquire)
                    || NEON_SHA2_DISABLED.load(Ordering::Acquire)
                    || NEON_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            ARM64_V3_CACHE.store(v, Ordering::Release);
            Ok(())
        }
    }

    /// Check if this token has been manually disabled process-wide.
    ///
    /// Returns `Err` when token availability is compile-time determined.
    #[inline(always)]
    #[allow(clippy::needless_return)]
    pub fn manually_disabled() -> Result<bool, crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            target_feature = "fhm",
            target_feature = "fcma",
            target_feature = "sha3",
            target_feature = "i8mm",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            return Err(crate::tokens::CompileTimeGuaranteedError {
                token_name: Self::NAME,
                target_features: Self::TARGET_FEATURES,
                disable_flags: Self::DISABLE_TARGET_FEATURES,
            });
        }
        #[cfg(not(all(
            target_feature = "neon",
            target_feature = "crc",
            target_feature = "rdm",
            target_feature = "dotprod",
            target_feature = "fp16",
            target_feature = "aes",
            target_feature = "sha2",
            target_feature = "fhm",
            target_feature = "fcma",
            target_feature = "sha3",
            target_feature = "i8mm",
            target_feature = "bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(ARM64_V3_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "neon",
    target_feature = "crc",
    target_feature = "rdm",
    target_feature = "dotprod",
    target_feature = "fp16",
    target_feature = "aes",
    target_feature = "sha2",
    target_feature = "fhm",
    target_feature = "fcma",
    target_feature = "sha3",
    target_feature = "i8mm",
    target_feature = "bf16",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn arm64_v3_detect() -> Option<Arm64V3Token> {
    let available = crate::is_aarch64_feature_available!("neon")
        && crate::is_aarch64_feature_available!("crc")
        && crate::is_aarch64_feature_available!("rdm")
        && crate::is_aarch64_feature_available!("dotprod")
        && crate::is_aarch64_feature_available!("fp16")
        && crate::is_aarch64_feature_available!("aes")
        && crate::is_aarch64_feature_available!("sha2")
        && crate::is_aarch64_feature_available!("fhm")
        && crate::is_aarch64_feature_available!("fcma")
        && crate::is_aarch64_feature_available!("sha3")
        && crate::is_aarch64_feature_available!("i8mm")
        && crate::is_aarch64_feature_available!("bf16");
    let pre_disabled = ARM64_V3_DISABLED.load(Ordering::Acquire)
        || ARM64_V2_DISABLED.load(Ordering::Acquire)
        || NEON_FHM_DISABLED.load(Ordering::Acquire)
        || NEON_FCMA_DISABLED.load(Ordering::Acquire)
        || NEON_SHA3_DISABLED.load(Ordering::Acquire)
        || NEON_I8MM_DISABLED.load(Ordering::Acquire)
        || NEON_BF16_DISABLED.load(Ordering::Acquire)
        || NEON_CRC_DISABLED.load(Ordering::Acquire)
        || NEON_RDM_DISABLED.load(Ordering::Acquire)
        || NEON_DOT_PROD_DISABLED.load(Ordering::Acquire)
        || NEON_FP16_DISABLED.load(Ordering::Acquire)
        || NEON_AES_DISABLED.load(Ordering::Acquire)
        || NEON_SHA2_DISABLED.load(Ordering::Acquire)
        || NEON_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = ARM64_V3_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = ARM64_V3_DISABLED.load(Ordering::Acquire)
        || ARM64_V2_DISABLED.load(Ordering::Acquire)
        || NEON_FHM_DISABLED.load(Ordering::Acquire)
        || NEON_FCMA_DISABLED.load(Ordering::Acquire)
        || NEON_SHA3_DISABLED.load(Ordering::Acquire)
        || NEON_I8MM_DISABLED.load(Ordering::Acquire)
        || NEON_BF16_DISABLED.load(Ordering::Acquire)
        || NEON_CRC_DISABLED.load(Ordering::Acquire)
        || NEON_RDM_DISABLED.load(Ordering::Acquire)
        || NEON_DOT_PROD_DISABLED.load(Ordering::Acquire)
        || NEON_FP16_DISABLED.load(Ordering::Acquire)
        || NEON_AES_DISABLED.load(Ordering::Acquire)
        || NEON_SHA2_DISABLED.load(Ordering::Acquire)
        || NEON_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { Arm64V3Token::from_context() })
    } else {
        None
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
