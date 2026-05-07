use crate::bond::Bond;
use crate::error::BondError;
use crate::query::{BondQuery, MetadataFilter};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf}; // Metadata map type for API methods

/// SQLite-backed manager for Bonds.
/// The `BondManager` struct provides high-level methods for managing the lifecycle of bonds, including creating, retrieving, updating, and deleting bonds. It handles the underlying SQLite database connection and schema management, as well as the filesystem operations required to create and update symlinks. The manager ensures that bond records are kept in sync with the actual state of the filesystem and provides error handling for various edge cases, such as invalid paths or conflicts with existing files.
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
    /// First tries exact name match, then falls back to ID prefix match. Errors if not found or if ID prefix is ambiguous.
    /// This method is used by CLI commands that accept either an ID or name as an identifier for a bond.
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
    /// This is the main method used by CLI commands to create bonds, and it keeps the signature simple for that use case. Library users who want metadata can call `create_bond_with_metadata` instead.
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
    /// This method is intended for library users who want to set metadata at creation time. It has a more complex signature than `create_bond`, but it avoids the need for a separate "update metadata" call after creation.
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
    /// This method performs the actual work of validating paths, creating the symlink, and inserting the record into the database. It is not exposed publicly because it has a more complex signature that includes metadata, which is not needed for the common CLI use case.
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
    /// This method is used by the CLI update command to modify the source or target paths of an existing bond. It validates the new paths, ensures that the target path does not conflict with existing files, updates the symlink on disk, and then updates the corresponding record in the SQLite database. The method returns the updated Bond object after successful completion.
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
    /// This method is used by the CLI command to update the metadata of an existing bond. It accepts a bond identifier (ID or name) and a new metadata map, which can be set to `None` to clear existing metadata. The method updates the metadata in the SQLite database and returns the updated Bond object with the new metadata. This allows users to manage custom key/value pairs associated with their bonds without affecting the source or target paths.
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
    /// This method is used by the CLI delete command to remove an existing bond. It first retrieves the bond by its identifier, checks if the target path exists, and if it does, it determines whether it's a symlink or a regular file/directory. If it's a symlink, it removes it. If it's not a symlink and `remove_target` is true, it removes the file or directory at the target path. Finally, it deletes the bond record from the SQLite database and returns the deleted Bond object. This allows users to clean up bonds and optionally remove the target files/directories they point to.
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
    /// This method is called internally when creating a BondManager from a rusqlite Connection. It ensures that the necessary tables and columns exist in the database, creating them if they are missing. This allows the application to work with both new and existing databases without requiring manual migration steps. The method handles the creation of the `bonds` table and the addition of new columns for name and metadata, while ignoring errors if those columns already exist (which can happen when connecting to an older database).
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

    /// Query bonds using optional source/target/metadata filters.
    ///
    /// This implementation filters in Rust after loading bonds from SQLite.
    /// It keeps behavior predictable across SQLite builds and is sufficient for current scale.
    pub fn query_bonds(&self, query: &BondQuery) -> Result<Vec<Bond>, BondError> {
        let bonds = self.list_bonds()?;

        let filtered = bonds
            .into_iter()
            .filter(|bond| {
                if let Some(source) = query.source.as_ref()
                    && bond.source() != source.as_path()
                {
                    return false;
                }

                if let Some(target) = query.target.as_ref()
                    && bond.target() != target.as_path()
                {
                    return false;
                }

                if let Some(meta_filter) = query.metadata.as_ref() {
                    let Some(metadata) = bond.metadata() else {
                        return false;
                    };

                    match meta_filter {
                        MetadataFilter::HasKey(key) => {
                            if !metadata.contains_key(key) {
                                return false;
                            }
                        }
                        MetadataFilter::KeyValue { key, value } => {
                            if metadata.get(key).map(|v| v.as_str()) != Some(value.as_str()) {
                                return false;
                            }
                        }
                    }
                }

                true
            })
            .collect();

        Ok(filtered)
    }

    /// Convenience helper for exact source matching.
    pub fn query_by_source<P: AsRef<Path>>(&self, source: P) -> Result<Vec<Bond>, BondError> {
        let query = BondQuery::new().with_source(source);
        self.query_bonds(&query)
    }

    /// Convenience helper for exact target matching.
    pub fn query_by_target<P: AsRef<Path>>(&self, target: P) -> Result<Vec<Bond>, BondError> {
        let query = BondQuery::new().with_target(target);
        self.query_bonds(&query)
    }

    /// Convenience helper for metadata key existence matching.
    pub fn query_by_metadata_key(&self, key: &str) -> Result<Vec<Bond>, BondError> {
        let query = BondQuery::new().with_metadata_key(key);
        self.query_bonds(&query)
    }

    /// Convenience helper for metadata key/value matching.
    pub fn query_by_metadata(&self, key: &str, value: &str) -> Result<Vec<Bond>, BondError> {
        let query = BondQuery::new().with_metadata(key, value);
        self.query_bonds(&query)
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

    #[test]
    #[cfg_attr(windows, ignore)]
    fn query_bonds_filters_by_source_target_and_metadata() {
        let mgr = test_manager();

        let (_src1_dir, src1) = temp_source();
        let (_src2_dir, src2) = temp_source();

        let tgt_dir = TempDir::new().unwrap();
        let tgt1 = tgt_dir.path().join("a");
        let tgt2 = tgt_dir.path().join("b");

        let mut meta1 = HashMap::new();
        meta1.insert("team".to_string(), "core".to_string());

        let mut meta2 = HashMap::new();
        meta2.insert("team".to_string(), "platform".to_string());

        let bond1 = mgr
            .create_bond_with_metadata(&src1, &tgt1, Some("alpha".into()), Some(meta1))
            .unwrap();
        let bond2 = mgr
            .create_bond_with_metadata(&src2, &tgt2, Some("beta".into()), Some(meta2))
            .unwrap();

        let by_source = mgr.query_by_source(&src1).unwrap();
        assert_eq!(by_source.len(), 1);
        assert_eq!(by_source[0].id(), bond1.id());

        let by_target = mgr.query_by_target(&tgt2).unwrap();
        assert_eq!(by_target.len(), 1);
        assert_eq!(by_target[0].id(), bond2.id());

        let by_key = mgr.query_by_metadata_key("team").unwrap();
        assert_eq!(by_key.len(), 2);

        let by_pair = mgr.query_by_metadata("team", "platform").unwrap();
        assert_eq!(by_pair.len(), 1);
        assert_eq!(by_pair[0].id(), bond2.id());

        let combined = mgr
            .query_bonds(
                &BondQuery::new()
                    .with_source(&src2)
                    .with_metadata("team", "platform"),
            )
            .unwrap();
        assert_eq!(combined.len(), 1);
        assert_eq!(combined[0].id(), bond2.id());
    }
}
