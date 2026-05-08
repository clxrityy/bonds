use super::*;
use crate::error::BondError;
use tempfile::TempDir;

#[test]
fn list_bonds_empty() {
    let mgr = test_manager();
    let bonds = mgr.list_bonds().expect("list bonds");
    assert!(bonds.is_empty());
}

#[test]
#[cfg_attr(windows, ignore)]
fn create_and_get_bond() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();
    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");

    let bond = mgr.create_bond(&src_path, &tgt_path, None).expect("create");
    let fetched = mgr.get_bond(bond.id()).expect("get");

    assert_eq!(fetched.id(), bond.id());
    assert_eq!(fetched.source(), src_path.as_path());
    assert_eq!(fetched.target(), tgt_path.as_path());
}

#[test]
fn create_bond_nonexistent_source() {
    let mgr = test_manager();
    let result = mgr.create_bond("/no/such/path", "/tmp/whatever", None);
    assert!(matches!(result, Err(BondError::InvalidPath(_))));
}
