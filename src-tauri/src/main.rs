// src-tauri/src/main.rs
mod app_config;
mod settings_api;
mod settings_window;
mod window_manager;

use settings_api::{apply_config, get_config, list_monitors, save_config, AppState};
use tauri::Manager; // <-- ADD

#[tauri::command]
fn open_settings(app: tauri::AppHandle) -> Result<(), String> {
    println!("open_settings called");
    settings_window::open_settings_window(&app).map_err(|e| e.to_string())
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| -> Result<(), Box<dyn std::error::Error>> {
            let handle = app.handle().clone();

            let cfg = app_config::load_or_init(&handle)?;
            app.manage(AppState {
                config: std::sync::Mutex::new(cfg.clone()),
            });

            if let Some(w) = app.get_webview_window("main") {
                let _ = w.hide();
            }

            std::thread::spawn(move || {
                if let Err(e) = window_manager::spawn_wallboard(handle, cfg) {
                    eprintln!("spawn_wallboard failed: {e}");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            list_monitors,
            open_settings,
            apply_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
