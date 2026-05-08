use super::*;
use std::fs;

/// Storage/open/read/schema concerns for BondManager.
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
        Self::from_connection(conn)
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

    /// Get a single bond by ID or name. ID can be a unique prefix.
    pub fn get_bond(&self, identifier: &str) -> Result<Bond, BondError> {
        // 1) Exact name
        let mut stmt = self.conn.prepare(
            "SELECT id, name, source, target, created_at, metadata FROM bonds WHERE name = ?1",
        )?;
        let mut rows = stmt.query(params![identifier])?;
        if let Some(row) = rows.next()? {
            return self.bond_from_row(row);
        }
        drop(rows);
        drop(stmt);

        // 2) ID prefix
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

    /// Parse a Bond from a rusqlite row.
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

        // Safe migrations for older DBs.
        let _ = conn.execute_batch("ALTER TABLE bonds ADD COLUMN name TEXT;");
        let _ = conn.execute_batch("ALTER TABLE bonds ADD COLUMN metadata TEXT;");

        Ok(Self {
            conn,
            hooks: RwLock::new(Vec::new()),
        })
    }
}
