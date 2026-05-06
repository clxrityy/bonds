use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "bond",
    version,
    about = "Manage directory bonds (symlinks with tracking)"
)]

/// The `Cli` struct defines the command-line interface for the bonds CLI application. It uses the `clap` crate to parse command-line arguments and subcommands. The `db` field allows users to specify a custom path to the database file, while the `command` field represents the specific action that the user wants to perform, such as adding a new bond, listing existing bonds, or managing configuration. Each command is defined as a variant of the `Commands` enum, which further organizes related actions into subcommands for better usability and clarity.
pub struct Cli {
    /// Path to the database file (overrides default ~/.bonds/bonds.db)
    #[arg(long, global = true)]
    pub db: Option<PathBuf>,

    /// Command to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// The `Commands` enum defines the various commands that the bonds CLI application supports. Each variant corresponds to a specific action that can be performed on the bonds, such as adding a new bond, listing existing bonds, updating bond information, and managing metadata. Some commands have their own subcommands for more granular actions, such as the `Config` and `Metadata` commands. This structure allows for a clear and organized command-line interface, making it easier for users to understand and use the available functionality of the CLI application.
#[derive(Subcommand)]
pub enum Commands {
    /// Create a new bond from source to target
    Add {
        /// The source directory or file to bond
        source: PathBuf,
        /// The target location (defaults to current directory + source name)
        target: Option<PathBuf>,
        /// Bond each child of source as a separate bond into target
        #[arg(long)]
        contents: bool,
        /// Give this bond a name for easy reference
        #[arg(long)]
        name: Option<String>,
    },

    /// List all bonds
    List,

    /// Show details of a specific bond
    Info {
        /// Bond ID
        id: String,
    },

    /// Remove a bond
    Remove {
        /// Bond ID
        id: String,
        /// Also delete the target directory/file (not just the symlink)
        #[arg(long)]
        with_target: bool,
    },

    /// View or modify configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Update an existing bond's source or target
    Update {
        /// Bond ID (or prefix)
        id: String,
        /// New source path
        #[arg(long)]
        source: Option<PathBuf>,
        /// New target path
        #[arg(long)]
        target: Option<PathBuf>,
        /// Set or change the bond's name
        #[arg(long)]
        name: Option<String>,
    },

    /// Move a bond's target to a new directory
    Migrate {
        /// Bond name or ID prefix
        id: String,
        /// Destination directory (defaults to configured default directory)
        dest: Option<PathBuf>,
    },

    /// Read or modify metadata for a bond
    Metadata {
        #[command(subcommand)]
        action: MetadataAction,
    },
}

/// Subcommands for the `config` command, which allows users to get or set configuration values for the bonds CLI application. The `Get` variant retrieves the current value of a specified configuration key, while the `Set` variant updates a configuration key with a new value. This allows users to customize the behavior of the CLI application by modifying its configuration settings.
#[derive(Subcommand)]
pub enum ConfigAction {
    /// Get the current value of a config key
    Get {
        /// Config key to read (e.g., "default")
        key: String,
    },
    /// Set a config key to a new value
    Set {
        /// Config key to set (e.g., "default")
        key: String,
        /// New value
        value: String,
    },
}

/// Subcommands for the `metadata` command, which allows users to get, set, or remove metadata key-value pairs associated with a specific bond. The `Get` variant retrieves the value of a specified metadata key for a given bond, or all metadata if no key is provided. The `Set` variant updates or adds a metadata key-value pair for a specified bond. The `Remove` variant deletes a specific metadata key from a given bond. This functionality enables users to manage additional information about their bonds in a flexible way.
#[derive(Subcommand)]
pub enum MetadataAction {
    /// Print metadata; pass key to read a single value
    Get {
        /// Bond name, full ID, or unique ID prefix
        id: String,
        /// Optional metadata key
        key: Option<String>,
    },
    /// Set (upsert) one metadata key/value
    Set {
        /// Bond name, full ID, or unique ID prefix
        id: String,
        /// Metadata key
        key: String,
        /// Metadata value
        value: String,
    },
    /// Remove one metadata key
    Remove {
        /// Bond name, full ID, or unique ID prefix
        id: String,
        /// Metadata key to remove
        key: String,
    },
}
