# 0.10.0 Design Review & Math Correctness Status

Recorded against commit `f1467f1d` on `main` (`lilith/archmage`).
This document captures the architectural design review (assumptions, gaps, footguns, and scaling concerns) and the baseline correctness/mode matrix for `magemath`, `magetypes` reciprocals/`sqrt`/`mul_add`, and deprecated flat transcendentals before applying follow-up fixes.

---

## 1. Architectural Review: Gaps, Footguns & Scaling Concerns

### H1. `And<Base, Extra>` conjunction proofs vs `incant!(.. with token)` & `SimdTypes`
- **Status:** Partially Fixed (`SimdTypes` fixed; `IntoConcreteToken` tracked)
- **Details:**
  - `src/tokens/generated/traits.rs` (`impl<Base, Extra> IntoConcreteToken for And<Base, Extra>`) overrides only `as_scalar()` (forwarding `self.base().as_scalar()`), leaving all SIMD `as_*` methods (`as_x64v3`, `as_x64v4`, `as_neon`, `as_wasm128`, etc.) returning `None`. Passing `X64V3Token.and(X64GfniToken)` to `incant!(f(x) with tok, [v3, scalar])` panics at runtime (or falls into `default` if present), whereas `ScalarToken.and(ScalarToken)` succeeds on `[..., scalar]`. Existing tests (`tests/conjunction_and_clumps.rs` and `tests/incant_macro.rs`) intentionally assert exact-type `incant!(.. with token)` and `ScalarToken.and(ScalarToken)` behavior, so changing `IntoConcreteToken` for `And<Base, Extra>` requires an explicit design decision (**Tracked**).
  - `SimdTypes` in `magetypes/src/types.rs` previously covered only 21 individual concrete tokens; **Fixed** via blanket `impl<T> crate::SimdTypes for T` in `magetypes/src/simd/backends/sealed.rs` (`xtask/src/simd_types/unify_gen.rs`), covering all 39 concrete leaf tokens and all `And<Base, Extra>` conjunctions with `docs/public-api/` regenerated.

### H2. `tests/downstream-compat` version skew against `0.10.0`
- **Status:** Tracked (`tier-modifiers` & `mixed-avx512` Fixed)
- `tests/downstream-compat/{linear-srgb,zensim,zenpixels-convert}/Cargo.toml` patch published `^0.9` crates via `[patch.crates-io]` with workspace `0.10.0` crates; Cargo ignores unused `0.10.0` patches with a warning and resolves `archmage v0.9.31` from crates.io instead of testing local `0.10.0` until those downstream crates release `0.10`-compatible versions (affects `.github/workflows/ci.yml` downstream compat checks).
- `tests/downstream-compat/tier-modifiers/with-avx512/Cargo.toml` was missing its `[features] default = ["avx512"]` / `avx512 = ["archmage/avx512"]` declaration (**Fixed**), and `mixed-avx512` now compiles warning-free under `#![deny(warnings)]` after fixing `#[allow(unexpected_cfgs)]` placement in `archmage-macros`.

### H3. `is_and_path` name collision & `#[autoversion]` token-rejection gap in `archmage-macros`
- **Status:** Fixed
- `archmage-macros/src/token_discovery.rs`: `is_and_path` now matches only `And`, `archmage::And`, or `archmage::tokens::And` (ignoring unrelated paths such as `my_mod::And<X64V3Token, Foo>`), and `discover_in_type` evaluates conjunction arguments in a scratch `ProofAccumulator` before committing.
- `archmage-macros/src/autoversion.rs`: `find_autoversion_token_param` now delegates to `discover_signature_proofs` and `diagnose_featureless_token`, catching `&Token`, tuples, `And<...>`, `impl Has*`, `T: Has*`, and featureless token bounds while preserving `SimdToken` and bare `ScalarToken` handling.

### H4. Default tier divergence (`incant!`/`#[autoversion]` vs `attuned!`) & feature unification
- **Status:** Partially Fixed (`incant!` & rewrite `#[allow(unexpected_cfgs)]` fixed; tier list divergence tracked)
- `archmage-macros/src/tiers.rs` sets `DEFAULT_TIER_NAMES = ["v4", "v3", "neon", "wasm128", "scalar"]`, whereas `docs/ATTUNE.md` notes elided `attuned!(work(data))` excludes `v4`/`v4x` even when `avx512` is enabled; neither default list includes `wasm128_relaxed_fma`, `arm_v2`, `arm_v3`, or `v3_vnni` (**Tracked**).
- **Fixed (`incant!` / `rewrite`):** `archmage-macros/src/tiers.rs` (`wrap_unexpected_cfg`) and `archmage-macros/src/rewrite.rs` now wrap implied `#[cfg(feature = "...")]` gates inside enclosing `#[allow(unexpected_cfgs)] { ... }` blocks so crates without `[features] avx512` compile warning-free under `#![deny(warnings)]` (note: item-level `#[cfg(feature = "avx512")]` on `#[magetypes]` variants still cannot suppress `unexpected_cfgs` via a same-item `#[allow]`).
- Workspace feature unification can still enable `magetypes/avx512` while `magemath/avx512` stays off, causing `magemath/src/token_canon.rs` to downgrade `X64V4Token`/`X64V4xToken` to `.v3()` (**Tracked**).

### H5. Compile-time & arity ceiling in `caps.rs` / `sealed.rs`
- **Status:** Tracked (Architectural)
- `src/tokens/generated/caps.rs` and `magetypes/src/simd/backends/sealed.rs` expand `X64CapSet<C0..C17>` and `SealedFromCaps<T>`. Current generated backend line counts stay within `COMPILE_BUDGETS` in `xtask/src/main.rs` (`x86_64 40,875 / 41,500`, `aarch64 37,664 / 38,500`, `wasm32 37,515 / 38,500`, `generic/generated 28,492 / 29,500` measured by `cargo run -p xtask -- validate`). Adding new orthogonal ISAs remains a long-term scaling constraint.

