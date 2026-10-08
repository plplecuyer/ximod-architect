//! Folder-structure wizard (V2 roadmap, priority 4).
//!
//! To bootstrap an installer fast, scan the mod's root folder and propose a
//! steps / groups / plugins skeleton from the directory tree — the common
//! convention being "one top-level subfolder = one option". The author then
//! tweaks the proposal instead of building every step by hand.
//!
//! The wizard produces a fresh [`Ximod`]; the caller then sets the project's root
//! to the scanned folder.
//!
//! Status (Lot B): implemented.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::models::{InstallFile, Plugin, PluginGroup, SelectionType, Step, Ximod};

/// How the wizard turns a folder tree into a project skeleton.
#[derive(Debug, Clone)]
pub struct WizardOptions {
    /// Treat each immediate subfolder of the root as one option (plugin).
    pub one_subfolder_per_option: bool,
    /// Selection type to give the generated group.
    pub selection_type: SelectionType,
    /// Group all first-level options under a single step/group (always true for
    /// now; kept for future "one step per folder" layouts).
    #[allow(dead_code)] // reserved for the "one step per folder" layout (later)
    pub single_step: bool,
}

impl Default for WizardOptions {
    fn default() -> Self {
        Self {
            one_subfolder_per_option: true,
            selection_type: SelectionType::SelectAny,
            single_step: true,
        }
    }
}

/// Image file extensions considered as a possible header image.
const IMAGE_EXTS: [&str; 3] = ["png", "jpg", "jpeg"];

/// Scan `root` and propose a project skeleton following `opts`.
///
/// Each immediate subfolder becomes one option (a plugin with a single folder
/// source = that subfolder). All options go into one group on one step. If an
/// obvious header image sits at the root, it is set as the module image.
pub fn propose_from_folder(root: &Path, opts: &WizardOptions) -> Result<Ximod> {
    if !root.is_dir() {
        anyhow::bail!("not a folder: {}", root.display());
    }
    let name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "New Mod".to_string());
    let mut ximod = Ximod::new(name);

    // Collect first-level subfolders and root-level images.
    let mut subdirs: Vec<String> = Vec::new();
    let mut images: Vec<String> = Vec::new();
    for entry in fs::read_dir(root).with_context(|| format!("reading {}", root.display()))? {
        let entry = entry?;
        let fname = entry.file_name().to_string_lossy().to_string();
        let ty = entry.file_type()?;
        if ty.is_dir() {
            // Skip the FOMOD folder and hidden/VCS folders.
            let low = fname.to_lowercase();
            if low == "fomod" || fname.starts_with('.') {
                continue;
            }
            subdirs.push(fname);
        } else if ty.is_file()
            && let Some(ext) = entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .map(str::to_ascii_lowercase)
            && IMAGE_EXTS.contains(&ext.as_str())
        {
            images.push(fname);
        }
    }
    subdirs.sort_by_key(|s| s.to_lowercase());
    images.sort_by_key(|s| s.to_lowercase());

    // Header image: prefer a name hinting at a banner; else the only image.
    ximod.header_image = pick_header_image(&images);

    if opts.one_subfolder_per_option && !subdirs.is_empty() {
        let mut group = PluginGroup::new("Options", opts.selection_type);
        for sub in &subdirs {
            let mut plugin = Plugin::new(sub.clone());
            plugin.files.push(InstallFile::new_folder(sub.clone()));
            group.plugins.push(plugin);
        }
        let mut step = Step::new("Installation");
        step.plugin_groups.push(group);
        ximod.steps.push(step);
    }

    Ok(ximod)
}

/// Choose the most likely header image from a list of root-level image names.
fn pick_header_image(images: &[String]) -> Option<String> {
    if images.is_empty() {
        return None;
    }
    const HINTS: [&str; 4] = ["header", "banner", "splash", "title"];
    for img in images {
        let low = img.to_lowercase();
        if HINTS.iter().any(|h| low.contains(h)) {
            return Some(img.clone());
        }
    }
    // Fall back to the single image if there is exactly one.
    if images.len() == 1 {
        return Some(images[0].clone());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_are_reasonable() {
        let o = WizardOptions::default();
        assert!(o.one_subfolder_per_option);
    }

    #[test]
    fn proposes_one_option_per_subfolder() {
        let dir = std::env::temp_dir().join(format!("ximod_wiz_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("2K Textures")).unwrap();
        fs::create_dir_all(dir.join("4K Textures")).unwrap();
        fs::create_dir_all(dir.join("fomod")).unwrap(); // must be skipped
        fs::write(dir.join("banner.png"), b"x").unwrap();

        let x = propose_from_folder(&dir, &WizardOptions::default()).unwrap();
        assert_eq!(x.steps.len(), 1);
        let g = &x.steps[0].plugin_groups[0];
        assert_eq!(g.plugins.len(), 2, "two subfolders -> two options");
        assert_eq!(x.header_image.as_deref(), Some("banner.png"));
        // each option has a folder source equal to its subfolder
        assert!(g.plugins.iter().all(|p| p.files.len() == 1));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_folder_errors() {
        let p = std::env::temp_dir().join("ximod_wiz_does_not_exist_zzz");
        let _ = fs::remove_dir_all(&p);
        assert!(propose_from_folder(&p, &WizardOptions::default()).is_err());
    }
}
