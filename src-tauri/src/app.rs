use tauri::{AppHandle, Manager};

pub fn setup_app_exit_handler(app: &AppHandle) {
    use crate::state::StateManager;

    let app_handle = app.clone();
    let window = app.get_webview_window("main").expect("Failed to get main window");

    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            println!("Saving config before exit...");

            let state_manager = app_handle.state::<StateManager>();

            // Block closing temporarily
            api.prevent_close();

            state_manager.save_to_file();

            // Sleep to ensure file write completes
            std::thread::sleep(std::time::Duration::from_millis(100));

            app_handle.exit(0);
        }
    });
}