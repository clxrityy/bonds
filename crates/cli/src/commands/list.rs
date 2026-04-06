use bonds_core::{BondError, BondManager};

pub fn cmd_list(manager: &BondManager) -> Result<(), BondError> {
    let bonds = manager.list_bonds()?;

    if bonds.is_empty() {
        println!("No bonds found.");
        return Ok(());
    }

    for bond in &bonds {
        let display_name = bond.name.as_deref().unwrap_or("-");
        println!(
            "{id}  {name}  {src} -> {tgt}  ({date})",
            id = &bond.id[..8],
            name = display_name,
            src = bond.source.display(),
            tgt = bond.target.display(),
            date = bond.created_at.format("%Y-%m-%d %H:%M"),
        );
    }

    println!("\n{} bond(s) total.", bonds.len());
    Ok(())
}
