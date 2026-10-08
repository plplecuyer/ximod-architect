//! Last-resort protection against losing work on a panic.
//!
//! Release builds use `panic = "abort"`, so an unexpected panic closes the
//! application without any chance to save. A panic hook still runs before the
//! abort, though. This module keeps a cheap, throttled copy of every modified
//! document and, from the hook, writes each one as a regular FOMOD folder
//! under `<config dir>/crash_recovery/<timestamp>/<mod name>/`, which the
//! user can simply open again with "Open Folder…".
//!
//! The same snapshots feed the periodic autosave (`flush_now`): every N
//! minutes they are written under `<config dir>/crash_recovery/autosave/`
//! with their own marker, which a clean exit removes (`clear_autosave`), so
//! the recovery prompt of the next start still fires only after an abnormal
//! exit (a crash, a kill, a power cut).
//!
//! Nothing here must ever panic itself: a panic inside a panic hook aborts
//! immediately and the recovery files would be lost.

use crate::models::Ximod;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Minimum delay between two snapshots of the open documents.
const SNAPSHOT_INTERVAL: Duration = Duration::from_secs(2);

/// Name of the marker file dropped next to the recovery folders so the next
/// start can tell the user where to look.
const PENDING_MARKER: &str = "pending.txt";

/// Folder (under the recovery dir) holding the periodic autosave copies.
const AUTOSAVE_DIR: &str = "autosave";

/// Marker of an autosave that was not cleared by a clean exit.
const AUTOSAVE_MARKER: &str = "autosave_pending.txt";

/// One document worth saving: its model and, when known, the mod root it
/// belongs to (used only to name the recovery folder).
pub struct Snapshot {
    pub root: Option<PathBuf>,
    pub ximod: Ximod,
}

struct State {
    snapshots: Vec<Snapshot>,
    last_update: Option<Instant>,
}

static STATE: Mutex<State> = Mutex::new(State {
    snapshots: Vec::new(),
    last_update: None,
});

/// Whether enough time has passed since the last snapshot for a new one to be
/// worth taking. Lets the caller skip the clone entirely most frames.
pub fn wants_snapshot() -> bool {
    match STATE.lock() {
        Ok(state) => state.last_update.is_none_or(|t| t.elapsed() >= SNAPSHOT_INTERVAL),
        Err(_) => false,
    }
}

/// Replace the guarded copy of the modified documents.
pub fn update(snapshots: Vec<Snapshot>) {
    if let Ok(mut state) = STATE.lock() {
        state.snapshots = snapshots;
        state.last_update = Some(Instant::now());
    }
}

/// Directory holding the recovery folders, when a config directory exists.
pub fn recovery_dir() -> Option<PathBuf> {
    crate::config::AppConfig::config_dir().map(|d| d.join("crash_recovery"))
}

/// Install the panic hook. Keeps the default hook (message on stderr) and
/// adds the recovery dump in front of it.
pub fn install_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        write_recovery();
        default_hook(info);
    }));
}

/// If the previous run left recovery files behind, return their folder and
/// clear the marker so the message is shown only once.
///
/// A crash dump (`pending.txt`) wins over an autosave left by an abnormal
/// exit (`autosave_pending.txt`); both markers are consumed either way.
pub fn take_pending_recovery() -> Option<PathBuf> {
    let dir = recovery_dir()?;
    let crash = take_marker(&dir.join(PENDING_MARKER));
    let autosave = take_marker(&dir.join(AUTOSAVE_MARKER));
    crash.or(autosave)
}

/// Read and delete a marker file; the folder it names when it still exists.
fn take_marker(marker: &Path) -> Option<PathBuf> {
    let target = std::fs::read_to_string(marker).ok()?;
    let _ = std::fs::remove_file(marker);
    let target = target.trim();
    if target.is_empty() {
        return None;
    }
    let path = PathBuf::from(target);
    path.is_dir().then_some(path)
}

/// Periodic autosave: write the guarded (modified) documents under
/// `<recovery dir>/autosave/` now, replacing the previous autosave, and
/// drop the autosave marker. With nothing modified the previous autosave is
/// removed instead (it would be stale). Returns the number of documents
/// written. Never called from the panic hook; errors are swallowed because
/// an autosave must never interrupt the editing session.
pub fn flush_now() -> usize {
    let Ok(state) = STATE.lock() else { return 0 };
    let Some(base) = recovery_dir() else { return 0 };
    let dir = base.join(AUTOSAVE_DIR);
    if state.snapshots.is_empty() {
        drop(state);
        clear_autosave();
        return 0;
    }
    // Rewrite from scratch so documents saved or closed since the previous
    // autosave do not linger.
    let _ = std::fs::remove_dir_all(&dir);
    let written = write_snapshots(&state.snapshots, &dir);
    if written > 0 {
        let _ = std::fs::write(base.join(AUTOSAVE_MARKER), dir.to_string_lossy().as_bytes());
    } else {
        let _ = std::fs::remove_file(base.join(AUTOSAVE_MARKER));
        let _ = std::fs::remove_dir(&dir);
    }
    written
}

