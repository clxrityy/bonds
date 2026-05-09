use std::collections::HashMap;

use bonds_cli::ui;
use bonds_core::{BondError, BondManager};

/// Metadata command handler for the `get` action, which retrieves and displays metadata associated with a specific bond. If a metadata key is provided, it retrieves the value for that key; otherwise, it lists all metadata key-value pairs for the bond. The command handles cases where metadata is not set or when a specific key is not found, providing appropriate user feedback in each case.
/// **Example usage:**
/// ```bash
/// bond metadata get <id | name> [key]
/// ```
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

            // ui::info(format!("{k}: {value}"));
            ui::title("Metadata:");
            ui::subheading(format!("  {k}:"));
            ui::dim(format!("    {value}"));
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
                // ui::info(format!("  {k}: {v}"));
                ui::subheading(format!("    {k}:"));
                ui::dim(format!("       {v}"));
            }
        }
    }

    Ok(())
}

/// Metadata command handler for the `set` action, which updates or adds a metadata key-value pair for a specific bond. The command retrieves the bond, updates the metadata with the provided key and value, and saves the changes using the bond manager. It provides user feedback on the success of the operation and displays the updated metadata key and value.
/// **Example usage:**
/// ```bash
/// bond metadata set <id | name> <key> <value>
/// ```
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

    ui::status_ok("✓ SUCCESS");
    ui::success("Metadata updated for bond:");
    ui::id(format!("   {}", &updated.id()[..8]));
    // ui::info(format!("  {key}: {value}"));
    ui::subheading(format!("  {key}:"));
    ui::dim(format!("     {value}"));
    Ok(())
}

/// Metadata command handler for the `remove` action, which deletes a specific metadata key from a bond. The command retrieves the bond, checks for the existence of the metadata key, and removes it if found. If the key is not found, it returns an error. After removal, it updates the bond's metadata and provides user feedback on the success of the operation, including the removed key. If the metadata map becomes empty after removal, it stores `None` to keep the database representation clean.
/// **Example usage:**
/// ```bash
/// bond metadata remove <id | name> <key>
/// ```
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

    ui::status_ok("✓ SUCCESS");
    ui::success("Metadata key removed for bond:");
    ui::id(format!("   {}", &updated.id()[..8]));
    ui::subheading(format!("  removed key: {key}"));
    Ok(())
}
