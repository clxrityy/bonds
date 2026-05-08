use super::*;

/// Health-scan concerns.
impl BondManager {
    /// Return broken bonds and emit BrokenDetected event for each.
    pub fn scan_broken_bonds(&self) -> Result<Vec<Bond>, BondError> {
        let bonds = self.list_bonds()?;
        let mut broken = Vec::new();

        for bond in bonds {
            if let Some(reason) = Self::broken_reason(&bond) {
                self.emit_event(BondEventPayload::BrokenDetected {
                    bond: bond.clone(),
                    reason,
                });
                broken.push(bond);
            }
        }

        Ok(broken)
    }

    fn broken_reason(bond: &Bond) -> Option<BondBrokenReason> {
        // exists() follows symlinks: broken symlinks report false.
        let target_exists = bond.target().exists();

        // symlink_metadata() inspects the link itself.
        let is_symlink = bond
            .target()
            .symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);

        if !target_exists {
            Some(BondBrokenReason::MissingTarget)
        } else if !is_symlink {
            Some(BondBrokenReason::TargetNotSymlink)
        } else {
            None
        }
    }
}
