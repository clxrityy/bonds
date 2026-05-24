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
