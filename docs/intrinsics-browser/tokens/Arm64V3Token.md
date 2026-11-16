# Arm64V3Token — Arm64-v3

Proof that the full modern ARM SIMD feature set is available (Arm64-v3).

**Architecture:** aarch64 | **Features:** neon, crc, rdm, dotprod, fp16, aes, sha2, fhm, fcma, sha3, i8mm, bf16
**Total intrinsics:** 4 (4 safe, 0 unsafe, 0 stable, 4 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = Arm64V3Token::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: Arm64V3Token, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: Arm64V3Token, chunk: &mut [f32; 4]) {
    let v = vld1q_f32(chunk);  // safe!
    let doubled = vaddq_f32(v, v);  // value intrinsic (safe inside #[rite])
    vst1q_f32(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Unstable/Nightly (4 intrinsics)

| Name | Description | Instruction |
|------|-------------|-------------|
| `vcadd_rot270_f16` | Floating-point complex add | fcadd |
| `vcadd_rot90_f16` | Floating-point complex add | fcadd |
| `vcaddq_rot270_f16` | Floating-point complex add | fcadd |
| `vcaddq_rot90_f16` | Floating-point complex add | fcadd |


