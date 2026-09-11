//! Slide image layer parse/serialize helpers (mirror of `services/highlights`).

use crate::models::SlideImage;

/// Parse the slide `images` JSON column. Unknown content falls back to empty
/// so legacy rows (pre-v9) and hand-edited projects stay safe.
pub fn parse_images(raw: &str) -> Vec<SlideImage> {
    serde_json::from_str(raw).unwrap_or_default()
}

pub fn serialize_images(images: &[SlideImage]) -> Result<String, String> {
    serde_json::to_string(images).map_err(|e| e.to_string())
}