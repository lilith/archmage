//! Generated from token-registry.toml — DO NOT EDIT.
//!
//! Sealed architecture markers, type-level capability flags, and `And` proof tags.

use crate::tokens::SimdToken;
use core::marker::PhantomData;

mod cap_sealed {
    pub trait Sealed {}
}

/// Sealed marker trait identifying a CPU architecture family.
pub trait ArchMarker: cap_sealed::Sealed + Copy + Clone + Send + Sync + 'static {}

/// Architecture marker for x86 / x86-64 tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64Arch;
impl cap_sealed::Sealed for X64Arch {}
impl ArchMarker for X64Arch {}

/// Architecture marker for AArch64 NEON tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArmArch;
impl cap_sealed::Sealed for ArmArch {}
impl ArchMarker for ArmArch {}

/// Architecture marker for WebAssembly SIMD tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WasmArch;
impl cap_sealed::Sealed for WasmArch {}
impl ArchMarker for WasmArch {}

/// Architecture marker for [`ScalarToken`](crate::tokens::ScalarToken).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScalarArch;
impl cap_sealed::Sealed for ScalarArch {}
impl ArchMarker for ScalarArch {}

/// Sealed type-level boolean flag for a single capability slot.
pub trait Flag:
    cap_sealed::Sealed
    + Copy
    + Clone
    + core::fmt::Debug
    + PartialEq
    + Eq
    + core::hash::Hash
    + Send
    + Sync
    + 'static
{
    const ENABLED: bool;
    type Or<R: Flag>: Flag;
    type And<R: Flag>: Flag;
}

/// Capability absent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct No;
impl cap_sealed::Sealed for No {}
impl Flag for No {
    const ENABLED: bool = false;
    type Or<R: Flag> = R;
    type And<R: Flag> = No;
}

/// Capability present.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Yes;
impl cap_sealed::Sealed for Yes {}
impl Flag for Yes {
    const ENABLED: bool = true;
    type Or<R: Flag> = Yes;
    type And<R: Flag> = R;
}

/// Capability set for [`ScalarToken`](crate::tokens::ScalarToken).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScalarCaps;

/// Sealed type-level union of two capability sets from the same architecture.
pub trait CapSetOr<Rhs>: cap_sealed::Sealed {
    type Output: Copy + Clone + Send + Sync + 'static;
}

impl cap_sealed::Sealed for ScalarCaps {}
impl CapSetOr<ScalarCaps> for ScalarCaps {
    type Output = ScalarCaps;
}

#[allow(clippy::type_complexity)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct X64CapSet<
    V1: Flag,
    V2: Flag,
    Crypto128: Flag,
    V3: Flag,
    VpclmulVaes: Flag,
    Gfni: Flag,
    Sha: Flag,
    Adx: Flag,
    AvxVnni: Flag,
    AvxIfma: Flag,
    AvxVnniInt8: Flag,
    AvxVnniInt16: Flag,
    AvxNeConvert: Flag,
    Sha512: Flag,
    V4: Flag,
    V4x: Flag,
    Avx512Fp16: Flag,
    Avx512Bf16: Flag,
>(
    PhantomData<
        fn() -> (
            V1,
            V2,
            Crypto128,
            V3,
            VpclmulVaes,
            Gfni,
            Sha,
            Adx,
            AvxVnni,
            AvxIfma,
            AvxVnniInt8,
            AvxVnniInt16,
            AvxNeConvert,
            Sha512,
            V4,
            V4x,
            Avx512Fp16,
            Avx512Bf16,
        ),
    >,
);

#[allow(clippy::type_complexity)]
impl<
    V1: Flag,
    V2: Flag,
    Crypto128: Flag,
    V3: Flag,
    VpclmulVaes: Flag,
    Gfni: Flag,
    Sha: Flag,
    Adx: Flag,
    AvxVnni: Flag,
    AvxIfma: Flag,
    AvxVnniInt8: Flag,
    AvxVnniInt16: Flag,
    AvxNeConvert: Flag,
    Sha512: Flag,
    V4: Flag,
    V4x: Flag,
    Avx512Fp16: Flag,
    Avx512Bf16: Flag,
> cap_sealed::Sealed
    for X64CapSet<
        V1,
        V2,
        Crypto128,
        V3,
        VpclmulVaes,
        Gfni,
        Sha,
        Adx,
        AvxVnni,
        AvxIfma,
        AvxVnniInt8,
        AvxVnniInt16,
        AvxNeConvert,
        Sha512,
        V4,
        V4x,
        Avx512Fp16,
        Avx512Bf16,
    >
{
}

