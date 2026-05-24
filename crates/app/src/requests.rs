use serde::Deserialize;

/// Payload for `create_bond` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateBondRequest {
    pub(crate) source: String,
    pub(crate) target: Option<String>,
    pub(crate) name: Option<String>,
}

/// Payload for `update_bond` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateBondRequest {
    pub(crate) id: String,
    pub(crate) source: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) name: Option<String>,
}

/// Payload for `delete_bond` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteBondRequest {
    pub(crate) id: String,
    pub(crate) with_target: bool,
}
