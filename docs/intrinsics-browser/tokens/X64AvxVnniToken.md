# X64AvxVnniToken — x86-64 AVX-VNNI

Orthogonal proof for VEX-encoded AVX-VNNI (`avxvnni`: `vpdpbusd`/`vpdpwssd`) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, avxvnni
**Total intrinsics:** 8 (8 safe, 0 unsafe, 8 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64AvxVnniToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64AvxVnniToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64AvxVnniToken, chunk: &mut [f32; 4]) {
    let v = _mm_loadu_ps(chunk);  // safe!
    let doubled = _mm_add_ps(v, v);  // value intrinsic (safe inside #[rite])
    _mm_storeu_ps(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (8 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_mm256_dpbusd_avx_epi32` | Multiply groups of 4 adjacent pairs of unsigned 8-bit intege... | vpdpbusd | — |
| `_mm256_dpbusds_avx_epi32` | Multiply groups of 4 adjacent pairs of unsigned 8-bit intege... | vpdpbusds | — |
| `_mm256_dpwssd_avx_epi32` | Multiply groups of 2 adjacent pairs of signed 16-bit integer... | vpdpwssd | — |
| `_mm256_dpwssds_avx_epi32` | Multiply groups of 2 adjacent pairs of signed 16-bit integer... | vpdpwssds | — |
| `_mm_dpbusd_avx_epi32` | Multiply groups of 4 adjacent pairs of unsigned 8-bit intege... | vpdpbusd | — |
| `_mm_dpbusds_avx_epi32` | Multiply groups of 4 adjacent pairs of unsigned 8-bit intege... | vpdpbusds | — |
| `_mm_dpwssd_avx_epi32` | Multiply groups of 2 adjacent pairs of signed 16-bit integer... | vpdpwssd | — |
| `_mm_dpwssds_avx_epi32` | Multiply groups of 2 adjacent pairs of signed 16-bit integer... | vpdpwssds | — |


