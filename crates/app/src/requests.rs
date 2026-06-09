use serde::Deserialize;
use std::collections::HashMap;

/// Payload for `create_bond` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateBondRequest {
    pub(crate) source: String,
    pub(crate) target: Option<String>,
    pub(crate) name: Option<String>,
}

/// Payload for `update_bond` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateBondRequest {
    pub(crate) id: String,
    pub(crate) source: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) name: Option<String>,
}

/// Payload for `delete_bond` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteBondRequest {
    pub(crate) id: String,
    pub(crate) with_target: bool,
}

/// Payload for `get_bond_detail` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetBondDetailRequest {
    pub(crate) id: String,
}

/// Payload for `update_bond_metadata` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateBondMetadataRequest {
    pub(crate) id: String,
    pub(crate) metadata: Option<HashMap<String, String>>,
}

/// Payload for history/snapshot commands that only need a bond identifier.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BondHistoryRequest {
    pub(crate) id: String,
}

/// Payload for restore command.
/// `snapshot_id` is mapped from `snapshotId` from the frontend via `rename_all`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RestoreSnapshotRequest {
    pub(crate) id: String,
    pub(crate) snapshot_id: String,
}

/// Payload for deleting a snapshot for a bond.
/// `snapshot_id` is mapped from `snapshotId` via `rename_all = "camelCase"`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteSnapshotRequest {
    pub(crate) id: String,
    pub(crate) snapshot_id: String,
}
