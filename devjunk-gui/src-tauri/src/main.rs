//! DevJunk GUI - Tauri application entry point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod dto;

use commands::{clean_paths, get_junk_kinds, scan_paths, validate_path};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            scan_paths,
            clean_paths,
            get_junk_kinds,
            validate_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
