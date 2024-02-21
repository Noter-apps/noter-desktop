use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{read, write};
use std::path::PathBuf;
use tauri::PathResolver;

const DEFAULT_WIDTH: f64 = 1920.0;
const DEFAULT_HEIGHT: f64 = 1080.0;

const APP_CONFIG_FILE: &str = "noter.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    notes_folders: Vec<PathBuf>,
    preffered_notes_folder: PathBuf,
    width: f64,
    height: f64,
}

impl Config {
    pub fn new(notes_folder: PathBuf) -> Self {
        Self {
            notes_folders: vec![notes_folder.clone()],
            preffered_notes_folder: notes_folder,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }

    pub fn get_config_file_path(path_resolver: &PathResolver) -> PathBuf {
        path_resolver
            .app_config_dir()
            .unwrap()
            .join(APP_CONFIG_FILE)
    }

    pub fn store(&self, path: &PathBuf) -> Result<()> {
        let config_raw = serde_json::to_vec_pretty(self)?;
        write(path, config_raw)?;
        Ok(())
    }
    pub fn load(path: &PathBuf) -> Result<Self> {
        let config_raw = read(path)?;
        let config: Self = serde_json::from_slice(&config_raw)?;
        Ok(config)
    }

    pub fn get_window_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn get_preffered_notes_folder(&self) -> PathBuf {
        self.preffered_notes_folder.clone()
    }
}
