use crate::{
    state::HeldState,
    types::{File, Id},
};
use tauri::State;

#[tauri::command]
pub fn delete_file(state: State<HeldState>, id: String) -> Result<(), String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };
    let notes_dir = state.get_notes_dir();
    let id = Id::new(id, &notes_dir);

    File::delete(id, notes_dir).map_err(|e| e.to_string())
}
