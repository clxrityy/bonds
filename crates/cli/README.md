# bonds-cli

[![Crates.io](https://img.shields.io/crates/v/bonds-cli.svg)](https://crates.io/crates/bonds-cli) [![CI](https://github.com/clxrityy/bonds/actions/workflows/ci.yml/badge.svg)](https://github.com/clxrityy/bonds/actions/workflows/ci.yml) [![Documentation](https://img.shields.io/badge/docs-blue?logo=rust&logoColor=white)](https://bonds.fyi/latest/api/bonds_cli/) [![License](https://img.shields.io/github/license/clxrityy/bonds.svg)](https://github.com/clxrityy/bonds/blob/master/LICENSE)

Relies on [bonds-core](https://bonds.fyi/latest/api/bonds_core/) for core logic, and provides a command-line interface for managing symlinks.

> It offers features such as creating, listing, and removing symlinks, as well as advanced functionalities like bulk operations and integration with file explorers. The CLI is designed to be user-friendly and efficient, making it easy to manage symlinks directly from the terminal.

![Crates.io Size](https://img.shields.io/crates/size/bonds-cli?style=flat)

```bash
cargo install bonds-cli
```

---

### Usage

- [Detailed usage instructions](#detailed-usage-instructions)
- [Adding a bond](#adding-a-bond)
- [Listing bonds](#listing-bonds)
- [Inspecting bond details](#inspecting-bond-details)
- [Removing a bond](#removing-a-bond)
- [Updating a bond](#updating-a-bond)
- [Migrating bonds](#migrating-bonds)
- [Configuration](#configuration)

##### Detailed usage instructions

```bash
# for detailed usage instructions
bond --help
```

##### Adding a bond

```bash
# create a bond
bond add ~/projects/my-app
# creates a bond (symlink):
# ~/projects/my-app -> ~/bonds/my-app

# give the bond a custom name
bond add ~/projects/my-app --name foo
```

##### Listing bonds

```bash
# list bonds
bond list
# OUTPUT:
# foo (abc12345) - ~/projects/my-app -> ~/bonds/my-app  (2026-04-06 12:00)
```

##### Inspecting bond details

```bash
# inspect bond details
bond info foo
# OUTPUT:
# ID: abc12345
# Source: ~/projects/my-app
# Target: ~/bonds/my-app
# Created At: 2026-04-06 12:00
# Health: Healthy (symlink intact)
```

##### Removing a bond

```bash
# remove a bond by name
bond remove foo
# To also delete the target, use:
bond remove foo --with-target
# by ID
bond remove abc12345
```

##### Updating a bond

```bash
# Update source and/or target
bond update foo --source ~/new-source/my-app
bond update foo --target ~/new-target/my-app
# Update name
bond update foo --name new-name
```

##### Migrating bonds

```bash
# Migrate bonds from an old target directory to a new one
bond migrate foo ~/new-target/
# if the bond is located in a different location (than ~/bonds or default target), you can run:
bond migrate foo # moves it to the default target (~/bonds)
```

##### Configuration

```bash
# View current default target directory
bond config get default
# Set default target directory
bond config set default ~/my-default-bonds
```
