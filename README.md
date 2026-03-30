# Bonds | The idea

Based off of GNU Stow, rather than *only* symlinking dotfiles, Bonds aims to provide a more flexible and powerful way to bond directories to different locations for ease of management and organization.  

While also being a basic CLI tool utilizing symlinks, Bonds will:

- [ ] Be an application, library, and extensible framework that can be utilized in various different forms including but not limited to a *bond* plugin on editors, other applications (games, web applications, etc.).
- [ ] Contain an API to programmatically manage and manipulate bonds, allowing developers to integrate Bonds' functionality into their own applications and workflows.

## Examples

### CLI

```bash
# Add a new bond
bond <source_directory> [target_directory]
```

- [ ] If the target directory is not specified, the bond will be created in the current working directory (or the default bond directory if configured).

### Application

- The main part of the window will have the application viewer, which displays the current bonds (in order from which have been recently updated).
  - Since there will be *no* bonds initially, the application viewer will display a bigger button to create a new bond.
  - The viewer will display the folders and applications that are currently bonded, allowing users to quickly access and manage them.
  - Users will be able to click on a bonded folder or application to open it directly from the application viewer.
  - The viewer will also provide options to edit or remove existing bonds, giving users full control over their bonded directories and applications.
- The sidebar/toolbar will have "bookmarks" or "favorites" section where users can quickly access their most frequently used bonds.
- Will show the details of the selected bond, including its source and target directories, last modified date, and any associated metadata.
  - Along with visualizations of the bond's structure and relationships to other bonds, providing users with a clear understanding of how their directories and applications are interconnected.

### API

- The API will provide functions to create, read, update, and delete bonds programmatically.
- Developers will be able to integrate Bond's functionality into their own applications, allowing for automated management of bonded directories and applications.
- The API will include methods to query the status of bonds, retrieve metadata, and visualize the relationships between different bonds.
- The API will also provide hooks and events that developers can listen to, allowing them to respond to changes in bonds in real-time.
- The API will also provide documentation and examples to help developers understand how to use the various functions and integrate Bond into their own projects.

### Tools & Utilities

- **Bond CLI**: A command-line interface for managing bonds, allowing users to create, view, and delete bonds directly from the terminal.
- **Bond GUI**: A graphical user interface application for managing bonds, providing a visual representation of bonded directories and applications.
- **Bond API**: A programmatic interface for integrating Bond's functionality into other applications, enabling automated management of bonds.
- **Bond Plugins**: Extend Bond's functionality through plugins for various editors and applications, allowing seamless integration into existing workflows.

## Language & Frameworks

- **Programming Language**: Rust
- **Frameworks**: Tauri (for the GUI application), Clap (for the CLI application), Serde (for serialization and deserialization of bond data), Tokio (for asynchronous operations), and any other relevant Rust crates for building the application.

## Features

- [ ] Contents of a directory, or *multiple* directories, can be bonded to different target location(s). (e.g., bonding `~/projects/*` to `~/bonds/new-projects/*`)
- [ ] If the target directory is not specified, the bond will be created in the current working, or default, directory.
  - [ ] Within the application (and likely CLI too), the default location will be something like `~/bonds`.
  - [ ] The default location is configurable.
    - [ ] If the default location is changed while there are existing bonds, those bonds will remain in their original locations unless explicitly moved to the new default location.
      - [ ] There will be an option/flag/command to migrate existing bonds to the new default location.
      - [ ] Changing the default location automatically creates a *backup* of the existing bonds in their original locations.
        - [ ] Eases the process of migrating bonds to a new default location without losing any data.
- [ ] A directory itself can be bonded into another directory (e.g., bonding `~/projects` into `~/bonds/projects`).
  - [ ] This allows for creating a hierarchical structure of bonded directories, where a parent directory can contain multiple bonded subdirectories.
  - [ ] Users can navigate through the bonded directories in the application viewer, maintaining a clear understanding of the relationships between different bonds.
- [ ] Users can view the details of a selected bond, including its source and target directories, last modified date, and any associated metadata.
  - [ ] The application viewer will provide visualizations of the bond's structure and relationships to other bonds, helping users understand how their directories and applications are interconnected.
- [ ] Upon deleting a bond, the application will provide an option to either delete the bond only or delete both the bond and the target directory.
  - [ ] Deleting a bond will remove the association between the source and target directories without affecting the actual files.
  - [ ] Deleting a bond along with the target directory will remove both the bond and the files in the target directory, with a confirmation prompt to prevent accidental data loss.
