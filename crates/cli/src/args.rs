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
}
