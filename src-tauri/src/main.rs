// src-tauri/src/main.rs
mod app_config;
mod settings_api;
mod settings_window;
mod window_manager;

use settings_api::{apply_config, get_config, list_monitors, save_config, AppState};
use tauri::Manager; // <-- ADD

#[tauri::command]
fn toggle_minimize_views(
  app: tauri::AppHandle,
  state: tauri::State<AppState>,
) -> Result<(), String> {
  let mut minimized = state.views_minimized.lock().unwrap();
  let want_minimize = !*minimized;

  for (label, win) in app.webview_windows() {
    if !label.starts_with("view-") {
      continue;
    }

    if want_minimize {
      // DEV macOS: hide ist zuverlässiger als minimize/unminimize
      #[cfg(target_os = "macos")]
      win.hide().map_err(|e| e.to_string())?;

      // Ziel Windows: echtes minimieren
      #[cfg(not(target_os = "macos"))]
      win.minimize().map_err(|e| e.to_string())?;
    } else {
      #[cfg(target_os = "macos")]
      {
        win.show().map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| e.to_string())?;
      }

      #[cfg(not(target_os = "macos"))]
      {
        win.unminimize().map_err(|e| e.to_string())?;
        win.show().map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| e.to_string())?;
      }
    }
  }

  *minimized = want_minimize;
  Ok(())
}

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
                views_minimized: std::sync::Mutex::new(false),
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
            toggle_minimize_views,
            apply_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
