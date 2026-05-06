use thiserror::Error;

/// Coarse error categories used by the CLI to pick a color.
/// This is not an exhaustive list of all error cases, just broad buckets for consistent CLI rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// User provided invalid or unusable data (e.g. path, timestamp, identifier).
    Input,
    /// No bond matched the provided identifier or path.
    NotFound,
    /// A bond already exists for the provided identifier or target path.
    Conflict,
    /// An unexpected error occurred at runtime (e.g. IO, database, serialization).
    Runtime,
    /// Configuration file parse or write error.
    Config,
}

/// Errors surfaced by the API
/// This is the main error type returned by all public methods in this crate. It wraps various underlying error types (IO, SQLite, serialization) as well as domain-specific errors like NotFound or AlreadyExists. The `kind()` method allows callers to categorize errors for consistent CLI rendering.
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

impl BondError {
    /// Return a broad category so the CLI can render the error consistently.
    /// This is not meant to be an exhaustive categorization of all error cases, just a few buckets that map to different CLI colors and exit codes.
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::InvalidPath(_) | Self::InvalidTimestamp(_) | Self::AmbiguousId(_) => {
                ErrorKind::Input
            }
            Self::NotFound(_) => ErrorKind::NotFound,
            Self::AlreadyExists | Self::TargetExists(_) => ErrorKind::Conflict,
            Self::Io(_) | Self::Sqlite(_) | Self::Serde(_) => ErrorKind::Runtime,
            Self::Config(_) => ErrorKind::Config,
        }
    }
}
