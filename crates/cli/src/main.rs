//! The main entry point for the bonds CLI application. It parses command-line arguments, initializes the bond manager, and dispatches commands to their respective handlers.
//!     The CLI supports various commands for managing symlinks, including adding, listing, updating, and removing bonds, as well as configuration and metadata management.
//! - Error handling is centralized, ensuring that any issues encountered during command execution are reported in a consistent manner.
//! - The application relies on the [`bonds-core`](https://crates.io/crates/bonds-core) crate for core logic and uses the `clap` crate for argument parsing. The user interface is handled through the `ui` module, which provides formatted output for errors and other messages.

use bonds_cli::args::{Cli, Commands, ConfigAction, MetadataAction};
use commands::{
    cmd_add, cmd_config_get, cmd_config_set, cmd_info, cmd_list, cmd_metadata_get,
    cmd_metadata_remove, cmd_metadata_set, cmd_migrate, cmd_remove, cmd_update,
};

/// The `commands` module contains the implementation of all the command handlers for the CLI application. Each command corresponds to a specific action that can be performed on the bonds, such as adding a new bond, listing existing bonds, updating bond information, and managing metadata. The command handlers interact with the `BondManager` from the `bonds-core` crate to perform the necessary operations on the underlying data store. This modular structure allows for clean separation of concerns and makes it easier to maintain and extend the CLI functionality in the future.
mod commands;
/// The `ui` module provides functions for formatting error messages and other user-facing output in a consistent and visually appealing way. It is used throughout the CLI application to ensure that all messages are presented in a clear and user-friendly manner.
mod ui;

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Config { action } => match action {
            ConfigAction::Get { key } => cmd_config_get(&key),
            ConfigAction::Set { key, value } => cmd_config_set(&key, &value),
        },
        cmd => {
            // Only init DB for commands that need it.
            let manager = match bonds_core::BondManager::new(cli.db) {
                Ok(m) => m,
                Err(e) => {
                    // Use the same color path as all other errors.
                    eprintln!(
                        "{}",
                        ui::format_context_error("Failed to initialize bond manager", &e)
                    );
                    std::process::exit(1);
                }
            };

            match cmd {
                Commands::Add {
                    source,
                    target,
                    contents,
                    name,
                } => cmd_add(&manager, source, target, contents, name),
                Commands::List => cmd_list(&manager),
                Commands::Info { id } => cmd_info(&manager, &id),
                Commands::Remove { id, with_target } => cmd_remove(&manager, &id, with_target),
                Commands::Config { .. } => unreachable!(),
                Commands::Update {
                    id,
                    source,
                    target,
                    name,
                } => cmd_update(&manager, &id, source, target, name),
                Commands::Migrate { id, dest } => cmd_migrate(&manager, &id, dest),
                Commands::Metadata { action } => match action {
                    MetadataAction::Get { id, key } => {
                        // None key means "print all metadata"
                        cmd_metadata_get(&manager, &id, key.as_deref())
                    }
                    MetadataAction::Set { id, key, value } => {
                        // Upsert metadata key/value on the selected bond
                        cmd_metadata_set(&manager, &id, &key, &value)
                    }
                    MetadataAction::Remove { id, key } => {
                        // Remove a single key from metadata map
                        cmd_metadata_remove(&manager, &id, &key)
                    }
                },
            }
        }
    };

    if let Err(e) = result {
        // One place for every command failure.
        eprintln!("{}", ui::format_error(&e));
        std::process::exit(1);
    }
}
