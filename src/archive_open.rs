//! Open a FOMOD straight from a mod archive (`.zip` / `.7z`).
//!
//! The archive is extracted into a work folder under the configuration
//! directory (`work/<archive stem>/`, or `translate_work/<stem>/` for the
//! translation window, which only needs the `fomod/` folder), then the
//! folder that contains `fomod/` is located — at the top of the archive or
//! inside a single wrapping folder (`MyMod/fomod/…`), as both layouts are
//! common on Nexus. Entries that would escape the destination (zip-slip)
//! are rejected.
//!
//! Errors are a language-neutral enum the UI maps to its locale; the CLI
//! prints their `Display` form.

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::export::Progress;

/// Archive formats that can be opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveKind {
    Zip,
    SevenZ,
}

/// Why an archive could not be opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveError {
    /// Not a `.zip` / `.7z` (for instance `.rar`, which needs 7-Zip).
    UnsupportedArchive { ext: String },
    /// An entry would be written outside the destination folder.
    UnsafeEntry { entry: String },
    /// Nothing named `fomod/` was found after extraction.
    NoFomod { path: PathBuf },
    /// The progress callback asked to stop.
    Cancelled,
    /// No usable work folder (no configuration directory).
    NoWorkDir,
    /// Anything else (I/O, corrupt archive…), as text.
    Io(String),
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::UnsupportedArchive { ext } => {
                write!(
                    f,
                    "unsupported archive format \".{ext}\" (only .zip and .7z can be opened)"
                )
            }
            ArchiveError::UnsafeEntry { entry } => {
                write!(f, "entry \"{entry}\" would be written outside the destination folder")
            }
            ArchiveError::NoFomod { path } => write!(f, "no \"fomod\" folder found in {}", path.display()),
            ArchiveError::Cancelled => f.write_str("extraction cancelled"),
            ArchiveError::NoWorkDir => f.write_str("no configuration directory to extract into"),
            ArchiveError::Io(e) => f.write_str(e),
        }
    }
}

impl std::error::Error for ArchiveError {}

impl From<std::io::Error> for ArchiveError {
    fn from(e: std::io::Error) -> Self {
        ArchiveError::Io(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ArchiveError>;

/// Outcome of an extraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractReport {
    /// Files written.
    pub files: usize,
    /// The folder that contains `fomod/`.
    pub root: PathBuf,
}

/// What opening an archive into its work folder requires.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prepared {
    /// A previous extraction of the same archive holds a `fomod/`: reuse
    /// that root as is, nothing to extract.
    Reused { root: PathBuf },
    /// Extract into `dest` (with [`extract_archive`]), then locate the root.
    Extract { dest: PathBuf },
}

/// Which work folder an archive is extracted into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkKind {
    /// `<config dir>/work/<stem>/`: the whole mod, for editing.
    Full,
    /// `<config dir>/translate_work/<stem>/`: only `fomod/`, for the
    /// translation window.
    FomodOnly,
}

/// The archive format, from the file extension (case-insensitive). Anything
/// other than `.zip` / `.7z` is `UnsupportedArchive`.
pub fn detect_kind(path: &Path) -> Result<ArchiveKind> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    match ext.as_str() {
        "zip" => Ok(ArchiveKind::Zip),
        "7z" => Ok(ArchiveKind::SevenZ),
        _ => Err(ArchiveError::UnsupportedArchive { ext }),
    }
}

/// The folder that contains `fomod/`: `dir` itself, or a folder at most
/// `max_depth` levels below it (the shallowest match wins, ties broken by
/// name). `None` when there is none.
pub fn find_fomod_root(dir: &Path, max_depth: usize) -> Option<PathBuf> {
    if has_fomod_dir(dir) {
        return Some(dir.to_path_buf());
    }
    if max_depth == 0 {
        return None;
    }
    let mut subdirs: Vec<PathBuf> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    subdirs.sort();
    // Breadth first: every direct child before any grand-child.
    for sub in &subdirs {
        if has_fomod_dir(sub) {
            return Some(sub.clone());
        }
    }
    subdirs.iter().find_map(|sub| find_fomod_root(sub, max_depth - 1))
}

