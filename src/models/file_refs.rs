//! Keeping file references in step with the disk (2.2.0).
//!
//! Every file, folder and image a project uses is a path relative to the mod
//! folder. When such a file is renamed or moved, the project points at
//! something that no longer exists. This module offers the two halves of the
//! cure, both free of any UI or i18n:
//!
//! * [`rename_on_disk`] renames a source from XIMOD itself and updates every
//!   reference to it (and, for a folder, every reference below it);
//! * [`find_missing`] lists the references that point at nothing and proposes
//!   candidates found in the mod folder (same name elsewhere → moved; similar
//!   name next to where it was → renamed), and [`retarget`] applies the
//!   choices.
//!
//! Comparisons ignore ASCII case and the separator, as on the file systems
//! mods are built on; replacements keep the separator style of each stored
//! reference, so a project written with `\` keeps its backslashes.

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use super::{FileType, Ximod};

/// What a reference designates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RefKind {
    File,
    Folder,
    Image,
}

impl RefKind {
    pub fn is_folder(self) -> bool {
        self == RefKind::Folder
    }
}

/// Normalized form of a reference: trimmed, `/` separators, no leading `./`,
/// no trailing `/`, ASCII lower case (length-preserving, so a normalized
/// prefix length is also a prefix length of the stored string... after the
/// same trimming).
pub fn norm(p: &str) -> String {
    let p = p.trim().replace('\\', "/");
    let p = p.trim_start_matches("./");
    p.trim_end_matches('/').to_ascii_lowercase()
}

/// The stored form of a `/`-separated relative path, written with the
/// separator `like` already uses (backslash when it has one).
fn styled(path_slash: &str, like: &str) -> String {
    if like.contains('\\') {
        path_slash.replace('/', "\\")
    } else {
        path_slash.to_string()
    }
}

/// Visit every file / folder / image reference of the project.
pub fn for_each_ref(ximod: &Ximod, mut f: impl FnMut(RefKind, &str)) {
    let kind = |t: FileType| {
        if t == FileType::Folder {
            RefKind::Folder
        } else {
            RefKind::File
        }
    };
    if let Some(img) = &ximod.header_image {
        f(RefKind::Image, img);
    }
    for file in &ximod.required_files {
        f(kind(file.file_type), &file.source);
    }
    for set in &ximod.conditional_files {
        for file in &set.files {
            f(kind(file.file_type), &file.source);
        }
    }
    for step in &ximod.steps {
        for group in &step.plugin_groups {
            for plugin in &group.plugins {
                if let Some(img) = &plugin.image_path {
                    f(RefKind::Image, img);
                }
                for file in &plugin.files {
                    f(kind(file.file_type), &file.source);
                }
            }
        }
    }
}

/// Mutable [`for_each_ref`].
pub fn for_each_ref_mut(ximod: &mut Ximod, mut f: impl FnMut(RefKind, &mut String)) {
    let kind = |t: FileType| {
        if t == FileType::Folder {
            RefKind::Folder
        } else {
            RefKind::File
        }
    };
    if let Some(img) = &mut ximod.header_image {
        f(RefKind::Image, img);
    }
    for file in &mut ximod.required_files {
        f(kind(file.file_type), &mut file.source);
    }
    for set in &mut ximod.conditional_files {
        for file in &mut set.files {
            f(kind(file.file_type), &mut file.source);
        }
    }
    for step in &mut ximod.steps {
        for group in &mut step.plugin_groups {
            for plugin in &mut group.plugins {
                if let Some(img) = &mut plugin.image_path {
                    f(RefKind::Image, img);
                }
                for file in &mut plugin.files {
                    f(kind(file.file_type), &mut file.source);
                }
            }
        }
    }
}

/// How many references designate `path` itself or, when `below` is true,
/// something inside it.
pub fn count_uses(ximod: &Ximod, path: &str, below: bool) -> usize {
    let target = norm(path);
    let prefix = format!("{target}/");
    let mut n = 0;
    for_each_ref(ximod, |_, r| {
        let r = norm(r);
        if r == target || (below && r.starts_with(&prefix)) {
            n += 1;
        }
    });
    n
}

