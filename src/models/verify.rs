//! Referenced-file verification (V2 roadmap, priority 1).
//!
//! Checks that every file/folder source and every image referenced by a project
//! really exists under the root folder, that no reference is absolute or escapes
//! the root, and (informative) which files under the root are not referenced by
//! any option — problems the FOMOD schema cannot catch.
//!
//! The model stays free of i18n: [`FileIssue`] / [`RefLoc`] are language-neutral;
//! the UI layer maps them to translated messages, exactly like [`ValidationError`].

use std::collections::HashSet;
use std::path::Path;

use walkdir::WalkDir;

use super::{FileType, Ximod};

/// Where a reference lives in the project (for building a context message).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefLoc {
    /// The module header image.
    Header,
    /// The always-installed "required files" list.
    RequiredFiles,
    /// A conditional file set (1-based index).
    ConditionalSet { index: usize },
    /// A plugin (option), by 1-based step/group index and plugin name.
    Plugin {
        step: usize,
        group: usize,
        plugin: String,
    },
}

/// A problem found while verifying referenced files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileIssue {
    /// A file/folder source does not exist under the root.
    MissingSource {
        loc: RefLoc,
        path: String,
        folder: bool,
    },
    /// An image does not exist under the root.
    MissingImage { loc: RefLoc, path: String },
    /// A reference is an absolute path (not portable inside the archive).
    AbsolutePath { loc: RefLoc, path: String },
    /// A reference escapes the root folder (contains "..").
    OutsideRoot { loc: RefLoc, path: String },
    /// A file present under the root is referenced by no option (informative).
    OrphanFile { path: String },
}

/// Upper bound on reported orphan files, to avoid a runaway report on huge trees.
const MAX_ORPHANS: usize = 500;

/// Verify all referenced files/images of `ximod` against the `root` folder.
pub fn verify_files(ximod: &Ximod, root: &Path) -> Vec<FileIssue> {
    let mut issues = Vec::new();
    // Normalized (lowercased, forward-slash) references that exist, for the
    // orphan pass: exact file/image paths and folder-source prefixes.
    let mut ref_files: HashSet<String> = HashSet::new();
    let mut ref_dirs: Vec<String> = Vec::new();

    // ---- header image ----
    if let Some(img) = image_of(&ximod.header_image) {
        register(
            check_ref(root, RefLoc::Header, img, false, true, &mut issues),
            false,
            &mut ref_files,
            &mut ref_dirs,
        );
    }

    // ---- required files ----
    for f in &ximod.required_files {
        let folder = f.file_type == FileType::Folder;
        register(
            check_ref(root, RefLoc::RequiredFiles, &f.source, folder, false, &mut issues),
            folder,
            &mut ref_files,
            &mut ref_dirs,
        );
    }

    // ---- conditional file sets ----
    for (i, set) in ximod.conditional_files.iter().enumerate() {
        for f in &set.files {
            let folder = f.file_type == FileType::Folder;
            register(
                check_ref(
                    root,
                    RefLoc::ConditionalSet { index: i + 1 },
                    &f.source,
                    folder,
                    false,
                    &mut issues,
                ),
                folder,
                &mut ref_files,
                &mut ref_dirs,
            );
        }
    }

    // ---- steps / groups / plugins ----
    for (si, step) in ximod.steps.iter().enumerate() {
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            for plugin in &group.plugins {
                let loc = || RefLoc::Plugin {
                    step: si + 1,
                    group: gi + 1,
                    plugin: plugin.name.clone(),
                };
                if let Some(img) = image_of(&plugin.image_path) {
                    register(
                        check_ref(root, loc(), img, false, true, &mut issues),
                        false,
                        &mut ref_files,
                        &mut ref_dirs,
                    );
                }
                for f in &plugin.files {
                    let folder = f.file_type == FileType::Folder;
                    register(
                        check_ref(root, loc(), &f.source, folder, false, &mut issues),
                        folder,
                        &mut ref_files,
                        &mut ref_dirs,
                    );
                }
            }
        }
    }

    // ---- orphan files under the root ----
    collect_orphans(root, &ref_files, &ref_dirs, &mut issues);

    issues
}

fn image_of(opt: &Option<String>) -> Option<&str> {
    opt.as_deref().map(str::trim).filter(|s| !s.is_empty())
}

/// Check one reference. Returns the normalized (lowercased) relative path when it
/// exists inside the root (so the orphan pass can mark it referenced); otherwise
/// pushes the appropriate issue and returns `None`.
fn check_ref(
    root: &Path,
    loc: RefLoc,
    raw: &str,
    folder: bool,
    is_image: bool,
    issues: &mut Vec<FileIssue>,
) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    if is_absolute_ref(raw) {
        issues.push(FileIssue::AbsolutePath { loc, path: raw.to_string() });
        return None;
    }
    let norm = raw.replace('\\', "/");
    let norm = norm.trim_start_matches("./");
    if norm.split('/').any(|c| c == "..") {
        issues.push(FileIssue::OutsideRoot { loc, path: raw.to_string() });
        return None;
    }
    let abs = root.join(norm);
    let exists = if folder { abs.is_dir() } else { abs.is_file() };
    if !exists {
        if is_image {
            issues.push(FileIssue::MissingImage { loc, path: raw.to_string() });
        } else {
            issues.push(FileIssue::MissingSource { loc, path: raw.to_string(), folder });
        }
        return None;
    }
    Some(norm.to_lowercase())
}

