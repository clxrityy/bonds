use thiserror::Error;

/// Errors surfaced by the API
#[derive(Error, Debug)]
pub enum BondError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Bond already exists")]
    AlreadyExists,

    #[error("Bond not found: {0}")]
    NotFound(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
