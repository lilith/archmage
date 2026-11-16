# 0.10.0 Design Review & Math Correctness Status

Recorded against commit `f1467f1d` on `main` (`lilith/archmage`).
This document captures the architectural design review (assumptions, gaps, footguns, and scaling concerns) and the baseline correctness/mode matrix for `magemath`, `magetypes` reciprocals/`sqrt`/`mul_add`, and deprecated flat transcendentals before applying follow-up fixes.

---

## 1. Architectural Review: Gaps, Footguns & Scaling Concerns

### H1. `And<Base, Extra>` conjunction proofs vs `incant!(.. with token)` & `SimdTypes`
- **Status:** Tracked
- **Symptom:**
  - `src/tokens/generated/traits.rs` (`impl<Base, Extra> IntoConcreteToken for And<Base, Extra>`) overrides only `as_scalar()` (forwarding `self.base().as_scalar()`), leaving all SIMD `as_*` methods (`as_x64v3`, `as_x64v4`, `as_neon`, `as_wasm128`, etc.) returning `None`. Passing `X64V3Token.and(X64GfniToken)` to `incant!(f(x) with tok, [v3, scalar])` panics at runtime (or falls into `default` if present), whereas `ScalarToken.and(ScalarToken)` succeeds on `[..., scalar]`. Note: existing tests (`tests/conjunction_and_clumps.rs` and `tests/incant_macro.rs`) pin exact-type `incant!(.. with token)` and `ScalarToken.and(ScalarToken)` behavior, so altering `IntoConcreteToken` for `And<Base, Extra>` requires an explicit design/contract decision.
  - `magetypes/src/types.rs` implements `SimdTypes` for only 21 individual concrete tokens, omitting `And<Base, Extra>`, `X64Avx512Bf16Token`, and several x86/NEON leaf tokens even though `MagetypesToken` in `magetypes/src/simd/generic/subtiers.rs` supports all concrete tokens and `And<Base, Extra>`.

### H2. `tests/downstream-compat` version skew against `0.10.0`
- **Status:** Tracked
- `tests/downstream-compat/{linear-srgb,zensim,zenpixels-convert}/Cargo.toml` patch published `^0.9` crates via `[patch.crates-io]` with workspace `0.10.0` crates, which Cargo rejects until those downstream crates target `0.10`. Meanwhile, `.github/workflows/ci.yml` invokes `cargo test` in those directories.

### H3. `is_and_path` name collision & `#[autoversion]` token-rejection gap in `archmage-macros`
- **Status:** Tracked
- `archmage-macros/src/token_discovery.rs`: `is_and_path` matches any type path whose last segment is `And<L, R>` (not just `archmage::And`), recurses into both type parameters mutating `ProofAccumulator`, and emits a compile error whenever `left_ok || right_ok` — rejecting unrelated user types such as `my_mod::And<X64V3Token, Foo>`.
- `archmage-macros/src/autoversion.rs`: `find_autoversion_token_param` only inspects bare `Type::Path` leaf tokens (`X64V3Token`), missing `&X64V3Token`, `(X64V3Token, X64GfniToken)`, `And<...>`, and `impl Has*`.

### H4. Default tier divergence (`incant!`/`#[autoversion]` vs `attuned!`) & feature unification
- **Status:** Tracked
- `archmage-macros/src/tiers.rs` sets `DEFAULT_TIER_NAMES = ["v4", "v3", "neon", "wasm128", "scalar"]`, whereas `docs/ATTUNE.md` notes elided `attuned!(work(data))` excludes `v4`/`v4x` even when `avx512` is enabled; neither default list includes `wasm128_relaxed_fma`, `arm_v2`, `arm_v3`, or `v3_vnni`.
- Workspace feature unification can enable `magetypes/avx512` while `magemath/avx512` stays off, causing `magemath/src/token_canon.rs` to downgrade `X64V4Token`/`X64V4xToken` to `.v3()`. Similarly, sibling crates without `avx512` inherit `archmage-macros/avx512` unless they gate `v4(cfg(avx512))` explicitly.

