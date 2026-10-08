//! Rotating backups of the FOMOD XML files (V2 roadmap, feature 3).
//!
//! Every save of an existing project first copies the previous `info.xml` and
//! `ModuleConfig.xml` into `<root>/fomod/backups/<YYYYMMDD-HHMMSS>/`, then
//! prunes the folder to the configured number of backups. A save whose
//! output would be byte-identical to what is already on disk makes no backup
//! (it would only clutter the folder with identical copies).
//!
//! The backup folder lives under `fomod/`, so it is never an "orphan" for the
//! file verification, and `export::is_junk_dir` keeps it out of distribution
//! archives.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::models::Ximod;

/// Name of the backup folder under `fomod/`.
pub const BACKUPS_DIR: &str = "backups";

/// The two files a backup holds.
const BACKED_UP: [&str; 2] = ["info.xml", "ModuleConfig.xml"];

/// Timestamp format of a backup folder name.
const STAMP_FORMAT: &str = "%Y%m%d-%H%M%S";

/// One backup folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupEntry {
    /// Folder name, `YYYYMMDD-HHMMSS`.
    pub stamp: String,
    /// Absolute path of the folder.
    pub path: PathBuf,
}

impl BackupEntry {
    /// The timestamp formatted for display: `YYYY-MM-DD HH:MM:SS`.
    pub fn display_time(&self) -> String {
        format_stamp(&self.stamp)
    }
}

/// `YYYYMMDD-HHMMSS` → `YYYY-MM-DD HH:MM:SS`; anything else is returned as is.
pub fn format_stamp(stamp: &str) -> String {
    if !is_stamp(stamp) {
        return stamp.to_string();
    }
    format!(
        "{}-{}-{} {}:{}:{}",
        &stamp[0..4],
        &stamp[4..6],
        &stamp[6..8],
        &stamp[9..11],
        &stamp[11..13],
        &stamp[13..15]
    )
}

/// Whether a folder name is a backup timestamp (`YYYYMMDD-HHMMSS`).
pub fn is_stamp(name: &str) -> bool {
    let b = name.as_bytes();
    b.len() == 15 && b[8] == b'-' && b[..8].iter().chain(&b[9..]).all(u8::is_ascii_digit)
}

/// `<root>/fomod/backups`.
pub fn backups_dir(root: &Path) -> PathBuf {
    crate::xml::installer_dir(root).join(BACKUPS_DIR)
}

/// The backups of `root`, newest first. Folders whose name is not a
/// timestamp are ignored.
pub fn list_backups(root: &Path) -> Vec<BackupEntry> {
    let dir = backups_dir(root);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut out: Vec<BackupEntry> = entries
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter_map(|e| {
            let stamp = e.file_name().to_string_lossy().into_owned();
            is_stamp(&stamp).then(|| BackupEntry { stamp, path: e.path() })
        })
        .collect();
    // The stamp sorts chronologically; newest first.
    out.sort_by(|a, b| b.stamp.cmp(&a.stamp));
    out
}

/// Whether `root` has at least one backup.
pub fn has_backups(root: &Path) -> bool {
    !list_backups(root).is_empty()
}

/// The bytes XIMOD writes for the two files, minus the BOM (so they compare
/// equal to a file read back with its BOM stripped).
fn upcoming_content(ximod: &Ximod) -> Result<[String; 2]> {
    Ok([
        crate::xml::info_xml_to_string(ximod)?,
        crate::xml::module_config_to_string(ximod)?,
    ])
}

/// Read a file and drop a leading UTF-8 BOM.
fn read_without_bom(path: &Path) -> Option<Vec<u8>> {
    let bytes = std::fs::read(path).ok()?;
    Some(match bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        Some(rest) => rest.to_vec(),
        None => bytes,
    })
}

