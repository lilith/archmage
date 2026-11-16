//! Composable sealed capability traits for sub-tier hardware instructions.
//!
//! All [`archmage::SimdToken`]s — including sub-tier proof tokens (`X64V3VnniToken`,
//! `X64V3Vnni2Token`, `X64V4xToken`, `Arm64V2Token`, `Arm64V3Token`,
//! `Wasm128RelaxedToken`, `Wasm128RelaxedFmaToken`), orthogonal leaf tokens,
//! and [`archmage::And`] conjunctions (`X64V3Token.and(X64AvxVnniToken)`, etc.) —
//! implement every base width backend trait directly. Binding a generic kernel on
//! one or more capability traits gives access to the sub-tier instructions as
//! tokenless methods directly on `f32x4<T>`, `i32x8<T>`, `u64x4<T>`, etc. —
//! with zero extra token arguments and zero `.magetypes()` conversions:
//!
//! - [`DotU8I8`]: x86 VNNI (`u8 * i8 -> i32`, `i16 * i16 -> i32` dot products).
//!   Implemented for every token or `And` conjunction providing V3 + VNNI
//!   (`X64V3Token.and(X64AvxVnniToken)`, `X64V3VnniToken`, `X64V3Vnni2Token`)
//!   and `X64V4xToken` (with `feature = "avx512"`).
//! - [`Int52Ifma`]: x86 52-bit unsigned integer multiply-accumulate (`VPMADD52LUQ` / `VPMADD52HUQ`).
//!   Implemented for every token or `And` conjunction providing V3 + AVX-IFMA
//!   (`X64V3Token.and(X64AvxIfmaToken)`, `X64V3Vnni2Token`) and `X64V4xToken`
//!   (EVEX `avx512ifma`, with `feature = "avx512"`).
//! - [`DotI8Signed`]: ARMv8.2-A DotProd signed 4-way `i8 * i8 -> i32` accumulate (`SDOT`).
//!   Implemented for every NEON + DotProd token (`NeonDotProdToken`, `Arm64V2Token`, `Arm64V3Token`).
//! - [`DotU8`]: ARMv8.2-A DotProd unsigned 4-way `u8 * u8 -> u32` accumulate (`UDOT`).
//!   Implemented for every NEON + DotProd token (`NeonDotProdToken`, `Arm64V2Token`, `Arm64V3Token`).
//! - [`ArmRdma`]: ARMv8.1-A RDMA saturating rounding doubling multiply-add/sub (`SQRDMLAH` / `SQRDMLSH`).
//!   Implemented for every NEON + RDM token (`NeonRdmToken`, `Arm64V2Token`, `Arm64V3Token`).
//! - [`ArmF16Fma`]: ARMv8.2-A FHM fused `f16 -> f32` multiply-add/sub long (`FMLAL` / `FMLSL`).
//!   Implemented for every NEON + FP16 + FHM token (`NeonFhmToken`, `Arm64V3Token`).
//! - [`RelaxedSimd`]: WASM Relaxed SIMD non-FMA instructions (`relaxed_min`, `relaxed_max`,
//!   relaxed truncations, `relaxed_q15mulr`, relaxed dot products). Implemented by
//!   `Wasm128RelaxedToken`, `Wasm128RelaxedFmaToken`, and `And` conjunctions containing them
//!   when compiled with `+simd128,+relaxed-simd`. Relaxed FMA remains gated on
//!   `Wasm128RelaxedFmaToken` and its runtime single-rounding probe.

#![forbid(unsafe_code)]
#![allow(private_bounds)]

use crate::simd::backends::{
    F32x4Backend, F32x8Backend, F64x2Backend, F64x4Backend, I8x16Backend, I8x32Backend,
    I16x8Backend, I16x16Backend, I32x4Backend, I32x8Backend, U8x16Backend, U8x32Backend,
    U16x8Backend, U32x4Backend, U32x8Backend, U64x2Backend, U64x4Backend,
};
use crate::simd::generic::{
    f32x4, f32x8, f64x2, f64x4, i8x16, i8x32, i16x8, i16x16, i32x4, i32x8, u8x16, u8x32, u16x8,
    u32x4, u32x8, u64x2, u64x4,
};
#[cfg(feature = "w512")]
use crate::simd::generic::{i8x64, i16x32, i32x16, u8x64, u32x16, u64x8};

pub(super) mod cap_sealed {
    use crate::simd::backends::{
        F32x4Backend, F32x8Backend, F64x2Backend, F64x4Backend, I8x16Backend, I8x32Backend,
        I16x8Backend, I16x16Backend, I32x4Backend, I32x8Backend, U8x16Backend, U8x32Backend,
        U16x8Backend, U32x4Backend, U32x8Backend, U64x2Backend, U64x4Backend,
    };
    #[cfg(feature = "w512")]
    use crate::simd::backends::{
        I8x64Backend, I16x32Backend, I32x16Backend, U8x64Backend, U32x16Backend, U64x8Backend,
    };

    pub trait Sealed {}

    pub trait DotU8I8Ops:
        I32x4Backend
        + U8x16Backend
        + I8x16Backend
        + I16x8Backend
        + I32x8Backend
        + U8x32Backend
        + I8x32Backend
        + I16x16Backend
    {
        fn dot_u8_i8_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as U8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr;
        fn dot_u8_i8_sat_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as U8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr;
        fn dot_i16_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr;
        fn dot_i16_sat_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr;

        fn dot_u8_i8_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as U8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr;
        fn dot_u8_i8_sat_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as U8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr;
        fn dot_i16_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr;
        fn dot_i16_sat_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr;
    }

    #[cfg(feature = "w512")]
    pub trait DotU8I8W512: I32x16Backend + U8x64Backend + I8x64Backend + I16x32Backend {
        fn dot_u8_i8_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as U8x64Backend>::Repr,
            b: <Self as I8x64Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr;
        fn dot_u8_i8_sat_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as U8x64Backend>::Repr,
            b: <Self as I8x64Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr;
        fn dot_i16_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as I16x32Backend>::Repr,
            b: <Self as I16x32Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr;
        fn dot_i16_sat_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as I16x32Backend>::Repr,
            b: <Self as I16x32Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr;
    }

    #[cfg(not(feature = "w512"))]
    pub trait DotU8I8W512 {}

    pub trait DotI8SignedOps: I32x4Backend + I8x16Backend + I32x8Backend + I8x32Backend {
        fn dot_i8_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as I8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr;
        fn dot_i8_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as I8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr;
    }

    #[cfg(feature = "w512")]
    pub trait DotI8SignedW512: I32x16Backend + I8x64Backend {
        fn dot_i8_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as I8x64Backend>::Repr,
            b: <Self as I8x64Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr;
    }

    #[cfg(not(feature = "w512"))]
    pub trait DotI8SignedW512 {}

    pub trait DotU8Ops: U32x4Backend + U8x16Backend + U32x8Backend + U8x32Backend {
        fn dot_u8_x4(
            self,
            acc: <Self as U32x4Backend>::Repr,
            a: <Self as U8x16Backend>::Repr,
            b: <Self as U8x16Backend>::Repr,
        ) -> <Self as U32x4Backend>::Repr;
        fn dot_u8_x8(
            self,
            acc: <Self as U32x8Backend>::Repr,
            a: <Self as U8x32Backend>::Repr,
            b: <Self as U8x32Backend>::Repr,
        ) -> <Self as U32x8Backend>::Repr;
    }

    #[cfg(feature = "w512")]
    pub trait DotU8W512: U32x16Backend + U8x64Backend {
        fn dot_u8_x16(
            self,
            acc: <Self as U32x16Backend>::Repr,
            a: <Self as U8x64Backend>::Repr,
            b: <Self as U8x64Backend>::Repr,
        ) -> <Self as U32x16Backend>::Repr;
    }

    #[cfg(not(feature = "w512"))]
    pub trait DotU8W512 {}

    pub trait Int52IfmaOps: U64x2Backend + U64x4Backend {
        fn madd52_lo_x2(
            self,
            acc: <Self as U64x2Backend>::Repr,
            a: <Self as U64x2Backend>::Repr,
            b: <Self as U64x2Backend>::Repr,
        ) -> <Self as U64x2Backend>::Repr;
        fn madd52_hi_x2(
            self,
            acc: <Self as U64x2Backend>::Repr,
            a: <Self as U64x2Backend>::Repr,
            b: <Self as U64x2Backend>::Repr,
        ) -> <Self as U64x2Backend>::Repr;
        fn madd52_lo_x4(
            self,
            acc: <Self as U64x4Backend>::Repr,
            a: <Self as U64x4Backend>::Repr,
            b: <Self as U64x4Backend>::Repr,
        ) -> <Self as U64x4Backend>::Repr;
        fn madd52_hi_x4(
            self,
            acc: <Self as U64x4Backend>::Repr,
            a: <Self as U64x4Backend>::Repr,
            b: <Self as U64x4Backend>::Repr,
        ) -> <Self as U64x4Backend>::Repr;
    }

    #[cfg(feature = "w512")]
    pub trait Int52IfmaW512: U64x8Backend {
        fn madd52_lo_x8(
            self,
            acc: <Self as U64x8Backend>::Repr,
            a: <Self as U64x8Backend>::Repr,
            b: <Self as U64x8Backend>::Repr,
        ) -> <Self as U64x8Backend>::Repr;
        fn madd52_hi_x8(
            self,
            acc: <Self as U64x8Backend>::Repr,
            a: <Self as U64x8Backend>::Repr,
            b: <Self as U64x8Backend>::Repr,
        ) -> <Self as U64x8Backend>::Repr;
    }

    #[cfg(not(feature = "w512"))]
    pub trait Int52IfmaW512 {}

