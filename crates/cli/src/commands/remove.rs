use bonds_core::{BondError, BondManager};

pub fn cmd_remove(
    manager: &BondManager,
    id: &str,
    with_target: bool,
) -> Result<(), BondError> {
    let bond = manager.delete_bond(id, with_target)?;
    println!("Bond removed: {}", bond.id);
    println!("  {} -> {}", bond.source.display(), bond.target.display());

    if with_target {
        println!("  Target directory also deleted.");
    }

    Ok(())
}
