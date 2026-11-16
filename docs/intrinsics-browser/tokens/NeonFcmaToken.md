# NeonFcmaToken — NEON+FCMA

Proof that NEON + FCMA (complex multiply-add, ARMv8.3-A) is available.

**Architecture:** aarch64 | **Features:** neon, fcma
**Total intrinsics:** 58 (58 safe, 0 unsafe, 0 stable, 58 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = NeonFcmaToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: NeonFcmaToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: NeonFcmaToken, chunk: &mut [f32; 4]) {
    let v = vld1q_f32(chunk);  // safe!
    let doubled = vaddq_f32(v, v);  // value intrinsic (safe inside #[rite])
    vst1q_f32(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Unstable/Nightly (58 intrinsics)

| Name | Description | Instruction |
|------|-------------|-------------|
| `vcadd_rot270_f32` | Floating-point complex add | fcadd |
| `vcadd_rot90_f32` | Floating-point complex add | fcadd |
| `vcaddq_rot270_f32` | Floating-point complex add | fcadd |
| `vcaddq_rot270_f64` | Floating-point complex add | fcadd |
| `vcaddq_rot90_f32` | Floating-point complex add | fcadd |
| `vcaddq_rot90_f64` | Floating-point complex add | fcadd |
| `vcmla_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_lane_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_lane_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_laneq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_laneq_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot180_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot180_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot180_lane_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot180_lane_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot180_laneq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot180_laneq_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot270_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot270_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot270_lane_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot270_lane_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot270_laneq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot270_laneq_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot90_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot90_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot90_lane_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot90_lane_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot90_laneq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmla_rot90_laneq_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_f64` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_lane_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_lane_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_laneq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_laneq_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot180_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot180_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot180_f64` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot180_lane_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot180_lane_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot180_laneq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot180_laneq_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot270_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot270_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot270_f64` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot270_lane_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot270_lane_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot270_laneq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot270_laneq_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot90_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot90_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot90_f64` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot90_lane_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot90_lane_f32` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot90_laneq_f16` | Floating-point complex multiply accumulate | fcmla |
| `vcmlaq_rot90_laneq_f32` | Floating-point complex multiply accumulate | fcmla |


