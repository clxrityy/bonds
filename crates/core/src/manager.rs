use crate::bond::Bond;
use crate::error::BondError;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

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
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS bonds (
                id TEXT PRIMARY KEY,
                source TEXT NOT NULL,
                target TEXT NOT NULL,
                created_at TEXT NOT NULL,
                metadata TEXT
            );",
        )?;
        Ok(Self { conn })
    }

    /// List all bonds (most-recent first).
    pub fn list_bonds(&self) -> Result<Vec<Bond>, BondError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source, target, created_at, metadata FROM bonds ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let source: String = row.get(1)?;
            let target: String = row.get(2)?;
            let created_at_str: String = row.get(3)?;
            let metadata_json: Option<String> = row.get(4)?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                    3,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                ))?;

            let metadata = match metadata_json {
                Some(s) => Some(serde_json::from_str(&s).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                    4,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                ))?),
                None => None,
            };

            Ok(Bond {
                id,
                source: PathBuf::from(source),
                target: PathBuf::from(target),
                created_at,
                metadata,
            })
        })?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    /// Get a single bond by id.
    pub fn get_bond(&self, id: &str) -> Result<Bond, BondError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source, target, created_at, metadata FROM bonds WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let source: String = row.get(1)?;
            let target: String = row.get(2)?;
            let created_at_str: String = row.get(3)?;
            let metadata_json: Option<String> = row.get(4)?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|e| BondError::InvalidPath(format!("invalid timestamp: {}", e)))?;

            let metadata = match metadata_json {
                Some(s) => Some(serde_json::from_str(&s)?),
                None => None,
            };

            Ok(Bond {
                id,
                source: PathBuf::from(source),
                target: PathBuf::from(target),
                created_at,
                metadata,
            })
        } else {
            Err(BondError::NotFound(id.to_string()))
        }
    }

    /// Create a symlink bond and persist it.
    pub fn create_bond<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        target: Q,
    ) -> Result<Bond, BondError> {
        let src = source.as_ref().to_path_buf();
        let tgt = target.as_ref().to_path_buf();

        if !src.exists() {
            return Err(BondError::InvalidPath(format!(
                "source does not exist: {:?}",
                src
            )));
        }
        if tgt.exists() {
            return Err(BondError::AlreadyExists);
        }

        if let Some(parent) = tgt.parent() {
            fs::create_dir_all(parent)?;
        }

        // Platform-specific symlink creation
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

        let bond = Bond::new(src.clone(), tgt.clone());
        let metadata_json: Option<String> = bond
            .metadata
            .as_ref()
            .map(|m| serde_json::to_string(m))
            .transpose()?;

        self.conn.execute(
            "INSERT INTO bonds (id, source, target, created_at, metadata) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                bond.id,
                bond.source.to_string_lossy().to_string(),
                bond.target.to_string_lossy().to_string(),
                bond.created_at_rfc3339(),
                metadata_json
            ],
        )?;

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
            .execute("DELETE FROM bonds WHERE id = ?1", params![id])?;
        Ok(bond)
    }
}
