//! Distribution packaging: build a ready-to-upload archive of the mod.
//!
//! The FOMOD XML (`fomod/info.xml` + `fomod/ModuleConfig.xml`) is written to the
//! root directory, then the whole root is zipped into a single archive whose
//! layout is exactly what a mod manager expects (the `fomod/` folder and the mod
//! files at the archive root). Shared by the GUI (menu) and the CLI.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::models::Ximod;

/// Files/directories never included in a distribution archive.
///
/// Besides the usual VCS/OS clutter this covers the by-products of the
/// "translate an existing FOMOD" feature: translation sidecars
/// (`*.ximod-translation`), the `*.bak` copies made before an in-place apply,
/// and the `fomod_<lang>` folders holding a translated copy of the installer.
pub(crate) fn is_junk(name: &str) -> bool {
    matches!(name, ".git" | ".gitignore" | ".DS_Store" | "desktop.ini")
        || name.eq_ignore_ascii_case("Thumbs.db")
        || is_translation_byproduct(name)
}

/// Directories never included in a distribution archive, identified by their
/// root-relative path: the rotating backups XIMOD keeps under
/// `fomod/backups/` (see `backups.rs`) and the simulator scenarios under
/// `fomod/scenarios/` (see `models/simulate.rs`). `rel` uses either separator.
pub(crate) fn is_junk_dir(rel: &Path) -> bool {
    let rel = rel.to_string_lossy().to_ascii_lowercase();
    let mut parts = rel.split(['/', '\\']).filter(|c| *c != "." && !c.is_empty());
    parts.next() == Some("fomod")
        && parts
            .next()
            .is_some_and(|d| d == crate::backups::BACKUPS_DIR || d == crate::models::simulate::SCENARIOS_DIR)
        && parts.next().is_none()
}

/// Sidecars, backups and `fomod_<xx>` / `fomod_<xxx>` folders (ASCII case is
/// ignored, as on the file systems mods are usually built on).
fn is_translation_byproduct(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".ximod-translation") || lower.ends_with(".bak") {
        return true;
    }
    lower
        .strip_prefix("fomod_")
        .is_some_and(|lang| (2..=3).contains(&lang.len()) && lang.bytes().all(|b| b.is_ascii_lowercase()))
}

/// Progress callback for the archive writers: `(files_done, files_total,
/// current_relative_path)`. Returning `false` cancels the operation; the
/// partially written archive is then removed and `Err` is returned.
pub type Progress<'a> = &'a mut dyn FnMut(usize, usize, &Path) -> bool;

/// Error returned when a progress callback asked to stop.
#[derive(Debug)]
pub struct Cancelled;

impl std::fmt::Display for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("export cancelled")
    }
}

impl std::error::Error for Cancelled {}

/// Write the FOMOD XML into `root_dir`, then package the whole directory into
/// `out_zip`. Returns the number of files written into the archive.
pub fn build_distribution_archive(ximod: &Ximod, root_dir: &Path, out_zip: &Path) -> Result<usize> {
    crate::xml::save_ximod(ximod, root_dir).with_context(|| "writing the FOMOD XML before packaging")?;
    zip_directory(root_dir, out_zip)
}

/// Collect the relative paths of every file to include in a distribution archive
/// of `root`, skipping junk files and the output archive itself. Sorted, with the
/// original path separators (callers normalize to forward slashes for entry
/// names). Shared by the ZIP and 7z writers.
pub fn collect_distribution_files(root: &Path, out_archive: &Path) -> Result<Vec<PathBuf>> {
    let out_abs = out_archive.canonicalize().unwrap_or_else(|_| out_archive.to_path_buf());
    let mut rels: Vec<PathBuf> = Vec::new();
    collect_files(root, root, &out_abs, &mut rels)?;
    rels.sort();
    Ok(rels)
}

/// Zip the contents of `root` (recursively) into `out_zip`, using relative paths
/// with forward slashes. Junk files and the output archive itself are skipped.
pub fn zip_directory(root: &Path, out_zip: &Path) -> Result<usize> {
    zip_directory_with(root, out_zip, &mut |_, _, _| true)
}

/// [`zip_directory`] with a progress/cancellation callback.
///
/// Files are streamed with `io::copy` instead of being read whole into memory
/// (a multi-gigabyte BSA/BA2 used to mean an equally large allocation), and
/// `large_file` is enabled so entries above 4 GiB do not fail.
pub fn zip_directory_with(root: &Path, out_zip: &Path, progress: Progress<'_>) -> Result<usize> {
    let rels = collect_distribution_files(root, out_zip)?;
    let total = rels.len();

    let result = (|| -> Result<usize> {
        let file = std::fs::File::create(out_zip).with_context(|| format!("creating {}", out_zip.display()))?;
        let mut zip = zip::ZipWriter::new(std::io::BufWriter::new(file));
        let opts: zip::write::FileOptions<()> = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .large_file(true);

        for (i, rel) in rels.iter().enumerate() {
            if !progress(i, total, rel) {
                return Err(Cancelled.into());
            }
            let abs = root.join(rel);
            let name = rel.to_string_lossy().replace('\\', "/");
            zip.start_file(name, opts)?;
            let mut reader = std::io::BufReader::new(
                std::fs::File::open(&abs).with_context(|| format!("reading {}", abs.display()))?,
            );
            std::io::copy(&mut reader, &mut zip).with_context(|| format!("adding {} to the archive", abs.display()))?;
        }
        progress(total, total, Path::new(""));
        zip.finish()?.into_inner().map_err(|e| e.into_error())?;
        Ok(total)
    })();

    if result.is_err() {
        let _ = std::fs::remove_file(out_zip);
    }
    result
}

