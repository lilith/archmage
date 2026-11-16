//! Generated from token-registry.toml — DO NOT EDIT.
//!
//! Regenerate with: cargo xtask generate

use crate::tokens::{
    HasAdx, HasAvx512Bf16, HasAvx512Fp16, HasAvxIfma, HasAvxNeConvert, HasAvxVnni, HasAvxVnniInt8,
    HasAvxVnniInt16, HasGfni, HasSha, HasSha512, HasX64Crypto, HasX64V1, HasX64V2, HasX64V3,
    HasX64V3Crypto, HasX64V3GfniCrypto, HasX64V3Vnni, HasX64V3Vnni2, HasX64V4, HasX64V4x,
};
use crate::tokens::{SimdLeafToken, SimdToken};
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

// Cache statics: 0 = unknown, 1 = unavailable, 2 = available
#[allow(dead_code)]
pub(super) static X64_V1_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V1_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_V2_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V2_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_CRYPTO_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_CRYPTO_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_V3_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V3_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_V3_CRYPTO_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V3_CRYPTO_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_V3_GFNI_CRYPTO_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V3_GFNI_CRYPTO_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_V3_VNNI_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V3_VNNI_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_V3_VNNI2_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V3_VNNI2_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_V4_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V4_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static AVX512_FP16_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static AVX512_FP16_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_AVX512_BF16_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_AVX512_BF16_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_V4X_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_V4X_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_GFNI_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_GFNI_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_SHA_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_SHA_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_ADX_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_ADX_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_AVX_VNNI_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_AVX_VNNI_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_AVX_IFMA_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_AVX_IFMA_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_AVX_VNNI_INT8_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_AVX_VNNI_INT8_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_AVX_VNNI_INT16_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_AVX_VNNI_INT16_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_AVX_NE_CONVERT_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_AVX_NE_CONVERT_DISABLED: AtomicBool = AtomicBool::new(false);
#[allow(dead_code)]
pub(super) static X64_SHA512_CACHE: AtomicU8 = AtomicU8::new(0);
#[allow(dead_code)]
pub(super) static X64_SHA512_DISABLED: AtomicBool = AtomicBool::new(false);

/// Proof that SSE + SSE2 are available (x86-64-v1 baseline level).
///
/// SSE2 is the x86_64 ABI baseline — every x86_64 CPU has it. However, Rust
/// still requires #[target_feature(enable = "sse2")] for SSE2 intrinsics to
/// be safe. This token provides that via `#[arcane]`.
///
/// On x86_64, summon() always returns Some. On 32-bit x86, returns None
/// (x86_64 tokens are not available on 32-bit).
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
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
            target_feature = "sse",
            target_feature = "sse2",
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
            target_feature = "sse",
            target_feature = "sse2",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V1_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v1_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v1_detect(),
            }
        }
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

impl X64V1Token {
    /// Construct a x86-64-v1 proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V1Token {
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
    /// - `X64V2Token`
    /// - `X64CryptoToken`
    /// - `X64V3Token`
    /// - `X64V3CryptoToken`
    /// - `X64V3GfniCryptoToken`
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    /// - `X64V4Token`
    /// - `Avx512Fp16Token`
    /// - `X64Avx512Bf16Token`
    /// - `X64V4xToken`
    /// - `X64GfniToken`
    /// - `X64ShaToken`
    /// - `X64AdxToken`
    /// - `X64AvxVnniToken`
    /// - `X64AvxIfmaToken`
    /// - `X64AvxVnniInt8Token`
    /// - `X64AvxVnniInt16Token`
    /// - `X64AvxNeConvertToken`
    /// - `X64Sha512Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
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
            target_feature = "sse",
            target_feature = "sse2",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V1_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled { 1 } else { 0 };
            X64_V1_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_GFNI_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_GFNI_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                AVX512_FP16_DISABLED.store(disabled, Ordering::Release);
            }
            AVX512_FP16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX512_BF16_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX512_BF16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4X_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4X_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_GFNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_GFNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_SHA_DISABLED.store(disabled, Ordering::Release);
            }
            X64_SHA_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_ADX_DISABLED.store(disabled, Ordering::Release);
            }
            X64_ADX_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX_IFMA_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX_IFMA_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX_VNNI_INT8_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX_VNNI_INT8_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX_VNNI_INT16_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX_VNNI_INT16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX_NE_CONVERT_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX_NE_CONVERT_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_SHA512_DISABLED.store(disabled, Ordering::Release);
            }
            X64_SHA512_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
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
            target_feature = "sse",
            target_feature = "sse2",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V1_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v1_detect() -> Option<X64V1Token> {
    let available =
        crate::is_x86_feature_available!("sse") && crate::is_x86_feature_available!("sse2");
    let pre_disabled = X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V1_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V1Token::from_context() })
    } else {
        None
    }
}

/// Proof that SSE4.2 + POPCNT are available (x86-64-v2 level).
///
/// x86-64-v2 implies: SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2, POPCNT, CX16, SAHF.
/// This is the Nehalem (2008) / Bulldozer (2011) baseline.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V2_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v2_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v2_detect(),
            }
        }
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

impl X64V2Token {
    /// Construct a x86-64-v2 proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V2Token {
    /// Extract a X64V1Token — guaranteed because x86-64-v2 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v2's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64V2Token {
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
    /// - `X64CryptoToken`
    /// - `X64V3Token`
    /// - `X64V3CryptoToken`
    /// - `X64V3GfniCryptoToken`
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    /// - `X64V4Token`
    /// - `Avx512Fp16Token`
    /// - `X64Avx512Bf16Token`
    /// - `X64V4xToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V2_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_V2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_GFNI_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_GFNI_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                AVX512_FP16_DISABLED.store(disabled, Ordering::Release);
            }
            AVX512_FP16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX512_BF16_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX512_BF16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4X_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4X_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V2_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v2_detect() -> Option<X64V2Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b");
    let pre_disabled =
        X64_V2_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V2_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        X64_V2_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V2Token::from_context() })
    } else {
        None
    }
}

/// Proof that PCLMULQDQ + AES-NI are available (on top of x86-64-v2).
///
/// PCLMULQDQ and AES-NI are not part of the psABI v2 spec but ship on every
/// x86-64 CPU since Westmere (2010). Original Nehalem (2008) and some VMs
/// (QEMU x86-64-v2, VMware EVC Nehalem baseline) lack them.
///
/// Use this token for CRC-32 folding, AES encryption, and GF(2) polynomial
/// arithmetic. Available on: Westmere+, Bulldozer+, Silvermont+, all Zen.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "pclmulqdq",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "pclmulqdq",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_CRYPTO_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_crypto_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_crypto_detect(),
            }
        }
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

impl X64CryptoToken {
    /// Construct a x86-64 Crypto proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,pclmulqdq,aes")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64CryptoToken {
    /// Extract a X64V1Token — guaranteed because x86-64 Crypto implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 Crypto's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because x86-64 Crypto implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 Crypto's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
}

impl X64CryptoToken {
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
    /// - `X64V3CryptoToken`
    /// - `X64V3GfniCryptoToken`
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    /// - `X64V4Token`
    /// - `Avx512Fp16Token`
    /// - `X64Avx512Bf16Token`
    /// - `X64V4xToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "pclmulqdq",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V2_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_GFNI_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_GFNI_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                AVX512_FP16_DISABLED.store(disabled, Ordering::Release);
            }
            AVX512_FP16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX512_BF16_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX512_BF16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4X_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4X_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "pclmulqdq",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_CRYPTO_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_crypto_detect() -> Option<X64CryptoToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes");
    let pre_disabled = X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_CRYPTO_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64CryptoToken::from_context() })
    } else {
        None
    }
}