/// Point every reference to `old` at `new` (both relative to the root, any
/// separator). With `below`, references inside `old` follow too (a renamed
/// or moved folder). Returns the number of references changed.
pub fn retarget(ximod: &mut Ximod, old: &str, new: &str, below: bool) -> usize {
    let old_n = norm(old);
    let prefix = format!("{old_n}/");
    let new_slash = new.trim().replace('\\', "/");
    let new_slash = new_slash.trim_start_matches("./").trim_end_matches('/').to_string();
    let mut changed = 0;
    for_each_ref_mut(ximod, |_, r| {
        let rn = norm(r);
        let replacement = if rn == old_n {
            Some(new_slash.clone())
        } else if below && rn.starts_with(&prefix) {
            // `rn` and the trimmed stored string have the same length, so
            // the tail can be cut from the stored text (keeps its case).
            let stored = r.trim().replace('\\', "/");
            let stored = stored.trim_start_matches("./").trim_end_matches('/');
            Some(format!("{new_slash}{}", &stored[old_n.len()..]))
        } else {
            None
        };
        if let Some(rep) = replacement {
            let rep = styled(&rep, r);
            if *r != rep {
                *r = rep;
                changed += 1;
            }
        }
    });
    changed
}

// ---------------------------------------------------------------------------
// Rename on disk
// ---------------------------------------------------------------------------

/// Why a rename was refused or failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenameError {
    /// The new name is empty.
    EmptyName,
    /// The new name contains a separator, `..`, or a character Windows forbids.
    InvalidName,
    /// The reference is absolute or escapes the mod folder.
    OutsideRoot,
    /// Nothing exists at the current path.
    Missing,
    /// Something else already has the new name.
    AlreadyExists,
    /// The file system refused (message from the OS).
    Io(String),
}

/// A successful rename.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameOutcome {
    /// New relative path (`/` separators).
    pub new_rel: String,
    /// References updated in the project.
    pub updated: usize,
}

/// Characters a file name may not contain on Windows (where mods are
/// installed), plus both separators.
fn valid_file_name(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && !name.ends_with(' ')
        && !name.ends_with('.')
        && !name
            .chars()
            .any(|c| matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') || c.is_control())
}

/// Split a reference into its trimmed `/` form, refusing absolute and
/// escaping paths.
fn relative_slash(raw: &str) -> Option<String> {
    let s = raw.trim().replace('\\', "/");
    let s = s.trim_start_matches("./").trim_end_matches('/').to_string();
    let b = s.as_bytes();
    let absolute = s.starts_with('/') || (b.len() >= 2 && b[1] == b':' && b[0].is_ascii_alphabetic());
    if s.is_empty() || absolute || s.split('/').any(|c| c == "..") {
        None
    } else {
        Some(s)
    }
}

/// Rename the file or folder `old_ref` (relative to `root`) to `new_name`
/// (a bare name, in the same folder), then update every reference of the
/// project — for a folder, the references inside it as well.
///
/// The disk is changed first; the model only when that succeeded. A change
/// of case only (`a.esp` → `A.esp`) is allowed on every file system.
pub fn rename_on_disk(
    ximod: &mut Ximod,
    root: &Path,
    old_ref: &str,
    new_name: &str,
) -> Result<RenameOutcome, RenameError> {
    let new_name = new_name.trim();
    if new_name.is_empty() {
        return Err(RenameError::EmptyName);
    }
    if !valid_file_name(new_name) {
        return Err(RenameError::InvalidName);
    }
    let old_rel = relative_slash(old_ref).ok_or(RenameError::OutsideRoot)?;
    let old_abs = root.join(&old_rel);
    if !old_abs.exists() {
        return Err(RenameError::Missing);
    }
    let folder = old_abs.is_dir();
    let new_rel = match old_rel.rfind('/') {
        Some(i) => format!("{}/{new_name}", &old_rel[..i]),
        None => new_name.to_string(),
    };
    let new_abs = root.join(&new_rel);
    let case_only = new_rel.eq_ignore_ascii_case(&old_rel);
    if new_rel == old_rel {
        return Ok(RenameOutcome { new_rel, updated: 0 });
    }
    if !case_only && new_abs.exists() {
        return Err(RenameError::AlreadyExists);
    }
    if case_only {
        // Some file systems treat a case-only rename as a no-op: go through
        // a temporary name.
        let tmp = old_abs.with_file_name(format!(".ximod-rename-{}", std::process::id()));
        std::fs::rename(&old_abs, &tmp).map_err(|e| RenameError::Io(e.to_string()))?;
        if let Err(e) = std::fs::rename(&tmp, &new_abs) {
            let _ = std::fs::rename(&tmp, &old_abs);
            return Err(RenameError::Io(e.to_string()));
        }
    } else {
        std::fs::rename(&old_abs, &new_abs).map_err(|e| RenameError::Io(e.to_string()))?;
    }
    let updated = retarget(ximod, &old_rel, &new_rel, folder);
    Ok(RenameOutcome { new_rel, updated })
}

// ---------------------------------------------------------------------------
// Missing files and their candidates
// ---------------------------------------------------------------------------

/// Why a candidate was proposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    /// Same name, somewhere else in the mod folder.
    Moved,
    /// Similar name, in the folder where the reference used to be.
    Renamed,
}

