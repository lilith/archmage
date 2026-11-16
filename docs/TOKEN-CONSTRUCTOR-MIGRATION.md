# Token constructor migration

magetypes 0.9.30 adds `_t` methods alongside the existing token-first
methods. Both spellings take the token first and have the same behavior:

```rust
use archmage::ScalarToken;
use magetypes::simd::generic::f32x8;

let old = f32x8::splat(ScalarToken, 2.0);
let prepared = f32x8::splat_t(ScalarToken, 2.0);
assert_eq!(old.to_array(), prepared.to_array());
```

Existing token-taking names remain callable but are deprecated, with diagnostics
pointing to the corresponding `_t` method. Projects using `deny(deprecated)` or
`deny(warnings)` must migrate those calls or explicitly allow the warnings.
Vector types stay
generic over one token parameter. No constructor modes or new function-attribute
syntax are part of this change.

## Migrate names before changing contexts

Rename token-taking calls while preserving their arguments and vector widths:

| Existing spelling | Migration spelling |
|---|---|
| `splat(token, value)` | `splat_t(token, value)` |
| `zero(token)` | `zero_t(token)` |
| `load(token, data)` | `load_t(token, data)` |
| `from_array(token, values)` | `from_array_t(token, values)` |
| `partition_slice_mut(token, data)` | `partition_slice_mut_t(token, data)` |
| `from_halves(token, lo, hi)` | `from_halves_t(token, lo, hi)` |

The aliases cover all public inherent methods with an explicit first token,
including conversions, byte loads, block loads, borrowed slice views, native raw
constructors, and the single-lane scalar types. Trait methods whose receiver is
the token do not gain aliases. Existing target and Cargo feature gates still
apply.

Native raw values have two spellings on every native-backend type:
`from_raw_t(token, raw)` for callers that hold a token, generic code included,
and `from_raw(raw)`, which takes no token and wraps one inside a matching
target-feature context that Rust checks. The deprecated x86 platform-named
constructors (`from_m128`, `from_m128d`, `from_m128i`, `from_m256`,
`from_m256d`, `from_m256i`) forward to `from_raw_t` and remain deprecated
ahead of removal in 0.11; they have no `_t` form, and no other type gets a
platform name.

Ordinary functions and backend-generic helpers can call `_t` methods without
target-feature annotations:

```rust
use magetypes::simd::backends::F32x8Backend;
use magetypes::simd::generic::f32x8;

fn broadcast<T: F32x8Backend>(token: T, value: f32) -> f32x8<T> {
    f32x8::splat_t(token, value)
}
```

Code written for the concrete vector types of 0.9.26 and earlier may call
one-argument conversions such as `f32x4::from_i32x4(v)`. The generic types take
the token first, like the constructors: `f32x4::from_i32x4_t(token, v)`.

Keep function token parameters, dispatch calls, and public vector signatures
unchanged during this rename. Existing `#[magetypes(define(...), ...)]` aliases
refer to the same types and support both method spellings.

## The unified-macro beta does not change constructors

The 0.9.31 beta archmage macros (`#[attune]`, `attuned!`, `reattune!`) add
definition and dispatch syntax but change no constructor signature: `_t(token,
...)` remains the token-first spelling, and the short names have not become
tokenless. See [the unified macro guide](ATTUNE.md) for that beta's API and its
own migration mapping.

## Future tokenless-short-name boundary (0.11+)

Magetypes 0.10 retains `_t(token, ...)` constructors alongside the deprecated
token-taking short names, giving callers another full release cycle before any
future signature change reserves short constructor names for compiler-checked
feature contexts. The explicit-token `_t` methods remain supported, and
generic backend bounds still require an explicit token (`_t`).

The macro contract continues to use `magetypes::simd::generic::TYPE<Token>`
for `define(...)`, and tier/backends namespaces for `import_magetypes`.

## Maintenance

`xtask/src/simd_types/generic_gen/token_aliases.rs` derives deprecated legacy forwarders from the canonical `_t`
implementation signatures, preserving argument order, bounds, lifetimes,
attributes, and feature gates. It handles generated vectors and the handwritten
cross-width and scalar modules. Regenerate with `cargo run -p xtask -- generate`;
do not maintain separate constructor lists or hand-edit the generated aliases.

`just check-packages` builds the crate archives together without publishing and
asserts that the normalized manifests keep the exact archmage→archmage-macros pin
and the ordinary magetypes→archmage version requirement. Pass `--target` to
`cargo xtask check-packages --target <triple>`, repeatedly, to check other targets.

## Compatibility checks

The migration tests exercise both spellings, baseline-callable function
pointers, generic token bounds, floating point bit preservation, mutable slice
views and tails, and existing `define` macro syntax across default,
no-default-feature, AVX-512, AArch64, WASM, and i686 configurations.

## Downstream compilation after deprecation

The [2026-09-27 consumer audit](DOWNSTREAM-COMPATIBILITY.md) covers 20 published
zen-prefixed consumers, linear-srgb, garb, jxl-encoder-simd, and 14 local
packages. It records exact versions, baseline comparisons, coverage limits,
and cargo-copter workarounds for yanked-version selection and inherited features.