/// Clean exit: the autosave copies are not needed any more. Removes the
/// autosave folder and its marker so the next start shows no prompt.
pub fn clear_autosave() {
    let Some(base) = recovery_dir() else { return };
    let _ = std::fs::remove_file(base.join(AUTOSAVE_MARKER));
    let _ = std::fs::remove_dir_all(base.join(AUTOSAVE_DIR));
}

/// Write every guarded document to disk. Called from the panic hook, so all
/// errors are swallowed deliberately.
fn write_recovery() {
    // `try_lock`: if the panic happened while the mutex was held, a blocking
    // lock would dead-lock the hook.
    let Ok(state) = STATE.try_lock() else { return };
    if state.snapshots.is_empty() {
        return;
    }
    let Some(base) = recovery_dir() else { return };
    let stamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let session_dir = base.join(stamp);
    let written = write_snapshots(&state.snapshots, &session_dir);
    if written > 0 {
        let _ = std::fs::write(base.join(PENDING_MARKER), session_dir.to_string_lossy().as_bytes());
        eprintln!(
            "XIMOD Architect: {written} project(s) saved to {} after a crash.",
            session_dir.display()
        );
    } else {
        let _ = std::fs::remove_dir(&session_dir);
    }
}

/// Write each snapshot as a FOMOD folder under `session_dir`; the number of
/// documents written. Shared by the panic hook and the autosave.
fn write_snapshots(snapshots: &[Snapshot], session_dir: &Path) -> usize {
    if std::fs::create_dir_all(session_dir).is_err() {
        return 0;
    }
    let mut written = 0usize;
    for (i, snap) in snapshots.iter().enumerate() {
        let name = snap
            .root
            .as_ref()
            .and_then(|r| r.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| {
                if snap.ximod.name.trim().is_empty() {
                    format!("untitled_{}", i + 1)
                } else {
                    snap.ximod.name.clone()
                }
            });
        let folder = session_dir.join(sanitize(&name));
        if std::fs::create_dir_all(&folder).is_err() {
            continue;
        }
        if crate::xml::save_ximod(&snap.ximod, &folder).is_ok() {
            written += 1;
        }
    }
    written
}

/// Make a mod name safe to use as a folder name on every platform.
fn sanitize(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_control() || matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') {
                '_'
            } else {
                c
            }
        })
        .collect();
    let trimmed = cleaned.trim_matches(|c: char| c == '.' || c == ' ');
    if trimmed.is_empty() {
        "project".to_string()
    } else {
        trimmed.chars().take(80).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_replaces_reserved_characters() {
        assert_eq!(sanitize("Guns & Roses: <4K>"), "Guns & Roses_ _4K_");
        assert_eq!(sanitize("   "), "project");
        assert_eq!(sanitize("..name.."), "name");
    }

    /// The autosave writer and the marker logic, on a scratch folder (the
    /// real recovery dir depends on the user's config dir).
    #[test]
    fn write_snapshots_and_markers() {
        let base = std::env::temp_dir().join(format!("ximod-crash-guard-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let snaps = vec![
            Snapshot {
                root: Some(PathBuf::from("/mods/My Mod")),
                ximod: Ximod::new("My Mod"),
            },
            Snapshot {
                root: None,
                ximod: Ximod::default(),
            },
        ];
        let dir = base.join(AUTOSAVE_DIR);
        assert_eq!(write_snapshots(&snaps, &dir), 2);
        assert!(dir.join("My Mod").join("fomod").join("ModuleConfig.xml").is_file());
        assert!(dir.join("untitled_2").join("fomod").join("info.xml").is_file());
        // A marker pointing at an existing folder is consumed once.
        let marker = base.join(AUTOSAVE_MARKER);
        std::fs::write(&marker, dir.to_string_lossy().as_bytes()).unwrap();
        assert_eq!(take_marker(&marker), Some(dir.clone()));
        assert!(!marker.exists());
        assert_eq!(take_marker(&marker), None);
        // A marker naming a missing folder yields nothing.
        std::fs::write(&marker, base.join("gone").to_string_lossy().as_bytes()).unwrap();
        assert_eq!(take_marker(&marker), None);
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn snapshot_throttle_and_update() {
        update(Vec::new());
        assert!(!wants_snapshot(), "a snapshot was just taken");
        if let Ok(mut s) = STATE.lock() {
            s.last_update = Some(Instant::now() - SNAPSHOT_INTERVAL * 2);
        }
        assert!(wants_snapshot());
    }
}
