# X64AdxToken — x86-64 ADX

Orthogonal proof for Multi-Precision Add-Carry (`adx`: `ADCX`/`ADOX`) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, adx
**Total intrinsics:** 2 (2 safe, 0 unsafe, 2 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64AdxToken::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64AdxToken, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64AdxToken, chunk: &mut [f32; 4]) {
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
| `_addcarryx_u32` | Adds unsigned 32-bit integers `a` and `b` with unsigned 8-bi... | adc | — |
| `_addcarryx_u64` | Adds unsigned 64-bit integers `a` and `b` with unsigned 8-bi... | adc | — |


