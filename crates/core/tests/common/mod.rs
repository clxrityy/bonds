use bonds_core::BondManager;
use tempfile::TempDir;

/// Shared integration helper: manager backed by a real file DB in a temp dir.
/// Keep returning TempDir so DB path stays alive for the test duration.
pub fn setup() -> (BondManager, TempDir) {
    let db_dir = TempDir::new().expect("create temp db dir");
    let db_path = db_dir.path().join("test.db");
    let mgr = BondManager::new(Some(db_path)).expect("create bond manager");
    (mgr, db_dir)
}
