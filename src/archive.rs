//! Archive export — 7z (priority 3) and BA2 packaging (priority 12).
//!
//! V1 exports a distribution `.zip` (see [`crate::export`]). V2 adds:
//! - **7z export**, preferred by Nexus for download size, selectable alongside
//!   ZIP with a compression level;
//! - **BA2 packaging**, bundling loose files into Bethesda archives (Skyrim SE,
//!   Fallout 4, Starfield) — an advanced, heavier feature.
//!
//! This module presents one entry point ([`export_archive`]) that dispatches on
//! [`ArchiveFormat`]; ZIP delegates to the existing V1 code so behavior is
//! unchanged, and 7z is added without touching the ZIP path.
//!
//! Status (Lot A): ZIP delegates to V1; **7z export is implemented** (LZMA2 via
//! the `sevenz-rust` crate). BA2 remains a TODO.

use std::collections::{BTreeMap, HashMap};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::export;
use crate::models::Ximod;

/// Output archive format for a distribution package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArchiveFormat {
    /// ZIP (V1 behavior, always available).
    #[default]
    Zip,
    /// 7-Zip (LZMA2). Smaller downloads; planned for V2.
    SevenZip,
}

impl ArchiveFormat {
    /// File extension (without the dot). Used by the UI/CLI format pickers.
    pub fn extension(self) -> &'static str {
        match self {
            ArchiveFormat::Zip => "zip",
            ArchiveFormat::SevenZip => "7z",
        }
    }
}

/// Compression effort, mapped per-format to concrete levels at build time.
/// Only `Normal` is wired to the UI today; the other levels await the 7z
/// level-selection control (Lot C/D).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionLevel {
    /// No compression (fastest, store only).
    Store,
    /// Fast, lower ratio.
    Fast,
    /// Balanced (default).
    #[default]
    Normal,
    /// Maximum ratio (slowest).
    Max,
}

/// Build a distribution archive of the FOMOD under `root_dir` into `out_path`,
/// in the requested `format`. Returns the number of files written.
///
/// ZIP delegates to [`export::build_distribution_archive`] (unchanged V1 path).
/// 7z is a TODO.
pub fn export_archive(
    ximod: &Ximod,
    root_dir: &Path,
    out_path: &Path,
    format: ArchiveFormat,
    level: CompressionLevel,
) -> Result<usize> {
    crate::xml::save_ximod(ximod, root_dir).with_context(|| "writing the FOMOD XML before packaging")?;
    package_directory(root_dir, out_path, format, level, &mut |_, _, _| true)
}

/// Package an already-saved mod folder into an archive, reporting progress.
///
/// Unlike [`export_archive`] this does not touch the FOMOD XML, so it can run
/// on a worker thread while the UI stays responsive: the GUI saves first, then
/// hands the folder over.
pub fn package_directory(
    root_dir: &Path,
    out_path: &Path,
    format: ArchiveFormat,
    level: CompressionLevel,
    progress: export::Progress<'_>,
) -> Result<usize> {
    match format {
        ArchiveFormat::Zip => export::zip_directory_with(root_dir, out_path, progress),
        ArchiveFormat::SevenZip => sevenzip_directory(root_dir, out_path, level, progress),
    }
}

/// Package the whole directory into a `.7z` archive (LZMA2). Mirrors
/// [`export::zip_directory_with`] but with 7-Zip output. Returns the number of
/// files written.
fn sevenzip_directory(
    root_dir: &Path,
    out_path: &Path,
    _level: CompressionLevel,
    progress: export::Progress<'_>,
) -> Result<usize> {
    export_with_overrides(
        root_dir,
        out_path,
        ArchiveFormat::SevenZip,
        &HashMap::new(),
        None,
        progress,
    )
}

/// Where the content of an archive entry comes from.
enum EntrySource<'a> {
    /// A file on disk (streamed).
    Disk(PathBuf),
    /// Bytes held in memory (an override).
    Bytes(&'a [u8]),
}

