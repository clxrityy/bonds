use crate::bond_view::{BondListItem, map_bond};
use crate::requests::UpdateBondRequest;
use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

fn non_empty(input: Option<String>) -> Option<String> {
    input.and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn resolve_update_source(source: Option<String>) -> Result<Option<PathBuf>, BondError> {
    match non_empty(source) {
        Some(raw) => {
            let path = PathBuf::from(raw);
            let canonical = path.canonicalize().map_err(|_| {
                BondError::InvalidPath(format!("cannot resolve source: {}", path.display()))
            })?;
            Ok(Some(canonical))
        }
        None => Ok(None),
    }
}

fn resolve_update_target(target: Option<String>) -> Result<Option<PathBuf>, BondError> {
    match non_empty(target) {
        Some(raw) => {
            let path = PathBuf::from(raw);
            // Target may not exist yet, so make absolute instead of canonicalize.
            let absolute = std::path::absolute(&path).map_err(|_| {
                BondError::InvalidPath(format!("cannot resolve target: {}", path.display()))
            })?;
            Ok(Some(absolute))
        }
        None => Ok(None),
    }
}

fn update_bond_item(
    request: UpdateBondRequest,
    db_path: Option<PathBuf>,
) -> Result<BondListItem, BondError> {
    let manager = BondManager::new(db_path)?;

    let source = resolve_update_source(request.source)?;
    let target = resolve_update_target(request.target)?;
    let name = non_empty(request.name);

    // Keep CLI parity: require at least one mutation field.
    if source.is_none() && target.is_none() && name.is_none() {
        return Err(BondError::InvalidPath(
            "provide at least one of --source, --target, or --name".into(),
        ));
    }

    let updated = manager.update_bond(&request.id, source, target, name)?;
    Ok(map_bond(updated))
}

#[tauri::command]
pub fn update_bond(
    request: UpdateBondRequest,
    db_path: Option<String>,
) -> Result<BondListItem, String> {
    update_bond_item(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::update_bond_item;
    use crate::requests::UpdateBondRequest;
    use bonds_core::{BondError, BondManager};
    use tempfile::TempDir;

    #[test]
    fn update_bond_item_requires_at_least_one_field() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("update-empty.db");

        let req = UpdateBondRequest {
            id: "any".into(),
            source: None,
            target: None,
            name: None,
        };

        let err = update_bond_item(req, Some(db_path)).expect_err("expected validation error");
        assert!(matches!(err, BondError::InvalidPath(msg) if msg.contains("provide at least one")));
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn update_bond_item_renames_bond() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("update-rename.db");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_root = TempDir::new().expect("target root");
        let tgt = tgt_root.path().join("viewer-link");

        let created = manager
            .create_bond(src.path(), &tgt, Some("viewer".into()))
            .expect("seed create");
        drop(manager);

        let req = UpdateBondRequest {
            id: created.id().to_string(),
            source: None,
            target: None,
            name: Some("renamed".into()),
        };

        let updated = update_bond_item(req, Some(db_path.clone())).expect("update");
        assert_eq!(updated.name.as_deref(), Some("renamed"));

        let manager = BondManager::new(Some(db_path)).expect("manager");
        let fetched = manager.get_bond(created.id()).expect("fetch");
        assert_eq!(fetched.name(), Some("renamed"));
    }

    #[test]
    fn update_bond_item_rejects_unresolvable_source() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("update-bad-source.db");
        let missing_source = db_dir.path().join("missing-source");

        let req = UpdateBondRequest {
            id: "any".into(),
            source: Some(missing_source.display().to_string()),
            target: None,
            name: Some("rename".into()),
        };

        let err = update_bond_item(req, Some(db_path)).expect_err("expected invalid source");
        assert!(
            matches!(err, BondError::InvalidPath(msg) if msg.contains("cannot resolve source"))
        );
    }
}
