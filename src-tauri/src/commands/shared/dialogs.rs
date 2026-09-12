//! Native file dialogs for import/export.

use std::sync::mpsc;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Clone, Copy)]
pub enum DialogMode {
    Save,
    SavePdf,
    Open,
}

/// Await an open file dialog restricted to raster image types.
pub fn dialog_pick_image_path(app: &AppHandle) -> Option<std::path::PathBuf> {
    let (tx, rx) = mpsc::channel();
    let builder = app
        .dialog()
        .file()
        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp", "bmp", "avif"]);
    builder.pick_file(move |path| {
        let _ = tx.send(path);
    });
    rx.recv()
        .ok()
        .flatten()
        .and_then(|fp| fp.into_path().ok())
}

/// Await a callback-based dialog on a worker-friendly channel.
pub fn dialog_pick_path(
    app: &AppHandle,
    mode: DialogMode,
    default_name: Option<&str>,
) -> Option<std::path::PathBuf> {
    let (tx, rx) = mpsc::channel();
    let mut builder = app.dialog().file();
    builder = match mode {
        DialogMode::Save => builder.add_filter("JSON", &["json"]),
        DialogMode::SavePdf => {
            builder
                .add_filter("PDF", &["pdf"])
                .set_default_extension("pdf")
        }
        DialogMode::Open => builder.add_filter("JSON", &["json"]),
    };
    if let Some(name) = default_name {
        builder = builder.set_file_name(name);
    }
    match mode {
        DialogMode::Save | DialogMode::SavePdf => {
            builder.save_file(move |path| {
                let _ = tx.send(path);
            });
        }
        DialogMode::Open => {
            builder.pick_file(move |path| {
                let _ = tx.send(path);
            });
        }
    }
    rx.recv()
        .ok()
        .flatten()
        .and_then(|fp| fp.into_path().ok())
}
