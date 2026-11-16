# X64GfniToken — x86-64 GFNI

Orthogonal proof for Galois Field New Instructions (`gfni`) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, gfni
**Total intrinsics:** 3 (3 safe, 0 unsafe, 3 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64GfniToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64GfniToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64GfniToken, chunk: &mut [f32; 4]) {
    let v = _mm_loadu_ps(chunk);  // safe!
    let doubled = _mm_add_ps(v, v);  // value intrinsic (safe inside #[rite])
    _mm_storeu_ps(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (3 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_mm_gf2p8affine_epi64_epi8` | Performs an affine transformation on the packed bytes in x. ... | gf2p8affineqb | — |
| `_mm_gf2p8affineinv_epi64_epi8` | Performs an affine transformation on the inverted packed byt... | gf2p8affineinvqb | — |
| `_mm_gf2p8mul_epi8` | Performs a multiplication in GF(2^8) on the packed bytes. Th... | gf2p8mulb | — |


