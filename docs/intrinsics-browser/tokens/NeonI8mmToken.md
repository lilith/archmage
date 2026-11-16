# NeonI8mmToken — NEON+I8MM

Proof that NEON + I8MM (`vmmla` / `usdot` / `sudot`, ARMv8.6-A) is available.

**Architecture:** aarch64 | **Features:** neon, i8mm
**Total intrinsics:** 13 (13 safe, 0 unsafe, 0 stable, 13 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = NeonI8mmToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: NeonI8mmToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: NeonI8mmToken, chunk: &mut [f32; 4]) {
    let v = vld1q_f32(chunk);  // safe!
    let doubled = vaddq_f32(v, v);  // value intrinsic (safe inside #[rite])
    vst1q_f32(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Unstable/Nightly (13 intrinsics)

| Name | Description | Instruction |
|------|-------------|-------------|
| `vmmlaq_s32` | 8-bit integer matrix multiply-accumulate | nop |
| `vmmlaq_u32` | 8-bit integer matrix multiply-accumulate | nop |
| `vsudot_lane_s32` | Dot product index form with signed and unsigned integers | vsudot |
| `vsudot_laneq_s32` | Dot product index form with signed and unsigned integers | vsudot |
| `vsudotq_lane_s32` | Dot product index form with signed and unsigned integers | vsudot |
| `vsudotq_laneq_s32` | Dot product index form with signed and unsigned integers | vsudot |
| `vusdot_lane_s32` | Dot product index form with unsigned and signed integers | vusdot |
| `vusdot_laneq_s32` | Dot product index form with unsigned and signed integers | vusdot |
| `vusdot_s32` | Dot product vector form with unsigned and signed integers | vusdot |
| `vusdotq_lane_s32` | Dot product index form with unsigned and signed integers | vusdot |
| `vusdotq_laneq_s32` | Dot product index form with unsigned and signed integers | vusdot |
| `vusdotq_s32` | Dot product vector form with unsigned and signed integers | vusdot |
| `vusmmlaq_s32` | Unsigned and signed 8-bit integer matrix multiply-accumulate | nop |


