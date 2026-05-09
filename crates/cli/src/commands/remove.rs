use bonds_cli::ui;
use bonds_core::{BondError, BondManager};

/// Remove command handler, which deletes a bond by its ID or name. The command retrieves the bond, deletes it using the bond manager's `delete_bond` method, and provides user feedback on the success of the operation, including details of the removed bond's source and target paths. If the `with_target` flag is set, it also deletes the target directory associated with the bond.
/// **Example usage:**
/// ```bash
/// bond remove <id | name> [--with-target]
/// ```
pub fn cmd_remove(manager: &BondManager, id: &str, with_target: bool) -> Result<(), BondError> {
    let bond = manager.delete_bond(id, with_target)?;
    ui::status_ok("✓ SUCCESS");
    ui::success("Bond removed:");
    ui::id(format!("   {}", bond.id()));
    ui::dim(format!("   {}", bond.source().display()));
    ui::normal("    ⤡ ");
    ui::dim(format!("    {}\n", bond.target().display()));

    if with_target {
        ui::info("  Target directory also deleted.");
    }

    Ok(())
}
