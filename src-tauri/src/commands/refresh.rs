use tauri::State;

use crate::{
    state::HeldState,
    types::{Directory, FilePreview, Metadata},
};

#[tauri::command]
pub fn refresh(state: State<HeldState>) -> Result<(Metadata, Vec<FilePreview>, Directory), String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let metadata = Metadata::new(state.get_notes_dir().clone());
    let files = state.get_files().clone();
    let directory = state.get_directory().clone();

    Ok((metadata, files, directory))
}