### H5. Compile-time & arity ceiling in `caps.rs` / `sealed.rs`
- **Status:** Tracked (Architectural)
- `src/tokens/generated/caps.rs` (~2,092 lines) uses an 18-slot `X64CapSet<C0..C17>` tuple of `Flag` markers (`Yes`/`No`), and `magetypes/src/simd/backends/sealed.rs` (~3,897 lines) expands `SealedFromCaps<T>` across that representation. Adding new orthogonal ISAs increases tuple arity, trait-solver work, and generated line count near `COMPILE_BUDGETS` in `xtask/src/main.rs` (`41,500` on `x86_64`, `38,500` on `aarch64`).

### H6. Implementation & documentation rough edges (`magemath`, `magetypes`, `xtask`, docs)
- **Dead `LEGACY` bit & stale coeff docs (`magemath`):** `ExpfOptions::LEGACY = 64` and `LogfOptions::LEGACY = 64` are excluded by `valid(bits)` (`bits & !0b11_1101 == 0`) yet still branched on in `expf/graph.rs` and `logf/graph.rs`. Because bit 6 is always zero, `expf`/`logf` use refitted coefficients while `powf/graph.rs` and deprecated `magetypes` transcendentals keep older coefficients, and `expf/options.rs` / `logf/options.rs` comments still claim `"Existing MIDP coefficients"`.
- **`try_log1p_*` ignores `BackendDependent` & scalarizes vectors:** `magemath/src/logf/mod.rs` and `logf/const_api.rs` extract lanes via `.to_array()` -> scalar `log1p_parts` and hardcode `.mul_add_fused(LN_2, c)`, ignoring `LogfPortability::BackendDependent` for the final multiply-add.
- **Duplicate WASM FMA probe caches:** `magetypes/src/wasm_fma.rs` and `magemath/src/wasm_fma.rs` maintain separate `AtomicU8` runtime probe caches and software round-to-odd FMA helpers.
- **Missing `*_f32xN_runtime` wrappers on `expf`/`logf`/`cbrtf`:** Present on `powf`, absent on the other three modules.
- **`try_exp` pre-check scaling:** `magemath/src/expf/mod.rs` multiplies `x * LOG2_E` before range checking, so finite `|x|` large enough that `x * LOG2_E` overflows to `±inf` is rejected as `InvalidInput` under `Bounded`. (Pinned by `magemath/tests/scalar_vector_parity_expf_cbrtf.rs`.)
- **`ScalarToken` `f32x16` `min`/`max` NaN quirk:** `magetypes/src/simd/impls/scalar.rs` uses `<` comparisons for `f32x16::min`/`max` (`min(1.0, qNaN) -> qNaN`) while `f32x4`/`f32x8` use `f32::min`/`f32::max` (`1.0`). (Documented in `docs/site/content/magetypes/isa-quirks.md`.)
- **Soundness scanner & doc/link drift:**
  - `xtask/src/soundness.rs` and `xtask/src/soundness/raw_context.rs` rely on brace/regex text scanning and fail closed on non-single-tier attribute forms for `from_context()`.
  - `src/lib.rs` mixes `imazen.github.io` and `lilith.github.io` links.
  - `docs/POWF.md` and `docs/POWF_REFERENCE.md` link to missing `../benchmarks/powf_module_2026-11-07/...` reports.
  - Constructor deprecation notes say `0.11` (`f32x4_impl.rs`) while `docs/TOKEN-CONSTRUCTOR-MIGRATION.md` says `0.11+` future plan.
  - `magemath/src/powf/highp_data.rs:3` references `magetypes/LICENSE-ARM` instead of `magemath/LICENSE-ARM`.

---

## 2. Correctness & Modes Status: `magemath`, `recip`/`rsqrt`, and Transcendentals

### A. `magemath` (`powf`, `expf`, `logf`, `cbrtf`)

