# Bonds | Contributing | Development Workflow

This page covers the day-to-day development workflow for contributing to Bonds.

## Repository layout

The main workspace structure is:

- `crates/core/` — core library code and tests
- `crates/cli/` — CLI entry points, commands, and tests
- `crates/app/` — Tauri shell and web frontend
- `docs/content/` — documentation source files
- `scripts/` — release and versioning utilities
- `.github/` — repository policies and CI workflows

## Prerequisites

For most contributions, you will want:

- Rust with a stable toolchain
- `pnpm` for the desktop app
- Python 3 for the documentation toolchain
- `make` for the convenience commands in the root `Makefile`

If you plan to work on `crates/app`, make sure your system also has the standard Tauri prerequisites for your platform installed.

## Initial setup

After cloning the repository and moving into the project directory, start with:

```bash
make help
make setup
```

`make setup` will:

- fetch Rust dependencies the workspace
- install app dependencies in `crates/app`
- create Python virtual environment for documentation toolchain

## Working conventions

##### Keep shared behavior in `bonds-core`

If logic is shared by the CLI and the app, it should usually live in `crates/core`.

Good examples include:

- bond lifecycle behavior
- path resolution and validation
- persistence and query logic
- metadata behavior
- event hooks and shared domain rules

Try to avoid duplicating core behavior in the CLI or frontend when it belongs in the shared library.

##### Keep interfaces thin

- `crates/cli` should stay focused on parsing input, calling into the core library, and formatting output
- `crates/app` should stay focused on UI, desktop integration, and presentation concerns

##### Prefer focused changes

Small, reviewable pull requests are much easier to validate and merge than broad refactors mixed with feature work.

Please avoid:

- unrelated cleanup in the same PR
- drive-by formatting changes in untouched files
- large refactors without a clear reason

##### Preserve cross-platform behavior

Bonds is validated across multiple operating systems. Be careful around:

- filesystem behavior
- symlink handling
- path normalization
- shell-specific assumptions
- OS-specific UI behavior in the app
- If behavior is platform-specific, document it clearly.

If behavior is platform-specific, document it clearly.

##### Keep docs in sync

If your change affects:

- CLI commands or flags
- public library APIs
- app workflows or screenshots
- architecture or roadmap expectations

please update the corresponding docs in `docs/content/`.

## Typical contribution flow

A good default workflow looks like this:

1. Fork the repository and create a focused branch
2. Make a small, intentional change
3. Add or update tests when behavior changes
4. Update docs when commands, APIs, or workflows change
5. Run the relevant checks locally
6. Open a pull request with a clear description of what changed and why

For larger features, opening an issue or discussion first is appreciated so the work can be aligned with the roadmap before a lot of code lands.
