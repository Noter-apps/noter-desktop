use crate::{state::*, Config};

use anyhow::Result;
use tauri::{AppHandle, Manager, WindowBuilder};

pub fn run_welcome(app: AppHandle) {
    let handle = std::thread::spawn(move || {
        WindowBuilder::new(
            &app,
            "welcome",
            tauri::WindowUrl::App("static/welcome.html".into()),
        )
        .title("Welcome")
        .inner_size(600.0, 800.0)
        .center()
        .focused(true)
        .visible(true)
        .resizable(false)
        .build()
        .unwrap()
        .show()
        .unwrap();
    });

    let _ = handle.join();
}

pub fn startup(config: Config, app: AppHandle, state: NoterState) {
    let (width, height) = config.get_window_size();

    let handle = std::thread::spawn(move || {
        let window = WindowBuilder::new(
            &app,
            "main",
            tauri::WindowUrl::App("static/index.html".into()),
        )
        .title("Noter")
        .inner_size(width, height)
        .center()
        .focused(true)
        .visible(true)
        .resizable(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .build()
        .unwrap();
        window.manage(HeldState::new(state));
    });

    let _ = handle.join();
}

pub fn run_startup(app: AppHandle) -> Result<()> {
    let path_resolver = app.path_resolver();
    let config_file = Config::get_config_file_path(&path_resolver);
    let config = Config::load(&config_file)?;

    let state = NoterState::new(config.get_preffered_notes_folder())?;

    startup(config, app, state);
    Ok(())
}
