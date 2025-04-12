use config::app_config::{Platform, PlatformConverters, PlatformSource};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{self, AppHandle, Emitter};
mod config;
mod handlers;
pub mod services;
mod tray_menu;

use crate::config::app_config::SourcesConfig;
use crate::services::clipboard::ClipboardManager;
use crate::services::link_converter::LinkConverter;
use crate::tray_menu::menu::{create_menu, create_tray};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
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
fn update_config(state_manager: tauri::State<StateManager>) -> Result<(), String> {
    state_manager.save_to_file();
    Ok(())
}

#[tauri::command]
fn get_state(state_manager: tauri::State<StateManager>) -> SourcesConfig {
    state_manager.get_state()
}

#[tauri::command]
fn toggle_platform(
    platform: String,
    enabled: bool,
    state_manager: tauri::State<StateManager>,
) -> Result<(), String> {
    state_manager.update_state(|state| {
        let platform_enum = match platform.to_lowercase().as_str() {
            "twitter" => Platform::Twitter,
            "bluesky" => Platform::Bluesky,
            _ => return,
        };

        for source in &mut state.sources {
            match (source, &platform_enum) {
                (PlatformSource::Twitter(data), Platform::Twitter) => {
                    data.enabled = enabled;
                }
                (PlatformSource::Bluesky(data), Platform::Bluesky) => {
                    data.enabled = enabled;
                }
                _ => {}
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn select_converter(
    platform: String,
    converter_name: String,
    state_manager: tauri::State<StateManager>,
) -> Result<(), String> {
    println!("select_converter called with platform: {}, converter_name: {}", platform, converter_name);

    state_manager.update_state(|state| {
        let platform_enum = match platform.to_lowercase().as_str() {
            "twitter" => Platform::Twitter,
            "bluesky" => Platform::Bluesky,
            _ => return,
        };

        for source in &mut state.sources {
            match (source, &platform_enum) {
                (PlatformSource::Twitter(data), Platform::Twitter) => {
                    if let Some(found) = data.converters.iter().find(|c| {
                        format!("{:?}", c).to_lowercase() == converter_name.to_lowercase()
                    }) {
                        data.selected = Some(found.clone());
                    }
                }
                (PlatformSource::Bluesky(data), Platform::Bluesky) => {
                    if let Some(found) = data.converters.iter().find(|c| {
                        format!("{:?}", c).to_lowercase() == converter_name.to_lowercase()
                    }) {
                        data.selected = Some(found.clone());
                    }
                }
                _ => {}
            }
        }
    });

    state_manager.save_to_file();

    Ok(())
}

// Create static instances
static CLIPBOARD_MANAGER: Lazy<Mutex<ClipboardManager>> =
    Lazy::new(|| Mutex::new(ClipboardManager::new()));
static LINK_CONVERTER: Lazy<LinkConverter> = Lazy::new(|| LinkConverter::new());
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

pub struct StateManager {
    state: Arc<Mutex<SourcesConfig>>,
    config_path: String,
    app: AppHandle,
}

impl StateManager {
    pub fn new(config_path: String, app: AppHandle) -> Self {
        let state = SourcesConfig::from_file_or_default(&config_path).expect("Failed to load config");

        Self {
            state: Arc::new(Mutex::new(state)),
            config_path,
            app,
        }
    }

    pub fn get_state(&self) -> SourcesConfig {
        self.state.lock().unwrap().clone()
    }

    pub fn update_state<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut SourcesConfig),
    {
        let mut state_guard = self.state.lock().unwrap();
        update_fn(&mut state_guard);
        // Clone the state while holding the lock to avoid deadlock
        let state_clone = state_guard.clone();
        // Drop the lock before emitting
        drop(state_guard);
        // Emit state change event with the cloned state
        if let Err(e) = self.app.emit("state-changed", state_clone) {
            eprintln!("Failed to emit state change: {}", e);
        }
    }

    pub fn save_to_file(&self) {
        let state = self.state.lock().unwrap();
        state.save_to_file(&self.config_path);
    }

    pub fn start_periodic_save(&self) {
        let state = Arc::clone(&self.state);
        let config_path = self.config_path.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(60));
                if let Ok(state) = state.lock() {
                    state.save_to_file(&config_path);
                }
            }
        });
    }
}

