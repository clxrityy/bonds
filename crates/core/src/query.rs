use std::path::{Path, PathBuf};

/// Metadata matching rules for bond queries.
/// - `HasKey`: matches bonds that contain a metadata key, regardless of value.
/// - `KeyValue`: matches bonds where `key == value`.
/// **Example usage:**
/// ```rust
/// let filter = MetadataFilter::HasKey("env".to_string());
/// let filter = MetadataFilter::KeyValue { key: "env".to_string(), value: "production".to_string() };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetadataFilter {
    /// Match bonds that contain a metadata key, regardless of value.
    HasKey(String),
    /// Match bonds where `key == value`.
    KeyValue { key: String, value: String },
}

/// Composable filter object for querying bonds.
///
/// All fields are optional. When multiple fields are set, they are combined with AND logic.
/// - `source` and `target` are matched with exact path equality.
/// - `metadata` is matched according to the rules defined in `MetadataFilter`.
/// **Example usage:**
/// ```rust
/// let query = BondQuery::new()
///     .with_source("/path/to/source")
///     .with_metadata("env", "production");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BondQuery {
    pub source: Option<PathBuf>,
    pub target: Option<PathBuf>,
    pub metadata: Option<MetadataFilter>,
}

impl BondQuery {
    /// Start with an empty query (matches all bonds).
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an exact source-path filter.
    pub fn with_source<P: AsRef<Path>>(mut self, source: P) -> Self {
        self.source = Some(source.as_ref().to_path_buf());
        self
    }

    /// Add an exact target-path filter.
    pub fn with_target<P: AsRef<Path>>(mut self, target: P) -> Self {
        self.target = Some(target.as_ref().to_path_buf());
        self
    }

    /// Add a metadata key-exists filter.
    pub fn with_metadata_key(mut self, key: impl Into<String>) -> Self {
        self.metadata = Some(MetadataFilter::HasKey(key.into()));
        self
    }

    /// Add a metadata key/value filter.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata = Some(MetadataFilter::KeyValue {
            key: key.into(),
            value: value.into(),
        });
        self
    }
}
