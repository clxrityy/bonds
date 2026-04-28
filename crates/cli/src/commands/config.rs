use bonds_cli::ui;
use bonds_core::{BondError, BondsConfig};
use std::path::PathBuf;

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
            ui::success(format!("default_target set to: {}", path.display()));
        }
        _ => return Err(BondError::Config(format!("unknown config key: {key}"))),
    }

    Ok(())
}
