use tauri::State;

use crate::{
    state::HeldState,
    types::{File, Id},
};

#[tauri::command]
pub fn get_file(state: State<HeldState>, id: String) -> Result<File, String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let id = Id::new(id, state.get_notes_dir());
    let notes_dir = state.get_notes_dir();
    File::get_from_file(id, notes_dir).map_err(|e| e.to_string())
}
