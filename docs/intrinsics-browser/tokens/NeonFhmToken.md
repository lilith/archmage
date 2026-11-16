# NeonFhmToken — NEON+FHM

Proof that NEON + FP16 + FHM (widening FP16-to-FP32 multiply-accumulate, ARMv8.4-A) is available.

**Architecture:** aarch64 | **Features:** neon, fp16, fhm
**Total intrinsics:** 24 (24 safe, 0 unsafe, 24 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = NeonFhmToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: NeonFhmToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: NeonFhmToken, chunk: &mut [f32; 4]) {
    let v = vld1q_f32(chunk);  // safe!
    let doubled = vaddq_f32(v, v);  // value intrinsic (safe inside #[rite])
    vst1q_f32(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (24 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `vfmlal_high_f16` | Floating-point fused Multiply-Add Long to accumulator (vecto... | fmlal2 | — |
| `vfmlal_lane_high_f16` | Floating-point fused Multiply-Add Long to accumulator (by el... |  | — |
| `vfmlal_lane_low_f16` | Floating-point fused Multiply-Add Long to accumulator (by el... |  | — |
| `vfmlal_laneq_high_f16` | Floating-point fused Multiply-Add Long to accumulator (by el... |  | — |
| `vfmlal_laneq_low_f16` | Floating-point fused Multiply-Add Long to accumulator (by el... |  | — |
| `vfmlal_low_f16` | Floating-point fused Multiply-Add Long to accumulator (vecto... | fmlal | — |
| `vfmlalq_high_f16` | Floating-point fused Multiply-Add Long to accumulator (vecto... | fmlal2 | — |
| `vfmlalq_lane_high_f16` | Floating-point fused Multiply-Add Long to accumulator (by el... |  | — |
| `vfmlalq_lane_low_f16` | Floating-point fused Multiply-Add Long to accumulator (by el... |  | — |
| `vfmlalq_laneq_high_f16` | Floating-point fused Multiply-Add Long to accumulator (by el... |  | — |
| `vfmlalq_laneq_low_f16` | Floating-point fused Multiply-Add Long to accumulator (by el... |  | — |
| `vfmlalq_low_f16` | Floating-point fused Multiply-Add Long to accumulator (vecto... | fmlal | — |
| `vfmlsl_high_f16` | Floating-point fused Multiply-Subtract Long from accumulator... | fmlsl2 | — |
| `vfmlsl_lane_high_f16` | Floating-point fused Multiply-Subtract Long from accumulator... |  | — |
| `vfmlsl_lane_low_f16` | Floating-point fused Multiply-Subtract Long from accumulator... |  | — |
| `vfmlsl_laneq_high_f16` | Floating-point fused Multiply-Subtract Long from accumulator... |  | — |
| `vfmlsl_laneq_low_f16` | Floating-point fused Multiply-Subtract Long from accumulator... |  | — |
| `vfmlsl_low_f16` | Floating-point fused Multiply-Subtract Long from accumulator... | fmlsl | — |
| `vfmlslq_high_f16` | Floating-point fused Multiply-Subtract Long from accumulator... | fmlsl2 | — |
| `vfmlslq_lane_high_f16` | Floating-point fused Multiply-Subtract Long from accumulator... |  | — |
| `vfmlslq_lane_low_f16` | Floating-point fused Multiply-Subtract Long from accumulator... |  | — |
| `vfmlslq_laneq_high_f16` | Floating-point fused Multiply-Subtract Long from accumulator... |  | — |
| `vfmlslq_laneq_low_f16` | Floating-point fused Multiply-Subtract Long from accumulator... |  | — |
| `vfmlslq_low_f16` | Floating-point fused Multiply-Subtract Long from accumulator... | fmlsl | — |


