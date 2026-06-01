use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotPolicy {
    pub bond_id: String,
    pub enabled: bool,
    pub interval_seconds: i64,
    pub keep_last: i64,
    pub storage_root: Option<PathBuf>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub last_error_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotRecord {
    pub id: String,
    pub bond_id: String,
    pub bond_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub storage_path: PathBuf,
    pub payload_path: PathBuf,
    pub manifest_path: PathBuf,
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub metadata: Option<HashMap<String, String>>,
    pub file_count: i64,
    pub bytes_total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotManifest {
    pub snapshot_id: String,
    pub bond_id: String,
    pub bond_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub payload_path: PathBuf,
    pub metadata: Option<HashMap<String, String>>,
    pub file_count: i64,
    pub bytes_total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestoreRecord {
    pub id: String,
    pub bond_id: String,
    pub snapshot_id: String,
    pub safety_snapshot_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub status: String,
    pub notes: Option<String>,
}

pub fn default_history_root() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".bonds")
        .join("history")
}
