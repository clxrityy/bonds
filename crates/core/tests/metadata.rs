mod common;

use common::setup;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
#[cfg_attr(windows, ignore)]
fn metadata_persists_after_manager_reopen() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let mgr = bonds_core::BondManager::new(Some(db_path.clone())).unwrap();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("meta_link");

    let mut metadata = HashMap::new();
    metadata.insert("scope".to_string(), "integration".to_string());
    metadata.insert("status".to_string(), "active".to_string());

    let created = mgr
        .create_bond_with_metadata(
            src.path(),
            &tgt,
            Some("meta-reopen".into()),
            Some(metadata.clone()),
        )
        .unwrap();

    // Re-open manager to prove data round-trips through SQLite.
    drop(mgr);

    let mgr2 = bonds_core::BondManager::new(Some(db_path)).unwrap();
    let fetched = mgr2.get_bond(created.id()).unwrap();
    assert_eq!(fetched.metadata(), Some(&metadata));
}

#[test]
#[cfg_attr(windows, ignore)]
fn metadata_update_round_trip_in_file_db() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("meta_update");

    let created = mgr.create_bond(src.path(), &tgt, None).unwrap();
    assert!(created.metadata().is_none());

    let mut metadata = HashMap::new();
    metadata.insert("team".to_string(), "core".to_string());

    let updated = mgr
        .update_bond_metadata(created.id(), Some(metadata.clone()))
        .unwrap();
    assert_eq!(updated.metadata(), Some(&metadata));

    let fetched = mgr.get_bond(created.id()).unwrap();
    assert_eq!(fetched.metadata(), Some(&metadata));
}
