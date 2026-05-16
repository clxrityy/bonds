# Bonds

[![CI](https://github.com/clxrityy/bonds/actions/workflows/ci.yml/badge.svg)](https://github.com/clxrityy/bonds/actions/workflows/ci.yml) ![License](https://img.shields.io/badge/GPL--v3.0-LICENSE?style=flat&logo=gnuprivacyguard&logoSize=auto&label=License&labelColor=auto&color=%230093DD&link=https%3A%2F%2Fgithub.com%2Fclxrityy%2Fbonds%2Fblob%2Fmaster%2FLICENSE)

Bonds is a tool for creating and managing "bonds" between directories using symlinks. A bond is a persistent, bidirectional link between a source directory and a target directory. This allows you to keep your files organized in one place while accessing them from another location.

#### What's inside?

###### **1.** A command line interface ([`bonds-cli`](https://bonds.fyi/latest/api/bonds_cli/)) for managing symlink-based bonds

> ```bash
> # Install the CLI globally
> cargo install bonds-cli
> ```
>
> [![CLI Crates.io Version](https://img.shields.io/crates/v/bonds-cli?style=flat&label=bonds-cli)](https://crates.io/crates/bonds-cli)

###### **2.** A Rust library ([`bonds-core`](https://bonds.fyi/latest/api/bonds_core/)) for programmatic integration

> ```bash
> # Add bonds-core to your Rust project
> cargo add bonds-core
> ```
>
> [![Core Crates.io Version](https://img.shields.io/crates/v/bonds-core?style=flat&label=bonds-core)](https://crates.io/crates/bonds-core)
