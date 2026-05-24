mod bond_view;
mod commands;
mod requests;

use crate::commands::{create_bond, delete_bond, list_bonds, update_bond};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            list_bonds,
            create_bond,
            update_bond,
            delete_bond
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Bonds desktop app");
}