/// Proof that AVX2 + FMA + BMI1/2 + F16C + LZCNT are available (x86-64-v3 level).
///
/// x86-64-v3 implies all of v2 plus: AVX, AVX2, FMA, BMI1, BMI2, F16C, LZCNT, MOVBE.
/// This is the Haswell (2013) / Zen 1 (2017) baseline.
///
/// This is the most commonly targeted level for high-performance SIMD code.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V3_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v3_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v3_detect(),
            }
        }
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

impl X64V3Token {
    /// Construct a x86-64-v3 proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V3Token {
    /// Extract a X64V1Token — guaranteed because x86-64-v3 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because x86-64-v3 implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
}

impl X64V3Token {
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
    /// - `X64V3CryptoToken`
    /// - `X64V3GfniCryptoToken`
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    /// - `X64V4Token`
    /// - `Avx512Fp16Token`
    /// - `X64Avx512Bf16Token`
    /// - `X64V4xToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V3_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V2_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_V3_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_GFNI_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_GFNI_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                AVX512_FP16_DISABLED.store(disabled, Ordering::Release);
            }
            AVX512_FP16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX512_BF16_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX512_BF16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4X_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4X_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V3_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v3_detect() -> Option<X64V3Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe");
    let pre_disabled = X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V3_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V3Token::from_context() })
    } else {
        None
    }
}

/// Proof that AVX2 + VPCLMULQDQ + VAES are available.
///
/// VPCLMULQDQ (256-bit carryless multiply) and VAES (256-bit AES) extend
/// PCLMULQDQ/AES-NI to 256-bit vectors. Available without AVX-512 on:
/// Zen 3+ (2020), Alder Lake P-core (2021), Raptor Lake (2022).
///
/// Use for 256-bit CRC-32 folding, AES-CTR/GCM, and GF(2) polynomial
/// arithmetic on CPUs that lack AVX-512.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V3_CRYPTO_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v3_crypto_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v3_crypto_detect(),
            }
        }
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

impl X64V3CryptoToken {
    /// Construct a x86-64-v3 Crypto proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V3CryptoToken {
    /// Extract a X64CryptoToken — guaranteed because x86-64-v3 Crypto implies x86-64 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_crypto(self) -> X64CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 Crypto's full contract, a superset of x86-64 Crypto's.
        unsafe { X64CryptoToken::from_context() }
    }
    /// Extract a X64V1Token — guaranteed because x86-64-v3 Crypto implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 Crypto's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because x86-64-v3 Crypto implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 Crypto's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
    /// Extract a X64V3Token — guaranteed because x86-64-v3 Crypto implies x86-64-v3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3(self) -> X64V3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 Crypto's full contract, a superset of x86-64-v3's.
        unsafe { X64V3Token::from_context() }
    }
}

impl X64V3CryptoToken {
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
    /// - `X64V3GfniCryptoToken`
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    /// - `X64V4xToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V3_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V3_DISABLED.load(Ordering::Acquire)
                    || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V2_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_V3_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_GFNI_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_GFNI_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4X_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4X_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    target_feature = "vpclmulqdq",
    target_feature = "vaes",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v3_crypto_detect() -> Option<X64V3CryptoToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes")
        && crate::is_x86_feature_available!("vpclmulqdq")
        && crate::is_x86_feature_available!("vaes");
    let pre_disabled = X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V3_CRYPTO_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V3CryptoToken::from_context() })
    } else {
        None
    }
}

/// Proof that AVX2 + GFNI + VPCLMULQDQ + VAES are available.
///
/// GFNI provides GF(2^8) affine transforms and field multiplication over the
/// AES/Rijndael polynomial. Its 256-bit VEX forms operate on YMM registers without
/// requiring AVX-512. At the x86-64-v3 level, GFNI-capable CPUs also provide the
/// AES, PCLMULQDQ, VAES, and VPCLMULQDQ features inherited from
/// `X64V3CryptoToken`.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V3_GFNI_CRYPTO_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v3_gfni_crypto_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v3_gfni_crypto_detect(),
            }
        }
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

impl X64V3GfniCryptoToken {
    /// Construct a x86-64-v3 GFNI Crypto proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V3GfniCryptoToken {
    /// Extract a X64CryptoToken — guaranteed because x86-64-v3 GFNI Crypto implies x86-64 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_crypto(self) -> X64CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 GFNI Crypto's full contract, a superset of x86-64 Crypto's.
        unsafe { X64CryptoToken::from_context() }
    }
    /// Extract a X64GfniToken — guaranteed because x86-64-v3 GFNI Crypto implies x86-64 GFNI.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_gfni(self) -> X64GfniToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 GFNI Crypto's full contract, a superset of x86-64 GFNI's.
        unsafe { X64GfniToken::from_context() }
    }

    /// Get a X64GfniToken (alias for `.x64_gfni()`)
    #[inline(always)]
    pub fn gfni(self) -> X64GfniToken {
        // SAFETY: identical to `.x64_gfni()` — self's contract is a
        // registry-verified superset of x86-64 GFNI's.
        unsafe { X64GfniToken::from_context() }
    }
    /// Extract a X64V1Token — guaranteed because x86-64-v3 GFNI Crypto implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 GFNI Crypto's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because x86-64-v3 GFNI Crypto implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 GFNI Crypto's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
    /// Extract a X64V3CryptoToken — guaranteed because x86-64-v3 GFNI Crypto implies x86-64-v3 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3_crypto(self) -> X64V3CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 GFNI Crypto's full contract, a superset of x86-64-v3 Crypto's.
        unsafe { X64V3CryptoToken::from_context() }
    }
    /// Extract a X64V3Token — guaranteed because x86-64-v3 GFNI Crypto implies x86-64-v3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3(self) -> X64V3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 GFNI Crypto's full contract, a superset of x86-64-v3's.
        unsafe { X64V3Token::from_context() }
    }
}

impl X64V3GfniCryptoToken {
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
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    /// - `X64V4xToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V3_GFNI_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_GFNI_DISABLED.load(Ordering::Acquire)
                    || X64_V3_DISABLED.load(Ordering::Acquire)
                    || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire)
                    || X64_V2_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_V3_GFNI_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4X_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4X_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    target_feature = "vpclmulqdq",
    target_feature = "vaes",
    target_feature = "gfni",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v3_gfni_crypto_detect() -> Option<X64V3GfniCryptoToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes")
        && crate::is_x86_feature_available!("vpclmulqdq")
        && crate::is_x86_feature_available!("vaes")
        && crate::is_x86_feature_available!("gfni");
    let pre_disabled = X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_GFNI_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V3_GFNI_CRYPTO_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_GFNI_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V3GfniCryptoToken::from_context() })
    } else {
        None
    }
}

/// Proof that VEX-encoded AVX-VNNI (`avxvnni`), `adx`, and `sha` are available on top of `X64V3GfniCryptoToken`.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V3_VNNI_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v3_vnni_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v3_vnni_detect(),
            }
        }
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

