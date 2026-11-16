use core::mem::size_of;
use std::collections::BTreeSet;

use archmage::{
    And, Arm64V2Token, Arm64V3Token, Avx512Fp16Token, HasArm64V2, HasAvxIfma, HasAvxVnni, HasGfni,
    HasNeon, HasNeonAes, HasNeonCrc, HasNeonDotProd, HasNeonFhm, HasNeonFp16, HasWasm128,
    HasWasm128Relaxed, HasWasm128RelaxedFma, HasX64V1, HasX64V2, HasX64V3, HasX64V3Crypto,
    HasX64V4, HasX64V4x, IntoConcreteToken, NeonAesToken, NeonBf16Token, NeonCrcToken,
    NeonDotProdToken, NeonFcmaToken, NeonFhmToken, NeonFp16Token, NeonI8mmToken, NeonRdmToken,
    NeonSha2Token, NeonSha3Token, NeonToken, ScalarToken, SimdLeafToken, SimdToken,
    Wasm128RelaxedFmaToken, Wasm128RelaxedToken, Wasm128Token, X64AdxToken, X64Avx512Bf16Token,
    X64AvxIfmaToken, X64AvxNeConvertToken, X64AvxVnniInt8Token, X64AvxVnniInt16Token,
    X64AvxVnniToken, X64CryptoToken, X64GfniToken, X64Sha512Token, X64ShaToken, X64V1Token,
    X64V2Token, X64V3CryptoToken, X64V3GfniCryptoToken, X64V3Token, X64V3Vnni2Token,
    X64V3VnniToken, X64V4Token, X64V4xToken,
};

fn check_leaf<T: SimdLeafToken + IntoConcreteToken>(expected_name: &str) {
    assert_eq!(size_of::<T>(), 0);
    assert_eq!(T::NAME, expected_name);
    let _ = T::compiled_with();
    let _ = T::summon();
    let _ = T::attempt();
    if let Some(tok) = T::summon() {
        assert_eq!(tok.name(), expected_name);
        assert_eq!(tok.to_string(), expected_name);
    }
}

fn check_conj<T: SimdToken>() {
    assert_eq!(size_of::<T>(), 0);
    let _ = T::compiled_with();
    let _ = T::summon();
    let _ = T::attempt();
}

#[test]
fn all_leaf_tokens_and_clumps_satisfy_contracts() {
    // Universal
    check_leaf::<ScalarToken>(ScalarToken::NAME);
    assert_eq!(ScalarToken.as_scalar(), Some(ScalarToken));

    // All 21 x86 tokens (18 capability clumps)
    check_leaf::<X64V1Token>(X64V1Token::NAME);
    check_leaf::<X64V2Token>(X64V2Token::NAME);
    check_leaf::<X64CryptoToken>(X64CryptoToken::NAME);
    check_leaf::<X64V3Token>(X64V3Token::NAME);
    check_leaf::<X64V3CryptoToken>(X64V3CryptoToken::NAME);
    check_leaf::<X64V3GfniCryptoToken>(X64V3GfniCryptoToken::NAME);
    check_leaf::<X64V3VnniToken>(X64V3VnniToken::NAME);
    check_leaf::<X64V3Vnni2Token>(X64V3Vnni2Token::NAME);
    check_leaf::<X64V4Token>(X64V4Token::NAME);
    check_leaf::<Avx512Fp16Token>(Avx512Fp16Token::NAME);
    check_leaf::<X64Avx512Bf16Token>(X64Avx512Bf16Token::NAME);
    check_leaf::<X64V4xToken>(X64V4xToken::NAME);
    check_leaf::<X64GfniToken>(X64GfniToken::NAME);
    check_leaf::<X64ShaToken>(X64ShaToken::NAME);
    check_leaf::<X64AdxToken>(X64AdxToken::NAME);
    check_leaf::<X64AvxVnniToken>(X64AvxVnniToken::NAME);
    check_leaf::<X64AvxIfmaToken>(X64AvxIfmaToken::NAME);
    check_leaf::<X64AvxVnniInt8Token>(X64AvxVnniInt8Token::NAME);
    check_leaf::<X64AvxVnniInt16Token>(X64AvxVnniInt16Token::NAME);
    check_leaf::<X64AvxNeConvertToken>(X64AvxNeConvertToken::NAME);
    check_leaf::<X64Sha512Token>(X64Sha512Token::NAME);

    // All 14 ARM tokens (12 capability clumps)
    check_leaf::<NeonToken>(NeonToken::NAME);
    check_leaf::<NeonAesToken>(NeonAesToken::NAME);
    check_leaf::<NeonSha2Token>(NeonSha2Token::NAME);
    check_leaf::<NeonSha3Token>(NeonSha3Token::NAME);
    check_leaf::<NeonCrcToken>(NeonCrcToken::NAME);
    check_leaf::<NeonRdmToken>(NeonRdmToken::NAME);
    check_leaf::<NeonDotProdToken>(NeonDotProdToken::NAME);
    check_leaf::<NeonFp16Token>(NeonFp16Token::NAME);
    check_leaf::<NeonFcmaToken>(NeonFcmaToken::NAME);
    check_leaf::<NeonI8mmToken>(NeonI8mmToken::NAME);
    check_leaf::<NeonBf16Token>(NeonBf16Token::NAME);
    check_leaf::<NeonFhmToken>(NeonFhmToken::NAME);
    check_leaf::<Arm64V2Token>(Arm64V2Token::NAME);
    check_leaf::<Arm64V3Token>(Arm64V3Token::NAME);

    // All 3 WASM tokens
    check_leaf::<Wasm128Token>(Wasm128Token::NAME);
    check_leaf::<Wasm128RelaxedToken>(Wasm128RelaxedToken::NAME);
    check_leaf::<Wasm128RelaxedFmaToken>(Wasm128RelaxedFmaToken::NAME);

    // Non-matching arch IntoConcreteToken returns None cleanly even for ScalarToken
    assert!(ScalarToken.as_x64v3().is_none());
    assert!(ScalarToken.as_neon().is_none());
    assert!(ScalarToken.as_wasm128().is_none());
}

