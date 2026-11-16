# NeonDotProdToken — NEON+DotProd

Proof that NEON + DotProd (`udot`/`sdot`, ARMv8.2-A) is available.

**Architecture:** aarch64 | **Features:** neon, dotprod
**Total intrinsics:** 12 (12 safe, 0 unsafe, 12 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = NeonDotProdToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: NeonDotProdToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: NeonDotProdToken, chunk: &mut [f32; 4]) {
    let v = vld1q_f32(chunk);  // safe!
    let doubled = vaddq_f32(v, v);  // value intrinsic (safe inside #[rite])
    vst1q_f32(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (12 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `vdot_lane_s32` | Dot product arithmetic (indexed) | vsdot | — |
| `vdot_lane_u32` | Dot product arithmetic (indexed) | vudot | — |
| `vdot_laneq_s32` | Dot product arithmetic (indexed) | vsdot | — |
| `vdot_laneq_u32` | Dot product arithmetic (indexed) | vudot | — |
| `vdot_s32` | Dot product arithmetic (vector) | vsdot | — |
| `vdot_u32` | Dot product arithmetic (vector) | vudot | — |
| `vdotq_lane_s32` | Dot product arithmetic (indexed) | vsdot | — |
| `vdotq_lane_u32` | Dot product arithmetic (indexed) | vudot | — |
| `vdotq_laneq_s32` | Dot product arithmetic (indexed) | vsdot | — |
| `vdotq_laneq_u32` | Dot product arithmetic (indexed) | vudot | — |
| `vdotq_s32` | Dot product arithmetic (vector) | vsdot | — |
| `vdotq_u32` | Dot product arithmetic (vector) | vudot | — |


