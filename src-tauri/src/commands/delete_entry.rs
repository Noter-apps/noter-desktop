use tauri::State;

use crate::{state::HeldState, types::Id};

#[tauri::command]
pub fn delete_entry(state: State<HeldState>, entry_id: String) -> Result<(), String> {
    let mut state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let entry_id = Id::from_string(entry_id);
    state.delete_file(&entry_id);

    Ok(())
}
