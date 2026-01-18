use tauri::Manager;

mod app;
mod commands;
mod config;
mod handlers;
mod platform_ops;
pub mod services;
mod state;
mod tray_menu;

use crate::app::setup_app_exit_handler;
use crate::commands::*;
use crate::config::app_config::SourcesConfig;
use crate::state::StateManager;
use crate::tray_menu::menu::{create_menu, create_tray};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// TODO: Remove this global state in favor of StateManager
static CONFIG: Lazy<Mutex<SourcesConfig>> = Lazy::new(|| Mutex::new(SourcesConfig::default()));

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Get appdata path
            let appdata_path = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data path");

            // Get the app handle
            let handle = app.handle();

            // Create menu for the tray
            let menu = create_menu(&handle)?;

            // create the system tray
            let _tray = create_tray(handle, &menu)?;

            let config_path = appdata_path.join("config.yaml");

            let state_manager = StateManager::new(config_path.display().to_string(), handle.clone());
            state_manager.start_periodic_save();

            app.manage(state_manager);

            // Load or create the configuration
            let config = SourcesConfig::from_file_or_default(
                config_path
                    .to_str()
                    .expect("Failed to convert config path to string"),
            )
            .expect("Failed to load or create configuration file");
            *CONFIG.lock().unwrap() = config;

            // Initialize the exit handler
            setup_app_exit_handler(&handle);

            // Start clipboard monitoring
            let state_manager = app.state::<StateManager>();
            start_clipboard_monitor(state_manager)?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            greet_from_app,
            get_config,
            get_frontend_config,
            update_config,
            get_state,
            update_state,
            toggle_platform,
            select_converter,
            convert_link,
            start_clipboard_monitor,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
