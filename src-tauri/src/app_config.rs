// src-tauri/src/app_config.rs
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonitorMode {
    Primary,
    Index,
    NameContains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorSelector {
    pub mode: MonitorMode,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewConfig {
    pub id: String,
    pub url: String,
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: u8,
    pub monitor: MonitorSelector,
    pub views: [ViewConfig; 4],
}

fn default_config() -> AppConfig {
    AppConfig {
        version: 1,
        monitor: MonitorSelector {
            mode: MonitorMode::Primary,
            value: None,
        },
        views: [
            ViewConfig {
                id: "topLeft".into(),
                url: "https://example.com".into(),
                profile: Some("view1".into()),
            },
            ViewConfig {
                id: "topRight".into(),
                url: "https://example.org".into(),
                profile: Some("view2".into()),
            },
            ViewConfig {
                id: "bottomLeft".into(),
                url: "https://example.net".into(),
                profile: Some("view3".into()),
            },
            ViewConfig {
                id: "bottomRight".into(),
                url: "https://www.wikipedia.org".into(),
                profile: Some("view4".into()),
            },
        ],
    }
}

pub fn config_path(app: &tauri::AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = app.path().app_config_dir()?; // <-- FIX: Result -> ?
    Ok(dir.join("config.json"))
}

pub fn load_or_init(app: &tauri::AppHandle) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let path = config_path(app)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        let cfg = default_config();
        fs::write(&path, serde_json::to_vec_pretty(&cfg)?)?;
        return Ok(cfg);
    }

    let bytes = fs::read(&path)?;
    Ok(serde_json::from_slice(&bytes)?)
}
