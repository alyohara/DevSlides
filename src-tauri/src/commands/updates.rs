//! Update checks — compare the running build against the latest GitHub release.
//!
//! The webview CSP forbids outbound network calls (`connect-src 'self' ipc:`),
//! so the frontend asks this command for update info instead of fetching
//! GitHub itself.

use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

const UPDATE_API_URL: &str =
    "https://api.github.com/repos/alyohara/DevSlides/releases/latest";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(12);

/// Result of an update check. `update_available` only becomes true when the
/// latest release is newer than the running build; every other state (no
/// network, equal versions, unparseable tags) stays conservative on purpose.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: Option<String>,
    pub release_url: Option<String>,
    pub release_notes: Option<String>,
    pub update_available: bool,
    pub check_error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    #[serde(default)]
    body: Option<String>,
}

#[tauri::command]
pub async fn check_for_updates() -> UpdateInfo {
    let current_version = env!("CARGO_PKG_VERSION").to_string();
    let fetched = tauri::async_runtime::spawn_blocking(fetch_latest_release).await;
    match fetched {
        Ok(Ok(release)) => {
            let update_available = match (
                parse_version(&release.tag_name),
                parse_version(&current_version),
            ) {
                (Some(latest), Some(current)) => latest > current,
                _ => false,
            };
            UpdateInfo {
                current_version,
                latest_version: Some(
                    release.tag_name.trim_start_matches('v').to_string(),
                ),
                release_url: Some(release.html_url),
                release_notes: release.body,
                update_available,
                check_error: None,
            }
        }
        Ok(Err(message)) => UpdateInfo {
            current_version,
            latest_version: None,
            release_url: None,
            release_notes: None,
            update_available: false,
            check_error: Some(message),
        },
        Err(join_error) => UpdateInfo {
            current_version,
            latest_version: None,
            release_url: None,
            release_notes: None,
            update_available: false,
            check_error: Some(format!("update check task failed: {join_error}")),
        },
    }
}

fn fetch_latest_release() -> Result<GitHubRelease, String> {
    let response = ureq::get(UPDATE_API_URL)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "DevSlides")
        .timeout(REQUEST_TIMEOUT)
        .call()
        .map_err(|err| err.to_string())?;
    let body = response
        .into_string()
        .map_err(|err| format!("failed to read response: {err}"))?;
    serde_json::from_str(&body).map_err(|err| format!("failed to parse release: {err}"))
}

/// Parse "1.3.0" / "v1.4.0" into comparable semver integers.
fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let mut parts = version.trim_start_matches('v').split('.');
    let major = parts.next().and_then(parse_digits)?;
    let minor = parts.next().and_then(parse_digits)?;
    let patch = parts.next().and_then(parse_digits)?;
    Some((major, minor, patch))
}

fn parse_digits(part: &str) -> Option<u32> {
    part.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}