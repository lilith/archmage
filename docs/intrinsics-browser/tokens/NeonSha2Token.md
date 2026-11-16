# NeonSha2Token — NEON+SHA2

Proof that NEON + SHA-256 (`sha2`) is available on AArch64.

**Architecture:** aarch64 | **Features:** neon, sha2
**Total intrinsics:** 10 (10 safe, 0 unsafe, 10 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = NeonSha2Token::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: NeonSha2Token, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: NeonSha2Token, chunk: &mut [f32; 4]) {
    let v = vld1q_f32(chunk);  // safe!
    let doubled = vaddq_f32(v, v);  // value intrinsic (safe inside #[rite])
    vst1q_f32(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (10 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `vsha1cq_u32` | SHA1 hash update accelerator, choose | sha1c | — |
| `vsha1h_u32` | SHA1 fixed rotate | sha1h | — |
| `vsha1mq_u32` | SHA1 hash update accelerator, majority | sha1m | — |
| `vsha1pq_u32` | SHA1 hash update accelerator, parity | sha1p | — |
| `vsha1su0q_u32` | SHA1 schedule update accelerator, first part | sha1su0 | — |
| `vsha1su1q_u32` | SHA1 schedule update accelerator, second part | sha1su1 | — |
| `vsha256h2q_u32` | SHA1 schedule update accelerator, upper part | sha256h2 | — |
| `vsha256hq_u32` | SHA1 schedule update accelerator, first part | sha256h | — |
| `vsha256su0q_u32` | SHA256 schedule update accelerator, first part | sha256su0 | — |
| `vsha256su1q_u32` | SHA256 schedule update accelerator, second part | sha256su1 | — |


