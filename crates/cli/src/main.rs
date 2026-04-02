mod args;
mod commands;

use args::{Cli, Commands};
use clap::Parser;
use commands::{cmd_add, cmd_info, cmd_list, cmd_remove};

fn main() {
    let cli = Cli::parse();

    let manager = match bonds_core::BondManager::new(cli.db) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to initialize bond manager: {e}");
            std::process::exit(1);
        }
    };

    let result = match cli.command {
        Commands::Add { source, target } => cmd_add(&manager, source, target),
        Commands::List => cmd_list(&manager),
        Commands::Info { id } => cmd_info(&manager, &id),
        Commands::Remove { id, with_target } => cmd_remove(&manager, &id, with_target),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
