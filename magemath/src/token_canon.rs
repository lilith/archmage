//! Canonical token normalization for `magemath` dispatch.
//!
//! Subtier, crypto, and [`archmage::And`] conjunction tokens (`X64V3CryptoToken`,
//! `Arm64V2Token`, `Wasm128RelaxedToken`, `And<X64V3Token, X64GfniToken>`, etc.)
//! project via zero-cost [`MagetypesToken::magetypes`] and [`IntoConcreteToken`]
//! onto the canonical base SIMD proof (`X64V3Token`, `NeonToken`, `Wasm128Token`,
//! or scalar) without `TypeId`, `&dyn Any`, or duplicate `graph::*` instantiations.

/// Canonical SIMD execution proof projected from a [`magetypes::simd::MagetypesToken`].
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128"),
))]
#[derive(Clone, Copy)]
pub(crate) enum CanonicalToken {
    #[cfg(target_arch = "x86_64")]
    V3(archmage::X64V3Token),
    #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
    V4(archmage::X64V4Token),
    #[cfg(all(target_arch = "x86_64", feature = "avx512"))]
    V4x(archmage::X64V4xToken),
    #[cfg(target_arch = "aarch64")]
    Neon(archmage::NeonToken),
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    Wasm {
        token: archmage::Wasm128Token,
        /// `true` only when the token (or conjunction) includes `Wasm128RelaxedFmaToken`,
        /// which verifies single-rounding `f32x4` and `f64x2` relaxed FMA at runtime.
        relaxed_fma: bool,
    },
    Scalar,
}

/// Map any [`magetypes::simd::MagetypesToken`] onto its canonical native proof, if any.
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128"),
))]
#[inline(always)]
pub(crate) fn canonicalize<T: magetypes::simd::MagetypesToken>(token: T) -> CanonicalToken {
    use archmage::IntoConcreteToken;

    let backend = token.magetypes();
    #[cfg(target_arch = "x86_64")]
    {
        if let Some(t) = backend.as_x64v3() {
            return CanonicalToken::V3(t);
        }
        #[cfg(feature = "avx512")]
        {
            if let Some(t) = backend.as_x64v4() {
                return CanonicalToken::V4(t);
            }
            if let Some(t) = backend.as_x64v4x() {
                return CanonicalToken::V4x(t);
            }
        }
        #[cfg(not(feature = "avx512"))]
        {
            // If a sibling crate enabled `magetypes/avx512` while `magemath/avx512`
            // is off, `token.magetypes()` yields `X64V4Token` / `X64V4xToken`.
            // Downgrade via `.v3()` so callers stay on hardware V3 instead of scalar.
            if let Some(t) = backend.as_x64v4() {
                return CanonicalToken::V3(t.v3());
            }
            if let Some(t) = backend.as_x64v4x() {
                return CanonicalToken::V3(t.v3());
            }
        }
    }
    #[cfg(target_arch = "aarch64")]
    if let Some(t) = backend.as_neon() {
        return CanonicalToken::Neon(t);
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        if let Some(t) = backend.as_wasm128() {
            return CanonicalToken::Wasm {
                token: t,
                relaxed_fma: (T::__ARCHMAGE_CONTRACT_BITS & 1) != 0,
            };
        }
        #[cfg(target_feature = "relaxed-simd")]
        if let Some(t) = backend.as_wasm128_relaxed_fma() {
            return CanonicalToken::Wasm {
                token: t.wasm128(),
                relaxed_fma: (T::__ARCHMAGE_CONTRACT_BITS & 1) != 0,
            };
        }
    }
    CanonicalToken::Scalar
}

/// Returns `true` when `T` backs `magetypes` vector ops with `ScalarToken`.
#[inline(always)]
pub(crate) fn is_scalar_backed<T: magetypes::simd::MagetypesToken>() -> bool {
    T::__IS_SCALAR_BACKED
}

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    use super::*;
    use archmage::SimdToken;

    #[test]
    fn canonicalize_v4_and_v4x_never_degrade_to_scalar() {
        if let Some(v4) = archmage::X64V4Token::summon() {
            let c = canonicalize(v4);
            #[cfg(feature = "avx512")]
            assert!(matches!(c, CanonicalToken::V4(_)));
            #[cfg(not(feature = "avx512"))]
            assert!(matches!(c, CanonicalToken::V3(_)));
        }
        if let Some(v4x) = archmage::X64V4xToken::summon() {
            let c = canonicalize(v4x);
            #[cfg(feature = "avx512")]
            assert!(matches!(c, CanonicalToken::V4x(_)));
            #[cfg(not(feature = "avx512"))]
            assert!(matches!(c, CanonicalToken::V3(_)));
        }
    }
}
