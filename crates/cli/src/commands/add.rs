use bonds_core::{BondError, BondManager};
use std::path::{Path, PathBuf};

pub fn cmd_add(
    manager: &BondManager,
    source: PathBuf,
    target: Option<PathBuf>,
    contents: bool,
    name: Option<String>,
) -> Result<(), BondError> {
    let source = source.canonicalize().map_err(|_| {
        BondError::InvalidPath(format!("cannot resolve source: {}", source.display()))
    })?;

    let target = match target {
        Some(t) => t,
        None => {
            let config = bonds_core::BondsConfig::load().unwrap_or_default();
            let name = source
                .file_name()
                .ok_or_else(|| BondError::InvalidPath("source has no file name".into()))?;
            match config.default_target {
                Some(default_dir) => default_dir.join(name),
                None => std::env::current_dir()?.join(name),
            }
        }
    };

    if contents {
        return add_contents(manager, &source, &target);
    }

    let bond = manager.create_bond(&source, &target, name)?;
    println!("Bond created: {}", bond.id);
    println!("  {} -> {}", bond.source.display(), bond.target.display());
    Ok(())
}

/// Bond each child of `source` as a separate bond into `target`.
fn add_contents(
    manager: &BondManager,
    source: &PathBuf,
    target: &Path,
) -> Result<(), BondError> {
    if !source.is_dir() {
        return Err(BondError::InvalidPath(format!(
            "--contents requires a directory, got: {}",
            source.display()
        )));
    }

    let mut created = 0u32;
    let mut failed = 0u32;

    // read_dir returns io::Error, which converts to BondError::Io via #[from]
    let entries = std::fs::read_dir(source)?;

    for entry in entries {
        let entry = entry?;
        let child = entry.path();
        let child_name = match child.file_name() {
            Some(n) => n.to_owned(),
            None => continue, // skip entries without a name (shouldn't happen)
        };

        let child_target = target.join(&child_name);

        match manager.create_bond(&child, &child_target, None) {
            Ok(bond) => {
                println!("  {} -> {}", bond.source.display(), bond.target.display());
                created += 1;
            }
            Err(e) => {
                eprintln!("  skip {}: {}", child_name.to_string_lossy(), e);
                failed += 1;
            }
        }
    }

    println!("\n{created} bond(s) created, {failed} skipped.");

    if created == 0 && failed > 0 {
        return Err(BondError::InvalidPath("no bonds were created".into()));
    }

    Ok(())
}
