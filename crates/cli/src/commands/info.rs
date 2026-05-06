use bonds_cli::ui;
use bonds_core::{BondError, BondManager};

/// Command handler for the `info` command, which displays detailed information about a specific bond identified by its ID. The command retrieves the bond from the bond manager, and if found, it prints out the bond's ID, source path, target path, creation timestamp, and any associated metadata. It also checks the status of the bond's target (whether it exists and is a symlink) and provides a status message indicating whether the bond is healthy, broken, or in a warning state. If the bond cannot be found or an error occurs during retrieval, it returns an appropriate error message to the user.
/// **Example usage:**
/// ```bash
/// bond info <id | name>
/// ```
pub fn cmd_info(manager: &BondManager, id: &str) -> Result<(), BondError> {
    let bond = manager.get_bond(id)?;

    ui::heading("Bond Details:");
    ui::info(format!("  ID:      {}", bond.id()));
    ui::info(format!("  Source:  {}", bond.source().display()));
    ui::info(format!("  Target:  {}", bond.target().display()));
    ui::info(format!(
        "  Created: {}",
        bond.created_at().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    if let Some(meta) = bond.metadata() {
        ui::info("  Metadata:");
        for (k, v) in meta {
            ui::info(format!("    {k}: {v}"));
        }
    }

    let target_exists = bond.target().exists();
    let is_symlink = bond
        .target()
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);

    println!(
        "  Status:  {}",
        match (target_exists, is_symlink) {
            (true, true) => "healthy (symlink intact)",
            (true, false) => "warning: target exists but is not a symlink",
            (false, _) => "broken (target missing)",
        }
    );

    Ok(())
}
