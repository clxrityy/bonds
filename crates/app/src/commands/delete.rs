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