/// Whether `dir` directly contains a `fomod` folder (ASCII case ignored, as
/// on the file systems mods are built on).
fn has_fomod_dir(dir: &Path) -> bool {
    std::fs::read_dir(dir)
        .map(|rd| {
            rd.flatten()
                .any(|e| e.file_name().to_str().is_some_and(|n| n.eq_ignore_ascii_case("fomod")) && e.path().is_dir())
        })
        .unwrap_or(false)
}

/// The work folder of `archive` for `kind` (not created).
pub fn work_dir(archive: &Path, kind: WorkKind) -> Result<PathBuf> {
    let base = crate::config::AppConfig::config_dir().ok_or(ArchiveError::NoWorkDir)?;
    let stem = archive
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("archive");
    // Keep the folder name safe whatever the archive was called.
    let safe: String = stem
        .chars()
        .map(|c| {
            if r#"\/:*?"<>|"#.contains(c) || c.is_control() {
                '_'
            } else {
                c
            }
        })
        .collect();
    let sub = match kind {
        WorkKind::Full => "work",
        WorkKind::FomodOnly => "translate_work",
    };
    Ok(base.join(sub).join(safe))
}

/// Decide how to open `archive` into its work folder (format check, work
/// folder, reuse of a previous extraction).
///
/// For `WorkKind::Full`, a work folder that already holds a `fomod/`
/// (a previous extraction of the same archive) is reused as is, without
/// extracting again. For `WorkKind::FomodOnly` the `fomod/` entries (plus
/// any `*.ximod-translation` sidecar) are always extracted again, over
/// whatever is already there, so a translation in progress is never lost
/// and the installer is always the archive's.
pub fn prepare(archive: &Path, kind: WorkKind) -> Result<Prepared> {
    detect_kind(archive)?;
    let dest = work_dir(archive, kind)?;
    if kind == WorkKind::Full
        && let Some(root) = find_fomod_root(&dest, 2)
    {
        return Ok(Prepared::Reused { root });
    }
    Ok(Prepared::Extract { dest })
}

/// The FOMOD root of a finished extraction into the work folder of
/// `archive` (see [`prepare`] and [`extract_archive`]).
pub fn extracted_root(archive: &Path, kind: WorkKind) -> Option<PathBuf> {
    work_dir(archive, kind).ok().and_then(|d| find_fomod_root(&d, 2))
}

/// Extract `archive` into `dest` (created if needed). With `only_fomod`,
/// only the entries under a `fomod/` folder (any depth, ASCII case ignored)
/// and the `*.ximod-translation` sidecars are written. `progress` is called
/// per entry with `(done, total, entry)` and may cancel. Returns the number
/// of files written and the folder containing `fomod/`.
pub fn extract_archive(archive: &Path, dest: &Path, only_fomod: bool, progress: Progress<'_>) -> Result<ExtractReport> {
    let kind = detect_kind(archive)?;
    std::fs::create_dir_all(dest)?;
    let files = match kind {
        ArchiveKind::Zip => extract_zip(archive, dest, only_fomod, progress)?,
        ArchiveKind::SevenZ => extract_7z(archive, dest, only_fomod, progress)?,
    };
    let root = find_fomod_root(dest, 2).ok_or_else(|| ArchiveError::NoFomod {
        path: dest.to_path_buf(),
    })?;
    Ok(ExtractReport { files, root })
}

/// The destination-relative path of an entry, or `None` when the entry
/// must be rejected: absolute, or escaping through `..`. Both separators
/// are accepted (7z archives built on Windows store backslashes).
fn safe_relative(name: &str) -> Option<PathBuf> {
    let name = name.replace('\\', "/");
    if name.is_empty() || name.starts_with('/') || name.contains(':') {
        return None;
    }
    let mut out = PathBuf::new();
    for part in name.split('/') {
        match part {
            "" | "." => continue,
            ".." => return None,
            p => out.push(p),
        }
    }
    (!out.as_os_str().is_empty()).then_some(out)
}

