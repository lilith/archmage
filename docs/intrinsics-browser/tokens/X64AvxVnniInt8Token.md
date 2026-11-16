# X64AvxVnniInt8Token — x86-64 AVX-VNNI-INT8

Orthogonal proof for AVX-VNNI-INT8 (`avxvnniint8`: signed/unsigned int8 dot products) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, avxvnniint8
**Total intrinsics:** 12 (12 safe, 0 unsafe, 12 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64AvxVnniInt8Token::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64AvxVnniInt8Token, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64AvxVnniInt8Token, chunk: &mut [f32; 4]) {
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
| `_mm256_dpbssd_epi32` | Multiply groups of 4 adjacent pairs of signed 8-bit integers... | vpdpbssd | — |
| `_mm256_dpbssds_epi32` | Multiply groups of 4 adjacent pairs of signed 8-bit integers... | vpdpbssds | — |
| `_mm256_dpbsud_epi32` | Multiply groups of 4 adjacent pairs of signed 8-bit integers... | vpdpbsud | — |
| `_mm256_dpbsuds_epi32` | Multiply groups of 4 adjacent pairs of signed 8-bit integers... | vpdpbsuds | — |
| `_mm256_dpbuud_epi32` | Multiply groups of 4 adjacent pairs of unsigned 8-bit intege... | vpdpbuud | — |
| `_mm256_dpbuuds_epi32` | Multiply groups of 4 adjacent pairs of unsigned 8-bit intege... | vpdpbuuds | — |
| `_mm_dpbssd_epi32` | Multiply groups of 4 adjacent pairs of signed 8-bit integers... | vpdpbssd | — |
| `_mm_dpbssds_epi32` | Multiply groups of 4 adjacent pairs of signed 8-bit integers... | vpdpbssds | — |
| `_mm_dpbsud_epi32` | Multiply groups of 4 adjacent pairs of signed 8-bit integers... | vpdpbsud | — |
| `_mm_dpbsuds_epi32` | Multiply groups of 4 adjacent pairs of signed 8-bit integers... | vpdpbsuds | — |
| `_mm_dpbuud_epi32` | Multiply groups of 4 adjacent pairs of unsigned 8-bit intege... | vpdpbuud | — |
| `_mm_dpbuuds_epi32` | Multiply groups of 4 adjacent pairs of unsigned 8-bit intege... | vpdpbuuds | — |


