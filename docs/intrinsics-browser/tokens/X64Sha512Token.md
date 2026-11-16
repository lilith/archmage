# X64Sha512Token — x86-64 SHA512

Orthogonal proof for SHA-512 vector instructions (`sha512` + `avx`) on x86-64.

**Architecture:** x86_64 | **Features:** sse, sse2, avx, sha512
**Total intrinsics:** 192 (162 safe, 30 unsafe, 192 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64Sha512Token::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64Sha512Token, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(4) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64Sha512Token, chunk: &mut [f32; 4]) {
    let v = _mm_loadu_ps(chunk);  // safe!
    let doubled = _mm_add_ps(v, v);  // value intrinsic (safe inside #[rite])
    _mm_storeu_ps(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```

## Safe Memory Operations (via import_intrinsics)

| Function | Safe Signature |
|----------|---------------|
| `_mm256_broadcast_pd` | `fn _mm256_broadcast_pd(mem_addr: &__m128d) -> __m256d` |
| `_mm256_broadcast_ps` | `fn _mm256_broadcast_ps(mem_addr: &__m128) -> __m256` |
| `_mm256_broadcast_sd` | `fn _mm256_broadcast_sd(mem_addr: &f64) -> __m256d` |
| `_mm256_broadcast_ss` | `fn _mm256_broadcast_ss(mem_addr: &f32) -> __m256` |
| `_mm256_loadu2_m128` | `fn _mm256_loadu2_m128(hiaddr: &[f32; 4], loaddr: &[f32; 4]) -> __m256` |
| `_mm256_loadu2_m128d` | `fn _mm256_loadu2_m128d(hiaddr: &[f64; 2], loaddr: &[f64; 2]) -> __m256d` |
| `_mm256_loadu2_m128i` | `fn _mm256_loadu2_m128i<T: Is128BitsUnaligned>(hiaddr: &T, loaddr: &T) -> __m256i` |
| `_mm256_loadu_pd` | `fn _mm256_loadu_pd(mem_addr: &[f64; 4]) -> __m256d` |
| `_mm256_loadu_ps` | `fn _mm256_loadu_ps(mem_addr: &[f32; 8]) -> __m256` |
| `_mm256_loadu_si256` | `fn _mm256_loadu_si256<T: Is256BitsUnaligned>(mem_addr: &T) -> __m256i` |
| `_mm256_storeu2_m128` | `fn _mm256_storeu2_m128(hiaddr: &mut [f32; 4], loaddr: &mut [f32; 4], a: __m256) -> ()` |
| `_mm256_storeu2_m128d` | `fn _mm256_storeu2_m128d(hiaddr: &mut [f64; 2], loaddr: &mut [f64; 2], a: __m256d) -> ()` |
| `_mm256_storeu2_m128i` | `fn _mm256_storeu2_m128i<T: Is128BitsUnaligned>(hiaddr: &mut T, loaddr: &mut T, a: __m256i) -> ()` |
| `_mm256_storeu_pd` | `fn _mm256_storeu_pd(mem_addr: &mut [f64; 4], a: __m256d) -> ()` |
| `_mm256_storeu_ps` | `fn _mm256_storeu_ps(mem_addr: &mut [f32; 8], a: __m256) -> ()` |
| `_mm256_storeu_si256` | `fn _mm256_storeu_si256<T: Is256BitsUnaligned>(mem_addr: &mut T, a: __m256i) -> ()` |
| `_mm_broadcast_ss` | `fn _mm_broadcast_ss(mem_addr: &f32) -> __m128` |


## All Intrinsics

### Stable, Safe (162 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_mm256_add_pd` | Adds packed double-precision (64-bit) floating-point element... | vaddpd | 3/1, 3/1 |
| `_mm256_add_ps` | Adds packed single-precision (32-bit) floating-point element... | vaddps | 3/1, 3/1 |
| `_mm256_addsub_pd` | Alternatively adds and subtracts packed double-precision (64... | vaddsubpd | — |
| `_mm256_addsub_ps` | Alternatively adds and subtracts packed single-precision (32... | vaddsubps | — |
| `_mm256_and_pd` | Computes the bitwise AND of a packed double-precision (64-bi... | vandp | 1/1, 1/1 |
| `_mm256_and_ps` | Computes the bitwise AND of packed single-precision (32-bit)... | vandps | 1/1, 1/1 |
| `_mm256_andnot_pd` | Computes the bitwise NOT of packed double-precision (64-bit)... | vandnp | 1/1, 1/1 |
| `_mm256_andnot_ps` | Computes the bitwise NOT of packed single-precision (32-bit)... | vandnps | 1/1, 1/1 |
| `_mm256_blend_pd` | Blends packed double-precision (64-bit) floating-point eleme... | vblendpd | 1/1, 1/1 |
| `_mm256_blend_ps` | Blends packed single-precision (32-bit) floating-point eleme... | vblendps | 1/1, 1/1 |
| `_mm256_blendv_pd` | Blends packed double-precision (64-bit) floating-point eleme... | vblendvpd | 1/1, 1/1 |
| `_mm256_blendv_ps` | Blends packed single-precision (32-bit) floating-point eleme... | vblendvps | 1/1, 1/1 |
| `_mm256_broadcast_pd` | Broadcasts 128 bits from memory (composed of 2 packed double... | vbroadcastf128 | — |
| `_mm256_broadcast_ps` | Broadcasts 128 bits from memory (composed of 4 packed single... | vbroadcastf128 | — |
| `_mm256_broadcast_sd` | Broadcasts a double-precision (64-bit) floating-point elemen... | vbroadcastsd | — |
| `_mm256_broadcast_ss` | Broadcasts a single-precision (32-bit) floating-point elemen... | vbroadcastss | — |
| `_mm256_castpd128_pd256` | Casts vector of type __m128d to type __m256d; the upper 128 ... |  | — |
| `_mm256_castpd256_pd128` | Casts vector of type __m256d to type __m128d |  | — |
| `_mm256_castpd_ps` | Cast vector of type __m256d to type __m256 |  | — |
| `_mm256_castpd_si256` | Casts vector of type __m256d to type __m256i |  | — |
| `_mm256_castps128_ps256` | Casts vector of type __m128 to type __m256; the upper 128 bi... |  | — |
| `_mm256_castps256_ps128` | Casts vector of type __m256 to type __m128 |  | — |
| `_mm256_castps_pd` | Cast vector of type __m256 to type __m256d |  | — |
| `_mm256_castps_si256` | Casts vector of type __m256 to type __m256i |  | — |
| `_mm256_castsi128_si256` | Casts vector of type __m128i to type __m256i; the upper 128 ... |  | — |
| `_mm256_castsi256_pd` | Casts vector of type __m256i to type __m256d |  | — |
| `_mm256_castsi256_ps` | Casts vector of type __m256i to type __m256 |  | — |
| `_mm256_castsi256_si128` | Casts vector of type __m256i to type __m128i |  | — |
| `_mm256_ceil_pd` | Rounds packed double-precision (64-bit) floating point eleme... | vroundpd | — |
| `_mm256_ceil_ps` | Rounds packed single-precision (32-bit) floating point eleme... | vroundps | — |
| `_mm256_cmp_pd` | Compares packed double-precision (64-bit) floating-point ele... | vcmpeqpd | 3/1, 3/1 |
| `_mm256_cmp_ps` | Compares packed single-precision (32-bit) floating-point ele... | vcmpeqps | 3/1, 3/1 |
| `_mm256_cvtepi32_pd` | Converts packed 32-bit integers in `a` to packed double-prec... | vcvtdq2pd | 4/1, 3/1 |
| `_mm256_cvtepi32_ps` | Converts packed 32-bit integers in `a` to packed single-prec... | vcvtdq2ps | 4/1, 3/1 |
| `_mm256_cvtpd_epi32` | Converts packed double-precision (64-bit) floating-point ele... | vcvtpd2dq | 4/1, 3/1 |
| `_mm256_cvtpd_ps` | Converts packed double-precision (64-bit) floating-point ele... | vcvtpd2ps | 4/1, 3/1 |
| `_mm256_cvtps_epi32` | Converts packed single-precision (32-bit) floating-point ele... | vcvtps2dq | 4/1, 3/1 |
| `_mm256_cvtps_pd` | Converts packed single-precision (32-bit) floating-point ele... | vcvtps2pd | 4/1, 3/1 |
| `_mm256_cvtsd_f64` | Returns the first element of the input vector of ` | movsd | 4/1, 3/1 |
| `_mm256_cvtsi256_si32` | Returns the first element of the input vector of ` |  | 4/1, 3/1 |
| `_mm256_cvtss_f32` | Returns the first element of the input vector of ` | movss | 4/1, 3/1 |
| `_mm256_cvttpd_epi32` | Converts packed double-precision (64-bit) floating-point ele... | vcvttpd2dq | 4/1, 3/1 |
| `_mm256_cvttps_epi32` | Converts packed single-precision (32-bit) floating-point ele... | vcvttps2dq | 4/1, 3/1 |
| `_mm256_div_pd` | Computes the division of each of the 4 packed 64-bit floatin... | vdivpd | 35/28, 13/10 |
| `_mm256_div_ps` | Computes the division of each of the 8 packed 32-bit floatin... | vdivps | 21/14, 10/7 |
| `_mm256_dp_ps` | Conditionally multiplies the packed single-precision (32-bit... | vdpps | — |
| `_mm256_extract_epi32` | Extracts a 32-bit integer from `a`, selected with `INDEX` |  | — |
| `_mm256_extract_epi64` | Extracts a 64-bit integer from `a`, selected with `INDEX` |  | — |
| `_mm256_extractf128_pd` | Extracts 128 bits (composed of 2 packed double-precision (64... | vextractf128 | — |
| `_mm256_extractf128_ps` | Extracts 128 bits (composed of 4 packed single-precision (32... | vextractf128 | — |
| `_mm256_extractf128_si256` | Extracts 128 bits (composed of integer data) from `a`, selec... | vextractf128 | — |
| `_mm256_floor_pd` | Rounds packed double-precision (64-bit) floating point eleme... | vroundpd | — |
| `_mm256_floor_ps` | Rounds packed single-precision (32-bit) floating point eleme... | vroundps | — |
| `_mm256_hadd_pd` | Horizontal addition of adjacent pairs in the two packed vect... | vhaddpd | 5/2, 4/2 |
| `_mm256_hadd_ps` | Horizontal addition of adjacent pairs in the two packed vect... | vhaddps | 5/2, 4/2 |
| `_mm256_hsub_pd` | Horizontal subtraction of adjacent pairs in the two packed v... | vhsubpd | 5/2, 4/2 |
| `_mm256_hsub_ps` | Horizontal subtraction of adjacent pairs in the two packed v... | vhsubps | 5/2, 4/2 |
| `_mm256_insert_epi16` | Copies `a` to result, and inserts the 16-bit integer `i` int... |  | — |
| `_mm256_insert_epi32` | Copies `a` to result, and inserts the 32-bit integer `i` int... |  | — |
| `_mm256_insert_epi64` | Copies `a` to result, and insert the 64-bit integer `i` into... |  | — |
| `_mm256_insert_epi8` | Copies `a` to result, and inserts the 8-bit integer `i` into... |  | — |
| `_mm256_insertf128_pd` | Copies `a` to result, then inserts 128 bits (composed of 2 p... | vinsertf128 | — |
| `_mm256_insertf128_ps` | Copies `a` to result, then inserts 128 bits (composed of 4 p... | vinsertf128 | — |
| `_mm256_insertf128_si256` | Copies `a` to result, then inserts 128 bits from `b` into re... | vinsertf128 | — |
| `_mm256_max_pd` | Compares packed double-precision (64-bit) floating-point ele... | vmaxpd | — |
| `_mm256_max_ps` | Compares packed single-precision (32-bit) floating-point ele... | vmaxps | — |
| `_mm256_min_pd` | Compares packed double-precision (64-bit) floating-point ele... | vminpd | — |
| `_mm256_min_ps` | Compares packed single-precision (32-bit) floating-point ele... | vminps | — |
| `_mm256_movedup_pd` | Duplicate even-indexed double-precision (64-bit) floating-po... | vmovddup | — |
| `_mm256_movehdup_ps` | Duplicate odd-indexed single-precision (32-bit) floating-poi... | vmovshdup | — |
| `_mm256_moveldup_ps` | Duplicate even-indexed single-precision (32-bit) floating-po... | vmovsldup | — |
| `_mm256_movemask_pd` | Sets each bit of the returned mask based on the most signifi... | vmovmskpd | — |
| `_mm256_movemask_ps` | Sets each bit of the returned mask based on the most signifi... | vmovmskps | — |
| `_mm256_mul_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vmulpd | 5/1, 3/1 |
| `_mm256_mul_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vmulps | 5/1, 3/1 |
| `_mm256_or_pd` | Computes the bitwise OR packed double-precision (64-bit) flo... | vorp | 1/1, 1/1 |
| `_mm256_or_ps` | Computes the bitwise OR packed single-precision (32-bit) flo... | vorps | 1/1, 1/1 |
| `_mm256_permute2f128_pd` | Shuffles 256 bits (composed of 4 packed double-precision (64... | vperm2f128 | 3/1, 2/1 |
| `_mm256_permute2f128_ps` | Shuffles 256 bits (composed of 8 packed single-precision (32... | vperm2f128 | 3/1, 2/1 |
| `_mm256_permute2f128_si256` | Shuffles 128-bits (composed of integer data) selected by `im... | vperm2f128 | 3/1, 2/1 |
| `_mm256_permute_pd` | Shuffles double-precision (64-bit) floating-point elements i... | vshufpd | 3/1, 2/1 |
| `_mm256_permute_ps` | Shuffles single-precision (32-bit) floating-point elements i... | vshufps | 3/1, 2/1 |
| `_mm256_permutevar_pd` | Shuffles double-precision (64-bit) floating-point elements i... | vpermilpd | 3/1, 2/1 |
| `_mm256_permutevar_ps` | Shuffles single-precision (32-bit) floating-point elements i... | vpermilps | 3/1, 2/1 |
| `_mm256_rcp_ps` | Computes the approximate reciprocal of packed single-precisi... | vrcpps | 7/1, 4/1 |
| `_mm256_round_pd` | Rounds packed double-precision (64-bit) floating point eleme... | vroundpd | — |
| `_mm256_round_ps` | Rounds packed single-precision (32-bit) floating point eleme... | vroundps | — |
| `_mm256_rsqrt_ps` | Computes the approximate reciprocal square root of packed si... | vrsqrtps | 7/1, 4/1 |
| `_mm256_set1_epi16` | Broadcasts 16-bit integer `a` to all elements of returned ve... | vpshufb | — |
| `_mm256_set1_epi32` | Broadcasts 32-bit integer `a` to all elements of returned ve... |  | — |
| `_mm256_set1_epi64x` | Broadcasts 64-bit integer `a` to all elements of returned ve... | vinsertf128 | — |
| `_mm256_set1_epi8` | Broadcasts 8-bit integer `a` to all elements of returned vec... |  | — |
| `_mm256_set1_pd` | Broadcasts double-precision (64-bit) floating-point value `a... |  | — |
| `_mm256_set1_ps` | Broadcasts single-precision (32-bit) floating-point value `a... |  | — |
| `_mm256_set_epi16` | Sets packed 16-bit integers in returned vector with the supp... |  | — |
| `_mm256_set_epi32` | Sets packed 32-bit integers in returned vector with the supp... |  | — |
| `_mm256_set_epi64x` | Sets packed 64-bit integers in returned vector with the supp... |  | — |
| `_mm256_set_epi8` | Sets packed 8-bit integers in returned vector with the suppl... |  | — |
| `_mm256_set_m128` | Sets packed __m256 returned vector with the supplied values | vinsertf128 | — |
| `_mm256_set_m128d` | Sets packed __m256d returned vector with the supplied values | vinsertf128 | — |
| `_mm256_set_m128i` | Sets packed __m256i returned vector with the supplied values | vinsertf128 | — |
| `_mm256_set_pd` | Sets packed double-precision (64-bit) floating-point element... |  | — |
| `_mm256_set_ps` | Sets packed single-precision (32-bit) floating-point element... |  | — |
| `_mm256_setr_epi16` | Sets packed 16-bit integers in returned vector with the supp... |  | — |
| `_mm256_setr_epi32` | Sets packed 32-bit integers in returned vector with the supp... |  | — |
| `_mm256_setr_epi64x` | Sets packed 64-bit integers in returned vector with the supp... |  | — |
| `_mm256_setr_epi8` | Sets packed 8-bit integers in returned vector with the suppl... |  | — |
| `_mm256_setr_m128` | Sets packed __m256 returned vector with the supplied values | vinsertf128 | — |
| `_mm256_setr_m128d` | Sets packed __m256d returned vector with the supplied values | vinsertf128 | — |
| `_mm256_setr_m128i` | Sets packed __m256i returned vector with the supplied values | vinsertf128 | — |
| `_mm256_setr_pd` | Sets packed double-precision (64-bit) floating-point element... |  | — |
| `_mm256_setr_ps` | Sets packed single-precision (32-bit) floating-point element... |  | — |
| `_mm256_setzero_pd` | Returns vector of type __m256d with all elements set to zero | vxorp | — |
| `_mm256_setzero_ps` | Returns vector of type __m256 with all elements set to zero | vxorps | — |
| `_mm256_setzero_si256` | Returns vector of type __m256i with all elements set to zero | vxor | — |
| `_mm256_sha512msg1_epi64` | This intrinsic is one of the two SHA512 message scheduling i... | vsha512msg1 | — |
| `_mm256_sha512msg2_epi64` | This intrinsic is one of the two SHA512 message scheduling i... | vsha512msg2 | — |
| `_mm256_sha512rnds2_epi64` | This intrinsic performs two rounds of SHA512 operation using... | vsha512rnds2 | — |
| `_mm256_shuffle_pd` | Shuffles double-precision (64-bit) floating-point elements w... | vshufpd | 1/1, 1/1 |
| `_mm256_shuffle_ps` | Shuffles single-precision (32-bit) floating-point elements i... | vshufps | 1/1, 1/1 |
| `_mm256_sqrt_pd` | Returns the square root of packed double-precision (64-bit) ... | vsqrtpd | 28/28, 20/13 |
| `_mm256_sqrt_ps` | Returns the square root of packed single-precision (32-bit) ... | vsqrtps | 19/14, 14/7 |
| `_mm256_sub_pd` | Subtracts packed double-precision (64-bit) floating-point el... | vsubpd | — |
| `_mm256_sub_ps` | Subtracts packed single-precision (32-bit) floating-point el... | vsubps | — |
| `_mm256_testc_pd` | Computes the bitwise AND of 256 bits (representing double-pr... | vtestpd | — |
| `_mm256_testc_ps` | Computes the bitwise AND of 256 bits (representing single-pr... | vtestps | — |
| `_mm256_testc_si256` | Computes the bitwise AND of 256 bits (representing integer d... | vptest | — |
| `_mm256_testnzc_pd` | Computes the bitwise AND of 256 bits (representing double-pr... | vtestpd | — |
| `_mm256_testnzc_ps` | Computes the bitwise AND of 256 bits (representing single-pr... | vtestps | — |
| `_mm256_testnzc_si256` | Computes the bitwise AND of 256 bits (representing integer d... | vptest | — |
| `_mm256_testz_pd` | Computes the bitwise AND of 256 bits (representing double-pr... | vtestpd | — |
| `_mm256_testz_ps` | Computes the bitwise AND of 256 bits (representing single-pr... | vtestps | — |
| `_mm256_testz_si256` | Computes the bitwise AND of 256 bits (representing integer d... | vptest | — |
| `_mm256_undefined_pd` | Returns vector of type `__m256d` with indeterminate elements... |  | — |
| `_mm256_undefined_ps` | Returns vector of type `__m256` with indeterminate elements.... |  | — |
| `_mm256_undefined_si256` | Returns vector of type __m256i with with indeterminate eleme... |  | — |
| `_mm256_unpackhi_pd` | Unpacks and interleave double-precision (64-bit) floating-po... | vunpckhpd | 1/1, 1/1 |
| `_mm256_unpackhi_ps` | Unpacks and interleave single-precision (32-bit) floating-po... | vunpckhps | 1/1, 1/1 |
| `_mm256_unpacklo_pd` | Unpacks and interleave double-precision (64-bit) floating-po... | vunpcklpd | 1/1, 1/1 |
| `_mm256_unpacklo_ps` | Unpacks and interleave single-precision (32-bit) floating-po... | vunpcklps | 1/1, 1/1 |
| `_mm256_xor_pd` | Computes the bitwise XOR of packed double-precision (64-bit)... | vxorp | 1/1, 1/1 |
| `_mm256_xor_ps` | Computes the bitwise XOR of packed single-precision (32-bit)... | vxorps | 1/1, 1/1 |
| `_mm256_zeroall` | Zeroes the contents of all XMM or YMM registers.  This opera... | vzeroall | — |
| `_mm256_zeroupper` | Zeroes the upper 128 bits of all YMM registers; the lower 12... | vzeroupper | — |
| `_mm256_zextpd128_pd256` | Constructs a 256-bit floating-point vector of ` |  | — |
| `_mm256_zextps128_ps256` | Constructs a 256-bit floating-point vector of ` |  | — |
| `_mm256_zextsi128_si256` | Constructs a 256-bit integer vector from a 128-bit integer v... |  | — |
| `_mm_broadcast_ss` | Broadcasts a single-precision (32-bit) floating-point elemen... | vbroadcastss | — |
| `_mm_cmp_pd` | Compares packed double-precision (64-bit) floating-point ele... | vcmpeqpd | 3/1, 3/1 |
| `_mm_cmp_ps` | Compares packed single-precision (32-bit) floating-point ele... | vcmpeqps | 3/1, 3/1 |
| `_mm_cmp_sd` | Compares the lower double-precision (64-bit) floating-point ... | vcmpeqsd | — |
| `_mm_cmp_ss` | Compares the lower single-precision (32-bit) floating-point ... | vcmpeqss | — |
| `_mm_permute_pd` | Shuffles double-precision (64-bit) floating-point elements i... | vshufpd | 1/1, 1/1 |
| `_mm_permute_ps` | Shuffles single-precision (32-bit) floating-point elements i... | vshufps | 1/1, 1/1 |
| `_mm_permutevar_pd` | Shuffles double-precision (64-bit) floating-point elements i... | vpermilpd | 1/1, 1/1 |
| `_mm_permutevar_ps` | Shuffles single-precision (32-bit) floating-point elements i... | vpermilps | 1/1, 1/1 |
| `_mm_testc_pd` | Computes the bitwise AND of 128 bits (representing double-pr... | vtestpd | — |
| `_mm_testc_ps` | Computes the bitwise AND of 128 bits (representing single-pr... | vtestps | — |
| `_mm_testnzc_pd` | Computes the bitwise AND of 128 bits (representing double-pr... | vtestpd | — |
| `_mm_testnzc_ps` | Computes the bitwise AND of 128 bits (representing single-pr... | vtestps | — |
| `_mm_testz_pd` | Computes the bitwise AND of 128 bits (representing double-pr... | vtestpd | — |
| `_mm_testz_ps` | Computes the bitwise AND of 128 bits (representing single-pr... | vtestps | — |

### Stable, Unsafe (30 intrinsics) — use import_intrinsics for safe versions

| Name | Description | Safe Variant |
|------|-------------|--------------|
| `_mm256_lddqu_si256` | Loads 256-bits of integer data from unaligned memory into re... | — |
| `_mm256_load_pd` | Loads 256-bits (composed of 4 packed double-precision (64-bi... | — |
| `_mm256_load_ps` | Loads 256-bits (composed of 8 packed single-precision (32-bi... | — |
| `_mm256_load_si256` | Loads 256-bits of integer data from memory into result. `mem... | — |
| `_mm256_loadu2_m128` | Loads two 128-bit values (composed of 4 packed single-precis... | `_mm256_loadu2_m128` (safe via import_intrinsics) |
| `_mm256_loadu2_m128d` | Loads two 128-bit values (composed of 2 packed double-precis... | `_mm256_loadu2_m128d` (safe via import_intrinsics) |
| `_mm256_loadu2_m128i` | Loads two 128-bit values (composed of integer data) from mem... | `_mm256_loadu2_m128i` (safe via import_intrinsics) |
| `_mm256_loadu_pd` | Loads 256-bits (composed of 4 packed double-precision (64-bi... | `_mm256_loadu_pd` (safe via import_intrinsics) |
| `_mm256_loadu_ps` | Loads 256-bits (composed of 8 packed single-precision (32-bi... | `_mm256_loadu_ps` (safe via import_intrinsics) |
| `_mm256_loadu_si256` | Loads 256-bits of integer data from memory into result. `mem... | `_mm256_loadu_si256` (safe via import_intrinsics) |
| `_mm256_maskload_pd` | Loads packed double-precision (64-bit) floating-point elemen... | — |
| `_mm256_maskload_ps` | Loads packed single-precision (32-bit) floating-point elemen... | — |
| `_mm256_maskstore_pd` | Stores packed double-precision (64-bit) floating-point eleme... | — |
| `_mm256_maskstore_ps` | Stores packed single-precision (32-bit) floating-point eleme... | — |
| `_mm256_store_pd` | Stores 256-bits (composed of 4 packed double-precision (64-b... | — |
| `_mm256_store_ps` | Stores 256-bits (composed of 8 packed single-precision (32-b... | — |
| `_mm256_store_si256` | Stores 256-bits of integer data from `a` into memory. `mem_a... | — |
| `_mm256_storeu2_m128` | Stores the high and low 128-bit halves (each composed of 4 p... | `_mm256_storeu2_m128` (safe via import_intrinsics) |
| `_mm256_storeu2_m128d` | Stores the high and low 128-bit halves (each composed of 2 p... | `_mm256_storeu2_m128d` (safe via import_intrinsics) |
| `_mm256_storeu2_m128i` | Stores the high and low 128-bit halves (each composed of int... | `_mm256_storeu2_m128i` (safe via import_intrinsics) |
| `_mm256_storeu_pd` | Stores 256-bits (composed of 4 packed double-precision (64-b... | `_mm256_storeu_pd` (safe via import_intrinsics) |
| `_mm256_storeu_ps` | Stores 256-bits (composed of 8 packed single-precision (32-b... | `_mm256_storeu_ps` (safe via import_intrinsics) |
| `_mm256_storeu_si256` | Stores 256-bits of integer data from `a` into memory. `mem_a... | `_mm256_storeu_si256` (safe via import_intrinsics) |
| `_mm256_stream_pd` | Moves double-precision values from a 256-bit vector of ` | — |
| `_mm256_stream_ps` | Moves single-precision floating point values from a 256-bit ... | — |
| `_mm256_stream_si256` | Moves integer data from a 256-bit integer vector to a 32-byt... | — |
| `_mm_maskload_pd` | Loads packed double-precision (64-bit) floating-point elemen... | — |
| `_mm_maskload_ps` | Loads packed single-precision (32-bit) floating-point elemen... | — |
| `_mm_maskstore_pd` | Stores packed double-precision (64-bit) floating-point eleme... | — |
| `_mm_maskstore_ps` | Stores packed single-precision (32-bit) floating-point eleme... | — |


