# X64Avx512Bf16Token — AVX-512 BF16

Proof that AVX-512 BF16 (BFloat16 dot products & conversions) is available.

**Architecture:** x86_64 | **Features:** sse, sse2, sse3, ssse3, sse4.1, sse4.2, popcnt, cmpxchg16b, avx, avx2, fma, bmi1, bmi2, f16c, lzcnt, movbe, pclmulqdq, aes, avx512f, avx512bw, avx512cd, avx512dq, avx512vl, avx512bf16
**Total intrinsics:** 38 (38 safe, 0 unsafe, 36 stable, 2 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64Avx512Bf16Token::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64Avx512Bf16Token, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(16) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64Avx512Bf16Token, chunk: &mut [f32; 16]) {
    let v = _mm512_loadu_ps(&*chunk);  // safe inside #[rite]
    let doubled = _mm512_add_ps(v, v);
    _mm512_storeu_ps(chunk, doubled);
}
// Use #![forbid(unsafe_code)] — import_intrinsics provides safe memory ops.
```



## All Intrinsics

### Stable, Safe (36 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_mm256_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm256_cvtneps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm256_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm256_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |
| `_mm256_mask_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm256_mask_cvtneps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm256_mask_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm256_mask_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |
| `_mm256_maskz_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm256_maskz_cvtneps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm256_maskz_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm256_maskz_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |
| `_mm512_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm512_cvtneps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm512_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm512_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |
| `_mm512_mask_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm512_mask_cvtneps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm512_mask_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm512_mask_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |
| `_mm512_maskz_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm512_maskz_cvtneps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm512_maskz_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm512_maskz_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |
| `_mm_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm_cvtneps_pbh` | Converts packed single-precision (32-bit) floating-point ele... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |
| `_mm_mask_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm_mask_cvtneps_pbh` | Converts packed single-precision (32-bit) floating-point ele... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm_mask_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm_mask_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |
| `_mm_maskz_cvtne2ps_pbh` | Convert packed single-precision (32-bit) floating-point elem... | "vcvtne2ps2bf16" | 4/1, 3/1 |
| `_mm_maskz_cvtneps_pbh` | Converts packed single-precision (32-bit) floating-point ele... | "vcvtneps2bf16" | 4/1, 3/1 |
| `_mm_maskz_cvtpbh_ps` | Converts packed BF16 (16-bit) floating-point elements in a t... |  | 4/1, 3/1 |
| `_mm_maskz_dpbf16_ps` | Compute dot-product of BF16 (16-bit) floating-point pairs in... | "vdpbf16ps" | — |

### Unstable/Nightly (2 intrinsics)

| Name | Description | Instruction |
|------|-------------|-------------|
| `_mm_cvtness_sbh` | Converts a single-precision (32-bit) floating-point element ... |  |
| `_mm_cvtsbh_ss` | Converts a single BF16 (16-bit) floating-point element in a ... |  |


