use crate::{
    handlers::window_visibility_manager,
    tray_menu::menu_component::{build_menu_items, process_menu_event},
};
use tauri::{
    menu::Menu,
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
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

fn handle_tray_icon_click_event<R: Runtime>(tray: &TrayIcon<R>, event: tauri::tray::TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            id: _,
            position: _,
            rect: _,
        } => {
            window_visibility_manager(tray.app_handle());
        }
        _ => {
            println!("unhandled event {event:?}");
        }
    }
}

pub fn create_tray<R: Runtime>(app: &AppHandle<R>, menu: &Menu<R>) -> tauri::Result<TrayIcon<R>> {
    let tray = TrayIconBuilder::new()
        .on_tray_icon_event(|tray, event| {
            handle_tray_icon_click_event(tray, event);
        })
        .on_menu_event(|app, event| {
            process_menu_event(app, event.id.as_ref());
        })
        .icon(app.default_window_icon().unwrap().clone())
        .menu(menu)
        .show_menu_on_left_click(false)
        .build(app)?;
    Ok(tray)
}
