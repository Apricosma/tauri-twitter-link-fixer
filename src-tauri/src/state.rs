use crate::config::app_config::SourcesConfig;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub struct StateManager {
    state: Arc<Mutex<SourcesConfig>>,
    config_path: String,
    pub app: AppHandle,
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

    pub fn update_state<F, R>(&self, update_fn: F) -> R
    where
        F: FnOnce(&mut SourcesConfig) -> R,
    {
        let mut state_guard = self.state.lock().unwrap();
        let result = update_fn(&mut state_guard);
        // Clone the state while holding the lock to avoid deadlock
        let state_clone = state_guard.clone();
        // Drop the lock before emitting
        drop(state_guard);
        // Emit state change event with the cloned state
        if let Err(e) = self.app.emit("state-changed", state_clone) {
            eprintln!("Failed to emit state change: {}", e);
        }
        result
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