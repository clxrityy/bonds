# Bonds | Command Line Interface

**Install the CLI globally:**

![Crates.io Size](https://img.shields.io/crates/size/bonds-cli?style=flat)

```bash
cargo install bonds-cli
```

---

| Command | Args | Description | Example |
| --- | --- | --- | --- |
| **`bond`** | `--help` | Main entry point, shows help if no subcommand is provided. | `bond` |
| **`add`** | `<source>` `[--name <name>]` | Create a bond (symlink) from source to target. | `bond add ~/projects/my-app --name foo` |
| **`list`** | `N/A` | List all bonds. | `bond list` |
| **`info`** | `<name\|id>` | Show details about a specific bond. | `bond info foo` |
| **`remove`** | `<name\|id>` `[--with-target]` | Remove a bond, with optional target deletion. | `bond remove foo` |
| **`update`** | `<name\|id>` `[--source <new-source>]` `[--target <new-target>]` | Update a bond's source and/or target. | `bond update foo --source ~/projects/new-app` |
| **`migrate`** | `<name\|id>` `[new-target]` | Move existing bonds to a new target location (with auto-backup). Moves to the default target if none is provided. | `bond migrate ~/new-bonds` |
| **`config`** | `<get\|set> <key>` | Get or set configuration values. | `bond config set default ~/my-bonds` |