/// Back up the XML files currently under `<root>/fomod/` before `ximod` is
/// written over them, then prune to `keep` backups.
///
/// Returns the new backup folder, or `None` when nothing was backed up: no
/// previous version on disk, backups disabled (`keep == 0`), or a previous
/// version identical to what is about to be written.
pub fn backup_before_save(ximod: &Ximod, root: &Path, keep: usize) -> Result<Option<PathBuf>> {
    if keep == 0 {
        return Ok(None);
    }
    let fomod = crate::xml::installer_dir(root);
    let existing: Vec<PathBuf> = BACKED_UP
        .iter()
        .map(|n| crate::xml::installer_file(&fomod, n))
        .filter(|p| p.is_file())
        .collect();
    if existing.is_empty() {
        return Ok(None);
    }
    // Skip the backup when the save would change nothing on disk.
    let upcoming = upcoming_content(ximod)?;
    let unchanged = BACKED_UP
        .iter()
        .zip(&upcoming)
        .all(|(name, content)| read_without_bom(&fomod.join(name)).is_some_and(|bytes| bytes == content.as_bytes()));
    if unchanged {
        return Ok(None);
    }
    let stamp = chrono::Local::now().format(STAMP_FORMAT).to_string();
    let dest = backups_dir(root).join(&stamp);
    std::fs::create_dir_all(&dest).with_context(|| format!("creating {}", dest.display()))?;
    for src in &existing {
        let name = src.file_name().unwrap_or_default();
        std::fs::copy(src, dest.join(name)).with_context(|| format!("backing up {}", src.display()))?;
    }
    prune(root, keep)?;
    Ok(Some(dest))
}

/// Delete the oldest backups so that at most `keep` remain. Returns the
/// number of folders removed.
pub fn prune(root: &Path, keep: usize) -> Result<usize> {
    let backups = list_backups(root);
    let mut removed = 0;
    for entry in backups.iter().skip(keep) {
        std::fs::remove_dir_all(&entry.path).with_context(|| format!("removing {}", entry.path.display()))?;
        removed += 1;
    }
    Ok(removed)
}

/// Delete every backup of `root` (and the `backups` folder itself).
pub fn delete_all(root: &Path) -> Result<usize> {
    let n = list_backups(root).len();
    let dir = backups_dir(root);
    if dir.is_dir() {
        std::fs::remove_dir_all(&dir).with_context(|| format!("removing {}", dir.display()))?;
    }
    Ok(n)
}

/// Load the project held by one backup folder.
pub fn load_backup(entry: &Path) -> Result<Ximod> {
    if !entry.join("info.xml").is_file() && !entry.join("ModuleConfig.xml").is_file() {
        anyhow::bail!("no FOMOD XML files in {}", entry.display());
    }
    crate::xml::load_from_installer_dir(entry).map(|(x, _)| x)
}

/// The backup of `root` whose stamp is `stamp`, if any.
pub fn find_backup(root: &Path, stamp: &str) -> Option<BackupEntry> {
    list_backups(root).into_iter().find(|b| b.stamp == stamp)
}

