# Archmage build and test commands (pure Rust / Cargo / xtask)

# Default: run all tests
default: test

# Run all workspace tests (excludes nightly-only features)
test:
    cargo test --workspace --features "std avx512"

# Exact float mask/negation bits, generated parity, and documented contracts.
test-float-contracts:
    cargo test -p magetypes --features "std avx512" --test float_inequality --test float_negation --test scalar_parity --test doc_examples

# Run the same float contracts under ARM emulation and strict WASM SIMD.
test-float-contracts-arm:
    cross test -p magetypes --target aarch64-unknown-linux-gnu --features std --test float_inequality --test float_negation --test scalar_parity --test doc_examples

test-float-contracts-wasm:
    RUSTFLAGS="-C target-feature=+simd128" CARGO_TARGET_WASM32_WASIP1_RUNNER=wasmtime cargo test -p magetypes --target wasm32-wasip1 --features std --test float_inequality --test float_negation --test scalar_parity --test doc_examples

# Emit the named neg8_codegen probe for instruction inspection.
float-contract-codegen:
    cargo rustc -p magetypes --release --features "std avx512" --test float_negation -- --emit=asm

# Run clippy across workspace and strict-lint consumer crates
lint:
    cargo clippy --workspace --all-targets --features "std avx512" -- -D warnings
    cargo clippy -p archmage-no-features-test -p archmage-clippy-consumer --all-targets -- -D warnings

# Clippy over every target and feature configuration
warnings:
    cargo clippy --workspace --all-targets --features "std avx512" -- -D warnings
    cargo clippy -p archmage --all-targets --no-default-features -- -D warnings
    for target in i686-unknown-linux-gnu aarch64-unknown-linux-gnu wasm32-wasip1 wasm32-unknown-unknown armv7-unknown-linux-gnueabihf; do \
        cargo clippy -p archmage --target "$target" -- -D warnings || exit 1; \
        cargo clippy -p magetypes --target "$target" -- -D warnings || exit 1; \
    done

# Format code + regenerate the public-API surface snapshots (docs/public-api/).
fmt:
    cargo fmt
    cargo test --manifest-path apidoc/Cargo.toml

# Regenerate the public-API surface snapshots only
api-doc:
    cargo test --manifest-path apidoc/Cargo.toml

# Verify the committed public-API snapshots are current
api-doc-check:
    ZEN_API_DOC=check cargo test --manifest-path apidoc/Cargo.toml

# Check formatting
fmt-check:
    cargo fmt -- --check

# Run Miri tests (token logic only)
miri-tokens:
    rustup run "$(tr -d '[:space:]' < xtask/miri-nightly.txt)" cargo miri test --test miri_safe --all-features

# Run Miri on magetypes with full SIMD support
miri:
    cargo run -p xtask -- miri

# Static soundness verification (validates intrinsics against stdarch database)
soundness:
    cargo run -p xtask -- soundness

# Safety audit (scan for critical code, check intrinsics freshness)
audit:
    cargo run -p xtask -- audit

# Refresh intrinsics database from current Rust toolchain (pure Rust)
intrinsics-refresh:
    cargo run -p xtask -- intrinsics-refresh

# Fuzz test for divergences between native and polyfill implementations
fuzz:
    cargo test -p magetypes --test fuzz_divergence --features avx512

# Regenerate all generated code (SIMD types, macro registry, docs)
generate:
    cargo run -p xtask -- generate

# Verify the working tree agrees with the generator (regeneration is a no-op)
check-generated:
    #!/usr/bin/env bash
    set -uo pipefail
    before="$(mktemp)"; after="$(mktemp)"
    trap 'rm -f "$before" "$after"' EXIT
    git ls-files -z | xargs -0 shasum -a 256 2>/dev/null | sort > "$before"
    cargo run -p xtask -- generate || exit 1
    git ls-files -z | xargs -0 shasum -a 256 2>/dev/null | sort > "$after"
    if diff -q "$before" "$after" >/dev/null; then
        echo "OK: generated code matches the generator (regeneration is a no-op)."
        exit 0
    fi
    echo "ERROR: regeneration changed files:"
    diff "$before" "$after" | awk '/^>/ {print "  " $3}' | sort -u
    exit 1

# Validate token-registry.toml (parse + structural checks)
validate-registry:
    cargo run -p xtask -- validate-registry

# Validate magetypes safety + summon() feature checks against registry
validate-tokens:
    cargo run -p xtask -- validate

# Check API parity across x86/ARM/WASM architectures
parity:
    cargo run -p xtask -- parity

# Test no_std compilation and tests for all crates
test-nostd:
    cargo check -p archmage --no-default-features --features "avx512"
    cargo check -p magetypes --no-default-features
    cargo test -p archmage --no-default-features --features "avx512"
    cargo test -p magetypes --no-default-features
    cargo check -p magetypes --no-default-features --target aarch64-unknown-none
    cargo check -p magetypes --no-default-features --target thumbv7m-none-eabi

# Run ALL CI checks
ci:
    cargo run -p xtask -- ci

all: ci

