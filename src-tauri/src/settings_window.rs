// src-tauri/src/settings_window.rs
use tauri::{webview::WebviewWindowBuilder, Manager, WebviewUrl};

pub fn open_settings_window(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  if let Some(win) = app.get_webview_window("settings") {
    let _ = win.show();
    let _ = win.set_focus();
    return Ok(());
  }

  WebviewWindowBuilder::new(
    app,
    "settings",
    WebviewUrl::App("settings/index.html".into()),
  )
  .title("Wallboard Settings")
  .inner_size(900.0, 650.0)
  .resizable(true)
  .decorations(true)
  .build()?;

  Ok(())
}