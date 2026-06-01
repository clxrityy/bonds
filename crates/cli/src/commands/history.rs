use bonds_cli::ui;
use bonds_core::{BondError, BondManager};
use chrono::Utc;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub fn cmd_history_enable(
    manager: &BondManager,
    id: &str,
    every_seconds: i64,
    keep_last: i64,
    storage_root: Option<PathBuf>,
) -> Result<(), BondError> {
    let policy = manager.set_snapshot_policy(id, every_seconds, keep_last, storage_root)?;

    ui::status_ok("history enabled");
    ui::success(format!(
        "bond={} interval={}s keep_last={}",
        policy.bond_id, policy.interval_seconds, policy.keep_last
    ));

    if let Some(root) = policy.storage_root {
        ui::info(format!("storage root: {}", root.display()));
    }

    Ok(())
}

pub fn cmd_history_disable(manager: &BondManager, id: &str) -> Result<(), BondError> {
    manager.disable_snapshot_policy(id)?;
    ui::status_ok("history disabled");
    ui::success(format!("disabled snapshot policy for {id}"));
    Ok(())
}

pub fn cmd_history_snapshot(manager: &BondManager, id: &str) -> Result<(), BondError> {
    let snapshot = manager.create_snapshot(id)?;
    let pruned = manager.prune_snapshots(id)?;

    ui::status_ok("snapshot created");
    ui::success(format!(
        "snapshot={} files={} bytes={}",
        snapshot.id, snapshot.file_count, snapshot.bytes_total
    ));
    ui::path(format!("stored at {}", snapshot.storage_path.display()));

    if !pruned.is_empty() {
        ui::warning(format!("pruned {} older snapshot(s)", pruned.len()));
    }

    Ok(())
}

pub fn cmd_history_list(manager: &BondManager, id: &str) -> Result<(), BondError> {
    let snapshots = manager.list_snapshots(id)?;

    ui::title("Snapshots");

    if snapshots.is_empty() {
        ui::warning("No snapshots found.");
        return Ok(());
    }

    for snapshot in snapshots {
        ui::subheading(format!(
            "{}  {}",
            snapshot.id,
            snapshot.created_at.to_rfc3339()
        ));
        ui::normal(format!(
            "  files={} bytes={}",
            snapshot.file_count, snapshot.bytes_total
        ));
        ui::path(format!("  {}", snapshot.storage_path.display()));
    }

    Ok(())
}

pub fn cmd_history_restore(
    manager: &BondManager,
    id: &str,
    snapshot_id: &str,
) -> Result<(), BondError> {
    let restore = manager.restore_snapshot(id, snapshot_id)?;

    ui::status_ok("restore completed");
    ui::success(format!(
        "restored snapshot {} for bond {}",
        restore.snapshot_id, restore.bond_id
    ));

    if let Some(safety_snapshot_id) = restore.safety_snapshot_id {
        ui::info(format!(
            "safety snapshot created first: {}",
            safety_snapshot_id
        ));
    }

    Ok(())
}

pub fn cmd_history_watch(
    manager: &BondManager,
    bond: Option<&str>,
    all_enabled: bool,
    poll_seconds: u64,
    verbose: bool,
) -> Result<(), BondError> {
    if bond.is_none() && !all_enabled {
        return Err(BondError::Config(
            "pass --bond <id|name> or --all-enabled".into(),
        ));
    }

    let selected_bond_id = match bond {
        Some(id) => {
            let bond = manager.get_bond(id)?;
            let Some(policy) = manager.get_snapshot_policy(bond.id())? else {
                return Err(BondError::NotFound(format!(
                    "snapshot policy not found for bond {id}"
                )));
            };

            if !policy.enabled {
                return Err(BondError::Config(format!(
                    "snapshot policy for {id} is disabled"
                )));
            }

            Some(bond.id().to_string())
        }
        None => None,
    };

    ui::status_ok("history watcher started");
    if let Some(bond_id) = selected_bond_id.as_deref() {
        ui::info(format!("watching bond {}", bond_id));
    } else {
        ui::info("watching all enabled bonds");
    }

    loop {
        let now = Utc::now();
        let policies = manager.list_due_snapshot_policies(now)?;

        let due_policies: Vec<_> = policies
            .into_iter()
            .filter(|policy| {
                selected_bond_id
                    .as_deref()
                    .map(|selected| policy.bond_id == selected)
                    .unwrap_or(true)
            })
            .collect();

        if verbose && !due_policies.is_empty() {
            ui::debug(format!("{} due policy(s) found", due_policies.len()));
        }

        for policy in due_policies {
            match manager.create_snapshot(&policy.bond_id) {
                Ok(snapshot) => match manager.prune_snapshots(&policy.bond_id) {
                    Ok(pruned) => {
                        let finished_at = Utc::now();
                        manager.mark_snapshot_policy_success(&policy.bond_id, finished_at)?;

                        ui::success(format!(
                            "snapshot ok bond={} snapshot={}",
                            policy.bond_id, snapshot.id
                        ));

                        if verbose {
                            ui::debug(format!(
                                "files={} bytes={} pruned={}",
                                snapshot.file_count,
                                snapshot.bytes_total,
                                pruned.len()
                            ));
                        }
                    }
                    Err(err) => {
                        manager.mark_snapshot_policy_failure(
                            &policy.bond_id,
                            &err.to_string(),
                            Utc::now(),
                        )?;
                        ui::error(format!("prune failed for bond {}: {}", policy.bond_id, err));
                    }
                },
                Err(err) => {
                    manager.mark_snapshot_policy_failure(
                        &policy.bond_id,
                        &err.to_string(),
                        Utc::now(),
                    )?;
                    ui::error(format!(
                        "snapshot failed for bond {}: {}",
                        policy.bond_id, err
                    ));
                }
            }
        }

        thread::sleep(Duration::from_secs(poll_seconds.max(1)));
    }
}
