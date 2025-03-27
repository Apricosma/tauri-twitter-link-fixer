// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Manager, Runtime,
};
pub mod services;
use crate::services::clipboard::ClipboardManager;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Create a new clipboard manager instance
static CLIPBOARD_MANAGER: Lazy<Mutex<ClipboardManager>> =
    Lazy::new(|| Mutex::new(ClipboardManager::new()));

// A list of menus in a simple format to pass as args
// to the create_menu function
// menu_id, menu_name, is_enabled, accelerator
// The following functions are for tricking the linter into forcing you to
// implement all the menu items in the MENUS array
const MENUS: [(&str, &str, bool, Option<&str>); 6] = [
    ("quit", "Quit", true, None),
    ("test", "Test", true, None),
    ("hide", "Hide", true, None),
    ("show", "Show", true, None),
    ("set_clipboard", "Set Clipboard", true, None),
    ("get_clipboard", "Get Clipboard", true, None),
];

#[derive(Debug)]
enum MenuId {
    Quit,
    Test,
    Hide,
    Show,
    SetClipboard,
    GetClipboard,
}

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

fn with_clipboard_manager<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce(&mut ClipboardManager) -> Result<R, Box<dyn std::error::Error + Send + Sync>>,
{
    match CLIPBOARD_MANAGER.lock() {
        Ok(mut clipboard_manager) => f(&mut clipboard_manager).map_err(|e| e.to_string()),
        Err(_) => Err("Failed to lock the clipboard manager.".to_string()),
    }
}

fn handle_set_clipboard() {
    println!("Set Clipboard menu item clicked");

    if let Err(e) = with_clipboard_manager(|manager| {
        manager.set_clipboard_content("Hello Clipboard! I live here now.")
    }) {
        println!("Error: {}", e);
    }
}

fn handle_get_clipboard() {
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

fn handle_hide_window<R: Runtime>(app: &AppHandle<R>) {
    with_main_window(app, |window| {
        let _ = window.hide();
    });
}

fn handle_show_window<R: Runtime>(app: &AppHandle<R>) {
    with_main_window(app, |window| {
        let _ = window.show();
    });

}

fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, menu_id_str: &str) {
    match parse_menu_id(menu_id_str) {
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

// Create a generic menu item
fn create_menu_item<R: Runtime>(
    app: &AppHandle<R>,
    menu_id: &str,
    menu_name: &str,
    is_enabled: bool,
    accelerator: Option<&str>,
) -> tauri::Result<MenuItem<R>> {
    let item = MenuItem::with_id(app, menu_id, menu_name, is_enabled, accelerator)?;
    Ok(item)
}

fn build_menu_items<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Vec<MenuItem<R>>> {
    let mut items = Vec::new();

    for (menu_id, menu_name, is_enabled, accelerator) in MENUS.iter() {
        let item = create_menu_item(app, menu_id, menu_name, *is_enabled, *accelerator)?;
        items.push(item);
    }

    Ok(items)
}

fn create_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let menu_items = build_menu_items(app)?;

    let item_refs: Vec<&dyn tauri::menu::IsMenuItem<R>> = menu_items
        .iter()
        .map(|item| item as &dyn tauri::menu::IsMenuItem<R>)
        .collect();

    let menu = Menu::with_items(app, &item_refs)?;
    Ok(menu)
}

fn create_tray<R: Runtime>(app: &AppHandle<R>, menu: &Menu<R>) -> tauri::Result<TrayIcon<R>> {
    let tray = TrayIconBuilder::new()
        .on_menu_event(|app, event| {
            handle_menu_event(app, event.id.as_ref());
        })
        .icon(app.default_window_icon().unwrap().clone())
        .menu(menu)
        .show_menu_on_left_click(true)
        .build(app)?;
    Ok(tray)
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

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
