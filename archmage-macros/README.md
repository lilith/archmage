# archmage-macros

**[Official guide and examples](https://lilith.github.io/archmage/)** · [Intrinsics browser](https://lilith.github.io/archmage/intrinsics/) · [Archmage API](https://docs.rs/archmage/latest/archmage/) · [Magetypes API](https://docs.rs/magetypes/latest/magetypes/)

Procedural macros for archmage's CPU-capability tokens: `#[arcane]`, `#[rite]`,
`#[magetypes]`, `#[autoversion]`, `incant!`, `#[attune]`, `attuned!`, and `reattune!`.

Application code should depend on [archmage](https://crates.io/crates/archmage),
which re-exports these macros and pins the compatible macro version. Add the
[magetypes vector crate](https://docs.rs/magetypes/latest/magetypes/) when using
vector types such as [`f32x4<T>`](https://docs.rs/magetypes/latest/magetypes/simd/generic/struct.f32x4.html).
The `#[magetypes]` attribute generates variants; the magetypes crate supplies
their vector operations.

Start with the [complete SIMD call chain](https://lilith.github.io/archmage/archmage/getting-started/first-simd/),
then [type and const generics](https://lilith.github.io/archmage/magetypes/dispatch/types-and-dispatch/).
For internal feature-enabled helpers, see [rite and from_context()](https://lilith.github.io/archmage/archmage/concepts/rite/).

Macro expansion tests, compile-fail tests, and optimized-code comparisons cover
different obligations. See the [testing guide](https://lilith.github.io/archmage/archmage/testing/dispatch-testing/).

## License

MIT OR Apache-2.0.

## Unified macros & token conjunctions

Version `0.10.0` supports `And<Base, Extra>` capability conjunctions,
unconditional tag and bound authentication across all targets, and `#[attune]`,
`attuned!`, and `reattune!`. Legacy attributes remain supported. Use the matching
archmage version; its exact dependency pin keeps generated code and runtime APIs
paired. See the [unified macro guide](https://github.com/lilith/archmage/blob/main/docs/ATTUNE.md).
