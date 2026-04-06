use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

pub fn cmd_add(
    manager: &BondManager,
    source: PathBuf,
    target: Option<PathBuf>,
) -> Result<(), BondError> {
    let source = source.canonicalize().map_err(|_| {
        BondError::InvalidPath(format!("cannot resolve source: {}", source.display()))
    })?;

    let target = match target {
        Some(t) => t,
        None => {
            let config = bonds_core::BondsConfig::load().unwrap_or_default(); // fail gracefully; don't block add

            let name = source
                .file_name()
                .ok_or_else(|| BondError::InvalidPath("source has no file name".into()))?;

            // Use configured default_target if set, else fall back to CWD
            match config.default_target {
                Some(default_dir) => default_dir.join(name),
                None => std::env::current_dir()?.join(name),
            }
        }
    };

    let bond = manager.create_bond(&source, &target)?;
    println!("Bond created: {}", bond.id);
    println!("  {} -> {}", bond.source.display(), bond.target.display());
    Ok(())
}
