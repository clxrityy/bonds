use super::*;
use crate::history::{
    RestoreRecord, SnapshotManifest, SnapshotPolicy, SnapshotRecord, default_history_root,
};
use chrono::Duration;
use std::fs;

#[derive(Debug, Default, Clone, Copy)]
struct CopyStats {
    files: i64,
    bytes_total: i64,
}

impl BondManager {
    pub fn set_snapshot_policy(
        &self,
        identifier: &str,
        interval_seconds: i64,
        keep_last: i64,
        storage_root: Option<PathBuf>,
    ) -> Result<SnapshotPolicy, BondError> {
        if interval_seconds <= 0 {
            return Err(BondError::Config(
                "interval_seconds must be greater than 0".into(),
            ));
        }

        if keep_last <= 0 {
            return Err(BondError::Config("keep_last must be greater than 0".into()));
        }

        let bond = self.get_bond(identifier)?;
        let now = Utc::now();

        let policy = SnapshotPolicy {
            bond_id: bond.id().to_string(),
            enabled: true,
            interval_seconds,
            keep_last,
            storage_root,
            last_run_at: None,
            next_run_at: Some(now),
            last_error: None,
            last_error_at: None,
            updated_at: now,
        };

        self.conn.execute(
            r#"
            INSERT INTO snapshot_policies (
                bond_id,
                enabled,
                interval_seconds,
                keep_last,
                storage_root,
                last_run_at,
                next_run_at,
                last_error,
                last_error_at,
                updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            ON CONFLICT(bond_id) DO UPDATE SET
                enabled = excluded.enabled,
                interval_seconds = excluded.interval_seconds,
                keep_last = excluded.keep_last,
                storage_root = excluded.storage_root,
                next_run_at = excluded.next_run_at,
                last_error = NULL,
                last_error_at = NULL,
                updated_at = excluded.updated_at
            "#,
            params![
                policy.bond_id,
                policy.enabled as i64,
                policy.interval_seconds,
                policy.keep_last,
                policy
                    .storage_root
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string()),
                policy.last_run_at.map(|dt| dt.to_rfc3339()),
                policy.next_run_at.map(|dt| dt.to_rfc3339()),
                policy.last_error,
                policy.last_error_at.map(|dt| dt.to_rfc3339()),
                policy.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(policy)
    }

    pub fn disable_snapshot_policy(&self, identifier: &str) -> Result<(), BondError> {
        let bond = self.get_bond(identifier)?;
        let changed = self.conn.execute(
            "UPDATE snapshot_policies SET enabled = 0, updated_at = ?2 WHERE bond_id = ?1",
            params![bond.id(), Utc::now().to_rfc3339()],
        )?;

        if changed == 0 {
            return Err(BondError::NotFound(format!(
                "snapshot policy not found for bond {identifier}"
            )));
        }

        Ok(())
    }

    pub fn get_snapshot_policy(
        &self,
        identifier: &str,
    ) -> Result<Option<SnapshotPolicy>, BondError> {
        let bond = self.get_bond(identifier)?;
        self.get_snapshot_policy_by_bond_id(bond.id())
    }

    pub fn list_due_snapshot_policies(
        &self,
        now: DateTime<Utc>,
    ) -> Result<Vec<SnapshotPolicy>, BondError> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT
                bond_id,
                enabled,
                interval_seconds,
                keep_last,
                storage_root,
                last_run_at,
                next_run_at,
                last_error,
                last_error_at,
                updated_at
            FROM snapshot_policies
            WHERE enabled = 1
                AND next_run_at IS NOT NULL
                AND next_run_at <= ?1
            ORDER BY next_run_at ASC
            "#,
        )?;

        let mut rows = stmt.query(params![now.to_rfc3339()])?;
        let mut out = Vec::new();

        while let Some(row) = rows.next()? {
            out.push(self.snapshot_policy_from_row(row)?);
        }

        Ok(out)
    }

    pub fn mark_snapshot_policy_success(
        &self,
        bond_id: &str,
        at: DateTime<Utc>,
    ) -> Result<(), BondError> {
        let Some(policy) = self.get_snapshot_policy_by_bond_id(bond_id)? else {
            return Ok(());
        };

        let next_run = at + Duration::seconds(policy.interval_seconds);

        self.conn.execute(
            r#"
            UPDATE snapshot_policies
            SET last_run_at = ?2,
                next_run_at = ?3,
                last_error = NULL,
                last_error_at = NULL,
                updated_at = ?4
            WHERE bond_id = ?1
            "#,
            params![
                bond_id,
                at.to_rfc3339(),
                next_run.to_rfc3339(),
                at.to_rfc3339()
            ],
        )?;

        Ok(())
    }

