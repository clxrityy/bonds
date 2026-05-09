use bonds_cli::ui;
use bonds_core::{BondError, BondManager};

/// Command handler for the `list` command, which retrieves and displays a list of all existing bonds managed by the bond manager. The command formats the output to show each bond's ID, source path, target path, and creation date in a user-friendly manner. If no bonds are found, it provides a warning message to the user. This command allows users to quickly see an overview of all their bonds and their key details.
/// **Example usage:**
/// ```bash
/// bond list
/// ```
pub fn cmd_list(manager: &BondManager) -> Result<(), BondError> {
    let bonds = manager.list_bonds()?;

    ui::title("All Bonds:");

    if bonds.is_empty() {
        ui::warning("⚠ No bonds found.");
        ui::newline();
        return Ok(());
    }

    for bond in &bonds {
        if bonds.len() - 1 == 0 {
            ui::subheading("----------------");
        }
        let label = match bond.name() {
            Some(name) => format!("{name} ({id})", id = &bond.id()[..8]),
            None => bond.id()[..8].to_string(),
        };
        // ui::info(format!(
        //     "{label}  -  {src} -> {tgt}  ({date})",
        //     src = bond.source().display(),
        //     tgt = bond.target().display(),
        //     date = bond.created_at().format("%Y-%m-%d %H:%M"),
        // ));
        let src = format!("{}", bond.source().display());
        let tgt = format!("{}", bond.target().display());
        let date = format!("{}", bond.created_at().format("%Y-%m-%d %H:%M"));
        ui::info(format!("{label}   -"));
        ui::key(format!("   ⑂   {}", src));
        ui::path(format!("   ⑃   {}", tgt));
        ui::dim(format!("   {}", date));
        ui::subheading("----------------");
    }

    ui::info(format!("\n{} bond(s) total.", bonds.len()));
    Ok(())
}
