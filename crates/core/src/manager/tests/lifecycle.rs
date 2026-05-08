use super::*;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
#[cfg_attr(windows, ignore)]
fn update_bond_metadata_set_and_clear() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();
    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");

    let created = mgr.create_bond(&src_path, &tgt_path, None).expect("create");
    assert!(created.metadata().is_none());

    let mut metadata = HashMap::new();
    metadata.insert("env".to_string(), "dev".to_string());

    let updated = mgr
        .update_bond_metadata(created.id(), Some(metadata.clone()))
        .expect("set metadata");
    assert_eq!(updated.metadata(), Some(&metadata));

    let cleared = mgr
        .update_bond_metadata(created.id(), None)
        .expect("clear metadata");
    assert!(cleared.metadata().is_none());
}

#[test]
#[cfg_attr(windows, ignore)]
fn delete_bond_removes_symlink() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();
    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");

    let bond = mgr.create_bond(&src_path, &tgt_path, None).expect("create");
    mgr.delete_bond(bond.id(), false).expect("delete");

    assert!(!tgt_path.exists());
}
