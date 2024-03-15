use tauri::State;

use crate::{
    state::HeldState,
    types::{Directory, Id},
};

#[tauri::command]
pub fn get_directory(state: State<HeldState>, id: String) -> Result<Directory, String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };
    let id = Id::new(id, state.get_notes_dir());
    let notes_dir = state.get_notes_dir();
    Directory::get_from_file(id, notes_dir).map_err(|e| e.to_string())
}