impl X64V3VnniToken {
    /// Construct a x86-64-v3 VNNI proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni,avxvnni,adx,sha"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V3VnniToken {
    /// Extract a X64AdxToken — guaranteed because x86-64-v3 VNNI implies x86-64 ADX.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_adx(self) -> X64AdxToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64 ADX's.
        unsafe { X64AdxToken::from_context() }
    }

    /// Get a X64AdxToken (alias for `.x64_adx()`)
    #[inline(always)]
    pub fn adx(self) -> X64AdxToken {
        // SAFETY: identical to `.x64_adx()` — self's contract is a
        // registry-verified superset of x86-64 ADX's.
        unsafe { X64AdxToken::from_context() }
    }
    /// Extract a X64AvxVnniToken — guaranteed because x86-64-v3 VNNI implies x86-64 AVX-VNNI.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_avx_vnni(self) -> X64AvxVnniToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64 AVX-VNNI's.
        unsafe { X64AvxVnniToken::from_context() }
    }
    /// Extract a X64CryptoToken — guaranteed because x86-64-v3 VNNI implies x86-64 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_crypto(self) -> X64CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64 Crypto's.
        unsafe { X64CryptoToken::from_context() }
    }
    /// Extract a X64GfniToken — guaranteed because x86-64-v3 VNNI implies x86-64 GFNI.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_gfni(self) -> X64GfniToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64 GFNI's.
        unsafe { X64GfniToken::from_context() }
    }

    /// Get a X64GfniToken (alias for `.x64_gfni()`)
    #[inline(always)]
    pub fn gfni(self) -> X64GfniToken {
        // SAFETY: identical to `.x64_gfni()` — self's contract is a
        // registry-verified superset of x86-64 GFNI's.
        unsafe { X64GfniToken::from_context() }
    }
    /// Extract a X64ShaToken — guaranteed because x86-64-v3 VNNI implies x86-64 SHA.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_sha(self) -> X64ShaToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64 SHA's.
        unsafe { X64ShaToken::from_context() }
    }

    /// Get a X64ShaToken (alias for `.x64_sha()`)
    #[inline(always)]
    pub fn sha(self) -> X64ShaToken {
        // SAFETY: identical to `.x64_sha()` — self's contract is a
        // registry-verified superset of x86-64 SHA's.
        unsafe { X64ShaToken::from_context() }
    }
    /// Extract a X64V1Token — guaranteed because x86-64-v3 VNNI implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because x86-64-v3 VNNI implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
    /// Extract a X64V3CryptoToken — guaranteed because x86-64-v3 VNNI implies x86-64-v3 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3_crypto(self) -> X64V3CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64-v3 Crypto's.
        unsafe { X64V3CryptoToken::from_context() }
    }
    /// Extract a X64V3GfniCryptoToken — guaranteed because x86-64-v3 VNNI implies x86-64-v3 GFNI Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3_gfni_crypto(self) -> X64V3GfniCryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64-v3 GFNI Crypto's.
        unsafe { X64V3GfniCryptoToken::from_context() }
    }
    /// Extract a X64V3Token — guaranteed because x86-64-v3 VNNI implies x86-64-v3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3(self) -> X64V3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI's full contract, a superset of x86-64-v3's.
        unsafe { X64V3Token::from_context() }
    }
}

impl X64V3VnniToken {
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
    /// - `X64V3Vnni2Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_AVX_VNNI_DISABLED.load(Ordering::Acquire)
                    || X64_ADX_DISABLED.load(Ordering::Acquire)
                    || X64_SHA_DISABLED.load(Ordering::Acquire)
                    || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_GFNI_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire)
                    || X64_V3_DISABLED.load(Ordering::Acquire)
                    || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V2_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V3_VNNI_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    target_feature = "vpclmulqdq",
    target_feature = "vaes",
    target_feature = "gfni",
    target_feature = "avxvnni",
    target_feature = "adx",
    target_feature = "sha",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v3_vnni_detect() -> Option<X64V3VnniToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes")
        && crate::is_x86_feature_available!("vpclmulqdq")
        && crate::is_x86_feature_available!("vaes")
        && crate::is_x86_feature_available!("gfni")
        && crate::is_x86_feature_available!("avxvnni")
        && crate::is_x86_feature_available!("adx")
        && crate::is_x86_feature_available!("sha");
    let pre_disabled = X64_V3_VNNI_DISABLED.load(Ordering::Acquire)
        || X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_AVX_VNNI_DISABLED.load(Ordering::Acquire)
        || X64_ADX_DISABLED.load(Ordering::Acquire)
        || X64_SHA_DISABLED.load(Ordering::Acquire)
        || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_GFNI_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V3_VNNI_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_V3_VNNI_DISABLED.load(Ordering::Acquire)
        || X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_AVX_VNNI_DISABLED.load(Ordering::Acquire)
        || X64_ADX_DISABLED.load(Ordering::Acquire)
        || X64_SHA_DISABLED.load(Ordering::Acquire)
        || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_GFNI_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V3VnniToken::from_context() })
    } else {
        None
    }
}

/// Proof that x86-64-v3 VNNI plus cross-vendor AVX-VNNI-2 extensions are available.
///
/// Extends [`X64V3VnniToken`] with `avxvnniint8`, `avxneconvert`, and `avxifma`
/// (pure VEX encoding — does not require AVX-512).
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            target_feature = "avxvnniint8",
            target_feature = "avxneconvert",
            target_feature = "avxifma",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            target_feature = "avxvnniint8",
            target_feature = "avxneconvert",
            target_feature = "avxifma",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            target_feature = "avxvnniint8",
            target_feature = "avxneconvert",
            target_feature = "avxifma",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            target_feature = "avxvnniint8",
            target_feature = "avxneconvert",
            target_feature = "avxifma",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V3_VNNI2_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v3_vnni2_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v3_vnni2_detect(),
            }
        }
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

