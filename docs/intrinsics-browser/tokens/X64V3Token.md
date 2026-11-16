# X64V3Token (Desktop64) — x86-64-v3

Proof that AVX2 + FMA + BMI1/2 + F16C + LZCNT are available (x86-64-v3 level).

**Architecture:** x86_64 | **Features:** sse, sse2, sse3, ssse3, sse4.1, sse4.2, popcnt, cmpxchg16b, avx, avx2, fma, bmi1, bmi2, f16c, lzcnt, movbe
**Total intrinsics:** 256 (215 safe, 41 unsafe, 256 stable, 0 unstable/unknown)

## Usage

```rust
use archmage::prelude::*;

if let Some(token) = X64V3Token::summon() {
    process(token, &mut data);
}

#[arcane(import_intrinsics)]  // Entry point only
fn process(token: X64V3Token, data: &mut [f32]) {
    for chunk in data.chunks_exact_mut(8) {
        process_chunk(token, chunk.try_into().unwrap());
    }
}

#[rite(import_intrinsics)]  // All inner helpers
fn process_chunk(_: X64V3Token, chunk: &mut [f32; 8]) {
    let v = _mm256_loadu_ps(chunk);  // safe!
    let doubled = _mm256_add_ps(v, v);    // value intrinsic (safe inside #[rite])
    _mm256_storeu_ps(chunk, doubled);  // safe!
}
// No unsafe anywhere. Use #![forbid(unsafe_code)] in your crate.
```



## All Intrinsics

### Stable, Safe (215 intrinsics)

