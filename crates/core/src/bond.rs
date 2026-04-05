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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_unique_ids() {
        let a = Bond::new(PathBuf::from("/a"), PathBuf::from("/b"));
        let b = Bond::new(PathBuf::from("/a"), PathBuf::from("/b"));
        assert_ne!(a.id, b.id); // UUID v4 should never collide
    }

    #[test]
    fn created_at_rfc3339_roundtrips() {
        let bond = Bond::new(PathBuf::from("/a"), PathBuf::from("/b"));
        let rfc = bond.created_at_rfc3339();
        // Verify it parses back cleanly
        let parsed = DateTime::parse_from_rfc3339(&rfc).unwrap();
        assert_eq!(parsed.with_timezone(&Utc), bond.created_at);
    }

    #[test]
    fn serializes_to_json() {
        let bond = Bond::new(PathBuf::from("/src"), PathBuf::from("/tgt"));
        let json = serde_json::to_string(&bond).unwrap();
        let deserialized: Bond = serde_json::from_str(&json).unwrap();
        assert_eq!(bond, deserialized);
    }
}
