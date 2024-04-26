use tauri::State;

use crate::{state::HeldState, types::Graph};

#[tauri::command]
pub fn get_graph(state: State<HeldState>) -> Result<Graph, String> {
    let state = match state.lock() {
        Ok(state) => state,
        Err(_) => return Err("Could not lock state".to_string()),
    };

    unimplemented!()
}
