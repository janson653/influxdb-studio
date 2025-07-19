// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



mod commands;
mod models;
mod influxdb;
mod error;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(commands::ConnectionMap::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::test_connection,
            commands::connect_to_database,
            commands::disconnect_from_database,
            commands::get_databases,
            commands::get_database_info,
            commands::execute_query,
            commands::create_database,
            commands::drop_database,
            commands::get_measurements,
            commands::get_app_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 