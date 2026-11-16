//! Generated from token-registry.toml — DO NOT EDIT.
//!
//! Sealed capability marker traits with blanket impls for tokens and `And` conjunctions,
//! plus [`IntoConcreteToken`] identity dispatch.

use crate::tokens::{And, SameArch, ScalarToken, SimdToken};

/// Marker trait for x86-64-v1 baseline (`sse`, `sse2`).
pub trait HasX64V1: SimdToken {}

impl<T> HasX64V1 for T where T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes> {}

/// Marker trait for x86-64-v2 level (Nehalem 2008+).
///
/// v2 includes: SSE3, SSSE3, SSE4.1, SSE4.2, POPCNT, CMPXCHG16B, LAHF-SAHF.
pub trait HasX64V2: HasX64V1 {}

impl<T> HasX64V2 for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, V2 = super::caps::Yes>
{
}

/// Marker trait for x86-64-v2 + PCLMULQDQ + AES-NI.
pub trait HasX64Crypto: HasX64V2 {}

impl<T> HasX64Crypto for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            Crypto128 = super::caps::Yes,
        >
{
}

/// Marker trait for x86-64-v3 level (Haswell 2013+, Zen 1 2017+).
pub trait HasX64V3: HasX64V2 {}

impl<T> HasX64V3 for T where
    T: SimdToken
        + super::caps::X64Caps<V1 = super::caps::Yes, V2 = super::caps::Yes, V3 = super::caps::Yes>
{
}

/// Marker trait for x86-64-v3 + VPCLMULQDQ + VAES.
pub trait HasX64V3Crypto: HasX64V3 + HasX64Crypto {}

impl<T> HasX64V3Crypto for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            Crypto128 = super::caps::Yes,
            V3 = super::caps::Yes,
            VpclmulVaes = super::caps::Yes,
        >
{
}

/// Marker trait for Galois Field New Instructions (`gfni`) on x86-64.
pub trait HasGfni: HasX64V1 {}

impl<T> HasGfni for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, Gfni = super::caps::Yes>
{
}

/// Marker trait for x86-64-v3 + VPCLMULQDQ + VAES + GFNI.
pub trait HasX64V3GfniCrypto: HasX64V3Crypto + HasGfni {}

impl<T> HasX64V3GfniCrypto for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            Crypto128 = super::caps::Yes,
            V3 = super::caps::Yes,
            VpclmulVaes = super::caps::Yes,
            Gfni = super::caps::Yes,
        >
{
}

/// Marker trait for SHA-1 / SHA-256 hardware extensions (`sha`) on x86-64.
pub trait HasSha: HasX64V1 {}

impl<T> HasSha for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, Sha = super::caps::Yes>
{
}

/// Marker trait for Multi-Precision Add-Carry (`adx`) on x86-64.
pub trait HasAdx: HasX64V1 {}

impl<T> HasAdx for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, Adx = super::caps::Yes>
{
}

/// Marker trait for VEX-encoded AVX-VNNI (`avxvnni`) on x86-64.
pub trait HasAvxVnni: HasX64V1 {}

impl<T> HasAvxVnni for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, AvxVnni = super::caps::Yes>
{
}

/// Marker trait for AVX-IFMA (`avxifma`) on x86-64.
pub trait HasAvxIfma: HasX64V1 {}

impl<T> HasAvxIfma for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, AvxIfma = super::caps::Yes>
{
}

/// Marker trait for AVX-VNNI-INT8 (`avxvnniint8`) on x86-64.
pub trait HasAvxVnniInt8: HasX64V1 {}

impl<T> HasAvxVnniInt8 for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, AvxVnniInt8 = super::caps::Yes>
{
}

/// Marker trait for AVX-VNNI-INT16 (`avxvnniint16`) on x86-64.
pub trait HasAvxVnniInt16: HasX64V1 {}

impl<T> HasAvxVnniInt16 for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, AvxVnniInt16 = super::caps::Yes>
{
}

/// Marker trait for AVX-NE-CONVERT (`avxneconvert`) on x86-64.
pub trait HasAvxNeConvert: HasX64V1 {}

impl<T> HasAvxNeConvert for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, AvxNeConvert = super::caps::Yes>
{
}

