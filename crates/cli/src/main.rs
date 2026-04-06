mod args;
mod commands;

use args::{Cli, Commands, ConfigAction};
use clap::Parser;
use commands::{cmd_add, cmd_config_get, cmd_config_set, cmd_info, cmd_list, cmd_remove};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Config { action } => match action {
            ConfigAction::Get { key } => cmd_config_get(&key),
            ConfigAction::Set { key, value } => cmd_config_set(&key, &value),
        },
        cmd => {
            // Only init DB for commands that need it
            let manager = match bonds_core::BondManager::new(cli.db) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Failed to initialize bond manager: {e}");
                    std::process::exit(1);
                }
            };
            match cmd {
                Commands::Add { source, target } => cmd_add(&manager, source, target),
                Commands::List => cmd_list(&manager),
                Commands::Info { id } => cmd_info(&manager, &id),
                Commands::Remove { id, with_target } => cmd_remove(&manager, &id, with_target),
                Commands::Config { .. } => unreachable!(),
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