/// Sealed capability projection trait for `x86` tokens and conjunctions.
pub trait X64Caps: crate::tokens::Sealed {
    type V1: Flag;
    type V2: Flag;
    type Crypto128: Flag;
    type V3: Flag;
    type VpclmulVaes: Flag;
    type Gfni: Flag;
    type Sha: Flag;
    type Adx: Flag;
    type AvxVnni: Flag;
    type AvxIfma: Flag;
    type AvxVnniInt8: Flag;
    type AvxVnniInt16: Flag;
    type AvxNeConvert: Flag;
    type Sha512: Flag;
    type V4: Flag;
    type V4x: Flag;
    type Avx512Fp16: Flag;
    type Avx512Bf16: Flag;
}

impl<Base, Extra> X64Caps for crate::tokens::And<Base, Extra>
where
    Base: crate::tokens::SameArch<Extra> + X64Caps,
    Extra: SimdToken + X64Caps,
{
    type V1 = <Base::V1 as Flag>::Or<Extra::V1>;
    type V2 = <Base::V2 as Flag>::Or<Extra::V2>;
    type Crypto128 = <Base::Crypto128 as Flag>::Or<Extra::Crypto128>;
    type V3 = <Base::V3 as Flag>::Or<Extra::V3>;
    type VpclmulVaes = <Base::VpclmulVaes as Flag>::Or<Extra::VpclmulVaes>;
    type Gfni = <Base::Gfni as Flag>::Or<Extra::Gfni>;
    type Sha = <Base::Sha as Flag>::Or<Extra::Sha>;
    type Adx = <Base::Adx as Flag>::Or<Extra::Adx>;
    type AvxVnni = <Base::AvxVnni as Flag>::Or<Extra::AvxVnni>;
    type AvxIfma = <Base::AvxIfma as Flag>::Or<Extra::AvxIfma>;
    type AvxVnniInt8 = <Base::AvxVnniInt8 as Flag>::Or<Extra::AvxVnniInt8>;
    type AvxVnniInt16 = <Base::AvxVnniInt16 as Flag>::Or<Extra::AvxVnniInt16>;
    type AvxNeConvert = <Base::AvxNeConvert as Flag>::Or<Extra::AvxNeConvert>;
    type Sha512 = <Base::Sha512 as Flag>::Or<Extra::Sha512>;
    type V4 = <Base::V4 as Flag>::Or<Extra::V4>;
    type V4x = <Base::V4x as Flag>::Or<Extra::V4x>;
    type Avx512Fp16 = <Base::Avx512Fp16 as Flag>::Or<Extra::Avx512Fp16>;
    type Avx512Bf16 = <Base::Avx512Bf16 as Flag>::Or<Extra::Avx512Bf16>;
}

#[allow(clippy::type_complexity)]
impl<
    L0: Flag,
    L1: Flag,
    L2: Flag,
    L3: Flag,
    L4: Flag,
    L5: Flag,
    L6: Flag,
    L7: Flag,
    L8: Flag,
    L9: Flag,
    L10: Flag,
    L11: Flag,
    L12: Flag,
    L13: Flag,
    L14: Flag,
    L15: Flag,
    L16: Flag,
    L17: Flag,
    R0: Flag,
    R1: Flag,
    R2: Flag,
    R3: Flag,
    R4: Flag,
    R5: Flag,
    R6: Flag,
    R7: Flag,
    R8: Flag,
    R9: Flag,
    R10: Flag,
    R11: Flag,
    R12: Flag,
    R13: Flag,
    R14: Flag,
    R15: Flag,
    R16: Flag,
    R17: Flag,
>
    CapSetOr<
        X64CapSet<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15, R16, R17>,
    >
    for X64CapSet<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15, L16, L17>
{
    type Output = X64CapSet<
        <L0 as Flag>::Or<R0>,
        <L1 as Flag>::Or<R1>,
        <L2 as Flag>::Or<R2>,
        <L3 as Flag>::Or<R3>,
        <L4 as Flag>::Or<R4>,
        <L5 as Flag>::Or<R5>,
        <L6 as Flag>::Or<R6>,
        <L7 as Flag>::Or<R7>,
        <L8 as Flag>::Or<R8>,
        <L9 as Flag>::Or<R9>,
        <L10 as Flag>::Or<R10>,
        <L11 as Flag>::Or<R11>,
        <L12 as Flag>::Or<R12>,
        <L13 as Flag>::Or<R13>,
        <L14 as Flag>::Or<R14>,
        <L15 as Flag>::Or<R15>,
        <L16 as Flag>::Or<R16>,
        <L17 as Flag>::Or<R17>,
    >;
}

