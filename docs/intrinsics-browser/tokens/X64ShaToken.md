# X64ShaToken — x86-64 SHA

Orthogonal proof for SHA-1 / SHA-256 hardware extensions (`sha`) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, sha
**Total intrinsics:** 7 (7 safe, 0 unsafe, 7 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64ShaToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64ShaToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64ShaToken, chunk: &mut [f32; 4]) {
    let v = _mm_loadu_ps(chunk);  // safe!
    let doubled = _mm_add_ps(v, v);  // value intrinsic (safe inside #[rite])
    _mm_storeu_ps(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (7 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_mm_sha1msg1_epu32` | Performs an intermediate calculation for the next four SHA1 ... | sha1msg1 | — |
| `_mm_sha1msg2_epu32` | Performs the final calculation for the next four SHA1 messag... | sha1msg2 | — |
| `_mm_sha1nexte_epu32` | Calculate SHA1 state variable E after four rounds of operati... | sha1nexte | — |
| `_mm_sha1rnds4_epu32` | Performs four rounds of SHA1 operation using an initial SHA1... | sha1rnds4 | — |
| `_mm_sha256msg1_epu32` | Performs an intermediate calculation for the next four SHA25... | sha256msg1 | — |
| `_mm_sha256msg2_epu32` | Performs the final calculation for the next four SHA256 mess... | sha256msg2 | — |
| `_mm_sha256rnds2_epu32` | Performs 2 rounds of SHA256 operation using an initial SHA25... | sha256rnds2 | — |


