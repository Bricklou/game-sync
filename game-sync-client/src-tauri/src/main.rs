// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod modules;
mod plugins;
mod setup;

fn main() {
    tauri::Builder::default()
        // Logger plugin
        .plugin(plugins::logger::register())
        // Single instance plugin
        .plugin(plugins::single_instance::register())
        // Store plugin
        .plugin(plugins::store::register())
        // Stronghold plugin
        .plugin(plugins::stronghold::register())
        // Setup
        .setup(setup::setup)
        // Invoke handlers
        .invoke_handler(tauri::generate_handler![
            commands::api::get_configured_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
