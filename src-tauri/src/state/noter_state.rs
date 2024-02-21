use std::{path::PathBuf, sync::Mutex};

pub struct NoterState {
    pub notes_dir: PathBuf,
}

impl NoterState {
    pub fn new(notes_dir: PathBuf) -> Self {
        Self { notes_dir }
    }
}

pub type HeldState = Mutex<NoterState>;
