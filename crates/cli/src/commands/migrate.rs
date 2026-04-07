use bonds_core::{BondError, BondManager, BondsConfig};
use std::path::PathBuf;

pub fn cmd_migrate(
    manager: &BondManager,
    id: &str,
    dest: Option<PathBuf>,
) -> Result<(), BondError> {
    let bond = manager.get_bond(id)?;

    // Resolve destination directory
    let dest_dir = match dest {
        Some(d) => std::path::absolute(&d)
            .map_err(|_| BondError::InvalidPath(format!("cannot resolve path: {}", d.display())))?,
        None => {
            let config = BondsConfig::load()?;
            config.default_target.ok_or_else(|| {
                BondError::Config("no destination given and no default directory configured".into())
            })?
        }
    };

    // Preserve the existing target's basename
    let basename = bond.target.file_name().ok_or_else(|| {
        BondError::InvalidPath(format!(
            "target has no file name: {}",
            bond.target.display()
        ))
    })?;

    let new_target = dest_dir.join(basename);

    // No-op if already there
    if new_target == bond.target {
        println!("Bond '{}' is already at {}", id, new_target.display());
        return Ok(());
    }

    // Delegate to update_bond -- it handles symlink removal, creation, and DB update
    let updated = manager.update_bond(&bond.id, None, Some(new_target), None)?;
    println!("Bond migrated: {}", id);
    println!(
        "  {} -> {}",
        updated.source.display(),
        updated.target.display()
    );
    Ok(())
}