fn register(
    normalized: Option<String>,
    folder: bool,
    ref_files: &mut HashSet<String>,
    ref_dirs: &mut Vec<String>,
) {
    if let Some(n) = normalized {
        if folder {
            if !ref_dirs.contains(&n) {
                ref_dirs.push(n);
            }
        } else {
            ref_files.insert(n);
        }
    }
}

/// True for absolute references: POSIX root, UNC/backslash root, or a `C:` drive.
fn is_absolute_ref(p: &str) -> bool {
    let b = p.as_bytes();
    p.starts_with('/')
        || p.starts_with('\\')
        || (b.len() >= 2 && b[1] == b':' && b[0].is_ascii_alphabetic())
}

fn collect_orphans(
    root: &Path,
    ref_files: &HashSet<String>,
    ref_dirs: &[String],
    issues: &mut Vec<FileIssue>,
) {
    let mut count = 0usize;
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(rel) = entry.path().strip_prefix(root) else {
            continue;
        };
        let rel = rel.to_string_lossy().replace('\\', "/");
        let rel_lower = rel.to_lowercase();
        // The FOMOD installer's own files are never "orphans".
        if rel_lower == "fomod" || rel_lower.starts_with("fomod/") {
            continue;
        }
        let referenced = ref_files.contains(&rel_lower)
            || ref_dirs
                .iter()
                .any(|d| rel_lower == *d || rel_lower.starts_with(&format!("{d}/")));
        if !referenced {
            if count < MAX_ORPHANS {
                issues.push(FileIssue::OrphanFile { path: rel });
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{InstallFile, Plugin, PluginGroup, SelectionType, Step, Ximod};
    use std::fs;

    fn touch(root: &Path, rel: &str) {
        let p = root.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(p, b"x").unwrap();
    }

    #[test]
    fn test_absolute_detection() {
        assert!(is_absolute_ref("C:\\mods\\a.esp"));
        assert!(is_absolute_ref("/home/x/a.esp"));
        assert!(is_absolute_ref("\\\\share\\a"));
        assert!(!is_absolute_ref("textures/a.dds"));
        assert!(!is_absolute_ref("a.esp"));
    }

    #[test]
    fn test_missing_orphan_outside_absolute() {
        let dir = std::env::temp_dir().join(format!("ximod_verify_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        touch(&dir, "MyMod.esp");
        touch(&dir, "textures/present.dds");
        touch(&dir, "textures/orphan.dds");
        touch(&dir, "fomod/ModuleConfig.xml"); // must be ignored by orphan pass

        let mut x = Ximod::new("Test");
        // required: a file that exists, and a folder that exists
        x.required_files.push(InstallFile::new_file("MyMod.esp"));
        x.required_files.push(InstallFile::new_folder("textures"));
        // an option referencing a missing file, an absolute path and an escaping one
        let mut plugin = Plugin::new("Opt");
        plugin.files.push(InstallFile::new_file("missing.esp"));
        plugin.files.push(InstallFile::new_file("C:\\abs\\x.esp"));
        plugin.files.push(InstallFile::new_file("..\\outside.esp"));
        plugin.image_path = Some("fomod/nope.png".to_string()); // missing image
        let mut group = PluginGroup::new("G", SelectionType::SelectAny);
        group.plugins.push(plugin);
        let mut step = Step::new("S");
        step.plugin_groups.push(group);
        x.steps.push(step);

        let issues = verify_files(&x, &dir);

        // Both textures files are covered by the folder source -> not orphan.
        assert!(!issues.iter().any(|i| matches!(i, FileIssue::OrphanFile { path } if path.contains("present"))));
        assert!(!issues.iter().any(|i| matches!(i, FileIssue::OrphanFile { path } if path.contains("orphan"))));
        // Actually "orphan.dds" IS under the referenced "textures" folder, so it
        // is covered too. Add a truly orphan file to be sure detection works.
        touch(&dir, "loose_readme.txt");
        let issues = verify_files(&x, &dir);
        assert!(issues.iter().any(|i| matches!(i, FileIssue::OrphanFile { path } if path == "loose_readme.txt")));

        assert!(issues.iter().any(|i| matches!(i, FileIssue::MissingSource { path, .. } if path == "missing.esp")));
        assert!(issues.iter().any(|i| matches!(i, FileIssue::AbsolutePath { .. })));
        assert!(issues.iter().any(|i| matches!(i, FileIssue::OutsideRoot { .. })));
        assert!(issues.iter().any(|i| matches!(i, FileIssue::MissingImage { .. })));

        let _ = fs::remove_dir_all(&dir);
    }
}
