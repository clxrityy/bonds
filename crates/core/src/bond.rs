use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Representation of a bond (source -> target)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bond {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) source: PathBuf,
    pub(crate) target: PathBuf,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) metadata: Option<HashMap<String, String>>,
}

impl Bond {
    /// Create a new Bond with a UUID and current timestamp.
    pub fn new(source: PathBuf, target: PathBuf, name: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
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

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn source(&self) -> &Path {
        &self.source
    }
    pub fn target(&self) -> &Path {
        &self.target
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn metadata(&self) -> Option<&HashMap<String, String>> {
        self.metadata.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_unique_ids() {
        let a = Bond::new(
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            Some("bond_a".to_string()),
        );
        let b = Bond::new(
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            Some("bond_b".to_string()),
        );
        assert_ne!(a.id, b.id); // UUID v4 should never collide
    }

    #[test]
    fn created_at_rfc3339_roundtrips() {
        let bond = Bond::new(
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            Some("bond".to_string()),
        );
        let rfc = bond.created_at_rfc3339();
        // Verify it parses back cleanly
        let parsed = DateTime::parse_from_rfc3339(&rfc).unwrap();
        assert_eq!(parsed.with_timezone(&Utc), bond.created_at);
    }

    #[test]
    fn serializes_to_json() {
        let bond = Bond::new(
            PathBuf::from("/src"),
            PathBuf::from("/tgt"),
            Some("bond".to_string()),
        );
        let json = serde_json::to_string(&bond).unwrap();
        let deserialized: Bond = serde_json::from_str(&json).unwrap();
        assert_eq!(bond, deserialized);
    }
}