/// A possible new location for a missing reference.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    /// Relative path, `/` separators, as found on disk.
    pub rel: String,
    pub reason: Reason,
    /// Higher is better (for sorting only).
    pub score: f32,
}

/// A reference that points at nothing.
#[derive(Debug, Clone, PartialEq)]
pub struct MissingRef {
    /// The reference as first stored in the project.
    pub path: String,
    pub kind: RefKind,
    /// How many references share it.
    pub uses: usize,
    /// Best first, at most [`MAX_CANDIDATES`].
    pub candidates: Vec<Candidate>,
}

impl MissingRef {
    /// The candidate worth pre-selecting: a single "moved" match.
    pub fn obvious(&self) -> Option<&Candidate> {
        let mut moved = self.candidates.iter().filter(|c| c.reason == Reason::Moved);
        match (moved.next(), moved.next()) {
            (Some(c), None) => Some(c),
            _ => None,
        }
    }
}

/// Candidates listed per missing reference.
pub const MAX_CANDIDATES: usize = 5;
/// Minimum name similarity for a "renamed" candidate.
const MIN_SIMILARITY: f32 = 0.4;

/// One entry of the mod folder.
struct Entry {
    rel: String,
    rel_lower: String,
    name_lower: String,
    parent_lower: String,
}

/// Files and folders of the mod folder, the installer's own folder and the
/// usual clutter excluded.
struct RootIndex {
    files: Vec<Entry>,
    dirs: Vec<Entry>,
}

impl RootIndex {
    fn scan(root: &Path) -> Self {
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        let walker = WalkDir::new(root).min_depth(1).into_iter().filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !crate::export::is_junk(&name)
        });
        for e in walker.filter_map(Result::ok) {
            let Ok(rel) = e.path().strip_prefix(root) else { continue };
            let rel = rel.to_string_lossy().replace('\\', "/");
            let rel_lower = rel.to_ascii_lowercase();
            if rel_lower == "fomod/backups" || rel_lower.starts_with("fomod/backups/") {
                continue;
            }
            let (parent_lower, name_lower) = match rel_lower.rfind('/') {
                Some(i) => (rel_lower[..i].to_string(), rel_lower[i + 1..].to_string()),
                None => (String::new(), rel_lower.clone()),
            };
            let entry = Entry {
                rel,
                rel_lower,
                name_lower,
                parent_lower,
            };
            if e.file_type().is_dir() {
                dirs.push(entry);
            } else if e.file_type().is_file() {
                files.push(entry);
            }
        }
        Self { files, dirs }
    }
}

