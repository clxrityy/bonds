#![warn(missing_docs)]

//! # bonds-core
//!
//! Core library for creating and managing bonds (symlink-based directory links)
//! backed by SQLite persistence.
//!
//! ## Quick example
//! ```no_run
//! use bonds_core::{BondError, BondManager};
//! use std::path::PathBuf;
//!
//! // `BondManager::new(None)` uses ~/.bonds/bonds.db by default.
//! let manager = BondManager::new(None)?;
//! let source = PathBuf::from("/path/to/source");
//! let target = PathBuf::from("/path/to/target-link");
//!
//! // This creates a symlink on disk and persists the record in SQLite.
//! let _bond = manager.create_bond(source, target, Some("my-bond".to_string()))?;
//! # Ok::<(), BondError>(())
//! ```

/// Bond domain model types (for example, `Bond`).
pub mod bond;
/// User configuration loading/saving (`~/.bonds/config.toml`).
pub mod config;
/// Public error types returned by this crate.
pub mod error;
/// High-level manager for bond lifecycle operations.
pub mod manager;

pub use bond::Bond;
pub use config::BondsConfig;
pub use error::BondError;
pub use manager::BondManager;
