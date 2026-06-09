mod add;
/// Handler for the `add` CLI command to create new bonds.
mod config;
/// Handler for the `update` CLI command to modify the source/target paths or name of an existing bond.
mod history;
/// Handlers for the `config` CLI subcommands to get/set user configuration.
mod info;
/// Handler for the `info` CLI command to display details of a specific bond.
mod list;
/// Handler for the `list` CLI command to display all existing bonds.
mod metadata;
/// Handlers for the `metadata` CLI subcommands to manage bond metadata key/value pairs.
mod migrate;
/// Handler for the `migrate` CLI command to perform database schema migrations.
mod remove;
/// Handler for the `remove` CLI command to delete existing bonds.
mod update;
/// Handler for the `history` CLI command to manage bond snapshotting and restoration history.
pub use add::cmd_add;
pub use config::{cmd_config_get, cmd_config_set};
pub use history::{
    cmd_history_delete, cmd_history_disable, cmd_history_enable, cmd_history_list,
    cmd_history_restore, cmd_history_snapshot, cmd_history_watch,
};
pub use info::cmd_info;
pub use list::cmd_list;
pub use metadata::{cmd_metadata_get, cmd_metadata_remove, cmd_metadata_set};
pub use migrate::cmd_migrate;
pub use remove::cmd_remove;
pub use update::cmd_update;
