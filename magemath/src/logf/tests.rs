use super::*;

#[test]
fn positive_rejects_invalid() {
    for x in [0.0, -0.0, -1.0, f32::NAN, f32::INFINITY, 1e-42] {
        assert_eq!(
            try_log2(x).unwrap_err().kind,
            LogfErrorKind::InvalidInput,
            "log2({x})"
        );
    }
}

#[test]
fn specials_domain() {
    const SPECIALS: u32 = LogfOptions::SPECIALS;
    assert_eq!(try_log2_with::<SPECIALS>(0.0), Ok(f32::NEG_INFINITY));
    assert!(try_log2_with::<SPECIALS>(-1.0).unwrap().is_nan());
    assert!(try_log2_with::<SPECIALS>(f32::NAN).unwrap().is_nan());
    assert_eq!(try_log2_with::<SPECIALS>(f32::INFINITY), Ok(f32::INFINITY));
}

#[test]
fn aliases_and_runtime() {
    let x = 4.0_f32;
    assert_eq!(try_ln(x), try_log2(x).map(|v| v * core::f32::consts::LN_2));
    assert_eq!(
        try_log10(x),
        try_log2(x).map(|v| v * core::f32::consts::LOG10_2)
    );
    assert_eq!(
        try_log2_array_with::<{ LogfOptions::DEFAULT }, 3>([1.0, 2.0, 3.0])
            .unwrap_err()
            .kind,
        LogfErrorKind::UnsupportedWidth
    );
    assert_eq!(try_log2_runtime(8.0, LogfOptions::new()), Ok(3.0));
}

#[test]
fn log1p_basics() {
    const CALLER: u32 = LogfOptions::DEFAULT | LogfOptions::CALLER;
    assert_eq!(try_log1p_with::<CALLER>(0.0), Ok(0.0));
    assert_eq!(
        try_log1p_with::<CALLER>(1.0).unwrap() - core::f32::consts::LN_2,
        0.0
    );
    assert!((try_log1p_with::<CALLER>(-0.5).unwrap() + core::f32::consts::LN_2).abs() < 1e-6);
    // correction recovers what u = 1+x rounds away for tiny x
    for x in [1e-8f32, -1e-8, 1e-20, 2.5e-5, -0.4142] {
        let got = try_log1p_with::<CALLER>(x).unwrap();
        let r = (x as f64).ln_1p() as f32;
        assert_eq!(got, r, "log1p({x}): {got} vs {r}");
    }
    // specials domain
    const S: u32 = LogfOptions::DEFAULT | LogfOptions::SPECIALS;
    assert_eq!(try_log1p_with::<S>(-1.0), Ok(f32::NEG_INFINITY));
    assert!(try_log1p_with::<S>(-1.5).unwrap().is_nan());
    assert_eq!(try_log1p_with::<S>(f32::INFINITY), Ok(f32::INFINITY));
    // positive domain rejects x <= -1
    assert_eq!(
        try_log1p_with::<CALLER>(-1.0).unwrap_err().kind,
        LogfErrorKind::InvalidInput
    );
    assert_eq!(
        try_log1p_with::<CALLER>(-2.0).unwrap_err().kind,
        LogfErrorKind::InvalidInput
    );
    // array + runtime parity
    assert_eq!(
        try_log1p_array_with::<CALLER, 1>([0.5]),
        Ok([f32::from_bits(0x3ecf_991f)]) // MPFR256 RNE log1p(0.5); std is an approximate oracle under Miri.
    );
    assert_eq!(
        try_log1p_runtime(1.0, LogfOptions::new()).unwrap(),
        core::f32::consts::LN_2
    );
}

#[test]
fn log1p_mixed_vector_lanes_and_backend_portability() {
    use archmage::ScalarToken;
    use magetypes::simd::generic::f32x4;

    const SPECIALS_CALLER: u32 = LogfOptions::DEFAULT | LogfOptions::SPECIALS | LogfOptions::CALLER;
    const SPECIALS_BACKEND_CALLER: u32 = SPECIALS_CALLER | LogfOptions::BACKEND;

    // Mixed lanes across every log1p branch: u >= 2, 1 < u < 2, tiny 1+x==1, +/-0, specials
    let lanes = [
        [-0.0_f32, 0.0, 1e-20, 1.5],
        [-1.0_f32, -2.0, f32::INFINITY, f32::NAN],
        [0.25_f32, 0.999_999_94, 1.0, 1e20],
    ];
    for xs in lanes {
        let vec_exact = try_log1p_f32x4_with::<SPECIALS_CALLER, _>(
            ScalarToken,
            f32x4::from_array_t(ScalarToken, xs),
        )
        .unwrap()
        .to_array();
        let vec_backend = try_log1p_f32x4_with::<SPECIALS_BACKEND_CALLER, _>(
            ScalarToken,
            f32x4::from_array_t(ScalarToken, xs),
        )
        .unwrap()
        .to_array();
        for i in 0..4 {
            let s_exact = try_log1p_with_token::<SPECIALS_CALLER, _>(ScalarToken, xs[i]).unwrap();
            let s_backend =
                try_log1p_with_token::<SPECIALS_BACKEND_CALLER, _>(ScalarToken, xs[i]).unwrap();
            if xs[i].is_nan() || xs[i] < -1.0 {
                assert!(vec_exact[i].is_nan() && s_exact.is_nan());
                assert!(vec_backend[i].is_nan() && s_backend.is_nan());
            } else {
                assert_eq!(vec_exact[i].to_bits(), s_exact.to_bits());
                assert_eq!(vec_backend[i].to_bits(), s_backend.to_bits());
            }
        }
    }

    // Verify BackendDependent allows separate mul + add under ScalarToken instead of forcing software FMA:
    // find an input in [0.5, 4.0] where mul_add_fused(val, LN_2, c) != val * LN_2 + c.
    let mut found_rounding_diff = false;
    for k in 1000..3000 {
        let x = k as f32 * 0.001_f32;
        let exact = try_log1p_with_token::<{ LogfOptions::CALLER }, _>(ScalarToken, x).unwrap();
        let relaxed = try_log1p_with_token::<{ LogfOptions::CALLER | LogfOptions::BACKEND }, _>(
            ScalarToken,
            x,
        )
        .unwrap();
        if exact.to_bits() != relaxed.to_bits() {
            found_rounding_diff = true;
            break;
        }
    }
    assert!(
        found_rounding_diff,
        "expected ScalarToken BitExact vs BackendDependent log1p rounding to diverge cleanly"
    );
}