All four modules expose const `.bits()` builders (`PowfOptions`, `ExpfOptions`, `LogfOptions`, `CbrtfOptions`) plus runtime adapters (`*_runtime` backed by literal const instantiations — `104` valid encodings in `powf` with `std` / `84` without `std`, and `32` each in `expf`, `logf`, `cbrtf`), with bit-exact scalar/vector parity across tokens and `Checked` stage-ordered whole-vector error masks (`lane_mask`).

| Axis | `powf` | `expf` (`exp2`, `exp`) | `logf` (`log2`, `ln`, `log10`, `log1p`) | `cbrtf` (`cbrt`) |
|---|---|---|---|---|
| **Domain** | `PositiveNormal` (default), `NonNegativeNormal`, `Gamma` (`+0` or `[2^-16, 2^16]`, exp `[0, 4]`), `PositiveGamma` (nonzero), `Full` (HIGHP only) | `Bounded` (default: rejects `NaN`/`±inf`; clamps finite `[-126, 127.99]` lowp / `[-126, 128)` midp), `Full` (`NaN→qNaN`, `+inf→+inf`, `-inf→0`) | `Positive` (default: finite normal `> 0`), `Specials` (`±0→-inf`, `<0→NaN`, `+inf→+inf`; subnormals inaccurate; `log1p` checks `u=1+x`, preserves `±0.0`) | `Bounded` (default: `±0.0` and normals `< f32::MAX / 3`), `Full` (subnormals `×2^24`/`2^-8`, `>=1e36` `×0.125`/`2`, `±0`, `±inf`, `qNaN`) |
| **Precision & Accuracy** | `Midp` (default, documented corpus max `47.902 ULP` Gamma / `< 438.504 ULP` PositiveNormal), `Lowp`, `Highp` (Arm v26.07 binary64 `TOINT=0` graph: `<= 0.506156 ULP` on 27,909 pairs, `<= 0.500008 ULP` on 515 hard witnesses per `docs/POWF_REFERENCE.md`) | `Midp` (default, target `<= 1.9 ULP` for `exp2 < 127.5`), `Lowp` (target `<= 0.56%` rel err) | `Midp` (default, target `<= 4.5 ULP` normal domain), `Lowp` (rational P2/Q2, `~6e-6` abs error; larger rel err near `1`) | `Midp` (default, test-asserted `<= 3.2 ULP`), `Lowp` (test-asserted `<= 259.0 ULP`, `~3e-5` rel err in `magemath/src/cbrtf/tests.rs`) |
| **Portability** | `BitExact` (default: fused single-rounding everywhere; runtime WASM `relaxed_madd` probe; `std` screens `mul_add`), `BackendDependent` (`mul_add_relaxed`, or `std` `f32::powf` for `Highp`+`std`) | `BitExact` (default: `mul_add_fused`), `BackendDependent` (`mul_add_relaxed`) | `BitExact` (default: `mul_add_fused`), `BackendDependent` (`mul_add_relaxed`; *caveat*: `log1p` final `LN_2` step currently uses `mul_add_fused`) | `BitExact` (default: explicit `(y³ + 2x)/(2y³ + x)`), `BackendDependent` (`mul_add_relaxed` in Halley step) |
| **Validation** | `Checked` (default) / `AssumeValid` | `Checked` / `AssumeValid` | `Checked` / `AssumeValid` | `Checked` / `AssumeValid` |
| **Execution** | `Automatic` (default: upgrades scalar once to V3/NEON/Wasm128, or `Plan::Std`) / `Caller` (retains caller token for tails) | `Automatic` / `Caller` | `Automatic` / `Caller` | `Automatic` / `Caller` |

### B. `magetypes` Reciprocals, `sqrt`, `mul_add`, and Deprecated Flat Transcendentals

