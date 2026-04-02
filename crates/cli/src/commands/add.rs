use bonds_core::{BondError, BondManager};
use std::path::PathBuf;

pub fn cmd_add(
    manager: &BondManager,
    source: PathBuf,
    target: Option<PathBuf>,
) -> Result<(), BondError> {
    let source = source.canonicalize().map_err(|_| {
        BondError::InvalidPath(format!("cannot resolve source: {}", source.display()))
    })?;

    let target = match target {
        Some(t) => t,
        None => {
            let name = source
                .file_name()
                .ok_or_else(|| BondError::InvalidPath("source has no file name".into()))?;
            std::env::current_dir()?.join(name)
        }
    };

    let bond = manager.create_bond(&source, &target)?;
    println!("Bond created: {}", bond.id);
    println!("  {} -> {}", bond.source.display(), bond.target.display());
    Ok(())
}
