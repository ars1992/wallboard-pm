// src-tauri/src/window_manager.rs
use crate::app_config::{AppConfig, MonitorMode};
use std::error::Error;
use std::io;
use tauri::{webview::WebviewWindowBuilder, Manager, WebviewUrl};

fn select_monitor(
    app: &tauri::AppHandle,
    cfg: &AppConfig,
) -> Result<tauri::Monitor, Box<dyn std::error::Error>> {
    match cfg.monitor.mode {
        MonitorMode::Primary => app
            .primary_monitor()?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No primary monitor found").into()),
        MonitorMode::Index => {
            let monitors = app.available_monitors()?;
            let idx: usize = cfg.monitor.value.as_deref().unwrap_or("0").parse()?;
            monitors.get(idx).cloned().ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, format!("No monitor at index {idx}")).into()
            })
        }
        MonitorMode::NameContains => {
            let needle = cfg.monitor.value.clone().unwrap_or_default().to_lowercase();
            let monitors = app.available_monitors()?;
            monitors
                .into_iter()
                .find(|m| {
                    let name_lc = m
                        .name()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                        .to_lowercase();
                    name_lc.contains(&needle)
                })
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("No monitor name contains '{needle}'"),
                    )
                    .into()
                })
        }
    }
}

pub fn spawn_wallboard(
    app: tauri::AppHandle,
    cfg: AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
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

        let target_url = view.url.parse()?; // url::Url
        let (x, y) = rects[i];

        if let Some(win) = app.get_webview_window(&label) {
            // Fenster existiert -> nur aktualisieren
            win.navigate(target_url)?; // ([docs.rs](https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html))
            win.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
                x, y,
            )))?;
            win.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(
                tile_w, tile_h,
            )))?;
            let _ = win.show();
            continue;
        }

        // Fenster existiert nicht -> neu erstellen (hier ist data_directory relevant)
        let profile_name = view.profile.clone().unwrap_or_else(|| label.clone());
        let data_dir = app_data_dir.join("profiles").join(profile_name);

        let win = WebviewWindowBuilder::new(&app, label, WebviewUrl::External(target_url))
            .title("Wallboard")
            .decorations(false)
            .resizable(false)
            .always_on_top(true)
            .data_directory(data_dir)
            .build()?;

        win.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
            x, y,
        )))?;
        win.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(
            tile_w, tile_h,
        )))?;
    }

    Ok(())
}

pub fn recreate_wallboard_windows(
    app: tauri::AppHandle,
    cfg: AppConfig,
) -> Result<(), Box<dyn Error>> {
    for (label, win) in app.webview_windows() {
        if label.starts_with("view-") {
            let _ = win.close();
        }
    }

    spawn_wallboard(app, cfg)?;
    Ok(())
}
