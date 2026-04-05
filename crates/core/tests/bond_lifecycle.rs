use bonds_core::{BondError, BondManager};
use tempfile::TempDir;

/// Integration helper: manager with a real file-backed DB in a temp dir.
fn setup() -> (BondManager, TempDir) {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");
    let mgr = BondManager::new(Some(db_path)).unwrap();
    (mgr, db_dir)  // hold db_dir to keep it alive
}

#[test]
fn full_lifecycle_create_list_delete() {
    let (mgr, _db) = setup();
    let src = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();
    let tgt = tgt_dir.path().join("my_bond");

    // Create
    let bond = mgr.create_bond(src.path(), &tgt).unwrap();
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
    mgr.create_bond(src.path(), &tgt).unwrap();

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

    let bond = mgr.create_bond(src.path(), &tgt).unwrap();

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
