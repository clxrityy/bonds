#![warn(missing_docs)]

//! # bonds-core
//!
//! Core library for creating and managing bonds (symlink-based directory links)
//! backed by SQLite persistence.

pub mod bond;
pub mod config;
pub mod error;
pub mod manager;

pub use bond::Bond;
pub use config::BondsConfig;
pub use error::BondError;
pub use manager::BondManager;
