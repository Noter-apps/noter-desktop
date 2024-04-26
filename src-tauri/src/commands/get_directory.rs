use tauri::State;

use crate::{state::HeldState, types::Directory};

#[tauri::command]
pub fn get_directory(state: State<HeldState>, id: String) -> Result<Directory, String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };
    let notes_dir = state.get_notes_dir();
    let dir = state.get_directory().map_err(|e| e.to_string())?;
    Ok(dir)
}
