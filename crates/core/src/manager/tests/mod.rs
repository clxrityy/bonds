use super::BondManager;
use crate::events::{BondEvent, BondEventHook};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use tempfile::TempDir;

mod history;
mod hooks;
mod lifecycle;
mod querying;
mod storage;

/// Shared hook collector used across hook-related tests.
#[derive(Default)]
struct TestHook {
    events: Mutex<Vec<BondEvent>>,
}

impl BondEventHook for TestHook {
    fn on_event(&self, event: &BondEvent) {
        self.events.lock().expect("events lock").push(event.clone());
    }
}

/// Shared helper: in-memory manager.
fn test_manager() -> BondManager {
    let conn = Connection::open_in_memory().expect("in-memory DB");
    BondManager::from_connection(conn).expect("manager from connection")
}

/// Shared helper: create source temp path and keep owner alive.
fn temp_source() -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("temp source");
    let path = dir.path().to_path_buf();
    (dir, path)
}