#[test]
fn and_join_chain_display_dedup_and_inherited_bounds() {
    fn assert_x86_v3_gfni<T: SimdToken + HasX64V1 + HasX64V2 + HasX64V3 + HasGfni>() {}
    fn assert_x86_vnni_ifma<T: SimdToken + HasX64V3 + HasAvxVnni + HasAvxIfma>() {}
    fn assert_x86_v4x<T: SimdToken + HasX64V4 + HasX64V4x + HasGfni + HasX64V3Crypto>() {}
    fn assert_arm_dot<T: SimdToken + HasNeon + HasNeonDotProd>() {}
    fn assert_arm_multi<T: SimdToken + HasNeon + HasNeonAes + HasNeonCrc>() {}
    fn assert_arm_v2_fhm<T: SimdToken + HasArm64V2 + HasNeonFp16 + HasNeonFhm>() {}
    fn assert_wasm_relaxed<T: SimdToken + HasWasm128 + HasWasm128Relaxed>() {}
    fn assert_wasm_fma<T: SimdToken + HasWasm128 + HasWasm128Relaxed + HasWasm128RelaxedFma>() {}

    assert_x86_v3_gfni::<And<X64V3Token, X64GfniToken>>();
    assert_x86_vnni_ifma::<And<X64V3Token, And<X64AvxVnniToken, X64AvxIfmaToken>>>();
    assert_x86_vnni_ifma::<And<And<X64V3Token, X64AvxVnniToken>, X64AvxIfmaToken>>();
    assert_x86_v4x::<And<X64V4Token, X64V4xToken>>();

    assert_arm_dot::<And<NeonToken, NeonDotProdToken>>();
    assert_arm_multi::<And<NeonToken, And<NeonAesToken, NeonCrcToken>>>();
    assert_arm_v2_fhm::<And<Arm64V2Token, NeonFhmToken>>();

    assert_wasm_relaxed::<And<Wasm128Token, Wasm128RelaxedToken>>();
    assert_wasm_fma::<And<Wasm128RelaxedToken, Wasm128RelaxedFmaToken>>();

    check_conj::<And<X64V3Token, X64GfniToken>>();
    check_conj::<And<NeonToken, NeonDotProdToken>>();
    check_conj::<And<Wasm128Token, Wasm128RelaxedToken>>();

    // Deduplication in visit_target_features and Display formatting
    let mut visited = Vec::new();
    And::<X64V3Token, X64V3CryptoToken>::visit_target_features(&mut |feat| visited.push(feat));
    let unique: BTreeSet<&'static str> = visited.iter().copied().collect();
    assert_eq!(
        visited.len(),
        unique.len(),
        "visit_target_features emitted duplicate features: {visited:?}"
    );
    assert!(And::<X64V3Token, X64V3CryptoToken>::has_target_feature(
        "avx2"
    ));
    assert!(And::<X64V3Token, X64V3CryptoToken>::has_target_feature(
        "vpclmulqdq"
    ));

    // Runtime join / chain / base / extra / Display when native tokens summon
    if let (Some(v3), Some(gfni)) = (X64V3Token::summon(), X64GfniToken::summon()) {
        let via_method = v3.and(gfni);
        let via_join = And::join(v3, gfni);
        assert_eq!(via_method, via_join);
        assert_eq!(via_method.base(), v3);
        assert_eq!(via_method.extra(), gfni);
        assert_eq!(
            via_method.to_string(),
            format!("{} + {}", X64V3Token::NAME, X64GfniToken::NAME)
        );
    }

    if let (Some(v3), Some(sha), Some(gfni)) = (
        X64V3Token::summon(),
        X64ShaToken::summon(),
        X64GfniToken::summon(),
    ) {
        let chained = v3.and(sha).and(gfni);
        assert_eq!(chained.base().base(), v3);
        assert_eq!(chained.base().extra(), sha);
        assert_eq!(chained.extra(), gfni);
        assert_eq!(
            chained.to_string(),
            format!(
                "{} + {} + {}",
                X64V3Token::NAME,
                X64ShaToken::NAME,
                X64GfniToken::NAME
            )
        );
    }

    #[cfg(not(all(
        target_arch = "wasm32",
        target_feature = "simd128",
        target_feature = "relaxed-simd"
    )))]
    assert_eq!(
        And::<Wasm128Token, Wasm128RelaxedFmaToken>::compiled_with(),
        Some(false)
    );
}