    pub fn mark_snapshot_policy_failure(
        &self,
        bond_id: &str,
        message: &str,
        at: DateTime<Utc>,
    ) -> Result<(), BondError> {
        let Some(policy) = self.get_snapshot_policy_by_bond_id(bond_id)? else {
            return Ok(());
        };

        let next_run = at + Duration::seconds(policy.interval_seconds.max(1));

        self.conn.execute(
            r#"
            UPDATE snapshot_policies
            SET last_error = ?2,
                last_error_at = ?3,
                next_run_at = ?4,
                updated_at = ?3
            WHERE bond_id = ?1
            "#,
            params![bond_id, message, at.to_rfc3339(), next_run.to_rfc3339()],
        )?;

        Ok(())
    }

    pub fn create_snapshot(&self, identifier: &str) -> Result<SnapshotRecord, BondError> {
        let bond = self.get_bond(identifier)?;

        let policy = self.get_snapshot_policy_by_bond_id(bond.id())?;
        let history_root = policy
            .as_ref()
            .and_then(|p| p.storage_root.clone())
            .unwrap_or_else(default_history_root);

        let snapshot_id = uuid::Uuid::new_v4().to_string();
        let snapshot_root = history_root.join(bond.id()).join(&snapshot_id);
        let payload_path = snapshot_root
            .join("payload")
            .join(snapshot_entry_name(bond.source()));
        let manifest_path = snapshot_root.join("manifest.json");

        fs::create_dir_all(&snapshot_root)?;
        let stats = copy_path(bond.source(), &payload_path)?;

        let snapshot = SnapshotRecord {
            id: snapshot_id,
            bond_id: bond.id().to_string(),
            bond_name: bond.name().map(str::to_string),
            created_at: Utc::now(),
            storage_path: snapshot_root.clone(),
            payload_path: payload_path.clone(),
            manifest_path: manifest_path.clone(),
            source_path: bond.source().to_path_buf(),
            target_path: bond.target().to_path_buf(),
            metadata: bond.metadata().cloned(),
            file_count: stats.files,
            bytes_total: stats.bytes_total,
        };

        let manifest = SnapshotManifest {
            snapshot_id: snapshot.id.clone(),
            bond_id: snapshot.bond_id.clone(),
            bond_name: snapshot.bond_name.clone(),
            created_at: snapshot.created_at,
            source_path: snapshot.source_path.clone(),
            target_path: snapshot.target_path.clone(),
            payload_path: snapshot.payload_path.clone(),
            metadata: snapshot.metadata.clone(),
            file_count: snapshot.file_count,
            bytes_total: snapshot.bytes_total,
        };

        fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;

        self.conn.execute(
            r#"
            INSERT INTO snapshots (
                id,
                bond_id,
                bond_name,
                created_at,
                storage_path,
                payload_path,
                manifest_path,
                source_path,
                target_path,
                metadata_json,
                file_count,
                bytes_total
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
            params![
                snapshot.id,
                snapshot.bond_id,
                snapshot.bond_name,
                snapshot.created_at.to_rfc3339(),
                snapshot.storage_path.to_string_lossy().to_string(),
                snapshot.payload_path.to_string_lossy().to_string(),
                snapshot.manifest_path.to_string_lossy().to_string(),
                snapshot.source_path.to_string_lossy().to_string(),
                snapshot.target_path.to_string_lossy().to_string(),
                snapshot
                    .metadata
                    .as_ref()
                    .map(serde_json::to_string)
                    .transpose()?,
                snapshot.file_count,
                snapshot.bytes_total,
            ],
        )?;

        Ok(snapshot)
    }

    pub fn list_snapshots(&self, identifier: &str) -> Result<Vec<SnapshotRecord>, BondError> {
        let bond = self.get_bond(identifier)?;
        let mut stmt = self.conn.prepare(
            r#"
            SELECT
                id,
                bond_id,
                bond_name,
                created_at,
                storage_path,
                payload_path,
                manifest_path,
                source_path,
                target_path,
                metadata_json,
                file_count,
                bytes_total
            FROM snapshots
            WHERE bond_id = ?1
            ORDER BY created_at DESC
            "#,
        )?;

        let mut rows = stmt.query(params![bond.id()])?;
        let mut out = Vec::new();

        while let Some(row) = rows.next()? {
            out.push(self.snapshot_from_row(row)?);
        }

        Ok(out)
    }

    pub fn prune_snapshots(&self, identifier: &str) -> Result<Vec<SnapshotRecord>, BondError> {
        let bond = self.get_bond(identifier)?;
        let Some(policy) = self.get_snapshot_policy_by_bond_id(bond.id())? else {
            return Ok(Vec::new());
        };

        let snapshots = self.list_snapshots(bond.id())?;
        if snapshots.len() <= policy.keep_last as usize {
            return Ok(Vec::new());
        }

        let to_delete: Vec<SnapshotRecord> = snapshots
            .into_iter()
            .skip(policy.keep_last as usize)
            .collect();

        for snapshot in &to_delete {
            if snapshot.storage_path.exists() {
                fs::remove_dir_all(&snapshot.storage_path)?;
            }

            self.conn
                .execute("DELETE FROM snapshots WHERE id = ?1", params![snapshot.id])?;
        }

        Ok(to_delete)
    }

    pub fn restore_snapshot(
        &self,
        identifier: &str,
        snapshot_identifier: &str,
    ) -> Result<RestoreRecord, BondError> {
        let bond = self.get_bond(identifier)?;
        let snapshot = self.get_snapshot_record(bond.id(), snapshot_identifier)?;

        if !snapshot.payload_path.exists() {
            return Err(BondError::InvalidPath(format!(
                "snapshot payload is missing: {}",
                snapshot.payload_path.display()
            )));
        }

        let safety_snapshot = if bond.source().exists() || bond.source().symlink_metadata().is_ok()
        {
            Some(self.create_snapshot(bond.id())?)
        } else {
            None
        };

        remove_path_if_exists(bond.source())?;
        copy_path(&snapshot.payload_path, bond.source())?;

        let restore = RestoreRecord {
            id: uuid::Uuid::new_v4().to_string(),
            bond_id: bond.id().to_string(),
            snapshot_id: snapshot.id.clone(),
            safety_snapshot_id: safety_snapshot.as_ref().map(|s| s.id.clone()),
            created_at: Utc::now(),
            status: "completed".to_string(),
            notes: None,
        };

        self.conn.execute(
            r#"
            INSERT INTO restore_events (
                id,
                bond_id,
                snapshot_id,
                safety_snapshot_id,
                created_at,
                status,
                notes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                restore.id,
                restore.bond_id,
                restore.snapshot_id,
                restore.safety_snapshot_id,
                restore.created_at.to_rfc3339(),
                restore.status,
                restore.notes,
            ],
        )?;

        Ok(restore)
    }

    fn get_snapshot_policy_by_bond_id(
        &self,
        bond_id: &str,
    ) -> Result<Option<SnapshotPolicy>, BondError> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT
                bond_id,
                enabled,
                interval_seconds,
                keep_last,
                storage_root,
                last_run_at,
                next_run_at,
                last_error,
                last_error_at,
                updated_at
            FROM snapshot_policies
            WHERE bond_id = ?1
            "#,
        )?;

