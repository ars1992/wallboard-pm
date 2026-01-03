// src-tauri/src/settings_window.rs
use tauri::{webview::WebviewWindowBuilder, Manager, WebviewUrl};

fn debug_list_windows(app: &tauri::AppHandle) {
  let labels: Vec<String> = app.webview_windows().keys().cloned().collect();
  println!("[settings] existing windows: {:?}", labels);
}

pub fn open_settings_window(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  debug_list_windows(app);

  if let Some(win) = app.get_webview_window("settings") {
    println!("[settings] settings window already exists -> show/focus/top");
    let _ = win.set_always_on_top(true);
    let _ = win.show();
    let _ = win.set_focus();
    let _ = win.center();
    return Ok(());
  }

  println!("[settings] creating settings window -> settings.html");

  let win = WebviewWindowBuilder::new(
    app,
    "settings",
    WebviewUrl::App("settings.html".into()),
  )
  .title("Wallboard Settings")
  .inner_size(900.0, 650.0)
  .resizable(true)
  .decorations(true)
  .always_on_top(true) // <-- WICHTIG, da deine Views always_on_top sind
  .focused(true)
  .build()?;

  let _ = win.show();
  let _ = win.set_focus();
  let _ = win.center();

  Ok(())
}