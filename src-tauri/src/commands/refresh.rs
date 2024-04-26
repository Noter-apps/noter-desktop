use tauri::State;

use crate::{
    state::HeldState,
    types::{Directory, FilePreview, Metadata, SortBy, SortOptions, SortOrder},
};

#[tauri::command]
pub fn refresh(state: State<HeldState>) -> Result<(Metadata, Vec<FilePreview>, Directory), String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let metadata = Metadata::new(state.get_notes_dir().clone());
    let files = state.get_all_files(Some(SortOptions {
        by: SortBy::Name,
        order: SortOrder::Ascending,
    }));
    let directory = state.get_directory().map_err(|e| e.to_string())?;

    Ok((metadata, files, directory))
}
