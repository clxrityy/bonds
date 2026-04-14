use thiserror::Error;

/// Errors surfaced by the API
#[derive(Error, Debug)]
pub enum BondError {
    /// Filesystem or OS I/O failure.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// SQLite query/connection failure.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// JSON serialization/deserialization failure.
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// A conflicting bond record already exists.
    #[error("Bond already exists")]
    AlreadyExists,

    /// The requested target path already exists and cannot be replaced.
    #[error("Target already exists: {0}")]
    TargetExists(String),

    /// No bond matched the provided identifier.
    #[error("Bond not found: {0}")]
    NotFound(String),

    /// A provided path is invalid or unusable.
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Configuration file parse or write error.
    #[error("config error: {0}")]
    Config(String),

    /// The identifier prefix matched more than one bond.
    #[error("ambiguous identifier '{0}': use more characters")]
    AmbiguousId(String),

    /// Failed to parse or interpret a timestamp.
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),
}
