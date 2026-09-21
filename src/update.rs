//! Update check.
//!
//! Queries the GitHub Releases API for the latest published release and reports
//! whether it is newer than the running build. The network request runs on a
//! background thread; the result is delivered to the UI thread through a channel
//! so the interface never blocks.
//!
//! This is intentionally *minimal*: it only detects and notifies. It never
//! downloads or replaces anything — the user follows a link to the download page.

use std::sync::mpsc::Receiver;
use std::time::Duration;

/// GitHub repository queried for releases (`owner/repo`).
const REPO: &str = "plplecuyer/ximod-architect";

/// GitHub Releases page (all platforms: Windows, Linux, macOS).
pub const RELEASES_URL: &str = "https://github.com/plplecuyer/ximod-architect/releases";

/// Nexus Mods page (Windows downloads and the mod's home page).
pub const NEXUS_URL: &str = "https://www.nexusmods.com/site/mods/2141";

/// Outcome of an update check, sent back to the UI thread.
#[derive(Debug, Clone)]
pub enum UpdateCheck {
    /// A newer release is available; carries its version (no leading `v`).
    Available(String),
    /// The running build is up to date.
    UpToDate,
    /// The check could not be completed (network, parsing…). The message is only
    /// surfaced for a *manual* check; automatic startup checks fail silently.
    Failed(String),
}

/// The running version, taken from `Cargo.toml` at compile time.
pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Spawn a background thread that performs the check and return its receiver.
/// Poll the receiver with `try_recv()` from the UI loop.
pub fn spawn_check() -> Receiver<UpdateCheck> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(check_blocking());
    });
    rx
}

/// Perform the check synchronously (runs on the background thread).
fn check_blocking() -> UpdateCheck {
    match latest_release_tag() {
        Ok(tag) => {
            let latest = tag.trim().trim_start_matches('v').trim().to_string();
            if latest.is_empty() {
                UpdateCheck::Failed("empty tag".to_string())
            } else if is_newer(&latest, current_version()) {
                UpdateCheck::Available(latest)
            } else {
                UpdateCheck::UpToDate
            }
        }
        Err(e) => UpdateCheck::Failed(e),
    }
}

/// Fetch the `tag_name` of the latest release from the GitHub API.
fn latest_release_tag() -> Result<String, String> {
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");
    let resp = ureq::get(&url)
        .set(
            "User-Agent",
            concat!("XIMOD-Architect/", env!("CARGO_PKG_VERSION")),
        )
        .set("Accept", "application/vnd.github+json")
        .timeout(Duration::from_secs(8))
        .call()
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    json.get("tag_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "no tag_name in response".to_string())
}

/// Return `true` when `candidate` is a strictly newer version than `current`.
///
/// Versions are compared as three dotted numbers (major.minor.patch). Any
/// non-numeric part (e.g. a `-beta` suffix) is reduced to its leading digits,
/// so a pre-release never reads as newer than the matching stable release.
pub fn is_newer(candidate: &str, current: &str) -> bool {
    parse(candidate) > parse(current)
}

/// Parse a dotted version into a `(major, minor, patch)` tuple. Missing or
/// non-numeric components become 0.
fn parse(v: &str) -> (u64, u64, u64) {
    let mut parts = v
        .split(['.', '-', '+'])
        .map(|p| {
            p.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or(0)
        });
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newer_detection() {
        assert!(is_newer("1.0.3", "1.0.2"));
        assert!(is_newer("1.1.0", "1.0.9"));
        assert!(is_newer("2.0.0", "1.9.9"));
        assert!(is_newer("1.0.10", "1.0.9"));
        assert!(!is_newer("1.0.2", "1.0.2"));
        assert!(!is_newer("1.0.1", "1.0.2"));
        // 'v' prefix and pre-release suffixes are tolerated.
        assert!(is_newer("1.0.3", "1.0.2"));
        assert!(!is_newer("1.0.2-beta", "1.0.2"));
    }
}
