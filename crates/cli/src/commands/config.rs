//! Command handler for the `config` command, which allows users to view or modify configuration settings for the bonds CLI application. The command supports subcommands for getting and setting specific configuration keys, such as the default target directory for new bonds. The handler loads the current configuration, performs the requested action based on the subcommand, and provides user feedback on the success or failure of the operation.

use bonds_cli::ui;
use bonds_core::{BondError, BondsConfig};
use std::path::PathBuf;

/// Command handler for the `config get` subcommand, which retrieves the current value of a specified configuration key and displays it to the user. If the key is not recognized, it returns an error indicating that the config key is unknown.
/// **Example usage:**
/// ```bash
/// bond config get default
/// ```
pub fn cmd_config_get(key: &str) -> Result<(), BondError> {
    let config = BondsConfig::load()?;

    match key {
        "default" => match config.default_target {
            Some(p) => ui::key(format!("{}", p.display())),
            None => ui::info("(not set)"),
        },
        _ => return Err(BondError::Config(format!("unknown config key: {key}"))),
    };

    Ok(())
}

/// Command handler for the `config set` subcommand, which updates a specified configuration key with a new value. It validates the input value based on the expected format for the key (e.g., ensuring that a path is valid for the `default` key) and saves the updated configuration. If the key is not recognized, it returns an error indicating that the config key is unknown.
/// **Example usage:**
/// ```bash
/// bond config set default /path/to/default/dir
/// ```
pub fn cmd_config_set(key: &str, value: &str) -> Result<(), BondError> {
    let mut config = BondsConfig::load()?;

    match key {
        "default" => {
            let path = PathBuf::from(value);

            let path = std::path::absolute(&path).map_err(|_| {
                BondError::InvalidPath(format!("cannot resolve path: {}", path.display()))
            })?;

            config.default_target = Some(path.clone());
            config.save()?;
            ui::status_ok("✓ SUCCESS");
            ui::info("Default target directory updated to:");
            ui::subheading(format!("{}", path.display()));
        }
        _ => return Err(BondError::Config(format!("unknown config key: {key}"))),
    }

    Ok(())
}
