use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub notes_dir: PathBuf,
}

impl Metadata {
    pub fn new(notes_dir: PathBuf) -> Self {
        Self { notes_dir }
    }
}
