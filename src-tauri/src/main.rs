#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;
mod types;
mod utils;

use std::fs::create_dir_all;

use crate::{
    commands::*,
    types::Config,
    utils::{run_startup, run_welcome},
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let path_resolver = app.path_resolver();

            // In case the whole config folder does not yet exist
            if !path_resolver.app_config_dir().unwrap().exists() {
                create_dir_all(path_resolver.app_config_dir().unwrap()).unwrap();
            }

            let config_file = Config::get_config_file_path(&path_resolver);
            match config_file.exists() {
                true => run_startup(app.handle()).unwrap(),
                false => run_welcome(app.handle()),
            };

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_file,
            delete_file,
            get_file,
            put_file,
            get_directory,
            welcome_startup,
            delete_entry,
            refresh
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
