use crate::bond_view::{RestoreResultItem, SnapshotItem, map_restore_result, map_snapshot};
use crate::requests::{BondHistoryRequest, RestoreSnapshotRequest};
use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

fn list_bond_snapshots_items(
    request: BondHistoryRequest,
    db_path: Option<PathBuf>,
) -> Result<Vec<SnapshotItem>, BondError> {
    let manager = BondManager::new(db_path)?;
    let snapshots = manager.list_snapshots(&request.id)?;
    Ok(snapshots.into_iter().map(map_snapshot).collect())
}

fn create_bond_snapshot_item(
    request: BondHistoryRequest,
    db_path: Option<PathBuf>,
) -> Result<SnapshotItem, BondError> {
    let manager = BondManager::new(db_path)?;
    let created = manager.create_snapshot(&request.id)?;

    // Keep policy behavior consistent with CLI history snapshot command.
    // If a policy exists, this removes old snapshots beyond keep_last.
    let _pruned = manager.prune_snapshots(&request.id)?;

    Ok(map_snapshot(created))
}

fn restore_bond_snapshot_item(
    request: RestoreSnapshotRequest,
    db_path: Option<PathBuf>,
) -> Result<RestoreResultItem, BondError> {
    let manager = BondManager::new(db_path)?;
    let restored = manager.restore_snapshot(&request.id, &request.snapshot_id)?;
    Ok(map_restore_result(restored))
}

#[tauri::command]
pub fn list_bond_snapshots(
    request: BondHistoryRequest,
    db_path: Option<String>,
) -> Result<Vec<SnapshotItem>, String> {
    list_bond_snapshots_items(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn create_bond_snapshot(
    request: BondHistoryRequest,
    db_path: Option<String>,
) -> Result<SnapshotItem, String> {
    create_bond_snapshot_item(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn restore_bond_snapshot(
    request: RestoreSnapshotRequest,
    db_path: Option<String>,
) -> Result<RestoreResultItem, String> {
    restore_bond_snapshot_item(request, db_path.map(PathBuf::from)).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::{create_bond_snapshot_item, list_bond_snapshots_items, restore_bond_snapshot_item};
    use crate::requests::{BondHistoryRequest, RestoreSnapshotRequest};
    use bonds_core::BondManager;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    #[cfg_attr(windows, ignore)]
    fn list_bond_snapshots_reads_snapshots_for_supplied_db() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("history-list.db");
        let history_dir = TempDir::new().expect("history dir");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_root = TempDir::new().expect("target root");
        let tgt = tgt_root.path().join("viewer-link");

        let created = manager
            .create_bond(src.path(), &tgt, Some("viewer".into()))
            .expect("create bond");

        manager
            .set_snapshot_policy(created.id(), 60, 10, Some(history_dir.path().join("history")))
            .expect("set policy");

        let seed = src.path().join("state.txt");
        fs::write(&seed, "alpha").expect("seed source");
        let seeded_snapshot = manager.create_snapshot(created.id()).expect("seed snapshot");
        drop(manager);

        let items = list_bond_snapshots_items(
            BondHistoryRequest {
                id: created.id().to_string(),
            },
            Some(db_path),
        )
        .expect("list snapshots");

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, seeded_snapshot.id);
        assert_eq!(items[0].file_count, 1);
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn create_bond_snapshot_creates_snapshot_item() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("history-create.db");
        let history_dir = TempDir::new().expect("history dir");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_root = TempDir::new().expect("target root");
        let tgt = tgt_root.path().join("viewer-link");

        let created = manager
            .create_bond(src.path(), &tgt, Some("viewer".into()))
            .expect("create bond");

        manager
            .set_snapshot_policy(created.id(), 60, 10, Some(history_dir.path().join("history")))
            .expect("set policy");

        fs::write(src.path().join("notes.txt"), "hello").expect("write source file");
        drop(manager);

        let snapshot = create_bond_snapshot_item(
            BondHistoryRequest {
                id: created.id().to_string(),
            },
            Some(db_path),
        )
        .expect("create snapshot");

        assert_eq!(snapshot.bond_id, created.id());
        assert!(snapshot.file_count >= 1);
        assert!(snapshot.storage_path.contains(created.id()));
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn restore_bond_snapshot_returns_restore_result_with_safety_snapshot() {
        let db_dir = TempDir::new().expect("temp db dir");
        let db_path = db_dir.path().join("history-restore.db");
        let history_dir = TempDir::new().expect("history dir");

        let manager = BondManager::new(Some(db_path.clone())).expect("manager");
        let src = TempDir::new().expect("src");
        let tgt_root = TempDir::new().expect("target root");
        let tgt = tgt_root.path().join("viewer-link");

        let created = manager
            .create_bond(src.path(), &tgt, Some("viewer".into()))
            .expect("create bond");

        manager
            .set_snapshot_policy(created.id(), 60, 10, Some(history_dir.path().join("history")))
            .expect("set policy");

        let live_file = src.path().join("state.txt");

        fs::write(&live_file, "before").expect("write before");
        let original = manager.create_snapshot(created.id()).expect("snapshot before");

        fs::write(&live_file, "after").expect("write after");
        drop(manager);

        let restored = restore_bond_snapshot_item(
            RestoreSnapshotRequest {
                id: created.id().to_string(),
                snapshot_id: original.id.clone(),
            },
            Some(db_path),
        )
        .expect("restore snapshot");

        assert_eq!(restored.snapshot_id, original.id);
        assert!(restored.safety_snapshot_id.is_some());
        assert_eq!(
            fs::read_to_string(&live_file).expect("read restored file"),
            "before"
        );
    }
}
