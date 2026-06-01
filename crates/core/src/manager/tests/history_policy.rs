use super::*;
use chrono::{DateTime, Duration, Utc};
use tempfile::TempDir;

fn fixed_utc(raw: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(raw)
        .expect("parse fixed timestamp")
        .with_timezone(&Utc)
}

#[test]
#[cfg_attr(windows, ignore)]
fn list_due_snapshot_policies_only_returns_enabled_due_entries() {
    let mgr = test_manager();

    let (_src_due_dir, src_due) = temp_source();
    let (_src_future_dir, src_future) = temp_source();
    let (_src_disabled_dir, src_disabled) = temp_source();

    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_due = tgt_dir.path().join("due");
    let tgt_future = tgt_dir.path().join("future");
    let tgt_disabled = tgt_dir.path().join("disabled");

    let due_bond = mgr
        .create_bond(&src_due, &tgt_due, Some("due".into()))
        .expect("create due bond");
    let future_bond = mgr
        .create_bond(&src_future, &tgt_future, Some("future".into()))
        .expect("create future bond");
    let disabled_bond = mgr
        .create_bond(&src_disabled, &tgt_disabled, Some("disabled".into()))
        .expect("create disabled bond");

    mgr.set_snapshot_policy(due_bond.id(), 300, 5, None)
        .expect("set due policy");
    mgr.set_snapshot_policy(future_bond.id(), 300, 5, None)
        .expect("set future policy");
    mgr.set_snapshot_policy(disabled_bond.id(), 300, 5, None)
        .expect("set disabled policy");

    let now = Utc::now();

    // Push this one into the future so it is not due yet.
    mgr.mark_snapshot_policy_success(future_bond.id(), now)
        .expect("mark future success");

    // Disabled policies should never show up as due.
    mgr.disable_snapshot_policy(disabled_bond.id())
        .expect("disable policy");

    let due = mgr
        .list_due_snapshot_policies(now)
        .expect("list due policies");

    assert_eq!(due.len(), 1);
    assert_eq!(due[0].bond_id, due_bond.id());
}

#[test]
#[cfg_attr(windows, ignore)]
fn mark_snapshot_policy_success_sets_last_run_and_clears_previous_error() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();

    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");

    let bond = mgr
        .create_bond(&src_path, &tgt_path, Some("alpha".into()))
        .expect("create bond");

    mgr.set_snapshot_policy(bond.id(), 120, 4, None)
        .expect("set policy");

    let failure_at = fixed_utc("2026-01-02T03:00:00Z");
    mgr.mark_snapshot_policy_failure(bond.id(), "disk full", failure_at)
        .expect("mark failure");

    let success_at = fixed_utc("2026-01-02T03:04:05Z");
    mgr.mark_snapshot_policy_success(bond.id(), success_at)
        .expect("mark success");

    let policy = mgr
        .get_snapshot_policy(bond.id())
        .expect("get policy")
        .expect("policy exists");

    assert_eq!(policy.last_run_at, Some(success_at));
    assert_eq!(
        policy.next_run_at,
        Some(success_at + Duration::seconds(120))
    );
    assert!(policy.last_error.is_none());
    assert!(policy.last_error_at.is_none());
}

#[test]
#[cfg_attr(windows, ignore)]
fn mark_snapshot_policy_failure_records_error_and_reschedules() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();

    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");

    let bond = mgr
        .create_bond(&src_path, &tgt_path, Some("alpha".into()))
        .expect("create bond");

    mgr.set_snapshot_policy(bond.id(), 90, 3, None)
        .expect("set policy");

    let failed_at = fixed_utc("2026-02-10T08:30:15Z");
    mgr.mark_snapshot_policy_failure(bond.id(), "permission denied", failed_at)
        .expect("mark failure");

    let policy = mgr
        .get_snapshot_policy(bond.id())
        .expect("get policy")
        .expect("policy exists");

    assert_eq!(policy.last_run_at, None);
    assert_eq!(policy.last_error.as_deref(), Some("permission denied"));
    assert_eq!(policy.last_error_at, Some(failed_at));
    assert_eq!(policy.next_run_at, Some(failed_at + Duration::seconds(90)));
}

#[test]
#[cfg_attr(windows, ignore)]
fn disable_snapshot_policy_marks_policy_disabled() {
    let mgr = test_manager();
    let (_src_dir, src_path) = temp_source();

    let tgt_dir = TempDir::new().expect("temp dir");
    let tgt_path = tgt_dir.path().join("link");

    let bond = mgr
        .create_bond(&src_path, &tgt_path, Some("alpha".into()))
        .expect("create bond");

    mgr.set_snapshot_policy(bond.id(), 60, 2, None)
        .expect("set policy");
    mgr.disable_snapshot_policy(bond.id())
        .expect("disable policy");

    let policy = mgr
        .get_snapshot_policy(bond.id())
        .expect("get policy")
        .expect("policy exists");

    assert!(!policy.enabled);

    let due = mgr
        .list_due_snapshot_policies(Utc::now())
        .expect("list due policies");

    assert!(due.iter().all(|entry| entry.bond_id != bond.id()));
}
