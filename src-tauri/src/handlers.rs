use crate::services::clipboard::{ClipboardManager, SystemClipboard};
use crate::tray_menu::MenuId;
use tauri::{AppHandle, Manager, Runtime};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Local clipboard manager for handlers
static CLIPBOARD_MANAGER: Lazy<Mutex<ClipboardManager<SystemClipboard>>> =
    Lazy::new(|| Mutex::new(ClipboardManager::new()));

pub fn handle_set_clipboard() {
    println!("Set Clipboard menu item clicked");

    if let Err(e) = with_clipboard_manager(|manager| {
        manager.set_clipboard_content("Hello Clipboard! I live here now.")
    }) {
        println!("Error: {}", e);
    }
}

pub fn handle_get_clipboard() {
    println!("Get Clipboard menu item clicked");

    if let Err(e) = with_clipboard_manager(|manager| match manager.get_clipboard_content() {
        Ok(clipboard_content) => {
            println!("Clipboard content: {}", clipboard_content);
            Ok(())
        }
        Err(e) => {
            println!("Failed to get clipboard content: {}", e);
            Err(e)
        }
    }) {
        println!("Error: {}", e);
    }
}

pub fn handle_hide_window<R: Runtime>(app: &AppHandle<R>) {
    with_main_window(app, |window| {
        let _ = window.hide();
    });
}

pub fn handle_show_window<R: Runtime>(app: &AppHandle<R>) {
    with_main_window(app, |window| {
        let _ = window.show();
    });
}

pub fn is_window_visible<R: Runtime>(app: &AppHandle<R>) -> bool {
    if let Some(window) = app.get_webview_window("main") {
        window.is_visible().unwrap_or(false)
    } else {
        println!("Main window not found.");
        false
    }
}

pub fn window_visibility_manager<R: Runtime>(app: &AppHandle<R>) {
    if !is_window_visible(app) {
        handle_show_window(app);
        return;
    }

    handle_hide_window(app);
}

pub fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, menu_id_str: &str) {
    match MenuId::parse_menu_id(menu_id_str) {
        Ok(menu_id) => match menu_id {
            MenuId::Test => {
                println!("Test menu item clicked");
            }
            MenuId::SetClipboard => {
                handle_set_clipboard();
            }
            MenuId::GetClipboard => {
                handle_get_clipboard();
            }
            MenuId::Hide => {
                handle_hide_window(app);
            }
            MenuId::Show => {
                handle_show_window(app);
            }
            MenuId::Quit => {
                println!("Quit menu item clicked");
                app.exit(0);
            }
        },
        Err(e) => {
            println!("Warning: {}", e); // Fallback handler for unlisted IDs
        }
    }
}

// Helper function to safely access the clipboard manager
fn with_clipboard_manager<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce(&mut ClipboardManager<SystemClipboard>) -> Result<R, Box<dyn std::error::Error + Send + Sync>>,
{
    match CLIPBOARD_MANAGER.lock() {
        Ok(mut clipboard_manager) => f(&mut clipboard_manager).map_err(|e| e.to_string()),
        Err(_) => Err("Failed to lock the clipboard manager.".to_string()),
    }
}

// Helper function to safely access the main window
fn with_main_window<R: Runtime, F>(app: &AppHandle<R>, operation: F)
where
    F: FnOnce(&tauri::WebviewWindow<R>),
{
    if let Some(window) = app.get_webview_window("main") {
        operation(&window);
    } else {
        println!("Main window not found.");
    }
}