impl X64V3Vnni2Token {
    /// Construct a x86-64-v3 VNNI-2 proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,vpclmulqdq,vaes,gfni,avxvnni,adx,sha,avxvnniint8,avxneconvert,avxifma"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V3Vnni2Token {
    /// Extract a X64AdxToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64 ADX.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_adx(self) -> X64AdxToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64 ADX's.
        unsafe { X64AdxToken::from_context() }
    }

    /// Get a X64AdxToken (alias for `.x64_adx()`)
    #[inline(always)]
    pub fn adx(self) -> X64AdxToken {
        // SAFETY: identical to `.x64_adx()` — self's contract is a
        // registry-verified superset of x86-64 ADX's.
        unsafe { X64AdxToken::from_context() }
    }
    /// Extract a X64AvxIfmaToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64 AVX-IFMA.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_avx_ifma(self) -> X64AvxIfmaToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64 AVX-IFMA's.
        unsafe { X64AvxIfmaToken::from_context() }
    }
    /// Extract a X64AvxNeConvertToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64 AVX-NE-CONVERT.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_avx_ne_convert(self) -> X64AvxNeConvertToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64 AVX-NE-CONVERT's.
        unsafe { X64AvxNeConvertToken::from_context() }
    }
    /// Extract a X64AvxVnniInt8Token — guaranteed because x86-64-v3 VNNI-2 implies x86-64 AVX-VNNI-INT8.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_avx_vnni_int8(self) -> X64AvxVnniInt8Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64 AVX-VNNI-INT8's.
        unsafe { X64AvxVnniInt8Token::from_context() }
    }
    /// Extract a X64AvxVnniToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64 AVX-VNNI.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_avx_vnni(self) -> X64AvxVnniToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64 AVX-VNNI's.
        unsafe { X64AvxVnniToken::from_context() }
    }
    /// Extract a X64CryptoToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_crypto(self) -> X64CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64 Crypto's.
        unsafe { X64CryptoToken::from_context() }
    }
    /// Extract a X64GfniToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64 GFNI.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_gfni(self) -> X64GfniToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64 GFNI's.
        unsafe { X64GfniToken::from_context() }
    }

    /// Get a X64GfniToken (alias for `.x64_gfni()`)
    #[inline(always)]
    pub fn gfni(self) -> X64GfniToken {
        // SAFETY: identical to `.x64_gfni()` — self's contract is a
        // registry-verified superset of x86-64 GFNI's.
        unsafe { X64GfniToken::from_context() }
    }
    /// Extract a X64ShaToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64 SHA.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_sha(self) -> X64ShaToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64 SHA's.
        unsafe { X64ShaToken::from_context() }
    }

    /// Get a X64ShaToken (alias for `.x64_sha()`)
    #[inline(always)]
    pub fn sha(self) -> X64ShaToken {
        // SAFETY: identical to `.x64_sha()` — self's contract is a
        // registry-verified superset of x86-64 SHA's.
        unsafe { X64ShaToken::from_context() }
    }
    /// Extract a X64V1Token — guaranteed because x86-64-v3 VNNI-2 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because x86-64-v3 VNNI-2 implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
    /// Extract a X64V3CryptoToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64-v3 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3_crypto(self) -> X64V3CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64-v3 Crypto's.
        unsafe { X64V3CryptoToken::from_context() }
    }
    /// Extract a X64V3GfniCryptoToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64-v3 GFNI Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3_gfni_crypto(self) -> X64V3GfniCryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64-v3 GFNI Crypto's.
        unsafe { X64V3GfniCryptoToken::from_context() }
    }
    /// Extract a X64V3Token — guaranteed because x86-64-v3 VNNI-2 implies x86-64-v3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3(self) -> X64V3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64-v3's.
        unsafe { X64V3Token::from_context() }
    }
    /// Extract a X64V3VnniToken — guaranteed because x86-64-v3 VNNI-2 implies x86-64-v3 VNNI.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3_vnni(self) -> X64V3VnniToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v3 VNNI-2's full contract, a superset of x86-64-v3 VNNI's.
        unsafe { X64V3VnniToken::from_context() }
    }
}

impl X64V3Vnni2Token {
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            target_feature = "avxvnniint8",
            target_feature = "avxneconvert",
            target_feature = "avxifma",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            target_feature = "avxvnniint8",
            target_feature = "avxneconvert",
            target_feature = "avxifma",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V3_VNNI_DISABLED.load(Ordering::Acquire)
                    || X64_AVX_VNNI_INT8_DISABLED.load(Ordering::Acquire)
                    || X64_AVX_NE_CONVERT_DISABLED.load(Ordering::Acquire)
                    || X64_AVX_IFMA_DISABLED.load(Ordering::Acquire)
                    || X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_AVX_VNNI_DISABLED.load(Ordering::Acquire)
                    || X64_ADX_DISABLED.load(Ordering::Acquire)
                    || X64_SHA_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire)
                    || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_GFNI_DISABLED.load(Ordering::Acquire)
                    || X64_V3_DISABLED.load(Ordering::Acquire)
                    || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V2_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            target_feature = "avxvnniint8",
            target_feature = "avxneconvert",
            target_feature = "avxifma",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "vpclmulqdq",
            target_feature = "vaes",
            target_feature = "gfni",
            target_feature = "avxvnni",
            target_feature = "adx",
            target_feature = "sha",
            target_feature = "avxvnniint8",
            target_feature = "avxneconvert",
            target_feature = "avxifma",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V3_VNNI2_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    target_feature = "vpclmulqdq",
    target_feature = "vaes",
    target_feature = "gfni",
    target_feature = "avxvnni",
    target_feature = "adx",
    target_feature = "sha",
    target_feature = "avxvnniint8",
    target_feature = "avxneconvert",
    target_feature = "avxifma",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v3_vnni2_detect() -> Option<X64V3Vnni2Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes")
        && crate::is_x86_feature_available!("vpclmulqdq")
        && crate::is_x86_feature_available!("vaes")
        && crate::is_x86_feature_available!("gfni")
        && crate::is_x86_feature_available!("avxvnni")
        && crate::is_x86_feature_available!("adx")
        && crate::is_x86_feature_available!("sha")
        && crate::is_x86_feature_available!("avxvnniint8")
        && crate::is_x86_feature_available!("avxneconvert")
        && crate::is_x86_feature_available!("avxifma");
    let pre_disabled = X64_V3_VNNI2_DISABLED.load(Ordering::Acquire)
        || X64_V3_VNNI_DISABLED.load(Ordering::Acquire)
        || X64_AVX_VNNI_INT8_DISABLED.load(Ordering::Acquire)
        || X64_AVX_NE_CONVERT_DISABLED.load(Ordering::Acquire)
        || X64_AVX_IFMA_DISABLED.load(Ordering::Acquire)
        || X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_AVX_VNNI_DISABLED.load(Ordering::Acquire)
        || X64_ADX_DISABLED.load(Ordering::Acquire)
        || X64_SHA_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire)
        || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_GFNI_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V3_VNNI2_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_V3_VNNI2_DISABLED.load(Ordering::Acquire)
        || X64_V3_VNNI_DISABLED.load(Ordering::Acquire)
        || X64_AVX_VNNI_INT8_DISABLED.load(Ordering::Acquire)
        || X64_AVX_NE_CONVERT_DISABLED.load(Ordering::Acquire)
        || X64_AVX_IFMA_DISABLED.load(Ordering::Acquire)
        || X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_AVX_VNNI_DISABLED.load(Ordering::Acquire)
        || X64_ADX_DISABLED.load(Ordering::Acquire)
        || X64_SHA_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire)
        || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_GFNI_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V3Vnni2Token::from_context() })
    } else {
        None
    }
}

/// Proof that AVX-512 (F + CD + VL + DQ + BW) is available.
///
/// This is the complete x86-64-v4 AVX-512 feature set, available on:
/// - Intel Skylake-X (2017+), Ice Lake, Sapphire Rapids
/// - AMD Zen 4+ (2022+)
///
/// Note: Intel 12th-14th gen consumer CPUs do NOT have AVX-512.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V4_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v4_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v4_detect(),
            }
        }
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

impl X64V4Token {
    /// Construct a AVX-512 proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V4Token {
    /// Extract a X64CryptoToken — guaranteed because AVX-512 implies x86-64 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_crypto(self) -> X64CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512's full contract, a superset of x86-64 Crypto's.
        unsafe { X64CryptoToken::from_context() }
    }
    /// Extract a X64V1Token — guaranteed because AVX-512 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because AVX-512 implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
    /// Extract a X64V3Token — guaranteed because AVX-512 implies x86-64-v3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3(self) -> X64V3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512's full contract, a superset of x86-64-v3's.
        unsafe { X64V3Token::from_context() }
    }
}

