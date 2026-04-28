use bonds_cli::ui;
use bonds_core::{BondError, BondManager};

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