/// Levenshtein distance on characters (names are short).
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cost = usize::from(ca != cb);
            cur[j + 1] = (prev[j] + cost).min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

/// Similarity of two names in 0..=1 (1 = identical).
pub fn similarity(a: &str, b: &str) -> f32 {
    let len = a.chars().count().max(b.chars().count());
    if len == 0 {
        return 1.0;
    }
    1.0 - levenshtein(a, b) as f32 / len as f32
}

/// The extension of a lower-case name (`""` when none).
fn ext_of(name: &str) -> &str {
    name.rfind('.').map(|i| &name[i..]).unwrap_or("")
}

/// Number of leading path components two `/` paths share.
fn common_dirs(a: &str, b: &str) -> usize {
    a.split('/').zip(b.split('/')).take_while(|(x, y)| x == y).count()
}

/// Candidates for one missing reference.
fn candidates_for(index: &RootIndex, path_n: &str, kind: RefKind, referenced: &HashSet<String>) -> Vec<Candidate> {
    let (parent, name) = match path_n.rfind('/') {
        Some(i) => (&path_n[..i], &path_n[i + 1..]),
        None => ("", path_n),
    };
    let pool = if kind.is_folder() { &index.dirs } else { &index.files };
    let mut out: Vec<Candidate> = Vec::new();

    // Moved: the same name elsewhere.
    for e in pool.iter().filter(|e| e.name_lower == name && e.rel_lower != path_n) {
        out.push(Candidate {
            rel: e.rel.clone(),
            reason: Reason::Moved,
            score: 100.0 + common_dirs(&e.rel_lower, path_n) as f32,
        });
    }

    // Renamed: a similar, unreferenced name in the folder it was in (when
    // that folder still exists), same extension for files.
    // `parent` is lower-cased: look it up in the index, not on a
    // case-sensitive disk.
    let parent_exists = parent.is_empty() || index.dirs.iter().any(|d| d.rel_lower == parent);
    if parent_exists {
        for e in pool.iter().filter(|e| e.parent_lower == parent && e.name_lower != name) {
            if referenced.contains(&e.rel_lower) {
                continue;
            }
            if !kind.is_folder() && ext_of(&e.name_lower) != ext_of(name) {
                continue;
            }
            let sim = similarity(&e.name_lower, name);
            if sim >= MIN_SIMILARITY {
                out.push(Candidate {
                    rel: e.rel.clone(),
                    reason: Reason::Renamed,
                    score: 50.0 * sim,
                });
            }
        }
    }

    out.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.rel.cmp(&b.rel)));
    out.truncate(MAX_CANDIDATES);
    out
}

/// Every reference of the project that points at nothing under `root`,
/// grouped (same path = one entry), each with its candidates. Absolute and
/// escaping references are left to the validation.
pub fn find_missing(ximod: &Ximod, root: &Path) -> Vec<MissingRef> {
    // Gather distinct missing references (first spelling kept) and the set
    // of existing ones (a candidate already in use is not "renamed").
    let mut missing: BTreeMap<(String, RefKind), (String, usize)> = BTreeMap::new();
    let mut referenced: HashSet<String> = HashSet::new();
    for_each_ref(ximod, |kind, raw| {
        let Some(rel) = relative_slash(raw) else { return };
        let abs = root.join(&rel);
        let exists = if kind.is_folder() { abs.is_dir() } else { abs.is_file() };
        let n = norm(&rel);
        if exists {
            referenced.insert(n);
        } else {
            let entry = missing.entry((n, kind)).or_insert_with(|| (raw.trim().to_string(), 0));
            entry.1 += 1;
        }
    });
    if missing.is_empty() {
        return Vec::new();
    }
    let index = RootIndex::scan(root);
    missing
        .into_iter()
        .map(|((n, kind), (path, uses))| MissingRef {
            candidates: candidates_for(&index, &n, kind, &referenced),
            path,
            kind,
            uses,
        })
        .collect()
}