/// Marker trait for SHA-512 vector extensions (`sha512` + `avx`) on x86-64.
pub trait HasSha512: HasX64V1 {}

impl<T> HasSha512 for T where
    T: SimdToken + super::caps::X64Caps<V1 = super::caps::Yes, Sha512 = super::caps::Yes>
{
}

/// Marker trait for x86-64-v3 GFNI Crypto + AVX-VNNI + ADX + SHA.
pub trait HasX64V3Vnni: HasX64V3GfniCrypto + HasAvxVnni + HasAdx + HasSha {}

impl<T> HasX64V3Vnni for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            Crypto128 = super::caps::Yes,
            V3 = super::caps::Yes,
            VpclmulVaes = super::caps::Yes,
            Gfni = super::caps::Yes,
            AvxVnni = super::caps::Yes,
            Adx = super::caps::Yes,
            Sha = super::caps::Yes,
        >
{
}

/// Marker trait for x86-64-v3 VNNI + AVX-VNNI-INT8 + AVX-NE-CONVERT + AVX-IFMA.
pub trait HasX64V3Vnni2: HasX64V3Vnni + HasAvxVnniInt8 + HasAvxNeConvert + HasAvxIfma {}

impl<T> HasX64V3Vnni2 for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            Crypto128 = super::caps::Yes,
            V3 = super::caps::Yes,
            VpclmulVaes = super::caps::Yes,
            Gfni = super::caps::Yes,
            AvxVnni = super::caps::Yes,
            Adx = super::caps::Yes,
            Sha = super::caps::Yes,
            AvxVnniInt8 = super::caps::Yes,
            AvxNeConvert = super::caps::Yes,
            AvxIfma = super::caps::Yes,
        >
{
}

/// Marker trait for x86-64-v4 level (Skylake-X 2017+, Zen 4 2022+).
pub trait HasX64V4: HasX64V3 {}

impl<T> HasX64V4 for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            V3 = super::caps::Yes,
            V4 = super::caps::Yes,
        >
{
}

/// Marker trait for AVX-512 FP16 (`avx512fp16` on top of x86-64-v4).
pub trait HasAvx512Fp16: HasX64V4 {}

impl<T> HasAvx512Fp16 for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            V3 = super::caps::Yes,
            V4 = super::caps::Yes,
            Avx512Fp16 = super::caps::Yes,
        >
{
}

/// Marker trait for AVX-512 BF16 (`avx512bf16` on top of x86-64-v4).
pub trait HasAvx512Bf16: HasX64V4 {}

impl<T> HasAvx512Bf16 for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            V3 = super::caps::Yes,
            V4 = super::caps::Yes,
            Avx512Bf16 = super::caps::Yes,
        >
{
}

/// Marker trait for x86-64-v4x level (Ice Lake / Zen 4+).
pub trait HasX64V4x: HasX64V4 + HasX64V3GfniCrypto {}

impl<T> HasX64V4x for T where
    T: SimdToken
        + super::caps::X64Caps<
            V1 = super::caps::Yes,
            V2 = super::caps::Yes,
            Crypto128 = super::caps::Yes,
            V3 = super::caps::Yes,
            VpclmulVaes = super::caps::Yes,
            Gfni = super::caps::Yes,
            V4 = super::caps::Yes,
            V4x = super::caps::Yes,
        >
{
}

/// Marker trait for NEON on AArch64.
pub trait HasNeon: SimdToken {}

impl<T> HasNeon for T where T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes> {}

/// Marker trait for NEON + AES on AArch64.
pub trait HasNeonAes: HasNeon {}

impl<T> HasNeonAes for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, Aes = super::caps::Yes>
{
}

/// Marker trait for NEON + SHA2 on AArch64.
pub trait HasNeonSha2: HasNeon {}

impl<T> HasNeonSha2 for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, Sha2 = super::caps::Yes>
{
}

/// Marker trait for NEON + SHA3 on AArch64.
pub trait HasNeonSha3: HasNeon {}

impl<T> HasNeonSha3 for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, Sha3 = super::caps::Yes>
{
}

/// Marker trait for NEON + CRC on AArch64.
pub trait HasNeonCrc: HasNeon {}