        let mut rows = stmt.query(params![bond_id])?;

        match rows.next()? {
            Some(row) => Ok(Some(self.snapshot_policy_from_row(row)?)),
            None => Ok(None),
        }
    }

    fn get_snapshot_record(
        &self,
        bond_id: &str,
        identifier: &str,
    ) -> Result<SnapshotRecord, BondError> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT
                id,
                bond_id,
                bond_name,
                created_at,
                storage_path,
                payload_path,
                manifest_path,
                source_path,
                target_path,
                metadata_json,
                file_count,
                bytes_total
            FROM snapshots
            WHERE bond_id = ?1
              AND id = ?2
            "#,
        )?;

        let mut rows = stmt.query(params![bond_id, identifier])?;
        if let Some(row) = rows.next()? {
            return self.snapshot_from_row(row);
        }

        drop(rows);
        drop(stmt);

        let mut stmt = self.conn.prepare(
            r#"
            SELECT
                id,
                bond_id,
                bond_name,
                created_at,
                storage_path,
                payload_path,
                manifest_path,
                source_path,
                target_path,
                metadata_json,
                file_count,
                bytes_total
            FROM snapshots
            WHERE bond_id = ?1
              AND id LIKE ?2 || '%'
            "#,
        )?;

        let mut rows = stmt.query(params![bond_id, identifier])?;

        let first = match rows.next()? {
            Some(row) => self.snapshot_from_row(row)?,
            None => {
                return Err(BondError::NotFound(format!(
                    "snapshot not found: {identifier}"
                )));
            }
        };

        if rows.next()?.is_some() {
            return Err(BondError::AmbiguousId(identifier.to_string()));
        }

        Ok(first)
    }

    fn snapshot_policy_from_row(&self, row: &rusqlite::Row) -> Result<SnapshotPolicy, BondError> {
        Ok(SnapshotPolicy {
            bond_id: row.get::<_, String>(0)?,
            enabled: row.get::<_, i64>(1)? != 0,
            interval_seconds: row.get(2)?,
            keep_last: row.get(3)?,
            storage_root: row.get::<_, Option<String>>(4)?.map(PathBuf::from),
            last_run_at: parse_optional_rfc3339(row.get::<_, Option<String>>(5)?)?,
            next_run_at: parse_optional_rfc3339(row.get::<_, Option<String>>(6)?)?,
            last_error: row.get(7)?,
            last_error_at: parse_optional_rfc3339(row.get::<_, Option<String>>(8)?)?,
            updated_at: parse_rfc3339(row.get::<_, String>(9)?)?,
        })
    }

    fn snapshot_from_row(&self, row: &rusqlite::Row) -> Result<SnapshotRecord, BondError> {
        let metadata = row
            .get::<_, Option<String>>(9)?
            .map(|json| serde_json::from_str(&json))
            .transpose()?;

        Ok(SnapshotRecord {
            id: row.get(0)?,
            bond_id: row.get(1)?,
            bond_name: row.get(2)?,
            created_at: parse_rfc3339(row.get::<_, String>(3)?)?,
            storage_path: PathBuf::from(row.get::<_, String>(4)?),
            payload_path: PathBuf::from(row.get::<_, String>(5)?),
            manifest_path: PathBuf::from(row.get::<_, String>(6)?),
            source_path: PathBuf::from(row.get::<_, String>(7)?),
            target_path: PathBuf::from(row.get::<_, String>(8)?),
            metadata,
            file_count: row.get(10)?,
            bytes_total: row.get(11)?,
        })
    }
}