/// Apply chosen new locations (`(missing path, new relative path, kind)`)
/// to the project. Returns the number of references changed.
pub fn apply_relinks(ximod: &mut Ximod, choices: &[(String, String, RefKind)]) -> usize {
    choices
        .iter()
        .map(|(old, new, kind)| retarget(ximod, old, new, kind.is_folder()))
        .sum()
}

/// A path chosen in a file dialog, made relative to `root` with `/`.
pub fn relative_to(root: &Path, picked: &Path) -> Option<String> {
    let rel: PathBuf = picked.strip_prefix(root).ok()?.to_path_buf();
    let s = rel.to_string_lossy().replace('\\', "/");
    if s.is_empty() { None } else { Some(s) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{InstallFile, Plugin, PluginGroup, SelectionType, Step};
    use std::fs;

    fn tmp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod_refs_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn touch(root: &Path, rel: &str) {
        let p = root.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, b"x").unwrap();
    }

    fn project(files: &[InstallFile], image: Option<&str>) -> Ximod {
        let mut x = Ximod::new("T");
        let mut p = Plugin::new("A");
        p.files.extend(files.iter().cloned());
        p.image_path = image.map(str::to_string);
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        x.steps.push(s);
        x
    }

    fn sources(x: &Ximod) -> Vec<String> {
        let mut v = Vec::new();
        for_each_ref(x, |_, r| v.push(r.to_string()));
        v
    }

    #[test]
    fn retarget_keeps_separators_case_and_follows_folders() {
        let mut x = project(
            &[
                InstallFile::new_folder("Feature D\\A"),
                InstallFile::new_file("Feature D\\A\\Plugin.esp"),
                InstallFile::new_file("Feature D\\AB\\other.esp"),
                InstallFile::new_file("feature d/a/textures/x.dds"),
            ],
            Some("Feature D\\A\\pic.png"),
        );
        let n = retarget(&mut x, "Feature D/A", "Feature D/Alpha", true);
        assert_eq!(n, 4);
        let s = sources(&x);
        assert!(s.contains(&"Feature D\\Alpha".to_string()));
        assert!(s.contains(&"Feature D\\Alpha\\Plugin.esp".to_string()));
        // A sibling sharing the prefix text is not inside the folder.
        assert!(s.contains(&"Feature D\\AB\\other.esp".to_string()));
        // The tail keeps its own case; forward slashes stay forward.
        assert!(s.contains(&"Feature D/Alpha/textures/x.dds".to_string()));
        assert!(s.contains(&"Feature D\\Alpha\\pic.png".to_string()));
        // A file rename touches the exact reference only.
        assert_eq!(
            retarget(&mut x, "feature d\\alpha\\plugin.esp", "Feature D/Alpha/P2.esp", false),
            1
        );
        assert_eq!(count_uses(&x, "Feature D/Alpha", true), 4);
        assert_eq!(count_uses(&x, "Feature D/Alpha", false), 1);
    }

    #[test]
    fn rename_on_disk_renames_and_updates_references() {
        let root = tmp("rename");
        touch(&root, "Feature D/A/Plugin.esp");
        touch(&root, "Feature D/B/Other.esp");
        let mut x = project(
            &[
                InstallFile::new_folder("Feature D\\A"),
                InstallFile::new_file("Feature D\\A\\Plugin.esp"),
            ],
            None,
        );

        // Folder rename: the folder moves, both references follow.
        let out = rename_on_disk(&mut x, &root, "Feature D\\A", "Alpha").unwrap();
        assert_eq!(out.new_rel, "Feature D/Alpha");
        assert_eq!(out.updated, 2);
        assert!(root.join("Feature D/Alpha/Plugin.esp").is_file());
        assert!(sources(&x).contains(&"Feature D\\Alpha\\Plugin.esp".to_string()));

        // Refusals.
        assert_eq!(
            rename_on_disk(&mut x, &root, "Feature D\\Alpha", ""),
            Err(RenameError::EmptyName)
        );
        assert_eq!(
            rename_on_disk(&mut x, &root, "Feature D\\Alpha", "a/b"),
            Err(RenameError::InvalidName)
        );
        assert_eq!(
            rename_on_disk(&mut x, &root, "Feature D\\Alpha", "B"),
            Err(RenameError::AlreadyExists)
        );
        assert_eq!(
            rename_on_disk(&mut x, &root, "Feature D\\Gone", "C"),
            Err(RenameError::Missing)
        );
        assert_eq!(
            rename_on_disk(&mut x, &root, "..\\up", "C"),
            Err(RenameError::OutsideRoot)
        );

        // File rename, then a case-only rename.
        let out = rename_on_disk(&mut x, &root, "Feature D\\Alpha\\Plugin.esp", "Main.esp").unwrap();
        assert_eq!(out.updated, 1);
        assert!(root.join("Feature D/Alpha/Main.esp").is_file());
        let out = rename_on_disk(&mut x, &root, "Feature D\\Alpha\\Main.esp", "MAIN.esp").unwrap();
        assert_eq!(out.new_rel, "Feature D/Alpha/MAIN.esp");
        assert!(sources(&x).contains(&"Feature D\\Alpha\\MAIN.esp".to_string()));
        let names: Vec<String> = fs::read_dir(root.join("Feature D/Alpha"))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        assert_eq!(names, vec!["MAIN.esp".to_string()]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn missing_references_get_moved_and_renamed_candidates() {
        let root = tmp("missing");
        touch(&root, "Common/Plugin.esp"); // moved here
        touch(&root, "Feature/Textures v2/a.dds"); // folder renamed
        touch(&root, "Feature/MyMod - Patch.esp"); // file renamed in place
        touch(&root, "Feature/Unrelated.txt");
        touch(&root, "fomod/backups/20260101-000000/Plugin.esp"); // never a candidate
        let mut x = project(
            &[
                InstallFile::new_file("Feature\\Plugin.esp"),
                InstallFile::new_folder("Feature\\Textures"),
                InstallFile::new_file("Feature\\MyMod Patch.esp"),
                InstallFile::new_file("Feature\\Plugin.esp"),
            ],
            None,
        );
        let missing = find_missing(&x, &root);
        assert_eq!(missing.len(), 3);

        let plugin = missing.iter().find(|m| m.path == "Feature\\Plugin.esp").unwrap();
        assert_eq!(plugin.uses, 2);
        assert_eq!(plugin.obvious().map(|c| c.rel.as_str()), Some("Common/Plugin.esp"));

        let tex = missing.iter().find(|m| m.kind == RefKind::Folder).unwrap();
        assert_eq!(tex.candidates[0].rel, "Feature/Textures v2");
        assert_eq!(tex.candidates[0].reason, Reason::Renamed);
        assert!(tex.obvious().is_none());

        let patch = missing.iter().find(|m| m.path == "Feature\\MyMod Patch.esp").unwrap();
        assert_eq!(patch.candidates[0].rel, "Feature/MyMod - Patch.esp");
        assert!(patch.candidates.iter().all(|c| !c.rel.ends_with(".txt")));

        let n = apply_relinks(
            &mut x,
            &[
                (plugin.path.clone(), plugin.candidates[0].rel.clone(), plugin.kind),
                (tex.path.clone(), tex.candidates[0].rel.clone(), tex.kind),
            ],
        );
        assert_eq!(n, 3);
        assert!(sources(&x).contains(&"Common\\Plugin.esp".to_string()));
        assert_eq!(find_missing(&x, &root).len(), 1);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn similarity_and_names() {
        assert!((similarity("abc", "abc") - 1.0).abs() < f32::EPSILON);
        assert!(similarity("mymod patch.esp", "mymod - patch.esp") > 0.8);
        assert!(similarity("a.esp", "zzzzzz.esp") < 0.4);
        assert!(valid_file_name("Main.esp"));
        assert!(!valid_file_name("a:b"));
        assert!(!valid_file_name("trailing."));
        assert_eq!(norm(".\\Feature\\A\\"), "feature/a");
    }
}
