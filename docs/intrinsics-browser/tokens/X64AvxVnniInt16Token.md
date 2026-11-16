# X64AvxVnniInt16Token — x86-64 AVX-VNNI-INT16

Orthogonal proof for AVX-VNNI-INT16 (`avxvnniint16`: mixed-sign int16 dot products) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, avxvnniint16
**Total intrinsics:** 12 (12 safe, 0 unsafe, 12 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64AvxVnniInt16Token::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64AvxVnniInt16Token, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64AvxVnniInt16Token, chunk: &mut [f32; 4]) {
    let v = _mm_loadu_ps(chunk);  // safe!
    let doubled = _mm_add_ps(v, v);  // value intrinsic (safe inside #[rite])
    _mm_storeu_ps(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (12 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_mm256_dpwsud_epi32` | Multiply groups of 2 adjacent pairs of signed 16-bit integer... | vpdpwsud | — |
| `_mm256_dpwsuds_epi32` | Multiply groups of 2 adjacent pairs of signed 16-bit integer... | vpdpwsuds | — |
| `_mm256_dpwusd_epi32` | Multiply groups of 2 adjacent pairs of unsigned 16-bit integ... | vpdpwusd | — |
| `_mm256_dpwusds_epi32` | Multiply groups of 2 adjacent pairs of unsigned 16-bit integ... | vpdpwusds | — |
| `_mm256_dpwuud_epi32` | Multiply groups of 2 adjacent pairs of unsigned 16-bit integ... | vpdpwuud | — |
| `_mm256_dpwuuds_epi32` | Multiply groups of 2 adjacent pairs of unsigned 16-bit integ... | vpdpwuuds | — |
| `_mm_dpwsud_epi32` | Multiply groups of 2 adjacent pairs of signed 16-bit integer... | vpdpwsud | — |
| `_mm_dpwsuds_epi32` | Multiply groups of 2 adjacent pairs of signed 16-bit integer... | vpdpwsuds | — |
| `_mm_dpwusd_epi32` | Multiply groups of 2 adjacent pairs of unsigned 16-bit integ... | vpdpwusd | — |
| `_mm_dpwusds_epi32` | Multiply groups of 2 adjacent pairs of unsigned 16-bit integ... | vpdpwusds | — |
| `_mm_dpwuud_epi32` | Multiply groups of 2 adjacent pairs of unsigned 16-bit integ... | vpdpwuud | — |
| `_mm_dpwuuds_epi32` | Multiply groups of 2 adjacent pairs of unsigned 16-bit integ... | vpdpwuuds | — |


