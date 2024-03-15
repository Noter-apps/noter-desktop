use tauri::State;

use crate::{
    state::HeldState,
    types::{File, FileContent, FileType, Id},
};

#[tauri::command]
pub fn create_file(
    state: State<HeldState>,
    parent_id: String,
    name: String,
    file_type: FileType,
) -> Result<File, String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let notes_dir = state.get_notes_dir();
    let parent_id = Id::new(parent_id, notes_dir);
    let file_id = parent_id.create_child(&name, file_type, notes_dir);
    let file_content = FileContent::default(file_type);
    let file = File::new(file_id, notes_dir, name, file_content).map_err(|e| e.to_string())?;
    Ok(file)
}
