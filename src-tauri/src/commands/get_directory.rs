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
    let notes_dir = state.get_notes_dir();
    let id = Id::new(id, notes_dir);
    Directory::read_dir(&id.path_from_id(notes_dir), notes_dir, &mut |_| {})
        .map_err(|e| e.to_string())
}
