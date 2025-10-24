# kpv Development Overview

## Project Summary
`kpv` is a command-line utility designed to simplify the management of `.env` files across multiple projects. It allows developers to save, link, and list environment configurations with ease, streamlining the process of switching between different development environments.

## Tech Stack
- **Language**: Rust
- **CLI Parsing**: `clap`
- **Development Dependencies**:
  - `assert_cmd`
  - `assert_fs`
  - `predicates`
  - `serial_test`
  - `tempfile`

## Coding Standards
- **Formatter**: `rustfmt` is used for code formatting. Key rules include a maximum line width of 100 characters, crate-level import granularity, and grouping imports by standard, external, and crate modules.
- **Linter**: `clippy` is used for linting, with a strict policy of treating all warnings as errors (`-D warnings`).

## Naming Conventions
- **Structs and Enums**: `PascalCase` (e.g., `Cli`, `Commands`)
- **Functions and Variables**: `snake_case` (e.g., `run_tests`, `test_context`)
- **Modules**: `snake_case` (e.g., `cli_commands.rs`)

## Key Commands
- **Build (Debug)**: `cargo build`
- **Build (Release)**: `cargo build --release`
- **Format Check**: `cargo fmt --check`
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Test**: `RUST_TEST_THREADS=1 cargo test --all-targets --all-features`

## Testing Strategy
- **Unit Tests**: Located within the `src/` directory alongside the code they test.
- **Core Logic Tests**: Found in `src/core/`, utilizing mock storage to ensure business logic is tested in isolation.
- **Integration Tests**: Housed in the `tests/` directory, these tests cover the public API and CLI user flows from an external perspective.
- **CI**: GitHub Actions automatically runs build, linting, and test workflows, as defined in `.github/workflows/`.
- **Sequential Testing**: The `serial_test` crate is employed for tests that interact with the filesystem to prevent race conditions.
