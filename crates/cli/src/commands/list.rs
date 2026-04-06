use bonds_core::{BondError, BondManager};

pub fn cmd_list(manager: &BondManager) -> Result<(), BondError> {
    let bonds = manager.list_bonds()?;

    if bonds.is_empty() {
        println!("No bonds found.");
        return Ok(());
    }

    for bond in &bonds {
        let label = match &bond.name {
            Some(name) => format!("{name} ({id})", id = &bond.id[..8]),
            None => bond.id[..8].to_string(),
        };
        println!(
            "{label}  -  {src} -> {tgt}  ({date})",
            src = bond.source.display(),
            tgt = bond.target.display(),
            date = bond.created_at.format("%Y-%m-%d %H:%M"),
        );
    }

    println!("\n{} bond(s) total.", bonds.len());
    Ok(())
}
