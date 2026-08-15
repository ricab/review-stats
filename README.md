# Review Stats

Stats concerning PR reviews on GitHub.

## Quickstart

This is a Rust CLI tool.
You can use Cargo to run: `cargo run -- <args>`.
Run with `--help` for usage information and examples.

## Build and test

You can also build and test with Cargo:

- Build: `cargo build`
- Test: `cargo test`

### Unit tests

Unit tests are located in the [tests](./tests) subfolder, to prevent access to private fields.
This is intentional.
The goal is to enforce testing only public interfaces, to prevent test brittleness.