#[allow(clippy::type_complexity)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArmCapSet<
    Neon: Flag,
    Aes: Flag,
    Sha2: Flag,
    Sha3: Flag,
    Crc: Flag,
    Rdm: Flag,
    DotProd: Flag,
    Fp16: Flag,
    Fhm: Flag,
    Fcma: Flag,
    I8mm: Flag,
    Bf16: Flag,
>(
    PhantomData<
        fn() -> (
            Neon,
            Aes,
            Sha2,
            Sha3,
            Crc,
            Rdm,
            DotProd,
            Fp16,
            Fhm,
            Fcma,
            I8mm,
            Bf16,
        ),
    >,
);

#[allow(clippy::type_complexity)]
impl<
    Neon: Flag,
    Aes: Flag,
    Sha2: Flag,
    Sha3: Flag,
    Crc: Flag,
    Rdm: Flag,
    DotProd: Flag,
    Fp16: Flag,
    Fhm: Flag,
    Fcma: Flag,
    I8mm: Flag,
    Bf16: Flag,
> cap_sealed::Sealed
    for ArmCapSet<Neon, Aes, Sha2, Sha3, Crc, Rdm, DotProd, Fp16, Fhm, Fcma, I8mm, Bf16>
{
}

/// Sealed capability projection trait for `aarch64` tokens and conjunctions.
pub trait ArmCaps: crate::tokens::Sealed {
    type Neon: Flag;
    type Aes: Flag;
    type Sha2: Flag;
    type Sha3: Flag;
    type Crc: Flag;
    type Rdm: Flag;
    type DotProd: Flag;
    type Fp16: Flag;
    type Fhm: Flag;
    type Fcma: Flag;
    type I8mm: Flag;
    type Bf16: Flag;
}

impl<Base, Extra> ArmCaps for crate::tokens::And<Base, Extra>
where
    Base: crate::tokens::SameArch<Extra> + ArmCaps,
    Extra: SimdToken + ArmCaps,
{
    type Neon = <Base::Neon as Flag>::Or<Extra::Neon>;
    type Aes = <Base::Aes as Flag>::Or<Extra::Aes>;
    type Sha2 = <Base::Sha2 as Flag>::Or<Extra::Sha2>;
    type Sha3 = <Base::Sha3 as Flag>::Or<Extra::Sha3>;
    type Crc = <Base::Crc as Flag>::Or<Extra::Crc>;
    type Rdm = <Base::Rdm as Flag>::Or<Extra::Rdm>;
    type DotProd = <Base::DotProd as Flag>::Or<Extra::DotProd>;
    type Fp16 = <Base::Fp16 as Flag>::Or<Extra::Fp16>;
    type Fhm = <Base::Fhm as Flag>::Or<Extra::Fhm>;
    type Fcma = <Base::Fcma as Flag>::Or<Extra::Fcma>;
    type I8mm = <Base::I8mm as Flag>::Or<Extra::I8mm>;
    type Bf16 = <Base::Bf16 as Flag>::Or<Extra::Bf16>;
}

#[allow(clippy::type_complexity)]
impl<
    L0: Flag,
    L1: Flag,
    L2: Flag,
    L3: Flag,
    L4: Flag,
    L5: Flag,
    L6: Flag,
    L7: Flag,
    L8: Flag,
    L9: Flag,
    L10: Flag,
    L11: Flag,
    R0: Flag,
    R1: Flag,
    R2: Flag,
    R3: Flag,
    R4: Flag,
    R5: Flag,
    R6: Flag,
    R7: Flag,
    R8: Flag,
    R9: Flag,
    R10: Flag,
    R11: Flag,