### H6. Implementation & documentation rough edges (`magemath`, `magetypes`, `xtask`, docs)
- **Dead `LEGACY` bit & stale coeff docs (`magemath`) — Fixed:** Removed dead `ExpfOptions::LEGACY` and `LogfOptions::LEGACY` branches in `expf/graph.rs` and `logf/graph.rs`, and updated `expf/options.rs` / `logf/options.rs` comments (`Refitted MIDP/LOWP`, `Rational P2/Q2`).
- **`try_log1p_*` ignores `BackendDependent` & scalarizes vectors — Fixed:** Vectorized `log1p_parts` in `magemath/src/logf/const_api.rs` and made the final `LN_2` multiply-add honor `LogfOptions::BACKEND` (`mul_add_relaxed` vs `mul_add_fused`).
- **License & documentation links — Fixed:** Fixed `magemath/LICENSE-ARM` references in `magemath/src/powf/highp_data.rs` and `docs/POWF.md`, aligned `src/lib.rs` and `MSRV.md` guide links to `https://lilith.github.io/archmage/`, and replaced dead relative `../benchmarks/powf_*` links in `docs/POWF.md` and `docs/POWF_REFERENCE.md` with plain external archive citations.
- **Remaining tracked rough edges:**
  - Duplicate WASM FMA probe caches in `magetypes/src/wasm_fma.rs` and `magemath/src/wasm_fma.rs`.
  - Missing `*_f32xN_runtime` vector wrappers on `expf`/`logf`/`cbrtf` (present on `powf`).
  - `try_exp` pre-check scaling (`x * LOG2_E` overflows to `±inf` before domain check; pinned by `magemath/tests/scalar_vector_parity_expf_cbrtf.rs`).
  - `ScalarToken` `f32x16::min`/`max` `<` vs `f32::min`/`f32::max` NaN divergence (`docs/site/content/magetypes/isa-quirks.md`).
  - `xtask/src/soundness.rs` and `xtask/src/soundness/raw_context.rs` brace/regex scanning and non-single-tier fail-closed `from_context()` checks.
  - Constructor deprecation wording (`0.11` in generated docs vs `0.11+` in `docs/TOKEN-CONSTRUCTOR-MIGRATION.md`).

---

## 2. Correctness & Modes Status: `magemath`, `recip`/`rsqrt`, and Transcendentals

### A. `magemath` (`powf`, `expf`, `logf`, `cbrtf`)

All four modules expose const `.bits()` builders (`PowfOptions`, `ExpfOptions`, `LogfOptions`, `CbrtfOptions`) plus runtime adapters (`*_runtime` backed by literal const instantiations — `104` valid encodings in `powf` with `std` / `84` without `std`, and `32` each in `expf`, `logf`, `cbrtf`), with bit-exact scalar/vector parity across tokens and `Checked` stage-ordered whole-vector error masks (`lane_mask`).

| Axis | `powf` | `expf` (`exp2`, `exp`) | `logf` (`log2`, `ln`, `log10`, `log1p`) | `cbrtf` (`cbrt`) |
|---|---|---|---|---|
| **Domain** | `PositiveNormal` (default), `NonNegativeNormal`, `Gamma` (`+0` or `[2^-16, 2^16]`, exp `[0, 4]`), `PositiveGamma` (nonzero), `Full` (HIGHP only) | `Bounded` (default: rejects `NaN`/`±inf`; clamps finite `[-126, 127.99]` lowp / `[-126, 128)` midp), `Full` (`NaN→qNaN`, `+inf→+inf`, `-inf→0`) | `Positive` (default: finite normal `> 0`), `Specials` (`±0→-inf`, `<0→NaN`, `+inf→+inf`; subnormals inaccurate; `log1p` checks `u=1+x`, preserves `±0.0`) | `Bounded` (default: `±0.0` and normals `< f32::MAX / 3`), `Full` (subnormals `×2^24`/`2^-8`, `>=1e36` `×0.125`/`2`, `±0`, `±inf`, `qNaN`) |
| **Precision & Accuracy** | `Midp` (default, documented corpus max `47.902 ULP` Gamma / `< 438.504 ULP` PositiveNormal), `Lowp`, `Highp` (Arm v26.07 binary64 `TOINT=0` graph: `<= 0.506156 ULP` on 27,909 pairs, `<= 0.500008 ULP` on 515 hard witnesses per `docs/POWF_REFERENCE.md`) | `Midp` (default, target `<= 1.9 ULP` for `exp2 < 127.5`), `Lowp` (target `<= 0.56%` rel err) | `Midp` (default, target `<= 4.5 ULP` normal domain), `Lowp` (rational P2/Q2, `~6e-6` abs error; larger rel err near `1`) | `Midp` (default, test-asserted `<= 3.2 ULP`), `Lowp` (test-asserted `<= 259.0 ULP`, `~3e-5` rel err in `magemath/src/cbrtf/tests.rs`) |
| **Portability** | `BitExact` (default: fused single-rounding everywhere; runtime WASM `relaxed_madd` probe; `std` screens `mul_add`), `BackendDependent` (`mul_add_relaxed`, or `std` `f32::powf` for `Highp`+`std`) | `BitExact` (default: `mul_add_fused`), `BackendDependent` (`mul_add_relaxed`) | `BitExact` (default: `mul_add_fused`), `BackendDependent` (`mul_add_relaxed`, including `log1p` final `LN_2` step) | `BitExact` (default: explicit `(y³ + 2x)/(2y³ + x)`), `BackendDependent` (`mul_add_relaxed` in Halley step) |
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