impl X64V4Token {
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
    /// - `Avx512Fp16Token`
    /// - `X64Avx512Bf16Token`
    /// - `X64V4xToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V4_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V3_DISABLED.load(Ordering::Acquire)
                    || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V2_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_V4_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                AVX512_FP16_DISABLED.store(disabled, Ordering::Release);
            }
            AVX512_FP16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_AVX512_BF16_DISABLED.store(disabled, Ordering::Release);
            }
            X64_AVX512_BF16_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4X_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4X_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V4_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "avx512cd",
    target_feature = "avx512dq",
    target_feature = "avx512vl",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v4_detect() -> Option<X64V4Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes")
        && crate::is_x86_feature_available!("avx512f")
        && crate::is_x86_feature_available!("avx512bw")
        && crate::is_x86_feature_available!("avx512cd")
        && crate::is_x86_feature_available!("avx512dq")
        && crate::is_x86_feature_available!("avx512vl");
    let pre_disabled = X64_V4_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V4_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_V4_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V4Token::from_context() })
    } else {
        None
    }
}

/// Proof that AVX-512 FP16 (half-precision) is available.
///
/// Available on Intel Sapphire Rapids (2023+), Emerald Rapids.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512fp16",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512fp16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512fp16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512fp16",
            not(feature = "testable_dispatch")
        )))]
        {
            match AVX512_FP16_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by avx512_fp16_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => avx512_fp16_detect(),
            }
        }
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

impl Avx512Fp16Token {
    /// Construct a AVX-512FP16 proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512fp16"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl Avx512Fp16Token {
    /// Extract a X64CryptoToken — guaranteed because AVX-512FP16 implies x86-64 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_crypto(self) -> X64CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512FP16's full contract, a superset of x86-64 Crypto's.
        unsafe { X64CryptoToken::from_context() }
    }
    /// Extract a X64V1Token — guaranteed because AVX-512FP16 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512FP16's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because AVX-512FP16 implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512FP16's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
    /// Extract a X64V3Token — guaranteed because AVX-512FP16 implies x86-64-v3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3(self) -> X64V3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512FP16's full contract, a superset of x86-64-v3's.
        unsafe { X64V3Token::from_context() }
    }
    /// Extract a X64V4Token — guaranteed because AVX-512FP16 implies AVX-512.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v4(self) -> X64V4Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512FP16's full contract, a superset of AVX-512's.
        unsafe { X64V4Token::from_context() }
    }

    /// Get a X64V4Token (alias for `.v4()`)
    #[inline(always)]
    pub fn avx512(self) -> X64V4Token {
        // SAFETY: identical to `.v4()` — self's contract is a
        // registry-verified superset of AVX-512's.
        unsafe { X64V4Token::from_context() }
    }
}

impl Avx512Fp16Token {
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512fp16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512fp16",
            not(feature = "testable_dispatch")
        )))]
        {
            AVX512_FP16_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V4_DISABLED.load(Ordering::Acquire)
                    || X64_V3_DISABLED.load(Ordering::Acquire)
                    || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V2_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            AVX512_FP16_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512fp16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512fp16",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(AVX512_FP16_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "avx512cd",
    target_feature = "avx512dq",
    target_feature = "avx512vl",
    target_feature = "avx512fp16",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn avx512_fp16_detect() -> Option<Avx512Fp16Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes")
        && crate::is_x86_feature_available!("avx512f")
        && crate::is_x86_feature_available!("avx512bw")
        && crate::is_x86_feature_available!("avx512cd")
        && crate::is_x86_feature_available!("avx512dq")
        && crate::is_x86_feature_available!("avx512vl")
        && crate::is_x86_feature_available!("avx512fp16");
    let pre_disabled = AVX512_FP16_DISABLED.load(Ordering::Acquire)
        || X64_V4_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = AVX512_FP16_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = AVX512_FP16_DISABLED.load(Ordering::Acquire)
        || X64_V4_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { Avx512Fp16Token::from_context() })
    } else {
        None
    }
}

/// Proof that AVX-512 BF16 (BFloat16 dot products & conversions) is available.
///
/// Available on Intel Cooper Lake / Sapphire Rapids+ and AMD Zen 4+.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512bf16",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512bf16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512bf16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_AVX512_BF16_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_avx512_bf16_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_avx512_bf16_detect(),
            }
        }
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

impl X64Avx512Bf16Token {
    /// Construct a AVX-512 BF16 proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512bf16"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64Avx512Bf16Token {
    /// Extract a X64CryptoToken — guaranteed because AVX-512 BF16 implies x86-64 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_crypto(self) -> X64CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512 BF16's full contract, a superset of x86-64 Crypto's.
        unsafe { X64CryptoToken::from_context() }
    }
    /// Extract a X64V1Token — guaranteed because AVX-512 BF16 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512 BF16's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because AVX-512 BF16 implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512 BF16's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
    /// Extract a X64V3Token — guaranteed because AVX-512 BF16 implies x86-64-v3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3(self) -> X64V3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512 BF16's full contract, a superset of x86-64-v3's.
        unsafe { X64V3Token::from_context() }
    }
    /// Extract a X64V4Token — guaranteed because AVX-512 BF16 implies AVX-512.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v4(self) -> X64V4Token {
        // SAFETY: holding `self` proves this environment satisfies
        // AVX-512 BF16's full contract, a superset of AVX-512's.
        unsafe { X64V4Token::from_context() }
    }

    /// Get a X64V4Token (alias for `.v4()`)
    #[inline(always)]
    pub fn avx512(self) -> X64V4Token {
        // SAFETY: identical to `.v4()` — self's contract is a
        // registry-verified superset of AVX-512's.
        unsafe { X64V4Token::from_context() }
    }
}

impl X64Avx512Bf16Token {
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512bf16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_AVX512_BF16_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V4_DISABLED.load(Ordering::Acquire)
                    || X64_V3_DISABLED.load(Ordering::Acquire)
                    || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V2_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_AVX512_BF16_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512bf16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512bf16",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_AVX512_BF16_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "avx512cd",
    target_feature = "avx512dq",
    target_feature = "avx512vl",
    target_feature = "avx512bf16",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_avx512_bf16_detect() -> Option<X64Avx512Bf16Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes")
        && crate::is_x86_feature_available!("avx512f")
        && crate::is_x86_feature_available!("avx512bw")
        && crate::is_x86_feature_available!("avx512cd")
        && crate::is_x86_feature_available!("avx512dq")
        && crate::is_x86_feature_available!("avx512vl")
        && crate::is_x86_feature_available!("avx512bf16");
    let pre_disabled = X64_AVX512_BF16_DISABLED.load(Ordering::Acquire)
        || X64_V4_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_AVX512_BF16_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_AVX512_BF16_DISABLED.load(Ordering::Acquire)
        || X64_V4_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64Avx512Bf16Token::from_context() })
    } else {
        None
    }
}