/// Whether an entry is kept under the `only_fomod` filter.
fn wanted(rel: &Path, only_fomod: bool) -> bool {
    if !only_fomod {
        return true;
    }
    let in_fomod = rel
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .any(|c| c.eq_ignore_ascii_case("fomod"));
    in_fomod
        || rel
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.to_ascii_lowercase().ends_with(".ximod-translation"))
}

fn extract_zip(archive: &Path, dest: &Path, only_fomod: bool, progress: Progress<'_>) -> Result<usize> {
    let file = std::fs::File::open(archive)?;
    let mut zip = zip::ZipArchive::new(std::io::BufReader::new(file)).map_err(|e| ArchiveError::Io(e.to_string()))?;
    let total = zip.len();
    let mut written = 0usize;
    for i in 0..total {
        let mut entry = zip.by_index(i).map_err(|e| ArchiveError::Io(e.to_string()))?;
        let name = entry.name().to_string();
        let Some(rel) = safe_relative(&name) else {
            if entry.is_dir() && name.trim_matches('/').is_empty() {
                continue;
            }
            return Err(ArchiveError::UnsafeEntry { entry: name });
        };
        if !progress(i, total, &rel) {
            return Err(ArchiveError::Cancelled);
        }
        if entry.is_dir() {
            if wanted(&rel, only_fomod) {
                std::fs::create_dir_all(dest.join(&rel))?;
            }
            continue;
        }
        if !wanted(&rel, only_fomod) {
            continue;
        }
        write_entry(dest, &rel, &mut entry)?;
        written += 1;
    }
    progress(total, total, Path::new(""));
    Ok(written)
}

fn extract_7z(archive: &Path, dest: &Path, only_fomod: bool, progress: Progress<'_>) -> Result<usize> {
    let mut reader = sevenz_rust::SevenZReader::open(archive, sevenz_rust::Password::empty())
        .map_err(|e| ArchiveError::Io(e.to_string()))?;
    let total = reader.archive().files.len();
    let mut written = 0usize;
    let mut done = 0usize;
    let mut failure: Option<ArchiveError> = None;
    let result = reader.for_each_entries(|entry, r| {
        let name = entry.name().to_string();
        let Some(rel) = safe_relative(&name) else {
            failure = Some(ArchiveError::UnsafeEntry { entry: name });
            return Ok(false);
        };
        if !progress(done, total, &rel) {
            failure = Some(ArchiveError::Cancelled);
            return Ok(false);
        }
        done += 1;
        if entry.is_directory() {
            if wanted(&rel, only_fomod) {
                std::fs::create_dir_all(dest.join(&rel)).map_err(sevenz_rust::Error::io)?;
            }
            return Ok(true);
        }
        if !wanted(&rel, only_fomod) {
            // Solid blocks must still be decoded in order: drain the entry.
            std::io::copy(r, &mut std::io::sink()).map_err(sevenz_rust::Error::io)?;
            return Ok(true);
        }
        match write_entry(dest, &rel, r) {
            Ok(()) => {
                written += 1;
                Ok(true)
            }
            Err(e) => {
                failure = Some(e);
                Ok(false)
            }
        }
    });
    if let Some(e) = failure {
        return Err(e);
    }
    result.map_err(|e| ArchiveError::Io(e.to_string()))?;
    progress(total, total, Path::new(""));
    Ok(written)
}

