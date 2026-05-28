use bonds_core::Bond;
use serde::Serialize;
use std::collections::HashMap;

/// UI-facing lightweight payload for list/grid views.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BondListItem {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) created_at: String,
    pub(crate) status: String,
    pub(crate) metadata_count: usize,
}

/// UI-facing detail payload for edit/detail views.
/// Includes full metadata map for editing.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BondDetailItem {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) created_at: String,
    pub(crate) status: String,
    pub(crate) metadata: Option<HashMap<String, String>>,
}

/// Map a core `Bond` into a lightweight list item.
pub(crate) fn map_bond(bond: Bond) -> BondListItem {
    BondListItem {
        id: bond.id().to_string(),
        name: bond.name().map(str::to_owned),
        source: bond.source().display().to_string(),
        target: bond.target().display().to_string(),
        created_at: bond.created_at_rfc3339(),
        status: bond_status(&bond),
        metadata_count: bond.metadata().map_or(0, |m| m.len()),
    }
}

/// Map a core `Bond` into a full detail item.
pub(crate) fn map_bond_detail(bond: Bond) -> BondDetailItem {
    BondDetailItem {
        id: bond.id().to_string(),
        name: bond.name().map(str::to_owned),
        source: bond.source().display().to_string(),
        target: bond.target().display().to_string(),
        created_at: bond.created_at_rfc3339(),
        status: bond_status(&bond),
        // Clone so we can serialize and return ownership to the UI.
        metadata: bond.metadata().cloned(),
    }
}

fn bond_status(bond: &Bond) -> String {
    let target_exists = bond.target().exists();
    let is_symlink = bond
        .target()
        .symlink_metadata()
        .map(|meta| meta.file_type().is_symlink())
        .unwrap_or(false);

    match (target_exists, is_symlink) {
        (true, true) => "healthy".to_string(),
        (true, false) => "warning".to_string(),
        (false, _) => "broken".to_string(),
    }
}
