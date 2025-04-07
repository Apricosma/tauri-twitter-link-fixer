use config::app_config::{Platform, PlatformSource};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri;
mod handlers;
pub mod services;
mod tray_menu;
mod config;

use crate::services::clipboard::ClipboardManager;
use crate::tray_menu::menu::{create_menu, create_tray};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::config::app_config::SourcesConfig;
use tauri::Manager;

// use handlers::handle_menu_event;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn greet_from_app(invoke_message: String) -> () {
    println!("Greet from app: {}", invoke_message);
}

#[tauri::command]
fn get_config() -> SourcesConfig {
    CONFIG.lock().unwrap().clone()
}

#[tauri::command]
fn update_config(platform: String, enabled: bool, selected_converter: Option<String>) -> Result<(), String> {
    let mut config = CONFIG.lock().unwrap();

    let platform_enum = match platform.to_lowercase().as_str() {
        "twitter" => Platform::Twitter,
        "bluesky" => Platform::Bluesky,
        _ => return Err(format!("Unknown platform: {}", platform)),
    };

    for source in &mut config.sources {
        match (source, &platform_enum) {
            (PlatformSource::Twitter(data), Platform::Twitter) => {
                data.enabled = enabled;
                if let Some(converter) = selected_converter.clone() {
                    if let Some(found) = data.converters.iter().find(|c| format!("{:?}", c).to_lowercase() == converter.to_lowercase()) {
                        data.selected = Some(found.clone());
                    } else {
                        return Err(format!("Selected converter '{}' not found in Twitter converters", converter));
                    }
                }
            }
            (PlatformSource::Bluesky(data), Platform::Bluesky) => {
                data.enabled = enabled;
                if let Some(converter) = selected_converter.clone() {
                    if let Some(found) = data.converters.iter().find(|c| format!("{:?}", c).to_lowercase() == converter.to_lowercase()) {
                        data.selected = Some(found.clone());
                    } else {
                        return Err(format!("Selected converter '{}' not found in Bluesky converters", converter));
                    }
                }
            }
            _ => {}
        }
    }

    config.save_to_file("config.yaml"); // Save immediately
    Ok(())
}


// Create a new clipboard manager instance
static CLIPBOARD_MANAGER: Lazy<Mutex<ClipboardManager>> =
    Lazy::new(|| Mutex::new(ClipboardManager::new()));

static CONFIG: Lazy<Mutex<SourcesConfig>> = Lazy::new(|| Mutex::new(SourcesConfig::default()));

#[derive(Debug)]
enum MenuId {
    Quit,
    Test,
    Hide,
    Show,
    SetClipboard,
    GetClipboard,
}

impl MenuId {
    fn parse_menu_id(id: &str) -> Result<MenuId, String> {
        match id {
            "quit" => Ok(MenuId::Quit),
            "test" => Ok(MenuId::Test),
            "hide" => Ok(MenuId::Hide),
            "show" => Ok(MenuId::Show),
            "set_clipboard" => Ok(MenuId::SetClipboard),
            "get_clipboard" => Ok(MenuId::GetClipboard),
            _ => Err(format!("Unknown menu id: {}", id)),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the app handle
            let handle = app.handle();

            // Create menu for the tray
            let menu = create_menu(&handle)?;

            // create the system tray
            let _tray = create_tray(handle, &menu)?;

            let config_path = "config.yaml";

            // Load or create the configuration
            let config = SourcesConfig::from_file_or_default(config_path)
                .expect("Failed to load or create configuration file");
            *CONFIG.lock().unwrap() = config;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            greet_from_app,
            get_config,
            update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
