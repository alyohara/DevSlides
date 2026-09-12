//! PDF export Tauri command.
//!
//! The frontend rasterizes each slide (via the shared Shiki highlighter and
//! jsPDF) and sends the finished PDF bytes here for a native save dialog. The
//! base64 transport keeps the payload compact over the JSON IPC bridge.

use crate::commands::helpers::{dialog_pick_path, sanitize_filename, DialogMode};
use crate::error::{CommandError, CommandResult};
use base64::Engine;
use tauri::AppHandle;

#[tauri::command]
pub async fn export_project_to_pdf(
    app: AppHandle,
    project_name: String,
    bytes_b64: String,
) -> CommandResult<String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(bytes_b64.as_bytes())
        .map_err(|e| CommandError::Failed(format!("Failed to decode PDF data: {e}")))?;
    if bytes.is_empty() {
        return Err(CommandError::Failed("PDF data is empty".to_string()));
    }

    let default_name = format!("{}.pdf", sanitize_filename(&project_name));
    let app_handle = app.clone();
    let mut path = tauri::async_runtime::spawn_blocking(move || {
        dialog_pick_path(&app_handle, DialogMode::SavePdf, Some(&default_name))
    })
    .await
    .map_err(|e| CommandError::Failed(format!("Dialog task failed: {e}")))?
    .ok_or_else(|| CommandError::Cancelled("Export cancelled".to_string()))?;

    // The native dialog may store the entered extension elsewhere or omit it;
    // make sure the artifact always lands as a .pdf file.
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("pdf"))
        != Some(true)
    {
        path.set_extension("pdf");
    }

    let write_path = path.clone();
    tauri::async_runtime::spawn_blocking(move || std::fs::write(write_path, &bytes))
        .await
        .map_err(|e| CommandError::Failed(format!("File write task failed: {e}")))?
        .map_err(|e| CommandError::Failed(format!("Failed to write file: {e}")))?;

    Ok(path.display().to_string())
}