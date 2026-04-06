use crate::error::BondError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// User-level configuration for Bonds.
/// Stored at ~/.bonds/config.toml
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BondsConfig {
    /// Where bonds are created when no target is specified.
    /// Defaults to None (falls back to current working directory).
    pub default_target: Option<PathBuf>,
}

impl BondsConfig {
    /// Returns the canonical path to the config file: ~/.bonds/config.toml
    pub fn config_path() -> PathBuf {
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(".bonds")
            .join("config.toml")
    }

    /// Load config from disk; returns Default::default() if the file doesn't exist yet.
    pub fn load() -> Result<Self, BondError> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        // toml::from_str returns a toml::de::Error; map it via BondError
        toml::from_str(&content).map_err(|e| BondError::Config(e.to_string()))
    }

    /// Save config to disk, creating ~/.bonds/ if needed.
    pub fn save(&self) -> Result<(), BondError> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| BondError::Config(e.to_string()))?;
        fs::write(&path, content)?;
        Ok(())
    }
}
