use bonds_cli::ui;
use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

/// Update command handler, which modifies the source path, target path, or name of an existing bond. The command accepts optional parameters for the source and target, allowing users to update any combination of these attributes. It validates that at least one of the source, target, or name is provided and resolves the paths to absolute form. The command then calls the bond manager's `update_bond` method to apply the changes and provides user feedback on the success of the operation, including details of the updated bond's source and target paths.
/// **Example usage:**
/// ```bash
/// bond update <id | name> [--source <new_source>] [--target <new_target>] [--name <new_name>]
/// ```
pub fn cmd_update(
    manager: &BondManager,
    id: &str,
    source: Option<PathBuf>,
    target: Option<PathBuf>,
    name: Option<String>,
    dry_run: bool,
    verbose: bool,
) -> Result<(), BondError> {
    // Validate that at least one of source, target, or name is provided
    if source.is_none() && target.is_none() && name.is_none() {
        return Err(BondError::InvalidPath(
            "provide at least one of --source, --target, or --name".into(),
        ));
    }

    // Resolve source to absolute if provided
    let source = match source {
        Some(s) => Some(s.canonicalize().map_err(|_| {
            BondError::InvalidPath(format!("cannot resolve source: {}", s.display()))
        })?),
        None => None,
    };

    // Resolve target to absolute if provided (doesn't need to exist yet)
    let target = match target {
        Some(t) => Some(std::path::absolute(&t).map_err(|_| {
            BondError::InvalidPath(format!("cannot resolve target: {}", t.display()))
        })?),
        None => None,
    };

    if verbose {
        ui::debug(format!("Updating bond with ID: {}", id));
        if let Some(ref s) = source {
            ui::debug(format!("New source path: {}", s.display()));
        }
        if let Some(ref t) = target {
            ui::debug(format!("New target path: {}", t.display()));
        }
        if let Some(ref n) = name {
            ui::debug(format!("New name: {}", n));
        }
    }

    if dry_run {
        ui::warning("Dry run enabled: no changes will be made.");
        ui::info(format!("Would update bond with ID: {}", id));
        if let Some(ref s) = source {
            ui::info(format!("New source path: {}", s.display()));
        }
        if let Some(ref t) = target {
            ui::info(format!("New target path: {}", t.display()));
        }
        if let Some(ref n) = name {
            ui::info(format!("New name: {}", n));
        }
        return Ok(());
    }

    let bond = manager.update_bond(id, source, target, name)?;

    ui::status_ok("✓ SUCCESS");
    ui::success("Bond updated successfully:");
    ui::subheading(format!("   {}", bond.id()));
    ui::dim(format!("   {}", bond.source().display()));
    ui::normal("    ⤡ ");
    ui::dim(format!("    {}\n", bond.target().display()));

    Ok(())
}
