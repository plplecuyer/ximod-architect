//! Distribution packaging: build a ready-to-upload archive of the mod.
//!
//! The FOMOD XML (`fomod/info.xml` + `fomod/ModuleConfig.xml`) is written to the
//! root directory, then the whole root is zipped into a single archive whose
//! layout is exactly what a mod manager expects (the `fomod/` folder and the mod
//! files at the archive root). Shared by the GUI (menu) and the CLI.

use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::models::Ximod;

/// Files/directories never included in a distribution archive.
fn is_junk(name: &str) -> bool {
    matches!(name, ".git" | ".gitignore" | ".DS_Store" | "desktop.ini")
        || name.eq_ignore_ascii_case("Thumbs.db")
}

/// Write the FOMOD XML into `root_dir`, then package the whole directory into
/// `out_zip`. Returns the number of files written into the archive.
pub fn build_distribution_archive(ximod: &Ximod, root_dir: &Path, out_zip: &Path) -> Result<usize> {
    crate::xml::save_ximod(ximod, root_dir)
        .with_context(|| "writing the FOMOD XML before packaging")?;
    zip_directory(root_dir, out_zip)
}

/// Zip the contents of `root` (recursively) into `out_zip`, using relative paths
/// with forward slashes. Junk files and the output archive itself are skipped.
pub fn zip_directory(root: &Path, out_zip: &Path) -> Result<usize> {
    let out_abs = out_zip
        .canonicalize()
        .unwrap_or_else(|_| out_zip.to_path_buf());

    let mut rels: Vec<PathBuf> = Vec::new();
    collect_files(root, root, &out_abs, &mut rels)?;
    rels.sort();

    let file = std::fs::File::create(out_zip)
        .with_context(|| format!("creating {}", out_zip.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    let opts: zip::write::FileOptions<()> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for rel in &rels {
        let abs = root.join(rel);
        let name = rel.to_string_lossy().replace('\\', "/");
        zip.start_file(name, opts)?;
        let data = std::fs::read(&abs).with_context(|| format!("reading {}", abs.display()))?;
        zip.write_all(&data)?;
    }
    zip.finish()?;
    Ok(rels.len())
}

fn collect_files(root: &Path, dir: &Path, out_abs: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if is_junk(&name) {
            continue;
        }
        // Never include the archive we are writing.
        if path.canonicalize().map(|p| p == *out_abs).unwrap_or(false) {
            continue;
        }
        if path.is_dir() {
            collect_files(root, &path, out_abs, out)?;
        } else if path.is_file() {
            if let Ok(rel) = path.strip_prefix(root) {
                out.push(rel.to_path_buf());
            }
        }
    }
    Ok(())
}

/// Default archive file name for a project: `<name>-<version>.zip`, sanitised.
pub fn default_archive_name(ximod: &Ximod) -> String {
    let base = if ximod.name.trim().is_empty() {
        "fomod".to_string()
    } else {
        ximod.name.trim().to_string()
    };
    let ver = ximod.version.trim();
    let stem = if ver.is_empty() {
        base
    } else {
        format!("{base}-{ver}")
    };
    let safe: String = stem
        .chars()
        .map(|c| if r#"\/:*?"<>|"#.contains(c) { '_' } else { c })
        .collect();
    format!("{safe}.zip")
}
