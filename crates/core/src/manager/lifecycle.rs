use super::*;
use std::fs;

/// Create/update/delete and metadata mutation flows.
impl BondManager {
    /// Create a symlink bond and persist it (no metadata).
    pub fn create_bond<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        target: Q,
        name: Option<String>,
    ) -> Result<Bond, BondError> {
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
        self.create_bond_internal(source, target, name, metadata)
    }

    /// Shared create implementation used by both create paths.
    fn create_bond_internal<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        target: Q,
        name: Option<String>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Bond, BondError> {
        let src = source.as_ref().to_path_buf();
        let tgt = target.as_ref().to_path_buf();

        // Name uniqueness check.
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
            // Permit replacing an empty directory.
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

        let mut bond = Bond::new(src.clone(), tgt.clone(), name);
        bond.metadata = metadata;

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

        self.emit_event(BondEventPayload::Created { bond: bond.clone() });
        Ok(bond)
    }

    /// Update source and/or target and/or name.
    pub fn update_bond(
        &self,
        id: &str,
        new_source: Option<PathBuf>,
        new_target: Option<PathBuf>,
        new_name: Option<String>,
    ) -> Result<Bond, BondError> {
        let mut bond = self.get_bond(id)?;
        let before = bond.clone();

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

        // Nothing changed.
        if source == bond.source && target == bond.target && new_name.is_none() {
            return Ok(bond);
        }

        if bond.target.exists() || bond.target.symlink_metadata().is_ok() {
            fs::remove_file(&bond.target)?;
        }

        if target != bond.target && target.exists() {
            return Err(BondError::AlreadyExists);
        }

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }

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

        self.emit_event(BondEventPayload::Updated {
            before,
            after: bond.clone(),
        });
        Ok(bond)
    }

    /// Replace full metadata map. Use None to clear.
    pub fn update_bond_metadata(
        &self,
        identifier: &str,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Bond, BondError> {
        let mut bond = self.get_bond(identifier)?;
        let before = bond.clone();

        let metadata_json: Option<String> =
            metadata.as_ref().map(serde_json::to_string).transpose()?;

        self.conn.execute(
            "UPDATE bonds SET metadata = ?1 WHERE id = ?2",
            params![metadata_json, bond.id()],
        )?;

        bond.metadata = metadata;

        self.emit_event(BondEventPayload::MetadataUpdated {
            before,
            after: bond.clone(),
        });
        Ok(bond)
    }

    /// Delete bond and optionally remove non-symlink target.
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

        self.emit_event(BondEventPayload::Deleted { bond: bond.clone() });
        Ok(bond)
    }
}
