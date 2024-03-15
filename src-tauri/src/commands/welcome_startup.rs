use crate::{state::NoterState, types::Config, utils::startup};
use anyhow::Result;
use std::path::PathBuf;
use tauri::{command, AppHandle, Manager};

#[command]
pub fn welcome_startup(notes_dir: PathBuf, app: AppHandle) -> Result<(), String> {
    let welcome_window = match app.get_window("welcome") {
        Some(w) => w,
        None => return Err("welcome window not found".to_string()),
    };

    let config = Config::new(notes_dir.clone());
    let config_path = Config::get_config_file_path(&app.path_resolver());
    match config.store(&config_path) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!(
                "error in welcome_startup command\nconfig file could not be stored: {}",
                e.to_string()
            ))
        }
    };

    let state = NoterState::new(notes_dir);
    startup(config, app, state);
    welcome_window.close().map_err(|e| e.to_string())
}
