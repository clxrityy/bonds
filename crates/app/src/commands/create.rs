use crate::bond_view::{BondListItem, map_bond};
use crate::requests::CreateBondRequest;
use bonds_core::{BondError, BondManager, BondsConfig};
use std::path::{Path, PathBuf};

/// Resolve target like the CLI:
/// - explicit target wins
/// - otherwise use config default target, then source leaf name
/// - fallback to current directory if no default target is configured
fn resolve_create_target(source: &Path, target: Option<String>) -> Result<PathBuf, BondError> {
    if let Some(raw_target) = target {
        let candidate = PathBuf::from(raw_target);

        // Target may not exist yet, so canonicalize is not appropriate here.
        return if candidate.is_absolute() {
            Ok(candidate)
        } else {
            Ok(std::env::current_dir()?.join(candidate))
        };
    }

    // CLI currently tolerates config-read failure with default config.
    // Keeping same behavior preserves parity.
    let config = BondsConfig::load().unwrap_or_default();

    let source_leaf = source
        .file_name()
        .ok_or_else(|| BondError::InvalidPath("source has no file name".into()))?;

    Ok(match config.default_target {
        Some(default_dir) => default_dir.join(source_leaf),
        None => std::env::current_dir()?.join(source_leaf),
    })
}

fn create_bond_item(
    request: CreateBondRequest,
    db_path: Option<PathBuf>,
) -> Result<BondListItem, BondError> {
    let manager = BondManager::new(db_path)?;

    let source_input = PathBuf::from(request.source);
    let source = source_input.canonicalize().map_err(|_| {
        BondError::InvalidPath(format!("cannot resolve source: {}", source_input.display()))
    })?;

    let target = resolve_create_target(&source, request.target)?;
    let created = manager.create_bond(&source, &target, request.name)?;

    Ok(map_bond(created))
}

#[tauri::command]
pub fn create_bond(
    request: CreateBondRequest,
    db_path: Option<String>,
) -> Result<BondListItem, String> {
    create_bond_item(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}
