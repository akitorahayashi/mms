# Manage MCP Servers (mms) – Agent Guide

## Project Snapshot
`mms` is a Rust CLI that centralises Model Context Protocol (MCP) server management. It ships with an embedded master catalogue, provisions user-wide `~/.mcp.json` files, manages project-local catalogues, and synchronises Gemini and Codex configurations so assistants can launch with the correct MCP context automatically.

## Stack & Tooling
- **Language**: Rust (Edition 2024)
- **CLI**: `clap` with derive macros (see `src/cli.rs`)
- **Data Handling**: `serde`, `serde_json`, `toml_edit`
- **Error Handling**: `thiserror`
- **Tests**: `assert_cmd`, `tempfile`, `assert_fs`

## Code Map
- `src/cli.rs` – clap definitions and argument parsing
- `src/main.rs` – entrypoint wiring CLI to command dispatcher
- `src/commands.rs` – high-level command orchestration (init/add/remove/sync/clean)
- `src/config/` – catalogue accessors (master/global/local) and path helpers
- `src/integration/` – Gemini (`settings.json`) and Codex (`config.toml`) synchronisation
- `tests/` – integration tests using isolated temp homes (`tests/common/mod.rs`)

## Development Practices
- Format with `cargo fmt`
- Lint with `cargo clippy --all-targets --all-features -- -D warnings`
- Run tests via `cargo test`
- Environment placeholders (e.g., `${MMS_GITHUB_PAT}`) are resolved from real env vars; `.env` files are not auto-loaded

## Typical Workflows
- Initialise local catalogue: `mms init [--from-global]`
- Add/remove servers: `mms add <names…>` / `mms remove <name>`
- Sync assistants: `mms sync [--skip-gemini] [--skip-codex]`
- Clean artefacts: `mms clean [--local|--global|--master|--all] [--dry-run]`

## Testing Notes
- Integration tests (`tests/init.rs`, `tests/manage.rs`, `tests/sync.rs`) compile and invoke the binary inside a sandboxed HOME
- No unit tests remain in `src/` after the migration; prefer CLI-level coverage mirroring real usage
