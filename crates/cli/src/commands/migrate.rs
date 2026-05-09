use bonds_cli::ui;
use bonds_core::{BondError, BondManager, BondsConfig};
use std::path::PathBuf;

/// Migrate command handler, which moves the target of a bond to a new location. The command retrieves the bond by its ID or name, resolves the destination directory (either from the provided argument or from the default configuration), and constructs the new target path by preserving the basename of the existing target. If the bond is already at the destination, it informs the user and exits without making changes. Otherwise, it delegates to the bond manager's `update_bond` method to handle the migration, which includes updating symlinks and the database record. Upon successful migration, it provides user feedback with details of the old and new target paths.
/// **Example usage:**
/// ```bash
/// bond migrate <id | name> [destination]
/// ```
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
    let basename = bond.target().file_name().ok_or_else(|| {
        BondError::InvalidPath(format!(
            "target has no file name: {}",
            bond.target().display()
        ))
    })?;

    let new_target = dest_dir.join(basename);

    // No-op if already there
    if new_target == bond.target() {
        ui::warning(format!(
            "Bond '{}' is already at {}",
            id,
            new_target.display()
        ));
        return Ok(());
    }

    // Delegate to update_bond -- it handles symlink removal, creation, and DB update
    let updated = manager.update_bond(bond.id(), None, Some(new_target), None)?;
    ui::status_ok("✓ SUCCESS");
    ui::success("Bond migrated successfully:");
    ui::subheading(format!("   {}", updated.id()));
    ui::key(format!("   {}", updated.source().display()));
    ui::normal("    ⤡ ");
    ui::path(format!("    {}\n", updated.target().display()));
    // ui::success(format!("Bond migrated: {}", id));
    // ui::info(format!(
    //     "  {} -> {}",
    //     updated.source().display(),
    //     updated.target().display()
    // ));
    Ok(())
}
