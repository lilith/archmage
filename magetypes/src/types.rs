//! SimdTypes trait - associates SIMD types with tokens.
//!
//! This trait allows generic programming over token types by providing
//! associated types for each SIMD element type.

use archmage::SimdToken;

/// Trait that associates SIMD vector types with a capability token.
///
/// This enables generic programming where the vector width is determined
/// by the token type.
///
/// # Example
///
/// ```rust,ignore
/// use archmage::{X64V3Token, SimdToken};
/// use magetypes::SimdTypes;
///
/// fn sum<T: SimdToken + SimdTypes>(token: T, data: &[f32]) -> f32 {
///     // Use token's associated F32 type
///     let zero = <T as SimdTypes>::F32::splat(token, 0.0);
///     // ...
/// }
/// ```
pub trait SimdTypes: SimdToken {
    /// 32-bit floating point vector (e.g., f32x8 for AVX2)
    type F32;
    /// 64-bit floating point vector (e.g., f64x4 for AVX2)
    type F64;
    /// 8-bit signed integer vector
    type I8;
    /// 16-bit signed integer vector
    type I16;
    /// 32-bit signed integer vector
    type I32;
    /// 64-bit signed integer vector
    type I64;
    /// 8-bit unsigned integer vector
    type U8;
    /// 16-bit unsigned integer vector
    type U16;
    /// 32-bit unsigned integer vector
    type U32;
    /// 64-bit unsigned integer vector
    type U64;

    /// Number of f32 lanes in the F32 type
    const F32_LANES: usize;
    /// Number of f64 lanes in the F64 type
    const F64_LANES: usize;
    /// Number of i32 lanes in the I32 type
    const I32_LANES: usize;
}

// Implementations of `SimdTypes` for all concrete tokens and `archmage::And<Base, Extra>`
// conjunctions are generated in `crate::simd::backends::sealed` via `SealedFromCaps`.

#[cfg(test)]
mod tests {
    use super::*;
    use core::any::TypeId;

    #[test]
    fn scalar_token_types() {
        // ScalarToken and ScalarToken.and(ScalarToken) associate with scalar types.
        fn check<T: SimdTypes>() {
            assert_eq!(T::F32_LANES, 1);
            assert_eq!(T::F64_LANES, 1);
            assert_eq!(T::I32_LANES, 1);
        }
        check::<archmage::ScalarToken>();
        check::<archmage::And<archmage::ScalarToken, archmage::ScalarToken>>();
        assert_eq!(
            TypeId::of::<
                <archmage::And<archmage::ScalarToken, archmage::ScalarToken> as SimdTypes>::F32,
            >(),
            TypeId::of::<f32>(),
        );
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn x86_all_leaf_and_conjunction_tokens_implement_simd_types() {
        fn check_lanes<T: SimdTypes>(expected_f32: usize, expected_f64: usize) {
            assert_eq!(T::F32_LANES, expected_f32);
            assert_eq!(T::F64_LANES, expected_f64);
            assert_eq!(T::I32_LANES, expected_f32);
        }

        // Scalar-backed baseline & standalone x86 extension leaf tokens
        check_lanes::<archmage::X64V1Token>(1, 1);
        check_lanes::<archmage::X64V2Token>(1, 1);
        check_lanes::<archmage::X64CryptoToken>(1, 1);
        check_lanes::<archmage::X64GfniToken>(1, 1);
        check_lanes::<archmage::X64ShaToken>(1, 1);
        check_lanes::<archmage::X64AdxToken>(1, 1);
        check_lanes::<archmage::X64AvxVnniToken>(1, 1);
        check_lanes::<archmage::X64AvxIfmaToken>(1, 1);
        check_lanes::<archmage::X64AvxVnniInt8Token>(1, 1);
        check_lanes::<archmage::X64AvxVnniInt16Token>(1, 1);
        check_lanes::<archmage::X64AvxNeConvertToken>(1, 1);
        check_lanes::<archmage::X64Sha512Token>(1, 1);

        // V3-backed tokens (256-bit)
        check_lanes::<archmage::X64V3Token>(8, 4);
        check_lanes::<archmage::X64V3CryptoToken>(8, 4);
        check_lanes::<archmage::X64V3GfniCryptoToken>(8, 4);
        check_lanes::<archmage::X64V3VnniToken>(8, 4);
        check_lanes::<archmage::X64V3Vnni2Token>(8, 4);

        // V4-backed tokens (512-bit when avx512 enabled, 256-bit otherwise)
        #[cfg(feature = "avx512")]
        let (v4_32, v4_64) = (16, 8);
        #[cfg(not(feature = "avx512"))]
        let (v4_32, v4_64) = (8, 4);
        check_lanes::<archmage::X64V4Token>(v4_32, v4_64);
        check_lanes::<archmage::X64V4xToken>(v4_32, v4_64);
        check_lanes::<archmage::Avx512Fp16Token>(v4_32, v4_64);
        check_lanes::<archmage::X64Avx512Bf16Token>(v4_32, v4_64);

        // Conjunctions inherit the full joined capability set
        type V3Gfni = archmage::And<archmage::X64V3Token, archmage::X64GfniToken>;
        check_lanes::<V3Gfni>(8, 4);
        assert_eq!(
            TypeId::of::<<V3Gfni as SimdTypes>::F32>(),
            TypeId::of::<crate::simd::generic::f32x8<V3Gfni>>(),
        );
        check_lanes::<archmage::And<archmage::X64V1Token, archmage::X64V3Token>>(8, 4);
        check_lanes::<archmage::And<archmage::X64V3Token, archmage::X64V4Token>>(v4_32, v4_64);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn arm_all_leaf_and_conjunction_tokens_implement_simd_types() {
        fn check_lanes<T: SimdTypes>() {
            assert_eq!(T::F32_LANES, 4);
            assert_eq!(T::F64_LANES, 2);
            assert_eq!(T::I32_LANES, 4);
        }
        check_lanes::<archmage::NeonToken>();
        check_lanes::<archmage::NeonAesToken>();
        check_lanes::<archmage::NeonSha2Token>();
        check_lanes::<archmage::NeonSha3Token>();
        check_lanes::<archmage::NeonCrcToken>();
        check_lanes::<archmage::NeonRdmToken>();
        check_lanes::<archmage::NeonDotProdToken>();
        check_lanes::<archmage::NeonFp16Token>();
        check_lanes::<archmage::NeonFcmaToken>();
        check_lanes::<archmage::NeonI8mmToken>();
        check_lanes::<archmage::NeonBf16Token>();
        check_lanes::<archmage::NeonFhmToken>();
        check_lanes::<archmage::Arm64V2Token>();
        check_lanes::<archmage::Arm64V3Token>();
        check_lanes::<archmage::And<archmage::NeonToken, archmage::NeonFp16Token>>();
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    fn wasm_all_leaf_and_conjunction_tokens_implement_simd_types() {
        fn check_lanes<T: SimdTypes>() {
            assert_eq!(T::F32_LANES, 4);
            assert_eq!(T::F64_LANES, 2);
            assert_eq!(T::I32_LANES, 4);
        }
        check_lanes::<archmage::Wasm128Token>();
        check_lanes::<archmage::Wasm128RelaxedToken>();
        check_lanes::<archmage::Wasm128RelaxedFmaToken>();
        check_lanes::<archmage::And<archmage::Wasm128Token, archmage::Wasm128RelaxedFmaToken>>();
    }
}