/// Stream one entry to `dest/rel`, creating the parent folders.
fn write_entry(dest: &Path, rel: &Path, reader: &mut dyn Read) -> Result<()> {
    let path = dest.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut out = std::io::BufWriter::new(std::fs::File::create(&path)?);
    std::io::copy(reader, &mut out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod_aopen_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Write a zip with the given `(name, content)` entries.
    fn make_zip(path: &Path, entries: &[(&str, &[u8])]) {
        let file = std::fs::File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let opts: zip::write::FileOptions<()> =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        for (name, content) in entries {
            zip.start_file(*name, opts).unwrap();
            zip.write_all(content).unwrap();
        }
        zip.finish().unwrap();
    }

    /// Write a 7z with the given `(name, content)` entries.
    fn make_7z(path: &Path, entries: &[(&str, &[u8])]) {
        let mut sz = sevenz_rust::SevenZWriter::create(path).unwrap();
        for (name, content) in entries {
            let mut header = sevenz_rust::SevenZArchiveEntry::new();
            header.name = name.to_string();
            header.has_stream = true;
            sz.push_archive_entry(header, Some(std::io::Cursor::new(content.to_vec())))
                .unwrap();
        }
        sz.finish().unwrap();
    }

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

    const WRAPPED: &[(&str, &[u8])] = &[
        ("MyMod/fomod/info.xml", b"<fomod/>"),
        ("MyMod/fomod/ModuleConfig.xml", b"<config/>"),
        ("MyMod/textures/a.dds", b"tex"),
        ("MyMod/readme.txt", b"hi"),
    ];
    const FLAT: &[(&str, &[u8])] = &[
        ("FOMOD/ModuleConfig.xml", b"<config/>"),
        ("Plugin.esp", b"esp"),
        ("translations/Mod.fra.ximod-translation", b"{}"),
    ];

    #[test]
    fn detect_kind_is_case_insensitive_and_rejects_rar() {
        assert_eq!(detect_kind(Path::new("a.ZIP")), Ok(ArchiveKind::Zip));
        assert_eq!(detect_kind(Path::new("a.7Z")), Ok(ArchiveKind::SevenZ));
        assert_eq!(
            detect_kind(Path::new("a.rar")),
            Err(ArchiveError::UnsupportedArchive { ext: "rar".into() })
        );
        assert!(matches!(
            detect_kind(Path::new("noext")),
            Err(ArchiveError::UnsupportedArchive { ext }) if ext.is_empty()
        ));
        assert_eq!(
            prepare(Path::new("x.rar"), WorkKind::Full),
            Err(ArchiveError::UnsupportedArchive { ext: "rar".into() })
        );
        let err = extract_archive(Path::new("x.rar"), Path::new("/nonexistent"), false, &mut |_, _, _| {
            true
        })
        .unwrap_err();
        assert!(err.to_string().contains(".rar"), "{err}");
    }

    #[test]
    fn zip_wrapped_in_a_folder_extracts_and_finds_the_root() {
        let dir = scratch("zipwrap");
        let archive = dir.join("MyMod-1.0.zip");
        make_zip(&archive, WRAPPED);
        let dest = dir.join("out");
        let mut seen = Vec::new();
        let rep = extract_archive(&archive, &dest, false, &mut |d, t, _| {
            seen.push((d, t));
            true
        })
        .unwrap();
        assert_eq!(rep.files, 4);
        assert_eq!(rep.root, dest.join("MyMod"));
        assert_eq!(seen.first(), Some(&(0, 4)));
        assert_eq!(seen.last(), Some(&(4, 4)));
        assert_eq!(
            files_under(&dest),
            vec![
                "MyMod/fomod/ModuleConfig.xml",
                "MyMod/fomod/info.xml",
                "MyMod/readme.txt",
                "MyMod/textures/a.dds"
            ]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn zip_flat_with_only_fomod_filter() {
        let dir = scratch("zipflat");
        let archive = dir.join("flat.zip");
        make_zip(&archive, FLAT);
        let dest = dir.join("out");
        let rep = extract_archive(&archive, &dest, true, &mut |_, _, _| true).unwrap();
        assert_eq!(rep.files, 2, "fomod/ (any case) + the sidecar, not the esp");
        assert_eq!(rep.root, dest);
        assert_eq!(
            files_under(&dest),
            vec!["FOMOD/ModuleConfig.xml", "translations/Mod.fra.ximod-translation"]
        );
        // Full extraction of the same archive into a fresh folder.
        let dest2 = dir.join("out2");
        let rep = extract_archive(&archive, &dest2, false, &mut |_, _, _| true).unwrap();
        assert_eq!(rep.files, 3);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn sevenz_wrapped_and_filtered() {
        let dir = scratch("7z");
        let archive = dir.join("MyMod.7z");
        make_7z(&archive, WRAPPED);
        let dest = dir.join("full");
        let rep = extract_archive(&archive, &dest, false, &mut |_, _, _| true).unwrap();
        assert_eq!(rep.files, 4);
        assert_eq!(rep.root, dest.join("MyMod"));
        assert_eq!(std::fs::read(dest.join("MyMod/textures/a.dds")).unwrap(), b"tex");

        let dest = dir.join("fomod_only");
        let rep = extract_archive(&archive, &dest, true, &mut |_, _, _| true).unwrap();
        assert_eq!(rep.files, 2);
        assert_eq!(rep.root, dest.join("MyMod"));
        assert_eq!(
            files_under(&dest),
            vec!["MyMod/fomod/ModuleConfig.xml", "MyMod/fomod/info.xml"]
        );

        // Cancellation stops the extraction.
        let dest = dir.join("cancelled");
        let err = extract_archive(&archive, &dest, false, &mut |d, _, _| d == 0).unwrap_err();
        assert_eq!(err, ArchiveError::Cancelled);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn zip_slip_is_rejected_and_no_fomod_is_an_error() {
        let dir = scratch("slip");
        let archive = dir.join("evil.zip");
        make_zip(
            &archive,
            &[("../../outside.txt", b"x"), ("fomod/info.xml", b"<fomod/>")],
        );
        let dest = dir.join("out");
        let err = extract_archive(&archive, &dest, false, &mut |_, _, _| true).unwrap_err();
        assert!(matches!(err, ArchiveError::UnsafeEntry { .. }), "{err}");
        assert!(!dir.join("outside.txt").exists());

        let archive = dir.join("plain.zip");
        make_zip(&archive, &[("readme.txt", b"no installer here")]);
        let dest = dir.join("plain");
        let err = extract_archive(&archive, &dest, false, &mut |_, _, _| true).unwrap_err();
        assert!(matches!(err, ArchiveError::NoFomod { .. }), "{err}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn safe_relative_normalises_and_rejects_escapes() {
        assert_eq!(safe_relative("a/b.txt"), Some(PathBuf::from("a/b.txt")));
        assert_eq!(safe_relative("./a\\b.txt"), Some(PathBuf::from("a/b.txt")));
        assert_eq!(safe_relative("a/../b"), None);
        assert_eq!(safe_relative("/etc/passwd"), None);
        assert_eq!(safe_relative("C:\\x"), None);
        assert_eq!(safe_relative(""), None);
    }

    #[test]
    fn find_fomod_root_prefers_the_shallowest_match() {
        let dir = scratch("root");
        std::fs::create_dir_all(dir.join("deep/er/fomod")).unwrap();
        assert_eq!(find_fomod_root(&dir, 2).as_deref(), Some(dir.join("deep/er").as_path()));
        assert_eq!(find_fomod_root(&dir, 1), None);
        std::fs::create_dir_all(dir.join("Fomod")).unwrap();
        assert_eq!(find_fomod_root(&dir, 2).as_deref(), Some(dir.as_path()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn work_dir_sanitises_the_stem() {
        let full = work_dir(Path::new("/dl/My:Mod*1.zip"), WorkKind::Full).unwrap();
        assert!(full.ends_with("work/My_Mod_1"), "{}", full.display());
        let tr = work_dir(Path::new("x.7z"), WorkKind::FomodOnly).unwrap();
        assert!(tr.ends_with("translate_work/x"), "{}", tr.display());
    }
}
