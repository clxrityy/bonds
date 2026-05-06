use std::collections::HashMap;

use bonds_cli::ui;
use bonds_core::{BondError, BondManager};

pub fn cmd_metadata_get(
    manager: &BondManager,
    id: &str,
    key: Option<&str>,
) -> Result<(), BondError> {
    let bond = manager.get_bond(id)?;

    match key {
        Some(k) => {
            // Single-key read: error if metadata or key is missing.
            let value = bond
                .metadata()
                .and_then(|m| m.get(k))
                .ok_or_else(|| BondError::NotFound(format!("metadata key not found: {k}")))?;

            ui::info(format!("{k}: {value}"));
        }
        None => {
            // Full read: print all keys in deterministic order.
            let Some(meta) = bond.metadata() else {
                ui::warning("No metadata set for this bond.");
                return Ok(());
            };

            if meta.is_empty() {
                ui::warning("No metadata set for this bond.");
                return Ok(());
            }

            let mut entries: Vec<(&String, &String)> = meta.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));

            ui::heading("Metadata:");
            for (k, v) in entries {
                ui::info(format!("  {k}: {v}"));
            }
        }
    }

    Ok(())
}

pub fn cmd_metadata_set(
    manager: &BondManager,
    id: &str,
    key: &str,
    value: &str,
) -> Result<(), BondError> {
    let bond = manager.get_bond(id)?;

    // Upsert behavior: preserve existing map, then overwrite/insert this key.
    let mut metadata: HashMap<String, String> = bond.metadata().cloned().unwrap_or_default();
    metadata.insert(key.to_string(), value.to_string());

    let updated = manager.update_bond_metadata(bond.id(), Some(metadata))?;

    ui::success(format!("Metadata updated for {}", &updated.id()[..8]));
    ui::info(format!("  {key}: {value}"));
    Ok(())
}

pub fn cmd_metadata_remove(manager: &BondManager, id: &str, key: &str) -> Result<(), BondError> {
    let bond = manager.get_bond(id)?;

    let mut metadata = bond
        .metadata()
        .cloned()
        .ok_or_else(|| BondError::NotFound(format!("metadata key not found: {key}")))?;

    if metadata.remove(key).is_none() {
        return Err(BondError::NotFound(format!(
            "metadata key not found: {key}"
        )));
    }

    // Store NULL when map becomes empty to keep DB representation clean.
    let next = if metadata.is_empty() {
        None
    } else {
        Some(metadata)
    };
    let updated = manager.update_bond_metadata(bond.id(), next)?;

    ui::success(format!("Metadata key removed for {}", &updated.id()[..8]));
    ui::info(format!("  removed key: {key}"));
    Ok(())
}
