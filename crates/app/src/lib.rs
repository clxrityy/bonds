mod bond_view;
mod commands;
mod requests;

use crate::commands::{
    create_bond, create_bond_snapshot, delete_bond, delete_bond_snapshot, get_bond_detail,
    list_bond_snapshots, list_bonds, restore_bond_snapshot, update_bond, update_bond_metadata,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            list_bonds,
            get_bond_detail,
            create_bond,
            update_bond,
            update_bond_metadata,
            delete_bond,
            list_bond_snapshots,
            create_bond_snapshot,
            delete_bond_snapshot,
            restore_bond_snapshot
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Bonds desktop app");
}
