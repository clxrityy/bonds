use crate::bond::Bond;
use crate::error::BondError;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf}; // Metadata map type for API methods

/// SQLite-backed manager for Bonds.
pub struct BondManager {
    conn: Connection,
}

impl BondManager {
    /// Open (or create) the DB at `db_path`. If None, defaults to `$HOME/.bonds/bonds.db`.
    pub fn new(db_path: Option<PathBuf>) -> Result<Self, BondError> {
        let db_path = db_path.unwrap_or_else(|| {
            std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(".bonds")
                .join("bonds.db")
        });

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(db_path)?;
        Self::from_connection(conn) // ← reuse the schema setup
    }

    /// List all bonds (most-recent first).
    pub fn list_bonds(&self) -> Result<Vec<Bond>, BondError> {
        let mut stmt = self.conn.prepare(
        "SELECT id, name, source, target, created_at, metadata FROM bonds ORDER BY created_at DESC",
    )?;
        let mut rows = stmt.query([])?;

        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(self.bond_from_row(row)?);
        }
        Ok(out)
    }

    /// Parse a Bond from a rusqlite Row.
    fn bond_from_row(&self, row: &rusqlite::Row) -> Result<Bond, BondError> {
        let id: String = row.get(0)?;
        let name: Option<String> = row.get(1)?;
        let source: String = row.get(2)?;
        let target: String = row.get(3)?;
        let created_at_str: String = row.get(4)?;
        let metadata_json: Option<String> = row.get(5)?;

        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| BondError::InvalidTimestamp(e.to_string()))?;

        let metadata = match metadata_json {
            Some(s) => Some(serde_json::from_str(&s)?),
            None => None,
        };

        Ok(Bond {
            id,
            name,
            source: PathBuf::from(source),
            target: PathBuf::from(target),
            created_at,
            metadata,
        })
    }

    /// Get a single bond by ID or name. ID can be a unique prefix.
    pub fn get_bond(&self, identifier: &str) -> Result<Bond, BondError> {
        // 1. Try exact name match
        let mut stmt = self.conn.prepare(
            "SELECT id, name, source, target, created_at, metadata FROM bonds WHERE name = ?1",
        )?;
        let mut rows = stmt.query(params![identifier])?;

        if let Some(row) = rows.next()? {
            return self.bond_from_row(row);
        }
        drop(rows);
        drop(stmt);

        // 2. Fall back to ID prefix match
        let mut stmt = self.conn.prepare(
        "SELECT id, name, source, target, created_at, metadata FROM bonds WHERE id LIKE ?1 || '%'",
    )?;
        let mut rows = stmt.query(params![identifier])?;

        let first = match rows.next()? {
            Some(row) => self.bond_from_row(row)?,
            None => return Err(BondError::NotFound(identifier.to_string())),
        };

        if rows.next()?.is_some() {
            return Err(BondError::AmbiguousId(identifier.to_string()));
        }

        Ok(first)
    }

    /// Create a symlink bond and persist it (no metadata).
    pub fn create_bond<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        target: Q,
        name: Option<String>,
    ) -> Result<Bond, BondError> {
        // Keep legacy signature stable for existing CLI/API callers.
        self.create_bond_internal(source, target, name, None)
    }

    /// Create a symlink bond with metadata and persist it.
    pub fn create_bond_with_metadata<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        target: Q,
        name: Option<String>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Bond, BondError> {
        // New API path: lets library users persist metadata at creation time.
        self.create_bond_internal(source, target, name, metadata)
    }

    /// Shared implementation used by both create methods.
    fn create_bond_internal<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        target: Q,
        name: Option<String>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Bond, BondError> {
        let src = source.as_ref().to_path_buf();
        let tgt = target.as_ref().to_path_buf();

        // Validate name uniqueness if provided.
        if let Some(ref n) = name {
            let mut stmt = self
                .conn
                .prepare("SELECT COUNT(*) FROM bonds WHERE name = ?1")?;
            let count: i64 = stmt.query_row(params![n], |row| row.get(0))?;
            if count > 0 {
                return Err(BondError::AlreadyExists);
            }
        }

        if !src.exists() {
            return Err(BondError::InvalidPath(format!(
                "source does not exist: {:?}",
                src
            )));
        }

        if tgt.exists() {
            // Allow replacing an empty directory at target path.
            let is_empty_dir = tgt.is_dir()
                && std::fs::read_dir(&tgt)
                    .map(|mut d| d.next().is_none())
                    .unwrap_or(false);

            if !is_empty_dir {
                return Err(BondError::TargetExists(format!("{}", tgt.display())));
            }

            std::fs::remove_dir(&tgt)?;
        }

        if let Some(parent) = tgt.parent() {
            fs::create_dir_all(parent)?;
        }

        #[cfg(unix)]
        std::os::unix::fs::symlink(&src, &tgt)?;
        #[cfg(windows)]
        {
            if src.is_dir() {
                std::os::windows::fs::symlink_dir(&src, &tgt)?;
            } else {
                std::os::windows::fs::symlink_file(&src, &tgt)?;
            }
        }

        // Keep returned Bond and DB row in sync.
        let mut bond = Bond::new(src.clone(), tgt.clone(), name);
        bond.metadata = metadata;

        // Store metadata as JSON in SQLite TEXT column.
        let metadata_json: Option<String> =
            bond.metadata().map(serde_json::to_string).transpose()?;

        self.conn.execute(
        "INSERT INTO bonds (id, name, source, target, created_at, metadata) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            bond.id(),
            bond.name(),
            bond.source().to_string_lossy().to_string(),
            bond.target().to_string_lossy().to_string(),
            bond.created_at_rfc3339(),
            metadata_json
        ],
    )?;

        Ok(bond)
    }

    /// Update a bond's source and/or target.
    /// Replaces the symlink on disk and updates the DB record.
    pub fn update_bond(
        &self,
        id: &str,
        new_source: Option<PathBuf>,
        new_target: Option<PathBuf>,
        new_name: Option<String>,
    ) -> Result<Bond, BondError> {
        let mut bond = self.get_bond(id)?;

        let source = match new_source {
            Some(s) => {
                if !s.exists() {
                    return Err(BondError::InvalidPath(format!(
                        "source does not exist: {:?}",
                        s
                    )));
                }
                s
            }
            None => bond.source.clone(),
        };

        let target = new_target.unwrap_or_else(|| bond.target.clone());

        // Nothing to do if both are unchanged
        if source == bond.source && target == bond.target && new_name.is_none() {
            return Ok(bond);
        }

        // Remove the old symlink (if it still exists)
        if bond.target.exists() || bond.target.symlink_metadata().is_ok() {
            fs::remove_file(&bond.target)?;
        }

        // If target changed and something already exists at the new path, reject
        if target != bond.target && target.exists() {
            return Err(BondError::AlreadyExists);
        }

        // Create parent dirs for new target if needed
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }

        // Create the new symlink
        #[cfg(unix)]
        std::os::unix::fs::symlink(&source, &target)?;
        #[cfg(windows)]
        {
            if source.is_dir() {
                std::os::windows::fs::symlink_dir(&source, &target)?;
            } else {
                std::os::windows::fs::symlink_file(&source, &target)?;
            }
        }

        // Update the DB record
        self.conn.execute(
            "UPDATE bonds SET source = ?1, target = ?2, name = ?3 WHERE id = ?4",
            params![
                source.to_string_lossy().to_string(),
                target.to_string_lossy().to_string(),
                new_name.as_ref().or(bond.name.as_ref()),
                bond.id,
            ],
        )?;

        bond.source = source;
        bond.target = target;
        if new_name.is_some() {
            bond.name = new_name;
        }
        Ok(bond)
    }

    /// Replace a bond's metadata. Pass `None` to clear metadata entirely.
    pub fn update_bond_metadata(
        &self,
        identifier: &str,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Bond, BondError> {
        // Reuse existing identifier resolution (name, full ID, or unique ID prefix).
        let mut bond = self.get_bond(identifier)?;

        let metadata_json: Option<String> =
            metadata.as_ref().map(serde_json::to_string).transpose()?;

        self.conn.execute(
            "UPDATE bonds SET metadata = ?1 WHERE id = ?2",
            params![metadata_json, bond.id()],
        )?;

        bond.metadata = metadata;
        Ok(bond)
    }

    /// Delete a bond by id. If `remove_target` is true, non-symlink targets are removed too.
    pub fn delete_bond(&self, id: &str, remove_target: bool) -> Result<Bond, BondError> {
        let bond = self.get_bond(id)?;

        if bond.target.exists() {
            let meta = fs::symlink_metadata(&bond.target)?;
            if meta.file_type().is_symlink() {
                fs::remove_file(&bond.target)?;
            } else if remove_target {
                if bond.target.is_dir() {
                    fs::remove_dir_all(&bond.target)?;
                } else {
                    fs::remove_file(&bond.target)?;
                }
            } else {
                return Err(BondError::InvalidPath(format!(
                    "target exists and is not a symlink: {:?}",
                    bond.target
                )));
            }
        }

        self.conn
            .execute("DELETE FROM bonds WHERE id = ?1", params![bond.id])?;
        Ok(bond)
    }

    /// Runs schema migration. Useful for testing with in-memory DBs.
    pub(crate) fn from_connection(conn: Connection) -> Result<Self, BondError> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS bonds (
            id TEXT PRIMARY KEY,
            name TEXT,
            source TEXT NOT NULL,
            target TEXT NOT NULL,
            created_at TEXT NOT NULL,
            metadata TEXT
        );",
        )?;

        // Migration: add name column (ignore error if it already exists)
        let _ = conn.execute_batch("ALTER TABLE bonds ADD COLUMN name TEXT;");
        // Migration: add metadata column for older databases (ignore if it already exists).
        let _ = conn.execute_batch("ALTER TABLE bonds ADD COLUMN metadata TEXT;");

        Ok(Self { conn })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::collections::HashMap;
    use tempfile::TempDir; // Needed for metadata assertions

    /// Helper: creates a BondManager backed by in-memory SQLite.
    fn test_manager() -> BondManager {
        let conn = Connection::open_in_memory().unwrap();
        BondManager::from_connection(conn).unwrap()
    }

    /// Helper: creates a real temp directory that acts as a bond source.
    /// Returns (TempDir, PathBuf) -- hold onto TempDir so it doesn't drop.
    fn temp_source() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_path_buf();
        (dir, path)
    }

    #[test]
    fn list_bonds_empty() {
        let mgr = test_manager();
        let bonds = mgr.list_bonds().unwrap();
        assert!(bonds.is_empty());
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn create_and_get_bond() {
        let mgr = test_manager();
        let (_src_dir, src_path) = temp_source();
        let tgt_dir = TempDir::new().unwrap();
        let tgt_path = tgt_dir.path().join("link");

        let bond = mgr.create_bond(&src_path, &tgt_path, None).unwrap();

        // Verify it's in the DB
        let fetched = mgr.get_bond(&bond.id).unwrap();
        assert_eq!(fetched.id, bond.id);
        assert_eq!(fetched.source, src_path);
        assert_eq!(fetched.target, tgt_path);

        // Verify the symlink actually exists
        assert!(
            tgt_path
                .symlink_metadata()
                .unwrap()
                .file_type()
                .is_symlink()
        );
    }

    #[test]
    fn create_bond_nonexistent_source() {
        let mgr = test_manager();
        let result = mgr.create_bond("/no/such/path", "/tmp/whatever", None);
        assert!(matches!(result, Err(BondError::InvalidPath(_))));
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn create_bond_target_already_exists() {
        let mgr = test_manager();
        let (_src_dir, src_path) = temp_source();
        let tgt_dir = TempDir::new().unwrap();
        let tgt_path = tgt_dir.path().join("occupied");

        // Create a non-empty directory at the target
        std::fs::create_dir(&tgt_path).unwrap();
        std::fs::write(tgt_path.join("file.txt"), "data").unwrap();

        let result = mgr.create_bond(&src_path, &tgt_path, None);
        assert!(matches!(result, Err(BondError::TargetExists(_))));
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn delete_bond_removes_symlink() {
        let mgr = test_manager();
        let (_src_dir, src_path) = temp_source();
        let tgt_dir = TempDir::new().unwrap();
        let tgt_path = tgt_dir.path().join("link");

        let bond = mgr.create_bond(&src_path, &tgt_path, None).unwrap();
        assert!(tgt_path.exists());

        mgr.delete_bond(&bond.id, false).unwrap();
        assert!(!tgt_path.exists());

        // Also gone from DB
        assert!(matches!(
            mgr.get_bond(&bond.id),
            Err(BondError::NotFound(_))
        ));
    }

    #[test]
    fn delete_bond_not_found() {
        let mgr = test_manager();
        let result = mgr.delete_bond("nonexistent-id", false);
        assert!(matches!(result, Err(BondError::NotFound(_))));
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn list_bonds_ordered_by_newest() {
        let mgr = test_manager();
        let (_src1, src1) = temp_source();
        let (_src2, src2) = temp_source();
        let tgt_dir = TempDir::new().unwrap();

        let bond1 = mgr
            .create_bond(&src1, tgt_dir.path().join("a"), None)
            .unwrap();
        let bond2 = mgr
            .create_bond(&src2, tgt_dir.path().join("b"), None)
            .unwrap();

        let bonds = mgr.list_bonds().unwrap();
        // bond2 was created second, should appear first (newest-first order)
        assert_eq!(bonds[0].id, bond2.id);
        assert_eq!(bonds[1].id, bond1.id);
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn create_bond_with_metadata_round_trips() {
        let mgr = test_manager();
        let (_src_dir, src_path) = temp_source();
        let tgt_dir = TempDir::new().unwrap();
        let tgt_path = tgt_dir.path().join("link");

        let mut metadata = HashMap::new();
        metadata.insert("project".to_string(), "bonds".to_string());
        metadata.insert("owner".to_string(), "core-team".to_string());

        let created = mgr
            .create_bond_with_metadata(
                &src_path,
                &tgt_path,
                Some("meta-bond".into()),
                Some(metadata.clone()),
            )
            .unwrap();

        assert_eq!(created.metadata(), Some(&metadata));

        let fetched = mgr.get_bond(created.id()).unwrap();
        assert_eq!(fetched.metadata(), Some(&metadata));
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn update_bond_metadata_set_and_clear() {
        let mgr = test_manager();
        let (_src_dir, src_path) = temp_source();
        let tgt_dir = TempDir::new().unwrap();
        let tgt_path = tgt_dir.path().join("link");

        let created = mgr.create_bond(&src_path, &tgt_path, None).unwrap();
        assert!(created.metadata().is_none());

        let mut metadata = HashMap::new();
        metadata.insert("env".to_string(), "dev".to_string());
        metadata.insert("team".to_string(), "platform".to_string());

        let updated = mgr
            .update_bond_metadata(created.id(), Some(metadata.clone()))
            .unwrap();
        assert_eq!(updated.metadata(), Some(&metadata));

        let fetched = mgr.get_bond(created.id()).unwrap();
        assert_eq!(fetched.metadata(), Some(&metadata));

        let cleared = mgr.update_bond_metadata(created.id(), None).unwrap();
        assert!(cleared.metadata().is_none());

        let fetched_again = mgr.get_bond(created.id()).unwrap();
        assert!(fetched_again.metadata().is_none());
    }
}
