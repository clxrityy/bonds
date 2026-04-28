use bonds_cli::ui;
use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

pub fn cmd_update(
    manager: &BondManager,
    id: &str,
    source: Option<PathBuf>,
    target: Option<PathBuf>,
    name: Option<String>,
) -> Result<(), BondError> {
    if source.is_none() && target.is_none() {
        return Err(BondError::InvalidPath(
            "provide at least one of --source or --target".into(),
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

    let bond = manager.update_bond(id, source, target, name)?;
    ui::success(format!("Bond updated: {}", &bond.id()[..8]));
    ui::info(format!(
        "  {} -> {}",
        bond.source().display(),
        bond.target().display()
    ));
    Ok(())
}
