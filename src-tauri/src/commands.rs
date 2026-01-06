use crate::config::app_config::SourcesConfig;
use crate::platform_ops::{parse_platform, try_convert_with_all_platforms, with_platform_data};
use crate::services::clipboard::{ClipboardManager, SystemClipboard};
use crate::services::link_converter::LinkConverter;
use crate::state::StateManager;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};


static CLIPBOARD_MANAGER: Lazy<Mutex<ClipboardManager<SystemClipboard>>> =
    Lazy::new(|| Mutex::new(ClipboardManager::new()));
static LINK_CONVERTER: Lazy<LinkConverter> = Lazy::new(|| LinkConverter::new());

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn greet_from_app(invoke_message: String) -> () {
    println!("Greet from app: {}", invoke_message);
}

#[tauri::command]
pub fn get_config() -> SourcesConfig {

    use once_cell::sync::Lazy;
    static CONFIG: Lazy<Mutex<SourcesConfig>> = Lazy::new(|| Mutex::new(SourcesConfig::default()));
    CONFIG.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_config(state_manager: tauri::State<StateManager>) -> Result<(), String> {
    state_manager.save_to_file();
    Ok(())
}

#[tauri::command]
pub fn get_state(state_manager: tauri::State<StateManager>) -> SourcesConfig {
    state_manager.get_state()
}

#[tauri::command]
pub fn toggle_platform(
    platform: String,
    enabled: bool,
    state_manager: tauri::State<StateManager>,
) -> Result<(), String> {
    let platform_enum = parse_platform(&platform)
        .ok_or_else(|| format!("Unknown platform: {}", platform))?;

    state_manager.update_state(|state| {
        with_platform_data(state, platform_enum, |data| {
            data.set_enabled(enabled);
        });
    });

    Ok(())
}

#[tauri::command]
pub fn select_converter(
    platform: String,
    converter_name: String,
    state_manager: tauri::State<StateManager>,
) -> Result<(), String> {
    println!("select_converter called with platform: {}, converter_name: {}", platform, converter_name);

    let platform_enum = parse_platform(&platform)
        .ok_or_else(|| format!("Unknown platform: {}", platform))?;

    let success = state_manager.update_state(|state| {
        with_platform_data(state, platform_enum, |data| {
            data.set_converter_by_name(&converter_name)
        }).unwrap_or(false)
    });

    if !success {
        return Err(format!("Failed to set converter '{}' for platform '{}'", converter_name, platform));
    }

    state_manager.save_to_file();
    Ok(())
}

#[tauri::command]
pub fn update_state(
    platform: String,
    enabled: bool,
    selected_converter: Option<String>,
    state_manager: tauri::State<StateManager>,
) -> Result<(), String> {
    let platform_enum = parse_platform(&platform)
        .ok_or_else(|| format!("Unknown platform: {}", platform))?;

    state_manager.update_state(|state| {
        with_platform_data(state, platform_enum, |data| {
            data.set_enabled(enabled);
            if let Some(converter) = &selected_converter {
                data.set_converter_by_name(converter);
            }
        });
    });

    Ok(())
}

#[tauri::command]
pub fn convert_link(url: String, state_manager: tauri::State<StateManager>) -> Result<String, String> {
    let state = state_manager.get_state();
    
    try_convert_with_all_platforms(&state, &LINK_CONVERTER, &url)
        .ok_or_else(|| "Unable to convert link".to_string())
}

#[tauri::command]
pub fn start_clipboard_monitor(state_manager: tauri::State<StateManager>) -> Result<(), String> {
    use std::thread;
    use std::time::Duration;

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
            
            // Try to convert the link with any available platform
            if let Some(converted) = try_convert_with_all_platforms(&state, &LINK_CONVERTER, &content) {
                if let Err(e) = update_clipboard_and_notify(&app_handle, &content, &converted) {
                    eprintln!("Failed to update clipboard: {}", e);
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

    let has_text = clipboard_manager
        .has_text_content()
        .map_err(|e| format!("Failed to check clipboard text content: {}", e))?;
    
    if !has_text {
        return Ok(None);
    }

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