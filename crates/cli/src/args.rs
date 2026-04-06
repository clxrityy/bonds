use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "bond",
    version,
    about = "Manage directory bonds (symlinks with tracking)"
)]
pub struct Cli {
    /// Path to the database file (overrides default ~/.bonds/bonds.db)
    #[arg(long, global = true)]
    pub db: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new bond from source to target
    Add {
        /// The source directory or file to bond
        source: PathBuf,
        /// The target location (defaults to current directory + source name)
        target: Option<PathBuf>,
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
    },
}

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
