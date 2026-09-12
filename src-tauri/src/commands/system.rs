//! System/app-level Tauri commands (about info, external links).

use crate::error::{CommandError, CommandResult};
use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub repository: &'static str,
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        description: "DevSlides — offline-first code presentation desktop app (a fork of OpenSlides)",
        repository: "https://github.com/alyohara/DevSlides",
    }
}

/// Open an external URL in the user's default browser.
#[tauri::command]
pub async fn open_url(app: AppHandle, url: String) -> CommandResult<()> {
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| CommandError::Failed(format!("Failed to open URL: {e}")))?;
    Ok(())
}