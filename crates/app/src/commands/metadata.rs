use crate::bond_view::{BondDetailItem, map_bond_detail};
use crate::requests::UpdateBondMetadataRequest;
use bonds_core::{BondError, BondManager};
use std::collections::HashMap;
use std::path::PathBuf;

/// Normalize empty metadata map to `None` to match core/CLI semantics.
fn normalize_metadata(
    metadata: Option<HashMap<String, String>>,
) -> Option<HashMap<String, String>> {
    match metadata {
        Some(map) if map.is_empty() => None,
        other => other,
    }
}

fn update_bond_metadata_item(
    request: UpdateBondMetadataRequest,
    db_path: Option<PathBuf>,
) -> Result<BondDetailItem, BondError> {
    let manager = BondManager::new(db_path)?;
    let normalized = normalize_metadata(request.metadata);
    let updated = manager.update_bond_metadata(&request.id, normalized)?;
    Ok(map_bond_detail(updated))
}

#[tauri::command]
pub fn update_bond_metadata(
    request: UpdateBondMetadataRequest,
    db_path: Option<String>,
) -> Result<BondDetailItem, String> {
    update_bond_metadata_item(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::update_bond_metadata_item;
    use crate::requests::UpdateBondMetadataRequest;
    use bonds_core::BondManager;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    #[cfg_attr(windows, ignore)]
    fn update_bond_metadata_item_sets_and_clears_metadata() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("metadata-success.db");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_root = TempDir::new().expect("target root");
        let tgt = tgt_root.path().join("viewer-link");

        let created = manager
            .create_bond(src.path(), &tgt, Some("viewer".into()))
            .expect("seed create");
        drop(manager);

        let mut metadata = HashMap::new();
        metadata.insert("env".to_string(), "dev".to_string());

        let set_req = UpdateBondMetadataRequest {
            id: created.id().to_string(),
            metadata: Some(metadata.clone()),
        };

        let updated =
            update_bond_metadata_item(set_req, Some(db_path.clone())).expect("set metadata");
        assert_eq!(updated.metadata.as_ref(), Some(&metadata));

        // Sending an empty map should clear metadata (store NULL).
        let clear_req = UpdateBondMetadataRequest {
            id: created.id().to_string(),
            metadata: Some(HashMap::<String, String>::new()),
        };

        let cleared = update_bond_metadata_item(clear_req, Some(db_path)).expect("clear metadata");
        assert!(cleared.metadata.is_none());
    }
}