#[cfg(target_arch = "x86_64")]
mod x86_macro_conjunctions {
    use archmage::{
        And, HasGfni, HasSha, HasX64V3, X64CryptoToken, X64GfniToken, X64ShaToken, X64V2Token,
        X64V3Token, arcane, attune, rite,
    };

    #[arcane]
    fn arcane_and(tok: And<X64V3Token, X64GfniToken>, x: i32) -> i32 {
        let _b: X64V3Token = tok.base();
        let _e: X64GfniToken = tok.extra();
        x + 1
    }

    #[arcane]
    fn arcane_multi_param(_v3: X64V3Token, _gfni: X64GfniToken, x: i32) -> i32 {
        x + 2
    }

    #[arcane]
    fn arcane_tuple((_v3, _sha): (X64V3Token, X64ShaToken), x: i32) -> i32 {
        x + 3
    }

    #[arcane]
    fn arcane_nested_tuple(
        ((_v2, _aes), _sha): ((X64V2Token, X64CryptoToken), X64ShaToken),
        x: i32,
    ) -> i32 {
        x + 4
    }

    #[arcane]
    fn arcane_multi_trait_impl(_tok: impl HasX64V3 + HasGfni, x: i32) -> i32 {
        x + 5
    }

    #[arcane]
    fn arcane_multi_trait_generic<T: HasX64V3 + HasSha>(_tok: T, x: i32) -> i32 {
        x + 6
    }

    #[rite]
    fn rite_and(_tok: And<X64V3Token, X64GfniToken>, x: i32) -> i32 {
        x + 10
    }

    #[rite]
    fn rite_tuple((_v3, _sha): (X64V3Token, X64ShaToken), x: i32) -> i32 {
        x + 20
    }

    #[rite]
    fn rite_multi_trait_generic<T: HasX64V3 + HasSha>(_tok: T, x: i32) -> i32 {
        x + 30
    }

    #[attune(wrap)]
    fn attune_and(_tok: And<X64V3Token, X64GfniToken>, x: i32) -> i32 {
        x + 100
    }

    #[attune(wrap)]
    fn attune_tuple((_v3, _sha): (X64V3Token, X64ShaToken), x: i32) -> i32 {
        x + 200
    }

    #[allow(clippy::toplevel_ref_arg)]
    #[arcane]
    fn arcane_mut_ref_patterns(
        mut tok: X64V3Token,
        ref rtok: X64V3Token,
        mut x: f32,
        (a, mut b): (f32, f32),
    ) -> f32 {
        let _copy = tok;
        let _alias = &mut tok;
        let _ref: &X64V3Token = rtok;
        x += a;
        b += 1.0;
        x + b
    }

