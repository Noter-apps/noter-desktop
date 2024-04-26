use crate::{state::HeldState, types::Id};
use tauri::State;

#[tauri::command]
pub fn delete_file(state: State<HeldState>, id: String) -> Result<(), String> {
    let mut state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let entry_id = Id::from_string(id);
    state.delete_file(&entry_id).map_err(|e| e.to_string())?;

    Ok(())
}
