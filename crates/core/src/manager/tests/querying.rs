use super::*;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
#[cfg_attr(windows, ignore)]
fn query_bonds_filters_by_source_target_and_metadata() {
    let mgr = test_manager();

    let (_src1_dir, src1) = temp_source();
    let (_src2_dir, src2) = temp_source();

    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt1 = tgt_dir.path().join("a");
    let tgt2 = tgt_dir.path().join("b");

    let mut meta1 = HashMap::new();
    meta1.insert("team".to_string(), "core".to_string());

    let mut meta2 = HashMap::new();
    meta2.insert("team".to_string(), "platform".to_string());

    let bond1 = mgr
        .create_bond_with_metadata(&src1, &tgt1, Some("alpha".into()), Some(meta1))
        .expect("create 1");
    let bond2 = mgr
        .create_bond_with_metadata(&src2, &tgt2, Some("beta".into()), Some(meta2))
        .expect("create 2");

    let by_source = mgr.query_by_source(&src1).expect("query source");
    assert_eq!(by_source.len(), 1);
    assert_eq!(by_source[0].id(), bond1.id());

    let by_pair = mgr
        .query_by_metadata("team", "platform")
        .expect("query metadata");
    assert_eq!(by_pair.len(), 1);
    assert_eq!(by_pair[0].id(), bond2.id());
}