> CapSetOr<ArmCapSet<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11>>
    for ArmCapSet<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11>
{
    type Output = ArmCapSet<
        <L0 as Flag>::Or<R0>,
        <L1 as Flag>::Or<R1>,
        <L2 as Flag>::Or<R2>,
        <L3 as Flag>::Or<R3>,
        <L4 as Flag>::Or<R4>,
        <L5 as Flag>::Or<R5>,
        <L6 as Flag>::Or<R6>,
        <L7 as Flag>::Or<R7>,
        <L8 as Flag>::Or<R8>,
        <L9 as Flag>::Or<R9>,
        <L10 as Flag>::Or<R10>,
        <L11 as Flag>::Or<R11>,
    >;
}

#[allow(clippy::type_complexity)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WasmCapSet<Simd128: Flag, Relaxed: Flag, RelaxedFma: Flag>(
    PhantomData<fn() -> (Simd128, Relaxed, RelaxedFma)>,
);

#[allow(clippy::type_complexity)]
impl<Simd128: Flag, Relaxed: Flag, RelaxedFma: Flag> cap_sealed::Sealed
    for WasmCapSet<Simd128, Relaxed, RelaxedFma>
{
}

/// Sealed capability projection trait for `wasm` tokens and conjunctions.
pub trait WasmCaps: crate::tokens::Sealed {
    type Simd128: Flag;
    type Relaxed: Flag;
    type RelaxedFma: Flag;
}

impl<Base, Extra> WasmCaps for crate::tokens::And<Base, Extra>
where
    Base: crate::tokens::SameArch<Extra> + WasmCaps,
    Extra: SimdToken + WasmCaps,
{
    type Simd128 = <Base::Simd128 as Flag>::Or<Extra::Simd128>;
    type Relaxed = <Base::Relaxed as Flag>::Or<Extra::Relaxed>;
    type RelaxedFma = <Base::RelaxedFma as Flag>::Or<Extra::RelaxedFma>;
}

#[allow(clippy::type_complexity)]
impl<L0: Flag, L1: Flag, L2: Flag, R0: Flag, R1: Flag, R2: Flag> CapSetOr<WasmCapSet<R0, R1, R2>>
    for WasmCapSet<L0, L1, L2>
{
    type Output = WasmCapSet<<L0 as Flag>::Or<R0>, <L1 as Flag>::Or<R1>, <L2 as Flag>::Or<R2>>;
}

