use bonds_core::{Bond, RestoreRecord, SnapshotRecord};
use serde::Serialize;
use std::collections::HashMap;

/// UI-facing lightweight payload for list/grid views.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BondListItem {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) created_at: String,
    pub(crate) status: String,
    pub(crate) metadata_count: usize,
    // Added so frontend can perform metadata-aware filtering without fetching detail per row.
    pub(crate) metadata: Option<HashMap<String, String>>,
}


/// UI-facing detail payload for edit/detail views.
/// Includes full metadata map for editing.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BondDetailItem {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) created_at: String,
    pub(crate) status: String,
    pub(crate) metadata: Option<HashMap<String, String>>,
}

/// UI-facing snapshot payload for history browsing in the desktop app.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SnapshotItem {
    pub(crate) id: String,
    pub(crate) bond_id: String,
    pub(crate) bond_name: Option<String>,
    pub(crate) created_at: String,
    pub(crate) source_path: String,
    pub(crate) target_path: String,
    pub(crate) storage_path: String,
    pub(crate) file_count: i64,
    pub(crate) bytes_total: i64,
    pub(crate) metadata_count: usize,
}

/// UI-facing restore payload returned after a restore operation completes.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RestoreResultItem {
    pub(crate) id: String,
    pub(crate) bond_id: String,
    pub(crate) snapshot_id: String,
    pub(crate) safety_snapshot_id: Option<String>,
    pub(crate) created_at: String,
    pub(crate) status: String,
    pub(crate) notes: Option<String>,
}

/// Map a core `Bond` into a lightweight list item.
pub(crate) fn map_bond(bond: Bond) -> BondListItem {
    BondListItem {
        id: bond.id().to_string(),
        name: bond.name().map(str::to_owned),
        source: bond.source().display().to_string(),
        target: bond.target().display().to_string(),
        created_at: bond.created_at_rfc3339(),
        status: bond_status(&bond),
        metadata_count: bond.metadata().map_or(0, |m| m.len()),
        metadata: bond.metadata().cloned(),
    }
}

/// Map a core `Bond` into a full detail item.
pub(crate) fn map_bond_detail(bond: Bond) -> BondDetailItem {
    BondDetailItem {
        id: bond.id().to_string(),
        name: bond.name().map(str::to_owned),
        source: bond.source().display().to_string(),
        target: bond.target().display().to_string(),
        created_at: bond.created_at_rfc3339(),
        status: bond_status(&bond),
        // Clone so we can serialize and return ownership to the UI.
        metadata: bond.metadata().cloned(),
    }
}

/// Map a core `SnapshotRecord` into a UI-safe payload with string paths/timestamps.
pub(crate) fn map_snapshot(snapshot: SnapshotRecord) -> SnapshotItem {
    SnapshotItem {
        id: snapshot.id,
        bond_id: snapshot.bond_id,
        bond_name: snapshot.bond_name,
        created_at: snapshot.created_at.to_rfc3339(),
        source_path: snapshot.source_path.display().to_string(),
        target_path: snapshot.target_path.display().to_string(),
        storage_path: snapshot.storage_path.display().to_string(),
        file_count: snapshot.file_count,
        bytes_total: snapshot.bytes_total,
        metadata_count: snapshot.metadata.as_ref().map_or(0, |m| m.len()),
    }
}

/// Map a core `RestoreRecord` into the UI result object.
pub(crate) fn map_restore_result(restore: RestoreRecord) -> RestoreResultItem {
    RestoreResultItem {
        id: restore.id,
        bond_id: restore.bond_id,
        snapshot_id: restore.snapshot_id,
        safety_snapshot_id: restore.safety_snapshot_id,
        created_at: restore.created_at.to_rfc3339(),
        status: restore.status,
        notes: restore.notes,
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
