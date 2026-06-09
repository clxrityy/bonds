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
    use std::collections::HashMap;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    #[cfg_attr(windows, ignore)]
    fn load_bonds_includes_metadata_for_filtering() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("bonds-app-test-metadata.db");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_dir = TempDir::new().expect("target dir");
        let tgt = tgt_dir.path().join("meta-link");

        let mut metadata = HashMap::new();
        metadata.insert("env".to_string(), "dev".to_string());

        manager
            .create_bond_with_metadata(src.path(), &tgt, Some("meta-case".into()), Some(metadata))
            .expect("create bond with metadata");

        drop(manager);

        let items = load_bonds(Some(db_path)).expect("load bonds");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].metadata_count, 1);
        assert_eq!(
            items[0]
                .metadata
                .as_ref()
                .and_then(|m| m.get("env"))
                .map(String::as_str),
            Some("dev")
        );
    }

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
        assert_eq!(items[0].status, "healthy");
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn load_bonds_marks_missing_target_as_broken() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("bonds-app-test-broken.db");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_dir = TempDir::new().expect("target dir");
        let tgt = tgt_dir.path().join("broken-link");

        manager
            .create_bond(src.path(), &tgt, Some("broken-case".into()))
            .expect("create bond");

        drop(manager);

        // Simulate a broken bond by removing the symlink target path itself.
        fs::remove_file(&tgt).expect("remove symlink target");
        assert!(!tgt.exists());

        let items = load_bonds(Some(db_path)).expect("load bonds");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].status, "broken");
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn load_bonds_marks_non_symlink_target_as_warning() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("bonds-app-test-warning.db");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_dir = TempDir::new().expect("target dir");
        let tgt = tgt_dir.path().join("warning-link");

        manager
            .create_bond(src.path(), &tgt, Some("warning-case".into()))
            .expect("create bond");

        drop(manager);

        // Replace symlink with a plain directory at the same path.
        fs::remove_file(&tgt).expect("remove symlink target");
        fs::create_dir_all(&tgt).expect("create plain directory target");

        let meta = fs::symlink_metadata(&tgt).expect("target metadata");
        assert!(!meta.file_type().is_symlink());

        let items = load_bonds(Some(db_path)).expect("load bonds");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].status, "warning");
    }
}