| Name | Description | Instruction | Timing (H/Z4) |
|------|-------------|-------------|---------------|
| `_andn_u32` | Bitwise logical `AND` of inverted `a` with `b` | andn | — |
| `_andn_u64` | Bitwise logical `AND` of inverted `a` with `b` | andn | — |
| `_bextr2_u32` | Extracts bits of `a` specified by `control` into the least s... | bextr | — |
| `_bextr2_u64` | Extracts bits of `a` specified by `control` into the least s... | bextr | — |
| `_bextr_u32` | Extracts bits in range | bextr | — |
| `_bextr_u64` | Extracts bits in range | bextr | — |
| `_blsi_u32` | Extracts lowest set isolated bit | blsi | — |
| `_blsi_u64` | Extracts lowest set isolated bit | blsi | — |
| `_blsmsk_u32` | Gets mask up to lowest set bit | blsmsk | — |
| `_blsmsk_u64` | Gets mask up to lowest set bit | blsmsk | — |
| `_blsr_u32` | Resets the lowest set bit of `x`.  If `x` is sets CF | blsr | — |
| `_blsr_u64` | Resets the lowest set bit of `x`.  If `x` is sets CF | blsr | — |
| `_bzhi_u32` | Zeroes higher bits of `a` >= `index` | bzhi | — |
| `_bzhi_u64` | Zeroes higher bits of `a` >= `index` | bzhi | — |
| `_lzcnt_u32` | Counts the leading most significant zero bits.  When the ope... | lzcnt | — |
| `_lzcnt_u64` | Counts the leading most significant zero bits.  When the ope... | lzcnt | — |
| `_mm256_abs_epi16` | Computes the absolute values of packed 16-bit integers in `a... | vpabsw | — |
| `_mm256_abs_epi32` | Computes the absolute values of packed 32-bit integers in `a... | vpabsd | — |
| `_mm256_abs_epi8` | Computes the absolute values of packed 8-bit integers in `a` | vpabsb | — |
| `_mm256_add_epi16` | Adds packed 16-bit integers in `a` and `b` | vpaddw | 1/1, 1/1 |
| `_mm256_add_epi32` | Adds packed 32-bit integers in `a` and `b` | vpaddd | 1/1, 1/1 |
| `_mm256_add_epi64` | Adds packed 64-bit integers in `a` and `b` | vpaddq | 1/1, 1/1 |
| `_mm256_add_epi8` | Adds packed 8-bit integers in `a` and `b` | vpaddb | 1/1, 1/1 |
| `_mm256_adds_epi16` | Adds packed 16-bit integers in `a` and `b` using saturation | vpaddsw | 1/1, 1/1 |
| `_mm256_adds_epi8` | Adds packed 8-bit integers in `a` and `b` using saturation | vpaddsb | 1/1, 1/1 |
| `_mm256_adds_epu16` | Adds packed unsigned 16-bit integers in `a` and `b` using sa... | vpaddusw | — |
| `_mm256_adds_epu8` | Adds packed unsigned 8-bit integers in `a` and `b` using sat... | vpaddusb | — |
| `_mm256_alignr_epi8` | Concatenates pairs of 16-byte blocks in `a` and `b` into a 3... | vpalignr | — |
| `_mm256_and_si256` | Computes the bitwise AND of 256 bits (representing integer d... | vandps | 1/1, 1/1 |
| `_mm256_andnot_si256` | Computes the bitwise NOT of 256 bits (representing integer d... | vandnps | 1/1, 1/1 |
| `_mm256_avg_epu16` | Averages packed unsigned 16-bit integers in `a` and `b` | vpavgw | — |
| `_mm256_avg_epu8` | Averages packed unsigned 8-bit integers in `a` and `b` | vpavgb | — |
| `_mm256_blend_epi16` | Blends packed 16-bit integers from `a` and `b` using control... | vpblendw | 1/1, 1/1 |
| `_mm256_blend_epi32` | Blends packed 32-bit integers from `a` and `b` using control... | vblendps | 1/1, 1/1 |
| `_mm256_blendv_epi8` | Blends packed 8-bit integers from `a` and `b` using `mask` | vpblendvb | 1/1, 1/1 |
| `_mm256_broadcastb_epi8` | Broadcasts the low packed 8-bit integer from `a` to all elem... | vpbroadcastb | — |
| `_mm256_broadcastd_epi32` | Broadcasts the low packed 32-bit integer from `a` to all ele... | vbroadcastss | — |
| `_mm256_broadcastq_epi64` | Broadcasts the low packed 64-bit integer from `a` to all ele... | vbroadcastsd | — |
| `_mm256_broadcastsd_pd` | Broadcasts the low double-precision (64-bit) floating-point ... | vbroadcastsd | — |
| `_mm256_broadcastsi128_si256` | Broadcasts 128 bits of integer data from a to all 128-bit la... |  | — |
| `_mm256_broadcastss_ps` | Broadcasts the low single-precision (32-bit) floating-point ... | vbroadcastss | — |
| `_mm256_broadcastw_epi16` | Broadcasts the low packed 16-bit integer from a to all eleme... | vpbroadcastw | — |
| `_mm256_bslli_epi128` | Shifts 128-bit lanes in `a` left by `imm8` bytes while shift... | vpslldq | — |
| `_mm256_bsrli_epi128` | Shifts 128-bit lanes in `a` right by `imm8` bytes while shif... | vpsrldq | — |
| `_mm256_cmpeq_epi16` | Compares packed 16-bit integers in `a` and `b` for equality | vpcmpeqw | 1/1, 1/1 |
| `_mm256_cmpeq_epi32` | Compares packed 32-bit integers in `a` and `b` for equality | vpcmpeqd | 1/1, 1/1 |
| `_mm256_cmpeq_epi64` | Compares packed 64-bit integers in `a` and `b` for equality | vpcmpeqq | 1/1, 1/1 |
| `_mm256_cmpeq_epi8` | Compares packed 8-bit integers in `a` and `b` for equality | vpcmpeqb | 1/1, 1/1 |
| `_mm256_cmpgt_epi16` | Compares packed 16-bit integers in `a` and `b` for greater-t... | vpcmpgtw | 1/1, 1/1 |
| `_mm256_cmpgt_epi32` | Compares packed 32-bit integers in `a` and `b` for greater-t... | vpcmpgtd | 1/1, 1/1 |
| `_mm256_cmpgt_epi64` | Compares packed 64-bit integers in `a` and `b` for greater-t... | vpcmpgtq | 1/1, 1/1 |
| `_mm256_cmpgt_epi8` | Compares packed 8-bit integers in `a` and `b` for greater-th... | vpcmpgtb | 1/1, 1/1 |
| `_mm256_cvtepi16_epi32` | Sign-extend 16-bit integers to 32-bit integers | vpmovsxwd | 4/1, 3/1 |
| `_mm256_cvtepi16_epi64` | Sign-extend 16-bit integers to 64-bit integers | vpmovsxwq | 4/1, 3/1 |
| `_mm256_cvtepi32_epi64` | Sign-extend 32-bit integers to 64-bit integers | vpmovsxdq | 4/1, 3/1 |
| `_mm256_cvtepi8_epi16` | Sign-extend 8-bit integers to 16-bit integers | vpmovsxbw | 4/1, 3/1 |
| `_mm256_cvtepi8_epi32` | Sign-extend 8-bit integers to 32-bit integers | vpmovsxbd | 4/1, 3/1 |
| `_mm256_cvtepi8_epi64` | Sign-extend 8-bit integers to 64-bit integers | vpmovsxbq | 4/1, 3/1 |
| `_mm256_cvtepu16_epi32` | Zeroes extend packed unsigned 16-bit integers in `a` to pack... | vpmovzxwd | 4/1, 3/1 |
| `_mm256_cvtepu16_epi64` | Zero-extend the lower four unsigned 16-bit integers in `a` t... | vpmovzxwq | 4/1, 3/1 |
| `_mm256_cvtepu32_epi64` | Zero-extend unsigned 32-bit integers in `a` to 64-bit intege... | vpmovzxdq | 4/1, 3/1 |
| `_mm256_cvtepu8_epi16` | Zero-extend unsigned 8-bit integers in `a` to 16-bit integer... | vpmovzxbw | 4/1, 3/1 |
| `_mm256_cvtepu8_epi32` | Zero-extend the lower eight unsigned 8-bit integers in `a` t... | vpmovzxbd | 4/1, 3/1 |
| `_mm256_cvtepu8_epi64` | Zero-extend the lower four unsigned 8-bit integers in `a` to... | vpmovzxbq | 4/1, 3/1 |
| `_mm256_cvtph_ps` | Converts the 8 x 16-bit half-precision float values in the 1... | "vcvtph2ps" | 4/1, 3/1 |
| `_mm256_cvtps_ph` | Converts the 8 x 32-bit float values in the 256-bit vector `... | "vcvtps2ph" | 4/1, 3/1 |
| `_mm256_extract_epi16` | Extracts a 16-bit integer from `a`, selected with `INDEX`. R... |  | — |
| `_mm256_extract_epi8` | Extracts an 8-bit integer from `a`, selected with `INDEX`. R... |  | — |
| `_mm256_extracti128_si256` | Extracts 128 bits (of integer data) from `a` selected with `... | vextractf128 | — |
| `_mm256_fmadd_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfmadd | 5/1, 4/1 |
| `_mm256_fmadd_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfmadd | 5/1, 4/1 |
| `_mm256_fmaddsub_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfmaddsub | 5/1, 4/1 |
| `_mm256_fmaddsub_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfmaddsub | 5/1, 4/1 |
| `_mm256_fmsub_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfmsub | 5/1, 4/1 |
| `_mm256_fmsub_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfmsub213ps | 5/1, 4/1 |
| `_mm256_fmsubadd_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfmsubadd | 5/1, 4/1 |
| `_mm256_fmsubadd_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfmsubadd | 5/1, 4/1 |
| `_mm256_fnmadd_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfnmadd | 5/1, 4/1 |
| `_mm256_fnmadd_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfnmadd | 5/1, 4/1 |
| `_mm256_fnmsub_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfnmsub | 5/1, 4/1 |
| `_mm256_fnmsub_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfnmsub | 5/1, 4/1 |
| `_mm256_hadd_epi16` | Horizontally adds adjacent pairs of 16-bit integers in `a` a... | vphaddw | 5/2, 4/2 |
| `_mm256_hadd_epi32` | Horizontally adds adjacent pairs of 32-bit integers in `a` a... | vphaddd | 5/2, 4/2 |
| `_mm256_hadds_epi16` | Horizontally adds adjacent pairs of 16-bit integers in `a` a... | vphaddsw | — |
| `_mm256_hsub_epi16` | Horizontally subtract adjacent pairs of 16-bit integers in `... | vphsubw | 5/2, 4/2 |
| `_mm256_hsub_epi32` | Horizontally subtract adjacent pairs of 32-bit integers in `... | vphsubd | 5/2, 4/2 |
| `_mm256_hsubs_epi16` | Horizontally subtract adjacent pairs of 16-bit integers in `... | vphsubsw | — |
| `_mm256_inserti128_si256` | Copies `a` to `dst`, then insert 128 bits (of integer data) ... | vinsertf128 | — |
| `_mm256_madd_epi16` | Multiplies packed signed 16-bit integers in `a` and `b`, pro... | vpmaddwd | — |
| `_mm256_maddubs_epi16` | Vertically multiplies each unsigned 8-bit integer from `a` w... | vpmaddubsw | — |
| `_mm256_max_epi16` | Compares packed 16-bit integers in `a` and `b`, and returns ... | vpmaxsw | — |
| `_mm256_max_epi32` | Compares packed 32-bit integers in `a` and `b`, and returns ... | vpmaxsd | — |
| `_mm256_max_epi8` | Compares packed 8-bit integers in `a` and `b`, and returns t... | vpmaxsb | — |
| `_mm256_max_epu16` | Compares packed unsigned 16-bit integers in `a` and `b`, and... | vpmaxuw | — |
| `_mm256_max_epu32` | Compares packed unsigned 32-bit integers in `a` and `b`, and... | vpmaxud | — |
| `_mm256_max_epu8` | Compares packed unsigned 8-bit integers in `a` and `b`, and ... | vpmaxub | — |
| `_mm256_min_epi16` | Compares packed 16-bit integers in `a` and `b`, and returns ... | vpminsw | — |
| `_mm256_min_epi32` | Compares packed 32-bit integers in `a` and `b`, and returns ... | vpminsd | — |
| `_mm256_min_epi8` | Compares packed 8-bit integers in `a` and `b`, and returns t... | vpminsb | — |
| `_mm256_min_epu16` | Compares packed unsigned 16-bit integers in `a` and `b`, and... | vpminuw | — |
| `_mm256_min_epu32` | Compares packed unsigned 32-bit integers in `a` and `b`, and... | vpminud | — |
| `_mm256_min_epu8` | Compares packed unsigned 8-bit integers in `a` and `b`, and ... | vpminub | — |
| `_mm256_movemask_epi8` | Creates mask from the most significant bit of each 8-bit ele... | vpmovmskb | — |
| `_mm256_mpsadbw_epu8` | Computes the sum of absolute differences (SADs) of quadruple... | vmpsadbw | — |
| `_mm256_mul_epi32` | Multiplies the low 32-bit integers from each packed 64-bit e... | vpmuldq | 10/2, 4/1 |
| `_mm256_mul_epu32` | Multiplies the low unsigned 32-bit integers from each packed... | vpmuludq | 10/2, 4/1 |
| `_mm256_mulhi_epi16` | Multiplies the packed 16-bit integers in `a` and `b`, produc... | vpmulhw | 5/1, 3/1 |
| `_mm256_mulhi_epu16` | Multiplies the packed unsigned 16-bit integers in `a` and `b... | vpmulhuw | 5/1, 3/1 |
| `_mm256_mulhrs_epi16` | Multiplies packed 16-bit integers in `a` and `b`, producing ... | vpmulhrsw | — |
| `_mm256_mullo_epi16` | Multiplies the packed 16-bit integers in `a` and `b`, produc... | vpmullw | 5/1, 3/1 |
| `_mm256_mullo_epi32` | Multiplies the packed 32-bit integers in `a` and `b`, produc... | vpmulld | 10/2, 4/1 |
| `_mm256_or_si256` | Computes the bitwise OR of 256 bits (representing integer da... | vorps | 1/1, 1/1 |
| `_mm256_packs_epi16` | Converts packed 16-bit integers from `a` and `b` to packed 8... | vpacksswb | 1/1, 1/1 |
| `_mm256_packs_epi32` | Converts packed 32-bit integers from `a` and `b` to packed 1... | vpackssdw | 1/1, 1/1 |
| `_mm256_packus_epi16` | Converts packed 16-bit integers from `a` and `b` to packed 8... | vpackuswb | 1/1, 1/1 |
| `_mm256_packus_epi32` | Converts packed 32-bit integers from `a` and `b` to packed 1... | vpackusdw | 1/1, 1/1 |
| `_mm256_permute2x128_si256` | Shuffles 128-bits of integer data selected by `imm8` from `a... | vperm2f128 | 3/1, 2/1 |
| `_mm256_permute4x64_epi64` | Permutes 64-bit integers from `a` using control mask `imm8` | vpermpd | 3/1, 2/1 |
| `_mm256_permute4x64_pd` | Shuffles 64-bit floating-point elements in `a` across lanes ... | vpermpd | 3/1, 2/1 |
| `_mm256_permutevar8x32_epi32` | Permutes packed 32-bit integers from `a` according to the co... | vpermps | 3/1, 2/1 |
| `_mm256_permutevar8x32_ps` | Shuffles eight 32-bit floating-point elements in `a` across ... | vpermps | 3/1, 2/1 |
| `_mm256_sad_epu8` | Computes the absolute differences of packed unsigned 8-bit i... | vpsadbw | — |
| `_mm256_shuffle_epi32` | Shuffles 32-bit integers in 128-bit lanes of `a` using the c... | vshufps | 1/1, 1/1 |
| `_mm256_shuffle_epi8` | Shuffles bytes from `a` according to the content of `b`.  Fo... | vpshufb | 1/1, 1/1 |
| `_mm256_shufflehi_epi16` | Shuffles 16-bit integers in the high 64 bits of 128-bit lane... | vpshufhw | 1/1, 1/1 |
| `_mm256_shufflelo_epi16` | Shuffles 16-bit integers in the low 64 bits of 128-bit lanes... | vpshuflw | 1/1, 1/1 |
| `_mm256_sign_epi16` | Negates packed 16-bit integers in `a` when the corresponding... | vpsignw | — |
| `_mm256_sign_epi32` | Negates packed 32-bit integers in `a` when the corresponding... | vpsignd | — |
| `_mm256_sign_epi8` | Negates packed 8-bit integers in `a` when the corresponding ... | vpsignb | — |
| `_mm256_sll_epi16` | Shifts packed 16-bit integers in `a` left by `count` while s... | vpsllw | 1/1, 1/1 |
| `_mm256_sll_epi32` | Shifts packed 32-bit integers in `a` left by `count` while s... | vpslld | 1/1, 1/1 |
| `_mm256_sll_epi64` | Shifts packed 64-bit integers in `a` left by `count` while s... | vpsllq | 1/1, 1/1 |
| `_mm256_slli_epi16` | Shifts packed 16-bit integers in `a` left by `IMM8` while sh... | vpsllw | 1/1, 1/1 |
| `_mm256_slli_epi32` | Shifts packed 32-bit integers in `a` left by `IMM8` while sh... | vpslld | 1/1, 1/1 |
| `_mm256_slli_epi64` | Shifts packed 64-bit integers in `a` left by `IMM8` while sh... | vpsllq | 1/1, 1/1 |
| `_mm256_slli_si256` | Shifts 128-bit lanes in `a` left by `imm8` bytes while shift... | vpslldq | 1/1, 1/1 |
| `_mm256_sllv_epi32` | Shifts packed 32-bit integers in `a` left by the amount spec... | vpsllvd | 1/1, 1/1 |
| `_mm256_sllv_epi64` | Shifts packed 64-bit integers in `a` left by the amount spec... | vpsllvq | 1/1, 1/1 |
| `_mm256_sra_epi16` | Shifts packed 16-bit integers in `a` right by `count` while ... | vpsraw | 1/1, 1/1 |
| `_mm256_sra_epi32` | Shifts packed 32-bit integers in `a` right by `count` while ... | vpsrad | 1/1, 1/1 |
| `_mm256_srai_epi16` | Shifts packed 16-bit integers in `a` right by `IMM8` while s... | vpsraw | 1/1, 1/1 |
| `_mm256_srai_epi32` | Shifts packed 32-bit integers in `a` right by `IMM8` while s... | vpsrad | 1/1, 1/1 |
| `_mm256_srav_epi32` | Shifts packed 32-bit integers in `a` right by the amount spe... | vpsravd | 1/1, 1/1 |
| `_mm256_srl_epi16` | Shifts packed 16-bit integers in `a` right by `count` while ... | vpsrlw | 1/1, 1/1 |
| `_mm256_srl_epi32` | Shifts packed 32-bit integers in `a` right by `count` while ... | vpsrld | 1/1, 1/1 |
| `_mm256_srl_epi64` | Shifts packed 64-bit integers in `a` right by `count` while ... | vpsrlq | 1/1, 1/1 |
| `_mm256_srli_epi16` | Shifts packed 16-bit integers in `a` right by `IMM8` while s... | vpsrlw | 1/1, 1/1 |
| `_mm256_srli_epi32` | Shifts packed 32-bit integers in `a` right by `IMM8` while s... | vpsrld | 1/1, 1/1 |
| `_mm256_srli_epi64` | Shifts packed 64-bit integers in `a` right by `IMM8` while s... | vpsrlq | 1/1, 1/1 |
| `_mm256_srli_si256` | Shifts 128-bit lanes in `a` right by `imm8` bytes while shif... | vpsrldq | 1/1, 1/1 |
| `_mm256_srlv_epi32` | Shifts packed 32-bit integers in `a` right by the amount spe... | vpsrlvd | 1/1, 1/1 |
| `_mm256_srlv_epi64` | Shifts packed 64-bit integers in `a` right by the amount spe... | vpsrlvq | 1/1, 1/1 |
| `_mm256_sub_epi16` | Subtract packed 16-bit integers in `b` from packed 16-bit in... | vpsubw | 1/1, 1/1 |
| `_mm256_sub_epi32` | Subtract packed 32-bit integers in `b` from packed 32-bit in... | vpsubd | 1/1, 1/1 |
| `_mm256_sub_epi64` | Subtract packed 64-bit integers in `b` from packed 64-bit in... | vpsubq | 1/1, 1/1 |
| `_mm256_sub_epi8` | Subtract packed 8-bit integers in `b` from packed 8-bit inte... | vpsubb | 1/1, 1/1 |
| `_mm256_subs_epi16` | Subtract packed 16-bit integers in `b` from packed 16-bit in... | vpsubsw | 1/1, 1/1 |
| `_mm256_subs_epi8` | Subtract packed 8-bit integers in `b` from packed 8-bit inte... | vpsubsb | 1/1, 1/1 |
| `_mm256_subs_epu16` | Subtract packed unsigned 16-bit integers in `b` from packed ... | vpsubusw | — |
| `_mm256_subs_epu8` | Subtract packed unsigned 8-bit integers in `b` from packed 8... | vpsubusb | — |
| `_mm256_unpackhi_epi16` | Unpacks and interleave 16-bit integers from the high half of... | vpunpckhwd | 1/1, 1/1 |
| `_mm256_unpackhi_epi32` | Unpacks and interleave 32-bit integers from the high half of... | vunpckhps | 1/1, 1/1 |
| `_mm256_unpackhi_epi64` | Unpacks and interleave 64-bit integers from the high half of... | vunpckhpd | 1/1, 1/1 |
| `_mm256_unpackhi_epi8` | Unpacks and interleave 8-bit integers from the high half of ... | vpunpckhbw | 1/1, 1/1 |
| `_mm256_unpacklo_epi16` | Unpacks and interleave 16-bit integers from the low half of ... | vpunpcklwd | 1/1, 1/1 |
| `_mm256_unpacklo_epi32` | Unpacks and interleave 32-bit integers from the low half of ... | vunpcklps | 1/1, 1/1 |
| `_mm256_unpacklo_epi64` | Unpacks and interleave 64-bit integers from the low half of ... | vunpcklpd | 1/1, 1/1 |
| `_mm256_unpacklo_epi8` | Unpacks and interleave 8-bit integers from the low half of e... | vpunpcklbw | 1/1, 1/1 |
| `_mm256_xor_si256` | Computes the bitwise XOR of 256 bits (representing integer d... | vxorps | 1/1, 1/1 |
| `_mm_blend_epi32` | Blends packed 32-bit integers from `a` and `b` using control... | vblendps | 1/1, 1/1 |
| `_mm_broadcastb_epi8` | Broadcasts the low packed 8-bit integer from `a` to all elem... | vpbroadcastb | — |
| `_mm_broadcastd_epi32` | Broadcasts the low packed 32-bit integer from `a` to all ele... | vbroadcastss | — |
| `_mm_broadcastq_epi64` | Broadcasts the low packed 64-bit integer from `a` to all ele... | vmovddup | — |
| `_mm_broadcastsd_pd` | Broadcasts the low double-precision (64-bit) floating-point ... | vmovddup | — |
| `_mm_broadcastsi128_si256` | Broadcasts 128 bits of integer data from a to all 128-bit la... |  | — |
| `_mm_broadcastss_ps` | Broadcasts the low single-precision (32-bit) floating-point ... | vbroadcastss | — |
| `_mm_broadcastw_epi16` | Broadcasts the low packed 16-bit integer from a to all eleme... | vpbroadcastw | — |
| `_mm_cvtph_ps` | Converts the 4 x 16-bit half-precision float values in the l... | "vcvtph2ps" | 4/1, 3/1 |
| `_mm_cvtps_ph` | Converts the 4 x 32-bit float values in the 128-bit vector `... | "vcvtps2ph" | 4/1, 3/1 |
| `_mm_fmadd_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfmadd | 5/1, 4/1 |
| `_mm_fmadd_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfmadd | 5/1, 4/1 |
| `_mm_fmadd_sd` | Multiplies the lower double-precision (64-bit) floating-poin... | vfmadd | 5/1, 4/1 |
| `_mm_fmadd_ss` | Multiplies the lower single-precision (32-bit) floating-poin... | vfmadd | 5/1, 4/1 |
| `_mm_fmaddsub_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfmaddsub | 5/1, 4/1 |
| `_mm_fmaddsub_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfmaddsub | 5/1, 4/1 |
| `_mm_fmsub_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfmsub | 5/1, 4/1 |
| `_mm_fmsub_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfmsub213ps | 5/1, 4/1 |
| `_mm_fmsub_sd` | Multiplies the lower double-precision (64-bit) floating-poin... | vfmsub | 5/1, 4/1 |
| `_mm_fmsub_ss` | Multiplies the lower single-precision (32-bit) floating-poin... | vfmsub | 5/1, 4/1 |
| `_mm_fmsubadd_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfmsubadd | 5/1, 4/1 |
| `_mm_fmsubadd_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfmsubadd | 5/1, 4/1 |
| `_mm_fnmadd_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfnmadd | 5/1, 4/1 |
| `_mm_fnmadd_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfnmadd | 5/1, 4/1 |
| `_mm_fnmadd_sd` | Multiplies the lower double-precision (64-bit) floating-poin... | vfnmadd | 5/1, 4/1 |
| `_mm_fnmadd_ss` | Multiplies the lower single-precision (32-bit) floating-poin... | vfnmadd | 5/1, 4/1 |
| `_mm_fnmsub_pd` | Multiplies packed double-precision (64-bit) floating-point e... | vfnmsub | 5/1, 4/1 |
| `_mm_fnmsub_ps` | Multiplies packed single-precision (32-bit) floating-point e... | vfnmsub | 5/1, 4/1 |
| `_mm_fnmsub_sd` | Multiplies the lower double-precision (64-bit) floating-poin... | vfnmsub | 5/1, 4/1 |
| `_mm_fnmsub_ss` | Multiplies the lower single-precision (32-bit) floating-poin... | vfnmsub | 5/1, 4/1 |
| `_mm_sllv_epi32` | Shifts packed 32-bit integers in `a` left by the amount spec... | vpsllvd | 1/1, 1/1 |
| `_mm_sllv_epi64` | Shifts packed 64-bit integers in `a` left by the amount spec... | vpsllvq | 1/1, 1/1 |
| `_mm_srav_epi32` | Shifts packed 32-bit integers in `a` right by the amount spe... | vpsravd | 1/1, 1/1 |
| `_mm_srlv_epi32` | Shifts packed 32-bit integers in `a` right by the amount spe... | vpsrlvd | 1/1, 1/1 |
| `_mm_srlv_epi64` | Shifts packed 64-bit integers in `a` right by the amount spe... | vpsrlvq | 1/1, 1/1 |
| `_mm_tzcnt_32` | Counts the number of trailing least significant zero bits.  ... | tzcnt | — |
| `_mm_tzcnt_64` | Counts the number of trailing least significant zero bits.  ... | tzcnt | — |
| `_mulx_u32` |  |  | — |
| `_mulx_u64` | Unsigned multiply without affecting flags.  Unsigned multipl... |  | — |
| `_pdep_u32` | Scatter contiguous low order bits of `a` to the result at th... | pdep | — |
| `_pdep_u64` | Scatter contiguous low order bits of `a` to the result at th... | pdep | — |
| `_pext_u32` | Gathers the bits of `x` specified by the `mask` into the con... | pext | — |
| `_pext_u64` | Gathers the bits of `x` specified by the `mask` into the con... | pext | — |
| `_tzcnt_u16` | Counts the number of trailing least significant zero bits.  ... | tzcnt | — |
| `_tzcnt_u32` | Counts the number of trailing least significant zero bits.  ... | tzcnt | — |
| `_tzcnt_u64` | Counts the number of trailing least significant zero bits.  ... | tzcnt | — |

### Stable, Unsafe (41 intrinsics) — use import_intrinsics for safe versions

| Name | Description | Safe Variant |
|------|-------------|--------------|
| `_mm256_i32gather_epi32` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_i32gather_epi64` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_i32gather_pd` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_i32gather_ps` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_i64gather_epi32` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_i64gather_epi64` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_i64gather_pd` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_i64gather_ps` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_mask_i32gather_epi32` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_mask_i32gather_epi64` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_mask_i32gather_pd` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_mask_i32gather_ps` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_mask_i64gather_epi32` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_mask_i64gather_epi64` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_mask_i64gather_pd` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_mask_i64gather_ps` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm256_maskload_epi32` | Loads packed 32-bit integers from memory pointed by `mem_add... | — |
| `_mm256_maskload_epi64` | Loads packed 64-bit integers from memory pointed by `mem_add... | — |
| `_mm256_maskstore_epi32` | Stores packed 32-bit integers from `a` into memory pointed b... | — |
| `_mm256_maskstore_epi64` | Stores packed 64-bit integers from `a` into memory pointed b... | — |
| `_mm256_stream_load_si256` | Load 256-bits of integer data from memory into dst using a n... | — |
| `_mm_i32gather_epi32` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_i32gather_epi64` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_i32gather_pd` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_i32gather_ps` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_i64gather_epi32` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_i64gather_epi64` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_i64gather_pd` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_i64gather_ps` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_mask_i32gather_epi32` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_mask_i32gather_epi64` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_mask_i32gather_pd` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_mask_i32gather_ps` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_mask_i64gather_epi32` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_mask_i64gather_epi64` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_mask_i64gather_pd` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_mask_i64gather_ps` | Returns values from `slice` at offsets determined by `offset... | — |
| `_mm_maskload_epi32` | Loads packed 32-bit integers from memory pointed by `mem_add... | — |
| `_mm_maskload_epi64` | Loads packed 64-bit integers from memory pointed by `mem_add... | — |
| `_mm_maskstore_epi32` | Stores packed 32-bit integers from `a` into memory pointed b... | — |
| `_mm_maskstore_epi64` | Stores packed 64-bit integers from `a` into memory pointed b... | — |