impl<T> HasNeonCrc for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, Crc = super::caps::Yes>
{
}

/// Marker trait for NEON + RDM on AArch64.
pub trait HasNeonRdm: HasNeon {}

impl<T> HasNeonRdm for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, Rdm = super::caps::Yes>
{
}

/// Marker trait for NEON + DotProd on AArch64.
pub trait HasNeonDotProd: HasNeon {}

impl<T> HasNeonDotProd for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, DotProd = super::caps::Yes>
{
}

/// Marker trait for NEON + FP16 on AArch64.
pub trait HasNeonFp16: HasNeon {}

impl<T> HasNeonFp16 for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, Fp16 = super::caps::Yes>
{
}

/// Marker trait for NEON + FP16 + FHM on AArch64.
pub trait HasNeonFhm: HasNeonFp16 {}

impl<T> HasNeonFhm for T where
    T: SimdToken
        + super::caps::ArmCaps<
            Neon = super::caps::Yes,
            Fp16 = super::caps::Yes,
            Fhm = super::caps::Yes,
        >
{
}

/// Marker trait for NEON + FCMA on AArch64.
pub trait HasNeonFcma: HasNeon {}

impl<T> HasNeonFcma for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, Fcma = super::caps::Yes>
{
}

/// Marker trait for NEON + I8MM on AArch64.
pub trait HasNeonI8mm: HasNeon {}

impl<T> HasNeonI8mm for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, I8mm = super::caps::Yes>
{
}

/// Marker trait for NEON + BF16 on AArch64.
pub trait HasNeonBf16: HasNeon {}

impl<T> HasNeonBf16 for T where
    T: SimdToken + super::caps::ArmCaps<Neon = super::caps::Yes, Bf16 = super::caps::Yes>
{
}

/// Marker trait for Arm64-v2 level (NEON, CRC, RDM, DotProd, FP16, AES, SHA2).
pub trait HasArm64V2:
    HasNeon + HasNeonAes + HasNeonSha2 + HasNeonCrc + HasNeonRdm + HasNeonDotProd + HasNeonFp16
{
}

impl<T> HasArm64V2 for T where
    T: SimdToken
        + super::caps::ArmCaps<
            Neon = super::caps::Yes,
            Crc = super::caps::Yes,
            Rdm = super::caps::Yes,
            DotProd = super::caps::Yes,
            Fp16 = super::caps::Yes,
            Aes = super::caps::Yes,
            Sha2 = super::caps::Yes,
        >
{
}

/// Marker trait for Arm64-v3 level (adds FHM, FCMA, SHA3, I8MM, BF16).
pub trait HasArm64V3:
    HasArm64V2 + HasNeonFhm + HasNeonFcma + HasNeonSha3 + HasNeonI8mm + HasNeonBf16
{
}

impl<T> HasArm64V3 for T where
    T: SimdToken
        + super::caps::ArmCaps<
            Neon = super::caps::Yes,
            Crc = super::caps::Yes,
            Rdm = super::caps::Yes,
            DotProd = super::caps::Yes,
            Fp16 = super::caps::Yes,
            Aes = super::caps::Yes,
            Sha2 = super::caps::Yes,
            Fhm = super::caps::Yes,
            Fcma = super::caps::Yes,
            Sha3 = super::caps::Yes,
            I8mm = super::caps::Yes,
            Bf16 = super::caps::Yes,
        >
{
}

/// Marker trait for WASM SIMD128.
pub trait HasWasm128: SimdToken {}

impl<T> HasWasm128 for T where T: SimdToken + super::caps::WasmCaps<Simd128 = super::caps::Yes> {}

/// Marker trait for WASM Relaxed SIMD.
pub trait HasWasm128Relaxed: HasWasm128 {}

impl<T> HasWasm128Relaxed for T where
    T: SimdToken + super::caps::WasmCaps<Simd128 = super::caps::Yes, Relaxed = super::caps::Yes>
{
}

/// Marker trait for WASM Relaxed SIMD with runtime-probed single-rounding FMA fusion.
pub trait HasWasm128RelaxedFma: HasWasm128Relaxed {}

