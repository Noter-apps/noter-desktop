use tauri::State;

use crate::{
    state::HeldState,
    types::{Entry, Id},
};

#[tauri::command]
pub fn delete_entry(state: State<HeldState>, entry_id: String) -> Result<(), String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let notes_dir = state.get_notes_dir();
    let entry_id = Id::new(entry_id, &notes_dir);
    Entry::delete_entry(entry_id, notes_dir).map_err(|e| e.to_string())
}
