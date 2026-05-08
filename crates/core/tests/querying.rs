mod common;

use bonds_core::BondQuery;
use common::setup;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
#[cfg_attr(windows, ignore)]
fn query_by_source_target_and_metadata_integration() {
    let (mgr, _db) = setup();

    let src1 = TempDir::new().unwrap();
    let src2 = TempDir::new().unwrap();
    let tgt_dir = TempDir::new().unwrap();

    let tgt1 = tgt_dir.path().join("a");
    let tgt2 = tgt_dir.path().join("b");

    let mut meta1 = HashMap::new();
    meta1.insert("team".to_string(), "core".to_string());

    let mut meta2 = HashMap::new();
    meta2.insert("team".to_string(), "platform".to_string());

    let b1 = mgr
        .create_bond_with_metadata(src1.path(), &tgt1, Some("alpha".into()), Some(meta1))
        .unwrap();
    let b2 = mgr
        .create_bond_with_metadata(src2.path(), &tgt2, Some("beta".into()), Some(meta2))
        .unwrap();

    let by_source = mgr.query_by_source(src1.path()).unwrap();
    assert_eq!(by_source.len(), 1);
    assert_eq!(by_source[0].id(), b1.id());

    let by_target = mgr.query_by_target(&tgt2).unwrap();
    assert_eq!(by_target.len(), 1);
    assert_eq!(by_target[0].id(), b2.id());

    let by_pair = mgr.query_by_metadata("team", "platform").unwrap();
    assert_eq!(by_pair.len(), 1);
    assert_eq!(by_pair[0].id(), b2.id());

    // Combined filter path via BondQuery builder.
    let combined = mgr
        .query_bonds(
            &BondQuery::new()
                .with_source(src2.path())
                .with_metadata("team", "platform"),
        )
        .unwrap();
    assert_eq!(combined.len(), 1);
    assert_eq!(combined[0].id(), b2.id());
}
