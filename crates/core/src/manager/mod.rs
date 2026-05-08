use crate::bond::Bond;
use crate::error::BondError;
use crate::events::{BondBrokenReason, BondEvent, BondEventHook, BondEventPayload};
use crate::query::{BondQuery, MetadataFilter};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

mod health;
mod hooks;
mod lifecycle;
mod querying;
mod storage;

#[cfg(test)]
mod tests;

/// SQLite-backed manager for Bonds.
///
/// Keep this file as the module root and shared type definition.
/// Concern-specific method implementations live in sibling modules.
pub struct BondManager {
    pub(super) conn: Connection,
    // Hooks are shared mutable state behind interior mutability since methods take &self.
    pub(super) hooks: RwLock<Vec<Arc<dyn BondEventHook>>>,
}