impl<T> HasWasm128RelaxedFma for T where
    T: SimdToken
        + super::caps::WasmCaps<
            Simd128 = super::caps::Yes,
            Relaxed = super::caps::Yes,
            RelaxedFma = super::caps::Yes,
        >
{
}

/// Trait for compile-time dispatch via monomorphization.
///
/// Each concrete leaf token implements this trait, returning `Some(self)` for
/// its own type and `None` for others. Composite [`And<Base, Extra>`] tokens
/// return `None` for all identity casts (they represent a conjunction of
/// proofs rather than a single leaf type).
pub trait IntoConcreteToken: SimdToken + Sized {
    /// Returns `Some(self)` if this is literally [`X64V1Token`](super::X64V1Token), `None` otherwise.
    #[inline(always)]
    fn as_x64v1(self) -> Option<super::X64V1Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64V2Token`](super::X64V2Token), `None` otherwise.
    #[inline(always)]
    fn as_x64v2(self) -> Option<super::X64V2Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64CryptoToken`](super::X64CryptoToken), `None` otherwise.
    #[inline(always)]
    fn as_x64_crypto(self) -> Option<super::X64CryptoToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64V3Token`](super::X64V3Token), `None` otherwise.
    #[inline(always)]
    fn as_x64v3(self) -> Option<super::X64V3Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64V3CryptoToken`](super::X64V3CryptoToken), `None` otherwise.
    #[inline(always)]
    fn as_x64v3_crypto(self) -> Option<super::X64V3CryptoToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64V3GfniCryptoToken`](super::X64V3GfniCryptoToken), `None` otherwise.
    #[inline(always)]
    fn as_x64v3_gfni_crypto(self) -> Option<super::X64V3GfniCryptoToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64V3VnniToken`](super::X64V3VnniToken), `None` otherwise.
    #[inline(always)]
    fn as_x64v3_vnni(self) -> Option<super::X64V3VnniToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64V3Vnni2Token`](super::X64V3Vnni2Token), `None` otherwise.
    #[inline(always)]
    fn as_x64v3_vnni2(self) -> Option<super::X64V3Vnni2Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64V4Token`](super::X64V4Token), `None` otherwise.
    #[inline(always)]
    fn as_x64v4(self) -> Option<super::X64V4Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`Avx512Fp16Token`](super::Avx512Fp16Token), `None` otherwise.
    #[inline(always)]
    fn as_avx512_fp16(self) -> Option<super::Avx512Fp16Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64Avx512Bf16Token`](super::X64Avx512Bf16Token), `None` otherwise.
    #[inline(always)]
    fn as_x64_avx512_bf16(self) -> Option<super::X64Avx512Bf16Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64V4xToken`](super::X64V4xToken), `None` otherwise.
    #[inline(always)]
    fn as_x64v4x(self) -> Option<super::X64V4xToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64GfniToken`](super::X64GfniToken), `None` otherwise.
    #[inline(always)]
    fn as_x64_gfni(self) -> Option<super::X64GfniToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64ShaToken`](super::X64ShaToken), `None` otherwise.
    #[inline(always)]
    fn as_x64_sha(self) -> Option<super::X64ShaToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64AdxToken`](super::X64AdxToken), `None` otherwise.
    #[inline(always)]
    fn as_x64_adx(self) -> Option<super::X64AdxToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64AvxVnniToken`](super::X64AvxVnniToken), `None` otherwise.
    #[inline(always)]
    fn as_x64_avx_vnni(self) -> Option<super::X64AvxVnniToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64AvxIfmaToken`](super::X64AvxIfmaToken), `None` otherwise.
    #[inline(always)]
    fn as_x64_avx_ifma(self) -> Option<super::X64AvxIfmaToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64AvxVnniInt8Token`](super::X64AvxVnniInt8Token), `None` otherwise.
    #[inline(always)]
    fn as_x64_avx_vnni_int8(self) -> Option<super::X64AvxVnniInt8Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64AvxVnniInt16Token`](super::X64AvxVnniInt16Token), `None` otherwise.
    #[inline(always)]
    fn as_x64_avx_vnni_int16(self) -> Option<super::X64AvxVnniInt16Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64AvxNeConvertToken`](super::X64AvxNeConvertToken), `None` otherwise.
    #[inline(always)]
    fn as_x64_avx_ne_convert(self) -> Option<super::X64AvxNeConvertToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`X64Sha512Token`](super::X64Sha512Token), `None` otherwise.
    #[inline(always)]
    fn as_x64_sha512(self) -> Option<super::X64Sha512Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonToken`](super::NeonToken), `None` otherwise.
    #[inline(always)]
    fn as_neon(self) -> Option<super::NeonToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonAesToken`](super::NeonAesToken), `None` otherwise.
    #[inline(always)]
    fn as_neon_aes(self) -> Option<super::NeonAesToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonSha2Token`](super::NeonSha2Token), `None` otherwise.
    #[inline(always)]
    fn as_neon_sha2(self) -> Option<super::NeonSha2Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonSha3Token`](super::NeonSha3Token), `None` otherwise.
    #[inline(always)]
    fn as_neon_sha3(self) -> Option<super::NeonSha3Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonCrcToken`](super::NeonCrcToken), `None` otherwise.
    #[inline(always)]
    fn as_neon_crc(self) -> Option<super::NeonCrcToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonRdmToken`](super::NeonRdmToken), `None` otherwise.
    #[inline(always)]
    fn as_neon_rdm(self) -> Option<super::NeonRdmToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonDotProdToken`](super::NeonDotProdToken), `None` otherwise.
    #[inline(always)]
    fn as_neon_dotprod(self) -> Option<super::NeonDotProdToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonFp16Token`](super::NeonFp16Token), `None` otherwise.
    #[inline(always)]
    fn as_neon_fp16(self) -> Option<super::NeonFp16Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonFcmaToken`](super::NeonFcmaToken), `None` otherwise.
    #[inline(always)]
    fn as_neon_fcma(self) -> Option<super::NeonFcmaToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonI8mmToken`](super::NeonI8mmToken), `None` otherwise.
    #[inline(always)]
    fn as_neon_i8mm(self) -> Option<super::NeonI8mmToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonBf16Token`](super::NeonBf16Token), `None` otherwise.
    #[inline(always)]
    fn as_neon_bf16(self) -> Option<super::NeonBf16Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`NeonFhmToken`](super::NeonFhmToken), `None` otherwise.
    #[inline(always)]
    fn as_neon_fhm(self) -> Option<super::NeonFhmToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`Arm64V2Token`](super::Arm64V2Token), `None` otherwise.
    #[inline(always)]
    fn as_arm_v2(self) -> Option<super::Arm64V2Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`Arm64V3Token`](super::Arm64V3Token), `None` otherwise.
    #[inline(always)]
    fn as_arm_v3(self) -> Option<super::Arm64V3Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`Wasm128Token`](super::Wasm128Token), `None` otherwise.
    #[inline(always)]
    fn as_wasm128(self) -> Option<super::Wasm128Token> {
        None
    }

    /// Returns `Some(self)` if this is literally [`Wasm128RelaxedToken`](super::Wasm128RelaxedToken), `None` otherwise.
    #[inline(always)]
    fn as_wasm128_relaxed(self) -> Option<super::Wasm128RelaxedToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`Wasm128RelaxedFmaToken`](super::Wasm128RelaxedFmaToken), `None` otherwise.
    #[inline(always)]
    fn as_wasm128_relaxed_fma(self) -> Option<super::Wasm128RelaxedFmaToken> {
        None
    }

    /// Returns `Some(self)` if this is literally [`ScalarToken`], `None` otherwise.
    #[inline(always)]
    fn as_scalar(self) -> Option<ScalarToken> {
        None
    }
}

impl IntoConcreteToken for ScalarToken {
    #[inline(always)]
    fn as_scalar(self) -> Option<ScalarToken> {
        Some(self)
    }
}

impl<Base, Extra> IntoConcreteToken for And<Base, Extra>
where
    Base: SameArch<Extra> + IntoConcreteToken,
    Extra: SimdToken,
{
    #[inline(always)]
    fn as_scalar(self) -> Option<ScalarToken> {
        self.base().as_scalar()
    }
}
