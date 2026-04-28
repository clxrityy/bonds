use bonds_cli::ui;
use bonds_core::{BondError, BondManager};

pub fn cmd_remove(manager: &BondManager, id: &str, with_target: bool) -> Result<(), BondError> {
    let bond = manager.delete_bond(id, with_target)?;
    ui::success(format!("Bond removed: {}", bond.id()));
    ui::info(format!(
        "  {} -> {}",
        bond.source().display(),
        bond.target().display()
    ));
    ui::info(format!(
        "  {} -> {}",
        bond.source().display(),
        bond.target().display()
    ));

    if with_target {
        ui::info("  Target directory also deleted.");
    }

    Ok(())
}
