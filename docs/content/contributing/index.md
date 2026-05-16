# Bonds | Contributing

Thank you for contributing to Bonds.

This documentation section is the canonical contributor guide for the project.
> The copy in `.github/CONTRIBUTING.md` is intentionally minimal and points here.

## In this guide

- [Development Workflow](development.md)
- [Testing](testing.md)
- [Pull Requests](pull-requests.md)
- [Reporting Issues](issues.md)

## Before you begin

It helps to review the project surface first:

- the repository `README.md`
- [Architecture](../architecture.md)
- [Roadmap](../roadmap.md)
- [CLI Reference](../cli.md)

A few expectations are consistent across most contributions:

- shared behavior should usually live in `crates/core`
- CLI- and app-specific layers should stay thin
- behavior changes should come with tests when practical
- user-facing changes should come with documentation updates
- contributions should remain focused and easy to review

## Project shape

Bonds is organized as a Rust workspace with three primary packages:

- `crates/core` — the shared Rust library (`bonds-core`)
- `crates/cli` — the command-line interface (`bonds-cli`)
- `crates/app` — the desktop application built with Tauri, React, and TypeScript

At a high level:

- `bonds-core` is the source of truth for shared behavior
- the CLI is responsible for parsing, orchestration, and terminal output
- the desktop app is responsible for UI and desktop workflows

## Related policies

- [Security Policy](../resources/security.md)
- [Code of Conduct](../resources/code-of-conduct.md)
- [License](../resources/license.md)