/// One file to write into an archive.
struct PlannedEntry<'a> {
    /// Archive-relative path, with `/` separators.
    name: String,
    source: EntrySource<'a>,
}

/// Archive-relative form of a path: `/` separators, no leading `./` or `/`.
fn archive_name(path: &str) -> String {
    let name = path.replace('\\', "/");
    let name = name.trim_start_matches("./").trim_start_matches('/');
    name.to_string()
}

/// List the entries of an archive of `root`, shared by the ZIP and 7z
/// writers: the distribution files of `root` (or, when `only` is given, just
/// the named ones), each replaced by its override when there is one, plus the
/// overrides that have no file on disk. Sorted by archive path.
fn plan_entries<'a>(
    root: &Path,
    out: &Path,
    overrides: &'a HashMap<String, Vec<u8>>,
    only: Option<&[String]>,
) -> Result<Vec<PlannedEntry<'a>>> {
    // BTreeMap: sorted and free of duplicates, whatever the callers pass.
    let mut planned: BTreeMap<String, EntrySource<'a>> = BTreeMap::new();
    match only {
        Some(paths) => {
            for path in paths {
                let name = archive_name(path);
                if name.is_empty() || name.split('/').any(|part| part == "..") {
                    anyhow::bail!("'{path}' is not a path inside the archive");
                }
                let abs = root.join(&name);
                planned.insert(name, EntrySource::Disk(abs));
            }
        }
        None => {
            for rel in export::collect_distribution_files(root, out)? {
                let name = archive_name(&rel.to_string_lossy());
                planned.insert(name, EntrySource::Disk(root.join(&rel)));
            }
        }
    }
    for (path, bytes) in overrides {
        let name = archive_name(path);
        if name.is_empty() {
            anyhow::bail!("an override has an empty archive path");
        }
        planned.insert(name, EntrySource::Bytes(bytes));
    }
    // A file named in `only` must exist, unless an override supplies it.
    for (name, source) in &planned {
        if let EntrySource::Disk(abs) = source
            && !abs.is_file()
        {
            anyhow::bail!("{name}: {} is not a file", abs.display());
        }
    }
    Ok(planned
        .into_iter()
        .map(|(name, source)| PlannedEntry { name, source })
        .collect())
}

/// Open the content of an entry for reading.
fn open_entry<'a>(entry: &PlannedEntry<'a>) -> Result<Box<dyn Read + 'a>> {
    Ok(match &entry.source {
        EntrySource::Disk(abs) => Box::new(std::io::BufReader::new(
            std::fs::File::open(abs).with_context(|| format!("reading {}", abs.display()))?,
        )),
        EntrySource::Bytes(bytes) => Box::new(Cursor::new(*bytes)),
    })
}

