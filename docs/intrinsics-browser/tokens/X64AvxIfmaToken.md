# X64AvxIfmaToken — x86-64 AVX-IFMA

Orthogonal proof for VEX-encoded AVX-IFMA (`avxifma`: 52-bit integer FMA) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, avxifma
**Total intrinsics:** 4 (4 safe, 0 unsafe, 4 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64AvxIfmaToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64AvxIfmaToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64AvxIfmaToken, chunk: &mut [f32; 4]) {
    let v = _mm_loadu_ps(chunk);  // safe!
    let doubled = _mm_add_ps(v, v);  // value intrinsic (safe inside #[rite])
    _mm_storeu_ps(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (4 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_mm256_madd52hi_avx_epu64` | Multiply packed unsigned 52-bit integers in each 64-bit elem... | vpmadd52huq | — |
| `_mm256_madd52lo_avx_epu64` | Multiply packed unsigned 52-bit integers in each 64-bit elem... | vpmadd52luq | — |
| `_mm_madd52hi_avx_epu64` | Multiply packed unsigned 52-bit integers in each 64-bit elem... | vpmadd52huq | — |
| `_mm_madd52lo_avx_epu64` | Multiply packed unsigned 52-bit integers in each 64-bit elem... | vpmadd52luq | — |


