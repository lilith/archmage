# ARM rsqrt/rcp family comparison — Neoverse-N1

Throughput of three reciprocal / reciprocal-sqrt implementation families on real
ARM hardware, for both the estimate (`_approx`) and full-precision tiers.

## Provenance

| | |
|---|---|
| Box | ARM64 cloud server (Neoverse-N1), Ampere Altra **Neoverse-N1**, 8 cores, fixed 3.0 GHz (no turbo) |
| OS / toolchain | Ubuntu aarch64, rustc 1.96.0 (stable) |
| Commit | `ffbee4b` (main) |
| Bench | `magetypes/benches/rsqrt_arm.rs` (`f32x4` / `NeonToken`) |
| Command | `cargo bench -p magetypes --bench rsqrt_arm --features std` |
| Harness | zenbench 0.1.2 (criterion-compat) |
| Workload | N = 2048 f32 (8 KB, L1-resident → compute-bound), 16 passes/iter, load → op → store |

The three families:

- **original** — the pre-change methods: raw `vrsqrteq`/`vrecpeq` 8-bit estimate; full precision = + **two manual** Newton steps (`mul`/`sub` + 0.5/3.0/2.0 splats).
- **frsqrts** — the current shipped fast methods: native NEON **FRSQRTS/FRECPS** assist instructions; `_approx` = + one step (~16-bit), full = + two steps.
- **portable** — the deterministic `_portable` family: integer **bit-trick seed + non-FMA** Newton step (8-bit); full precision via IEEE **div/sqrt**.

## Results

Per-element time = (µs/iter) ÷ (16 × 2048). Ratios are within each tier vs the original.

### rsqrt — estimate tier

| family | µs/iter | ns/elem | vs original |
|---|--:|--:|--:|
| `original` raw `vrsqrteq` (8-bit) | 6.1 | 0.186 | 1.00× |
| `frsqrts` +1 FRSQRTS step (16-bit) | 10.0 | 0.305 | 1.64× |
| `portable` bit-hack +1 non-FMA (8-bit) | 14.9 | 0.455 | 2.44× |

### rsqrt — full precision

| family | µs/iter | ns/elem | vs original |
|---|--:|--:|--:|
| `original` 2 manual Newton | 25.4 | 0.775 | 1.00× |
| **`frsqrts` 2 FRSQRTS steps** | **16.8** | **0.513** | **0.66× (1.51× faster)** |
| `portable` div/sqrt | 36.1 | 1.102 | 1.42× (slower) |

### rcp — estimate tier

| family | µs/iter | ns/elem | vs original |
|---|--:|--:|--:|
| `original` raw `vrecpeq` (8-bit) | 5.8 | 0.177 | 1.00× |
| `frsqrts` +1 FRECPS step (16-bit) | 8.1 | 0.247 | 1.40× |
| `portable` bit-hack +1 non-FMA (8-bit) | 8.8 | 0.269 | 1.52× |

### rcp — full precision

| family | µs/iter | ns/elem | vs original |
|---|--:|--:|--:|
| `original` 2 manual Newton | 16.0 | 0.488 | 1.00× |
| **`frsqrts` 2 FRECPS steps** | **12.1** | **0.369** | **0.76× (1.32× faster)** |
| `portable` div | 16.8 | 0.513 | 1.05× (~par) |

## Takeaways

1. **The FRSQRTS/FRECPS change made full-precision `recip`/`rsqrt` faster on ARM** — 1.51× (rsqrt) and 1.32× (rcp) over the old two-manual-step path. The native fused assist instructions do the `(3 − a·y²)/2` / `(2 − a·y)` factor in one rounding and drop the 0.5/3.0/2.0 splats. This is the headline win of the existing-method change.

2. **Deterministic full precision (`*_portable` via div/sqrt) is the slowest full path** — 2.15× the `frsqrts` path for rsqrt (div+sqrt latency), ~par with the old manual path for rcp (a single div). That is the cost of bit-exact, correctly-rounded full precision; it buys reproducibility, not speed.

3. **In the estimate tier, the raw hardware estimate is cheapest** because it's a single instruction. Refinement isn't free: `frsqrts` (16-bit) costs +40–64%, and the deterministic `portable` (8-bit) costs +52–144% — most for rsqrt, where the alternative is one `vrsqrteq`. The bit-hack reciprocal estimate is nearly free over `frsqrts` (8.8 vs 8.1 µs), but the rsqrt bit-hack (extra mul + non-FMA step) is not (14.9 vs 10.0 µs).

**Guidance:** on ARM, prefer the `frsqrts` full methods for speed+accuracy; reach for `_portable` only when cross-machine bit-identity is required, and budget ~1.4–2.2× for it.

## Caveats

- Numbers are per-element including the shared load→store; deltas reflect op cost. Absolute ns/elem are L1-resident, compute-bound — larger memory-bound working sets compress the gaps as all families converge toward memory bandwidth.
- zenbench reported "0 rounds" (criterion-compat path doesn't run the interleaved round-robin). On a fixed-frequency Ampere part (no turbo) thermal/turbo bias is negligible, and per-bench 95% CIs were tight (≤ ±2%) except the first measurement (`rsqrt_approx` original, CV≈21%, a warmup artifact; ~6 µs is consistent with run 1's 365 ns × 16).
- Raw output: `rsqrt_arm_neoverse-n1_2026-06-21.raw.txt`.