# Test testable_dispatch with CompileTimePolicy::Fail (must not panic)
test-dispatch:
    cargo test --features "std avx512 testable_dispatch" --test token_permutations -- --test-threads=1

# Parity tests (cross-architecture)
test-parity-x86:
    cargo test -p magetypes --test cross_arch_parity --features "std avx512"

test-parity-arm:
    cross test -p magetypes --test cross_arch_parity --target aarch64-unknown-linux-gnu

test-parity-wasm:
    RUSTFLAGS="-C target-feature=+simd128" cargo test -p magetypes --test cross_arch_parity --target wasm32-wasip1

test-parity: test-parity-x86 test-parity-arm test-parity-wasm

# Intel SDE testing (requires sde64)
test-p4:
    sde64 -p4 -- cargo test --all-features
test-nehalem:
    sde64 -nhm -- cargo test --all-features
test-haswell:
    sde64 -hsw -- cargo test --all-features
test-skylake:
    sde64 -skx -- cargo test --all-features
test-icelake:
    sde64 -icl -- cargo test --all-features
test-all-cpus: test-p4 test-nehalem test-haswell test-skylake test-icelake

# Cross-compilation testing
test-i686:
    cross test --all-features --target i686-unknown-linux-gnu
    cross test -p archmage-no-features-test --target i686-unknown-linux-gnu

test-aarch64:
    cross test --lib --target aarch64-unknown-linux-gnu
    cross test --test miri_safe --test feature_consistency --test arm_safe_intrinsics --test arm_feature_intrinsics --target aarch64-unknown-linux-gnu

test-armv7:
    cross test --lib --target armv7-unknown-linux-gnueabihf
    cross test --test miri_safe --test feature_consistency --target armv7-unknown-linux-gnueabihf

test-cross: test-i686 test-aarch64 test-armv7

# Clippy per target
clippy-x86_64:
    cargo clippy --all-features --target x86_64-unknown-linux-gnu -- -D warnings
clippy-aarch64:
    cargo clippy --all-features --target aarch64-unknown-linux-gnu -- -D warnings
clippy-i686:
    cargo clippy --all-features --target i686-unknown-linux-gnu -- -D warnings
clippy-all: clippy-x86_64 clippy-aarch64 clippy-i686

# Check rustdoc builds cleanly
doc-check:
    RUSTDOCFLAGS="-Dwarnings" cargo doc --features "std avx512" --no-deps

# Build the documentation site (Zola)
docs:
    cd docs/site && zola build -o ../../target/site

docs-serve:
    cd docs/site && zola serve --port 3100

# Refresh macro expansion snapshots
expand-snapshots:
    cargo run -p xtask -- gen-expand
    MACROTEST=overwrite cargo test -p archmage --test macro_expand -- --test-threads=1
    MACROTEST=overwrite cargo test -p magetypes --test macro_expand -- --test-threads=1
    ARCHMAGE_RAW_SNAPSHOTS=overwrite cargo test -p archmage-macros raw_snapshots

expand-check:
    cargo test -p archmage --test macro_expand -- --test-threads=1

# Build the actual crate archives together without publishing
check-packages *TARGETS:
    cargo run -p xtask -- check-packages {{TARGETS}}

# Fused arithmetic regression and WASM checks
test-fused:
    cargo test -p magetypes --all-features --test fused_arithmetic

test-fused-wasm runner="wasmtime" flags="+simd128":
    RUSTFLAGS="-Ctarget-feature={{flags}}" CARGO_TARGET_WASM32_WASIP1_RUNNER="{{runner}}" cargo test -p magetypes --target wasm32-wasip1 --test fused_arithmetic --test doc_examples --test scalar_parity

test-fused-wasm-no-std runner="wasmtime" flags="+simd128":
    RUSTFLAGS="-Ctarget-feature={{flags}}" CARGO_TARGET_WASM32_WASIP1_RUNNER="{{runner}}" cargo test -p magetypes --no-default-features --features w512 --target wasm32-wasip1 --test fused_arithmetic --test doc_examples --test scalar_parity

fma-contract:
    cargo test -p magetypes --features avx512 --test fma_contract --test fused_arithmetic
    cargo test -p magetypes --no-default-features --features w512 --test fma_contract --test fused_arithmetic

# Magemath verification
magemath-check:
    cargo test -p magemath --features "std avx512"
    cargo test -p magemath --no-default-features --features w512
    cargo test -p magemath --release --features "std avx512" --test contracts
    cargo clippy -p magemath --all-targets --features "std avx512" -- -D warnings
    cargo test -p magemath --test expf_logf_contracts
    cargo fmt -p magemath -p magetypes -p xtask -- --check

magemath-check-wasm:
    RUSTFLAGS="-C target-feature=+simd128" CARGO_TARGET_WASM32_WASIP1_RUNNER=wasmtime cargo test -p magemath --features "std w512" --target wasm32-wasip1
    RUSTFLAGS="-C target-feature=+simd128" CARGO_TARGET_WASM32_WASIP1_RUNNER=wasmtime cargo test -p magemath --no-default-features --features w512 --target wasm32-wasip1

# Benchmarks
bench:
    RUSTFLAGS="-C target-cpu=native" cargo bench
