use super::*;

/// Query and filtering concerns.
impl BondManager {
    /// Query bonds using optional source/target/metadata filters (AND semantics).
    pub fn query_bonds(&self, query: &BondQuery) -> Result<Vec<Bond>, BondError> {
        let bonds = self.list_bonds()?;

        let filtered = bonds
            .into_iter()
            .filter(|bond| {
                if let Some(source) = query.source.as_ref()
                    && bond.source() != source.as_path()
                {
                    return false;
                }

                if let Some(target) = query.target.as_ref()
                    && bond.target() != target.as_path()
                {
                    return false;
                }

                if let Some(meta_filter) = query.metadata.as_ref() {
                    let Some(metadata) = bond.metadata() else {
                        return false;
                    };

                    match meta_filter {
                        MetadataFilter::HasKey(key) => metadata.contains_key(key),
                        MetadataFilter::KeyValue { key, value } => {
                            metadata.get(key).map(|v| v.as_str()) == Some(value.as_str())
                        }
                    }
                } else {
                    true
                }
            })
            .collect();

        Ok(filtered)
    }

    pub fn query_by_source<P: AsRef<Path>>(&self, source: P) -> Result<Vec<Bond>, BondError> {
        self.query_bonds(&BondQuery::new().with_source(source))
    }

    pub fn query_by_target<P: AsRef<Path>>(&self, target: P) -> Result<Vec<Bond>, BondError> {
        self.query_bonds(&BondQuery::new().with_target(target))
    }

    pub fn query_by_metadata_key(&self, key: &str) -> Result<Vec<Bond>, BondError> {
        self.query_bonds(&BondQuery::new().with_metadata_key(key))
    }

    pub fn query_by_metadata(&self, key: &str, value: &str) -> Result<Vec<Bond>, BondError> {
        self.query_bonds(&BondQuery::new().with_metadata(key, value))
    }
}
