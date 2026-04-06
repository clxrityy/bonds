# bonds

A tool for creating and managing "bonds" between files and directories.

> Build with [symlinks](https://en.wikipedia.org/wiki/Symbolic_link) and a [SQLite](https://www.sqlite.org/)
>> Inspired by [GNU Stow](https://www.gnu.org/software/stow/)

---

##### Creating a bond

```bash
# default target: ~/bonds
bond add ~/projects/my-app # creates a bond from ~/projects/my-app to ~/bonds/my-app
# custom target
bond add ~/projects/my-app ~/my-app-link
# bulk bond all contents of a directory
bond add ~/projects/ ~/bonds --contents # creates bonds for each child of ~/projects
# with a name
bond add ~/projects/my-app ~/bonds/my-app --name foo
```

##### Listing bonds

```bash
bond list
# OUTPUT:
# my-app (abc12345)  -  ~/projects/my-app -> ~/bonds/my-app  (2026-04-06 12:00)
```

##### Inspecting bond details

```bash
bond info abc12345
# OUTPUT:
# ID: abc12345
# Source: ~/projects/my-app
# Target: ~/bonds/my-app
# Created At: 2026-04-06 12:00
# Health: Healthy (symlink intact)
```

##### Removing a bond

```bash
# remove by ID
bond remove abc12345
# remove by (specified) name
bond remove foo
# To also delete the target, use:
bond remove abc12345 --with-target
```

##### Updating a bond

```bash
# Update source and/or target
bond update abc12345 --source ~/new-source/my-app
bond update abc12345 --target ~/new-target/my-app
# Update name
bond update abc12345 --name new-name
```

##### Migrating bonds

```bash
# Move bond to a new default location (with auto-backup)
bond migrate abc12345 ~/new-default-bonds/
# if the bond is located in a different location, you can run
bond migrate abc12345
# will automatically move the bond to the current default location (e.g., ~/bonds/)
```

##### Configuration

```bash
# Set default target directory
bond config set default ~/my-default-bonds
# Get current default target directory
bond config get default
```
