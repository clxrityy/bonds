use crate::bond_view::{map_bond_detail, BondDetailItem};
use crate::requests::GetBondDetailRequest;
use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

fn get_bond_detail_item(
    request: GetBondDetailRequest,
    db_path: Option<PathBuf>,
) -> Result<BondDetailItem, BondError> {
    let manager = BondManager::new(db_path)?;
    let bond = manager.get_bond(&request.id)?;
    Ok(map_bond_detail(bond))
}

#[tauri::command]
pub fn get_bond_detail(
    request: GetBondDetailRequest,
    db_path: Option<String>,
) -> Result<BondDetailItem, String> {
    get_bond_detail_item(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::get_bond_detail_item;
    use crate::requests::GetBondDetailRequest;
    use bonds_core::BondManager;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    #[cfg_attr(windows, ignore)]
    fn get_bond_detail_item_includes_metadata() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("detail-success.db");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_root = TempDir::new().expect("target root");
        let tgt = tgt_root.path().join("viewer-link");

        let mut metadata = HashMap::new();
        metadata.insert("project".to_string(), "bonds".to_string());

        let created = manager
            .create_bond_with_metadata(
                src.path(),
                &tgt,
                Some("viewer".into()),
                Some(metadata),
            )
            .expect("seed create");

        let req = GetBondDetailRequest {
            id: created.id().to_string(),
        };

        let item = get_bond_detail_item(req, Some(db_path)).expect("detail");
        assert_eq!(item.id, created.id());
        assert_eq!(item.name.as_deref(), Some("viewer"));
        assert_eq!(
            item.metadata
                .as_ref()
                .and_then(|m| m.get("project"))
                .map(String::as_str),
            Some("bonds")
        );
    }
}
