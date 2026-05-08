use super::*;
use crate::events::{BondBrokenReason, BondEvent, BondEventKind, BondEventPayload};
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::TempDir;

#[test]
#[cfg_attr(windows, ignore)]
fn hooks_receive_lifecycle_events_in_order() {
    let mgr = test_manager();
    let collector = Arc::new(TestHook::default());
    mgr.register_hook_arc(collector.clone());

    let (_src_dir, src_path) = temp_source();
    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");

    let created = mgr
        .create_bond(&src_path, &tgt_path, Some("old".into()))
        .expect("create");
    mgr.update_bond(created.id(), None, None, Some("new".into()))
        .expect("update");

    let mut metadata = HashMap::new();
    metadata.insert("team".to_string(), "core".to_string());
    mgr.update_bond_metadata(created.id(), Some(metadata))
        .expect("metadata");
    mgr.delete_bond(created.id(), false).expect("delete");

    let events = collector.events.lock().expect("events lock");
    let kinds: Vec<BondEventKind> = events.iter().map(BondEvent::kind).collect();

    assert_eq!(
        kinds,
        vec![
            BondEventKind::Created,
            BondEventKind::Updated,
            BondEventKind::MetadataUpdated,
            BondEventKind::Deleted
        ]
    );
}

#[test]
#[cfg_attr(windows, ignore)]
fn scan_broken_bonds_emits_broken_detected_event() {
    let mgr = test_manager();
    let collector = Arc::new(TestHook::default());
    mgr.register_hook_arc(collector.clone());

    let (_src_dir, src_path) = temp_source();
    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");

    let created = mgr.create_bond(&src_path, &tgt_path, None).expect("create");

    // Simulate breakage by removing the symlink itself.
    std::fs::remove_file(created.target()).expect("remove symlink");

    let broken = mgr.scan_broken_bonds().expect("scan broken");
    assert_eq!(broken.len(), 1);
    assert_eq!(broken[0].id(), created.id());

    let events = collector.events.lock().expect("events lock");
    let broken_events: Vec<&BondEvent> = events
        .iter()
        .filter(|e| e.kind() == BondEventKind::BrokenDetected)
        .collect();

    assert_eq!(broken_events.len(), 1);
    assert!(matches!(
        broken_events[0].payload,
        BondEventPayload::BrokenDetected {
            reason: BondBrokenReason::MissingTarget,
            ..
        }
    ));
}
