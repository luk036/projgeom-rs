# AGENTS.md - projgeom-rs

## Project Overview

Single-crate Rust library for projective, elliptic, hyperbolic, and Euclidean geometry computations.

## Build & Test Commands

```bash
# Run all tests (quickcheck + doc tests)
cargo test --all-features --workspace

# Run benchmarks
cargo bench

# Clippy lints (strict)
cargo clippy --all-targets --all-features --workspace

# Format check
cargo fmt --all -- --check

# Build docs (fails on warnings)
cargo doc --no-deps --document-private-items --all-features --workspace --examples
```

## CI Pipeline (enforced on PRs)

1. `cargo test --all-features --workspace`
2. `cargo fmt --all --check`
3. `cargo clippy --all-targets --all-features --workspace`
4. `cargo doc` (private items, no deps)

## Pre-commit Hooks

```bash
pip install pre-commit
pre-commit install
```

Runs: trailing-whitespace, end-of-file-fixer, rustfmt, cargo-check, clippy, cargo-test, cargo-doc.

## Project Structure

- `src/lib.rs` - Main entry, re-exports types
- `src/pg_object.rs`, `src/pg_plane.rs` - Core projective geometry
- `src/ell_object.rs`, `src/hyp_object.rs`, `src/euclid_object.rs` - Geometry variants
- `src/ck_plane.rs` - Cayley-Klein plane traits
- `src/cross_ratio.rs`, `src/conic.rs`, `src/transform.rs` - Specialized operations
- `examples/` - Runnable example scripts

## Clippy Config

Custom `clippy.toml`:
- `doc-valid-idents` includes: `PgPoint`, `PgLine`, `Euclid`, `Elliptic`, `Hyperbolic`, `CayleyKlein`
- Allows `unwrap-in-tests`, `expect-in-tests`, `dbg-in-tests`
- Raised threshold for `too-many-arguments` (7) and `type-complexity` (250)

## Notable Dependencies

- `fractions-rs` - Rational number arithmetic (not `fraction` crate)
- `svgbobdoc` - ASCII diagram rendering in docs
- `quickcheck`, `quickcheck_macros` - Property-based testing
- `criterion` - Benchmarking

## Things Agents Commonly Get Wrong

1. **Not using `--all-features`**: Several features/flags require `--all-features` to compile
2. **Doc test failures**: The library uses `fractions-rs`, doc examples may need rational types
3. **Custom clippy lints**: `doc-valid-idents` enforcement - don't use non-Pg/Elliptic/etc names in docs
4. **CayleyKlein trait**: `ell_object.rs`, `hyp_object.rs`, `euclid_object.rs` all implement `CayleyKlein` trait - prefer trait methods over type-specific code