/// Write `entries` into a new archive at `out`, streaming each file. Returns
/// the number of files written. A failed or cancelled archive is removed.
fn write_archive(
    out: &Path,
    format: ArchiveFormat,
    entries: &[PlannedEntry<'_>],
    progress: export::Progress<'_>,
) -> Result<usize> {
    let total = entries.len();
    // Report progress before each entry; `false` cancels.
    let mut step = |i: usize, entry: &PlannedEntry<'_>| -> Result<()> {
        if progress(i, total, Path::new(&entry.name)) {
            Ok(())
        } else {
            Err(export::Cancelled.into())
        }
    };

    let result = (|| -> Result<()> {
        match format {
            ArchiveFormat::Zip => {
                let file = std::fs::File::create(out).with_context(|| format!("creating {}", out.display()))?;
                let mut zip = zip::ZipWriter::new(std::io::BufWriter::new(file));
                let opts: zip::write::FileOptions<()> = zip::write::FileOptions::default()
                    .compression_method(zip::CompressionMethod::Deflated)
                    .large_file(true);
                for (i, entry) in entries.iter().enumerate() {
                    step(i, entry)?;
                    zip.start_file(entry.name.as_str(), opts)?;
                    std::io::copy(&mut open_entry(entry)?, &mut zip)
                        .with_context(|| format!("adding {} to the archive", entry.name))?;
                }
                zip.finish()?.into_inner().map_err(|e| e.into_error())?;
            }
            ArchiveFormat::SevenZip => {
                let mut sz =
                    sevenz_rust::SevenZWriter::create(out).with_context(|| format!("creating {}", out.display()))?;
                // Default content method is LZMA2. TODO(v2): map a compression
                // level to an LZMA2 preset via set_content_methods.
                for (i, entry) in entries.iter().enumerate() {
                    step(i, entry)?;
                    let header = match &entry.source {
                        EntrySource::Disk(abs) => sevenz_rust::SevenZArchiveEntry::from_path(abs, entry.name.clone()),
                        EntrySource::Bytes(_) => {
                            let mut header = sevenz_rust::SevenZArchiveEntry::new();
                            header.name = entry.name.clone();
                            header.has_stream = true;
                            if let Ok(now) = std::time::SystemTime::now().try_into() {
                                header.last_modified_date = now;
                                header.has_last_modified_date = true;
                            }
                            header
                        }
                    };
                    sz.push_archive_entry(header, Some(open_entry(entry)?))
                        .with_context(|| format!("adding {} to the 7z archive", entry.name))?;
                }
                sz.finish().with_context(|| "finalizing the 7z archive")?;
            }
        }
        Ok(())
    })();

    match result {
        Ok(()) => {
            progress(total, total, Path::new(""));
            Ok(total)
        }
        Err(e) => {
            let _ = std::fs::remove_file(out);
            Err(e)
        }
    }
}

/// Package `root` into `out` like [`package_directory`], but with some files
/// replaced and, optionally, limited to a list of files.
///
/// * `overrides` maps an archive-relative path (`/` separators, matched
///   exactly, case included) to the bytes to store instead of the file on
///   disk. An override is always included, even when no such file exists on
///   disk and even when `only` does not name it.
/// * `only`, when given, replaces the enumeration of `root`: the archive holds
///   exactly those archive-relative paths (read from disk unless overridden)
///   plus the overrides. Naming a file that neither exists nor is overridden
///   is an error.
///
/// Returns the number of files written. `root` itself is never modified.
pub fn export_with_overrides(
    root: &Path,
    out: &Path,
    format: ArchiveFormat,
    overrides: &HashMap<String, Vec<u8>>,
    only: Option<&[String]>,
    progress: export::Progress<'_>,
) -> Result<usize> {
    let entries = plan_entries(root, out, overrides, only)?;
    write_archive(out, format, &entries, progress)
}

/// Target game for a BA2 archive (controls the BA2 header/version).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ba2Game {
    /// Skyrim Special Edition / Fallout 4 (general archive, version 1).
    SkyrimSE,
    /// Fallout 4 (same general v1 container as SSE).
    Fallout4,
    /// Starfield (general archive; written as v1 here — see note).
    Starfield,
}

impl Ba2Game {
    /// BTDX header version to write for a general (GNRL) archive.
    fn gnrl_version(self) -> u32 {
        // SSE/FO4 use version 1 for GNRL. Starfield's native GNRL is version 2/3;
        // we currently emit version 1, which most tools read. See the EXPERIMENTAL
        // note on `package_ba2`.
        1
    }
}

/// Options for BA2 packaging.
#[derive(Debug, Clone)]
pub struct Ba2Options {
    /// Which game's BA2 variant to produce.
    pub game: Ba2Game,
    /// Pack general files. Only general (GNRL) archives are supported; texture
    /// (DX10) archives are not produced yet (`false` is rejected).
    pub general: bool,
}

