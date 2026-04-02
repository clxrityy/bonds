use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Representation of a bond (source -> target)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bond {
    pub id: String,
    pub source: PathBuf,
    pub target: PathBuf,
    pub created_at: DateTime<Utc>,
    pub metadata: Option<HashMap<String, String>>,
}

impl Bond {
    /// Create a new Bond with a UUID and current timestamp.
    pub fn new(source: PathBuf, target: PathBuf) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source,
            target,
            created_at: Utc::now(),
            metadata: None,
        }
    }

    /// Helper to serialize `created_at` for DB storage.
    pub fn created_at_rfc3339(&self) -> String {
        self.created_at.to_rfc3339()
    }
}
