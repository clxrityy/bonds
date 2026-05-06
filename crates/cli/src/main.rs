use bonds_cli::args::{Cli, Commands, ConfigAction, MetadataAction};
use commands::{
    cmd_add, cmd_config_get, cmd_config_set, cmd_info, cmd_list, cmd_metadata_get,
    cmd_metadata_remove, cmd_metadata_set, cmd_migrate, cmd_remove, cmd_update,
};
mod commands;
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