/// Standard CRC-32 (IEEE 802.3) of `bytes` — the hash BA2 uses for name/dir.
pub(crate) fn crc32(bytes: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in bytes {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// Bundle loose files under `src_dir` into a general (GNRL) BA2 archive at
/// `out_path`, uncompressed.
///
/// **EXPERIMENTAL.** Writes the BTDX / GNRL version-1 container (header, 36-byte
/// file records with CRC-32 name/dir hashes, raw file data, then the name table).
/// It has not been validated against a running game here; treat the output as a
/// best-effort build to verify with your mod manager / the game before shipping.
/// Texture (DX10) archives are out of scope.
pub fn package_ba2(src_dir: &Path, out_path: &Path, opts: &Ba2Options) -> Result<usize> {
    use std::io::Write;

    if !opts.general {
        anyhow::bail!("only general (GNRL) BA2 archives are supported");
    }
    if !src_dir.is_dir() {
        anyhow::bail!("not a folder: {}", src_dir.display());
    }

    // Collect files (relative paths), skipping junk and the output itself.
    let out_abs = out_path.canonicalize().unwrap_or_else(|_| out_path.to_path_buf());
    let mut entries: Vec<(std::path::PathBuf, Vec<u8>)> = Vec::new();
    for e in walkdir::WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
        if !e.file_type().is_file() {
            continue;
        }
        if e.path().canonicalize().map(|p| p == out_abs).unwrap_or(false) {
            continue;
        }
        let name = e.file_name().to_string_lossy();
        if crate::export::is_junk(&name) {
            continue;
        }
        let Ok(rel) = e.path().strip_prefix(src_dir) else {
            continue;
        };
        if rel.ancestors().any(crate::export::is_junk_dir) {
            continue;
        }
        let data = std::fs::read(e.path()).with_context(|| format!("reading {}", e.path().display()))?;
        entries.push((rel.to_path_buf(), data));
    }
    if entries.is_empty() {
        anyhow::bail!("no files to pack under {}", src_dir.display());
    }

    let file_count = entries.len() as u32;
    const HEADER: u64 = 24;
    const RECORD: u64 = 36;
    let data_start = HEADER + RECORD * file_count as u64;

    // Compute per-file metadata and data offsets.
    struct Rec {
        name_hash: u32,
        ext: [u8; 4],
        dir_hash: u32,
        offset: u64,
        size: u32,
        rel_lower_backslash: String,
    }
    let mut recs = Vec::with_capacity(entries.len());
    let mut cursor = data_start;
    for (rel, data) in &entries {
        let rel_s = rel.to_string_lossy().replace('/', "\\").to_lowercase();
        let (dir, file) = match rel_s.rsplit_once('\\') {
            Some((d, f)) => (d.to_string(), f.to_string()),
            None => (String::new(), rel_s.clone()),
        };
        let (stem, ext) = match file.rsplit_once('.') {
            Some((s, e)) => (s.to_string(), e.to_string()),
            None => (file.clone(), String::new()),
        };
        let mut ext4 = [0u8; 4];
        for (i, b) in ext.bytes().take(4).enumerate() {
            ext4[i] = b;
        }
        recs.push(Rec {
            name_hash: crc32(stem.as_bytes()),
            ext: ext4,
            dir_hash: crc32(dir.as_bytes()),
            offset: cursor,
            size: data.len() as u32,
            rel_lower_backslash: rel_s,
        });
        cursor += data.len() as u64;
    }
    let name_table_offset = cursor;

    let mut out = std::fs::File::create(out_path).with_context(|| format!("creating {}", out_path.display()))?;
    // Header.
    out.write_all(b"BTDX")?;
    out.write_all(&opts.game.gnrl_version().to_le_bytes())?;
    out.write_all(b"GNRL")?;
    out.write_all(&file_count.to_le_bytes())?;
    out.write_all(&name_table_offset.to_le_bytes())?;
    // File records.
    for r in &recs {
        out.write_all(&r.name_hash.to_le_bytes())?;
        out.write_all(&r.ext)?;
        out.write_all(&r.dir_hash.to_le_bytes())?;
        out.write_all(&0u32.to_le_bytes())?; // flags/unknown
        out.write_all(&r.offset.to_le_bytes())?;
        out.write_all(&0u32.to_le_bytes())?; // packedSize = 0 (uncompressed)
        out.write_all(&r.size.to_le_bytes())?; // unpackedSize
        out.write_all(&0xBAAD_F00Du32.to_le_bytes())?;
    }
    // File data (in the same order the offsets were computed).
    for (_, data) in &entries {
        out.write_all(data)?;
    }
    // Name table: u16 length + bytes, per file.
    for r in &recs {
        let bytes = r.rel_lower_backslash.as_bytes();
        out.write_all(&(bytes.len() as u16).to_le_bytes())?;
        out.write_all(bytes)?;
    }
    out.flush()?;
    Ok(entries.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extensions_are_correct() {
        assert_eq!(ArchiveFormat::Zip.extension(), "zip");
        assert_eq!(ArchiveFormat::SevenZip.extension(), "7z");
    }

    #[test]
    fn sevenzip_export_writes_a_readable_archive() {
        use std::fs;
        let dir = std::env::temp_dir().join(format!("ximod_7z_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("textures")).unwrap();
        fs::write(dir.join("MyMod.esp"), b"plugin").unwrap();
        fs::write(dir.join("textures/a.dds"), b"texture-bytes").unwrap();

        let mut x = Ximod::new("Test7z");
        x.required_files.push(crate::models::InstallFile::new_file("MyMod.esp"));

        let out = dir.join("Test7z.7z");
        let n = export_archive(&x, &dir, &out, ArchiveFormat::SevenZip, CompressionLevel::Normal)
            .expect("7z export should succeed");
        // At least the two mod files + the two FOMOD XML files written by save_ximod.
        assert!(n >= 3, "expected several files, got {n}");
        assert!(out.is_file());
        // The archive must decompress back to the same files.
        let back = dir.join("unpacked");
        sevenz_rust::decompress_file(&out, &back).expect("7z should be readable");
        assert!(back.join("MyMod.esp").is_file());
        assert!(back.join("fomod/ModuleConfig.xml").is_file());

        let _ = fs::remove_dir_all(&dir);
    }

    /// Names of the files of a ZIP archive, in archive order.
    fn zip_names(path: &Path) -> Vec<String> {
        let zip = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
        (0..zip.len())
            .map(|i| zip.name_for_index(i).unwrap().to_string())
            .collect()
    }

    fn zip_read(path: &Path, name: &str) -> Vec<u8> {
        let mut zip = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
        let mut out = Vec::new();
        zip.by_name(name).unwrap().read_to_end(&mut out).unwrap();
        out
    }

    /// Relative paths (with `/`) of the files under `dir`, sorted.
    fn files_under(dir: &Path) -> Vec<String> {
        let mut out: Vec<String> = walkdir::WalkDir::new(dir)
            .into_iter()
            .flatten()
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().strip_prefix(dir).unwrap().to_string_lossy().replace('\\', "/"))
            .collect();
        out.sort();
        out
    }

    fn overrides_root(tag: &str) -> PathBuf {
        use std::fs;
        let dir = std::env::temp_dir().join(format!("ximod_ovr_{tag}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("root/fomod")).unwrap();
        fs::create_dir_all(dir.join("root/textures")).unwrap();
        fs::write(dir.join("root/fomod/info.xml"), b"<original/>").unwrap();
        fs::write(dir.join("root/fomod/ModuleConfig.xml"), b"<config/>").unwrap();
        fs::write(dir.join("root/textures/a.dds"), b"texture-bytes").unwrap();
        fs::write(dir.join("root/Plugin.esp"), b"plugin").unwrap();
        fs::write(dir.join("root/Thumbs.db"), b"junk").unwrap();
        dir
    }

    #[test]
    fn export_with_overrides_zip() {
        let dir = overrides_root("zip");
        let root = dir.join("root");
        let mut overrides = HashMap::new();
        overrides.insert("fomod/info.xml".to_string(), b"<translated/>".to_vec());
        overrides.insert("README_FR.txt".to_string(), b"lisez-moi".to_vec());

        // Limited to two files from disk (one of them overridden) + the
        // override that only exists in memory.
        let only = vec!["fomod/info.xml".to_string(), "Plugin.esp".to_string()];
        let out = dir.join("only.zip");
        let mut seen = Vec::new();
        let n = export_with_overrides(
            &root,
            &out,
            ArchiveFormat::Zip,
            &overrides,
            Some(&only),
            &mut |d, t, p| {
                seen.push((d, t, p.to_string_lossy().to_string()));
                true
            },
        )
        .unwrap();
        assert_eq!(n, 3);
        assert_eq!(zip_names(&out), vec!["Plugin.esp", "README_FR.txt", "fomod/info.xml"]);
        assert_eq!(zip_read(&out, "fomod/info.xml"), b"<translated/>");
        assert_eq!(zip_read(&out, "Plugin.esp"), b"plugin");
        assert_eq!(zip_read(&out, "README_FR.txt"), b"lisez-moi");
        assert_eq!(seen.first(), Some(&(0, 3, "Plugin.esp".to_string())));
        assert_eq!(seen.last(), Some(&(3, 3, String::new())));

        // Whole folder: everything but junk, with the override applied.
        let out = dir.join("full.zip");
        let n = export_with_overrides(&root, &out, ArchiveFormat::Zip, &overrides, None, &mut |_, _, _| true).unwrap();
        assert_eq!(n, 5);
        assert_eq!(
            zip_names(&out),
            vec![
                "Plugin.esp",
                "README_FR.txt",
                "fomod/ModuleConfig.xml",
                "fomod/info.xml",
                "textures/a.dds"
            ]
        );
        assert_eq!(zip_read(&out, "fomod/info.xml"), b"<translated/>");
        assert_eq!(zip_read(&out, "fomod/ModuleConfig.xml"), b"<config/>");

        // The folder itself is untouched.
        assert_eq!(std::fs::read(root.join("fomod/info.xml")).unwrap(), b"<original/>");
        assert!(!root.join("README_FR.txt").exists());

        // A missing file that nothing overrides is an error; cancelling
        // removes the partial archive.
        let out = dir.join("bad.zip");
        let missing = vec!["nope.esp".to_string()];
        assert!(
            export_with_overrides(
                &root,
                &out,
                ArchiveFormat::Zip,
                &overrides,
                Some(&missing),
                &mut |_, _, _| true
            )
            .is_err()
        );
        let escape = vec!["../outside.txt".to_string()];
        assert!(
            export_with_overrides(
                &root,
                &out,
                ArchiveFormat::Zip,
                &overrides,
                Some(&escape),
                &mut |_, _, _| true
            )
            .is_err()
        );
        let err = export_with_overrides(&root, &out, ArchiveFormat::Zip, &overrides, None, &mut |d, _, _| d == 0)
            .unwrap_err();
        assert!(err.downcast_ref::<export::Cancelled>().is_some(), "{err}");
        assert!(!out.exists());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn export_with_overrides_sevenzip() {
        let dir = overrides_root("7z");
        let root = dir.join("root");
        let mut overrides = HashMap::new();
        overrides.insert("fomod/info.xml".to_string(), b"<translated/>".to_vec());
        overrides.insert("README_FR.txt".to_string(), b"lisez-moi".to_vec());

        let only = vec!["fomod/info.xml".to_string(), "textures/a.dds".to_string()];
        let out = dir.join("only.7z");
        let n = export_with_overrides(
            &root,
            &out,
            ArchiveFormat::SevenZip,
            &overrides,
            Some(&only),
            &mut |_, _, _| true,
        )
        .unwrap();
        assert_eq!(n, 3);
        let back = dir.join("unpacked");
        sevenz_rust::decompress_file(&out, &back).expect("7z should be readable");
        assert_eq!(
            files_under(&back),
            vec!["README_FR.txt", "fomod/info.xml", "textures/a.dds"]
        );
        assert_eq!(std::fs::read(back.join("fomod/info.xml")).unwrap(), b"<translated/>");
        assert_eq!(std::fs::read(back.join("textures/a.dds")).unwrap(), b"texture-bytes");
        assert_eq!(std::fs::read(back.join("README_FR.txt")).unwrap(), b"lisez-moi");

        let out = dir.join("full.7z");
        let n = export_with_overrides(
            &root,
            &out,
            ArchiveFormat::SevenZip,
            &overrides,
            None,
            &mut |_, _, _| true,
        )
        .unwrap();
        assert_eq!(n, 5);
        let back = dir.join("unpacked_full");
        sevenz_rust::decompress_file(&out, &back).expect("7z should be readable");
        assert_eq!(
            files_under(&back),
            vec![
                "Plugin.esp",
                "README_FR.txt",
                "fomod/ModuleConfig.xml",
                "fomod/info.xml",
                "textures/a.dds"
            ]
        );
        assert_eq!(std::fs::read(back.join("fomod/info.xml")).unwrap(), b"<translated/>");
        assert_eq!(std::fs::read(root.join("fomod/info.xml")).unwrap(), b"<original/>");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn crc32_matches_reference_vector() {
        // Standard CRC-32/IEEE check value for "123456789".
        assert_eq!(super::crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn ba2_writes_valid_header_and_name_table() {
        use std::fs;
        let dir = std::env::temp_dir().join(format!("ximod_ba2_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("textures")).unwrap();
        fs::write(dir.join("a.dds"), b"AAAA").unwrap();
        fs::write(dir.join("textures/b.dds"), b"BBBBBB").unwrap();

        let out = dir.join("out.ba2");
        let n = package_ba2(
            &dir,
            &out,
            &Ba2Options {
                game: Ba2Game::SkyrimSE,
                general: true,
            },
        )
        .unwrap();
        assert_eq!(n, 2);

        let bytes = fs::read(&out).unwrap();
        assert_eq!(&bytes[0..4], b"BTDX");
        assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 1);
        assert_eq!(&bytes[8..12], b"GNRL");
        let count = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        assert_eq!(count, 2);
        let nto = u64::from_le_bytes([
            bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21], bytes[22], bytes[23],
        ]) as usize;
        // Name table holds two length-prefixed names summing to its content.
        let mut o = nto;
        let mut names = Vec::new();
        for _ in 0..count {
            let len = u16::from_le_bytes([bytes[o], bytes[o + 1]]) as usize;
            o += 2;
            names.push(String::from_utf8_lossy(&bytes[o..o + len]).to_string());
            o += len;
        }
        assert!(names.iter().any(|n| n == "a.dds"));
        assert!(names.iter().any(|n| n == "textures\\b.dds"));
        assert_eq!(o, bytes.len(), "name table should end the file");

        let _ = fs::remove_dir_all(&dir);
    }

    /// The experimental writer's output must list back through the reader
    /// with the on-disk paths and sizes, and its records must carry the
    /// CRC-32 name / directory hashes the format specifies.
    #[test]
    fn package_ba2_round_trips_through_the_reader() {
        use crate::models::bethesda_archive::{ArchiveFormat as Fmt, list_archive};
        use std::fs;
        let dir = std::env::temp_dir().join(format!("ximod_ba2_rt_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("src/Textures/Armor")).unwrap();
        fs::create_dir_all(dir.join("src/meshes")).unwrap();
        fs::write(dir.join("src/Textures/Armor/Cuirass.dds"), vec![1u8; 1234]).unwrap();
        fs::write(dir.join("src/meshes/sword.nif"), b"nif-data").unwrap();
        fs::write(dir.join("src/README"), b"readme without extension").unwrap();
        fs::write(dir.join("src/Thumbs.db"), b"junk").unwrap();

        let out = dir.join("Mod - Main.ba2");
        let n = package_ba2(
            &dir.join("src"),
            &out,
            &Ba2Options {
                game: Ba2Game::Fallout4,
                general: true,
            },
        )
        .unwrap();
        assert_eq!(n, 3, "junk files are skipped");

        let listing = list_archive(&out).unwrap();
        assert_eq!(listing.format, Fmt::Ba2Gnrl { version: 1 });
        let mut got: Vec<(String, u64, bool)> = listing
            .entries
            .iter()
            .map(|e| (e.path.clone(), e.size, e.compressed))
            .collect();
        got.sort();
        assert_eq!(
            got,
            vec![
                ("meshes/sword.nif".to_string(), 8, false),
                ("readme".to_string(), 24, false),
                ("textures/armor/cuirass.dds".to_string(), 1234, false),
            ]
        );

        // Records, read raw: hashes are CRC-32 of the lowercase stem and of
        // the lowercase directory (backslashes), the extension is stored as
        // up to four lowercase bytes, and each offset points at the data.
        let bytes = fs::read(&out).unwrap();
        let u32_at = |o: usize| u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
        let u64_at = |o: usize| u64::from_le_bytes(bytes[o..o + 8].try_into().unwrap());
        let count = u32_at(12) as usize;
        assert_eq!(count, listing.entries.len());
        let name_table = u64_at(16) as usize;
        for (i, entry) in listing.entries.iter().enumerate() {
            let rec = 24 + 36 * i;
            let (dir_part, file_part) = entry.path.rsplit_once('/').unwrap_or(("", entry.path.as_str()));
            let (stem, ext) = file_part.rsplit_once('.').unwrap_or((file_part, ""));
            let mut ext4 = [0u8; 4];
            for (k, b) in ext.bytes().take(4).enumerate() {
                ext4[k] = b;
            }
            assert_eq!(u32_at(rec), crc32(stem.as_bytes()), "name hash of {}", entry.path);
            assert_eq!(&bytes[rec + 4..rec + 8], &ext4, "extension of {}", entry.path);
            assert_eq!(
                u32_at(rec + 8),
                crc32(dir_part.replace('/', "\\").as_bytes()),
                "dir hash of {}",
                entry.path
            );
            let offset = u64_at(rec + 16) as usize;
            let packed = u32_at(rec + 24);
            let unpacked = u32_at(rec + 28) as usize;
            assert_eq!(packed, 0, "stored, not compressed");
            assert_eq!(unpacked as u64, entry.size);
            assert!(
                offset >= 24 + 36 * count && offset + unpacked <= name_table,
                "{}",
                entry.path
            );
            let rel = entry.path.replace('/', std::path::MAIN_SEPARATOR_STR);
            let on_disk = fs::read(dir.join("src").join(&rel))
                .or_else(|_| {
                    // The archive lowercases names; find the file case-insensitively.
                    let want = entry.path.clone();
                    walkdir::WalkDir::new(dir.join("src"))
                        .into_iter()
                        .flatten()
                        .find(|e| {
                            e.path()
                                .strip_prefix(dir.join("src"))
                                .map(|p| p.to_string_lossy().replace('\\', "/").to_lowercase() == want)
                                .unwrap_or(false)
                        })
                        .map(|e| fs::read(e.path()).unwrap())
                        .ok_or_else(|| std::io::Error::other("not found"))
                })
                .unwrap();
            assert_eq!(
                &bytes[offset..offset + unpacked],
                &on_disk[..],
                "data of {}",
                entry.path
            );
        }

        let _ = fs::remove_dir_all(&dir);
    }
}
