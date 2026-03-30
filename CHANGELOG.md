# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-03-30

### Added

- Unit tests for `generate_password` (8 tests + 2 doctests)
- CI workflow: formatting, linting, and tests via GitHub Actions
- CD workflow: automatic publish to crates.io on release
- Code coverage with `cargo-tarpaulin` and Codecov
- README badges: CI status, coverage, docs.rs

### Changed

- Extracted core logic from `main.rs` into `lib.rs`

## [0.1.0] - 2026-03-27

### Added

- Password generation with configurable length
- Multiple password generation (`-n` flag)
- Exclude symbols (`--no-symbols`)
- Exclude numbers (`--no-numbers`)
- Exclude uppercase letters (`--no-uppercase`)
- Colored terminal output by character type
- Input validation with clear error messages