/// Restore a backup over the project files: the backup's XML is loaded and
/// written back as the current project (the previous version is itself
/// backed up first, so a restore can be undone with another restore).
pub fn restore_to_disk(root: &Path, stamp: &str, keep: usize) -> Result<Ximod> {
    let entry =
        find_backup(root, stamp).ok_or_else(|| anyhow::anyhow!("no backup {stamp} under {}", root.display()))?;
    let ximod = load_backup(&entry.path)?;
    backup_before_save(&ximod, root, keep)?;
    crate::xml::save_ximod(&ximod, root)?;
    Ok(ximod)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Step;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod-backups-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn project(name: &str) -> Ximod {
        let mut x = Ximod::new(name);
        x.steps.push(Step::new("S"));
        x
    }

    /// Fake a backup folder with the given stamp (the real ones are made
    /// within one second of each other and would collide).
    fn fake_backup(root: &Path, stamp: &str, ximod: &Ximod) {
        let dir = backups_dir(root).join(stamp);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("info.xml"), crate::xml::info_xml_to_string(ximod).unwrap()).unwrap();
        std::fs::write(
            dir.join("ModuleConfig.xml"),
            crate::xml::module_config_to_string(ximod).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn stamps_are_recognised_and_formatted() {
        assert!(is_stamp("20260105-134501"));
        assert!(!is_stamp("2026-01-05"));
        assert!(!is_stamp("20260105_134501"));
        assert!(!is_stamp("backup"));
        assert_eq!(format_stamp("20260105-134501"), "2026-01-05 13:45:01");
        assert_eq!(format_stamp("odd"), "odd");
    }

    #[test]
    fn first_save_makes_no_backup_and_identical_content_is_skipped() {
        let root = scratch("identical");
        let x = project("A");
        assert_eq!(backup_before_save(&x, &root, 10).unwrap(), None);
        crate::xml::save_ximod(&x, &root).unwrap();
        // Same content again: nothing to back up.
        assert_eq!(backup_before_save(&x, &root, 10).unwrap(), None);
        assert!(list_backups(&root).is_empty());
        // Changed content: one backup holding the previous version.
        let mut y = x.clone();
        y.name = "B".into();
        let made = backup_before_save(&y, &root, 10).unwrap().expect("a backup");
        assert!(made.join("info.xml").is_file() && made.join("ModuleConfig.xml").is_file());
        let backups = list_backups(&root);
        assert_eq!(backups.len(), 1);
        assert_eq!(backups[0].path, made);
        assert_eq!(load_backup(&made).unwrap().name, "A");
        // Backups disabled: nothing is made.
        let mut z = y.clone();
        z.name = "C".into();
        assert_eq!(backup_before_save(&z, &root, 0).unwrap(), None);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn rotation_keeps_the_newest_n() {
        let root = scratch("rotate");
        let x = project("A");
        for stamp in [
            "20260101-100000",
            "20260103-100000",
            "20260102-100000",
            "20260104-100000",
        ] {
            fake_backup(&root, stamp, &x);
        }
        // A stray folder is ignored.
        std::fs::create_dir_all(backups_dir(&root).join("notes")).unwrap();
        let listed: Vec<String> = list_backups(&root).into_iter().map(|b| b.stamp).collect();
        assert_eq!(
            listed,
            vec![
                "20260104-100000".to_string(),
                "20260103-100000".to_string(),
                "20260102-100000".to_string(),
                "20260101-100000".to_string()
            ]
        );
        assert_eq!(prune(&root, 2).unwrap(), 2);
        let kept: Vec<String> = list_backups(&root).into_iter().map(|b| b.stamp).collect();
        assert_eq!(kept, listed[..2].to_vec());
        assert!(has_backups(&root));
        assert_eq!(delete_all(&root).unwrap(), 2);
        assert!(!has_backups(&root));
        assert!(!backups_dir(&root).exists());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn restore_to_disk_writes_the_backup_back_and_keeps_the_current_version() {
        let root = scratch("restore");
        let old = project("Old");
        fake_backup(&root, "20260101-100000", &old);
        let current = project("Current");
        crate::xml::save_ximod(&current, &root).unwrap();
        let restored = restore_to_disk(&root, "20260101-100000", 10).unwrap();
        assert_eq!(restored.name, "Old");
        assert_eq!(crate::xml::load_ximod(&root).unwrap().name, "Old");
        // The version that was on disk before the restore is now a backup.
        let names: Vec<String> = list_backups(&root)
            .iter()
            .map(|b| load_backup(&b.path).unwrap().name)
            .collect();
        assert!(names.contains(&"Current".to_string()), "{names:?}");
        assert!(restore_to_disk(&root, "19990101-000000", 10).is_err());
        let _ = std::fs::remove_dir_all(&root);
    }
}