1. **`f32` Reciprocals & Reciprocal Sqrt (`f32x4` / `f32x8` / `f32x16`)**
   - **Fast approx (`rcp_approx`, `rsqrt_approx`):** pinned `>= 10.0` bits (`magetypes/tests/reciprocal_precision.rs`); IEEE rails unspecified (scalar/wasm128 `rsqrt_approx` uses `0x5f37_59df` + 2 non-FMA Newton steps and produces inaccurate output at `±0`/subnormals).
   - **Working tier (`recip`, `rsqrt`):** pinned `<= 4 ULP` on x86/NEON and `0 ULP` on WASM/Scalar, with exact IEEE rails (`±0 -> ±inf`, `+inf -> +0`, `-inf -> -0` [`recip`] / `NaN` [`rsqrt`], `NaN -> NaN`).
     - `x86_v3`: estimate + 1 FMA-Newton + `_CMP_UNORD_Q` `blendv` rescue.
     - `x86_v4` (`f32x16`): `rcp14`/`rsqrt14` + 1 Newton + `_mm512_fixupimm_ps::<0>`.
     - `arm_neon`: 2nd `FRECPS`/`FRSQRTS` refinement step (preserves rails).
     - `wasm128` & `scalar`: exact `1.0 / a` and `1.0 / sqrt(a)`.
     - Known accuracy caveat (`f32x4_impl.rs`, `docs/site/content/magetypes/isa-quirks.md`): subnormal inputs are unspecified, and `x86_v3` `recip()` flushes subnormal outputs (e.g., `recip(±f32::MAX) -> ±0.0`).
   - **Precise & Cross-Arch Bit-Exact tiers:**
     - `recip_portable()` (`1.0 / self`) & `rsqrt_portable()` (`1.0 / self.sqrt()`): `0 ULP`, IEEE rails and subnormals exact, bit-identical cross-arch.
     - `rcp_approx_portable` / `rsqrt_approx_portable` (+ `*_newton_portable`): integer seeds (`0x7ef127ea` / `0x5f3759df`) + plain non-FMA mul/sub; bit-identical across all ISAs and widths (`4`/`8`/`16`), pinned `< 6e-3` (1 step) / `< 1e-4` (+1 refine) in `magetypes/tests/portable_reciprocal.rs`.

2. **`f64` Reciprocals, `sqrt()`, and `mul_add_*()`**
   - `f64` `recip()` / `rsqrt()` use exact `1.0 / x` and `1.0 / sqrt(x)` (`0 ULP`, pinned `>= 50` bits). `rcp_approx()` / `rsqrt_approx()` use `rcp14_pd`/`rsqrt14_pd` on V4 (`512`-bit), `vrecpeq_f64`/`vrsqrteq_f64` + 1 step on NEON, and exact div/sqrt elsewhere.
   - `sqrt()` uses hardware intrinsics on SIMD targets and correctly rounded Goldschmidt in `magetypes/src/nostd_math.rs` on scalar.
   - `mul_add` / `mul_sub` are deprecated (planned fused in `0.11`): `mul_add_relaxed` tracks hardware FMA / separate mul+add / WASM relaxed madd; `mul_add_fused` (`mul_add_portable`) guarantees single rounding everywhere (native FMA on V3/V4/NEON, zero-probe `relaxed_madd` under `Wasm128RelaxedFmaToken`, cached probe + software round-to-odd/Dekker fallback under `Wasm128Token` and `ScalarToken`).

3. **Deprecated Flat `magetypes` Transcendentals (`transcendentals_f32x{4,8,16}.rs`)**
   - Kept bit-for-bit identical to `0.9.31` (verified by `just magemath-public-boundary`) and deprecated in favor of `magemath`.
   - Use `mul_add_relaxed` (so `pow_midp(2.4)` can differ between scalar and AVX2 by up to `178 ULP` on ~11% of inputs per `docs/transcendentals.md`), omit whole-vector error checks, return inaccurate values on subnormals (except `cbrt_midp_precise`), use scalar `.to_array()` Kahan `/ 3` in `cbrt_{lowp,midp}`, and are asserted in `magetypes/tests/transcendental_accuracy.rs` (`max_ulp: 4` for `log*_midp` & `cbrt_midp*`, `max_ulp: 128` for `exp*_midp` & `pow_midp*`, `max_ulp: 256` for `cbrt_lowp`, `RelOnly` for other `*_lowp`).
