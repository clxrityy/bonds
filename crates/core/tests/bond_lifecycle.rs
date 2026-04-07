use bonds_core::{BondError, BondManager};
use tempfile::TempDir;

/// Integration helper: manager with a real file-backed DB in a temp dir.
fn setup() -> (BondManager, TempDir) {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");
    let mgr = BondManager::new(Some(db_path)).unwrap();
    (mgr, db_dir) // hold db_dir to keep it alive
}

#[test]
fn full_lifecycle_create_list_delete() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("my_bond");

    // Create
    let bond = mgr.create_bond(src.path(), &tgt, None).unwrap();
    assert!(tgt.exists());
    assert!(tgt.symlink_metadata().unwrap().file_type().is_symlink());

    // List
    let bonds = mgr.list_bonds().unwrap();
    assert_eq!(bonds.len(), 1);
    assert_eq!(bonds[0].id, bond.id);

    // Delete
    mgr.delete_bond(&bond.id, false).unwrap();
    assert!(!tgt.exists());
    assert!(mgr.list_bonds().unwrap().is_empty());
}

#[test]
fn symlink_resolves_to_source_contents() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();

    // Write a file inside source
    std::fs::write(src.path().join("hello.txt"), "world").unwrap();

    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");
    mgr.create_bond(src.path(), &tgt, None).unwrap();

    // Reading through the symlink should see the source's contents
    let content = std::fs::read_to_string(tgt.join("hello.txt")).unwrap();
    assert_eq!(content, "world");
}

#[test]
fn delete_with_target_removes_actual_files() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");

    let bond = mgr.create_bond(src.path(), &tgt, None).unwrap();

    // First remove the symlink and replace with a real directory
    // (simulates someone who broke the symlink)
    std::fs::remove_file(&tgt).unwrap();
    std::fs::create_dir(&tgt).unwrap();
    std::fs::write(tgt.join("file.txt"), "data").unwrap();

    // Without --with-target, this should error
    let err = mgr.delete_bond(&bond.id, false).unwrap_err();
    assert!(matches!(err, BondError::InvalidPath(_)));

    // With --with-target, it should succeed and remove everything
    let removed = mgr.delete_bond(&bond.id, true).unwrap();
    assert_eq!(removed.id, bond.id);
    assert!(!tgt.exists());
}

#[test]
fn update_bond_target() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let old_tgt = tgt_dir.path().join("old_link");
    let new_tgt = tgt_dir.path().join("new_link");

    // Create a bond
    let bond = mgr.create_bond(src.path(), &old_tgt, None).unwrap();
    assert!(old_tgt.symlink_metadata().unwrap().file_type().is_symlink());

    // Update only target
    let updated = mgr
        .update_bond(&bond.id, None, Some(new_tgt.clone()), None)
        .unwrap();
    assert_eq!(updated.id, bond.id); // same bond
    assert_eq!(updated.target, new_tgt); // target changed
    assert_eq!(updated.source, bond.source); // source unchanged
    assert!(!old_tgt.exists()); // old symlink removed
    assert!(new_tgt.symlink_metadata().unwrap().file_type().is_symlink()); // new symlink exists

    // Verify DB is consistent
    let fetched = mgr.get_bond(&bond.id).unwrap();
    assert_eq!(fetched.target, new_tgt);
}

#[test]
fn update_bond_source() {
    let (mgr, _db) = setup();
    let old_src = TempDir::new().unwrap();
    std::fs::write(old_src.path().join("a.txt"), "aaa").unwrap();

    let new_src = TempDir::new().unwrap();
    std::fs::write(new_src.path().join("b.txt"), "bbb").unwrap();

    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");

    let bond = mgr.create_bond(old_src.path(), &tgt, None).unwrap();

    // Symlink currently points to old source
    assert!(tgt.join("a.txt").exists());

    // Update source
    let updated = mgr
        .update_bond(&bond.id, Some(new_src.path().to_path_buf()), None, None)
        .unwrap();
    assert_eq!(updated.source, new_src.path());

    // Symlink now points to new source
    assert!(tgt.join("b.txt").exists());
    assert!(!tgt.join("a.txt").exists());
}

#[test]
fn update_bond_rejects_missing_source() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");

    let bond = mgr.create_bond(src.path(), &tgt, None).unwrap();

    let bad_src = std::path::PathBuf::from("/nonexistent/path");
    let err = mgr
        .update_bond(&bond.id, Some(bad_src), None, None)
        .unwrap_err();
    assert!(matches!(err, BondError::InvalidPath(_)));

    // Original bond should be untouched
    assert!(tgt.symlink_metadata().unwrap().file_type().is_symlink());
}

#[test]
fn update_bond_rejects_occupied_target() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");

    let bond = mgr.create_bond(src.path(), &tgt, None).unwrap();

    // Create something at the new target path
    let occupied = tgt_dir.path().join("occupied");
    std::fs::create_dir(&occupied).unwrap();

    let err = mgr
        .update_bond(&bond.id, None, Some(occupied), None)
        .unwrap_err();
    assert!(matches!(err, BondError::AlreadyExists));
}

#[test]
fn create_and_lookup_by_name() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");

    let bond = mgr
        .create_bond(src.path(), &tgt, Some("my-project".into()))
        .unwrap();
    assert_eq!(bond.name.as_deref(), Some("my-project"));

    // Lookup by name
    let found = mgr.get_bond("my-project").unwrap();
    assert_eq!(found.id, bond.id);

    // Name should appear in list
    let all = mgr.list_bonds().unwrap();
    assert_eq!(all[0].name.as_deref(), Some("my-project"));
}

#[test]
fn duplicate_name_rejected() {
    let (mgr, _db) = setup();
    let src1 = TempDir::new().unwrap();
    let src2 = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();

    mgr.create_bond(src1.path(), tgt_dir.path().join("a"), Some("taken".into()))
        .unwrap();
    let err = mgr
        .create_bond(src2.path(), tgt_dir.path().join("b"), Some("taken".into()))
        .unwrap_err();
    assert!(matches!(err, BondError::AlreadyExists));
}

#[test]
fn get_bond_by_prefix() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");

    let bond = mgr.create_bond(src.path(), &tgt, None).unwrap();

    // First 8 chars should resolve
    let prefix = &bond.id[..8];
    let found = mgr.get_bond(prefix).unwrap();
    assert_eq!(found.id, bond.id);
}

#[test]
fn update_bond_name() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");

    let bond = mgr
        .create_bond(src.path(), &tgt, Some("old-name".into()))
        .unwrap();

    let updated = mgr
        .update_bond(&bond.id, None, None, Some("new-name".into()))
        .unwrap();
    assert_eq!(updated.name.as_deref(), Some("new-name"));

    // Lookup by new name works
    let found = mgr.get_bond("new-name").unwrap();
    assert_eq!(found.id, bond.id);
}

#[test]
fn create_bond_into_empty_dir() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("link");

    // Pre-create an empty directory at the target path
    std::fs::create_dir(&tgt).unwrap();

    // Should succeed -- empty dir gets replaced by symlink
    let bond = mgr.create_bond(src.path(), &tgt, None).unwrap();
    assert!(tgt.symlink_metadata().unwrap().file_type().is_symlink());
}