/// Proof that extended AVX-512 features are available (x86-64-v4x = Ice Lake / Zen 4 level).
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512vpopcntdq",
            target_feature = "avx512ifma",
            target_feature = "avx512vbmi",
            target_feature = "avx512vbmi2",
            target_feature = "avx512bitalg",
            target_feature = "avx512vnni",
            target_feature = "vpclmulqdq",
            target_feature = "gfni",
            target_feature = "vaes",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512vpopcntdq",
            target_feature = "avx512ifma",
            target_feature = "avx512vbmi",
            target_feature = "avx512vbmi2",
            target_feature = "avx512bitalg",
            target_feature = "avx512vnni",
            target_feature = "vpclmulqdq",
            target_feature = "gfni",
            target_feature = "vaes",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512vpopcntdq",
            target_feature = "avx512ifma",
            target_feature = "avx512vbmi",
            target_feature = "avx512vbmi2",
            target_feature = "avx512bitalg",
            target_feature = "avx512vnni",
            target_feature = "vpclmulqdq",
            target_feature = "gfni",
            target_feature = "vaes",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512vpopcntdq",
            target_feature = "avx512ifma",
            target_feature = "avx512vbmi",
            target_feature = "avx512vbmi2",
            target_feature = "avx512bitalg",
            target_feature = "avx512vnni",
            target_feature = "vpclmulqdq",
            target_feature = "gfni",
            target_feature = "vaes",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_V4X_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_v4x_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_v4x_detect(),
            }
        }
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

impl X64V4xToken {
    /// Construct a x86-64-v4x proof from the caller's statically proven
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
    #[target_feature(
        enable = "sse,sse2,sse3,ssse3,sse4.1,sse4.2,popcnt,cmpxchg16b,avx,avx2,fma,bmi1,bmi2,f16c,lzcnt,movbe,pclmulqdq,aes,avx512f,avx512bw,avx512cd,avx512dq,avx512vl,avx512vpopcntdq,avx512ifma,avx512vbmi,avx512vbmi2,avx512bitalg,avx512vnni,vpclmulqdq,gfni,vaes"
    )]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64V4xToken {
    /// Extract a X64CryptoToken — guaranteed because x86-64-v4x implies x86-64 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_crypto(self) -> X64CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v4x's full contract, a superset of x86-64 Crypto's.
        unsafe { X64CryptoToken::from_context() }
    }
    /// Extract a X64GfniToken — guaranteed because x86-64-v4x implies x86-64 GFNI.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn x64_gfni(self) -> X64GfniToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v4x's full contract, a superset of x86-64 GFNI's.
        unsafe { X64GfniToken::from_context() }
    }

    /// Get a X64GfniToken (alias for `.x64_gfni()`)
    #[inline(always)]
    pub fn gfni(self) -> X64GfniToken {
        // SAFETY: identical to `.x64_gfni()` — self's contract is a
        // registry-verified superset of x86-64 GFNI's.
        unsafe { X64GfniToken::from_context() }
    }
    /// Extract a X64V1Token — guaranteed because x86-64-v4x implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v4x's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
    /// Extract a X64V2Token — guaranteed because x86-64-v4x implies x86-64-v2.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v2(self) -> X64V2Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v4x's full contract, a superset of x86-64-v2's.
        unsafe { X64V2Token::from_context() }
    }
    /// Extract a X64V3CryptoToken — guaranteed because x86-64-v4x implies x86-64-v3 Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3_crypto(self) -> X64V3CryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v4x's full contract, a superset of x86-64-v3 Crypto's.
        unsafe { X64V3CryptoToken::from_context() }
    }
    /// Extract a X64V3GfniCryptoToken — guaranteed because x86-64-v4x implies x86-64-v3 GFNI Crypto.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3_gfni_crypto(self) -> X64V3GfniCryptoToken {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v4x's full contract, a superset of x86-64-v3 GFNI Crypto's.
        unsafe { X64V3GfniCryptoToken::from_context() }
    }
    /// Extract a X64V3Token — guaranteed because x86-64-v4x implies x86-64-v3.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v3(self) -> X64V3Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v4x's full contract, a superset of x86-64-v3's.
        unsafe { X64V3Token::from_context() }
    }
    /// Extract a X64V4Token — guaranteed because x86-64-v4x implies AVX-512.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v4(self) -> X64V4Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64-v4x's full contract, a superset of AVX-512's.
        unsafe { X64V4Token::from_context() }
    }

    /// Get a X64V4Token (alias for `.v4()`)
    #[inline(always)]
    pub fn avx512(self) -> X64V4Token {
        // SAFETY: identical to `.v4()` — self's contract is a
        // registry-verified superset of AVX-512's.
        unsafe { X64V4Token::from_context() }
    }
}

impl X64V4xToken {
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512vpopcntdq",
            target_feature = "avx512ifma",
            target_feature = "avx512vbmi",
            target_feature = "avx512vbmi2",
            target_feature = "avx512bitalg",
            target_feature = "avx512vnni",
            target_feature = "vpclmulqdq",
            target_feature = "gfni",
            target_feature = "vaes",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512vpopcntdq",
            target_feature = "avx512ifma",
            target_feature = "avx512vbmi",
            target_feature = "avx512vbmi2",
            target_feature = "avx512bitalg",
            target_feature = "avx512vnni",
            target_feature = "vpclmulqdq",
            target_feature = "gfni",
            target_feature = "vaes",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_V4X_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled
                || (X64_V4_DISABLED.load(Ordering::Acquire)
                    || X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V3_DISABLED.load(Ordering::Acquire)
                    || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
                    || X64_GFNI_DISABLED.load(Ordering::Acquire)
                    || X64_V2_DISABLED.load(Ordering::Acquire)
                    || X64_V1_DISABLED.load(Ordering::Acquire))
            {
                1
            } else {
                0
            };
            X64_V4X_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512vpopcntdq",
            target_feature = "avx512ifma",
            target_feature = "avx512vbmi",
            target_feature = "avx512vbmi2",
            target_feature = "avx512bitalg",
            target_feature = "avx512vnni",
            target_feature = "vpclmulqdq",
            target_feature = "gfni",
            target_feature = "vaes",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sse3",
            target_feature = "ssse3",
            target_feature = "sse4.1",
            target_feature = "sse4.2",
            target_feature = "popcnt",
            target_feature = "cmpxchg16b",
            target_feature = "avx",
            target_feature = "avx2",
            target_feature = "fma",
            target_feature = "bmi1",
            target_feature = "bmi2",
            target_feature = "f16c",
            target_feature = "lzcnt",
            target_feature = "movbe",
            target_feature = "pclmulqdq",
            target_feature = "aes",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "avx512cd",
            target_feature = "avx512dq",
            target_feature = "avx512vl",
            target_feature = "avx512vpopcntdq",
            target_feature = "avx512ifma",
            target_feature = "avx512vbmi",
            target_feature = "avx512vbmi2",
            target_feature = "avx512bitalg",
            target_feature = "avx512vnni",
            target_feature = "vpclmulqdq",
            target_feature = "gfni",
            target_feature = "vaes",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_V4X_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse3",
    target_feature = "ssse3",
    target_feature = "sse4.1",
    target_feature = "sse4.2",
    target_feature = "popcnt",
    target_feature = "cmpxchg16b",
    target_feature = "avx",
    target_feature = "avx2",
    target_feature = "fma",
    target_feature = "bmi1",
    target_feature = "bmi2",
    target_feature = "f16c",
    target_feature = "lzcnt",
    target_feature = "movbe",
    target_feature = "pclmulqdq",
    target_feature = "aes",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "avx512cd",
    target_feature = "avx512dq",
    target_feature = "avx512vl",
    target_feature = "avx512vpopcntdq",
    target_feature = "avx512ifma",
    target_feature = "avx512vbmi",
    target_feature = "avx512vbmi2",
    target_feature = "avx512bitalg",
    target_feature = "avx512vnni",
    target_feature = "vpclmulqdq",
    target_feature = "gfni",
    target_feature = "vaes",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_v4x_detect() -> Option<X64V4xToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sse3")
        && crate::is_x86_feature_available!("ssse3")
        && crate::is_x86_feature_available!("sse4.1")
        && crate::is_x86_feature_available!("sse4.2")
        && crate::is_x86_feature_available!("popcnt")
        && crate::is_x86_feature_available!("cmpxchg16b")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("avx2")
        && crate::is_x86_feature_available!("fma")
        && crate::is_x86_feature_available!("bmi1")
        && crate::is_x86_feature_available!("bmi2")
        && crate::is_x86_feature_available!("f16c")
        && crate::is_x86_feature_available!("lzcnt")
        && crate::is_x86_feature_available!("movbe")
        && crate::is_x86_feature_available!("pclmulqdq")
        && crate::is_x86_feature_available!("aes")
        && crate::is_x86_feature_available!("avx512f")
        && crate::is_x86_feature_available!("avx512bw")
        && crate::is_x86_feature_available!("avx512cd")
        && crate::is_x86_feature_available!("avx512dq")
        && crate::is_x86_feature_available!("avx512vl")
        && crate::is_x86_feature_available!("avx512vpopcntdq")
        && crate::is_x86_feature_available!("avx512ifma")
        && crate::is_x86_feature_available!("avx512vbmi")
        && crate::is_x86_feature_available!("avx512vbmi2")
        && crate::is_x86_feature_available!("avx512bitalg")
        && crate::is_x86_feature_available!("avx512vnni")
        && crate::is_x86_feature_available!("vpclmulqdq")
        && crate::is_x86_feature_available!("gfni")
        && crate::is_x86_feature_available!("vaes");
    let pre_disabled = X64_V4X_DISABLED.load(Ordering::Acquire)
        || X64_V4_DISABLED.load(Ordering::Acquire)
        || X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_GFNI_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_V4X_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_V4X_DISABLED.load(Ordering::Acquire)
        || X64_V4_DISABLED.load(Ordering::Acquire)
        || X64_V3_GFNI_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V3_DISABLED.load(Ordering::Acquire)
        || X64_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_V3_CRYPTO_DISABLED.load(Ordering::Acquire)
        || X64_GFNI_DISABLED.load(Ordering::Acquire)
        || X64_V2_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64V4xToken::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for Galois Field New Instructions (`gfni`) on x86-64.
///
/// Compose with `X64V3Token.and(X64GfniToken)` or `X64V4Token.and(X64GfniToken)` for 256/512-bit GFNI.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "gfni",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "gfni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "gfni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "gfni",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_GFNI_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_gfni_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_gfni_detect(),
            }
        }
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

