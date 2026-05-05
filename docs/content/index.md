# Bonds

[![CI](https://github.com/clxrityy/bonds/actions/workflows/ci.yml/badge.svg)](https://github.com/clxrityy/bonds/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/clxrityy/bonds.svg)](https://github.com/clxrityy/bonds/blob/master/LICENSE)

Bonds is a tool for creating and managing "bonds" between directories using symlinks. A bond is a persistent, bidirectional link between a source directory and a target directory. This allows you to keep your files organized in one place while accessing them from another location.

**Bonds provides:**

[![Core Crates.io Version](https://img.shields.io/crates/v/bonds-core?style=flat&label=bonds-core)](https://crates.io/crates/bonds-core) [![CLI Crates.io Version](https://img.shields.io/crates/v/bonds-cli?style=flat&label=bonds-cli)](https://crates.io/crates/bonds-cli)

- A CLI ([`bonds-cli`](https://bonds.fyi/latest/api/bonds_cli/)) for managing symlink-based bonds.
- A Rust library ([`bonds-core`](https://bonds.fyi/latest/api/bonds_core/)) for programmatic integration.

```bash
# Install the CLI globally
cargo install bonds-cli
```

```bash
# Add bonds-core to your Rust project
cargo add bonds-core
```
