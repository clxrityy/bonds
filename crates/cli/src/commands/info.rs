use bonds_cli::ui::{self};
use bonds_core::{BondError, BondManager};

/// Command handler for the `info` command, which displays detailed information about a specific bond identified by its ID. The command retrieves the bond from the bond manager, and if found, it prints out the bond's ID, source path, target path, creation timestamp, and any associated metadata. It also checks the status of the bond's target (whether it exists and is a symlink) and provides a status message indicating whether the bond is healthy, broken, or in a warning state. If the bond cannot be found or an error occurs during retrieval, it returns an appropriate error message to the user.
/// **Example usage:**
/// ```bash
/// bond info <id | name>
/// ```
pub fn cmd_info(manager: &BondManager, id: &str) -> Result<(), BondError> {
    let bond = manager.get_bond(id)?;

    ui::title("Bond Details:");
    ui::newline();

    // Dispaly status
    let target_exists = bond.target().exists();
    let is_symlink = bond
        .target()
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);

    let (status_number, status_msg) = match (target_exists, is_symlink) {
        (true, true) => (0, "healthy (symlink intact)"),
        (true, false) => (1, "warning: target exists but is not a symlink"),
        (false, _) => (2, "broken (target missing)"),
    };

    status(status_number, status_msg);

    ui::heading("  ID:");
    ui::id(format!("     {}", bond.id()));
    ui::heading("  Source:");

    ui::key(format!("     {}", bond.source().display()));
    ui::heading("  Target:");
    ui::path(format!("     {}", bond.target().display()));
    ui::heading("  Created:");
    ui::dim(format!(
        "     {}",
        bond.created_at().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    if let Some(meta) = bond.metadata() {
        ui::heading("  Metadata:");
        for (k, v) in meta {
            ui::subheading(format!("    {k}:"));
            ui::dim(format!("       {v}"));
        }
    }

    Ok(())
}

fn status(status_number: i32, status: &str) {
    if status_number == 0 {
        ui::status_ok("  STATUS: ✓");
        ui::success(format!("     {}", status));
    } else if status_number == 1 {
        ui::status_warn("  STATUS: ⚠");
        ui::warning(format!("     {}", status));
    } else {
        ui::status_bad("  STATUS: ✗");
        ui::error(format!("     {}", status));
    }
    ui::newline();
}