impl X64GfniToken {
    /// Construct a x86-64 GFNI proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,gfni")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64GfniToken {
    /// Extract a X64V1Token — guaranteed because x86-64 GFNI implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 GFNI's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64GfniToken {
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
    /// - `X64V3GfniCryptoToken`
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    /// - `X64V4xToken`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "gfni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "gfni",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_GFNI_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_GFNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_GFNI_CRYPTO_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_GFNI_CRYPTO_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V4X_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V4X_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "gfni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "gfni",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_GFNI_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "gfni",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_gfni_detect() -> Option<X64GfniToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("gfni");
    let pre_disabled =
        X64_GFNI_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_GFNI_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        X64_GFNI_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64GfniToken::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for SHA-1 / SHA-256 hardware extensions (`sha`) on x86-64.
///
/// Compose with `X64V3Token` or `X64V4Token` via `.and(...)` (e.g., Zen 1/4/5).
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sha",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sha",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sha",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sha",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_SHA_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_sha_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_sha_detect(),
            }
        }
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

impl X64ShaToken {
    /// Construct a x86-64 SHA proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,sha")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64ShaToken {
    /// Extract a X64V1Token — guaranteed because x86-64 SHA implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 SHA's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64ShaToken {
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
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sha",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sha",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_SHA_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_SHA_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sha",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "sha",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_SHA_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sha",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_sha_detect() -> Option<X64ShaToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("sha");
    let pre_disabled =
        X64_SHA_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_SHA_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        X64_SHA_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64ShaToken::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for Multi-Precision Add-Carry (`adx`: `ADCX`/`ADOX`) on x86-64.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "adx",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "adx",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "adx",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "adx",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_ADX_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_adx_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_adx_detect(),
            }
        }
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

impl X64AdxToken {
    /// Construct a x86-64 ADX proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,adx")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64AdxToken {
    /// Extract a X64V1Token — guaranteed because x86-64 ADX implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 ADX's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64AdxToken {
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
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "adx",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "adx",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_ADX_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_ADX_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "adx",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "adx",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_ADX_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "adx",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_adx_detect() -> Option<X64AdxToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("adx");
    let pre_disabled =
        X64_ADX_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_ADX_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        X64_ADX_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64AdxToken::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for VEX-encoded AVX-VNNI (`avxvnni`: `vpdpbusd`/`vpdpwssd`) on x86-64.
///
/// Compose with `X64V3Token.and(X64AvxVnniToken)` for quantized ML inference without requiring GFNI/ADX/SHA.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnni",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnni",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_AVX_VNNI_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_avx_vnni_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_avx_vnni_detect(),
            }
        }
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

impl X64AvxVnniToken {
    /// Construct a x86-64 AVX-VNNI proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,avxvnni")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64AvxVnniToken {
    /// Extract a X64V1Token — guaranteed because x86-64 AVX-VNNI implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 AVX-VNNI's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64AvxVnniToken {
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
    /// - `X64V3VnniToken`
    /// - `X64V3Vnni2Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnni",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_AVX_VNNI_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_AVX_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnni",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnni",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_AVX_VNNI_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "avxvnni",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_avx_vnni_detect() -> Option<X64AvxVnniToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("avxvnni");
    let pre_disabled =
        X64_AVX_VNNI_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_AVX_VNNI_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        X64_AVX_VNNI_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64AvxVnniToken::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for VEX-encoded AVX-IFMA (`avxifma`: 52-bit integer FMA) on x86-64.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxifma",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxifma",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxifma",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxifma",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_AVX_IFMA_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_avx_ifma_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_avx_ifma_detect(),
            }
        }
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

impl X64AvxIfmaToken {
    /// Construct a x86-64 AVX-IFMA proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,avxifma")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64AvxIfmaToken {
    /// Extract a X64V1Token — guaranteed because x86-64 AVX-IFMA implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 AVX-IFMA's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64AvxIfmaToken {
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
    /// - `X64V3Vnni2Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxifma",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxifma",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_AVX_IFMA_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_AVX_IFMA_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxifma",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxifma",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_AVX_IFMA_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "avxifma",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_avx_ifma_detect() -> Option<X64AvxIfmaToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("avxifma");
    let pre_disabled =
        X64_AVX_IFMA_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_AVX_IFMA_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        X64_AVX_IFMA_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64AvxIfmaToken::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for AVX-VNNI-INT8 (`avxvnniint8`: signed/unsigned int8 dot products) on x86-64.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint8",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint8",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint8",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint8",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_AVX_VNNI_INT8_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_avx_vnni_int8_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_avx_vnni_int8_detect(),
            }
        }
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

