## Overview

`mms` is a Rust CLI for administering Model Context Protocol (MCP) server catalogues without depending on external provisioning scripts. It embeds the authoritative server list, keeps `~/.mcp.json` in sync (resolving environment placeholders such as `${MMS_GITHUB_PAT}` when set), and mirrors project-local catalogues into Gemini and Codex configurations so assistants can start with the right context automatically.

## Features

- **Embedded master catalogue** – the canonical MCP server list ships with the binary and is persisted under `~/.config/mms/master.json`.
- **Global initialisation** – automatically provisions `~/.mcp.json` with environment-variable substitution (e.g. `${MMS_GITHUB_PAT}`) when first needed.
- **Project workflows** – create, populate, and trim local `.mcp.json` files with `init`, `add`, `remove`, and `clean` commands.
- **Assistant integrations** – `sync` updates `.gemini/settings.json` in the workspace and the `[mcp_servers]` block inside `~/.codex/config.toml`.
- **Command discovery** – `command` prints the full server launch command and can copy it to the clipboard on macOS.

## Installation

```bash
cargo install --path .
# or build locally
cargo build --release
```

The optimised binary is emitted at `target/release/mms`.

## Usage

```text
mms [OPTIONS] <COMMAND>

Commands:
  init        Initialise a project-local .mcp.json (empty or from global)
  list        Display servers registered in ~/.mcp.json
  add         Add servers from the global catalogue into the project file
  remove      Remove a server from the project file
  command     Show the launch command for a server (optionally copy it)
  sync        Sync the project file with Gemini and Codex configurations
  clean       Remove generated catalogues and caches (local/global/master)
  help        Print command-specific help

Options:
  -v, --verbose   Enable verbose logging
      --version   Show version information
  -h, --help      Show the global usage message
```

### Typical Workflow

```bash
# Create an empty project catalogue in the current directory
mms init

# Add two servers maintained in ~/.mcp.json
mms add context7 serena

# Regenerate Gemini and Codex config from the project file
mms sync

# Inspect the launch command for a server
mms command context7 --copy

# Reset generated files (local catalogue + Gemini settings)
mms clean
```

### Clean Command Flags

```text
mms clean [--local] [--gemini] [--global] [--master] [--all] [--dry-run]
```

- `--local` removes the nearest `.mcp.json` discovered from the current directory (default selection when no flags are provided).
- `--gemini` deletes `.gemini/settings.json` alongside the project catalogue.
- `--global` removes `~/.mcp.json` so it will be recreated from the embedded master on the next command.
- `--master` removes the cached master copy at `~/.config/mms/master.json`.
- `--all` selects every scope at once (local, Gemini, Codex, global, master).
- `--dry-run` previews removals without touching the filesystem.

### Environment Variables

- Set secrets such as `MMS_GITHUB_PAT` in your shell before running commands. The CLI does **not** load `.env` files automatically.
- When the variable is present, `~/.mcp.json` is written with the resolved value; otherwise the `${MMS_GITHUB_PAT}` placeholder remains for downstream tools to interpret.

## Testing

The project relies on integration tests that exercise the compiled binary inside isolated temporary workspaces:

```bash
cargo test
```

Test helpers live in `tests/common/` and sandbox `HOME` so operations never touch real user files.

## Updating the Embedded Catalogue

The JSON catalogue bundled at `src/config/master_data.json` is treated as the single source of truth for `mms`. When upstream MCP server definitions change, refresh that file and rebuild the CLI so the embedded master and any derived global catalogues stay aligned.
