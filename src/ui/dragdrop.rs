//! Drag-and-drop file assignment (V2 roadmap, priority 8).
//!
//! Instead of typing sources into the files table, the author can drop files or
//! folders (from the OS) onto the window to assign them to the currently selected
//! option. Dropped paths under the project root become root-relative sources
//! (files → file entries, directories → folder entries); paths outside the root
//! are rejected (they would not be portable inside the archive).
//!
//! egui exposes dropped files via `ctx.input(|i| i.raw.dropped_files)`.
//!
//! Status (Lot B): helpers implemented; wired to the selected plugin in the
//! options editor.

use std::path::{Path, PathBuf};

use eframe::egui;

use crate::models::{InstallFile, Plugin};

/// Collect files/folders the user dropped onto the window this frame (absolute
/// paths), if any.
pub fn dropped_paths(ctx: &egui::Context) -> Vec<PathBuf> {
    ctx.input(|i| i.raw.dropped_files.iter().filter_map(|f| f.path.clone()).collect())
}

/// Make `path` a root-relative, forward-slash source if it is inside `root`.
/// Returns `None` for paths outside the root.
fn relativize(root: &Path, path: &Path) -> Option<String> {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let rel = path.strip_prefix(&root).ok()?;
    let s = rel.to_string_lossy().replace('\\', "/");
    if s.is_empty() { None } else { Some(s) }
}

/// Assign dropped `paths` to `plugin` as file/folder sources, relative to `root`.
/// Returns `(added, rejected)` counts. Duplicate sources (already present) are not
/// added again; paths outside the root are counted as rejected.
pub fn assign_dropped_to_plugin(plugin: &mut Plugin, paths: &[PathBuf], root: &Path) -> (usize, usize) {
    let mut added = 0;
    let mut rejected = 0;
    for p in paths {
        let Some(src) = relativize(root, p) else {
            rejected += 1;
            continue;
        };
        if plugin.files.iter().any(|f| f.source.replace('\\', "/") == src) {
            continue; // already present
        }
        let is_dir = root.join(&src).is_dir() || p.is_dir();
        plugin.files.push(if is_dir {
            InstallFile::new_folder(src)
        } else {
            InstallFile::new_file(src)
        });
        added += 1;
    }
    (added, rejected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Plugin;
    use std::fs;

    #[test]
    fn assigning_nothing_adds_nothing() {
        let mut p = Plugin::new("Opt");
        let (a, r) = assign_dropped_to_plugin(&mut p, &[], Path::new("."));
        assert_eq!((a, r), (0, 0));
    }

    #[test]
    fn inside_root_added_outside_rejected() {
        let dir = std::env::temp_dir().join(format!("ximod_dd_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("textures")).unwrap();
        fs::write(dir.join("a.esp"), b"x").unwrap();
        fs::write(dir.join("textures/t.dds"), b"x").unwrap();

        let mut p = Plugin::new("Opt");
        let inside_file = dir.join("a.esp");
        let inside_dir = dir.join("textures");
        let outside = std::env::temp_dir().join("ximod_dd_outside_zzz.esp");
        let _ = fs::write(&outside, b"x");

        let (added, rejected) =
            assign_dropped_to_plugin(&mut p, &[inside_file.clone(), inside_dir, outside.clone()], &dir);
        assert_eq!(added, 2);
        assert_eq!(rejected, 1);
        // dropping the same file again does not duplicate it
        let (added2, _) = assign_dropped_to_plugin(&mut p, &[inside_file], &dir);
        assert_eq!(added2, 0);
        assert!(p.files.iter().any(|f| f.source == "a.esp"));
        assert!(
            p.files
                .iter()
                .any(|f| f.source == "textures" && matches!(f.file_type, crate::models::FileType::Folder))
        );

        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_file(&outside);
    }
}