    pub trait ArmRdmaOps: I16x8Backend + I16x16Backend {
        fn qrdmlah_x8(
            self,
            acc: <Self as I16x8Backend>::Repr,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I16x8Backend>::Repr;
        fn qrdmlsh_x8(
            self,
            acc: <Self as I16x8Backend>::Repr,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I16x8Backend>::Repr;
        fn qrdmlah_x16(
            self,
            acc: <Self as I16x16Backend>::Repr,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I16x16Backend>::Repr;
        fn qrdmlsh_x16(
            self,
            acc: <Self as I16x16Backend>::Repr,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I16x16Backend>::Repr;
    }

    #[cfg(feature = "w512")]
    pub trait ArmRdmaW512: I16x32Backend {
        fn qrdmlah_x32(
            self,
            acc: <Self as I16x32Backend>::Repr,
            a: <Self as I16x32Backend>::Repr,
            b: <Self as I16x32Backend>::Repr,
        ) -> <Self as I16x32Backend>::Repr;
        fn qrdmlsh_x32(
            self,
            acc: <Self as I16x32Backend>::Repr,
            a: <Self as I16x32Backend>::Repr,
            b: <Self as I16x32Backend>::Repr,
        ) -> <Self as I16x32Backend>::Repr;
    }

    #[cfg(not(feature = "w512"))]
    pub trait ArmRdmaW512 {}

    pub trait ArmF16FmaOps: F32x4Backend + F32x8Backend + U16x8Backend {
        fn fmlal_low_x4(
            self,
            acc: <Self as F32x4Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr;
        fn fmlal_high_x4(
            self,
            acc: <Self as F32x4Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr;
        fn fmlsl_low_x4(
            self,
            acc: <Self as F32x4Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr;
        fn fmlsl_high_x4(
            self,
            acc: <Self as F32x4Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr;
        fn fmlal_x8(
            self,
            acc: <Self as F32x8Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x8Backend>::Repr;
        fn fmlsl_x8(
            self,
            acc: <Self as F32x8Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x8Backend>::Repr;
    }

    pub trait RelaxedSimdOps:
        F32x4Backend
        + F32x8Backend
        + F64x2Backend
        + F64x4Backend
        + I8x16Backend
        + I8x32Backend
        + I16x8Backend
        + I16x16Backend
        + I32x4Backend
        + I32x8Backend
        + U32x4Backend
        + U32x8Backend
    {
        fn f32x4_relaxed_min(
            self,
            a: <Self as F32x4Backend>::Repr,
            b: <Self as F32x4Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr;
        fn f32x4_relaxed_max(
            self,
            a: <Self as F32x4Backend>::Repr,
            b: <Self as F32x4Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr;
        fn f32x4_relaxed_to_i32(
            self,
            a: <Self as F32x4Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr;
        fn f32x4_relaxed_to_u32(
            self,
            a: <Self as F32x4Backend>::Repr,
        ) -> <Self as U32x4Backend>::Repr;

        fn f32x8_relaxed_min(
            self,
            a: <Self as F32x8Backend>::Repr,
            b: <Self as F32x8Backend>::Repr,
        ) -> <Self as F32x8Backend>::Repr;
        fn f32x8_relaxed_max(
            self,
            a: <Self as F32x8Backend>::Repr,
            b: <Self as F32x8Backend>::Repr,
        ) -> <Self as F32x8Backend>::Repr;
        fn f32x8_relaxed_to_i32(
            self,
            a: <Self as F32x8Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr;
        fn f32x8_relaxed_to_u32(
            self,
            a: <Self as F32x8Backend>::Repr,
        ) -> <Self as U32x8Backend>::Repr;

        fn f64x2_relaxed_min(
            self,
            a: <Self as F64x2Backend>::Repr,
            b: <Self as F64x2Backend>::Repr,
        ) -> <Self as F64x2Backend>::Repr;
        fn f64x2_relaxed_max(
            self,
            a: <Self as F64x2Backend>::Repr,
            b: <Self as F64x2Backend>::Repr,
        ) -> <Self as F64x2Backend>::Repr;

        fn f64x4_relaxed_min(
            self,
            a: <Self as F64x4Backend>::Repr,
            b: <Self as F64x4Backend>::Repr,
        ) -> <Self as F64x4Backend>::Repr;
        fn f64x4_relaxed_max(
            self,
            a: <Self as F64x4Backend>::Repr,
            b: <Self as F64x4Backend>::Repr,
        ) -> <Self as F64x4Backend>::Repr;

        fn i16x8_relaxed_mul_hrs_q15(
            self,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I16x8Backend>::Repr;
        fn i16x8_relaxed_dot_i8_i7(
            self,
            a: <Self as I8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I16x8Backend>::Repr;

        fn i16x16_relaxed_mul_hrs_q15(
            self,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I16x16Backend>::Repr;
        fn i16x16_relaxed_dot_i8_i7(
            self,
            a: <Self as I8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I16x16Backend>::Repr;

        fn i32x4_relaxed_dot_i8_i7_add(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as I8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr;
        fn i32x8_relaxed_dot_i8_i7_add(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as I8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr;
    }
}

/// Sealed capability trait for x86 VNNI `u8 * i8 -> i32` and `i16 * i16 -> i32`
/// dot-product accumulation (`VPDPBUSD`, `VPDPBUSDS`, `VPDPWSSD`, `VPDPWSSDS`).
///
/// Bound generic kernels on `T: DotU8I8` to call [`i32x4::dot_u8_i8`],
/// [`i32x4::dot_u8_i8_sat`], [`i32x4::dot_i16`], [`i32x4::dot_i16_sat`] (and their
/// `i32x8` / 512-bit `i32x16` counterparts) without passing extra token parameters.
///
/// Implemented for all tokens and [`archmage::And`] conjunctions providing
/// V3 + AVX-VNNI (`X64V3Token.and(X64AvxVnniToken)`, `X64V3VnniToken`,
/// `X64V3Vnni2Token`), and `X64V4xToken` when `feature = "avx512"` is enabled.
pub trait DotU8I8:
    cap_sealed::Sealed
    + cap_sealed::DotU8I8Ops
    + cap_sealed::DotU8I8W512
    + I32x4Backend
    + U8x16Backend
    + I8x16Backend
    + I16x8Backend
    + I32x8Backend
    + U8x32Backend
    + I8x32Backend
    + I16x16Backend
    + archmage::SimdToken
    + Copy
    + 'static
{
}

/// Sealed capability trait for ARMv8.2-A DotProd signed `i8 * i8 -> i32`
/// 4-way dot-product accumulation (`SDOT`).
///
/// Bound generic kernels on `T: DotI8Signed` to call [`i32x4::dot_i8_acc`] and
/// [`i32x8::dot_i8_acc`] without extra token parameters.
///
/// Implemented for all NEON + DotProd tokens and [`archmage::And`] conjunctions
/// (`NeonDotProdToken`, `Arm64V2Token`, `Arm64V3Token`).
pub trait DotI8Signed:
    cap_sealed::Sealed
    + cap_sealed::DotI8SignedOps
    + cap_sealed::DotI8SignedW512
    + I32x4Backend
    + I8x16Backend
    + I32x8Backend
    + I8x32Backend
    + archmage::SimdToken
    + Copy
    + 'static
{
}

/// Sealed capability trait for ARMv8.2-A DotProd unsigned `u8 * u8 -> u32`
/// 4-way dot-product accumulation (`UDOT`).
///
/// Bound generic kernels on `T: DotU8` to call [`u32x4::dot_u8_acc`] and
/// [`u32x8::dot_u8_acc`] without extra token parameters.
///
/// Implemented for all NEON + DotProd tokens and [`archmage::And`] conjunctions
/// (`NeonDotProdToken`, `Arm64V2Token`, `Arm64V3Token`).
pub trait DotU8:
    cap_sealed::Sealed
    + cap_sealed::DotU8Ops
    + cap_sealed::DotU8W512
    + U32x4Backend
    + U8x16Backend
    + U32x8Backend
    + U8x32Backend
    + archmage::SimdToken
    + Copy
    + 'static
{
}

/// Sealed capability trait for x86 52-bit unsigned integer multiply-accumulate
/// (`VPMADD52LUQ` / `VPMADD52HUQ`).
///
/// Bound generic kernels on `T: Int52Ifma` to call [`u64x2::madd52_lo`],
/// [`u64x2::madd52_hi`], [`u64x4::madd52_lo`], and [`u64x4::madd52_hi`]
/// (plus 512-bit `u64x8` under `w512`).
///
/// Implemented for all tokens and [`archmage::And`] conjunctions providing
/// V3 + AVX-IFMA (`X64V3Token.and(X64AvxIfmaToken)`, `X64V3Vnni2Token`),
/// and `X64V4xToken` when `feature = "avx512"` is enabled (EVEX `avx512ifma`).
pub trait Int52Ifma:
    cap_sealed::Sealed
    + cap_sealed::Int52IfmaOps
    + cap_sealed::Int52IfmaW512
    + U64x2Backend
    + U64x4Backend
    + archmage::SimdToken
    + Copy
    + 'static
{
}

/// Sealed capability trait for ARMv8.1-A RDMA signed saturating rounding doubling
/// multiply-accumulate/subtract high half (`SQRDMLAH`, `SQRDMLSH`).
///
/// Bound generic kernels on `T: ArmRdma` to call [`i16x8::qrdmlah_acc`],
/// [`i16x8::qrdmlsh_acc`], [`i16x16::qrdmlah_acc`], and [`i16x16::qrdmlsh_acc`].
///
/// Implemented for all NEON + RDM tokens and [`archmage::And`] conjunctions
/// (`NeonRdmToken`, `Arm64V2Token`, `Arm64V3Token`).
pub trait ArmRdma:
    cap_sealed::Sealed
    + cap_sealed::ArmRdmaOps
    + cap_sealed::ArmRdmaW512
    + I16x8Backend
    + I16x16Backend
    + archmage::SimdToken
    + Copy
    + 'static
{
}

/// Sealed capability trait for ARMv8.2-A FHM fused `f16 -> f32` multiply-add/subtract
/// long (`FMLAL`, `FMLAL2`, `FMLSL`, `FMLSL2`).
///
/// Bound generic kernels on `T: ArmF16Fma` to call [`f32x4::fmlal_low_f16_acc`],
/// [`f32x4::fmlal_high_f16_acc`], [`f32x4::fmlsl_low_f16_acc`], [`f32x4::fmlsl_high_f16_acc`],
/// [`f32x8::fmlal_f16_acc`], and [`f32x8::fmlsl_f16_acc`].
///
/// Implemented for all NEON + FP16 + FHM tokens and [`archmage::And`] conjunctions
/// (`NeonFhmToken`, `Arm64V3Token`).
pub trait ArmF16Fma:
    cap_sealed::Sealed
    + cap_sealed::ArmF16FmaOps
    + F32x4Backend
    + F32x8Backend
    + U16x8Backend
    + archmage::SimdToken
    + Copy
    + 'static
{
}

/// Sealed capability trait for WASM Relaxed SIMD non-FMA operations
/// (`relaxed_min`, `relaxed_max`, relaxed float-to-int truncations,
/// `relaxed_q15mulr`, and relaxed `i8 * i7` dot products).
///
/// Implemented by `archmage::Wasm128RelaxedToken` and `archmage::Wasm128RelaxedFmaToken`
/// when compiled with `target_feature = "simd128"` and `target_feature = "relaxed-simd"`.
///
/// Note: single-rounding FMA (`relaxed_madd`) is intentionally **not** part of this trait;
/// FMA execution remains exclusively gated on `Wasm128RelaxedFmaToken`'s runtime fusion probe.
pub trait RelaxedSimd:
    cap_sealed::Sealed
    + cap_sealed::RelaxedSimdOps
    + F32x4Backend
    + F32x8Backend
    + F64x2Backend
    + F64x4Backend
    + I8x16Backend
    + I8x32Backend
    + I16x8Backend
    + I16x16Backend
    + I32x4Backend
    + I32x8Backend
    + U32x4Backend
    + U32x8Backend
    + archmage::SimdToken
    + Copy
    + 'static
{
}

// ============================================================================
// Inherent vector methods gated by capability traits
// ============================================================================

impl<T: DotU8I8> i32x4<T> {
    /// Multiply groups of 4 unsigned `u8` lanes in `a` by signed `i8` lanes in `b`
    /// and wrapping-accumulate each 4-product sum into `self` (`VPDPBUSD`).
    #[inline(always)]
    pub fn dot_u8_i8(self, a: u8x16<T>, b: i8x16<T>) -> Self {
        Self(self.1.dot_u8_i8_x4(self.0, a.0, b.0), self.1)
    }

    /// Signed-saturating `u8 * i8` 4-way dot-product accumulate into `self` (`VPDPBUSDS`).
    #[inline(always)]
    pub fn dot_u8_i8_sat(self, a: u8x16<T>, b: i8x16<T>) -> Self {
        Self(self.1.dot_u8_i8_sat_x4(self.0, a.0, b.0), self.1)
    }

    /// Multiply pairs of signed `i16` lanes in `a` and `b` and wrapping-accumulate
    /// each 2-product sum into `self` (`VPDPWSSD`).
    #[inline(always)]
    pub fn dot_i16(self, a: i16x8<T>, b: i16x8<T>) -> Self {
        Self(self.1.dot_i16_x4(self.0, a.0, b.0), self.1)
    }

    /// Signed-saturating `i16 * i16` 2-way dot-product accumulate into `self` (`VPDPWSSDS`).
    #[inline(always)]
    pub fn dot_i16_sat(self, a: i16x8<T>, b: i16x8<T>) -> Self {
        Self(self.1.dot_i16_sat_x4(self.0, a.0, b.0), self.1)
    }
}

impl<T: DotU8I8> i32x8<T> {
    /// Multiply groups of 4 unsigned `u8` lanes in `a` by signed `i8` lanes in `b`
    /// and wrapping-accumulate each 4-product sum into `self` (`VPDPBUSD`).
    #[inline(always)]
    pub fn dot_u8_i8(self, a: u8x32<T>, b: i8x32<T>) -> Self {
        Self(self.1.dot_u8_i8_x8(self.0, a.0, b.0), self.1)
    }

    /// Signed-saturating `u8 * i8` 4-way dot-product accumulate into `self` (`VPDPBUSDS`).
    #[inline(always)]
    pub fn dot_u8_i8_sat(self, a: u8x32<T>, b: i8x32<T>) -> Self {
        Self(self.1.dot_u8_i8_sat_x8(self.0, a.0, b.0), self.1)
    }

    /// Multiply pairs of signed `i16` lanes in `a` and `b` and wrapping-accumulate
    /// each 2-product sum into `self` (`VPDPWSSD`).
    #[inline(always)]
    pub fn dot_i16(self, a: i16x16<T>, b: i16x16<T>) -> Self {
        Self(self.1.dot_i16_x8(self.0, a.0, b.0), self.1)
    }

    /// Signed-saturating `i16 * i16` 2-way dot-product accumulate into `self` (`VPDPWSSDS`).
    #[inline(always)]
    pub fn dot_i16_sat(self, a: i16x16<T>, b: i16x16<T>) -> Self {
        Self(self.1.dot_i16_sat_x8(self.0, a.0, b.0), self.1)
    }
}

#[cfg(feature = "w512")]
impl<T: DotU8I8> i32x16<T> {
    /// Multiply groups of 4 unsigned `u8` lanes in `a` by signed `i8` lanes in `b`
    /// and wrapping-accumulate each 4-product sum into `self`.
    #[inline(always)]
    pub fn dot_u8_i8(self, a: u8x64<T>, b: i8x64<T>) -> Self {
        Self(self.1.dot_u8_i8_x16(self.0, a.0, b.0), self.1)
    }

    /// Signed-saturating `u8 * i8` 4-way dot-product accumulate into `self`.
    #[inline(always)]
    pub fn dot_u8_i8_sat(self, a: u8x64<T>, b: i8x64<T>) -> Self {
        Self(self.1.dot_u8_i8_sat_x16(self.0, a.0, b.0), self.1)
    }

    /// Multiply pairs of signed `i16` lanes in `a` and `b` and wrapping-accumulate
    /// each 2-product sum into `self`.
    #[inline(always)]
    pub fn dot_i16(self, a: i16x32<T>, b: i16x32<T>) -> Self {
        Self(self.1.dot_i16_x16(self.0, a.0, b.0), self.1)
    }

    /// Signed-saturating `i16 * i16` 2-way dot-product accumulate into `self`.
    #[inline(always)]
    pub fn dot_i16_sat(self, a: i16x32<T>, b: i16x32<T>) -> Self {
        Self(self.1.dot_i16_sat_x16(self.0, a.0, b.0), self.1)
    }
}

impl<T: DotI8Signed> i32x4<T> {
    /// Signed 4-way `i8 * i8` dot-product accumulate into `self` (`SDOT`).
    #[inline(always)]
    pub fn dot_i8_acc(self, a: i8x16<T>, b: i8x16<T>) -> Self {
        Self(self.1.dot_i8_x4(self.0, a.0, b.0), self.1)
    }
}

impl<T: DotI8Signed> i32x8<T> {
    /// Two-register signed 4-way `i8 * i8` dot-product accumulate into `self` (`2x SDOT`).
    #[inline(always)]
    pub fn dot_i8_acc(self, a: i8x32<T>, b: i8x32<T>) -> Self {
        Self(self.1.dot_i8_x8(self.0, a.0, b.0), self.1)
    }
}

#[cfg(feature = "w512")]
impl<T: DotI8Signed> i32x16<T> {
    /// Four-register signed 4-way `i8 * i8` dot-product accumulate into `self` (`4x SDOT`).
    #[inline(always)]
    pub fn dot_i8_acc(self, a: i8x64<T>, b: i8x64<T>) -> Self {
        Self(self.1.dot_i8_x16(self.0, a.0, b.0), self.1)
    }
}

impl<T: DotU8> u32x4<T> {
    /// Unsigned 4-way `u8 * u8` dot-product accumulate into `self` (`UDOT`).
    #[inline(always)]
    pub fn dot_u8_acc(self, a: u8x16<T>, b: u8x16<T>) -> Self {
        Self(self.1.dot_u8_x4(self.0, a.0, b.0), self.1)
    }
}

impl<T: DotU8> u32x8<T> {
    /// Two-register unsigned 4-way `u8 * u8` dot-product accumulate into `self` (`2x UDOT`).
    #[inline(always)]
    pub fn dot_u8_acc(self, a: u8x32<T>, b: u8x32<T>) -> Self {
        Self(self.1.dot_u8_x8(self.0, a.0, b.0), self.1)
    }
}

#[cfg(feature = "w512")]
impl<T: DotU8> u32x16<T> {
    /// Four-register unsigned 4-way `u8 * u8` dot-product accumulate into `self` (`4x UDOT`).
    #[inline(always)]
    pub fn dot_u8_acc(self, a: u8x64<T>, b: u8x64<T>) -> Self {
        Self(self.1.dot_u8_x16(self.0, a.0, b.0), self.1)
    }
}

impl<T: Int52Ifma> u64x2<T> {
    /// Multiply low 52 bits of `a` and `b` per lane and add bits `[51:0]` of the
    /// 104-bit product to `self` (`VPMADD52LUQ`).
    #[inline(always)]
    pub fn madd52_lo(self, a: Self, b: Self) -> Self {
        Self(self.1.madd52_lo_x2(self.0, a.0, b.0), self.1)
    }

    /// Multiply low 52 bits of `a` and `b` per lane and add bits `[103:52]` of the
    /// 104-bit product to `self` (`VPMADD52HUQ`).
    #[inline(always)]
    pub fn madd52_hi(self, a: Self, b: Self) -> Self {
        Self(self.1.madd52_hi_x2(self.0, a.0, b.0), self.1)
    }
}

impl<T: Int52Ifma> u64x4<T> {
    /// Multiply low 52 bits of `a` and `b` per lane and add bits `[51:0]` of the
    /// 104-bit product to `self` (`VPMADD52LUQ`).
    #[inline(always)]
    pub fn madd52_lo(self, a: Self, b: Self) -> Self {
        Self(self.1.madd52_lo_x4(self.0, a.0, b.0), self.1)
    }

    /// Multiply low 52 bits of `a` and `b` per lane and add bits `[103:52]` of the
    /// 104-bit product to `self` (`VPMADD52HUQ`).
    #[inline(always)]
    pub fn madd52_hi(self, a: Self, b: Self) -> Self {
        Self(self.1.madd52_hi_x4(self.0, a.0, b.0), self.1)
    }
}

#[cfg(feature = "w512")]
impl<T: Int52Ifma> u64x8<T> {
    /// Multiply low 52 bits of `a` and `b` per lane and add bits `[51:0]` of the
    /// 104-bit product to `self` (`VPMADD52LUQ`).
    #[inline(always)]
    pub fn madd52_lo(self, a: Self, b: Self) -> Self {
        Self(self.1.madd52_lo_x8(self.0, a.0, b.0), self.1)
    }

    /// Multiply low 52 bits of `a` and `b` per lane and add bits `[103:52]` of the
    /// 104-bit product to `self` (`VPMADD52HUQ`).
    #[inline(always)]
    pub fn madd52_hi(self, a: Self, b: Self) -> Self {
        Self(self.1.madd52_hi_x8(self.0, a.0, b.0), self.1)
    }
}

impl<T: ArmRdma> i16x8<T> {
    /// Signed saturating rounding doubling multiply-accumulate high half (`SQRDMLAH`).
    #[inline(always)]
    pub fn qrdmlah_acc(self, a: Self, b: Self) -> Self {
        Self(self.1.qrdmlah_x8(self.0, a.0, b.0), self.1)
    }

    /// Signed saturating rounding doubling multiply-subtract high half (`SQRDMLSH`).
    #[inline(always)]
    pub fn qrdmlsh_acc(self, a: Self, b: Self) -> Self {
        Self(self.1.qrdmlsh_x8(self.0, a.0, b.0), self.1)
    }
}

impl<T: ArmRdma> i16x16<T> {
    /// Two-register signed saturating rounding doubling multiply-accumulate high half (`2x SQRDMLAH`).
    #[inline(always)]
    pub fn qrdmlah_acc(self, a: Self, b: Self) -> Self {
        Self(self.1.qrdmlah_x16(self.0, a.0, b.0), self.1)
    }

    /// Two-register signed saturating rounding doubling multiply-subtract high half (`2x SQRDMLSH`).
    #[inline(always)]
    pub fn qrdmlsh_acc(self, a: Self, b: Self) -> Self {
        Self(self.1.qrdmlsh_x16(self.0, a.0, b.0), self.1)
    }
}

#[cfg(feature = "w512")]
impl<T: ArmRdma> i16x32<T> {
    /// Four-register signed saturating rounding doubling multiply-accumulate high half (`4x SQRDMLAH`).
    #[inline(always)]
    pub fn qrdmlah_acc(self, a: Self, b: Self) -> Self {
        Self(self.1.qrdmlah_x32(self.0, a.0, b.0), self.1)
    }

    /// Four-register signed saturating rounding doubling multiply-subtract high half (`4x SQRDMLSH`).
    #[inline(always)]
    pub fn qrdmlsh_acc(self, a: Self, b: Self) -> Self {
        Self(self.1.qrdmlsh_x32(self.0, a.0, b.0), self.1)
    }
}

impl<T: ArmF16Fma> f32x4<T> {
    /// Fused half-precision multiply-add long (low 4 `f16` lanes `0..4`: `self + f32(a_lo) * f32(b_lo)`).
    #[inline(always)]
    pub fn fmlal_low_f16_acc(self, a: u16x8<T>, b: u16x8<T>) -> Self {
        Self(self.1.fmlal_low_x4(self.0, a.0, b.0), self.1)
    }

    /// Fused half-precision multiply-add long (high 4 `f16` lanes `4..8`: `self + f32(a_hi) * f32(b_hi)`).
    #[inline(always)]
    pub fn fmlal_high_f16_acc(self, a: u16x8<T>, b: u16x8<T>) -> Self {
        Self(self.1.fmlal_high_x4(self.0, a.0, b.0), self.1)
    }

    /// Fused half-precision multiply-subtract long (low 4 `f16` lanes `0..4`: `self - f32(a_lo) * f32(b_lo)`).
    #[inline(always)]
    pub fn fmlsl_low_f16_acc(self, a: u16x8<T>, b: u16x8<T>) -> Self {
        Self(self.1.fmlsl_low_x4(self.0, a.0, b.0), self.1)
    }

    /// Fused half-precision multiply-subtract long (high 4 `f16` lanes `4..8`: `self - f32(a_hi) * f32(b_hi)`).
    #[inline(always)]
    pub fn fmlsl_high_f16_acc(self, a: u16x8<T>, b: u16x8<T>) -> Self {
        Self(self.1.fmlsl_high_x4(self.0, a.0, b.0), self.1)
    }
}

impl<T: ArmF16Fma> f32x8<T> {
    /// Fused 8-lane half-precision multiply-add long (`FMLAL` + `FMLAL2`).
    #[inline(always)]
    pub fn fmlal_f16_acc(self, a: u16x8<T>, b: u16x8<T>) -> Self {
        Self(self.1.fmlal_x8(self.0, a.0, b.0), self.1)
    }

    /// Fused 8-lane half-precision multiply-subtract long (`FMLSL` + `FMLSL2`).
    #[inline(always)]
    pub fn fmlsl_f16_acc(self, a: u16x8<T>, b: u16x8<T>) -> Self {
        Self(self.1.fmlsl_x8(self.0, a.0, b.0), self.1)
    }
}

impl<T: RelaxedSimd> f32x4<T> {
    /// Lane-wise relaxed minimum (`f32x4.relaxed_min`).
    #[inline(always)]
    pub fn relaxed_min(self, other: Self) -> Self {
        Self(self.1.f32x4_relaxed_min(self.0, other.0), self.1)
    }

    /// Lane-wise relaxed maximum (`f32x4.relaxed_max`).
    #[inline(always)]
    pub fn relaxed_max(self, other: Self) -> Self {
        Self(self.1.f32x4_relaxed_max(self.0, other.0), self.1)
    }

    /// Relaxed float-to-signed-`i32` truncation (`i32x4.relaxed_trunc_f32x4_s`).
    #[inline(always)]
    pub fn relaxed_to_i32(self) -> i32x4<T> {
        i32x4::from_repr_unchecked(self.1, self.1.f32x4_relaxed_to_i32(self.0))
    }

    /// Relaxed float-to-unsigned-`u32` truncation (`i32x4.relaxed_trunc_f32x4_u`).
    #[inline(always)]
    pub fn relaxed_to_u32(self) -> u32x4<T> {
        u32x4::from_repr_unchecked(self.1, self.1.f32x4_relaxed_to_u32(self.0))
    }
}

impl<T: RelaxedSimd> f32x8<T> {
    /// Two-register lane-wise relaxed minimum (`2x f32x4.relaxed_min`).
    #[inline(always)]
    pub fn relaxed_min(self, other: Self) -> Self {
        Self(self.1.f32x8_relaxed_min(self.0, other.0), self.1)
    }

    /// Two-register lane-wise relaxed maximum (`2x f32x4.relaxed_max`).
    #[inline(always)]
    pub fn relaxed_max(self, other: Self) -> Self {
        Self(self.1.f32x8_relaxed_max(self.0, other.0), self.1)
    }

    /// Two-register relaxed float-to-signed-`i32` truncation (`2x i32x4.relaxed_trunc_f32x4_s`).
    #[inline(always)]
    pub fn relaxed_to_i32(self) -> i32x8<T> {
        i32x8::from_repr_unchecked(self.1, self.1.f32x8_relaxed_to_i32(self.0))
    }

    /// Two-register relaxed float-to-unsigned-`u32` truncation (`2x i32x4.relaxed_trunc_f32x4_u`).
    #[inline(always)]
    pub fn relaxed_to_u32(self) -> u32x8<T> {
        u32x8::from_repr_unchecked(self.1, self.1.f32x8_relaxed_to_u32(self.0))
    }
}

impl<T: RelaxedSimd> f64x2<T> {
    /// Lane-wise relaxed minimum (`f64x2.relaxed_min`).
    #[inline(always)]
    pub fn relaxed_min(self, other: Self) -> Self {
        Self(self.1.f64x2_relaxed_min(self.0, other.0), self.1)
    }

    /// Lane-wise relaxed maximum (`f64x2.relaxed_max`).
    #[inline(always)]
    pub fn relaxed_max(self, other: Self) -> Self {
        Self(self.1.f64x2_relaxed_max(self.0, other.0), self.1)
    }
}

impl<T: RelaxedSimd> f64x4<T> {
    /// Two-register lane-wise relaxed minimum (`2x f64x2.relaxed_min`).
    #[inline(always)]
    pub fn relaxed_min(self, other: Self) -> Self {
        Self(self.1.f64x4_relaxed_min(self.0, other.0), self.1)
    }

    /// Two-register lane-wise relaxed maximum (`2x f64x2.relaxed_max`).
    #[inline(always)]
    pub fn relaxed_max(self, other: Self) -> Self {
        Self(self.1.f64x4_relaxed_max(self.0, other.0), self.1)
    }
}

impl<T: RelaxedSimd> i16x8<T> {
    /// Relaxed Q15 rounding multiply (`i16x8.relaxed_q15mulr_s`).
    #[inline(always)]
    pub fn relaxed_mul_hrs_q15(self, other: Self) -> Self {
        Self(self.1.i16x8_relaxed_mul_hrs_q15(self.0, other.0), self.1)
    }

    /// Relaxed pairwise `i8 * i7 -> i16` dot product (`i16x8.relaxed_dot_i8x16_i7x16_s`).
    ///
    /// Bytes of `b` must be in `0..=127` (`i7`).
    #[inline(always)]
    pub fn relaxed_dot_i8_i7_t(token: T, a: i8x16<T>, b: i8x16<T>) -> Self {
        Self(token.i16x8_relaxed_dot_i8_i7(a.0, b.0), token)
    }
}

impl<T: RelaxedSimd> i16x16<T> {
    /// Two-register relaxed Q15 rounding multiply (`2x i16x8.relaxed_q15mulr_s`).
    #[inline(always)]
    pub fn relaxed_mul_hrs_q15(self, other: Self) -> Self {
        Self(self.1.i16x16_relaxed_mul_hrs_q15(self.0, other.0), self.1)
    }

    /// Two-register relaxed pairwise `i8 * i7 -> i16` dot product (`2x i16x8.relaxed_dot_i8x16_i7x16_s`).
    ///
    /// Bytes of `b` must be in `0..=127` (`i7`).
    #[inline(always)]
    pub fn relaxed_dot_i8_i7_t(token: T, a: i8x32<T>, b: i8x32<T>) -> Self {
        Self(token.i16x16_relaxed_dot_i8_i7(a.0, b.0), token)
    }
}

impl<T: RelaxedSimd> i32x4<T> {
    /// Relaxed 4-way `i8 * i7 -> i32` dot-product accumulate (`i32x4.relaxed_dot_i8x16_i7x16_add`).
    ///
    /// Bytes of `b` must be in `0..=127` (`i7`).
    #[inline(always)]
    pub fn relaxed_dot_i8_i7_add(self, a: i8x16<T>, b: i8x16<T>) -> Self {
        Self(self.1.i32x4_relaxed_dot_i8_i7_add(self.0, a.0, b.0), self.1)
    }
}

impl<T: RelaxedSimd> i32x8<T> {
    /// Two-register relaxed 4-way `i8 * i7 -> i32` dot-product accumulate (`2x i32x4.relaxed_dot_i8x16_i7x16_add`).
    ///
    /// Bytes of `b` must be in `0..=127` (`i7`).
    #[inline(always)]
    pub fn relaxed_dot_i8_i7_add(self, a: i8x32<T>, b: i8x32<T>) -> Self {
        Self(self.1.i32x8_relaxed_dot_i8_i7_add(self.0, a.0, b.0), self.1)
    }
}

// ============================================================================
// x86_64 implementations
// ============================================================================

impl<T: crate::simd::backends::sealed::Sealed> cap_sealed::Sealed for T {}

#[cfg(target_arch = "x86_64")]
mod x86_impls {
    use super::{DotU8I8, Int52Ifma, cap_sealed};
    use crate::simd::backends::{
        I8x16Backend, I8x32Backend, I16x8Backend, I16x16Backend, I32x4Backend, I32x8Backend,
        U8x16Backend, U8x32Backend, U64x2Backend, U64x4Backend,
    };
    #[cfg(feature = "w512")]
    use crate::simd::backends::{
        I8x64Backend, I16x32Backend, I32x16Backend, U8x64Backend, U64x8Backend,
    };
    #[cfg(feature = "avx512")]
    use archmage::caps::No;
    use archmage::caps::{self, Flag, X64CapSet, Yes};
    use core::arch::x86_64::{
        __m128i, __m256i, _mm_dpbusd_avx_epi32, _mm_dpbusds_avx_epi32, _mm_dpwssd_avx_epi32,
        _mm_dpwssds_avx_epi32, _mm_madd52hi_avx_epu64, _mm_madd52lo_avx_epu64,
        _mm256_dpbusd_avx_epi32, _mm256_dpbusds_avx_epi32, _mm256_dpwssd_avx_epi32,
        _mm256_dpwssds_avx_epi32, _mm256_madd52hi_avx_epu64, _mm256_madd52lo_avx_epu64,
    };
    #[cfg(feature = "avx512")]
    use core::arch::x86_64::{
        __m512i, _mm_dpbusd_epi32, _mm_dpbusds_epi32, _mm_dpwssd_epi32, _mm_dpwssds_epi32,
        _mm_madd52hi_epu64, _mm_madd52lo_epu64, _mm256_dpbusd_epi32, _mm256_dpbusds_epi32,
        _mm256_dpwssd_epi32, _mm256_dpwssds_epi32, _mm256_madd52hi_epu64, _mm256_madd52lo_epu64,
    };
    #[cfg(all(feature = "avx512", feature = "w512"))]
    use core::arch::x86_64::{
        _mm512_castsi256_si512, _mm512_castsi512_si256, _mm512_dpbusd_epi32, _mm512_dpbusds_epi32,
        _mm512_dpwssd_epi32, _mm512_dpwssds_epi32, _mm512_extracti64x4_epi64, _mm512_inserti64x4,
        _mm512_madd52hi_epu64, _mm512_madd52lo_epu64,
    };

    #[archmage::arcane]
    fn vex_dot_u8_i8_x4(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxVnniToken>,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_dpbusd_avx_epi32(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_dot_u8_i8_sat_x4(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxVnniToken>,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_dpbusds_avx_epi32(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_dot_i16_x4(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxVnniToken>,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_dpwssd_avx_epi32(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_dot_i16_sat_x4(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxVnniToken>,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_dpwssds_avx_epi32(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_dot_u8_i8_x8(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxVnniToken>,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_dpbusd_avx_epi32(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_dot_u8_i8_sat_x8(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxVnniToken>,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_dpbusds_avx_epi32(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_dot_i16_x8(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxVnniToken>,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_dpwssd_avx_epi32(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_dot_i16_sat_x8(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxVnniToken>,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_dpwssds_avx_epi32(acc, a, b)
    }

    #[archmage::arcane]
    fn vex_madd52_lo_x2(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxIfmaToken>,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_madd52lo_avx_epu64(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_madd52_hi_x2(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxIfmaToken>,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_madd52hi_avx_epu64(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_madd52_lo_x4(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxIfmaToken>,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_madd52lo_avx_epu64(acc, a, b)
    }
    #[archmage::arcane]
    fn vex_madd52_hi_x4(
        _t: archmage::And<archmage::X64V3Token, archmage::X64AvxIfmaToken>,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_madd52hi_avx_epu64(acc, a, b)
    }

    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_dot_u8_i8_x4(
        _t: archmage::X64V4xToken,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_dpbusd_epi32(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_dot_u8_i8_sat_x4(
        _t: archmage::X64V4xToken,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_dpbusds_epi32(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_dot_i16_x4(_t: archmage::X64V4xToken, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
        _mm_dpwssd_epi32(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_dot_i16_sat_x4(
        _t: archmage::X64V4xToken,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_dpwssds_epi32(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_dot_u8_i8_x8(
        _t: archmage::X64V4xToken,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_dpbusd_epi32(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_dot_u8_i8_sat_x8(
        _t: archmage::X64V4xToken,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_dpbusds_epi32(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_dot_i16_x8(_t: archmage::X64V4xToken, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
        _mm256_dpwssd_epi32(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_dot_i16_sat_x8(
        _t: archmage::X64V4xToken,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_dpwssds_epi32(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_madd52_lo_x2(
        _t: archmage::X64V4xToken,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_madd52lo_epu64(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_madd52_hi_x2(
        _t: archmage::X64V4xToken,
        acc: __m128i,
        a: __m128i,
        b: __m128i,
    ) -> __m128i {
        _mm_madd52hi_epu64(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_madd52_lo_x4(
        _t: archmage::X64V4xToken,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_madd52lo_epu64(acc, a, b)
    }
    #[cfg(feature = "avx512")]
    #[archmage::arcane]
    fn evex_madd52_hi_x4(
        _t: archmage::X64V4xToken,
        acc: __m256i,
        a: __m256i,
        b: __m256i,
    ) -> __m256i {
        _mm256_madd52hi_epu64(acc, a, b)
    }

    #[cfg(all(feature = "avx512", feature = "w512"))]
    #[archmage::arcane]
    fn v4_split_m512i(_t: archmage::X64V4Token, v: __m512i) -> [__m256i; 2] {
        [_mm512_castsi512_si256(v), _mm512_extracti64x4_epi64::<1>(v)]
    }
    #[cfg(all(feature = "avx512", feature = "w512"))]
    #[archmage::arcane]
    fn v4_join_m512i(_t: archmage::X64V4Token, lo: __m256i, hi: __m256i) -> __m512i {
        _mm512_inserti64x4::<1>(_mm512_castsi256_si512(lo), hi)
    }
    #[cfg(all(feature = "avx512", feature = "w512"))]
    #[archmage::arcane]
    fn evex_dot_u8_i8_x16(
        _t: archmage::X64V4xToken,
        acc: __m512i,
        a: __m512i,
        b: __m512i,
    ) -> __m512i {
        _mm512_dpbusd_epi32(acc, a, b)
    }
    #[cfg(all(feature = "avx512", feature = "w512"))]
    #[archmage::arcane]
    fn evex_dot_u8_i8_sat_x16(
        _t: archmage::X64V4xToken,
        acc: __m512i,
        a: __m512i,
        b: __m512i,
    ) -> __m512i {
        _mm512_dpbusds_epi32(acc, a, b)
    }
    #[cfg(all(feature = "avx512", feature = "w512"))]
    #[archmage::arcane]
    fn evex_dot_i16_x16(
        _t: archmage::X64V4xToken,
        acc: __m512i,
        a: __m512i,
        b: __m512i,
    ) -> __m512i {
        _mm512_dpwssd_epi32(acc, a, b)
    }
    #[cfg(all(feature = "avx512", feature = "w512"))]
    #[archmage::arcane]
    fn evex_dot_i16_sat_x16(
        _t: archmage::X64V4xToken,
        acc: __m512i,
        a: __m512i,
        b: __m512i,
    ) -> __m512i {
        _mm512_dpwssds_epi32(acc, a, b)
    }
    #[cfg(all(feature = "avx512", feature = "w512"))]
    #[archmage::arcane]
    fn evex_madd52_lo_x8(
        _t: archmage::X64V4xToken,
        acc: __m512i,
        a: __m512i,
        b: __m512i,
    ) -> __m512i {
        _mm512_madd52lo_epu64(acc, a, b)
    }
    #[cfg(all(feature = "avx512", feature = "w512"))]
    #[archmage::arcane]
    fn evex_madd52_hi_x8(
        _t: archmage::X64V4xToken,
        acc: __m512i,
        a: __m512i,
        b: __m512i,
    ) -> __m512i {
        _mm512_madd52hi_epu64(acc, a, b)
    }

    trait DotU8I8FromCaps<T>
    where
        T: I32x4Backend
            + U8x16Backend
            + I8x16Backend
            + I16x8Backend
            + I32x8Backend
            + U8x32Backend
            + I8x32Backend
            + I16x16Backend,
    {
        fn dot_u8_i8_x4(
            proof: T,
            acc: <T as I32x4Backend>::Repr,
            a: <T as U8x16Backend>::Repr,
            b: <T as I8x16Backend>::Repr,
        ) -> <T as I32x4Backend>::Repr;
        fn dot_u8_i8_sat_x4(
            proof: T,
            acc: <T as I32x4Backend>::Repr,
            a: <T as U8x16Backend>::Repr,
            b: <T as I8x16Backend>::Repr,
        ) -> <T as I32x4Backend>::Repr;
        fn dot_i16_x4(
            proof: T,
            acc: <T as I32x4Backend>::Repr,
            a: <T as I16x8Backend>::Repr,
            b: <T as I16x8Backend>::Repr,
        ) -> <T as I32x4Backend>::Repr;
        fn dot_i16_sat_x4(
            proof: T,
            acc: <T as I32x4Backend>::Repr,
            a: <T as I16x8Backend>::Repr,
            b: <T as I16x8Backend>::Repr,
        ) -> <T as I32x4Backend>::Repr;
        fn dot_u8_i8_x8(
            proof: T,
            acc: <T as I32x8Backend>::Repr,
            a: <T as U8x32Backend>::Repr,
            b: <T as I8x32Backend>::Repr,
        ) -> <T as I32x8Backend>::Repr;
        fn dot_u8_i8_sat_x8(
            proof: T,
            acc: <T as I32x8Backend>::Repr,
            a: <T as U8x32Backend>::Repr,
            b: <T as I8x32Backend>::Repr,
        ) -> <T as I32x8Backend>::Repr;
        fn dot_i16_x8(
            proof: T,
            acc: <T as I32x8Backend>::Repr,
            a: <T as I16x16Backend>::Repr,
            b: <T as I16x16Backend>::Repr,
        ) -> <T as I32x8Backend>::Repr;
        fn dot_i16_sat_x8(
            proof: T,
            acc: <T as I32x8Backend>::Repr,
            a: <T as I16x16Backend>::Repr,
            b: <T as I16x16Backend>::Repr,
        ) -> <T as I32x8Backend>::Repr;
    }

    #[cfg(feature = "w512")]
    trait DotU8I8W512FromCaps<T>
    where
        T: I32x16Backend + U8x64Backend + I8x64Backend + I16x32Backend,
    {
        fn dot_u8_i8_x16(
            proof: T,
            acc: <T as I32x16Backend>::Repr,
            a: <T as U8x64Backend>::Repr,
            b: <T as I8x64Backend>::Repr,
        ) -> <T as I32x16Backend>::Repr;
        fn dot_u8_i8_sat_x16(
            proof: T,
            acc: <T as I32x16Backend>::Repr,
            a: <T as U8x64Backend>::Repr,
            b: <T as I8x64Backend>::Repr,
        ) -> <T as I32x16Backend>::Repr;
        fn dot_i16_x16(
            proof: T,
            acc: <T as I32x16Backend>::Repr,
            a: <T as I16x32Backend>::Repr,
            b: <T as I16x32Backend>::Repr,
        ) -> <T as I32x16Backend>::Repr;
        fn dot_i16_sat_x16(
            proof: T,
            acc: <T as I32x16Backend>::Repr,
            a: <T as I16x32Backend>::Repr,
            b: <T as I16x32Backend>::Repr,
        ) -> <T as I32x16Backend>::Repr;
    }

    trait Int52IfmaFromCaps<T>
    where
        T: U64x2Backend + U64x4Backend,
    {
        fn madd52_lo_x2(
            proof: T,
            acc: <T as U64x2Backend>::Repr,
            a: <T as U64x2Backend>::Repr,
            b: <T as U64x2Backend>::Repr,
        ) -> <T as U64x2Backend>::Repr;
        fn madd52_hi_x2(
            proof: T,
            acc: <T as U64x2Backend>::Repr,
            a: <T as U64x2Backend>::Repr,
            b: <T as U64x2Backend>::Repr,
        ) -> <T as U64x2Backend>::Repr;
        fn madd52_lo_x4(
            proof: T,
            acc: <T as U64x4Backend>::Repr,
            a: <T as U64x4Backend>::Repr,
            b: <T as U64x4Backend>::Repr,
        ) -> <T as U64x4Backend>::Repr;
        fn madd52_hi_x4(
            proof: T,
            acc: <T as U64x4Backend>::Repr,
            a: <T as U64x4Backend>::Repr,
            b: <T as U64x4Backend>::Repr,
        ) -> <T as U64x4Backend>::Repr;
    }

    #[cfg(feature = "w512")]
    trait Int52IfmaW512FromCaps<T>
    where
        T: U64x8Backend,
    {
        fn madd52_lo_x8(
            proof: T,
            acc: <T as U64x8Backend>::Repr,
            a: <T as U64x8Backend>::Repr,
            b: <T as U64x8Backend>::Repr,
        ) -> <T as U64x8Backend>::Repr;
        fn madd52_hi_x8(
            proof: T,
            acc: <T as U64x8Backend>::Repr,
            a: <T as U64x8Backend>::Repr,
            b: <T as U64x8Backend>::Repr,
        ) -> <T as U64x8Backend>::Repr;
    }

    impl<T> cap_sealed::DotU8I8Ops for T
    where
        T: cap_sealed::Sealed
            + archmage::SimdToken
            + I32x4Backend
            + U8x16Backend
            + I8x16Backend
            + I16x8Backend
            + I32x8Backend
            + U8x32Backend
            + I8x32Backend
            + I16x16Backend,
        <T as archmage::SimdToken>::__Caps: DotU8I8FromCaps<T>,
    {
        #[inline(always)]
        fn dot_u8_i8_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as U8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8FromCaps<T>>::dot_u8_i8_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_u8_i8_sat_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as U8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8FromCaps<T>>::dot_u8_i8_sat_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_i16_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8FromCaps<T>>::dot_i16_x4(self, acc, a, b)
        }
        #[inline(always)]
        fn dot_i16_sat_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8FromCaps<T>>::dot_i16_sat_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_u8_i8_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as U8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8FromCaps<T>>::dot_u8_i8_x8(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_u8_i8_sat_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as U8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8FromCaps<T>>::dot_u8_i8_sat_x8(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_i16_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8FromCaps<T>>::dot_i16_x8(self, acc, a, b)
        }
        #[inline(always)]
        fn dot_i16_sat_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8FromCaps<T>>::dot_i16_sat_x8(
                self, acc, a, b,
            )
        }
    }

    #[cfg(not(feature = "w512"))]
    impl<T: cap_sealed::DotU8I8Ops> cap_sealed::DotU8I8W512 for T {}

    #[cfg(feature = "w512")]
    impl<T> cap_sealed::DotU8I8W512 for T
    where
        T: cap_sealed::Sealed
            + archmage::SimdToken
            + I32x16Backend
            + U8x64Backend
            + I8x64Backend
            + I16x32Backend,
        <T as archmage::SimdToken>::__Caps: DotU8I8W512FromCaps<T>,
    {
        #[inline(always)]
        fn dot_u8_i8_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as U8x64Backend>::Repr,
            b: <Self as I8x64Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8W512FromCaps<T>>::dot_u8_i8_x16(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_u8_i8_sat_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as U8x64Backend>::Repr,
            b: <Self as I8x64Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8W512FromCaps<T>>::dot_u8_i8_sat_x16(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_i16_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as I16x32Backend>::Repr,
            b: <Self as I16x32Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8W512FromCaps<T>>::dot_i16_x16(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_i16_sat_x16(
            self,
            acc: <Self as I32x16Backend>::Repr,
            a: <Self as I16x32Backend>::Repr,
            b: <Self as I16x32Backend>::Repr,
        ) -> <Self as I32x16Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8I8W512FromCaps<T>>::dot_i16_sat_x16(
                self, acc, a, b,
            )
        }
    }

    impl<T> cap_sealed::Int52IfmaOps for T
    where
        T: cap_sealed::Sealed + archmage::SimdToken + U64x2Backend + U64x4Backend,
        <T as archmage::SimdToken>::__Caps: Int52IfmaFromCaps<T>,
    {
        #[inline(always)]
        fn madd52_lo_x2(
            self,
            acc: <Self as U64x2Backend>::Repr,
            a: <Self as U64x2Backend>::Repr,
            b: <Self as U64x2Backend>::Repr,
        ) -> <Self as U64x2Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as Int52IfmaFromCaps<T>>::madd52_lo_x2(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn madd52_hi_x2(
            self,
            acc: <Self as U64x2Backend>::Repr,
            a: <Self as U64x2Backend>::Repr,
            b: <Self as U64x2Backend>::Repr,
        ) -> <Self as U64x2Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as Int52IfmaFromCaps<T>>::madd52_hi_x2(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn madd52_lo_x4(
            self,
            acc: <Self as U64x4Backend>::Repr,
            a: <Self as U64x4Backend>::Repr,
            b: <Self as U64x4Backend>::Repr,
        ) -> <Self as U64x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as Int52IfmaFromCaps<T>>::madd52_lo_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn madd52_hi_x4(
            self,
            acc: <Self as U64x4Backend>::Repr,
            a: <Self as U64x4Backend>::Repr,
            b: <Self as U64x4Backend>::Repr,
        ) -> <Self as U64x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as Int52IfmaFromCaps<T>>::madd52_hi_x4(
                self, acc, a, b,
            )
        }
    }

    #[cfg(not(feature = "w512"))]
    impl<T: cap_sealed::Int52IfmaOps> cap_sealed::Int52IfmaW512 for T {}

    #[cfg(feature = "w512")]
    impl<T> cap_sealed::Int52IfmaW512 for T
    where
        T: cap_sealed::Sealed + archmage::SimdToken + U64x8Backend,
        <T as archmage::SimdToken>::__Caps: Int52IfmaW512FromCaps<T>,
    {
        #[inline(always)]
        fn madd52_lo_x8(
            self,
            acc: <Self as U64x8Backend>::Repr,
            a: <Self as U64x8Backend>::Repr,
            b: <Self as U64x8Backend>::Repr,
        ) -> <Self as U64x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as Int52IfmaW512FromCaps<T>>::madd52_lo_x8(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn madd52_hi_x8(
            self,
            acc: <Self as U64x8Backend>::Repr,
            a: <Self as U64x8Backend>::Repr,
            b: <Self as U64x8Backend>::Repr,
        ) -> <Self as U64x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as Int52IfmaW512FromCaps<T>>::madd52_hi_x8(
                self, acc, a, b,
            )
        }
    }

    impl<T> DotU8I8 for T where
        T: cap_sealed::Sealed
            + cap_sealed::DotU8I8Ops
            + cap_sealed::DotU8I8W512
            + I32x4Backend
            + U8x16Backend
            + I8x16Backend
            + I16x8Backend
            + I32x8Backend
            + U8x32Backend
            + I8x32Backend
            + I16x16Backend
            + archmage::SimdToken
            + Copy
            + 'static
    {
    }

    impl<T> Int52Ifma for T where
        T: cap_sealed::Sealed
            + cap_sealed::Int52IfmaOps
            + cap_sealed::Int52IfmaW512
            + U64x2Backend
            + U64x4Backend
            + archmage::SimdToken
            + Copy
            + 'static
    {
    }

    macro_rules! impl_dot_u8_i8_vex_ops {
        ($($p:ident),* => $cap:ty) => {
            #[allow(clippy::type_complexity)]
            impl<T, $($p: Flag),*> DotU8I8FromCaps<T> for $cap
            where
                T: archmage::SimdToken<__Caps = Self>
                    + I32x4Backend<Repr = __m128i>
                    + U8x16Backend<Repr = __m128i>
                    + I8x16Backend<Repr = __m128i>
                    + I16x8Backend<Repr = __m128i>
                    + I32x8Backend<Repr = __m256i>
                    + U8x32Backend<Repr = __m256i>
                    + I8x32Backend<Repr = __m256i>
                    + I16x16Backend<Repr = __m256i>,
            {
                #[inline(always)]
                fn dot_u8_i8_x4(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                    vex_dot_u8_i8_x4(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn dot_u8_i8_sat_x4(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                    vex_dot_u8_i8_sat_x4(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn dot_i16_x4(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                    vex_dot_i16_x4(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn dot_i16_sat_x4(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                    vex_dot_i16_sat_x4(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn dot_u8_i8_x8(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                    vex_dot_u8_i8_x8(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn dot_u8_i8_sat_x8(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                    vex_dot_u8_i8_sat_x8(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn dot_i16_x8(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                    vex_dot_i16_x8(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn dot_i16_sat_x8(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                    vex_dot_i16_sat_x8(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof)),
                        acc, a, b,
                    )
                }
            }
        };
    }

    #[cfg(feature = "w512")]
    macro_rules! impl_dot_u8_i8_w512_array {
        ($($p:ident),* => $cap:ty) => {
            #[allow(clippy::type_complexity)]
            impl<T, $($p: Flag),*> DotU8I8W512FromCaps<T> for $cap
            where
                T: archmage::SimdToken<__Caps = Self>
                    + I32x16Backend<Repr = [__m256i; 2]>
                    + U8x64Backend<Repr = [__m256i; 2]>
                    + I8x64Backend<Repr = [__m256i; 2]>
                    + I16x32Backend<Repr = [__m256i; 2]>,
            {
                #[inline(always)]
                fn dot_u8_i8_x16(proof: T, acc: [__m256i; 2], a: [__m256i; 2], b: [__m256i; 2]) -> [__m256i; 2] {
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof));
                    [vex_dot_u8_i8_x8(t, acc[0], a[0], b[0]), vex_dot_u8_i8_x8(t, acc[1], a[1], b[1])]
                }
                #[inline(always)]
                fn dot_u8_i8_sat_x16(proof: T, acc: [__m256i; 2], a: [__m256i; 2], b: [__m256i; 2]) -> [__m256i; 2] {
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof));
                    [vex_dot_u8_i8_sat_x8(t, acc[0], a[0], b[0]), vex_dot_u8_i8_sat_x8(t, acc[1], a[1], b[1])]
                }
                #[inline(always)]
                fn dot_i16_x16(proof: T, acc: [__m256i; 2], a: [__m256i; 2], b: [__m256i; 2]) -> [__m256i; 2] {
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof));
                    [vex_dot_i16_x8(t, acc[0], a[0], b[0]), vex_dot_i16_x8(t, acc[1], a[1], b[1])]
                }
                #[inline(always)]
                fn dot_i16_sat_x16(proof: T, acc: [__m256i; 2], a: [__m256i; 2], b: [__m256i; 2]) -> [__m256i; 2] {
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof));
                    [vex_dot_i16_sat_x8(t, acc[0], a[0], b[0]), vex_dot_i16_sat_x8(t, acc[1], a[1], b[1])]
                }
            }
        };
    }

    #[cfg(not(feature = "w512"))]
    macro_rules! impl_dot_u8_i8_w512_array {
        ($($p:ident),* => $cap:ty) => {};
    }

    #[cfg(all(feature = "avx512", feature = "w512"))]
    macro_rules! impl_dot_u8_i8_w512_split {
        ($($p:ident),* => $cap:ty) => {
            #[allow(clippy::type_complexity)]
            impl<T, $($p: Flag),*> DotU8I8W512FromCaps<T> for $cap
            where
                T: archmage::SimdToken<__Caps = Self>
                    + I32x16Backend<Repr = __m512i>
                    + U8x64Backend<Repr = __m512i>
                    + I8x64Backend<Repr = __m512i>
                    + I16x32Backend<Repr = __m512i>,
            {
                #[inline(always)]
                fn dot_u8_i8_x16(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                    let v4 = caps::extract_v4_caps(proof);
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof));
                    let acc2 = v4_split_m512i(v4, acc);
                    let a2 = v4_split_m512i(v4, a);
                    let b2 = v4_split_m512i(v4, b);
                    v4_join_m512i(
                        v4,
                        vex_dot_u8_i8_x8(t, acc2[0], a2[0], b2[0]),
                        vex_dot_u8_i8_x8(t, acc2[1], a2[1], b2[1]),
                    )
                }
                #[inline(always)]
                fn dot_u8_i8_sat_x16(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                    let v4 = caps::extract_v4_caps(proof);
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof));
                    let acc2 = v4_split_m512i(v4, acc);
                    let a2 = v4_split_m512i(v4, a);
                    let b2 = v4_split_m512i(v4, b);
                    v4_join_m512i(
                        v4,
                        vex_dot_u8_i8_sat_x8(t, acc2[0], a2[0], b2[0]),
                        vex_dot_u8_i8_sat_x8(t, acc2[1], a2[1], b2[1]),
                    )
                }
                #[inline(always)]
                fn dot_i16_x16(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                    let v4 = caps::extract_v4_caps(proof);
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof));
                    let acc2 = v4_split_m512i(v4, acc);
                    let a2 = v4_split_m512i(v4, a);
                    let b2 = v4_split_m512i(v4, b);
                    v4_join_m512i(
                        v4,
                        vex_dot_i16_x8(t, acc2[0], a2[0], b2[0]),
                        vex_dot_i16_x8(t, acc2[1], a2[1], b2[1]),
                    )
                }
                #[inline(always)]
                fn dot_i16_sat_x16(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                    let v4 = caps::extract_v4_caps(proof);
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_vnni_caps(proof));
                    let acc2 = v4_split_m512i(v4, acc);
                    let a2 = v4_split_m512i(v4, a);
                    let b2 = v4_split_m512i(v4, b);
                    v4_join_m512i(
                        v4,
                        vex_dot_i16_sat_x8(t, acc2[0], a2[0], b2[0]),
                        vex_dot_i16_sat_x8(t, acc2[1], a2[1], b2[1]),
                    )
                }
            }
        };
    }

    #[cfg(all(feature = "avx512", not(feature = "w512")))]
    macro_rules! impl_dot_u8_i8_w512_split {
        ($($p:ident),* => $cap:ty) => {};
    }

    macro_rules! impl_int52_ifma_vex_ops {
        ($($p:ident),* => $cap:ty) => {
            #[allow(clippy::type_complexity)]
            impl<T, $($p: Flag),*> Int52IfmaFromCaps<T> for $cap
            where
                T: archmage::SimdToken<__Caps = Self>
                    + U64x2Backend<Repr = __m128i>
                    + U64x4Backend<Repr = __m256i>,
            {
                #[inline(always)]
                fn madd52_lo_x2(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                    vex_madd52_lo_x2(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_ifma_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn madd52_hi_x2(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                    vex_madd52_hi_x2(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_ifma_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn madd52_lo_x4(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                    vex_madd52_lo_x4(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_ifma_caps(proof)),
                        acc, a, b,
                    )
                }
                #[inline(always)]
                fn madd52_hi_x4(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                    vex_madd52_hi_x4(
                        archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_ifma_caps(proof)),
                        acc, a, b,
                    )
                }
            }
        };
    }

    #[cfg(feature = "w512")]
    macro_rules! impl_int52_ifma_w512_array {
        ($($p:ident),* => $cap:ty) => {
            #[allow(clippy::type_complexity)]
            impl<T, $($p: Flag),*> Int52IfmaW512FromCaps<T> for $cap
            where
                T: archmage::SimdToken<__Caps = Self> + U64x8Backend<Repr = [__m256i; 2]>,
            {
                #[inline(always)]
                fn madd52_lo_x8(proof: T, acc: [__m256i; 2], a: [__m256i; 2], b: [__m256i; 2]) -> [__m256i; 2] {
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_ifma_caps(proof));
                    [vex_madd52_lo_x4(t, acc[0], a[0], b[0]), vex_madd52_lo_x4(t, acc[1], a[1], b[1])]
                }
                #[inline(always)]
                fn madd52_hi_x8(proof: T, acc: [__m256i; 2], a: [__m256i; 2], b: [__m256i; 2]) -> [__m256i; 2] {
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_ifma_caps(proof));
                    [vex_madd52_hi_x4(t, acc[0], a[0], b[0]), vex_madd52_hi_x4(t, acc[1], a[1], b[1])]
                }
            }
        };
    }

    #[cfg(not(feature = "w512"))]
    macro_rules! impl_int52_ifma_w512_array {
        ($($p:ident),* => $cap:ty) => {};
    }

    #[cfg(all(feature = "avx512", feature = "w512"))]
    macro_rules! impl_int52_ifma_w512_split {
        ($($p:ident),* => $cap:ty) => {
            #[allow(clippy::type_complexity)]
            impl<T, $($p: Flag),*> Int52IfmaW512FromCaps<T> for $cap
            where
                T: archmage::SimdToken<__Caps = Self> + U64x8Backend<Repr = __m512i>,
            {
                #[inline(always)]
                fn madd52_lo_x8(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                    let v4 = caps::extract_v4_caps(proof);
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_ifma_caps(proof));
                    let acc2 = v4_split_m512i(v4, acc);
                    let a2 = v4_split_m512i(v4, a);
                    let b2 = v4_split_m512i(v4, b);
                    v4_join_m512i(
                        v4,
                        vex_madd52_lo_x4(t, acc2[0], a2[0], b2[0]),
                        vex_madd52_lo_x4(t, acc2[1], a2[1], b2[1]),
                    )
                }
                #[inline(always)]
                fn madd52_hi_x8(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                    let v4 = caps::extract_v4_caps(proof);
                    let t = archmage::And::join(caps::extract_v3_caps(proof), caps::extract_x64_avx_ifma_caps(proof));
                    let acc2 = v4_split_m512i(v4, acc);
                    let a2 = v4_split_m512i(v4, a);
                    let b2 = v4_split_m512i(v4, b);
                    v4_join_m512i(
                        v4,
                        vex_madd52_hi_x4(t, acc2[0], a2[0], b2[0]),
                        vex_madd52_hi_x4(t, acc2[1], a2[1], b2[1]),
                    )
                }
            }
        };
    }

    #[cfg(all(feature = "avx512", not(feature = "w512")))]
    macro_rules! impl_int52_ifma_w512_split {
        ($($p:ident),* => $cap:ty) => {};
    }

    #[cfg(not(feature = "avx512"))]
    mod no_avx512_x64 {
        use super::*;

        impl_dot_u8_i8_vex_ops!(
            C2, C4, C5, C6, C7, C9, C10, C11, C12, C13, C14, C15, C16, C17
            => X64CapSet<Yes, Yes, C2, Yes, C4, C5, C6, C7, Yes, C9, C10, C11, C12, C13, C14, C15, C16, C17>
        );
        impl_dot_u8_i8_w512_array!(
            C2, C4, C5, C6, C7, C9, C10, C11, C12, C13, C14, C15, C16, C17
            => X64CapSet<Yes, Yes, C2, Yes, C4, C5, C6, C7, Yes, C9, C10, C11, C12, C13, C14, C15, C16, C17>
        );

        impl_int52_ifma_vex_ops!(
            C2, C4, C5, C6, C7, C8, C10, C11, C12, C13, C14, C15, C16, C17
            => X64CapSet<Yes, Yes, C2, Yes, C4, C5, C6, C7, C8, Yes, C10, C11, C12, C13, C14, C15, C16, C17>
        );
        impl_int52_ifma_w512_array!(
            C2, C4, C5, C6, C7, C8, C10, C11, C12, C13, C14, C15, C16, C17
            => X64CapSet<Yes, Yes, C2, Yes, C4, C5, C6, C7, C8, Yes, C10, C11, C12, C13, C14, C15, C16, C17>
        );
    }

    #[cfg(feature = "avx512")]
    mod avx512_x64 {
        use super::*;

        // V3 + AvxVnni (W512 backed by [__m256i; 2])
        impl_dot_u8_i8_vex_ops!(
            C4, C5, C6, C7, C9, C10, C11, C12, C13, C14, C15, C16, C17
            => X64CapSet<Yes, Yes, No, Yes, C4, C5, C6, C7, Yes, C9, C10, C11, C12, C13, C14, C15, C16, C17>
        );
        impl_dot_u8_i8_w512_array!(
            C4, C5, C6, C7, C9, C10, C11, C12, C13, C14, C15, C16, C17
            => X64CapSet<Yes, Yes, No, Yes, C4, C5, C6, C7, Yes, C9, C10, C11, C12, C13, C14, C15, C16, C17>
        );
        impl_dot_u8_i8_vex_ops!(
            C4, C5, C6, C7, C9, C10, C11, C12, C13, C15, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, C5, C6, C7, Yes, C9, C10, C11, C12, C13, No, C15, C16, C17>
        );
        impl_dot_u8_i8_w512_array!(
            C4, C5, C6, C7, C9, C10, C11, C12, C13, C15, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, C5, C6, C7, Yes, C9, C10, C11, C12, C13, No, C15, C16, C17>
        );

        // V4 (without full V4x) + AvxVnni (W512 backed by split/join __m512i)
        impl_dot_u8_i8_vex_ops!(
            C4, C5, C6, C7, C9, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, C5, C6, C7, Yes, C9, C10, C11, C12, C13, Yes, No, C16, C17>
        );
        impl_dot_u8_i8_w512_split!(
            C4, C5, C6, C7, C9, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, C5, C6, C7, Yes, C9, C10, C11, C12, C13, Yes, No, C16, C17>
        );
        impl_dot_u8_i8_vex_ops!(
            C4, C6, C7, C9, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, No, C6, C7, Yes, C9, C10, C11, C12, C13, Yes, Yes, C16, C17>
        );
        impl_dot_u8_i8_w512_split!(
            C4, C6, C7, C9, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, No, C6, C7, Yes, C9, C10, C11, C12, C13, Yes, Yes, C16, C17>
        );
        impl_dot_u8_i8_vex_ops!(
            C6, C7, C9, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, No, Yes, C6, C7, Yes, C9, C10, C11, C12, C13, Yes, Yes, C16, C17>
        );
        impl_dot_u8_i8_w512_split!(
            C6, C7, C9, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, No, Yes, C6, C7, Yes, C9, C10, C11, C12, C13, Yes, Yes, C16, C17>
        );

        // V4x EVEX native
        #[allow(clippy::type_complexity)]
        impl<
            T,
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
        > DotU8I8FromCaps<T>
            for X64CapSet<
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
            >
        where
            T: archmage::SimdToken<__Caps = Self>
                + I32x4Backend<Repr = __m128i>
                + U8x16Backend<Repr = __m128i>
                + I8x16Backend<Repr = __m128i>
                + I16x8Backend<Repr = __m128i>
                + I32x8Backend<Repr = __m256i>
                + U8x32Backend<Repr = __m256i>
                + I8x32Backend<Repr = __m256i>
                + I16x16Backend<Repr = __m256i>,
        {
            #[inline(always)]
            fn dot_u8_i8_x4(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                evex_dot_u8_i8_x4(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_u8_i8_sat_x4(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                evex_dot_u8_i8_sat_x4(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_i16_x4(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                evex_dot_i16_x4(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_i16_sat_x4(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                evex_dot_i16_sat_x4(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_u8_i8_x8(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                evex_dot_u8_i8_x8(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_u8_i8_sat_x8(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                evex_dot_u8_i8_sat_x8(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_i16_x8(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                evex_dot_i16_x8(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_i16_sat_x8(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                evex_dot_i16_sat_x8(caps::extract_v4x_caps(proof), acc, a, b)
            }
        }

        #[cfg(feature = "w512")]
        #[allow(clippy::type_complexity)]
        impl<
            T,
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
        > DotU8I8W512FromCaps<T>
            for X64CapSet<
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
            >
        where
            T: archmage::SimdToken<__Caps = Self>
                + I32x16Backend<Repr = __m512i>
                + U8x64Backend<Repr = __m512i>
                + I8x64Backend<Repr = __m512i>
                + I16x32Backend<Repr = __m512i>,
        {
            #[inline(always)]
            fn dot_u8_i8_x16(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                evex_dot_u8_i8_x16(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_u8_i8_sat_x16(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                evex_dot_u8_i8_sat_x16(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_i16_x16(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                evex_dot_i16_x16(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn dot_i16_sat_x16(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                evex_dot_i16_sat_x16(caps::extract_v4x_caps(proof), acc, a, b)
            }
        }

        // Int52Ifma VEX (W512 backed by [__m256i; 2])
        impl_int52_ifma_vex_ops!(
            C4, C5, C6, C7, C8, C10, C11, C12, C13, C14, C15, C16, C17
            => X64CapSet<Yes, Yes, No, Yes, C4, C5, C6, C7, C8, Yes, C10, C11, C12, C13, C14, C15, C16, C17>
        );
        impl_int52_ifma_w512_array!(
            C4, C5, C6, C7, C8, C10, C11, C12, C13, C14, C15, C16, C17
            => X64CapSet<Yes, Yes, No, Yes, C4, C5, C6, C7, C8, Yes, C10, C11, C12, C13, C14, C15, C16, C17>
        );
        impl_int52_ifma_vex_ops!(
            C4, C5, C6, C7, C8, C10, C11, C12, C13, C15, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, C5, C6, C7, C8, Yes, C10, C11, C12, C13, No, C15, C16, C17>
        );
        impl_int52_ifma_w512_array!(
            C4, C5, C6, C7, C8, C10, C11, C12, C13, C15, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, C5, C6, C7, C8, Yes, C10, C11, C12, C13, No, C15, C16, C17>
        );

        // Int52Ifma V4 (without full V4x) + AvxIfma
        impl_int52_ifma_vex_ops!(
            C4, C5, C6, C7, C8, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, C5, C6, C7, C8, Yes, C10, C11, C12, C13, Yes, No, C16, C17>
        );
        impl_int52_ifma_w512_split!(
            C4, C5, C6, C7, C8, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, C5, C6, C7, C8, Yes, C10, C11, C12, C13, Yes, No, C16, C17>
        );
        impl_int52_ifma_vex_ops!(
            C4, C6, C7, C8, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, No, C6, C7, C8, Yes, C10, C11, C12, C13, Yes, Yes, C16, C17>
        );
        impl_int52_ifma_w512_split!(
            C4, C6, C7, C8, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, C4, No, C6, C7, C8, Yes, C10, C11, C12, C13, Yes, Yes, C16, C17>
        );
        impl_int52_ifma_vex_ops!(
            C6, C7, C8, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, No, Yes, C6, C7, C8, Yes, C10, C11, C12, C13, Yes, Yes, C16, C17>
        );
        impl_int52_ifma_w512_split!(
            C6, C7, C8, C10, C11, C12, C13, C16, C17
            => X64CapSet<Yes, Yes, Yes, Yes, No, Yes, C6, C7, C8, Yes, C10, C11, C12, C13, Yes, Yes, C16, C17>
        );

        // Int52Ifma V4x EVEX native
        #[allow(clippy::type_complexity)]
        impl<
            T,
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
        > Int52IfmaFromCaps<T>
            for X64CapSet<
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
            >
        where
            T: archmage::SimdToken<__Caps = Self>
                + U64x2Backend<Repr = __m128i>
                + U64x4Backend<Repr = __m256i>,
        {
            #[inline(always)]
            fn madd52_lo_x2(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                evex_madd52_lo_x2(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn madd52_hi_x2(proof: T, acc: __m128i, a: __m128i, b: __m128i) -> __m128i {
                evex_madd52_hi_x2(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn madd52_lo_x4(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                evex_madd52_lo_x4(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn madd52_hi_x4(proof: T, acc: __m256i, a: __m256i, b: __m256i) -> __m256i {
                evex_madd52_hi_x4(caps::extract_v4x_caps(proof), acc, a, b)
            }
        }

        #[cfg(feature = "w512")]
        #[allow(clippy::type_complexity)]
        impl<
            T,
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
        > Int52IfmaW512FromCaps<T>
            for X64CapSet<
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
            >
        where
            T: archmage::SimdToken<__Caps = Self> + U64x8Backend<Repr = __m512i>,
        {
            #[inline(always)]
            fn madd52_lo_x8(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                evex_madd52_lo_x8(caps::extract_v4x_caps(proof), acc, a, b)
            }
            #[inline(always)]
            fn madd52_hi_x8(proof: T, acc: __m512i, a: __m512i, b: __m512i) -> __m512i {
                evex_madd52_hi_x8(caps::extract_v4x_caps(proof), acc, a, b)
            }
        }
    }
}

// ============================================================================
// aarch64 implementations
// ============================================================================

#[cfg(target_arch = "aarch64")]
mod arm_impls {
    #[rustversion::before(1.94)]
    use super::super::i32x4;
    use super::{ArmF16Fma, ArmRdma, DotI8Signed, DotU8, cap_sealed};
    use crate::simd::backends::{
        F32x4Backend, F32x8Backend, I8x16Backend, I8x32Backend, I16x8Backend, I16x16Backend,
        I32x4Backend, I32x8Backend, U8x16Backend, U8x32Backend, U16x8Backend, U32x4Backend,
        U32x8Backend,
    };
    #[cfg(feature = "w512")]
    use crate::simd::backends::{
        I8x64Backend, I16x32Backend, I32x16Backend, U8x64Backend, U32x16Backend,
    };
    use archmage::caps::{self, ArmCapSet, Flag, Yes};
    use core::arch::aarch64::{
        float32x4_t, int8x16_t, int16x8_t, int32x4_t, uint8x16_t, uint16x8_t, uint32x4_t,
        vqrdmlahq_s16, vqrdmlshq_s16,
    };

    #[rustversion::since(1.98)]
    #[allow(clippy::incompatible_msrv)]
    #[archmage::arcane]
    fn arm_dot_i8_x4(
        _t: archmage::NeonDotProdToken,
        acc: int32x4_t,
        a: int8x16_t,
        b: int8x16_t,
    ) -> int32x4_t {
        core::arch::aarch64::vdotq_s32(acc, a, b)
    }

    #[rustversion::before(1.98)]
    #[archmage::arcane]
    fn arm_dot_i8_x4(
        _t: archmage::NeonDotProdToken,
        acc: int32x4_t,
        a: int8x16_t,
        b: int8x16_t,
    ) -> int32x4_t {
        use core::arch::aarch64::{
            vaddq_s32, vget_low_s8, vmull_high_s8, vmull_s8, vpaddlq_s16, vpaddq_s32,
        };
        let lo = vpaddlq_s16(vmull_s8(vget_low_s8(a), vget_low_s8(b)));
        let hi = vpaddlq_s16(vmull_high_s8(a, b));
        vaddq_s32(acc, vpaddq_s32(lo, hi))
    }

    #[rustversion::since(1.98)]
    #[allow(clippy::incompatible_msrv)]
    #[archmage::arcane]
    fn arm_dot_u8_x4(
        _t: archmage::NeonDotProdToken,
        acc: uint32x4_t,
        a: uint8x16_t,
        b: uint8x16_t,
    ) -> uint32x4_t {
        core::arch::aarch64::vdotq_u32(acc, a, b)
    }

    #[rustversion::before(1.98)]
    #[archmage::arcane]
    fn arm_dot_u8_x4(
        _t: archmage::NeonDotProdToken,
        acc: uint32x4_t,
        a: uint8x16_t,
        b: uint8x16_t,
    ) -> uint32x4_t {
        use core::arch::aarch64::{
            vaddq_u32, vget_low_u8, vmull_high_u8, vmull_u8, vpaddlq_u16, vpaddq_u32,
        };
        let lo = vpaddlq_u16(vmull_u8(vget_low_u8(a), vget_low_u8(b)));
        let hi = vpaddlq_u16(vmull_high_u8(a, b));
        vaddq_u32(acc, vpaddq_u32(lo, hi))
    }

    #[archmage::arcane]
    fn arm_qrdmlah_x8(
        _t: archmage::NeonRdmToken,
        acc: int16x8_t,
        a: int16x8_t,
        b: int16x8_t,
    ) -> int16x8_t {
        vqrdmlahq_s16(acc, a, b)
    }

    #[archmage::arcane]
    fn arm_qrdmlsh_x8(
        _t: archmage::NeonRdmToken,
        acc: int16x8_t,
        a: int16x8_t,
        b: int16x8_t,
    ) -> int16x8_t {
        vqrdmlshq_s16(acc, a, b)
    }

    #[rustversion::since(1.94)]
    #[allow(clippy::incompatible_msrv)]
    #[archmage::arcane]
    fn arm_fmlal_low_x4(
        _t: archmage::NeonFhmToken,
        acc: float32x4_t,
        a: uint16x8_t,
        b: uint16x8_t,
    ) -> float32x4_t {
        use core::arch::aarch64::{vfmlalq_low_f16, vreinterpretq_f16_u16};
        vfmlalq_low_f16(acc, vreinterpretq_f16_u16(a), vreinterpretq_f16_u16(b))
    }

    #[rustversion::before(1.94)]
    #[archmage::arcane]
    fn arm_fmlal_low_x4(
        token: archmage::NeonFhmToken,
        acc: float32x4_t,
        a: uint16x8_t,
        b: uint16x8_t,
    ) -> float32x4_t {
        let aa = crate::simd::generic::u16x8::from_repr_unchecked(token, a).to_array();
        let bb = crate::simd::generic::u16x8::from_repr_unchecked(token, b).to_array();
        let af = i32x4::from_array_t(
            token,
            [aa[0] as i32, aa[1] as i32, aa[2] as i32, aa[3] as i32],
        )
        .f16_to_f32();
        let bf = i32x4::from_array_t(
            token,
            [bb[0] as i32, bb[1] as i32, bb[2] as i32, bb[3] as i32],
        )
        .f16_to_f32();
        core::arch::aarch64::vfmaq_f32(acc, af.0, bf.0)
    }

    #[rustversion::since(1.94)]
    #[allow(clippy::incompatible_msrv)]
    #[archmage::arcane]
    fn arm_fmlal_high_x4(
        _t: archmage::NeonFhmToken,
        acc: float32x4_t,
        a: uint16x8_t,
        b: uint16x8_t,
    ) -> float32x4_t {
        use core::arch::aarch64::{vfmlalq_high_f16, vreinterpretq_f16_u16};
        vfmlalq_high_f16(acc, vreinterpretq_f16_u16(a), vreinterpretq_f16_u16(b))
    }

    #[rustversion::before(1.94)]
    #[archmage::arcane]
    fn arm_fmlal_high_x4(
        token: archmage::NeonFhmToken,
        acc: float32x4_t,
        a: uint16x8_t,
        b: uint16x8_t,
    ) -> float32x4_t {
        let aa = crate::simd::generic::u16x8::from_repr_unchecked(token, a).to_array();
        let bb = crate::simd::generic::u16x8::from_repr_unchecked(token, b).to_array();
        let af = i32x4::from_array_t(
            token,
            [aa[4] as i32, aa[5] as i32, aa[6] as i32, aa[7] as i32],
        )
        .f16_to_f32();
        let bf = i32x4::from_array_t(
            token,
            [bb[4] as i32, bb[5] as i32, bb[6] as i32, bb[7] as i32],
        )
        .f16_to_f32();
        core::arch::aarch64::vfmaq_f32(acc, af.0, bf.0)
    }

    #[rustversion::since(1.94)]
    #[allow(clippy::incompatible_msrv)]
    #[archmage::arcane]
    fn arm_fmlsl_low_x4(
        _t: archmage::NeonFhmToken,
        acc: float32x4_t,
        a: uint16x8_t,
        b: uint16x8_t,
    ) -> float32x4_t {
        use core::arch::aarch64::{vfmlslq_low_f16, vreinterpretq_f16_u16};
        vfmlslq_low_f16(acc, vreinterpretq_f16_u16(a), vreinterpretq_f16_u16(b))
    }

    #[rustversion::before(1.94)]
    #[archmage::arcane]
    fn arm_fmlsl_low_x4(
        token: archmage::NeonFhmToken,
        acc: float32x4_t,
        a: uint16x8_t,
        b: uint16x8_t,
    ) -> float32x4_t {
        let aa = crate::simd::generic::u16x8::from_repr_unchecked(token, a).to_array();
        let bb = crate::simd::generic::u16x8::from_repr_unchecked(token, b).to_array();
        let af = i32x4::from_array_t(
            token,
            [aa[0] as i32, aa[1] as i32, aa[2] as i32, aa[3] as i32],
        )
        .f16_to_f32();
        let bf = i32x4::from_array_t(
            token,
            [bb[0] as i32, bb[1] as i32, bb[2] as i32, bb[3] as i32],
        )
        .f16_to_f32();
        core::arch::aarch64::vfmsq_f32(acc, af.0, bf.0)
    }

    #[rustversion::since(1.94)]
    #[allow(clippy::incompatible_msrv)]
    #[archmage::arcane]
    fn arm_fmlsl_high_x4(
        _t: archmage::NeonFhmToken,
        acc: float32x4_t,
        a: uint16x8_t,
        b: uint16x8_t,
    ) -> float32x4_t {
        use core::arch::aarch64::{vfmlslq_high_f16, vreinterpretq_f16_u16};
        vfmlslq_high_f16(acc, vreinterpretq_f16_u16(a), vreinterpretq_f16_u16(b))
    }

    #[rustversion::before(1.94)]
    #[archmage::arcane]
    fn arm_fmlsl_high_x4(
        token: archmage::NeonFhmToken,
        acc: float32x4_t,
        a: uint16x8_t,
        b: uint16x8_t,
    ) -> float32x4_t {
        let aa = crate::simd::generic::u16x8::from_repr_unchecked(token, a).to_array();
        let bb = crate::simd::generic::u16x8::from_repr_unchecked(token, b).to_array();
        let af = i32x4::from_array_t(
            token,
            [aa[4] as i32, aa[5] as i32, aa[6] as i32, aa[7] as i32],
        )
        .f16_to_f32();
        let bf = i32x4::from_array_t(
            token,
            [bb[4] as i32, bb[5] as i32, bb[6] as i32, bb[7] as i32],
        )
        .f16_to_f32();
        core::arch::aarch64::vfmsq_f32(acc, af.0, bf.0)
    }

    trait DotI8SignedFromCaps<T: I32x4Backend + I8x16Backend + I32x8Backend + I8x32Backend> {
        fn dot_i8_x4(
            proof: T,
            acc: <T as I32x4Backend>::Repr,
            a: <T as I8x16Backend>::Repr,
            b: <T as I8x16Backend>::Repr,
        ) -> <T as I32x4Backend>::Repr;
        fn dot_i8_x8(
            proof: T,
            acc: <T as I32x8Backend>::Repr,
            a: <T as I8x32Backend>::Repr,
            b: <T as I8x32Backend>::Repr,
        ) -> <T as I32x8Backend>::Repr;
    }

    impl<
        T,
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
    > DotI8SignedFromCaps<T> for ArmCapSet<Yes, C1, C2, C3, C4, C5, Yes, C7, C8, C9, C10, C11>
    where
        T: archmage::SimdToken<__Caps = Self>
            + I32x4Backend<Repr = int32x4_t>
            + I8x16Backend<Repr = int8x16_t>
            + I32x8Backend<Repr = [int32x4_t; 2]>
            + I8x32Backend<Repr = [int8x16_t; 2]>,
    {
        #[inline(always)]
        fn dot_i8_x4(proof: T, acc: int32x4_t, a: int8x16_t, b: int8x16_t) -> int32x4_t {
            arm_dot_i8_x4(caps::extract_neon_dotprod_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn dot_i8_x8(
            proof: T,
            acc: [int32x4_t; 2],
            a: [int8x16_t; 2],
            b: [int8x16_t; 2],
        ) -> [int32x4_t; 2] {
            let t = caps::extract_neon_dotprod_caps(proof);
            [
                arm_dot_i8_x4(t, acc[0], a[0], b[0]),
                arm_dot_i8_x4(t, acc[1], a[1], b[1]),
            ]
        }
    }

    impl<T> cap_sealed::DotI8SignedOps for T
    where
        T: cap_sealed::Sealed
            + archmage::SimdToken
            + I32x4Backend
            + I8x16Backend
            + I32x8Backend
            + I8x32Backend,
        <T as archmage::SimdToken>::__Caps: DotI8SignedFromCaps<T>,
    {
        #[inline(always)]
        fn dot_i8_x4(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as I8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotI8SignedFromCaps<T>>::dot_i8_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn dot_i8_x8(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as I8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotI8SignedFromCaps<T>>::dot_i8_x8(
                self, acc, a, b,
            )
        }
    }

    #[cfg(not(feature = "w512"))]
    impl<T: cap_sealed::DotI8SignedOps> cap_sealed::DotI8SignedW512 for T {}

    #[cfg(feature = "w512")]
    impl<T> cap_sealed::DotI8SignedW512 for T
    where
        T: cap_sealed::DotI8SignedOps
            + I32x4Backend<Repr = int32x4_t>
            + I8x16Backend<Repr = int8x16_t>
            + I32x16Backend<Repr = [int32x4_t; 4]>
            + I8x64Backend<Repr = [int8x16_t; 4]>,
    {
        #[inline(always)]
        fn dot_i8_x16(
            self,
            acc: [int32x4_t; 4],
            a: [int8x16_t; 4],
            b: [int8x16_t; 4],
        ) -> [int32x4_t; 4] {
            core::array::from_fn(|i| self.dot_i8_x4(acc[i], a[i], b[i]))
        }
    }

    trait DotU8FromCaps<T: U32x4Backend + U8x16Backend + U32x8Backend + U8x32Backend> {
        fn dot_u8_x4(
            proof: T,
            acc: <T as U32x4Backend>::Repr,
            a: <T as U8x16Backend>::Repr,
            b: <T as U8x16Backend>::Repr,
        ) -> <T as U32x4Backend>::Repr;
        fn dot_u8_x8(
            proof: T,
            acc: <T as U32x8Backend>::Repr,
            a: <T as U8x32Backend>::Repr,
            b: <T as U8x32Backend>::Repr,
        ) -> <T as U32x8Backend>::Repr;
    }

    impl<
        T,
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
    > DotU8FromCaps<T> for ArmCapSet<Yes, C1, C2, C3, C4, C5, Yes, C7, C8, C9, C10, C11>
    where
        T: archmage::SimdToken<__Caps = Self>
            + U32x4Backend<Repr = uint32x4_t>
            + U8x16Backend<Repr = uint8x16_t>
            + U32x8Backend<Repr = [uint32x4_t; 2]>
            + U8x32Backend<Repr = [uint8x16_t; 2]>,
    {
        #[inline(always)]
        fn dot_u8_x4(proof: T, acc: uint32x4_t, a: uint8x16_t, b: uint8x16_t) -> uint32x4_t {
            arm_dot_u8_x4(caps::extract_neon_dotprod_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn dot_u8_x8(
            proof: T,
            acc: [uint32x4_t; 2],
            a: [uint8x16_t; 2],
            b: [uint8x16_t; 2],
        ) -> [uint32x4_t; 2] {
            let t = caps::extract_neon_dotprod_caps(proof);
            [
                arm_dot_u8_x4(t, acc[0], a[0], b[0]),
                arm_dot_u8_x4(t, acc[1], a[1], b[1]),
            ]
        }
    }

    impl<T> cap_sealed::DotU8Ops for T
    where
        T: cap_sealed::Sealed
            + archmage::SimdToken
            + U32x4Backend
            + U8x16Backend
            + U32x8Backend
            + U8x32Backend,
        <T as archmage::SimdToken>::__Caps: DotU8FromCaps<T>,
    {
        #[inline(always)]
        fn dot_u8_x4(
            self,
            acc: <Self as U32x4Backend>::Repr,
            a: <Self as U8x16Backend>::Repr,
            b: <Self as U8x16Backend>::Repr,
        ) -> <Self as U32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8FromCaps<T>>::dot_u8_x4(self, acc, a, b)
        }
        #[inline(always)]
        fn dot_u8_x8(
            self,
            acc: <Self as U32x8Backend>::Repr,
            a: <Self as U8x32Backend>::Repr,
            b: <Self as U8x32Backend>::Repr,
        ) -> <Self as U32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as DotU8FromCaps<T>>::dot_u8_x8(self, acc, a, b)
        }
    }

    #[cfg(not(feature = "w512"))]
    impl<T: cap_sealed::DotU8Ops> cap_sealed::DotU8W512 for T {}

    #[cfg(feature = "w512")]
    impl<T> cap_sealed::DotU8W512 for T
    where
        T: cap_sealed::DotU8Ops
            + U32x4Backend<Repr = uint32x4_t>
            + U8x16Backend<Repr = uint8x16_t>
            + U32x16Backend<Repr = [uint32x4_t; 4]>
            + U8x64Backend<Repr = [uint8x16_t; 4]>,
    {
        #[inline(always)]
        fn dot_u8_x16(
            self,
            acc: [uint32x4_t; 4],
            a: [uint8x16_t; 4],
            b: [uint8x16_t; 4],
        ) -> [uint32x4_t; 4] {
            core::array::from_fn(|i| self.dot_u8_x4(acc[i], a[i], b[i]))
        }
    }

    trait ArmRdmaFromCaps<T: I16x8Backend + I16x16Backend> {
        fn qrdmlah_x8(
            proof: T,
            acc: <T as I16x8Backend>::Repr,
            a: <T as I16x8Backend>::Repr,
            b: <T as I16x8Backend>::Repr,
        ) -> <T as I16x8Backend>::Repr;
        fn qrdmlsh_x8(
            proof: T,
            acc: <T as I16x8Backend>::Repr,
            a: <T as I16x8Backend>::Repr,
            b: <T as I16x8Backend>::Repr,
        ) -> <T as I16x8Backend>::Repr;
        fn qrdmlah_x16(
            proof: T,
            acc: <T as I16x16Backend>::Repr,
            a: <T as I16x16Backend>::Repr,
            b: <T as I16x16Backend>::Repr,
        ) -> <T as I16x16Backend>::Repr;
        fn qrdmlsh_x16(
            proof: T,
            acc: <T as I16x16Backend>::Repr,
            a: <T as I16x16Backend>::Repr,
            b: <T as I16x16Backend>::Repr,
        ) -> <T as I16x16Backend>::Repr;
    }

    impl<
        T,
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
    > ArmRdmaFromCaps<T> for ArmCapSet<Yes, C1, C2, C3, C4, Yes, C6, C7, C8, C9, C10, C11>
    where
        T: archmage::SimdToken<__Caps = Self>
            + I16x8Backend<Repr = int16x8_t>
            + I16x16Backend<Repr = [int16x8_t; 2]>,
    {
        #[inline(always)]
        fn qrdmlah_x8(proof: T, acc: int16x8_t, a: int16x8_t, b: int16x8_t) -> int16x8_t {
            arm_qrdmlah_x8(caps::extract_neon_rdm_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn qrdmlsh_x8(proof: T, acc: int16x8_t, a: int16x8_t, b: int16x8_t) -> int16x8_t {
            arm_qrdmlsh_x8(caps::extract_neon_rdm_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn qrdmlah_x16(
            proof: T,
            acc: [int16x8_t; 2],
            a: [int16x8_t; 2],
            b: [int16x8_t; 2],
        ) -> [int16x8_t; 2] {
            let t = caps::extract_neon_rdm_caps(proof);
            [
                arm_qrdmlah_x8(t, acc[0], a[0], b[0]),
                arm_qrdmlah_x8(t, acc[1], a[1], b[1]),
            ]
        }
        #[inline(always)]
        fn qrdmlsh_x16(
            proof: T,
            acc: [int16x8_t; 2],
            a: [int16x8_t; 2],
            b: [int16x8_t; 2],
        ) -> [int16x8_t; 2] {
            let t = caps::extract_neon_rdm_caps(proof);
            [
                arm_qrdmlsh_x8(t, acc[0], a[0], b[0]),
                arm_qrdmlsh_x8(t, acc[1], a[1], b[1]),
            ]
        }
    }

    impl<T> cap_sealed::ArmRdmaOps for T
    where
        T: cap_sealed::Sealed + archmage::SimdToken + I16x8Backend + I16x16Backend,
        <T as archmage::SimdToken>::__Caps: ArmRdmaFromCaps<T>,
    {
        #[inline(always)]
        fn qrdmlah_x8(
            self,
            acc: <Self as I16x8Backend>::Repr,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I16x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmRdmaFromCaps<T>>::qrdmlah_x8(self, acc, a, b)
        }
        #[inline(always)]
        fn qrdmlsh_x8(
            self,
            acc: <Self as I16x8Backend>::Repr,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I16x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmRdmaFromCaps<T>>::qrdmlsh_x8(self, acc, a, b)
        }
        #[inline(always)]
        fn qrdmlah_x16(
            self,
            acc: <Self as I16x16Backend>::Repr,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I16x16Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmRdmaFromCaps<T>>::qrdmlah_x16(self, acc, a, b)
        }
        #[inline(always)]
        fn qrdmlsh_x16(
            self,
            acc: <Self as I16x16Backend>::Repr,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I16x16Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmRdmaFromCaps<T>>::qrdmlsh_x16(self, acc, a, b)
        }
    }

    #[cfg(not(feature = "w512"))]
    impl<T: cap_sealed::ArmRdmaOps> cap_sealed::ArmRdmaW512 for T {}

    #[cfg(feature = "w512")]
    impl<T> cap_sealed::ArmRdmaW512 for T
    where
        T: cap_sealed::ArmRdmaOps
            + I16x8Backend<Repr = int16x8_t>
            + I16x32Backend<Repr = [int16x8_t; 4]>,
    {
        #[inline(always)]
        fn qrdmlah_x32(
            self,
            acc: [int16x8_t; 4],
            a: [int16x8_t; 4],
            b: [int16x8_t; 4],
        ) -> [int16x8_t; 4] {
            core::array::from_fn(|i| self.qrdmlah_x8(acc[i], a[i], b[i]))
        }
        #[inline(always)]
        fn qrdmlsh_x32(
            self,
            acc: [int16x8_t; 4],
            a: [int16x8_t; 4],
            b: [int16x8_t; 4],
        ) -> [int16x8_t; 4] {
            core::array::from_fn(|i| self.qrdmlsh_x8(acc[i], a[i], b[i]))
        }
    }

    trait ArmF16FmaFromCaps<T: F32x4Backend + F32x8Backend + U16x8Backend> {
        fn fmlal_low_x4(
            proof: T,
            acc: <T as F32x4Backend>::Repr,
            a: <T as U16x8Backend>::Repr,
            b: <T as U16x8Backend>::Repr,
        ) -> <T as F32x4Backend>::Repr;
        fn fmlal_high_x4(
            proof: T,
            acc: <T as F32x4Backend>::Repr,
            a: <T as U16x8Backend>::Repr,
            b: <T as U16x8Backend>::Repr,
        ) -> <T as F32x4Backend>::Repr;
        fn fmlsl_low_x4(
            proof: T,
            acc: <T as F32x4Backend>::Repr,
            a: <T as U16x8Backend>::Repr,
            b: <T as U16x8Backend>::Repr,
        ) -> <T as F32x4Backend>::Repr;
        fn fmlsl_high_x4(
            proof: T,
            acc: <T as F32x4Backend>::Repr,
            a: <T as U16x8Backend>::Repr,
            b: <T as U16x8Backend>::Repr,
        ) -> <T as F32x4Backend>::Repr;
        fn fmlal_x8(
            proof: T,
            acc: <T as F32x8Backend>::Repr,
            a: <T as U16x8Backend>::Repr,
            b: <T as U16x8Backend>::Repr,
        ) -> <T as F32x8Backend>::Repr;
        fn fmlsl_x8(
            proof: T,
            acc: <T as F32x8Backend>::Repr,
            a: <T as U16x8Backend>::Repr,
            b: <T as U16x8Backend>::Repr,
        ) -> <T as F32x8Backend>::Repr;
    }

    impl<
        T,
        C1: Flag,
        C2: Flag,
        C3: Flag,
        C4: Flag,
        C5: Flag,
        C6: Flag,
        C9: Flag,
        C10: Flag,
        C11: Flag,
    > ArmF16FmaFromCaps<T> for ArmCapSet<Yes, C1, C2, C3, C4, C5, C6, Yes, Yes, C9, C10, C11>
    where
        T: archmage::SimdToken<__Caps = Self>
            + F32x4Backend<Repr = float32x4_t>
            + F32x8Backend<Repr = [float32x4_t; 2]>
            + U16x8Backend<Repr = uint16x8_t>,
    {
        #[inline(always)]
        fn fmlal_low_x4(proof: T, acc: float32x4_t, a: uint16x8_t, b: uint16x8_t) -> float32x4_t {
            arm_fmlal_low_x4(caps::extract_neon_fhm_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn fmlal_high_x4(proof: T, acc: float32x4_t, a: uint16x8_t, b: uint16x8_t) -> float32x4_t {
            arm_fmlal_high_x4(caps::extract_neon_fhm_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn fmlsl_low_x4(proof: T, acc: float32x4_t, a: uint16x8_t, b: uint16x8_t) -> float32x4_t {
            arm_fmlsl_low_x4(caps::extract_neon_fhm_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn fmlsl_high_x4(proof: T, acc: float32x4_t, a: uint16x8_t, b: uint16x8_t) -> float32x4_t {
            arm_fmlsl_high_x4(caps::extract_neon_fhm_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn fmlal_x8(
            proof: T,
            acc: [float32x4_t; 2],
            a: uint16x8_t,
            b: uint16x8_t,
        ) -> [float32x4_t; 2] {
            let t = caps::extract_neon_fhm_caps(proof);
            [
                arm_fmlal_low_x4(t, acc[0], a, b),
                arm_fmlal_high_x4(t, acc[1], a, b),
            ]
        }
        #[inline(always)]
        fn fmlsl_x8(
            proof: T,
            acc: [float32x4_t; 2],
            a: uint16x8_t,
            b: uint16x8_t,
        ) -> [float32x4_t; 2] {
            let t = caps::extract_neon_fhm_caps(proof);
            [
                arm_fmlsl_low_x4(t, acc[0], a, b),
                arm_fmlsl_high_x4(t, acc[1], a, b),
            ]
        }
    }

    impl<T> cap_sealed::ArmF16FmaOps for T
    where
        T: cap_sealed::Sealed + archmage::SimdToken + F32x4Backend + F32x8Backend + U16x8Backend,
        <T as archmage::SimdToken>::__Caps: ArmF16FmaFromCaps<T>,
    {
        #[inline(always)]
        fn fmlal_low_x4(
            self,
            acc: <Self as F32x4Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmF16FmaFromCaps<T>>::fmlal_low_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn fmlal_high_x4(
            self,
            acc: <Self as F32x4Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmF16FmaFromCaps<T>>::fmlal_high_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn fmlsl_low_x4(
            self,
            acc: <Self as F32x4Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmF16FmaFromCaps<T>>::fmlsl_low_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn fmlsl_high_x4(
            self,
            acc: <Self as F32x4Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmF16FmaFromCaps<T>>::fmlsl_high_x4(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn fmlal_x8(
            self,
            acc: <Self as F32x8Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmF16FmaFromCaps<T>>::fmlal_x8(self, acc, a, b)
        }
        #[inline(always)]
        fn fmlsl_x8(
            self,
            acc: <Self as F32x8Backend>::Repr,
            a: <Self as U16x8Backend>::Repr,
            b: <Self as U16x8Backend>::Repr,
        ) -> <Self as F32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as ArmF16FmaFromCaps<T>>::fmlsl_x8(self, acc, a, b)
        }
    }

    impl<T> DotI8Signed for T where
        T: cap_sealed::Sealed
            + cap_sealed::DotI8SignedOps
            + cap_sealed::DotI8SignedW512
            + I32x4Backend
            + I8x16Backend
            + I32x8Backend
            + I8x32Backend
            + archmage::SimdToken
            + Copy
            + 'static
    {
    }

    impl<T> DotU8 for T where
        T: cap_sealed::Sealed
            + cap_sealed::DotU8Ops
            + cap_sealed::DotU8W512
            + U32x4Backend
            + U8x16Backend
            + U32x8Backend
            + U8x32Backend
            + archmage::SimdToken
            + Copy
            + 'static
    {
    }

    impl<T> ArmRdma for T where
        T: cap_sealed::Sealed
            + cap_sealed::ArmRdmaOps
            + cap_sealed::ArmRdmaW512
            + I16x8Backend
            + I16x16Backend
            + archmage::SimdToken
            + Copy
            + 'static
    {
    }

    impl<T> ArmF16Fma for T where
        T: cap_sealed::Sealed
            + cap_sealed::ArmF16FmaOps
            + F32x4Backend
            + F32x8Backend
            + U16x8Backend
            + archmage::SimdToken
            + Copy
            + 'static
    {
    }
}

// ============================================================================
// wasm32 relaxed-simd implementations
// ============================================================================

#[cfg(all(
    target_arch = "wasm32",
    target_feature = "simd128",
    target_feature = "relaxed-simd"
))]
mod wasm_impls {
    use super::{RelaxedSimd, cap_sealed};
    use crate::simd::backends::{
        F32x4Backend, F32x8Backend, F64x2Backend, F64x4Backend, I8x16Backend, I8x32Backend,
        I16x8Backend, I16x16Backend, I32x4Backend, I32x8Backend, U32x4Backend, U32x8Backend,
    };
    use archmage::caps::{self, Flag, WasmCapSet, Yes};
    use core::arch::wasm32::{
        f32x4_relaxed_max, f32x4_relaxed_min, f64x2_relaxed_max, f64x2_relaxed_min,
        i16x8_relaxed_dot_i8x16_i7x16, i16x8_relaxed_q15mulr, i32x4_relaxed_dot_i8x16_i7x16_add,
        i32x4_relaxed_trunc_f32x4, u32x4_relaxed_trunc_f32x4, v128,
    };

    #[archmage::arcane]
    fn wasm_f32x4_relaxed_min(_t: archmage::Wasm128RelaxedToken, a: v128, b: v128) -> v128 {
        f32x4_relaxed_min(a, b)
    }
    #[archmage::arcane]
    fn wasm_f32x4_relaxed_max(_t: archmage::Wasm128RelaxedToken, a: v128, b: v128) -> v128 {
        f32x4_relaxed_max(a, b)
    }
    #[archmage::arcane]
    fn wasm_f32x4_relaxed_to_i32(_t: archmage::Wasm128RelaxedToken, a: v128) -> v128 {
        i32x4_relaxed_trunc_f32x4(a)
    }
    #[archmage::arcane]
    fn wasm_f32x4_relaxed_to_u32(_t: archmage::Wasm128RelaxedToken, a: v128) -> v128 {
        u32x4_relaxed_trunc_f32x4(a)
    }
    #[archmage::arcane]
    fn wasm_f32x8_relaxed_min(
        _t: archmage::Wasm128RelaxedToken,
        a: [v128; 2],
        b: [v128; 2],
    ) -> [v128; 2] {
        [f32x4_relaxed_min(a[0], b[0]), f32x4_relaxed_min(a[1], b[1])]
    }
    #[archmage::arcane]
    fn wasm_f32x8_relaxed_max(
        _t: archmage::Wasm128RelaxedToken,
        a: [v128; 2],
        b: [v128; 2],
    ) -> [v128; 2] {
        [f32x4_relaxed_max(a[0], b[0]), f32x4_relaxed_max(a[1], b[1])]
    }
    #[archmage::arcane]
    fn wasm_f32x8_relaxed_to_i32(_t: archmage::Wasm128RelaxedToken, a: [v128; 2]) -> [v128; 2] {
        [
            i32x4_relaxed_trunc_f32x4(a[0]),
            i32x4_relaxed_trunc_f32x4(a[1]),
        ]
    }
    #[archmage::arcane]
    fn wasm_f32x8_relaxed_to_u32(_t: archmage::Wasm128RelaxedToken, a: [v128; 2]) -> [v128; 2] {
        [
            u32x4_relaxed_trunc_f32x4(a[0]),
            u32x4_relaxed_trunc_f32x4(a[1]),
        ]
    }
    #[archmage::arcane]
    fn wasm_f64x2_relaxed_min(_t: archmage::Wasm128RelaxedToken, a: v128, b: v128) -> v128 {
        f64x2_relaxed_min(a, b)
    }
    #[archmage::arcane]
    fn wasm_f64x2_relaxed_max(_t: archmage::Wasm128RelaxedToken, a: v128, b: v128) -> v128 {
        f64x2_relaxed_max(a, b)
    }
    #[archmage::arcane]
    fn wasm_f64x4_relaxed_min(
        _t: archmage::Wasm128RelaxedToken,
        a: [v128; 2],
        b: [v128; 2],
    ) -> [v128; 2] {
        [f64x2_relaxed_min(a[0], b[0]), f64x2_relaxed_min(a[1], b[1])]
    }
    #[archmage::arcane]
    fn wasm_f64x4_relaxed_max(
        _t: archmage::Wasm128RelaxedToken,
        a: [v128; 2],
        b: [v128; 2],
    ) -> [v128; 2] {
        [f64x2_relaxed_max(a[0], b[0]), f64x2_relaxed_max(a[1], b[1])]
    }
    #[archmage::arcane]
    fn wasm_i16x8_relaxed_mul_hrs_q15(_t: archmage::Wasm128RelaxedToken, a: v128, b: v128) -> v128 {
        i16x8_relaxed_q15mulr(a, b)
    }
    #[archmage::arcane]
    fn wasm_i16x8_relaxed_dot_i8_i7(_t: archmage::Wasm128RelaxedToken, a: v128, b: v128) -> v128 {
        i16x8_relaxed_dot_i8x16_i7x16(a, b)
    }
    #[archmage::arcane]
    fn wasm_i16x16_relaxed_mul_hrs_q15(
        _t: archmage::Wasm128RelaxedToken,
        a: [v128; 2],
        b: [v128; 2],
    ) -> [v128; 2] {
        [
            i16x8_relaxed_q15mulr(a[0], b[0]),
            i16x8_relaxed_q15mulr(a[1], b[1]),
        ]
    }
    #[archmage::arcane]
    fn wasm_i16x16_relaxed_dot_i8_i7(
        _t: archmage::Wasm128RelaxedToken,
        a: [v128; 2],
        b: [v128; 2],
    ) -> [v128; 2] {
        [
            i16x8_relaxed_dot_i8x16_i7x16(a[0], b[0]),
            i16x8_relaxed_dot_i8x16_i7x16(a[1], b[1]),
        ]
    }
    #[archmage::arcane]
    fn wasm_i32x4_relaxed_dot_i8_i7_add(
        _t: archmage::Wasm128RelaxedToken,
        acc: v128,
        a: v128,
        b: v128,
    ) -> v128 {
        i32x4_relaxed_dot_i8x16_i7x16_add(a, b, acc)
    }
    #[archmage::arcane]
    fn wasm_i32x8_relaxed_dot_i8_i7_add(
        _t: archmage::Wasm128RelaxedToken,
        acc: [v128; 2],
        a: [v128; 2],
        b: [v128; 2],
    ) -> [v128; 2] {
        [
            i32x4_relaxed_dot_i8x16_i7x16_add(a[0], b[0], acc[0]),
            i32x4_relaxed_dot_i8x16_i7x16_add(a[1], b[1], acc[1]),
        ]
    }

    trait RelaxedSimdFromCaps<T>
    where
        T: F32x4Backend
            + F32x8Backend
            + F64x2Backend
            + F64x4Backend
            + I8x16Backend
            + I8x32Backend
            + I16x8Backend
            + I16x16Backend
            + I32x4Backend
            + I32x8Backend
            + U32x4Backend
            + U32x8Backend,
    {
        fn f32x4_relaxed_min(
            proof: T,
            a: <T as F32x4Backend>::Repr,
            b: <T as F32x4Backend>::Repr,
        ) -> <T as F32x4Backend>::Repr;
        fn f32x4_relaxed_max(
            proof: T,
            a: <T as F32x4Backend>::Repr,
            b: <T as F32x4Backend>::Repr,
        ) -> <T as F32x4Backend>::Repr;
        fn f32x4_relaxed_to_i32(
            proof: T,
            a: <T as F32x4Backend>::Repr,
        ) -> <T as I32x4Backend>::Repr;
        fn f32x4_relaxed_to_u32(
            proof: T,
            a: <T as F32x4Backend>::Repr,
        ) -> <T as U32x4Backend>::Repr;
        fn f32x8_relaxed_min(
            proof: T,
            a: <T as F32x8Backend>::Repr,
            b: <T as F32x8Backend>::Repr,
        ) -> <T as F32x8Backend>::Repr;
        fn f32x8_relaxed_max(
            proof: T,
            a: <T as F32x8Backend>::Repr,
            b: <T as F32x8Backend>::Repr,
        ) -> <T as F32x8Backend>::Repr;
        fn f32x8_relaxed_to_i32(
            proof: T,
            a: <T as F32x8Backend>::Repr,
        ) -> <T as I32x8Backend>::Repr;
        fn f32x8_relaxed_to_u32(
            proof: T,
            a: <T as F32x8Backend>::Repr,
        ) -> <T as U32x8Backend>::Repr;
        fn f64x2_relaxed_min(
            proof: T,
            a: <T as F64x2Backend>::Repr,
            b: <T as F64x2Backend>::Repr,
        ) -> <T as F64x2Backend>::Repr;
        fn f64x2_relaxed_max(
            proof: T,
            a: <T as F64x2Backend>::Repr,
            b: <T as F64x2Backend>::Repr,
        ) -> <T as F64x2Backend>::Repr;
        fn f64x4_relaxed_min(
            proof: T,
            a: <T as F64x4Backend>::Repr,
            b: <T as F64x4Backend>::Repr,
        ) -> <T as F64x4Backend>::Repr;
        fn f64x4_relaxed_max(
            proof: T,
            a: <T as F64x4Backend>::Repr,
            b: <T as F64x4Backend>::Repr,
        ) -> <T as F64x4Backend>::Repr;
        fn i16x8_relaxed_mul_hrs_q15(
            proof: T,
            a: <T as I16x8Backend>::Repr,
            b: <T as I16x8Backend>::Repr,
        ) -> <T as I16x8Backend>::Repr;
        fn i16x8_relaxed_dot_i8_i7(
            proof: T,
            a: <T as I8x16Backend>::Repr,
            b: <T as I8x16Backend>::Repr,
        ) -> <T as I16x8Backend>::Repr;
        fn i16x16_relaxed_mul_hrs_q15(
            proof: T,
            a: <T as I16x16Backend>::Repr,
            b: <T as I16x16Backend>::Repr,
        ) -> <T as I16x16Backend>::Repr;
        fn i16x16_relaxed_dot_i8_i7(
            proof: T,
            a: <T as I8x32Backend>::Repr,
            b: <T as I8x32Backend>::Repr,
        ) -> <T as I16x16Backend>::Repr;
        fn i32x4_relaxed_dot_i8_i7_add(
            proof: T,
            acc: <T as I32x4Backend>::Repr,
            a: <T as I8x16Backend>::Repr,
            b: <T as I8x16Backend>::Repr,
        ) -> <T as I32x4Backend>::Repr;
        fn i32x8_relaxed_dot_i8_i7_add(
            proof: T,
            acc: <T as I32x8Backend>::Repr,
            a: <T as I8x32Backend>::Repr,
            b: <T as I8x32Backend>::Repr,
        ) -> <T as I32x8Backend>::Repr;
    }

    impl<T, C2: Flag> RelaxedSimdFromCaps<T> for WasmCapSet<Yes, Yes, C2>
    where
        T: archmage::SimdToken<__Caps = Self>
            + F32x4Backend<Repr = v128>
            + F32x8Backend<Repr = [v128; 2]>
            + F64x2Backend<Repr = v128>
            + F64x4Backend<Repr = [v128; 2]>
            + I8x16Backend<Repr = v128>
            + I8x32Backend<Repr = [v128; 2]>
            + I16x8Backend<Repr = v128>
            + I16x16Backend<Repr = [v128; 2]>
            + I32x4Backend<Repr = v128>
            + I32x8Backend<Repr = [v128; 2]>
            + U32x4Backend<Repr = v128>
            + U32x8Backend<Repr = [v128; 2]>,
    {
        #[inline(always)]
        fn f32x4_relaxed_min(proof: T, a: v128, b: v128) -> v128 {
            wasm_f32x4_relaxed_min(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn f32x4_relaxed_max(proof: T, a: v128, b: v128) -> v128 {
            wasm_f32x4_relaxed_max(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn f32x4_relaxed_to_i32(proof: T, a: v128) -> v128 {
            wasm_f32x4_relaxed_to_i32(caps::extract_wasm128_relaxed_caps(proof), a)
        }
        #[inline(always)]
        fn f32x4_relaxed_to_u32(proof: T, a: v128) -> v128 {
            wasm_f32x4_relaxed_to_u32(caps::extract_wasm128_relaxed_caps(proof), a)
        }
        #[inline(always)]
        fn f32x8_relaxed_min(proof: T, a: [v128; 2], b: [v128; 2]) -> [v128; 2] {
            wasm_f32x8_relaxed_min(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn f32x8_relaxed_max(proof: T, a: [v128; 2], b: [v128; 2]) -> [v128; 2] {
            wasm_f32x8_relaxed_max(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn f32x8_relaxed_to_i32(proof: T, a: [v128; 2]) -> [v128; 2] {
            wasm_f32x8_relaxed_to_i32(caps::extract_wasm128_relaxed_caps(proof), a)
        }
        #[inline(always)]
        fn f32x8_relaxed_to_u32(proof: T, a: [v128; 2]) -> [v128; 2] {
            wasm_f32x8_relaxed_to_u32(caps::extract_wasm128_relaxed_caps(proof), a)
        }
        #[inline(always)]
        fn f64x2_relaxed_min(proof: T, a: v128, b: v128) -> v128 {
            wasm_f64x2_relaxed_min(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn f64x2_relaxed_max(proof: T, a: v128, b: v128) -> v128 {
            wasm_f64x2_relaxed_max(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn f64x4_relaxed_min(proof: T, a: [v128; 2], b: [v128; 2]) -> [v128; 2] {
            wasm_f64x4_relaxed_min(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn f64x4_relaxed_max(proof: T, a: [v128; 2], b: [v128; 2]) -> [v128; 2] {
            wasm_f64x4_relaxed_max(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn i16x8_relaxed_mul_hrs_q15(proof: T, a: v128, b: v128) -> v128 {
            wasm_i16x8_relaxed_mul_hrs_q15(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn i16x8_relaxed_dot_i8_i7(proof: T, a: v128, b: v128) -> v128 {
            wasm_i16x8_relaxed_dot_i8_i7(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn i16x16_relaxed_mul_hrs_q15(proof: T, a: [v128; 2], b: [v128; 2]) -> [v128; 2] {
            wasm_i16x16_relaxed_mul_hrs_q15(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn i16x16_relaxed_dot_i8_i7(proof: T, a: [v128; 2], b: [v128; 2]) -> [v128; 2] {
            wasm_i16x16_relaxed_dot_i8_i7(caps::extract_wasm128_relaxed_caps(proof), a, b)
        }
        #[inline(always)]
        fn i32x4_relaxed_dot_i8_i7_add(proof: T, acc: v128, a: v128, b: v128) -> v128 {
            wasm_i32x4_relaxed_dot_i8_i7_add(caps::extract_wasm128_relaxed_caps(proof), acc, a, b)
        }
        #[inline(always)]
        fn i32x8_relaxed_dot_i8_i7_add(
            proof: T,
            acc: [v128; 2],
            a: [v128; 2],
            b: [v128; 2],
        ) -> [v128; 2] {
            wasm_i32x8_relaxed_dot_i8_i7_add(caps::extract_wasm128_relaxed_caps(proof), acc, a, b)
        }
    }

    impl<T> cap_sealed::RelaxedSimdOps for T
    where
        T: cap_sealed::Sealed
            + archmage::SimdToken
            + F32x4Backend
            + F32x8Backend
            + F64x2Backend
            + F64x4Backend
            + I8x16Backend
            + I8x32Backend
            + I16x8Backend
            + I16x16Backend
            + I32x4Backend
            + I32x8Backend
            + U32x4Backend
            + U32x8Backend,
        <T as archmage::SimdToken>::__Caps: RelaxedSimdFromCaps<T>,
    {
        #[inline(always)]
        fn f32x4_relaxed_min(
            self,
            a: <Self as F32x4Backend>::Repr,
            b: <Self as F32x4Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f32x4_relaxed_min(
                self, a, b,
            )
        }
        #[inline(always)]
        fn f32x4_relaxed_max(
            self,
            a: <Self as F32x4Backend>::Repr,
            b: <Self as F32x4Backend>::Repr,
        ) -> <Self as F32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f32x4_relaxed_max(
                self, a, b,
            )
        }
        #[inline(always)]
        fn f32x4_relaxed_to_i32(
            self,
            a: <Self as F32x4Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f32x4_relaxed_to_i32(
                self, a,
            )
        }
        #[inline(always)]
        fn f32x4_relaxed_to_u32(
            self,
            a: <Self as F32x4Backend>::Repr,
        ) -> <Self as U32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f32x4_relaxed_to_u32(
                self, a,
            )
        }
        #[inline(always)]
        fn f32x8_relaxed_min(
            self,
            a: <Self as F32x8Backend>::Repr,
            b: <Self as F32x8Backend>::Repr,
        ) -> <Self as F32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f32x8_relaxed_min(
                self, a, b,
            )
        }
        #[inline(always)]
        fn f32x8_relaxed_max(
            self,
            a: <Self as F32x8Backend>::Repr,
            b: <Self as F32x8Backend>::Repr,
        ) -> <Self as F32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f32x8_relaxed_max(
                self, a, b,
            )
        }
        #[inline(always)]
        fn f32x8_relaxed_to_i32(
            self,
            a: <Self as F32x8Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f32x8_relaxed_to_i32(
                self, a,
            )
        }
        #[inline(always)]
        fn f32x8_relaxed_to_u32(
            self,
            a: <Self as F32x8Backend>::Repr,
        ) -> <Self as U32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f32x8_relaxed_to_u32(
                self, a,
            )
        }
        #[inline(always)]
        fn f64x2_relaxed_min(
            self,
            a: <Self as F64x2Backend>::Repr,
            b: <Self as F64x2Backend>::Repr,
        ) -> <Self as F64x2Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f64x2_relaxed_min(
                self, a, b,
            )
        }
        #[inline(always)]
        fn f64x2_relaxed_max(
            self,
            a: <Self as F64x2Backend>::Repr,
            b: <Self as F64x2Backend>::Repr,
        ) -> <Self as F64x2Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f64x2_relaxed_max(
                self, a, b,
            )
        }
        #[inline(always)]
        fn f64x4_relaxed_min(
            self,
            a: <Self as F64x4Backend>::Repr,
            b: <Self as F64x4Backend>::Repr,
        ) -> <Self as F64x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f64x4_relaxed_min(
                self, a, b,
            )
        }
        #[inline(always)]
        fn f64x4_relaxed_max(
            self,
            a: <Self as F64x4Backend>::Repr,
            b: <Self as F64x4Backend>::Repr,
        ) -> <Self as F64x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::f64x4_relaxed_max(
                self, a, b,
            )
        }
        #[inline(always)]
        fn i16x8_relaxed_mul_hrs_q15(
            self,
            a: <Self as I16x8Backend>::Repr,
            b: <Self as I16x8Backend>::Repr,
        ) -> <Self as I16x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::i16x8_relaxed_mul_hrs_q15(
                self, a, b,
            )
        }
        #[inline(always)]
        fn i16x8_relaxed_dot_i8_i7(
            self,
            a: <Self as I8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I16x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::i16x8_relaxed_dot_i8_i7(
                self, a, b,
            )
        }
        #[inline(always)]
        fn i16x16_relaxed_mul_hrs_q15(
            self,
            a: <Self as I16x16Backend>::Repr,
            b: <Self as I16x16Backend>::Repr,
        ) -> <Self as I16x16Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::i16x16_relaxed_mul_hrs_q15(
                self, a, b,
            )
        }
        #[inline(always)]
        fn i16x16_relaxed_dot_i8_i7(
            self,
            a: <Self as I8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I16x16Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::i16x16_relaxed_dot_i8_i7(
                self, a, b,
            )
        }
        #[inline(always)]
        fn i32x4_relaxed_dot_i8_i7_add(
            self,
            acc: <Self as I32x4Backend>::Repr,
            a: <Self as I8x16Backend>::Repr,
            b: <Self as I8x16Backend>::Repr,
        ) -> <Self as I32x4Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::i32x4_relaxed_dot_i8_i7_add(
                self, acc, a, b,
            )
        }
        #[inline(always)]
        fn i32x8_relaxed_dot_i8_i7_add(
            self,
            acc: <Self as I32x8Backend>::Repr,
            a: <Self as I8x32Backend>::Repr,
            b: <Self as I8x32Backend>::Repr,
        ) -> <Self as I32x8Backend>::Repr {
            <<T as archmage::SimdToken>::__Caps as RelaxedSimdFromCaps<T>>::i32x8_relaxed_dot_i8_i7_add(
                self, acc, a, b,
            )
        }
    }

    impl<T> RelaxedSimd for T where
        T: cap_sealed::Sealed
            + cap_sealed::RelaxedSimdOps
            + F32x4Backend
            + F32x8Backend
            + F64x2Backend
            + F64x4Backend
            + I8x16Backend
            + I8x32Backend
            + I16x8Backend
            + I16x16Backend
            + I32x4Backend
            + I32x8Backend
            + U32x4Backend
            + U32x8Backend
            + archmage::SimdToken
            + Copy
            + 'static
    {
    }
}
