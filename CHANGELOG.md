# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive unit test coverage using cargo llvm-cov
- Integration tests for complex geometric theorems (Desargues, Pappus, Pascal, Menelaus)
- Property-based tests using quickcheck for geometric invariants

### Changed
- Re-enabled Clippy checks in CI workflow
- Improved code organization in euclid_object.rs (moved public functions before test module)

### Fixed
- Rust 2024 compatibility warning in FFI layer (raw const reference for static mutable)
- CI/CD build issues
- Code coverage configuration for CodeCov

### Internal
- Enhanced test coverage across all modules
- Improved code formatting and style consistency
