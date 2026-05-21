use bonds_core::{Bond, BondError, BondManager, BondsConfig};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BondListItem {
    id: String,
    name: Option<String>,
    source: String,
    target: String,
    created_at: String,
    status: String,
    metadata_count: usize,
}

fn load_bonds(db_path: Option<PathBuf>) -> Result<Vec<BondListItem>, BondError> {
    let manager = BondManager::new(db_path)?;
    let bonds = manager.list_bonds()?;
    Ok(bonds.into_iter().map(map_bond).collect())
}

fn map_bond(bond: Bond) -> BondListItem {
    BondListItem {
        id: bond.id().to_string(),
        name: bond.name().map(str::to_owned),
        source: bond.source().display().to_string(),
        target: bond.target().display().to_string(),
        created_at: bond.created_at_rfc3339(),
        status: bond_status(&bond),
        metadata_count: bond.metadata().map_or(0, |m| m.len()),
    }
}

fn bond_status(bond: &Bond) -> String {
    let target_exists = bond.target().exists();
    let is_symlink = bond
        .target()
        .symlink_metadata()
        .map(|meta| meta.file_type().is_symlink())
        .unwrap_or(false);

    match (target_exists, is_symlink) {
        (true, true) => "healthy".to_string(),
        (true, false) => "warning".to_string(),
        (false, _) => "broken".to_string(),
    }
}

#[tauri::command]
fn list_bonds(db_path: Option<String>) -> Result<Vec<BondListItem>, String> {
    load_bonds(db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_bonds, create_bond])
        .run(tauri::generate_context!())
        .expect("failed to run Bonds desktop app");
}

// TODO:: Modularize

// adding a bond
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBondRequest {
    source: String,
    target: Option<String>,
    name: Option<String>,
}

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
fn create_bond(
    request: CreateBondRequest,
    db_path: Option<String>,
) -> Result<BondListItem, String> {
    create_bond_item(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

// TESTS
#[cfg(test)]
mod tests {
    use super::load_bonds;
    use bonds_core::BondManager;
    use tempfile::TempDir;

    #[test]
    #[cfg_attr(windows, ignore)]
    fn load_bonds_reads_from_supplied_db() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("bonds-app-test.db");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_dir = TempDir::new().expect("target dir");
        let tgt = tgt_dir.path().join("viewer-link");

        let created = manager
            .create_bond(src.path(), &tgt, Some("viewer".into()))
            .expect("create bond");

        drop(manager);

        let items = load_bonds(Some(db_path)).expect("load bonds");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, created.id());
        assert_eq!(items[0].name.as_deref(), Some("viewer"));
    }
}
