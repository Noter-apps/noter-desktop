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
    let mut state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let notes_dir = state.get_notes_dir();
    let parent_id = Id::from_string(parent_id);
    let id = Id::create_id(&name, &parent_id, file_type, notes_dir);
    let time = chrono::Utc::now();
    let file = File::new(id, FileContent::default(file_type), time, time);
    state.create_file(&file).map_err(|e| e.to_string())?;
    Ok(file)
}
