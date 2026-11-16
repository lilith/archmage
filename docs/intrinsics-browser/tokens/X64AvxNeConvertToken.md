# X64AvxNeConvertToken — x86-64 AVX-NE-CONVERT

Orthogonal proof for AVX-NE-CONVERT (`avxneconvert`: BF16/FP16 conversion without AVX-512) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, avxneconvert
**Total intrinsics:** 14 (2 safe, 12 unsafe, 10 stable, 4 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64AvxNeConvertToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64AvxNeConvertToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64AvxNeConvertToken, chunk: &mut [f32; 4]) {
    let v = _mm_loadu_ps(chunk);  // safe!
    let doubled = _mm_add_ps(v, v);  // value intrinsic (safe inside #[rite])
    _mm_storeu_ps(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (2 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_mm256_cvtneps_avx_pbh` | Convert packed single precision (32-bit) floating-point elem... | vcvtneps2bf16 | 4/1, 3/1 |
| `_mm_cvtneps_avx_pbh` | Convert packed single precision (32-bit) floating-point elem... | vcvtneps2bf16 | 4/1, 3/1 |

### Stable, Unsafe (8 intrinsics) — use import_intrinsics for safe versions

| Name | Description | Safe Variant |
|------|-------------|--------------|
| `_mm256_cvtneebf16_ps` | Convert packed BF16 (16-bit) floating-point even-indexed ele... | — |
| `_mm256_cvtneeph_ps` | Convert packed half-precision (16-bit) floating-point even-i... | — |
| `_mm256_cvtneobf16_ps` | Convert packed BF16 (16-bit) floating-point odd-indexed elem... | — |
| `_mm256_cvtneoph_ps` | Convert packed half-precision (16-bit) floating-point odd-in... | — |
| `_mm_cvtneebf16_ps` | Convert packed BF16 (16-bit) floating-point even-indexed ele... | — |
| `_mm_cvtneeph_ps` | Convert packed half-precision (16-bit) floating-point even-i... | — |
| `_mm_cvtneobf16_ps` | Convert packed BF16 (16-bit) floating-point odd-indexed elem... | — |
| `_mm_cvtneoph_ps` | Convert packed half-precision (16-bit) floating-point odd-in... | — |

### Unstable/Nightly (4 intrinsics)

| Name | Description | Instruction |
|------|-------------|-------------|
| `_mm256_bcstnebf16_ps` | Convert scalar BF16 (16-bit) floating point element stored a... | vbcstnebf162ps |
| `_mm256_bcstnesh_ps` | Convert scalar half-precision (16-bit) floating-point elemen... | vbcstnesh2ps |
| `_mm_bcstnebf16_ps` | Convert scalar BF16 (16-bit) floating point element stored a... | vbcstnebf162ps |
| `_mm_bcstnesh_ps` | Convert scalar half-precision (16-bit) floating-point elemen... | vbcstnesh2ps |