    #[allow(clippy::toplevel_ref_arg)]
    #[rite]
    fn rite_mut_ref_patterns(
        mut tok: X64V3Token,
        ref rtok: X64V3Token,
        mut x: f32,
        (a, mut b): (f32, f32),
    ) -> f32 {
        let _copy = tok;
        let _alias = &mut tok;
        let _ref: &X64V3Token = rtok;
        x += a;
        b += 2.0;
        x + b
    }

    #[allow(clippy::toplevel_ref_arg)]
    #[attune(wrap)]
    fn attune_mut_ref_patterns(
        mut tok: X64V3Token,
        ref rtok: X64V3Token,
        mut x: f32,
        (a, mut b): (f32, f32),
    ) -> f32 {
        let _copy = tok;
        let _alias = &mut tok;
        let _ref: &X64V3Token = rtok;
        x += a;
        b += 3.0;
        x + b
    }

    #[arcane]
    fn call_rites(v3: X64V3Token, gfni: X64GfniToken, sha: X64ShaToken) -> (i32, i32, i32, f32) {
        (
            rite_and(v3.and(gfni), 1),
            rite_tuple((v3, sha), 1),
            rite_multi_trait_generic(v3.and(sha), 1),
            rite_mut_ref_patterns(v3, v3, 1.0, (2.0, 3.0)),
        )
    }

    #[derive(Clone, Copy)]
    struct Acc {
        bias: i32,
    }

    trait SelfOps {
        fn complex_self(&self, tok: X64V3Token, extra: i32) -> i32;
    }

    impl Acc {
        fn scale(x: i32) -> i32 {
            x * 10
        }
    }

    impl SelfOps for Acc {
        #[arcane(_self = Acc)]
        fn complex_self(&self, _tok: X64V3Token, extra: i32) -> i32 {
            fn inner_take(s: Self) -> i32 {
                s.bias + Self::scale(1)
            }
            let Self { bias } = *_self;
            let built = Self { bias: bias + extra };
            let closure = |k: i32| Self::scale(k) + built.bias;
            inner_take(built) + closure(2)
        }
    }

    #[test]
    fn conjunction_macros_and_patterns_work() {
        use archmage::SimdToken;

        if let Some(v3) = X64V3Token::summon() {
            assert_eq!(arcane_mut_ref_patterns(v3, v3, 1.0, (2.0, 3.0)), 7.0);
            assert_eq!(attune_mut_ref_patterns(v3, v3, 1.0, (2.0, 3.0)), 9.0);
            let acc = Acc { bias: 5 };
            // built = Acc { bias: 5 + 3 = 8 }; inner_take(built) = 8 + 10 = 18; closure(2) = 20 + 8 = 28 -> 46
            assert_eq!(acc.complex_self(v3, 3), 46);

            if let (Some(gfni), Some(sha)) = (X64GfniToken::summon(), X64ShaToken::summon()) {
                assert_eq!(arcane_and(v3.and(gfni), 5), 6);
                assert_eq!(arcane_multi_param(v3, gfni, 5), 7);
                assert_eq!(arcane_tuple((v3, sha), 5), 8);
                assert_eq!(arcane_multi_trait_impl(v3.and(gfni), 5), 10);
                assert_eq!(arcane_multi_trait_generic(v3.and(sha), 5), 11);
                assert_eq!(attune_and(v3.and(gfni), 5), 105);
                assert_eq!(attune_tuple((v3, sha), 5), 205);
                assert_eq!(call_rites(v3, gfni, sha), (11, 21, 31, 8.0));
            }
            if let (Some(v2), Some(aes), Some(sha)) = (
                X64V2Token::summon(),
                X64CryptoToken::summon(),
                X64ShaToken::summon(),
            ) {
                assert_eq!(arcane_nested_tuple(((v2, aes), sha), 5), 9);
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[arcane]
    fn pass_v3(_t: X64V3Token, x: i32) -> i32 {
        x + 100
    }

    fn pass_scalar(_t: archmage::ScalarToken, x: i32) -> i32 {
        x + 7
    }

    #[test]
    fn scalar_conjunction_and_incant_passthrough() {
        use archmage::{IntoConcreteToken, ScalarToken, SimdToken};

        let joined = ScalarToken.and(ScalarToken);
        assert!(joined.as_scalar().is_some());
        assert!(!joined.to_string().is_empty());
        assert_eq!(
            <And<ScalarToken, ScalarToken> as SimdToken>::compiled_with(),
            Some(true)
        );
        assert!(<And<ScalarToken, ScalarToken> as SimdToken>::summon().is_some());
        assert_eq!(archmage::incant!(pass(5) with joined, [v3, scalar]), 12);
    }
}
