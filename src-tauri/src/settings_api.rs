// src-tauri/src/settings_api.rs
use crate::{app_config, window_manager};
use serde::Serialize;
use std::sync::Mutex;

pub struct AppState {
    pub config: Mutex<app_config::AppConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
    pub index: usize,
    pub name: String,
    pub is_primary: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
}

#[tauri::command]
pub fn apply_config(app: tauri::AppHandle, state: tauri::State<AppState>) -> Result<(), String> {
  let cfg = state.config.lock().unwrap().clone();
  // statt recreate (close/rebuild) -> idempotentes apply
  window_manager::apply_wallboard(app, cfg).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_config(state: tauri::State<AppState>) -> app_config::AppConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
pub fn list_monitors(app: tauri::AppHandle) -> Result<Vec<MonitorInfo>, String> {
    let primary = app.primary_monitor().map_err(|e| e.to_string())?;
    let monitors = app.available_monitors().map_err(|e| e.to_string())?;

    let primary_sig = primary.as_ref().map(|m| {
        (
            m.position().x,
            m.position().y,
            m.size().width,
            m.size().height,
        )
    });

    let out = monitors
        .into_iter()
        .enumerate()
        .map(|(index, m)| {
            let sig = (
                m.position().x,
                m.position().y,
                m.size().width,
                m.size().height,
            );
            MonitorInfo {
                index,
                name: m
                    .name()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("Monitor {index}")),
                is_primary: primary_sig.map(|p| p == sig).unwrap_or(false),
                position: (m.position().x, m.position().y),
                size: (m.size().width, m.size().height),
            }
        })
        .collect();

    Ok(out)
}

#[tauri::command]
pub fn save_config(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    new_cfg: app_config::AppConfig,
) -> Result<(), String> {
    for v in new_cfg.views.iter() {
        if !(v.url.starts_with("http://") || v.url.starts_with("https://")) {
            return Err(format!(
                "URL for '{}' must start with http:// or https://",
                v.id
            ));
        }
    }

    let path = app_config::config_path(&app).map_err(|e| e.to_string())?;
    let bytes = serde_json::to_vec_pretty(&new_cfg).map_err(|e| e.to_string())?;
    std::fs::write(&path, bytes).map_err(|e| e.to_string())?;

    *state.config.lock().unwrap() = new_cfg;
    Ok(())
}


