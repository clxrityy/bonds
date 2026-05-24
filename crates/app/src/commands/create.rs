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

#[cfg(test)]
mod tests {
    use super::create_bond_item;
    use crate::requests::CreateBondRequest;
    use bonds_core::{BondError, BondManager};
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    #[cfg_attr(windows, ignore)]
    fn create_bond_item_creates_record_with_explicit_target() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("create-explicit.db");

        let src = TempDir::new().expect("src");
        let tgt_root = TempDir::new().expect("target root");
        let target = tgt_root.path().join("viewer-link");

        let req = CreateBondRequest {
            source: src.path().display().to_string(),
            target: Some(target.display().to_string()),
            name: Some("viewer".into()),
        };

        let created = create_bond_item(req, Some(db_path.clone())).expect("create");
        assert_eq!(created.name.as_deref(), Some("viewer"));
        assert_eq!(PathBuf::from(&created.target), target);

        let manager = BondManager::new(Some(db_path)).expect("manager");
        let bonds = manager.list_bonds().expect("list");
        assert_eq!(bonds.len(), 1);
    }

    #[test]
    fn create_bond_item_rejects_missing_source_path() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("create-missing-source.db");
        let missing_source = db_dir.path().join("does-not-exist");

        let req = CreateBondRequest {
            source: missing_source.display().to_string(),
            target: None,
            name: Some("bad".into()),
        };

        let err = create_bond_item(req, Some(db_path)).expect_err("expected invalid source");
        assert!(
            matches!(err, BondError::InvalidPath(msg) if msg.contains("cannot resolve source"))
        );
    }
}
