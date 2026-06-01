use super::*;
use std::collections::HashMap;
use std::fs;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

#[test]
#[cfg_attr(windows, ignore)]
fn create_snapshot_captures_manifest_and_files() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();
    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");
    let history_dir = TempDir::new().expect("history dir");

    // Put real content in the source so the snapshot has something to copy.
    fs::write(src_path.join("notes.txt"), "alpha").expect("write source file");

    // Include metadata so we prove the manifest records it too.
    let mut metadata = HashMap::new();
    metadata.insert("project".to_string(), "bonds".to_string());

    let bond = mgr
        .create_bond_with_metadata(
            &src_path,
            &tgt_path,
            Some("alpha".into()),
            Some(metadata.clone()),
        )
        .expect("create bond");

    // Use a temp history root so the test stays isolated from ~/.bonds/history.
    mgr.set_snapshot_policy(bond.id(), 60, 5, Some(history_dir.path().join("history")))
        .expect("set policy");

    let snapshot = mgr.create_snapshot(bond.id()).expect("create snapshot");

    assert!(snapshot.storage_path.exists());
    assert!(snapshot.payload_path.exists());
    assert!(snapshot.manifest_path.exists());

    // The copied payload should contain the same file contents as the live source.
    let copied_file = snapshot.payload_path.join("notes.txt");
    assert_eq!(
        fs::read_to_string(&copied_file).expect("read copied file"),
        "alpha"
    );

    // The manifest should describe the snapshot accurately.
    let manifest_json = fs::read_to_string(&snapshot.manifest_path).expect("read manifest");
    let manifest: crate::history::SnapshotManifest =
        serde_json::from_str(&manifest_json).expect("parse manifest");

    assert_eq!(manifest.snapshot_id, snapshot.id);
    assert_eq!(manifest.bond_id, bond.id());
    assert_eq!(manifest.bond_name.as_deref(), Some("alpha"));
    assert_eq!(manifest.metadata, Some(metadata));
    assert_eq!(manifest.file_count, 1);
    assert_eq!(manifest.bytes_total, 5);
}

#[test]
#[cfg_attr(windows, ignore)]
fn prune_snapshots_keeps_only_keep_last() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();
    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");
    let history_dir = TempDir::new().expect("history dir");

    let bond = mgr
        .create_bond(&src_path, &tgt_path, Some("alpha".into()))
        .expect("create bond");

    // Keep only the newest two snapshots.
    mgr.set_snapshot_policy(bond.id(), 60, 2, Some(history_dir.path().join("history")))
        .expect("set policy");

    fs::write(src_path.join("state.txt"), "v1").expect("write v1");
    let first = mgr.create_snapshot(bond.id()).expect("snapshot 1");

    // Small sleeps keep created_at ordering deterministic in the test.
    thread::sleep(Duration::from_millis(20));

    fs::write(src_path.join("state.txt"), "v2").expect("write v2");
    let second = mgr.create_snapshot(bond.id()).expect("snapshot 2");

    thread::sleep(Duration::from_millis(20));

    fs::write(src_path.join("state.txt"), "v3").expect("write v3");
    let third = mgr.create_snapshot(bond.id()).expect("snapshot 3");

    let pruned = mgr.prune_snapshots(bond.id()).expect("prune snapshots");

    assert_eq!(pruned.len(), 1);
    assert_eq!(pruned[0].id, first.id);

    let remaining = mgr.list_snapshots(bond.id()).expect("list snapshots");
    assert_eq!(remaining.len(), 2);

    // Newest first.
    assert_eq!(remaining[0].id, third.id);
    assert_eq!(remaining[1].id, second.id);

    assert!(!first.storage_path.exists());
    assert!(second.storage_path.exists());
    assert!(third.storage_path.exists());
}

#[test]
#[cfg_attr(windows, ignore)]
fn restore_snapshot_restores_source_and_creates_safety_snapshot() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();
    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");
    let history_dir = TempDir::new().expect("history dir");

    let bond = mgr
        .create_bond(&src_path, &tgt_path, Some("alpha".into()))
        .expect("create bond");

    // Configure a temp snapshot root so both the original snapshot and the safety
    // snapshot stay inside the test sandbox.
    mgr.set_snapshot_policy(bond.id(), 60, 10, Some(history_dir.path().join("history")))
        .expect("set policy");

    let live_file = src_path.join("state.txt");

    // Snapshot the original contents.
    fs::write(&live_file, "before").expect("write before");
    let original_snapshot = mgr.create_snapshot(bond.id()).expect("create snapshot");

    // Mutate the live source after the snapshot so restore has something to undo.
    fs::write(&live_file, "after").expect("write after");

    let restore = mgr
        .restore_snapshot(bond.id(), &original_snapshot.id)
        .expect("restore snapshot");

    // The live source should be back to the snapshotted contents.
    assert_eq!(
        fs::read_to_string(&live_file).expect("read restored file"),
        "before"
    );

    assert_eq!(restore.snapshot_id, original_snapshot.id);

    // Restore should have created a safety snapshot of the "after" state first.
    let safety_snapshot_id = restore
        .safety_snapshot_id
        .expect("restore should create a safety snapshot");

    let snapshots = mgr.list_snapshots(bond.id()).expect("list snapshots");
    assert_eq!(snapshots.len(), 2);

    let safety_snapshot = snapshots
        .iter()
        .find(|snapshot| snapshot.id == safety_snapshot_id)
        .expect("find safety snapshot");

    let safety_file = safety_snapshot.payload_path.join("state.txt");
    assert_eq!(
        fs::read_to_string(&safety_file).expect("read safety snapshot file"),
        "after"
    );
}
