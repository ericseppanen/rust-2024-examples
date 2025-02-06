# Upgrading to Rust 2024

This repository contains examples of Rust code that will be affected by an upgrade to the 2024 edition.
`Cargo.toml` contains lints that can be enabled to reveal (and fix) issues one at a time.

The examples and lints work on rust 2021, but a beta or nightly compiler will be needed if you actually want to try the 2024 edition before the 1.85 toolchain is released on February 20, 2025.

### How to see the edition changes

- Enable one or more of the lints in `Cargo.toml`.
- Run `cargo check` to see the warnings.
- To see the automatically-applied fixes, run `cargo fix` (Alternatively, run `cargo fix --edition` to get all the fixes at once.)
