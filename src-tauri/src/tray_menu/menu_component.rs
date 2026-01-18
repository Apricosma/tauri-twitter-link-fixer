use tauri::{menu::MenuItem, AppHandle, Runtime};

// Define the menu structure
pub const MENUS: [(&str, &str, bool, Option<&str>); 6] = [
    ("quit", "Quit", true, None),
    ("test", "Test", true, None),
    ("hide", "Hide", true, None),
    ("show", "Show", true, None),
    ("set_clipboard", "Set Clipboard", true, None),
    ("get_clipboard", "Get Clipboard", true, None),
];

pub fn build_menu_items<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Vec<MenuItem<R>>> {
    let mut items = Vec::new();

    for (menu_id, menu_name, is_enabled, accelerator) in MENUS.iter() {
        let item = create_menu_item(app, menu_id, menu_name, *is_enabled, *accelerator)?;
        items.push(item);
    }

    Ok(items)
}

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

pub fn process_menu_event<R: Runtime>(app: &AppHandle<R>, menu_id_str: &str) {
    crate::handlers::handle_menu_event(app, menu_id_str);
}