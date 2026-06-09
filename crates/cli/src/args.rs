//! The `args` module defines the command-line argument structures and parsing logic for the bonds CLI application using the `clap` crate. It includes the main `Cli` struct, which represents the overall command-line interface, and the `Commands` enum, which defines the various subcommands that users can execute. Each subcommand may have its own set of arguments and options, allowing for a flexible and user-friendly command-line experience when managing directory bonds (symlinks with tracking).

use clap::{Args, Parser, Subcommand};
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
    pub command: Option<Commands>,
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
        /// Give this bond a name for easy reference
        #[arg(long)]
        name: Option<String>,
        /// Common flags for mutating commands
        #[command(flatten)]
        flags: ActionFlags,
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
        /// Common flags for mutating commands
        #[command(flatten)]
        flags: ActionFlags,
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
        /// Common flags for mutating commands
        #[command(flatten)]
        flags: ActionFlags,
    },

    /// Move a bond's target to a new directory
    Migrate {
        /// Bond name or ID prefix
        id: String,
        /// Destination directory (defaults to configured default directory)
        dest: Option<PathBuf>,
        /// Common flags for mutating commands
        #[command(flatten)]
        flags: ActionFlags,
    },

    /// Read or modify metadata for a bond
    Metadata {
        #[command(subcommand)]
        action: MetadataAction,
    },

    /// Manage bond snapshot history
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },
}

/// Defines common command-line flags that can be reused across multiple mutating commands in the bonds CLI application. The `dry_run` flag allows users to preview the outcome of a command without making any actual changes to the filesystem or database, which is useful for testing and verification. The `verbose` flag enables the printing of extra execution details, providing users with more insight into what the command is doing, which can be helpful for debugging and inspection purposes. By centralizing these flags in a single struct, we can maintain consistency and reduce redundancy across different commands that perform mutations.
#[derive(Debug, Clone, Args, Default)]
pub struct ActionFlags {
    /// Preview outcome without applying filesystem/database changes.
    #[arg(long)]
    pub dry_run: bool,

    /// Print extra execution details for debugging/inspection.
    #[arg(long)]
    pub verbose: bool,
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

/// Subcommands for the `history` command, which allows users to manage bond snapshot history. The `Enable` variant sets up automated snapshots for a specified bond with a given cadence and retention policy. The `Disable` variant turns off automated snapshots for a specified bond. The `Snapshot` variant creates a snapshot of a specified bond immediately. The `List` variant displays all snapshots associated with a specified bond. The `Restore` variant allows users to restore a bond from a specific snapshot. The `Watch` variant runs a foreground process that continuously checks for due snapshot work and executes it according to the configured policies. This set of commands provides comprehensive management of bond snapshots and their restoration history.
#[derive(Subcommand)]
pub enum HistoryAction {
    /// Enable automated snapshots for a bond
    Enable {
        /// Bond name, full ID, or unique ID prefix
        id: String,
        /// Snapshot cadence, in seconds
        #[arg(long)]
        every_seconds: i64,
        /// Keep only the newest N snapshots
        #[arg(long, default_value_t = 10)]
        keep_last: i64,
        /// Optional override for where snapshots should be stored
        #[arg(long)]
        storage_root: Option<PathBuf>,
    },

    /// Disable automated snapshots for a bond
    Disable {
        /// Bond name, full ID, or unique ID prefix
        id: String,
    },

    /// Create a snapshot immediately
    Snapshot {
        /// Bond name, full ID, or unique ID prefix
        id: String,
    },

    /// List snapshots for a bond
    List {
        /// Bond name, full ID, or unique ID prefix
        id: String,
    },

    /// Restore a bond from a snapshot
    Restore {
        /// Bond name, full ID, or unique ID prefix
        id: String,
        /// Snapshot ID or unique prefix
        snapshot_id: String,
    },

    /// Run a foreground watcher that keeps snapshotting until stopped
    Watch {
        /// Watch only one configured bond
        #[arg(long, conflicts_with = "all_enabled")]
        bond: Option<String>,
        /// Watch all enabled bond policies
        #[arg(long)]
        all_enabled: bool,
        /// Polling interval for due work
        #[arg(long, default_value_t = 10)]
        poll_seconds: u64,
        /// Print extra loop details
        #[arg(long)]
        verbose: bool,
    },

    /// Delete a snapshot
    Delete {
        /// Bond name, full ID, or unique ID prefix
        id: String,
        /// Snapshot ID or unique prefix
        snapshot_id: String,
    },
}
