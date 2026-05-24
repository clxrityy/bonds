use crate::bond_view::{BondListItem, map_bond};
use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

fn load_bonds(db_path: Option<PathBuf>) -> Result<Vec<BondListItem>, BondError> {
    let manager = BondManager::new(db_path)?;
    let bonds = manager.list_bonds()?;
    Ok(bonds.into_iter().map(map_bond).collect())
}

#[tauri::command]
pub fn list_bonds(db_path: Option<String>) -> Result<Vec<BondListItem>, String> {
    load_bonds(db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

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