fn collect_files(root: &Path, dir: &Path, out_abs: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if is_junk(&name) {
            continue;
        }
        // Never include the archive we are writing. Only canonicalise when the
        // file name matches: one `canonicalize` per entry was a syscall per
        // file on large mods.
        if path.file_name() == out_abs.file_name() && path.canonicalize().map(|p| p == *out_abs).unwrap_or(false) {
            continue;
        }
        if path.is_dir() {
            if path.strip_prefix(root).is_ok_and(is_junk_dir) {
                continue;
            }
            collect_files(root, &path, out_abs, out)?;
        } else if path.is_file()
            && let Ok(rel) = path.strip_prefix(root)
        {
            out.push(rel.to_path_buf());
        }
    }
    Ok(())
}

/// Default archive file name for a project: `<name>-<version>.zip`, sanitised.
pub fn default_archive_name(ximod: &Ximod) -> String {
    default_archive_name_with_ext(ximod, "zip")
}

/// Default archive file name with an explicit extension (e.g. `"zip"`, `"7z"`),
/// sanitised: `<name>-<version>.<ext>`.
pub fn default_archive_name_with_ext(ximod: &Ximod, ext: &str) -> String {
    let base = if ximod.name.trim().is_empty() {
        "fomod".to_string()
    } else {
        ximod.name.trim().to_string()
    };
    let ver = ximod.version.trim();
    let stem = if ver.is_empty() { base } else { format!("{base}-{ver}") };
    let safe: String = stem
        .chars()
        .map(|c| if r#"\/:*?"<>|"#.contains(c) { '_' } else { c })
        .collect();
    format!("{safe}.{ext}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_root(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod_export_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("fomod")).unwrap();
        std::fs::create_dir_all(dir.join("textures")).unwrap();
        std::fs::write(dir.join("fomod/info.xml"), "<fomod/>").unwrap();
        std::fs::write(dir.join("textures/a.dds"), vec![7u8; 4096]).unwrap();
        std::fs::write(dir.join("Thumbs.db"), b"junk").unwrap();
        dir
    }

    #[test]
    fn zip_reports_progress_and_skips_junk() {
        let root = sample_root("progress");
        let out = root.join("out.zip");
        let mut seen: Vec<(usize, usize)> = Vec::new();
        let n = zip_directory_with(&root, &out, &mut |done, total, _| {
            seen.push((done, total));
            true
        })
        .unwrap();
        assert_eq!(n, 2, "junk and the output archive itself are excluded");
        assert_eq!(seen.first(), Some(&(0, 2)));
        assert_eq!(seen.last(), Some(&(2, 2)));
        assert!(out.is_file());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn zip_cancellation_removes_partial_archive() {
        let root = sample_root("cancel");
        let out = root.join("out.zip");
        let err = zip_directory_with(&root, &out, &mut |done, _, _| done == 0).unwrap_err();
        assert!(err.downcast_ref::<Cancelled>().is_some(), "{err}");
        assert!(!out.exists(), "partial archive must be deleted");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn translation_byproducts_are_junk() {
        for name in [
            "Mod.fra.ximod-translation",
            "info.xml.20260101-120000.bak",
            "ModuleConfig.XML.BAK",
            "fomod_fr",
            "fomod_fra",
            "Fomod_DE",
        ] {
            assert!(is_junk(name), "{name}");
        }
        for name in [
            "fomod",
            "fomod_",
            "fomod_x",
            "fomod_fran",
            "fomod_f1",
            "translations",
            "info.xml",
            "backup",
        ] {
            assert!(!is_junk(name), "{name}");
        }
    }

    #[test]
    fn fomod_backups_folder_is_excluded_from_archives() {
        assert!(is_junk_dir(Path::new("fomod/backups")));
        assert!(is_junk_dir(Path::new("Fomod\\Backups")));
        assert!(is_junk_dir(Path::new("fomod/scenarios")));
        assert!(!is_junk_dir(Path::new("fomod/scenarios/x.json")));
        assert!(!is_junk_dir(Path::new("fomod")));
        assert!(!is_junk_dir(Path::new("backups")));
        assert!(!is_junk_dir(Path::new("textures/backups")));
        assert!(!is_junk_dir(Path::new("fomod/backups/20260101-100000")));

        let root = sample_root("backups");
        let bk = root.join("fomod").join("backups").join("20260101-100000");
        std::fs::create_dir_all(&bk).unwrap();
        std::fs::write(bk.join("info.xml"), "<fomod/>").unwrap();
        std::fs::write(bk.join("ModuleConfig.xml"), "<config/>").unwrap();
        let out = root.join("out.zip");
        let rels = collect_distribution_files(&root, &out).unwrap();
        assert_eq!(rels.len(), 2, "{rels:?}");
        assert!(
            rels.iter().all(|r| !r.to_string_lossy().contains("backups")),
            "{rels:?}"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn default_archive_name_is_sanitised() {
        let mut x = Ximod::new("My: Mod/Name");
        x.version = "1.2".into();
        assert_eq!(default_archive_name(&x), "My_ Mod_Name-1.2.zip");
        assert_eq!(default_archive_name_with_ext(&Ximod::default(), "7z"), "fomod.7z");
    }
}