impl X64AvxVnniInt8Token {
    /// Construct a x86-64 AVX-VNNI-INT8 proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,avxvnniint8")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64AvxVnniInt8Token {
    /// Extract a X64V1Token — guaranteed because x86-64 AVX-VNNI-INT8 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 AVX-VNNI-INT8's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64AvxVnniInt8Token {
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
    /// - `X64V3Vnni2Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint8",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint8",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_AVX_VNNI_INT8_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_AVX_VNNI_INT8_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint8",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint8",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_AVX_VNNI_INT8_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "avxvnniint8",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_avx_vnni_int8_detect() -> Option<X64AvxVnniInt8Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("avxvnniint8");
    let pre_disabled = X64_AVX_VNNI_INT8_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_AVX_VNNI_INT8_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_AVX_VNNI_INT8_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64AvxVnniInt8Token::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for AVX-VNNI-INT16 (`avxvnniint16`: mixed-sign int16 dot products) on x86-64.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint16",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint16",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_AVX_VNNI_INT16_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_avx_vnni_int16_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_avx_vnni_int16_detect(),
            }
        }
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

impl X64AvxVnniInt16Token {
    /// Construct a x86-64 AVX-VNNI-INT16 proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,avxvnniint16")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64AvxVnniInt16Token {
    /// Extract a X64V1Token — guaranteed because x86-64 AVX-VNNI-INT16 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 AVX-VNNI-INT16's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64AvxVnniInt16Token {
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint16",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_AVX_VNNI_INT16_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_AVX_VNNI_INT16_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint16",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxvnniint16",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_AVX_VNNI_INT16_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "avxvnniint16",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_avx_vnni_int16_detect() -> Option<X64AvxVnniInt16Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("avxvnniint16");
    let pre_disabled = X64_AVX_VNNI_INT16_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_AVX_VNNI_INT16_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_AVX_VNNI_INT16_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64AvxVnniInt16Token::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for AVX-NE-CONVERT (`avxneconvert`: BF16/FP16 conversion without AVX-512) on x86-64.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxneconvert",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxneconvert",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxneconvert",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxneconvert",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_AVX_NE_CONVERT_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_avx_ne_convert_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_avx_ne_convert_detect(),
            }
        }
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

impl X64AvxNeConvertToken {
    /// Construct a x86-64 AVX-NE-CONVERT proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,avxneconvert")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64AvxNeConvertToken {
    /// Extract a X64V1Token — guaranteed because x86-64 AVX-NE-CONVERT implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 AVX-NE-CONVERT's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64AvxNeConvertToken {
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
    /// - `X64V3Vnni2Token`
    #[allow(clippy::needless_return)]
    pub fn dangerously_disable_token_process_wide(
        disabled: bool,
    ) -> Result<(), crate::tokens::CompileTimeGuaranteedError> {
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxneconvert",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxneconvert",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_AVX_NE_CONVERT_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_AVX_NE_CONVERT_CACHE.store(v, Ordering::Release);
            if disabled || v == 0 {
                X64_V3_VNNI2_DISABLED.store(disabled, Ordering::Release);
            }
            X64_V3_VNNI2_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxneconvert",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avxneconvert",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_AVX_NE_CONVERT_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "avxneconvert",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_avx_ne_convert_detect() -> Option<X64AvxNeConvertToken> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("avxneconvert");
    let pre_disabled = X64_AVX_NE_CONVERT_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_AVX_NE_CONVERT_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled = X64_AVX_NE_CONVERT_DISABLED.load(Ordering::Acquire)
        || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64AvxNeConvertToken::from_context() })
    } else {
        None
    }
}

/// Orthogonal proof for SHA-512 vector instructions (`sha512` + `avx`) on x86-64.
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
        #[cfg(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avx",
            target_feature = "sha512",
            not(feature = "testable_dispatch")
        ))]
        {
            Some(true)
        }
        #[cfg(not(all(
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avx",
            target_feature = "sha512",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avx",
            target_feature = "sha512",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avx",
            target_feature = "sha512",
            not(feature = "testable_dispatch")
        )))]
        {
            match X64_SHA512_CACHE.load(Ordering::Acquire) {
                // SAFETY: 2 is only ever stored by x64_sha512_detect()
                // after a positive runtime check of every feature.
                2 => Some(unsafe { Self::from_context() }),
                1 => None,
                _ => x64_sha512_detect(),
            }
        }
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

impl X64Sha512Token {
    /// Construct a x86-64 SHA512 proof from the caller's statically proven
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
    #[target_feature(enable = "sse,sse2,avx,sha512")]
    pub fn from_context() -> Self {
        Self { _private: () }
    }
}

impl X64Sha512Token {
    /// Extract a X64V1Token — guaranteed because x86-64 SHA512 implies x86-64-v1.
    ///
    /// Zero-cost: compiles away entirely.
    #[inline(always)]
    pub fn v1(self) -> X64V1Token {
        // SAFETY: holding `self` proves this environment satisfies
        // x86-64 SHA512's full contract, a superset of x86-64-v1's.
        unsafe { X64V1Token::from_context() }
    }
}

impl X64Sha512Token {
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avx",
            target_feature = "sha512",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avx",
            target_feature = "sha512",
            not(feature = "testable_dispatch")
        )))]
        {
            X64_SHA512_DISABLED.store(disabled, Ordering::Release);
            let v = if disabled || (X64_V1_DISABLED.load(Ordering::Acquire)) {
                1
            } else {
                0
            };
            X64_SHA512_CACHE.store(v, Ordering::Release);
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avx",
            target_feature = "sha512",
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
            target_feature = "sse",
            target_feature = "sse2",
            target_feature = "avx",
            target_feature = "sha512",
            not(feature = "testable_dispatch")
        )))]
        {
            Ok(X64_SHA512_DISABLED.load(Ordering::Acquire))
        }
    }
}
#[cfg(not(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "avx",
    target_feature = "sha512",
    not(feature = "testable_dispatch")
)))]
#[cold]
#[inline(never)]
fn x64_sha512_detect() -> Option<X64Sha512Token> {
    let available = crate::is_x86_feature_available!("sse")
        && crate::is_x86_feature_available!("sse2")
        && crate::is_x86_feature_available!("avx")
        && crate::is_x86_feature_available!("sha512");
    let pre_disabled =
        X64_SHA512_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    // Publish only while the cache is still undetected (0): a
    // concurrent disable/enable writes the cache itself, and an
    // unconditional store here could overwrite it — resurrecting a
    // token that was disabled mid-detect (lost update: summon()
    // returning Some for a disabled token).
    let _ = X64_SHA512_CACHE.compare_exchange(
        0,
        if available && !pre_disabled { 2 } else { 1 },
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    // disable() stores the flag before the cache, so loading the
    // flag after the CAS observes every disable that could have
    // raced this probe.
    let disabled =
        X64_SHA512_DISABLED.load(Ordering::Acquire) || X64_V1_DISABLED.load(Ordering::Acquire);
    if available && !disabled {
        // SAFETY: `available` — runtime detection just confirmed every
        // feature this token asserts is present on this CPU.
        Some(unsafe { X64Sha512Token::from_context() })
    } else {
        None
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
