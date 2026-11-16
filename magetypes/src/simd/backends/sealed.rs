//! Marker trait sealing all SIMD backend traits to validated archmage tokens
//! and routing all tokens and `And` conjunctions through canonical `*Ops` traits.
//!
//! **Auto-generated** by `cargo xtask generate` - do not edit manually.

/// Crate-internal supertrait seal for `MagetypesToken`.
pub trait MagetypesSeal {}

mod seal_cap {
    pub trait SealedBase {
        type Narrow: crate::simd::backends::ops::NarrowOps;
        #[cfg(feature = "w512")]
        type W512: crate::simd::backends::ops::W512Ops;
        type MagetypesBackend: archmage::SimdToken + archmage::IntoConcreteToken;
        type Proof: Copy;
        const IS_SCALAR_BACKED: bool;
        type SimdF32<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdF64<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdI8<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdI16<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdI32<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdI64<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdU8<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdU16<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdU32<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        type SimdU64<Token: archmage::SimdToken + super::Sealed + Copy + 'static>;
        const SIMD_F32_LANES: usize;
        const SIMD_F64_LANES: usize;
        const SIMD_I32_LANES: usize;

        fn narrow(proof: Self::Proof) -> Self::Narrow;
        #[cfg(feature = "w512")]
        fn w512(proof: Self::Proof) -> Self::W512;
        fn magetypes_backend(proof: Self::Proof) -> Self::MagetypesBackend;

