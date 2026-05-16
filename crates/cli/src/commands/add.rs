use bonds_cli::ui;
use bonds_core::{BondError, BondManager};
use std::path::{PathBuf};

/// The `add` command allows users to create a new bond between a source and target path. It supports options for bonding the contents of a directory as separate bonds, as well as giving the bond a name for easy reference. The command handles various error cases, such as invalid paths or issues during bond creation, and provides user-friendly output to indicate the success or failure of the operation.
/// **Example usage:**
/// ```bash
/// bond add /path/to/target --name foo
/// ```
pub fn cmd_add(
    manager: &BondManager,
    source: PathBuf,
    target: Option<PathBuf>,
    name: Option<String>,
    dry_run: bool,
    verbose: bool,
) -> Result<(), BondError> {

    if verbose {
        ui::debug(format!("Adding bond with source: {}", source.display()));
        if let Some(ref t) = target {
            ui::debug(format!("Target specified: {}", t.display()));
        } else {
            ui::debug("No target specified, will use default configuration or current directory.");
        }
        if let Some(ref n) = name {
            ui::debug(format!("Bond name: {}", n));
        } else {
            ui::debug("No name specified, bond will be unnamed.");
        }
    }

    if dry_run {
        ui::warning("Dry run enabled: no changes will be made.");
        ui::info(format!("Would add bond with source: {}", source.display()));
        if let Some(ref t) = target {
            ui::info(format!("Would use target: {}", t.display()));
        } else {
            ui::info("Would use default configuration or current directory as target.");
        }
        if let Some(ref n) = name {
            ui::info(format!("Would name bond: {}", n));
        } else {
            ui::info("Bond would be unnamed.");
        }
        return Ok(());
    }

    let source = source.canonicalize().map_err(|_| {
        BondError::InvalidPath(format!("cannot resolve source: {}", source.display()))
    })?;

    let target = match target {
        Some(t) => t,
        None => {
            let config = bonds_core::BondsConfig::load().unwrap_or_default();
            let name = source
                .file_name()
                .ok_or_else(|| BondError::InvalidPath("source has no file name".into()))?;
            match config.default_target {
                Some(default_dir) => default_dir.join(name),
                None => std::env::current_dir()?.join(name),
            }
        }
    };

    let bond = manager.create_bond(&source, &target, name)?;
    ui::status_ok("✓ Bond created:");
    ui::subheading(format!("   {}", bond.id()));
    ui::newline();
    ui::dim(format!("   {}", bond.source().display()));
    ui::normal("    ⤡ ");
    ui::dim(format!("    {}\n", bond.target().display()));
    Ok(())
}
