// src-tauri/src/window_manager.rs
use crate::app_config::{AppConfig, MonitorMode};
use std::{io, path::PathBuf};
use tauri::{webview::WebviewWindowBuilder, Manager, WebviewUrl};

fn select_monitor(app: &tauri::AppHandle, cfg: &AppConfig) -> Result<tauri::Monitor, Box<dyn std::error::Error>> {
  match cfg.monitor.mode {
    MonitorMode::Primary => {
      app.primary_monitor()?
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No primary monitor found").into())
    }
    MonitorMode::Index => {
      let monitors = app.available_monitors()?;
      let idx: usize = cfg.monitor.value.as_deref().unwrap_or("0").parse()?;
      monitors
        .get(idx)
        .cloned()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, format!("No monitor at index {idx}")).into())
    }
    MonitorMode::NameContains => {
      let needle = cfg.monitor.value.clone().unwrap_or_default().to_lowercase();
      let monitors = app.available_monitors()?;
      monitors
        .into_iter()
        .find(|m| {
          let name_lc = m.name().map(|s| s.to_string()).unwrap_or_default().to_lowercase();
          name_lc.contains(&needle)
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, format!("No monitor name contains '{needle}'")).into())
    }
  }
}

pub fn spawn_wallboard(app: tauri::AppHandle, cfg: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
  let monitor = select_monitor(&app, &cfg)?;
  let origin = monitor.position();
  let size = monitor.size();

  let tile_w = size.width / 2;
  let tile_h = size.height / 2;

  let rects = [
    (origin.x, origin.y),
    (origin.x + tile_w as i32, origin.y),
    (origin.x, origin.y + tile_h as i32),
    (origin.x + tile_w as i32, origin.y + tile_h as i32),
  ];

  // <-- FIX: app_data_dir() ist Result bei dir
  let app_data_dir = app.path().app_data_dir()?;

  for (i, view) in cfg.views.iter().enumerate() {
    let label = format!("view-{}", view.id);
    let url = WebviewUrl::External(view.url.parse()?);

    let profile_name = view.profile.clone().unwrap_or_else(|| label.clone());
    let data_dir: PathBuf = app_data_dir.join("profiles").join(profile_name);

    let win = WebviewWindowBuilder::new(&app, label, url)
      .title("Wallboard")
      .decorations(false)
      .resizable(false)
      .always_on_top(true)
      .data_directory(data_dir) // 1 Profil pro View
      .build()?;

    let (x, y) = rects[i];
    win.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(x, y)))?;
    win.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(tile_w, tile_h)))?;
  }

  Ok(())
}