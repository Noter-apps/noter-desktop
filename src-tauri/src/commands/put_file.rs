use tauri::State;

use crate::{
    state::HeldState,
    types::{File, FileContent, Id},
};

/*
Put command for creating or updating a file.
id: Required, is file exists, it will be updated, otherwise new file will be created.
name: Optional, if provided, file will be renamed.
content: Optional, if provided, file content will be updated.
file_type: Required, if file does not exist, it will be created with this file type.
 */
#[tauri::command]
pub fn put_file(
    state: State<HeldState>,
    id: String,
    name: Option<String>,
    content: Option<String>,
) -> Result<File, String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    let id = Id::new(id, &state.notes_dir);

    match id.exists() {
        true => {
            let mut file = File::get_from_file(id.clone()).map_err(|e| e.to_string())?;
            let file_type = file.get_type();

            if let Some(name) = name {
                file.rename(name).map_err(|e| e.to_string())?;
            }

            if let Some(content) = content {
                let file_content = FileContent::custom_deserialize(content.as_bytes(), file_type)
                    .map_err(|e| e.to_string())?;
                file.put(file_content).map_err(|e| e.to_string())?;
            }

            Ok(file)
        }
        false => Err("File does not exist".to_string()),
    }
}