        #[inline(always)]
        fn v4_f32x4_to_u8_bytes(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 4] {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::to_u8_bytes(
                Self::narrow(proof),
                a,
            )
        }
        #[inline(always)]
        fn v4_f32x4_store_rgba_bytes(
            proof: Self::Proof,
            r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 16] {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::store_rgba_bytes(
                Self::narrow(proof),
                r,
                g,
                b,
                a,
            )
        }
        #[inline(always)]
        fn v4_f32x4_store_rgb_bytes(
            proof: Self::Proof,
            r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 12] {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::store_rgb_bytes(
                Self::narrow(proof),
                r,
                g,
                b,
            )
        }
        #[inline(always)]
        fn v4_f32x8_to_u8_bytes(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> [u8; 8] {
            <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::to_u8_bytes(
                Self::narrow(proof),
                a,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_add(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::mul_add(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_sub(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::mul_sub(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_add_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::mul_add_portable(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::mul_sub_portable(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x4_min_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::min_fast(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x4_max_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::max_fast(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_add_lane_relaxed<const LANE: i32>(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            acc: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::mul_add_lane_relaxed::<LANE>(
                Self::narrow(proof),
                a,
                v_in,
                acc,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_add(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::mul_add(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_sub(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::mul_sub(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_add_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::mul_add_portable(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::mul_sub_portable(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_min_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::min_fast(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_max_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::max_fast(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_add_lane_relaxed<const LANE: i32>(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            v_in: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::mul_add_lane_relaxed::<LANE>(
                Self::narrow(proof),
                a,
                v_in,
                acc,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_mul_add(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F32x16Ops>::mul_add(
                Self::w512(proof),
                a,
                b,
                c,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_mul_sub(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F32x16Ops>::mul_sub(
                Self::w512(proof),
                a,
                b,
                c,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_mul_add_portable(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F32x16Ops>::mul_add_portable(
                Self::w512(proof),
                a,
                b,
                c,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F32x16Ops>::mul_sub_portable(
                Self::w512(proof),
                a,
                b,
                c,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_min_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F32x16Ops>::min_fast(Self::w512(proof), a, b)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_max_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F32x16Ops>::max_fast(Self::w512(proof), a, b)
        }
        #[inline(always)]
        fn wasm_fma_f64x2_mul_add(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::mul_add(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x2_mul_sub(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::mul_sub(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x2_mul_add_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::mul_add_portable(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x2_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::mul_sub_portable(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x2_min_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::min_fast(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x2_max_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::max_fast(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x4_mul_add(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::mul_add(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x4_mul_sub(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::mul_sub(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x4_mul_add_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::mul_add_portable(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x4_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::mul_sub_portable(
                Self::narrow(proof),
                a,
                b,
                c,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x4_min_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::min_fast(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[inline(always)]
        fn wasm_fma_f64x4_max_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::max_fast(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_mul_add(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F64x8Ops>::mul_add(
                Self::w512(proof),
                a,
                b,
                c,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_mul_sub(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F64x8Ops>::mul_sub(
                Self::w512(proof),
                a,
                b,
                c,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_mul_add_portable(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F64x8Ops>::mul_add_portable(
                Self::w512(proof),
                a,
                b,
                c,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F64x8Ops>::mul_sub_portable(
                Self::w512(proof),
                a,
                b,
                c,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_min_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F64x8Ops>::min_fast(Self::w512(proof), a, b)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_max_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F64x8Ops>::max_fast(Self::w512(proof), a, b)
        }
        #[inline(always)]
        fn wasm_fma_f32x4_cvt_convert_f32_to_i32_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x4ConvertOps>::convert_f32_to_i32_fast(
                Self::narrow(proof),
                a,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_cvt_convert_f32_to_i32_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::F32x8ConvertOps>::convert_f32_to_i32_fast(
                Self::narrow(proof),
                a,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_cvt_convert_f32_to_i32_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::F32x16ConvertOps>::convert_f32_to_i32_fast(
                Self::w512(proof),
                a,
            )
        }
        #[inline(always)]
        fn wasm_fma_i16x8_mul_hrs_q15(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::mul_hrs_q15(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[inline(always)]
        fn wasm_fma_i16x16_mul_hrs_q15(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr {
            <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::mul_hrs_q15(
                Self::narrow(proof),
                a,
                b,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_i16x32_mul_hrs_q15(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr {
            <Self::W512 as crate::simd::backends::ops::I16x32Ops>::mul_hrs_q15(
                Self::w512(proof),
                a,
                b,
            )
        }

        #[inline(always)]
        fn f32x8_from_halves(
            proof: Self::Proof,
            lo: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            hi: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::generic::cross_width::NarrowCrossWidthOps>::from_halves_x8(
                Self::narrow(proof),
                lo,
                hi,
            )
        }

        #[inline(always)]
        fn f32x8_low(
            proof: Self::Proof,
            wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::generic::cross_width::NarrowCrossWidthOps>::low_x8(
                Self::narrow(proof),
                wide,
            )
        }

        #[inline(always)]
        fn f32x8_high(
            proof: Self::Proof,
            wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            <Self::Narrow as crate::simd::generic::cross_width::NarrowCrossWidthOps>::high_x8(
                Self::narrow(proof),
                wide,
            )
        }

        #[inline(always)]
        fn mul_add_lane4_relaxed_x8<const LANE: i32>(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            <Self::Narrow as crate::simd::generic::cross_width::NarrowCrossWidthOps>::mul_add_lane4_relaxed_x8::<LANE>(
                Self::narrow(proof),
                a,
                v_in,
                acc,
            )
        }

        #[cfg(feature = "w512")]
        fn f32x16_from_halves(
            proof: Self::Proof,
            lo: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            hi: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;

        #[cfg(feature = "w512")]
        fn f32x16_low(
            proof: Self::Proof,
            wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

        #[cfg(feature = "w512")]
        fn f32x16_high(
            proof: Self::Proof,
            wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

        #[inline(always)]
        fn f32x4_to_u32_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::U32x4Ops>::Repr {
            <Self::Narrow as crate::simd::generic::fast_ops::NarrowToU32Ops>::f32x4_to_u32_fast(
                Self::narrow(proof),
                a,
            )
        }

        #[inline(always)]
        fn f32x8_to_u32_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::U32x8Ops>::Repr {
            <Self::Narrow as crate::simd::generic::fast_ops::NarrowToU32Ops>::f32x8_to_u32_fast(
                Self::narrow(proof),
                a,
            )
        }

        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_to_u32_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::U32x16Ops>::Repr {
            <Self::W512 as crate::simd::generic::fast_ops::W512ToU32Ops>::f32x16_to_u32_fast(
                Self::w512(proof),
                a,
            )
        }

        #[inline(always)]
        fn i32x4_dot_i8_i7_add(
            proof: Self::Proof,
            acc: <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr,
            a: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr {
            <Self::Narrow as crate::simd::generic::quant_ops::NarrowDotI8Ops>::dot_i8_i7_add_x4(
                Self::narrow(proof),
                acc,
                a,
                b,
            )
        }

        #[inline(always)]
        fn i32x8_dot_i8_i7_add(
            proof: Self::Proof,
            acc: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
            a: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {
            <Self::Narrow as crate::simd::generic::quant_ops::NarrowDotI8Ops>::dot_i8_i7_add_x8(
                Self::narrow(proof),
                acc,
                a,
                b,
            )
        }

        #[cfg(feature = "w512")]
        #[inline(always)]
        fn i32x16_dot_i8_i7_add(
            proof: Self::Proof,
            acc: <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr,
            a: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr {
            <Self::W512 as crate::simd::generic::quant_ops::W512DotI8Ops>::dot_i8_i7_add_x16(
                Self::w512(proof),
                acc,
                a,
                b,
            )
        }

        fn f16_to_f32_into_ops(proof: Self::Proof, input: &[u16], output: &mut [f32]);

        fn f32_to_f16_into_ops(proof: Self::Proof, input: &[f32], output: &mut [u16]);

        #[cfg(feature = "w512")]
        fn f64x8_convert_to_i32(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

        #[cfg(feature = "w512")]
        fn f64x8_convert_to_i32_saturating(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

        #[cfg(feature = "w512")]
        fn f64x8_convert_to_i32_round(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

        #[cfg(feature = "w512")]
        fn f64x8_convert_from_i32(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
    }

    pub trait SealedFromCaps<T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static>
    {
        type Base: SealedBase;
        fn into_proof(token: T) -> <Self::Base as SealedBase>::Proof;
    }
}

/// Seals backend traits so only validated archmage tokens can implement them.
///
/// Also supplies the canonical `Narrow` / `W512` projections and optional
/// hardware override hooks (AVX-512VL pixel packing, WASM `relaxed-simd`
/// single-rounding ops, AVX-512F `to_u32_fast` / `dot_i8` / `f16` slice,
/// `F32x16FromHalves`).
pub trait Sealed: crate::simd::generic::MagetypesToken + Copy + 'static {
    /// Canonical 128/256-bit backend token implementing [`super::ops::NarrowOps`].
    type Narrow: crate::simd::backends::ops::NarrowOps;

    /// Canonical 512-bit backend token implementing [`super::ops::W512Ops`].
    #[cfg(feature = "w512")]
    type W512: crate::simd::backends::ops::W512Ops;

    /// Project `self` to its canonical narrow backend proof.
    fn narrow(self) -> Self::Narrow;

    /// Project `self` to its canonical 512-bit backend proof.
    #[cfg(feature = "w512")]
    fn w512(self) -> Self::W512;

    fn v4_f32x4_to_u8_bytes(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> [u8; 4];
    fn v4_f32x4_store_rgba_bytes(
        self,
        r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> [u8; 16];
    fn v4_f32x4_store_rgb_bytes(
        self,
        r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> [u8; 12];
    fn v4_f32x8_to_u8_bytes(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> [u8; 8];
    fn wasm_fma_f32x4_mul_add(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;
    fn wasm_fma_f32x4_mul_sub(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;
    fn wasm_fma_f32x4_mul_add_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;
    fn wasm_fma_f32x4_mul_sub_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;
    fn wasm_fma_f32x4_min_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;
    fn wasm_fma_f32x4_max_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;
    fn wasm_fma_f32x4_mul_add_lane_relaxed<const LANE: i32>(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        acc: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;
    fn wasm_fma_f32x8_mul_add(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;
    fn wasm_fma_f32x8_mul_sub(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;
    fn wasm_fma_f32x8_mul_add_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;
    fn wasm_fma_f32x8_mul_sub_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;
    fn wasm_fma_f32x8_min_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;
    fn wasm_fma_f32x8_max_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;
    fn wasm_fma_f32x8_mul_add_lane_relaxed<const LANE: i32>(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        v_in: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f32x16_mul_add(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f32x16_mul_sub(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f32x16_mul_add_portable(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f32x16_mul_sub_portable(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f32x16_min_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f32x16_max_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;
    fn wasm_fma_f64x2_mul_add(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr;
    fn wasm_fma_f64x2_mul_sub(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr;
    fn wasm_fma_f64x2_mul_add_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr;
    fn wasm_fma_f64x2_mul_sub_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr;
    fn wasm_fma_f64x2_min_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr;
    fn wasm_fma_f64x2_max_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr;
    fn wasm_fma_f64x4_mul_add(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr;
    fn wasm_fma_f64x4_mul_sub(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr;
    fn wasm_fma_f64x4_mul_add_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr;
    fn wasm_fma_f64x4_mul_sub_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr;
    fn wasm_fma_f64x4_min_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr;
    fn wasm_fma_f64x4_max_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f64x8_mul_add(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f64x8_mul_sub(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f64x8_mul_add_portable(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f64x8_mul_sub_portable(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f64x8_min_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f64x8_max_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
    fn wasm_fma_f32x4_cvt_convert_f32_to_i32_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr;
    fn wasm_fma_f32x8_cvt_convert_f32_to_i32_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_f32x16_cvt_convert_f32_to_i32_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr;
    fn wasm_fma_i16x8_mul_hrs_q15(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr;
    fn wasm_fma_i16x16_mul_hrs_q15(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr;
    #[cfg(feature = "w512")]
    fn wasm_fma_i16x32_mul_hrs_q15(
        self,
        a: <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr;

    fn f32x8_from_halves(
        self,
        lo: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        hi: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

    fn f32x8_low(
        self,
        wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;

    fn f32x8_high(
        self,
        wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr;

    fn mul_add_lane4_relaxed_x8<const LANE: i32>(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

    #[cfg(feature = "w512")]
    fn f32x16_from_halves(
        self,
        lo: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        hi: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr;

    #[cfg(feature = "w512")]
    fn f32x16_low(
        self,
        wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

    #[cfg(feature = "w512")]
    fn f32x16_high(
        self,
        wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr;

    fn f32x4_to_u32_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::U32x4Ops>::Repr;

    fn f32x8_to_u32_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::U32x8Ops>::Repr;

    #[cfg(feature = "w512")]
    fn f32x16_to_u32_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::U32x16Ops>::Repr;

    fn i32x4_dot_i8_i7_add(
        self,
        acc: <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr,
        a: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr;

    fn i32x8_dot_i8_i7_add(
        self,
        acc: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
        a: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

    #[cfg(feature = "w512")]
    fn i32x16_dot_i8_i7_add(
        self,
        acc: <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr,
        a: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr;

    fn f16_to_f32_into_ops(self, input: &[u16], output: &mut [f32]);

    fn f32_to_f16_into_ops(self, input: &[f32], output: &mut [u16]);

    #[cfg(feature = "w512")]
    fn f64x8_convert_to_i32(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

    #[cfg(feature = "w512")]
    fn f64x8_convert_to_i32_saturating(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

    #[cfg(feature = "w512")]
    fn f64x8_convert_to_i32_round(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr;

    #[cfg(feature = "w512")]
    fn f64x8_convert_from_i32(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr;
}

impl<T> MagetypesSeal for T
where
    T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
    <T as archmage::SimdToken>::__Caps: seal_cap::SealedFromCaps<T>,
{
}

impl<T> crate::simd::generic::MagetypesToken for T
where
    T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
    <T as archmage::SimdToken>::__Caps: seal_cap::SealedFromCaps<T>,
{
    type MagetypesBackend = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::MagetypesBackend;
    const __IS_SCALAR_BACKED: bool = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::IS_SCALAR_BACKED;

    #[inline(always)]
    fn magetypes(self) -> Self::MagetypesBackend {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::magetypes_backend(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
        )
    }
}

impl<T> crate::SimdTypes for T
where
    T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
    <T as archmage::SimdToken>::__Caps: seal_cap::SealedFromCaps<T>,
{
    type F32 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdF32<T>;
    type F64 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdF64<T>;
    type I8 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdI8<T>;
    type I16 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdI16<T>;
    type I32 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdI32<T>;
    type I64 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdI64<T>;
    type U8 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdU8<T>;
    type U16 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdU16<T>;
    type U32 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdU32<T>;
    type U64 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SimdU64<T>;
    const F32_LANES: usize = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SIMD_F32_LANES;
    const F64_LANES: usize = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SIMD_F64_LANES;
    const I32_LANES: usize = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::SIMD_I32_LANES;
}

impl<T> Sealed for T
where
    T: archmage::SimdToken + archmage::IntoConcreteToken + Copy + 'static,
    <T as archmage::SimdToken>::__Caps: seal_cap::SealedFromCaps<T>,
{
    type Narrow = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::Narrow;
    #[cfg(feature = "w512")]
    type W512 = <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::W512;

    #[inline(always)]
    fn narrow(self) -> Self::Narrow {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::narrow(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn w512(self) -> Self::W512 {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::w512(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
        )
    }

    #[inline(always)]
    fn v4_f32x4_to_u8_bytes(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> [u8; 4] {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: v4_f32x4_to_u8_bytes (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a)
    }
    #[inline(always)]
    fn v4_f32x4_store_rgba_bytes(
        self,
        r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> [u8; 16] {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: v4_f32x4_store_rgba_bytes (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , r , g , b , a)
    }
    #[inline(always)]
    fn v4_f32x4_store_rgb_bytes(
        self,
        r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> [u8; 12] {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: v4_f32x4_store_rgb_bytes (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , r , g , b)
    }
    #[inline(always)]
    fn v4_f32x8_to_u8_bytes(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> [u8; 8] {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: v4_f32x8_to_u8_bytes (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a)
    }
    #[inline(always)]
    fn wasm_fma_f32x4_mul_add(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x4_mul_add (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f32x4_mul_sub(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x4_mul_sub (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f32x4_mul_add_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x4_mul_add_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f32x4_mul_sub_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x4_mul_sub_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f32x4_min_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x4_min_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f32x4_max_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x4_max_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f32x4_mul_add_lane_relaxed<const LANE: i32>(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        acc: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x4_mul_add_lane_relaxed :: < LANE > (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , v_in , acc)
    }
    #[inline(always)]
    fn wasm_fma_f32x8_mul_add(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x8_mul_add (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f32x8_mul_sub(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x8_mul_sub (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f32x8_mul_add_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x8_mul_add_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f32x8_mul_sub_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x8_mul_sub_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f32x8_min_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x8_min_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f32x8_max_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x8_max_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f32x8_mul_add_lane_relaxed<const LANE: i32>(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        v_in: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x8_mul_add_lane_relaxed :: < LANE > (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , v_in , acc)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f32x16_mul_add(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x16_mul_add (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f32x16_mul_sub(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x16_mul_sub (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f32x16_mul_add_portable(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x16_mul_add_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f32x16_mul_sub_portable(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x16_mul_sub_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f32x16_min_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x16_min_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f32x16_max_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x16_max_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f64x2_mul_add(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x2_mul_add (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f64x2_mul_sub(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x2_mul_sub (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f64x2_mul_add_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x2_mul_add_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f64x2_mul_sub_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x2_mul_sub_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f64x2_min_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x2_min_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f64x2_max_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x2_max_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f64x4_mul_add(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x4_mul_add (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f64x4_mul_sub(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x4_mul_sub (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f64x4_mul_add_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x4_mul_add_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f64x4_mul_sub_portable(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x4_mul_sub_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[inline(always)]
    fn wasm_fma_f64x4_min_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x4_min_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f64x4_max_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x4_max_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f64x8_mul_add(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x8_mul_add (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f64x8_mul_sub(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x8_mul_sub (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f64x8_mul_add_portable(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x8_mul_add_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f64x8_mul_sub_portable(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x8_mul_sub_portable (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b , c)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f64x8_min_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x8_min_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f64x8_max_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f64x8_max_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_f32x4_cvt_convert_f32_to_i32_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x4_cvt_convert_f32_to_i32_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a)
    }
    #[inline(always)]
    fn wasm_fma_f32x8_cvt_convert_f32_to_i32_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x8_cvt_convert_f32_to_i32_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_f32x16_cvt_convert_f32_to_i32_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_f32x16_cvt_convert_f32_to_i32_fast (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a)
    }
    #[inline(always)]
    fn wasm_fma_i16x8_mul_hrs_q15(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_i16x8_mul_hrs_q15 (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[inline(always)]
    fn wasm_fma_i16x16_mul_hrs_q15(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_i16x16_mul_hrs_q15 (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }
    #[cfg(feature = "w512")]
    #[inline(always)]
    fn wasm_fma_i16x32_mul_hrs_q15(
        self,
        a: <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr {
        << < T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: Base as seal_cap :: SealedBase > :: wasm_fma_i16x32_mul_hrs_q15 (<< T as archmage :: SimdToken > :: __Caps as seal_cap :: SealedFromCaps < T >> :: into_proof (self) , a , b)
    }

    #[inline(always)]
    fn f32x8_from_halves(
        self,
        lo: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        hi: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x8_from_halves(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            lo,
            hi,
        )
    }

    #[inline(always)]
    fn f32x8_low(
        self,
        wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x8_low(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            wide,
        )
    }

    #[inline(always)]
    fn f32x8_high(
        self,
        wide: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x8_high(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            wide,
        )
    }

    #[inline(always)]
    fn mul_add_lane4_relaxed_x8<const LANE: i32>(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::mul_add_lane4_relaxed_x8::<LANE>(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            a,
            v_in,
            acc,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn f32x16_from_halves(
        self,
        lo: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        hi: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x16_from_halves(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            lo,
            hi,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn f32x16_low(
        self,
        wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x16_low(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            wide,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn f32x16_high(
        self,
        wide: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x16_high(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            wide,
        )
    }

    #[inline(always)]
    fn f32x4_to_u32_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::U32x4Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x4_to_u32_fast(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            a,
        )
    }

    #[inline(always)]
    fn f32x8_to_u32_fast(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::U32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x8_to_u32_fast(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            a,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn f32x16_to_u32_fast(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::U32x16Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32x16_to_u32_fast(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            a,
        )
    }

    #[inline(always)]
    fn i32x4_dot_i8_i7_add(
        self,
        acc: <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr,
        a: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::I8x16Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::i32x4_dot_i8_i7_add(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            acc,
            a,
            b,
        )
    }

    #[inline(always)]
    fn i32x8_dot_i8_i7_add(
        self,
        acc: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
        a: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
        b: <Self::Narrow as crate::simd::backends::ops::I8x32Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::i32x8_dot_i8_i7_add(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            acc,
            a,
            b,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn i32x16_dot_i8_i7_add(
        self,
        acc: <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr,
        a: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
        b: <Self::W512 as crate::simd::backends::ops::I8x64Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::i32x16_dot_i8_i7_add(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            acc,
            a,
            b,
        )
    }

    #[inline(always)]
    fn f16_to_f32_into_ops(self, input: &[u16], output: &mut [f32]) {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f16_to_f32_into_ops(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            input,
            output,
        )
    }

    #[inline(always)]
    fn f32_to_f16_into_ops(self, input: &[f32], output: &mut [u16]) {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f32_to_f16_into_ops(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            input,
            output,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn f64x8_convert_to_i32(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f64x8_convert_to_i32(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            a,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn f64x8_convert_to_i32_saturating(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f64x8_convert_to_i32_saturating(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            a,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn f64x8_convert_to_i32_round(
        self,
        a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
    ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f64x8_convert_to_i32_round(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            a,
        )
    }

    #[cfg(feature = "w512")]
    #[inline(always)]
    fn f64x8_convert_from_i32(
        self,
        a: <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr,
    ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
        <<<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::Base as seal_cap::SealedBase>::f64x8_convert_from_i32(
            <<T as archmage::SimdToken>::__Caps as seal_cap::SealedFromCaps<T>>::into_proof(self),
            a,
        )
    }
}

mod seal_base_impls {
    use super::seal_cap::SealedBase;

    macro_rules! impl_simd_types_assoc {
        (scalar) => {
            type SimdF32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = f32;
            type SimdF64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = f64;
            type SimdI8<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = i8;
            type SimdI16<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = i16;
            type SimdI32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = i32;
            type SimdI64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = i64;
            type SimdU8<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = u8;
            type SimdU16<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = u16;
            type SimdU32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = u32;
            type SimdU64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> = u64;
            const SIMD_F32_LANES: usize = 1;
            const SIMD_F64_LANES: usize = 1;
            const SIMD_I32_LANES: usize = 1;
        };
        ($f32:ident, $f64:ident, $i8:ident, $i16:ident, $i32:ident, $i64:ident, $u8:ident, $u16:ident, $u32:ident, $u64:ident, $l32:literal, $l64:literal) => {
            type SimdF32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$f32<Token>;
            type SimdF64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$f64<Token>;
            type SimdI8<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$i8<Token>;
            type SimdI16<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$i16<Token>;
            type SimdI32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$i32<Token>;
            type SimdI64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$i64<Token>;
            type SimdU8<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$u8<Token>;
            type SimdU16<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$u16<Token>;
            type SimdU32<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$u32<Token>;
            type SimdU64<Token: archmage::SimdToken + super::Sealed + Copy + 'static> =
                crate::simd::generic::$u64<Token>;
            const SIMD_F32_LANES: usize = $l32;
            const SIMD_F64_LANES: usize = $l64;
            const SIMD_I32_LANES: usize = $l32;
        };
    }

    pub struct ScalarBase;

    impl SealedBase for ScalarBase {
        type Narrow = archmage::ScalarToken;
        #[cfg(feature = "w512")]
        type W512 = archmage::ScalarToken;
        type MagetypesBackend = archmage::ScalarToken;
        type Proof = archmage::ScalarToken;
        const IS_SCALAR_BACKED: bool = true;
        impl_simd_types_assoc!(scalar);

        #[inline(always)]
        fn narrow(proof: archmage::ScalarToken) -> Self::Narrow {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn w512(proof: archmage::ScalarToken) -> Self::W512 {
            proof
        }
        #[inline(always)]
        fn magetypes_backend(proof: archmage::ScalarToken) -> Self::MagetypesBackend {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_from_halves(
            _proof: archmage::ScalarToken,
            lo: [f32; 8],
            hi: [f32; 8],
        ) -> [f32; 16] {
            crate::simd::generic::cross_width::scalar_f32x16_from_halves(lo, hi)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_low(_proof: archmage::ScalarToken, wide: [f32; 16]) -> [f32; 8] {
            crate::simd::generic::cross_width::scalar_f32x16_low(wide)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_high(_proof: archmage::ScalarToken, wide: [f32; 16]) -> [f32; 8] {
            crate::simd::generic::cross_width::scalar_f32x16_high(wide)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32(_proof: archmage::ScalarToken, a: [f64; 8]) -> [i32; 8] {
            crate::simd::impls::scalar::scalar_f64x8_to_i32(a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_saturating(_proof: archmage::ScalarToken, a: [f64; 8]) -> [i32; 8] {
            crate::simd::impls::scalar::scalar_f64x8_to_i32(a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_round(_proof: archmage::ScalarToken, a: [f64; 8]) -> [i32; 8] {
            crate::simd::impls::scalar::scalar_f64x8_to_i32_round(a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_from_i32(_proof: archmage::ScalarToken, a: [i32; 8]) -> [f64; 8] {
            crate::simd::impls::scalar::scalar_f64x8_from_i32(a)
        }
        #[inline(always)]
        fn f16_to_f32_into_ops(proof: archmage::ScalarToken, input: &[u16], output: &mut [f32]) {
            crate::simd::generic::convert_f16::f16_to_f32_slice_soft(proof, input, output);
        }
        #[inline(always)]
        fn f32_to_f16_into_ops(proof: archmage::ScalarToken, input: &[f32], output: &mut [u16]) {
            crate::simd::generic::convert_f16::f32_to_f16_slice_soft(proof, input, output);
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub struct X64V3Base;

    #[cfg(target_arch = "x86_64")]
    impl SealedBase for X64V3Base {
        type Narrow = archmage::X64V3Token;
        #[cfg(feature = "w512")]
        type W512 = archmage::X64V3Token;
        type MagetypesBackend = archmage::X64V3Token;
        type Proof = archmage::X64V3Token;
        const IS_SCALAR_BACKED: bool = false;
        impl_simd_types_assoc!(
            f32x8, f64x4, i8x32, i16x16, i32x8, i64x4, u8x32, u16x16, u32x8, u64x4, 8, 4
        );

        #[inline(always)]
        fn narrow(proof: archmage::X64V3Token) -> Self::Narrow {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn w512(proof: archmage::X64V3Token) -> Self::W512 {
            proof
        }
        #[inline(always)]
        fn magetypes_backend(proof: archmage::X64V3Token) -> Self::MagetypesBackend {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_from_halves(
            _proof: archmage::X64V3Token,
            lo: core::arch::x86_64::__m256,
            hi: core::arch::x86_64::__m256,
        ) -> [core::arch::x86_64::__m256; 2] {
            [lo, hi]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_low(
            _proof: archmage::X64V3Token,
            wide: [core::arch::x86_64::__m256; 2],
        ) -> core::arch::x86_64::__m256 {
            wide[0]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_high(
            _proof: archmage::X64V3Token,
            wide: [core::arch::x86_64::__m256; 2],
        ) -> core::arch::x86_64::__m256 {
            wide[1]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32(
            proof: archmage::X64V3Token,
            a: [core::arch::x86_64::__m256d; 2],
        ) -> core::arch::x86_64::__m256i {
            crate::simd::impls::x86_v3::v3_f64x8_to_i32(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_saturating(
            proof: archmage::X64V3Token,
            a: [core::arch::x86_64::__m256d; 2],
        ) -> core::arch::x86_64::__m256i {
            crate::simd::impls::x86_v3::v3_f64x8_to_i32_saturating(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_round(
            proof: archmage::X64V3Token,
            a: [core::arch::x86_64::__m256d; 2],
        ) -> core::arch::x86_64::__m256i {
            crate::simd::impls::x86_v3::v3_f64x8_to_i32_round(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_from_i32(
            proof: archmage::X64V3Token,
            a: core::arch::x86_64::__m256i,
        ) -> [core::arch::x86_64::__m256d; 2] {
            crate::simd::impls::x86_v3::v3_f64x8_from_i32(proof, a)
        }
        #[inline(always)]
        fn f16_to_f32_into_ops(proof: archmage::X64V3Token, input: &[u16], output: &mut [f32]) {
            crate::simd::generic::convert_f16::f16c_decode_select(proof, input, output);
        }
        #[inline(always)]
        fn f32_to_f16_into_ops(proof: archmage::X64V3Token, input: &[f32], output: &mut [u16]) {
            crate::simd::generic::convert_f16::f16c_encode_select(proof, input, output);
        }
    }

    #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
    pub struct X64V4Base;

    #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
    impl SealedBase for X64V4Base {
        type Narrow = archmage::X64V3Token;
        #[cfg(feature = "w512")]
        type W512 = archmage::X64V4Token;
        type MagetypesBackend = archmage::X64V4Token;
        type Proof = archmage::X64V4Token;
        const IS_SCALAR_BACKED: bool = false;
        impl_simd_types_assoc!(
            f32x16, f64x8, i8x64, i16x32, i32x16, i64x8, u8x64, u16x32, u32x16, u64x8, 16, 8
        );

        #[inline(always)]
        fn narrow(proof: archmage::X64V4Token) -> Self::Narrow {
            archmage::caps::extract_v3_caps(proof)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn w512(proof: archmage::X64V4Token) -> Self::W512 {
            proof
        }
        #[inline(always)]
        fn magetypes_backend(proof: archmage::X64V4Token) -> Self::MagetypesBackend {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_from_halves(
            proof: archmage::X64V4Token,
            lo: core::arch::x86_64::__m256,
            hi: core::arch::x86_64::__m256,
        ) -> core::arch::x86_64::__m512 {
            crate::simd::generic::cross_width::v4_f32x16_from_halves(proof, lo, hi)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_low(
            proof: archmage::X64V4Token,
            wide: core::arch::x86_64::__m512,
        ) -> core::arch::x86_64::__m256 {
            crate::simd::generic::cross_width::v4_f32x16_low(proof, wide)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_high(
            proof: archmage::X64V4Token,
            wide: core::arch::x86_64::__m512,
        ) -> core::arch::x86_64::__m256 {
            crate::simd::generic::cross_width::v4_f32x16_high(proof, wide)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32(
            proof: archmage::X64V4Token,
            a: core::arch::x86_64::__m512d,
        ) -> core::arch::x86_64::__m256i {
            crate::simd::impls::x86_v4::v4_f64x8_to_i32(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_saturating(
            proof: archmage::X64V4Token,
            a: core::arch::x86_64::__m512d,
        ) -> core::arch::x86_64::__m256i {
            crate::simd::impls::x86_v4::v4_f64x8_to_i32_saturating(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_round(
            proof: archmage::X64V4Token,
            a: core::arch::x86_64::__m512d,
        ) -> core::arch::x86_64::__m256i {
            crate::simd::impls::x86_v4::v4_f64x8_to_i32_round(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_from_i32(
            proof: archmage::X64V4Token,
            a: core::arch::x86_64::__m256i,
        ) -> core::arch::x86_64::__m512d {
            crate::simd::impls::x86_v4::v4_f64x8_from_i32(proof, a)
        }
        #[inline(always)]
        fn v4_f32x4_to_u8_bytes(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 4] {
            crate::simd::impls::x86_v4_f32_overrides::to_u8_bytes_x4_v4(proof, a)
        }
        #[inline(always)]
        fn v4_f32x4_store_rgba_bytes(
            proof: Self::Proof,
            r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 16] {
            crate::simd::impls::x86_v4_f32_overrides::store_rgba_bytes_x4_v4(proof, r, g, b, a)
        }
        #[inline(always)]
        fn v4_f32x4_store_rgb_bytes(
            proof: Self::Proof,
            r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 12] {
            crate::simd::impls::x86_v4_f32_overrides::store_rgb_bytes_x4_v4(proof, r, g, b)
        }
        #[inline(always)]
        fn v4_f32x8_to_u8_bytes(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> [u8; 8] {
            crate::simd::impls::x86_v4_f32_overrides::to_u8_bytes_x8_v4(proof, a)
        }
        #[inline(always)]
        fn f32x4_to_u32_fast(
            proof: archmage::X64V4Token,
            a: core::arch::x86_64::__m128,
        ) -> core::arch::x86_64::__m128i {
            crate::simd::generic::fast_ops::v4_f32x4_to_u32_fast(proof, a)
        }
        #[inline(always)]
        fn f32x8_to_u32_fast(
            proof: archmage::X64V4Token,
            a: core::arch::x86_64::__m256,
        ) -> core::arch::x86_64::__m256i {
            crate::simd::generic::fast_ops::v4_f32x8_to_u32_fast(proof, a)
        }
        #[inline(always)]
        fn f16_to_f32_into_ops(proof: archmage::X64V4Token, input: &[u16], output: &mut [f32]) {
            crate::simd::generic::convert_f16::v4_f16_to_f32_into(proof, input, output);
        }
        #[inline(always)]
        fn f32_to_f16_into_ops(proof: archmage::X64V4Token, input: &[f32], output: &mut [u16]) {
            crate::simd::generic::convert_f16::v4_f32_to_f16_into(proof, input, output);
        }
    }

    #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
    pub struct X64V4xBase;

    #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
    impl SealedBase for X64V4xBase {
        type Narrow = archmage::X64V3Token;
        #[cfg(feature = "w512")]
        type W512 = archmage::X64V4Token;
        type MagetypesBackend = archmage::X64V4xToken;
        type Proof = archmage::X64V4xToken;
        const IS_SCALAR_BACKED: bool = false;
        impl_simd_types_assoc!(
            f32x16, f64x8, i8x64, i16x32, i32x16, i64x8, u8x64, u16x32, u32x16, u64x8, 16, 8
        );

        #[inline(always)]
        fn narrow(proof: archmage::X64V4xToken) -> Self::Narrow {
            archmage::caps::extract_v3_caps(proof)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn w512(proof: archmage::X64V4xToken) -> Self::W512 {
            archmage::caps::extract_v4_caps(proof)
        }
        #[inline(always)]
        fn magetypes_backend(proof: archmage::X64V4xToken) -> Self::MagetypesBackend {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_from_halves(
            proof: archmage::X64V4xToken,
            lo: core::arch::x86_64::__m256,
            hi: core::arch::x86_64::__m256,
        ) -> core::arch::x86_64::__m512 {
            <X64V4Base as SealedBase>::f32x16_from_halves(
                archmage::caps::extract_v4_caps(proof),
                lo,
                hi,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_low(
            proof: archmage::X64V4xToken,
            wide: core::arch::x86_64::__m512,
        ) -> core::arch::x86_64::__m256 {
            <X64V4Base as SealedBase>::f32x16_low(archmage::caps::extract_v4_caps(proof), wide)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_high(
            proof: archmage::X64V4xToken,
            wide: core::arch::x86_64::__m512,
        ) -> core::arch::x86_64::__m256 {
            <X64V4Base as SealedBase>::f32x16_high(archmage::caps::extract_v4_caps(proof), wide)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32(
            proof: archmage::X64V4xToken,
            a: core::arch::x86_64::__m512d,
        ) -> core::arch::x86_64::__m256i {
            <X64V4Base as SealedBase>::f64x8_convert_to_i32(
                archmage::caps::extract_v4_caps(proof),
                a,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_saturating(
            proof: archmage::X64V4xToken,
            a: core::arch::x86_64::__m512d,
        ) -> core::arch::x86_64::__m256i {
            <X64V4Base as SealedBase>::f64x8_convert_to_i32_saturating(
                archmage::caps::extract_v4_caps(proof),
                a,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_round(
            proof: archmage::X64V4xToken,
            a: core::arch::x86_64::__m512d,
        ) -> core::arch::x86_64::__m256i {
            <X64V4Base as SealedBase>::f64x8_convert_to_i32_round(
                archmage::caps::extract_v4_caps(proof),
                a,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_from_i32(
            proof: archmage::X64V4xToken,
            a: core::arch::x86_64::__m256i,
        ) -> core::arch::x86_64::__m512d {
            <X64V4Base as SealedBase>::f64x8_convert_from_i32(
                archmage::caps::extract_v4_caps(proof),
                a,
            )
        }
        #[inline(always)]
        fn v4_f32x4_to_u8_bytes(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 4] {
            <X64V4Base as SealedBase>::v4_f32x4_to_u8_bytes(
                archmage::caps::extract_v4_caps(proof),
                a,
            )
        }
        #[inline(always)]
        fn v4_f32x4_store_rgba_bytes(
            proof: Self::Proof,
            r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 16] {
            <X64V4Base as SealedBase>::v4_f32x4_store_rgba_bytes(
                archmage::caps::extract_v4_caps(proof),
                r,
                g,
                b,
                a,
            )
        }
        #[inline(always)]
        fn v4_f32x4_store_rgb_bytes(
            proof: Self::Proof,
            r: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            g: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> [u8; 12] {
            <X64V4Base as SealedBase>::v4_f32x4_store_rgb_bytes(
                archmage::caps::extract_v4_caps(proof),
                r,
                g,
                b,
            )
        }
        #[inline(always)]
        fn v4_f32x8_to_u8_bytes(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> [u8; 8] {
            <X64V4Base as SealedBase>::v4_f32x8_to_u8_bytes(
                archmage::caps::extract_v4_caps(proof),
                a,
            )
        }
        #[inline(always)]
        fn f32x4_to_u32_fast(
            proof: archmage::X64V4xToken,
            a: core::arch::x86_64::__m128,
        ) -> core::arch::x86_64::__m128i {
            <X64V4Base as SealedBase>::f32x4_to_u32_fast(archmage::caps::extract_v4_caps(proof), a)
        }
        #[inline(always)]
        fn f32x8_to_u32_fast(
            proof: archmage::X64V4xToken,
            a: core::arch::x86_64::__m256,
        ) -> core::arch::x86_64::__m256i {
            <X64V4Base as SealedBase>::f32x8_to_u32_fast(archmage::caps::extract_v4_caps(proof), a)
        }
        #[inline(always)]
        fn f16_to_f32_into_ops(proof: archmage::X64V4xToken, input: &[u16], output: &mut [f32]) {
            <X64V4Base as SealedBase>::f16_to_f32_into_ops(
                archmage::caps::extract_v4_caps(proof),
                input,
                output,
            );
        }
        #[inline(always)]
        fn f32_to_f16_into_ops(proof: archmage::X64V4xToken, input: &[f32], output: &mut [u16]) {
            <X64V4Base as SealedBase>::f32_to_f16_into_ops(
                archmage::caps::extract_v4_caps(proof),
                input,
                output,
            );
        }
        #[inline(always)]
        fn i32x4_dot_i8_i7_add(
            proof: archmage::X64V4xToken,
            acc: core::arch::x86_64::__m128i,
            a: core::arch::x86_64::__m128i,
            b: core::arch::x86_64::__m128i,
        ) -> core::arch::x86_64::__m128i {
            crate::simd::generic::quant_ops::v4x_i32x4_dot_i8(proof, acc, a, b)
        }
        #[inline(always)]
        fn i32x8_dot_i8_i7_add(
            proof: archmage::X64V4xToken,
            acc: core::arch::x86_64::__m256i,
            a: core::arch::x86_64::__m256i,
            b: core::arch::x86_64::__m256i,
        ) -> core::arch::x86_64::__m256i {
            crate::simd::generic::quant_ops::v4x_i32x8_dot_i8(proof, acc, a, b)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn i32x16_dot_i8_i7_add(
            proof: archmage::X64V4xToken,
            acc: core::arch::x86_64::__m512i,
            a: core::arch::x86_64::__m512i,
            b: core::arch::x86_64::__m512i,
        ) -> core::arch::x86_64::__m512i {
            crate::simd::generic::quant_ops::v4x_i32x16_dot_i8(proof, acc, a, b)
        }
    }

    #[cfg(target_arch = "aarch64")]
    pub struct NeonBase;

    #[cfg(target_arch = "aarch64")]
    impl SealedBase for NeonBase {
        type Narrow = archmage::NeonToken;
        #[cfg(feature = "w512")]
        type W512 = archmage::NeonToken;
        type MagetypesBackend = archmage::NeonToken;
        type Proof = archmage::NeonToken;
        const IS_SCALAR_BACKED: bool = false;
        impl_simd_types_assoc!(
            f32x4, f64x2, i8x16, i16x8, i32x4, i64x2, u8x16, u16x8, u32x4, u64x2, 4, 2
        );

        #[inline(always)]
        fn narrow(proof: archmage::NeonToken) -> Self::Narrow {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn w512(proof: archmage::NeonToken) -> Self::W512 {
            proof
        }
        #[inline(always)]
        fn magetypes_backend(proof: archmage::NeonToken) -> Self::MagetypesBackend {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_from_halves(
            _proof: archmage::NeonToken,
            lo: [core::arch::aarch64::float32x4_t; 2],
            hi: [core::arch::aarch64::float32x4_t; 2],
        ) -> [core::arch::aarch64::float32x4_t; 4] {
            [lo[0], lo[1], hi[0], hi[1]]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_low(
            _proof: archmage::NeonToken,
            wide: [core::arch::aarch64::float32x4_t; 4],
        ) -> [core::arch::aarch64::float32x4_t; 2] {
            [wide[0], wide[1]]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_high(
            _proof: archmage::NeonToken,
            wide: [core::arch::aarch64::float32x4_t; 4],
        ) -> [core::arch::aarch64::float32x4_t; 2] {
            [wide[2], wide[3]]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32(
            proof: archmage::NeonToken,
            a: [core::arch::aarch64::float64x2_t; 4],
        ) -> [core::arch::aarch64::int32x4_t; 2] {
            crate::simd::impls::arm_neon::neon_f64x8_to_i32(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_saturating(
            proof: archmage::NeonToken,
            a: [core::arch::aarch64::float64x2_t; 4],
        ) -> [core::arch::aarch64::int32x4_t; 2] {
            crate::simd::impls::arm_neon::neon_f64x8_to_i32(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_round(
            proof: archmage::NeonToken,
            a: [core::arch::aarch64::float64x2_t; 4],
        ) -> [core::arch::aarch64::int32x4_t; 2] {
            crate::simd::impls::arm_neon::neon_f64x8_to_i32_round(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_from_i32(
            proof: archmage::NeonToken,
            a: [core::arch::aarch64::int32x4_t; 2],
        ) -> [core::arch::aarch64::float64x2_t; 4] {
            crate::simd::impls::arm_neon::neon_f64x8_from_i32(proof, a)
        }
        #[inline(always)]
        fn f16_to_f32_into_ops(proof: archmage::NeonToken, input: &[u16], output: &mut [f32]) {
            crate::simd::generic::convert_f16::neon_f16_decode(proof, input, output);
        }
        #[inline(always)]
        fn f32_to_f16_into_ops(proof: archmage::NeonToken, input: &[f32], output: &mut [u16]) {
            crate::simd::generic::convert_f16::neon_f16_encode(proof, input, output);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub struct Wasm128Base;

    #[cfg(target_arch = "wasm32")]
    impl SealedBase for Wasm128Base {
        type Narrow = archmage::Wasm128Token;
        #[cfg(feature = "w512")]
        type W512 = archmage::Wasm128Token;
        type MagetypesBackend = archmage::Wasm128Token;
        type Proof = archmage::Wasm128Token;
        const IS_SCALAR_BACKED: bool = false;
        impl_simd_types_assoc!(
            f32x4, f64x2, i8x16, i16x8, i32x4, i64x2, u8x16, u16x8, u32x4, u64x2, 4, 2
        );

        #[inline(always)]
        fn narrow(proof: archmage::Wasm128Token) -> Self::Narrow {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn w512(proof: archmage::Wasm128Token) -> Self::W512 {
            proof
        }
        #[inline(always)]
        fn magetypes_backend(proof: archmage::Wasm128Token) -> Self::MagetypesBackend {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_from_halves(
            _proof: archmage::Wasm128Token,
            lo: [core::arch::wasm32::v128; 2],
            hi: [core::arch::wasm32::v128; 2],
        ) -> [core::arch::wasm32::v128; 4] {
            [lo[0], lo[1], hi[0], hi[1]]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_low(
            _proof: archmage::Wasm128Token,
            wide: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            [wide[0], wide[1]]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_high(
            _proof: archmage::Wasm128Token,
            wide: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            [wide[2], wide[3]]
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32(
            proof: archmage::Wasm128Token,
            a: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            crate::simd::impls::wasm128::wasm_f64x8_to_i32(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_saturating(
            proof: archmage::Wasm128Token,
            a: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            crate::simd::impls::wasm128::wasm_f64x8_to_i32(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_round(
            proof: archmage::Wasm128Token,
            a: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            crate::simd::impls::wasm128::wasm_f64x8_to_i32_round(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_from_i32(
            proof: archmage::Wasm128Token,
            a: [core::arch::wasm32::v128; 2],
        ) -> [core::arch::wasm32::v128; 4] {
            crate::simd::impls::wasm128::wasm_f64x8_from_i32(proof, a)
        }
        #[inline(always)]
        fn f16_to_f32_into_ops(proof: archmage::Wasm128Token, input: &[u16], output: &mut [f32]) {
            crate::simd::generic::convert_f16::f16_to_f32_slice_soft(proof, input, output);
        }
        #[inline(always)]
        fn f32_to_f16_into_ops(proof: archmage::Wasm128Token, input: &[f32], output: &mut [u16]) {
            crate::simd::generic::convert_f16::f32_to_f16_slice_soft(proof, input, output);
        }
    }

    #[cfg(all(
        target_arch = "wasm32",
        target_feature = "simd128",
        target_feature = "relaxed-simd"
    ))]
    pub struct Wasm128RelaxedFmaBase;

    #[cfg(all(
        target_arch = "wasm32",
        target_feature = "simd128",
        target_feature = "relaxed-simd"
    ))]
    impl SealedBase for Wasm128RelaxedFmaBase {
        type Narrow = archmage::Wasm128Token;
        #[cfg(feature = "w512")]
        type W512 = archmage::Wasm128Token;
        type MagetypesBackend = archmage::Wasm128RelaxedFmaToken;
        type Proof = archmage::Wasm128RelaxedFmaToken;
        const IS_SCALAR_BACKED: bool = false;
        impl_simd_types_assoc!(
            f32x4, f64x2, i8x16, i16x8, i32x4, i64x2, u8x16, u16x8, u32x4, u64x2, 4, 2
        );

        #[inline(always)]
        fn narrow(proof: archmage::Wasm128RelaxedFmaToken) -> Self::Narrow {
            archmage::caps::extract_wasm128_caps(proof)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn w512(proof: archmage::Wasm128RelaxedFmaToken) -> Self::W512 {
            archmage::caps::extract_wasm128_caps(proof)
        }
        #[inline(always)]
        fn magetypes_backend(proof: archmage::Wasm128RelaxedFmaToken) -> Self::MagetypesBackend {
            proof
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_from_halves(
            proof: archmage::Wasm128RelaxedFmaToken,
            lo: [core::arch::wasm32::v128; 2],
            hi: [core::arch::wasm32::v128; 2],
        ) -> [core::arch::wasm32::v128; 4] {
            <Wasm128Base as SealedBase>::f32x16_from_halves(
                archmage::caps::extract_wasm128_caps(proof),
                lo,
                hi,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_low(
            proof: archmage::Wasm128RelaxedFmaToken,
            wide: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            <Wasm128Base as SealedBase>::f32x16_low(
                archmage::caps::extract_wasm128_caps(proof),
                wide,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_high(
            proof: archmage::Wasm128RelaxedFmaToken,
            wide: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            <Wasm128Base as SealedBase>::f32x16_high(
                archmage::caps::extract_wasm128_caps(proof),
                wide,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32(
            proof: archmage::Wasm128RelaxedFmaToken,
            a: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            <Wasm128Base as SealedBase>::f64x8_convert_to_i32(
                archmage::caps::extract_wasm128_caps(proof),
                a,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_saturating(
            proof: archmage::Wasm128RelaxedFmaToken,
            a: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            <Wasm128Base as SealedBase>::f64x8_convert_to_i32_saturating(
                archmage::caps::extract_wasm128_caps(proof),
                a,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_to_i32_round(
            proof: archmage::Wasm128RelaxedFmaToken,
            a: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 2] {
            <Wasm128Base as SealedBase>::f64x8_convert_to_i32_round(
                archmage::caps::extract_wasm128_caps(proof),
                a,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f64x8_convert_from_i32(
            proof: archmage::Wasm128RelaxedFmaToken,
            a: [core::arch::wasm32::v128; 2],
        ) -> [core::arch::wasm32::v128; 4] {
            <Wasm128Base as SealedBase>::f64x8_convert_from_i32(
                archmage::caps::extract_wasm128_caps(proof),
                a,
            )
        }
        #[inline(always)]
        fn f16_to_f32_into_ops(
            proof: archmage::Wasm128RelaxedFmaToken,
            input: &[u16],
            output: &mut [f32],
        ) {
            <Wasm128Base as SealedBase>::f16_to_f32_into_ops(
                archmage::caps::extract_wasm128_caps(proof),
                input,
                output,
            );
        }
        #[inline(always)]
        fn f32_to_f16_into_ops(
            proof: archmage::Wasm128RelaxedFmaToken,
            input: &[f32],
            output: &mut [u16],
        ) {
            <Wasm128Base as SealedBase>::f32_to_f16_into_ops(
                archmage::caps::extract_wasm128_caps(proof),
                input,
                output,
            );
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_add(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f32x4(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_sub(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f32x4(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_add_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f32x4(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f32x4(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f32x4_min_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::min_fast_f32x4(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f32x4_max_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::max_fast_f32x4(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f32x4_mul_add_lane_relaxed<const LANE: i32>(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            v_in: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
            acc: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_lane_f32x4::<LANE>(
                proof, a, v_in, acc,
            )
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_add(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f32x8(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_sub(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f32x8(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_add_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f32x8(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f32x8(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f32x8_min_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::min_fast_f32x8(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f32x8_max_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::max_fast_f32x8(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f32x8_mul_add_lane_relaxed<const LANE: i32>(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            v_in: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
            acc: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_lane_f32x8::<LANE>(
                proof, a, v_in, acc,
            )
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_mul_add(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f32x16(proof, a, b, c)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_mul_sub(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f32x16(proof, a, b, c)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_mul_add_portable(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f32x16(proof, a, b, c)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f32x16(proof, a, b, c)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_min_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::min_fast_f32x16(proof, a, b)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_max_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::max_fast_f32x16(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f64x2_mul_add(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f64x2(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f64x2_mul_sub(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f64x2(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f64x2_mul_add_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f64x2(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f64x2_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f64x2(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f64x2_min_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::min_fast_f64x2(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f64x2_max_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x2Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::max_fast_f64x2(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f64x4_mul_add(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f64x4(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f64x4_mul_sub(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f64x4(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f64x4_mul_add_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f64x4(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f64x4_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            c: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f64x4(proof, a, b, c)
        }
        #[inline(always)]
        fn wasm_fma_f64x4_min_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::min_fast_f64x4(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f64x4_max_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::F64x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::max_fast_f64x4(proof, a, b)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_mul_add(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f64x8(proof, a, b, c)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_mul_sub(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f64x8(proof, a, b, c)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_mul_add_portable(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_add_f64x8(proof, a, b, c)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_mul_sub_portable(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            c: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_sub_f64x8(proof, a, b, c)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_min_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::min_fast_f64x8(proof, a, b)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f64x8_max_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::F64x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::max_fast_f64x8(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_f32x4_cvt_convert_f32_to_i32_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x4Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x4Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::convert_f32_to_i32_fast_x4(proof, a)
        }
        #[inline(always)]
        fn wasm_fma_f32x8_cvt_convert_f32_to_i32_fast(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::F32x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I32x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::convert_f32_to_i32_fast_x8(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_f32x16_cvt_convert_f32_to_i32_fast(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::F32x16Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::I32x16Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::convert_f32_to_i32_fast_x16(proof, a)
        }
        #[inline(always)]
        fn wasm_fma_i16x8_mul_hrs_q15(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I16x8Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_hrs_q15_x8(proof, a, b)
        }
        #[inline(always)]
        fn wasm_fma_i16x16_mul_hrs_q15(
            proof: Self::Proof,
            a: <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr,
            b: <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr,
        ) -> <Self::Narrow as crate::simd::backends::ops::I16x16Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_hrs_q15_x16(proof, a, b)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn wasm_fma_i16x32_mul_hrs_q15(
            proof: Self::Proof,
            a: <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr,
            b: <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr,
        ) -> <Self::W512 as crate::simd::backends::ops::I16x32Ops>::Repr {
            crate::simd::impls::wasm128_fma_overrides::mul_hrs_q15_x32(proof, a, b)
        }
        #[inline(always)]
        fn mul_add_lane4_relaxed_x8<const LANE: i32>(
            proof: archmage::Wasm128RelaxedFmaToken,
            a: [core::arch::wasm32::v128; 2],
            v_in: core::arch::wasm32::v128,
            acc: [core::arch::wasm32::v128; 2],
        ) -> [core::arch::wasm32::v128; 2] {
            crate::simd::generic::cross_width::wasm_fma_mul_add_lane4_relaxed::<LANE>(
                proof, a, v_in, acc,
            )
        }
        #[inline(always)]
        fn f32x4_to_u32_fast(
            proof: archmage::Wasm128RelaxedFmaToken,
            a: core::arch::wasm32::v128,
        ) -> core::arch::wasm32::v128 {
            crate::simd::generic::fast_ops::wasm_fma_f32x4_to_u32_fast(proof, a)
        }
        #[inline(always)]
        fn f32x8_to_u32_fast(
            proof: archmage::Wasm128RelaxedFmaToken,
            a: [core::arch::wasm32::v128; 2],
        ) -> [core::arch::wasm32::v128; 2] {
            crate::simd::generic::fast_ops::wasm_fma_f32x8_to_u32_fast(proof, a)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn f32x16_to_u32_fast(
            proof: archmage::Wasm128RelaxedFmaToken,
            a: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 4] {
            crate::simd::generic::fast_ops::wasm_fma_f32x16_to_u32_fast(proof, a)
        }
        #[inline(always)]
        fn i32x4_dot_i8_i7_add(
            proof: archmage::Wasm128RelaxedFmaToken,
            acc: core::arch::wasm32::v128,
            a: core::arch::wasm32::v128,
            b: core::arch::wasm32::v128,
        ) -> core::arch::wasm32::v128 {
            crate::simd::generic::quant_ops::wasm_fma_i32x4_dot_i8(proof, acc, a, b)
        }
        #[inline(always)]
        fn i32x8_dot_i8_i7_add(
            proof: archmage::Wasm128RelaxedFmaToken,
            acc: [core::arch::wasm32::v128; 2],
            a: [core::arch::wasm32::v128; 2],
            b: [core::arch::wasm32::v128; 2],
        ) -> [core::arch::wasm32::v128; 2] {
            crate::simd::generic::quant_ops::wasm_fma_i32x8_dot_i8(proof, acc, a, b)
        }
        #[cfg(feature = "w512")]
        #[inline(always)]
        fn i32x16_dot_i8_i7_add(
            proof: archmage::Wasm128RelaxedFmaToken,
            acc: [core::arch::wasm32::v128; 4],
            a: [core::arch::wasm32::v128; 4],
            b: [core::arch::wasm32::v128; 4],
        ) -> [core::arch::wasm32::v128; 4] {
            crate::simd::generic::quant_ops::wasm_fma_i32x16_dot_i8(proof, acc, a, b)
        }
    }
}

#[allow(clippy::type_complexity)]
impl<T> crate::simd::backends::sealed::seal_cap::SealedFromCaps<T> for archmage::caps::ScalarCaps
where
    T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
{
    type Base = crate::simd::backends::sealed::seal_base_impls::ScalarBase;
    #[inline(always)]
    fn into_proof(token: T) -> archmage::ScalarToken {
        let _ = token;
        archmage::ScalarToken
    }
}

#[cfg(target_arch = "x86_64")]
mod x64_cap_sealed {
    #[allow(clippy::type_complexity)]
    impl<
        T,
        C0: archmage::caps::Flag,
        C1: archmage::caps::Flag,
        C2: archmage::caps::Flag,
        C4: archmage::caps::Flag,
        C5: archmage::caps::Flag,
        C6: archmage::caps::Flag,
        C7: archmage::caps::Flag,
        C8: archmage::caps::Flag,
        C9: archmage::caps::Flag,
        C10: archmage::caps::Flag,
        C11: archmage::caps::Flag,
        C12: archmage::caps::Flag,
        C13: archmage::caps::Flag,
        C14: archmage::caps::Flag,
        C15: archmage::caps::Flag,
        C16: archmage::caps::Flag,
        C17: archmage::caps::Flag,
    > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
        for archmage::caps::X64CapSet<
            C0,
            C1,
            C2,
            archmage::caps::No,
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
        >
    where
        T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
    {
        type Base = crate::simd::backends::sealed::seal_base_impls::ScalarBase;
        #[inline(always)]
        fn into_proof(token: T) -> archmage::ScalarToken {
            let _ = token;
            archmage::ScalarToken
        }
    }

    #[allow(clippy::type_complexity)]
    impl<
        T,
        C1: archmage::caps::Flag,
        C2: archmage::caps::Flag,
        C4: archmage::caps::Flag,
        C5: archmage::caps::Flag,
        C6: archmage::caps::Flag,
        C7: archmage::caps::Flag,
        C8: archmage::caps::Flag,
        C9: archmage::caps::Flag,
        C10: archmage::caps::Flag,
        C11: archmage::caps::Flag,
        C12: archmage::caps::Flag,
        C13: archmage::caps::Flag,
        C14: archmage::caps::Flag,
        C15: archmage::caps::Flag,
        C16: archmage::caps::Flag,
        C17: archmage::caps::Flag,
    > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
        for archmage::caps::X64CapSet<
            archmage::caps::No,
            C1,
            C2,
            archmage::caps::Yes,
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
        >
    where
        T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
    {
        type Base = crate::simd::backends::sealed::seal_base_impls::ScalarBase;
        #[inline(always)]
        fn into_proof(token: T) -> archmage::ScalarToken {
            let _ = token;
            archmage::ScalarToken
        }
    }

    #[allow(clippy::type_complexity)]
    impl<
        T,
        C2: archmage::caps::Flag,
        C4: archmage::caps::Flag,
        C5: archmage::caps::Flag,
        C6: archmage::caps::Flag,
        C7: archmage::caps::Flag,
        C8: archmage::caps::Flag,
        C9: archmage::caps::Flag,
        C10: archmage::caps::Flag,
        C11: archmage::caps::Flag,
        C12: archmage::caps::Flag,
        C13: archmage::caps::Flag,
        C14: archmage::caps::Flag,
        C15: archmage::caps::Flag,
        C16: archmage::caps::Flag,
        C17: archmage::caps::Flag,
    > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
        for archmage::caps::X64CapSet<
            archmage::caps::Yes,
            archmage::caps::No,
            C2,
            archmage::caps::Yes,
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
        >
    where
        T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
    {
        type Base = crate::simd::backends::sealed::seal_base_impls::ScalarBase;
        #[inline(always)]
        fn into_proof(token: T) -> archmage::ScalarToken {
            let _ = token;
            archmage::ScalarToken
        }
    }

    #[cfg(not(feature = "avx512"))]
    mod no_avx512_cap_sealed {
        #[allow(clippy::type_complexity)]
        impl<
            T,
            C2: archmage::caps::Flag,
            C4: archmage::caps::Flag,
            C5: archmage::caps::Flag,
            C6: archmage::caps::Flag,
            C7: archmage::caps::Flag,
            C8: archmage::caps::Flag,
            C9: archmage::caps::Flag,
            C10: archmage::caps::Flag,
            C11: archmage::caps::Flag,
            C12: archmage::caps::Flag,
            C13: archmage::caps::Flag,
            C14: archmage::caps::Flag,
            C15: archmage::caps::Flag,
            C16: archmage::caps::Flag,
            C17: archmage::caps::Flag,
        > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::X64CapSet<
                archmage::caps::Yes,
                archmage::caps::Yes,
                C2,
                archmage::caps::Yes,
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
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::X64V3Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::X64V3Token {
                archmage::caps::extract_v3_caps(token)
            }
        }
    }
    #[cfg(feature = "avx512")]
    mod avx512_cap_sealed {
        #[allow(clippy::type_complexity)]
        impl<
            T,
            C2: archmage::caps::Flag,
            C4: archmage::caps::Flag,
            C5: archmage::caps::Flag,
            C6: archmage::caps::Flag,
            C7: archmage::caps::Flag,
            C8: archmage::caps::Flag,
            C9: archmage::caps::Flag,
            C10: archmage::caps::Flag,
            C11: archmage::caps::Flag,
            C12: archmage::caps::Flag,
            C13: archmage::caps::Flag,
            C15: archmage::caps::Flag,
            C16: archmage::caps::Flag,
            C17: archmage::caps::Flag,
        > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::X64CapSet<
                archmage::caps::Yes,
                archmage::caps::Yes,
                C2,
                archmage::caps::Yes,
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
                archmage::caps::No,
                C15,
                C16,
                C17,
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::X64V3Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::X64V3Token {
                archmage::caps::extract_v3_caps(token)
            }
        }

        #[allow(clippy::type_complexity)]
        impl<
            T,
            C4: archmage::caps::Flag,
            C5: archmage::caps::Flag,
            C6: archmage::caps::Flag,
            C7: archmage::caps::Flag,
            C8: archmage::caps::Flag,
            C9: archmage::caps::Flag,
            C10: archmage::caps::Flag,
            C11: archmage::caps::Flag,
            C12: archmage::caps::Flag,
            C13: archmage::caps::Flag,
            C15: archmage::caps::Flag,
            C16: archmage::caps::Flag,
            C17: archmage::caps::Flag,
        > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::X64CapSet<
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::No,
                archmage::caps::Yes,
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
                archmage::caps::Yes,
                C15,
                C16,
                C17,
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::X64V3Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::X64V3Token {
                archmage::caps::extract_v3_caps(token)
            }
        }

        #[allow(clippy::type_complexity)]
        impl<
            T,
            C4: archmage::caps::Flag,
            C5: archmage::caps::Flag,
            C6: archmage::caps::Flag,
            C7: archmage::caps::Flag,
            C8: archmage::caps::Flag,
            C9: archmage::caps::Flag,
            C10: archmage::caps::Flag,
            C11: archmage::caps::Flag,
            C12: archmage::caps::Flag,
            C13: archmage::caps::Flag,
            C16: archmage::caps::Flag,
            C17: archmage::caps::Flag,
        > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::X64CapSet<
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
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
                archmage::caps::Yes,
                archmage::caps::No,
                C16,
                C17,
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::X64V4Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::X64V4Token {
                archmage::caps::extract_v4_caps(token)
            }
        }

        #[allow(clippy::type_complexity)]
        impl<
            T,
            C4: archmage::caps::Flag,
            C6: archmage::caps::Flag,
            C7: archmage::caps::Flag,
            C8: archmage::caps::Flag,
            C9: archmage::caps::Flag,
            C10: archmage::caps::Flag,
            C11: archmage::caps::Flag,
            C12: archmage::caps::Flag,
            C13: archmage::caps::Flag,
            C16: archmage::caps::Flag,
            C17: archmage::caps::Flag,
        > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::X64CapSet<
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                C4,
                archmage::caps::No,
                C6,
                C7,
                C8,
                C9,
                C10,
                C11,
                C12,
                C13,
                archmage::caps::Yes,
                archmage::caps::Yes,
                C16,
                C17,
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::X64V4Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::X64V4Token {
                archmage::caps::extract_v4_caps(token)
            }
        }

        #[allow(clippy::type_complexity)]
        impl<
            T,
            C6: archmage::caps::Flag,
            C7: archmage::caps::Flag,
            C8: archmage::caps::Flag,
            C9: archmage::caps::Flag,
            C10: archmage::caps::Flag,
            C11: archmage::caps::Flag,
            C12: archmage::caps::Flag,
            C13: archmage::caps::Flag,
            C16: archmage::caps::Flag,
            C17: archmage::caps::Flag,
        > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::X64CapSet<
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::No,
                archmage::caps::Yes,
                C6,
                C7,
                C8,
                C9,
                C10,
                C11,
                C12,
                C13,
                archmage::caps::Yes,
                archmage::caps::Yes,
                C16,
                C17,
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::X64V4Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::X64V4Token {
                archmage::caps::extract_v4_caps(token)
            }
        }

        #[allow(clippy::type_complexity)]
        impl<
            T,
            C6: archmage::caps::Flag,
            C7: archmage::caps::Flag,
            C8: archmage::caps::Flag,
            C9: archmage::caps::Flag,
            C10: archmage::caps::Flag,
            C11: archmage::caps::Flag,
            C12: archmage::caps::Flag,
            C13: archmage::caps::Flag,
            C16: archmage::caps::Flag,
            C17: archmage::caps::Flag,
        > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::X64CapSet<
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
                C6,
                C7,
                C8,
                C9,
                C10,
                C11,
                C12,
                C13,
                archmage::caps::Yes,
                archmage::caps::Yes,
                C16,
                C17,
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::X64V4xBase;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::X64V4xToken {
                archmage::caps::extract_v4x_caps(token)
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
mod arm_cap_sealed {
    #[allow(clippy::type_complexity)]
    impl<
        T,
        C1: archmage::caps::Flag,
        C2: archmage::caps::Flag,
        C3: archmage::caps::Flag,
        C4: archmage::caps::Flag,
        C5: archmage::caps::Flag,
        C6: archmage::caps::Flag,
        C7: archmage::caps::Flag,
        C8: archmage::caps::Flag,
        C9: archmage::caps::Flag,
        C10: archmage::caps::Flag,
        C11: archmage::caps::Flag,
    > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
        for archmage::caps::ArmCapSet<
            archmage::caps::No,
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
        >
    where
        T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
    {
        type Base = crate::simd::backends::sealed::seal_base_impls::ScalarBase;
        #[inline(always)]
        fn into_proof(token: T) -> archmage::ScalarToken {
            let _ = token;
            archmage::ScalarToken
        }
    }

    #[allow(clippy::type_complexity)]
    impl<
        T,
        C1: archmage::caps::Flag,
        C2: archmage::caps::Flag,
        C3: archmage::caps::Flag,
        C4: archmage::caps::Flag,
        C5: archmage::caps::Flag,
        C6: archmage::caps::Flag,
        C7: archmage::caps::Flag,
        C8: archmage::caps::Flag,
        C9: archmage::caps::Flag,
        C10: archmage::caps::Flag,
        C11: archmage::caps::Flag,
    > crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
        for archmage::caps::ArmCapSet<
            archmage::caps::Yes,
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
        >
    where
        T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
    {
        type Base = crate::simd::backends::sealed::seal_base_impls::NeonBase;
        #[inline(always)]
        fn into_proof(token: T) -> archmage::NeonToken {
            archmage::caps::extract_neon_caps(token)
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_cap_sealed {
    #[allow(clippy::type_complexity)]
    impl<T, C1: archmage::caps::Flag, C2: archmage::caps::Flag>
        crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
        for archmage::caps::WasmCapSet<archmage::caps::No, C1, C2>
    where
        T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
    {
        type Base = crate::simd::backends::sealed::seal_base_impls::ScalarBase;
        #[inline(always)]
        fn into_proof(token: T) -> archmage::ScalarToken {
            let _ = token;
            archmage::ScalarToken
        }
    }

    #[cfg(not(all(target_feature = "simd128", target_feature = "relaxed-simd")))]
    mod wasm_base_cap_sealed {
        #[allow(clippy::type_complexity)]
        impl<T, C1: archmage::caps::Flag, C2: archmage::caps::Flag>
            crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::WasmCapSet<archmage::caps::Yes, C1, C2>
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::Wasm128Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::Wasm128Token {
                archmage::caps::extract_wasm128_caps(token)
            }
        }
    }
    #[cfg(all(target_feature = "simd128", target_feature = "relaxed-simd"))]
    mod wasm_relaxed_cap_sealed {
        #[allow(clippy::type_complexity)]
        impl<T, C1: archmage::caps::Flag> crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::WasmCapSet<archmage::caps::Yes, C1, archmage::caps::No>
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::Wasm128Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::Wasm128Token {
                archmage::caps::extract_wasm128_caps(token)
            }
        }

        #[allow(clippy::type_complexity)]
        impl<T> crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::WasmCapSet<
                archmage::caps::Yes,
                archmage::caps::No,
                archmage::caps::Yes,
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::Wasm128Base;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::Wasm128Token {
                archmage::caps::extract_wasm128_caps(token)
            }
        }

        #[allow(clippy::type_complexity)]
        impl<T> crate::simd::backends::sealed::seal_cap::SealedFromCaps<T>
            for archmage::caps::WasmCapSet<
                archmage::caps::Yes,
                archmage::caps::Yes,
                archmage::caps::Yes,
            >
        where
            T: archmage::SimdToken<__Caps = Self> + archmage::IntoConcreteToken + Copy + 'static,
        {
            type Base = crate::simd::backends::sealed::seal_base_impls::Wasm128RelaxedFmaBase;
            #[inline(always)]
            fn into_proof(token: T) -> archmage::Wasm128RelaxedFmaToken {
                archmage::caps::extract_wasm128_relaxed_fma_caps(token)
            }
        }
    }
}
