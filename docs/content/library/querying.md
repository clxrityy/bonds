# Bonds | Querying

[`bonds-core`](https://bonds.fyi/latest/api/bonds_core/) provides composable query filtering through `BondQuery` plus convenience methods on `BondManager`.

## Quick examples

```rust
use bonds_core::{BondManager, BondQuery};

let manager = BondManager::new(None)?;

// Query by source
let by_source = manager.query_by_source("/path/to/source")?;

// Query by target
let by_target = manager.query_by_target("/path/to/target-link")?;

// Query by metadata key
let tagged = manager.query_by_metadata_key("project")?;

// Query by metadata key/value
let alpha = manager.query_by_metadata("project", "alpha")?;

// Combine filters (AND semantics)
let combined = manager.query_bonds(
    &BondQuery::new()
        .with_source("/path/to/source")
        .with_metadata("project", "alpha"),
)?;
# Ok::<(), bonds_core::BondError>(())
```