fn parse_rfc3339(raw: String) -> Result<DateTime<Utc>, BondError> {
    DateTime::parse_from_rfc3339(&raw)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| BondError::InvalidTimestamp(e.to_string()))
}

fn parse_optional_rfc3339(raw: Option<String>) -> Result<Option<DateTime<Utc>>, BondError> {
    raw.map(parse_rfc3339).transpose()
}

fn snapshot_entry_name(source: &Path) -> String {
    source
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "source".to_string())
}

fn remove_path_if_exists(path: &Path) -> Result<(), BondError> {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return Ok(());
    };

    let file_type = meta.file_type();
    if file_type.is_symlink() || meta.is_file() {
        fs::remove_file(path)?;
    } else if meta.is_dir() {
        fs::remove_dir_all(path)?;
    }

    Ok(())
}

fn copy_path(source: &Path, destination: &Path) -> Result<CopyStats, BondError> {
    let meta = fs::symlink_metadata(source)?;
    let mut stats = CopyStats::default();

    if meta.file_type().is_symlink() {
        copy_symlink(source, destination, &mut stats)?;
        return Ok(stats);
    }

    if meta.is_dir() {
        fs::create_dir_all(destination)?;
        copy_dir_contents(source, destination, &mut stats)?;
        return Ok(stats);
    }

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    stats.bytes_total += fs::copy(source, destination)? as i64;
    stats.files += 1;
    Ok(stats)
}

fn copy_dir_contents(
    source_dir: &Path,
    destination_dir: &Path,
    stats: &mut CopyStats,
) -> Result<(), BondError> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination_dir.join(entry.file_name());
        let meta = fs::symlink_metadata(&source_path)?;

        if meta.file_type().is_symlink() {
            copy_symlink(&source_path, &destination_path, stats)?;
            continue;
        }

        if meta.is_dir() {
            fs::create_dir_all(&destination_path)?;
            copy_dir_contents(&source_path, &destination_path, stats)?;
            continue;
        }

        if let Some(parent) = destination_path.parent() {
            fs::create_dir_all(parent)?;
        }

        stats.bytes_total += fs::copy(&source_path, &destination_path)? as i64;
        stats.files += 1;
    }

    Ok(())
}

fn copy_symlink(source: &Path, destination: &Path, stats: &mut CopyStats) -> Result<(), BondError> {
    let target = fs::read_link(source)?;

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, destination)?;

    #[cfg(windows)]
    {
        if source.is_dir() {
            std::os::windows::fs::symlink_dir(&target, destination)?;
        } else {
            std::os::windows::fs::symlink_file(&target, destination)?;
        }
    }

    stats.files += 1;
    Ok(())
}
