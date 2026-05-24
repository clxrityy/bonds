use crate::bond_view::{BondListItem, map_bond};
use crate::requests::DeleteBondRequest;
use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

fn delete_bond_item(
    request: DeleteBondRequest,
    db_path: Option<PathBuf>,
) -> Result<BondListItem, BondError> {
    let manager = BondManager::new(db_path)?;
    let deleted = manager.delete_bond(&request.id, request.with_target)?;
    Ok(map_bond(deleted))
}

#[tauri::command]
pub fn delete_bond(
    request: DeleteBondRequest,
    db_path: Option<String>,
) -> Result<BondListItem, String> {
    delete_bond_item(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::delete_bond_item;
    use crate::requests::DeleteBondRequest;
    use bonds_core::{BondError, BondManager};
    use tempfile::TempDir;

    #[test]
    #[cfg_attr(windows, ignore)]
    fn delete_bond_item_removes_bond_record() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("delete-success.db");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_root = TempDir::new().expect("target root");
        let tgt = tgt_root.path().join("viewer-link");

        let created = manager
            .create_bond(src.path(), &tgt, Some("viewer".into()))
            .expect("seed create");
        drop(manager);

        let req = DeleteBondRequest {
            id: created.id().to_string(),
            with_target: false,
        };

        let deleted = delete_bond_item(req, Some(db_path.clone())).expect("delete");
        assert_eq!(deleted.id, created.id());

        let manager = BondManager::new(Some(db_path)).expect("manager");
        let bonds = manager.list_bonds().expect("list");
        assert!(bonds.is_empty());
    }

    #[test]
    fn delete_bond_item_errors_for_unknown_identifier() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("delete-missing.db");

        let req = DeleteBondRequest {
            id: "missing".into(),
            with_target: false,
        };

        let err = delete_bond_item(req, Some(db_path)).expect_err("expected not found");
        assert!(matches!(err, BondError::NotFound(_)));
    }
}
