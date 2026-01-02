// src-tauri/src/main.rs
mod app_config;
mod window_manager;

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| -> Result<(), Box<dyn std::error::Error>> {
      let handle = app.handle().clone();

      // Default-Template-Fenster ausblenden
      if let Some(main_win) = app.get_webview_window("main") {
        let _ = main_win.hide();
      }

      let cfg = app_config::load_or_init(&handle)?;

      if let Ok(path) = app_config::config_path(&handle) {
        println!("Wallboard config path: {}", path.display());
      }

      std::thread::spawn(move || {
        if let Err(e) = window_manager::spawn_wallboard(handle, cfg) {
          eprintln!("spawn_wallboard failed: {e}");
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}