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

    // Branded default landing output when no subcommand is provided.
    let Some(cmd) = cli.command else {
        ui::landing(env!("CARGO_PKG_VERSION"));
        return;
    };

    let result = match cmd {
        Commands::Config { action } => match action {
            ConfigAction::Get { key } => cmd_config_get(&key),
            ConfigAction::Set { key, value } => cmd_config_set(&key, &value),
        },
        Commands::Add {
            source,
            target,
            name,
            flags,
        } => {
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
            cmd_add(&manager, source, target, name, flags.dry_run, flags.verbose)
        }
        Commands::List => {
            let manager = match bonds_core::BondManager::new(cli.db) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!(
                        "{}",
                        ui::format_context_error("Failed to initialize bond manager", &e)
                    );
                    std::process::exit(1);
                }
            };
            cmd_list(&manager)
        }
        Commands::Info { id } => {
            let manager = match bonds_core::BondManager::new(cli.db) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!(
                        "{}",
                        ui::format_context_error("Failed to initialize bond manager", &e)
                    );
                    std::process::exit(1);
                }
            };
            cmd_info(&manager, &id)
        }
        Commands::Remove {
            id,
            with_target,
            flags,
        } => {
            let manager = match bonds_core::BondManager::new(cli.db) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!(
                        "{}",
                        ui::format_context_error("Failed to initialize bond manager", &e)
                    );
                    std::process::exit(1);
                }
            };
            cmd_remove(&manager, &id, with_target, flags.dry_run, flags.verbose)
        }
        Commands::Update {
            id,
            source,
            target,
            name,
            flags,
        } => {
            let manager = match bonds_core::BondManager::new(cli.db) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!(
                        "{}",
                        ui::format_context_error("Failed to initialize bond manager", &e)
                    );
                    std::process::exit(1);
                }
            };
            cmd_update(
                &manager,
                &id,
                source,
                target,
                name,
                flags.dry_run,
                flags.verbose,
            )
        }
        Commands::Migrate { id, dest, flags } => {
            let manager = match bonds_core::BondManager::new(cli.db) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!(
                        "{}",
                        ui::format_context_error("Failed to initialize bond manager", &e)
                    );
                    std::process::exit(1);
                }
            };
            cmd_migrate(&manager, &id, dest, flags.dry_run, flags.verbose)
        }
        Commands::Metadata { action } => {
            let manager = match bonds_core::BondManager::new(cli.db) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!(
                        "{}",
                        ui::format_context_error("Failed to initialize bond manager", &e)
                    );
                    std::process::exit(1);
                }
            };
            match action {
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
            }
        }
    };

    if let Err(e) = result {
        // One place for every command failure.
        ui::format_error(&e);
        std::process::exit(1);
    }
}
