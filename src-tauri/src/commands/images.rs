//! Image asset commands: native file picker + system clipboard reads.
//!
//! Images are embedded as data URLs so slides stay self-contained (they
//! serialize straight into the SQLite `images` JSON column and survive
//! export/import as-is).

use crate::commands::helpers::dialog_pick_image_path;
use crate::error::{CommandError, CommandResult};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use std::path::Path;
use tauri::AppHandle;

/// Safety cap for a single embedded image (12 MB).
const MAX_IMAGE_BYTES: u64 = 12 * 1024 * 1024;

fn mime_for_path(path: &Path) -> Option<&'static str> {
    match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "webp" => Some("image/webp"),
        "bmp" => Some("image/bmp"),
        "avif" => Some("image/avif"),
        _ => None,
    }
}

/// Native dialog → read the chosen file → `data:<mime>;base64,...`.
#[tauri::command]
pub async fn pick_image_file(app: AppHandle) -> CommandResult<String> {
    let app_handle = app.clone();
    let path = tauri::async_runtime::spawn_blocking(move || {
        dialog_pick_image_path(&app_handle)
    })
    .await
    .map_err(|e| CommandError::Failed(format!("Dialog task failed: {e}")))?
    .ok_or_else(|| CommandError::Cancelled("Image selection cancelled".to_string()))?;

    let mime = mime_for_path(&path).ok_or_else(|| {
        CommandError::Validation(
            "Unsupported image format — choose PNG, JPEG, GIF, WebP, BMP or AVIF".to_string(),
        )
    })?;

    let read_path = path.clone();
    let bytes = tauri::async_runtime::spawn_blocking(move || {
        let meta = std::fs::metadata(&read_path)
            .map_err(|e| CommandError::Failed(format!("Failed to read image: {e}")))?;
        if meta.len() > MAX_IMAGE_BYTES {
            return Err(CommandError::Validation(
                "That image is larger than 12 MB — OpenSlides keeps images embedded in the project, so try a smaller file.".to_string(),
            ));
        }
        std::fs::read(&read_path)
            .map_err(|e| CommandError::Failed(format!("Failed to read image: {e}")))
    })
    .await
    .map_err(|e| CommandError::Failed(format!("Image read task failed: {e}")))?;

    let bytes: Vec<u8> = bytes?;
    let encoded = BASE64.encode(bytes);
    Ok(format!("data:{mime};base64,{encoded}"))
}

/// Read an image from the system clipboard (if any) as a PNG data URL.
#[tauri::command]
pub fn read_clipboard_image() -> CommandResult<Option<String>> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| CommandError::Failed(format!("Clipboard unavailable: {e}")))?;
    let image = match clipboard.get_image() {
        Ok(image) => image,
        Err(arboard::Error::ContentNotAvailable) => return Ok(None),
        Err(e) => {
            return Err(CommandError::Failed(format!(
                "Clipboard image read failed: {e}"
            )))
        }
    };

    // arboard's ImageData is raw RGBA; the webview needs an encoded file.
    let mut png = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png, image.width as u32, image.height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer.write_image_data(&image.bytes).map_err(|e| e.to_string())?;
    }

    if png.len() > MAX_IMAGE_BYTES as usize {
        return Err(CommandError::Validation(
            "That clipboard image is larger than 12 MB — try a smaller image.".to_string(),
        ));
    }

    Ok(Some(format!("data:image/png;base64,{}", BASE64.encode(png))))
}