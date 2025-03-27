use crate::tray_menu::menu_component::{build_menu_items, process_menu_event};
use tauri::{
    menu::Menu,
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Runtime,
};

pub fn create_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let menu_items = build_menu_items(app)?;

    let item_refs: Vec<&dyn tauri::menu::IsMenuItem<R>> = menu_items
        .iter()
        .map(|item| item as &dyn tauri::menu::IsMenuItem<R>)
        .collect();

    let menu = Menu::with_items(app, &item_refs)?;
    Ok(menu)
}

pub fn create_tray<R: Runtime>(app: &AppHandle<R>, menu: &Menu<R>) -> tauri::Result<TrayIcon<R>> {
    let tray = TrayIconBuilder::new()
        .on_menu_event(|app, event| {
            process_menu_event(app, event.id.as_ref());
        })
        .icon(app.default_window_icon().unwrap().clone())
        .menu(menu)
        .show_menu_on_left_click(true)
        .build(app)?;
    Ok(tray)
}