#[tauri::command]
fn update_state(
    platform: String,
    enabled: bool,
    selected_converter: Option<String>,
    state_manager: tauri::State<StateManager>,
) -> Result<(), String> {
    state_manager.update_state(|state| {
        let platform_enum = match platform.to_lowercase().as_str() {
            "twitter" => Platform::Twitter,
            "bluesky" => Platform::Bluesky,
            _ => return,
        };

        for source in &mut state.sources {
            match (source, &platform_enum) {
                (PlatformSource::Twitter(data), Platform::Twitter) => {
                    data.enabled = enabled;
                    if let Some(converter) = selected_converter.clone() {
                        if let Some(found) = data.converters.iter().find(|c| {
                            format!("{:?}", c).to_lowercase() == converter.to_lowercase()
                        }) {
                            data.selected = Some(found.clone());
                        }
                    }
                }
                (PlatformSource::Bluesky(data), Platform::Bluesky) => {
                    data.enabled = enabled;
                    if let Some(converter) = selected_converter.clone() {
                        if let Some(found) = data.converters.iter().find(|c| {
                            format!("{:?}", c).to_lowercase() == converter.to_lowercase()
                        }) {
                            data.selected = Some(found.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn start_clipboard_monitor(state_manager: tauri::State<StateManager>) -> Result<(), String> {
    let app_handle = state_manager.app.clone();
    
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));
            
            // Get clipboard content if changed
            let content = match get_new_clipboard_content() {
                Ok(Some(content)) => content,
                Ok(None) => continue, // No changes
                Err(e) => {
                    eprintln!("Clipboard error: {}", e);
                    continue;
                }
            };

            let state = app_handle.state::<StateManager>().get_state();
            
            // Try each platform's conversion
            for source in state.sources.iter() {
                match source {
                    PlatformSource::Twitter(data) if data.enabled => {
                        if let Some(selected) = &data.selected {
                            if let Some(converted) = LINK_CONVERTER.convert_link(&content, "twitter", &format!("{:?}", selected).to_lowercase()) {
                                if let Err(e) = update_clipboard_and_notify(&app_handle, &content, &converted) {
                                    eprintln!("Failed to update clipboard: {}", e);
                                }
                                break;
                            }
                        }
                    },
                    PlatformSource::Bluesky(data) if data.enabled => {
                        if let Some(selected) = &data.selected {
                            if let Some(converted) = LINK_CONVERTER.convert_link(&content, "bluesky", &format!("{:?}", selected).to_lowercase()) {
                                if let Err(e) = update_clipboard_and_notify(&app_handle, &content, &converted) {
                                    eprintln!("Failed to update clipboard: {}", e);
                                }
                                break;
                            }
                        }
                    },
                    _ => continue,
                }
            }
        }
    });

    Ok(())
}

fn get_new_clipboard_content() -> Result<Option<String>, String> {
    let mut clipboard_manager = CLIPBOARD_MANAGER
        .lock()
        .map_err(|e| format!("Failed to lock clipboard manager: {}", e))?;

    let content = clipboard_manager
        .get_clipboard_content()
        .map_err(|e| format!("Failed to get clipboard content: {}", e))?;

    if clipboard_manager
        .has_clipboard_changed()
        .map_err(|e| format!("Failed to check clipboard changes: {}", e))? {
        Ok(Some(content))
    } else {
        Ok(None)
    }
}

fn get_twitter_config(state: &SourcesConfig) -> Option<&PlatformConverters<config::app_config::TwitterConverters>> {
    state.sources.iter()
        .find(|s| matches!(s, PlatformSource::Twitter(_)))
        .and_then(|s| match s {
            PlatformSource::Twitter(data) => Some(data),
            _ => None,
        })
}

fn update_clipboard_and_notify(app_handle: &AppHandle, original: &str, converted: &str) -> Result<(), String> {
    let mut clipboard_manager = CLIPBOARD_MANAGER
        .lock()
        .map_err(|e| format!("Failed to lock clipboard manager: {}", e))?;

    clipboard_manager
        .set_clipboard_content(converted)
        .map_err(|e| format!("Failed to set clipboard content: {}", e))?;

    app_handle
        .emit(
            "link-converted",
            serde_json::json!({
                "original": original,
                "converted": converted
            }),
        )
        .map_err(|e| format!("Failed to emit conversion event: {}", e))?;

    Ok(())
}

#[tauri::command]
fn convert_link(url: String, state_manager: tauri::State<StateManager>) -> Result<String, String> {
    let state = state_manager.get_state();
    
    // Try each platform's conversion
    for source in state.sources.iter() {
        match source {
            PlatformSource::Twitter(data) if data.enabled => {
                if let Some(selected) = &data.selected {
                    if let Some(converted) = LINK_CONVERTER.convert_link(&url, "twitter", &format!("{:?}", selected).to_lowercase()) {
                        return Ok(converted);
                    }
                }
            },
            PlatformSource::Bluesky(data) if data.enabled => {
                if let Some(selected) = &data.selected {
                    if let Some(converted) = LINK_CONVERTER.convert_link(&url, "bluesky", &format!("{:?}", selected).to_lowercase()) {
                        return Ok(converted);
                    }
                }
            },
            _ => continue,
        }
    }
    
    Err("Unable to convert link".to_string())
}

fn setup_app_exit_handler(app: &AppHandle) {
    let app_handle = app.clone();
    let window = app.get_webview_window("main").expect("Failed to get main window");

    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            println!("Saving config before exit...");

            let state_manager = app_handle.state::<StateManager>();

            // Block closing temporarily
            api.prevent_close();

            // Save the state
            state_manager.save_to_file();

            // Optional sleep if you want absolute guarantee
            std::thread::sleep(std::time::Duration::from_millis(100));

            // 👇 Correct safe way to close:
            app_handle.exit(0);
        }
    });
}




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