// Canonical zero-cost token extractors (guarded by sealed capability slots)
#[doc(hidden)]
#[inline(always)]
pub fn extract_v1<T: SimdToken + X64Caps<V1 = Yes>>(_proof: T) -> super::X64V1Token {
    super::X64V1Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v1_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V1Token {
    super::X64V1Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_v2<T: SimdToken + X64Caps<V1 = Yes, V2 = Yes>>(_proof: T) -> super::X64V2Token {
    super::X64V2Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v2_caps<
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V2Token {
    super::X64V2Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_crypto<T: SimdToken + X64Caps<Crypto128 = Yes, V1 = Yes, V2 = Yes>>(
    _proof: T,
) -> super::X64CryptoToken {
    super::X64CryptoToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_crypto_caps<
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64CryptoToken {
    super::X64CryptoToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_v3<T: SimdToken + X64Caps<V1 = Yes, V2 = Yes, V3 = Yes>>(
    _proof: T,
) -> super::X64V3Token {
    super::X64V3Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v3_caps<
    C2: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            C2,
            Yes,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V3Token {
    super::X64V3Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_v3_crypto<
    T: SimdToken + X64Caps<Crypto128 = Yes, V1 = Yes, V2 = Yes, V3 = Yes, VpclmulVaes = Yes>,
>(
    _proof: T,
) -> super::X64V3CryptoToken {
    super::X64V3CryptoToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v3_crypto_caps<
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V3CryptoToken {
    super::X64V3CryptoToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_v3_gfni_crypto<
    T: SimdToken
        + X64Caps<Crypto128 = Yes, Gfni = Yes, V1 = Yes, V2 = Yes, V3 = Yes, VpclmulVaes = Yes>,
>(
    _proof: T,
) -> super::X64V3GfniCryptoToken {
    super::X64V3GfniCryptoToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v3_gfni_crypto_caps<
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V3GfniCryptoToken {
    super::X64V3GfniCryptoToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_v3_vnni<
    T: SimdToken
        + X64Caps<
            Adx = Yes,
            AvxVnni = Yes,
            Crypto128 = Yes,
            Gfni = Yes,
            Sha = Yes,
            V1 = Yes,
            V2 = Yes,
            V3 = Yes,
            VpclmulVaes = Yes,
        >,
>(
    _proof: T,
) -> super::X64V3VnniToken {
    super::X64V3VnniToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v3_vnni_caps<
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V3VnniToken {
    super::X64V3VnniToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_v3_vnni2<
    T: SimdToken
        + X64Caps<
            Adx = Yes,
            AvxIfma = Yes,
            AvxNeConvert = Yes,
            AvxVnni = Yes,
            AvxVnniInt8 = Yes,
            Crypto128 = Yes,
            Gfni = Yes,
            Sha = Yes,
            V1 = Yes,
            V2 = Yes,
            V3 = Yes,
            VpclmulVaes = Yes,
        >,
>(
    _proof: T,
) -> super::X64V3Vnni2Token {
    super::X64V3Vnni2Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v3_vnni2_caps<
    C11: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            C11,
            Yes,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V3Vnni2Token {
    super::X64V3Vnni2Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_v4<
    T: SimdToken + X64Caps<Crypto128 = Yes, V1 = Yes, V2 = Yes, V3 = Yes, V4 = Yes>,
>(
    _proof: T,
) -> super::X64V4Token {
    super::X64V4Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v4_caps<
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            Yes,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            Yes,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V4Token {
    super::X64V4Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_fp16<
    T: SimdToken + X64Caps<Avx512Fp16 = Yes, Crypto128 = Yes, V1 = Yes, V2 = Yes, V3 = Yes, V4 = Yes>,
>(
    _proof: T,
) -> super::Avx512Fp16Token {
    super::Avx512Fp16Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_fp16_caps<
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C15: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            Yes,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            Yes,
            C15,
            Yes,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::Avx512Fp16Token {
    super::Avx512Fp16Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_avx512_bf16<
    T: SimdToken + X64Caps<Avx512Bf16 = Yes, Crypto128 = Yes, V1 = Yes, V2 = Yes, V3 = Yes, V4 = Yes>,
>(
    _proof: T,
) -> super::X64Avx512Bf16Token {
    super::X64Avx512Bf16Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_avx512_bf16_caps<
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C15: Flag,
    C16: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            Yes,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            Yes,
            C15,
            C16,
            Yes,
        >,
    >,
>(
    _proof: T,
) -> super::X64Avx512Bf16Token {
    super::X64Avx512Bf16Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_v4x<
    T: SimdToken
        + X64Caps<
            Crypto128 = Yes,
            Gfni = Yes,
            V1 = Yes,
            V2 = Yes,
            V3 = Yes,
            V4 = Yes,
            V4x = Yes,
            VpclmulVaes = Yes,
        >,
>(
    _proof: T,
) -> super::X64V4xToken {
    super::X64V4xToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_v4x_caps<
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            Yes,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            Yes,
            Yes,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64V4xToken {
    super::X64V4xToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_gfni<T: SimdToken + X64Caps<Gfni = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64GfniToken {
    super::X64GfniToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_gfni_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            Yes,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64GfniToken {
    super::X64GfniToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_sha<T: SimdToken + X64Caps<Sha = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64ShaToken {
    super::X64ShaToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_sha_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            Yes,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64ShaToken {
    super::X64ShaToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_adx<T: SimdToken + X64Caps<Adx = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64AdxToken {
    super::X64AdxToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_adx_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            Yes,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64AdxToken {
    super::X64AdxToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_avx_vnni<T: SimdToken + X64Caps<AvxVnni = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64AvxVnniToken {
    super::X64AvxVnniToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_avx_vnni_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            Yes,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64AvxVnniToken {
    super::X64AvxVnniToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_avx_ifma<T: SimdToken + X64Caps<AvxIfma = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64AvxIfmaToken {
    super::X64AvxIfmaToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_avx_ifma_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            Yes,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64AvxIfmaToken {
    super::X64AvxIfmaToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_avx_vnni_int8<T: SimdToken + X64Caps<AvxVnniInt8 = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64AvxVnniInt8Token {
    super::X64AvxVnniInt8Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_avx_vnni_int8_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C11: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            Yes,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64AvxVnniInt8Token {
    super::X64AvxVnniInt8Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_avx_vnni_int16<T: SimdToken + X64Caps<AvxVnniInt16 = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64AvxVnniInt16Token {
    super::X64AvxVnniInt16Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_avx_vnni_int16_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C12: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            Yes,
            C12,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64AvxVnniInt16Token {
    super::X64AvxVnniInt16Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_avx_ne_convert<T: SimdToken + X64Caps<AvxNeConvert = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64AvxNeConvertToken {
    super::X64AvxNeConvertToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_avx_ne_convert_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C13: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            Yes,
            C13,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64AvxNeConvertToken {
    super::X64AvxNeConvertToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_x64_sha512<T: SimdToken + X64Caps<Sha512 = Yes, V1 = Yes>>(
    _proof: T,
) -> super::X64Sha512Token {
    super::X64Sha512Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_x64_sha512_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    C12: Flag,
    C14: Flag,
    C15: Flag,
    C16: Flag,
    C17: Flag,
    T: SimdToken<
        __Caps = X64CapSet<
            Yes,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            Yes,
            C14,
            C15,
            C16,
            C17,
        >,
    >,
>(
    _proof: T,
) -> super::X64Sha512Token {
    super::X64Sha512Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon<T: SimdToken + ArmCaps<Neon = Yes>>(_proof: T) -> super::NeonToken {
    super::NeonToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonToken {
    super::NeonToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_aes<T: SimdToken + ArmCaps<Aes = Yes, Neon = Yes>>(
    _proof: T,
) -> super::NeonAesToken {
    super::NeonAesToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_aes_caps<
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, Yes, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonAesToken {
    super::NeonAesToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_sha2<T: SimdToken + ArmCaps<Neon = Yes, Sha2 = Yes>>(
    _proof: T,
) -> super::NeonSha2Token {
    super::NeonSha2Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_sha2_caps<
    C1: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, Yes, C3, C4, C5, C6, C7, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonSha2Token {
    super::NeonSha2Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_sha3<T: SimdToken + ArmCaps<Neon = Yes, Sha3 = Yes>>(
    _proof: T,
) -> super::NeonSha3Token {
    super::NeonSha3Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_sha3_caps<
    C1: Flag,
    C2: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, Yes, C4, C5, C6, C7, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonSha3Token {
    super::NeonSha3Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_crc<T: SimdToken + ArmCaps<Crc = Yes, Neon = Yes>>(
    _proof: T,
) -> super::NeonCrcToken {
    super::NeonCrcToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_crc_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, Yes, C5, C6, C7, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonCrcToken {
    super::NeonCrcToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_rdm<T: SimdToken + ArmCaps<Neon = Yes, Rdm = Yes>>(
    _proof: T,
) -> super::NeonRdmToken {
    super::NeonRdmToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_rdm_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, C4, Yes, C6, C7, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonRdmToken {
    super::NeonRdmToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_dotprod<T: SimdToken + ArmCaps<DotProd = Yes, Neon = Yes>>(
    _proof: T,
) -> super::NeonDotProdToken {
    super::NeonDotProdToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_dotprod_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, C4, C5, Yes, C7, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonDotProdToken {
    super::NeonDotProdToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_fp16<T: SimdToken + ArmCaps<Fp16 = Yes, Neon = Yes>>(
    _proof: T,
) -> super::NeonFp16Token {
    super::NeonFp16Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_fp16_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, C4, C5, C6, Yes, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonFp16Token {
    super::NeonFp16Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_fcma<T: SimdToken + ArmCaps<Fcma = Yes, Neon = Yes>>(
    _proof: T,
) -> super::NeonFcmaToken {
    super::NeonFcmaToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_fcma_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, C4, C5, C6, C7, C8, Yes, C10, C11>>,
>(
    _proof: T,
) -> super::NeonFcmaToken {
    super::NeonFcmaToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_i8mm<T: SimdToken + ArmCaps<I8mm = Yes, Neon = Yes>>(
    _proof: T,
) -> super::NeonI8mmToken {
    super::NeonI8mmToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_i8mm_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, C4, C5, C6, C7, C8, C9, Yes, C11>>,
>(
    _proof: T,
) -> super::NeonI8mmToken {
    super::NeonI8mmToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_bf16<T: SimdToken + ArmCaps<Bf16 = Yes, Neon = Yes>>(
    _proof: T,
) -> super::NeonBf16Token {
    super::NeonBf16Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_bf16_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C7: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, Yes>>,
>(
    _proof: T,
) -> super::NeonBf16Token {
    super::NeonBf16Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_neon_fhm<T: SimdToken + ArmCaps<Fhm = Yes, Fp16 = Yes, Neon = Yes>>(
    _proof: T,
) -> super::NeonFhmToken {
    super::NeonFhmToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_neon_fhm_caps<
    C1: Flag,
    C2: Flag,
    C3: Flag,
    C4: Flag,
    C5: Flag,
    C6: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, C1, C2, C3, C4, C5, C6, Yes, Yes, C9, C10, C11>>,
>(
    _proof: T,
) -> super::NeonFhmToken {
    super::NeonFhmToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_arm_v2<
    T: SimdToken
        + ArmCaps<Aes = Yes, Crc = Yes, DotProd = Yes, Fp16 = Yes, Neon = Yes, Rdm = Yes, Sha2 = Yes>,
>(
    _proof: T,
) -> super::Arm64V2Token {
    super::Arm64V2Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_arm_v2_caps<
    C3: Flag,
    C8: Flag,
    C9: Flag,
    C10: Flag,
    C11: Flag,
    T: SimdToken<__Caps = ArmCapSet<Yes, Yes, Yes, C3, Yes, Yes, Yes, Yes, C8, C9, C10, C11>>,
>(
    _proof: T,
) -> super::Arm64V2Token {
    super::Arm64V2Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_arm_v3<
    T: SimdToken
        + ArmCaps<
            Aes = Yes,
            Bf16 = Yes,
            Crc = Yes,
            DotProd = Yes,
            Fcma = Yes,
            Fhm = Yes,
            Fp16 = Yes,
            I8mm = Yes,
            Neon = Yes,
            Rdm = Yes,
            Sha2 = Yes,
            Sha3 = Yes,
        >,
>(
    _proof: T,
) -> super::Arm64V3Token {
    super::Arm64V3Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_arm_v3_caps<
    T: SimdToken<__Caps = ArmCapSet<Yes, Yes, Yes, Yes, Yes, Yes, Yes, Yes, Yes, Yes, Yes, Yes>>,
>(
    _proof: T,
) -> super::Arm64V3Token {
    super::Arm64V3Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_wasm128<T: SimdToken + WasmCaps<Simd128 = Yes>>(_proof: T) -> super::Wasm128Token {
    super::Wasm128Token { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_wasm128_caps<C1: Flag, C2: Flag, T: SimdToken<__Caps = WasmCapSet<Yes, C1, C2>>>(
    _proof: T,
) -> super::Wasm128Token {
    super::Wasm128Token { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_wasm128_relaxed<T: SimdToken + WasmCaps<Relaxed = Yes, Simd128 = Yes>>(
    _proof: T,
) -> super::Wasm128RelaxedToken {
    super::Wasm128RelaxedToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_wasm128_relaxed_caps<C2: Flag, T: SimdToken<__Caps = WasmCapSet<Yes, Yes, C2>>>(
    _proof: T,
) -> super::Wasm128RelaxedToken {
    super::Wasm128RelaxedToken { _private: () }
}
#[doc(hidden)]
#[inline(always)]
pub fn extract_wasm128_relaxed_fma<
    T: SimdToken + WasmCaps<Relaxed = Yes, RelaxedFma = Yes, Simd128 = Yes>,
>(
    _proof: T,
) -> super::Wasm128RelaxedFmaToken {
    super::Wasm128RelaxedFmaToken { _private: () }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn extract_wasm128_relaxed_fma_caps<T: SimdToken<__Caps = WasmCapSet<Yes, Yes, Yes>>>(
    _proof: T,
) -> super::Wasm128RelaxedFmaToken {
    super::Wasm128RelaxedFmaToken { _private: () }
}

/// FNV-1a verification tag for [`And`](crate::tokens::And) conjunction types.
#[doc(hidden)]
pub const AND_ASSERT_TAG: u32 = 0x71B5ED46;

impl crate::tokens::ScalarToken {
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0xD141EACA;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_TIER_D141EACA: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0xD141EACA) as usize];
}

impl<Base, Extra> crate::tokens::And<Base, Extra>
where
    Base: crate::tokens::SameArch<Extra>,
    Extra: SimdToken,
{
    #[doc(hidden)]
    pub const __ARCHMAGE_TIER_TAG: u32 = 0x71B5ED46;

    #[doc(hidden)]
    pub const __ARCHMAGE_ASSERT_AND_71B5ED46: () =
        [()][!(Self::__ARCHMAGE_TIER_TAG == 0x71B5ED46) as usize];
}
