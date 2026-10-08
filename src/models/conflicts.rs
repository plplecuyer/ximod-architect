//! Destination-conflict detection (V2 roadmap, priority 2).
//!
//! Two options (plugins) or file sets that write different sources to the **same
//! install destination path** will overwrite each other at install time — a
//! class of bug the FOMOD schema cannot catch, because each option is valid on
//! its own. This module computes the final install path of every referenced
//! file, groups the references by that path, and reports the collisions.
//!
//! Final path rules (matching FOMOD semantics):
//! - a *file* entry installs to `destination/<basename(source)>` (or to the Data
//!   root when the destination is empty);
//! - a *folder* entry installs its **contents** under `destination`, preserving
//!   substructure. Folder contents are expanded by walking the source folder
//!   under `root`, so folder-vs-folder and file-vs-folder collisions are detected
//!   at the exact file level (no false positives from folders that merely share a
//!   destination but hold different files).
//!
//! Bethesda archives (`.bsa` / `.ba2`) are opened too: every file they hold
//! is a contributor at `<destination of the archive>/<entry path>`, with a
//! source written `<archive>::<entry>`, so two archives packing the same
//! asset, or an archive and a loose file, are reported. Mod managers load
//! loose files over archives, which is why [`Conflict::kind`] tells them
//! apart.
//!
//! Like [`super::verify`], this module is i18n-free: the UI maps [`Conflict`] /
//! [`ConflictSource`] to translated messages, reusing [`RefLoc`] for the "where".

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use walkdir::WalkDir;

use super::bethesda_archive::{ArchiveListing, is_bethesda_archive, list_archive};
use super::verify::RefLoc;
use super::{FileType, SelectionType, Ximod};

/// Separator between an archive and an entry inside it in a
/// [`ConflictSource::source`].
pub const ARCHIVE_SEP: &str = "::";

/// One reference participating in a destination collision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictSource {
    /// Where the reference lives (plugin, required files, conditional set…).
    pub loc: RefLoc,
    /// The source path (as authored, before normalization).
    pub source: String,
}

impl ConflictSource {
    /// Whether the reference is a file *inside* a Bethesda archive
    /// (`<archive>::<entry>`).
    pub fn in_archive(&self) -> bool {
        self.source.contains(ARCHIVE_SEP)
    }
}

/// What kinds of files collide, which decides who wins at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictKind {
    /// Loose files only: the install order decides.
    Loose,
    /// A file inside an archive against a loose file: the loose file wins,
    /// whatever the installer does.
    ArchiveVsLoose,
    /// Files inside two archives: the game's archive load order decides.
    ArchiveVsArchive,
}

impl ConflictKind {
    /// Derive the kind from the contributors.
    fn of(sources: &[ConflictSource]) -> Self {
        let archived = sources.iter().filter(|s| s.in_archive()).count();
        if archived == 0 {
            ConflictKind::Loose
        } else if archived == sources.len() {
            ConflictKind::ArchiveVsArchive
        } else {
            ConflictKind::ArchiveVsLoose
        }
    }
}

/// A set of two or more references that resolve to the same destination path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conflict {
    /// Normalized final destination path (lowercased, forward slashes).
    pub destination: String,
    /// The references writing to that path (length ≥ 2), from distinct locations.
    pub sources: Vec<ConflictSource>,
    /// Whether at least two of the colliding references can be active together in
    /// a valid selection (a *certain* conflict), as opposed to one that only
    /// arises across mutually exclusive options or condition-gated sets.
    pub certain: bool,
    /// Whether archives are involved (see [`ConflictKind`]).
    pub kind: ConflictKind,
}

impl std::fmt::Display for Conflict {
    /// English one-liner for the CLI.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = match (self.kind, self.certain) {
            (ConflictKind::Loose, true) => "destination conflict",
            (ConflictKind::Loose, false) => "possible destination conflict",
            (ConflictKind::ArchiveVsLoose, _) => "archive vs loose file (the loose file wins)",
            (ConflictKind::ArchiveVsArchive, _) => "same asset in several archives",
        };
        let sources: Vec<String> = self
            .sources
            .iter()
            .map(|s| format!("{} ({:?})", s.source, s.loc))
            .collect();
        write!(f, "{what}: \"{}\" <- {}", self.destination, sources.join(", "))
    }
}

/// How strict the scan is about what counts as a conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConflictMode {
    /// Report only collisions that can actually happen together (default).
    #[default]
    CertainOnly,
    /// Report every path shared by references from distinct locations, even
    /// across mutually exclusive options (noisier, but exhaustive).
    /// Used by tests and a future "show all conflicts" UI toggle.
    #[allow(dead_code)]
    All,
}

/// Upper bound on reported conflicts, to avoid a runaway report on huge trees.
const MAX_CONFLICTS: usize = 500;

/// Upper bound on the archives opened by one scan.
const MAX_ARCHIVES: usize = 50;

/// Per-scan cache of archive listings: an archive referenced by several
/// options is opened once; one that cannot be read stays `None`.
type ArchiveCache = HashMap<PathBuf, Option<Rc<ArchiveListing>>>;

/// The listing of the archive at `abs`, through the cache. Unreadable or
/// over-budget archives yield `None` (the file still conflicts by name).
fn cached_listing(cache: &mut ArchiveCache, abs: &Path) -> Option<Rc<ArchiveListing>> {
    if let Some(hit) = cache.get(abs) {
        return hit.clone();
    }
    let listing = if cache.len() < MAX_ARCHIVES {
        list_archive(abs).ok().map(Rc::new)
    } else {
        None
    };
    cache.insert(abs.to_path_buf(), listing.clone());
    listing
}

/// Push one contributor per file inside the archive at `abs`, installed at
/// `dest_dir` as `<source>::<entry>`.
fn add_archive_entries(
    cache: &mut ArchiveCache,
    abs: &Path,
    dest_dir: &str,
    loc: &RefLoc,
    source: &str,
    refs: &mut Vec<TargetRef>,
) {
    let Some(listing) = cached_listing(cache, abs) else {
        return;
    };
    for entry in &listing.entries {
        refs.push(TargetRef {
            target: join_norm(dest_dir, &entry.path),
            loc: loc.clone(),
            source: format!("{source}{ARCHIVE_SEP}{}", entry.path),
        });
    }
}

/// Destination directory of a file installed at `target` (its parent).
fn parent_dir(target: &str) -> &str {
    target.rsplit_once('/').map(|(d, _)| d).unwrap_or("")
}

/// A single (final path → contributor) record collected during the scan.
struct TargetRef {
    /// Normalized final install path.
    target: String,
    loc: RefLoc,
    source: String,
}

/// Detect destination conflicts across the whole project, resolving folder
/// sources against the `root` folder.
pub fn detect_conflicts(ximod: &Ximod, root: &Path, mode: ConflictMode) -> Vec<Conflict> {
    let mut refs: Vec<TargetRef> = Vec::new();
    let mut archives: ArchiveCache = HashMap::new();

    let mut add = |loc: RefLoc, file: &super::InstallFile, refs: &mut Vec<TargetRef>| {
        let source = file.source.trim();
        if source.is_empty() || is_absolute_ref(source) || escapes_root(source) {
            // Absolute/escaping refs are reported by `verify`; skip here.
            return;
        }
        match file.file_type {
            FileType::File => {
                let target = join_norm(&file.destination, basename(source));
                if is_bethesda_archive(source) {
                    let abs = root.join(norm_keep_case(source));
                    add_archive_entries(&mut archives, &abs, parent_dir(&target), &loc, source, refs);
                }
                refs.push(TargetRef {
                    target,
                    loc: loc.clone(),
                    source: source.to_string(),
                });
            }
            FileType::Folder => {
                // Expand the folder's contents under the root.
                let base = root.join(norm_keep_case(source));
                if !base.is_dir() {
                    return; // missing folder is a `verify` concern
                }
                for entry in WalkDir::new(&base).into_iter().filter_map(Result::ok) {
                    if !entry.file_type().is_file() {
                        continue;
                    }
                    let Ok(rel) = entry.path().strip_prefix(&base) else {
                        continue;
                    };
                    let rel = rel.to_string_lossy();
                    let target = join_norm(&file.destination, &rel);
                    if is_bethesda_archive(entry.path()) {
                        let inner = format!("{source}/{}", rel.replace('\\', "/"));
                        add_archive_entries(&mut archives, entry.path(), parent_dir(&target), &loc, &inner, refs);
                    }
                    refs.push(TargetRef {
                        target,
                        loc: loc.clone(),
                        source: source.to_string(),
                    });
                }
            }
        }
    };

    for f in &ximod.required_files {
        add(RefLoc::RequiredFiles, f, &mut refs);
    }
    for (i, set) in ximod.conditional_files.iter().enumerate() {
        for f in &set.files {
            add(RefLoc::ConditionalSet { index: i + 1 }, f, &mut refs);
        }
    }
    for (si, step) in ximod.steps.iter().enumerate() {
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            for plugin in &group.plugins {
                let loc = RefLoc::Plugin {
                    step: si + 1,
                    group: gi + 1,
                    plugin: plugin.name.clone(),
                };
                for f in &plugin.files {
                    add(loc.clone(), f, &mut refs);
                }
            }
        }
    }

    // Group references by final target path.
    let mut by_target: HashMap<String, Vec<(RefLoc, String)>> = HashMap::new();
    for r in refs {
        by_target.entry(r.target).or_default().push((r.loc, r.source));
    }

    let mut conflicts: Vec<Conflict> = Vec::new();
    for (target, mut contributors) in by_target {
        // Keep one entry per (loc, source); a collision needs ≥ 2 *distinct*
        // locations (two different options/places writing the same path).
        contributors.sort_by(|a, b| describe_loc(&a.0).cmp(&describe_loc(&b.0)).then(a.1.cmp(&b.1)));
        contributors.dedup();
        let distinct_locs = {
            let mut keys: Vec<String> = contributors.iter().map(|(l, _)| describe_loc(l)).collect();
            keys.sort();
            keys.dedup();
            keys.len()
        };
        if distinct_locs < 2 {
            continue;
        }

        let certain = any_pair_coexists(ximod, &contributors);
        if mode == ConflictMode::CertainOnly && !certain {
            continue;
        }

        let sources: Vec<ConflictSource> = contributors
            .into_iter()
            .map(|(loc, source)| ConflictSource { loc, source })
            .collect();
        let kind = ConflictKind::of(&sources);
        conflicts.push(Conflict {
            destination: target,
            sources,
            certain,
            kind,
        });
    }

    // Stable, useful ordering: certain first, then by path.
    conflicts.sort_by(|a, b| b.certain.cmp(&a.certain).then(a.destination.cmp(&b.destination)));
    conflicts.truncate(MAX_CONFLICTS);
    conflicts
}

/// True if any two contributors can be active together in a valid selection.
fn any_pair_coexists(ximod: &Ximod, contributors: &[(RefLoc, String)]) -> bool {
    for i in 0..contributors.len() {
        for j in (i + 1)..contributors.len() {
            if can_coexist(ximod, &contributors[i].0, &contributors[j].0) {
                return true;
            }
        }
    }
    false
}

/// Whether two reference locations can both be active in some valid selection.
///
/// The only structural impossibility is two distinct plugins in the **same**
/// group when that group forces at most one choice (`SelectExactlyOne` /
/// `SelectAtMostOne`). Condition-gated sets depend on runtime flags we do not
/// evaluate here, so any `ConditionalSet` is treated as *not certain*.
fn can_coexist(ximod: &Ximod, a: &RefLoc, b: &RefLoc) -> bool {
    if matches!(a, RefLoc::ConditionalSet { .. }) || matches!(b, RefLoc::ConditionalSet { .. }) {
        return false;
    }
    if let (
        RefLoc::Plugin {
            step: sa,
            group: ga,
            plugin: pa,
        },
        RefLoc::Plugin {
            step: sb,
            group: gb,
            plugin: pb,
        },
    ) = (a, b)
    {
        if sa == sb && ga == gb && pa != pb {
            // Same group: mutually exclusive selection types cannot coexist.
            if let Some(group) = ximod
                .steps
                .get(sa.saturating_sub(1))
                .and_then(|s| s.plugin_groups.get(ga.saturating_sub(1)))
            {
                return !matches!(
                    group.selection_type,
                    SelectionType::SelectExactlyOne | SelectionType::SelectAtMostOne
                );
            }
        }
        let ia = plugin_index(ximod, *sa, *ga, pa);
        let ib = plugin_index(ximod, *sb, *gb, pb);
        if let (Some(ia), Some(ib)) = (ia, ib)
            && (excluded_by_flags(ximod, ia, ib) || excluded_by_flags(ximod, ib, ia))
        {
            return false;
        }
    }
    true
}

/// `(step, group, plugin)` indices (0-based) of a plugin location.
fn plugin_index(ximod: &Ximod, step: usize, group: usize, plugin: &str) -> Option<(usize, usize, usize)> {
    let si = step.checked_sub(1)?;
    let gi = group.checked_sub(1)?;
    let pi = ximod
        .steps
        .get(si)?
        .plugin_groups
        .get(gi)?
        .plugins
        .iter()
        .position(|p| p.name == plugin)?;
    Some((si, gi, pi))
}

/// Flag values a step's visibility requires in *every* satisfying
/// assignment: the flag leaves of an `And` group plus, recursively, those of
/// its nested `And` groups. Nothing is taken from an `Or` group (any one
/// branch may do), so an `Or` step is treated as always reachable.
fn step_requirements(ximod: &Ximod, si: usize) -> Vec<(&str, &str)> {
    let Some(step) = ximod.steps.get(si) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    required_leaves(&step.visibility, &mut out);
    out
}

/// See [`step_requirements`].
fn required_leaves<'a>(group: &'a super::DependencyGroup, out: &mut Vec<(&'a str, &'a str)>) {
    if group.operator != super::LogicalOperator::And {
        return;
    }
    for item in &group.items {
        match item {
            super::DependencyItem::Leaf(d) if d.is_flag() => out.push((d.name.as_str(), d.value.as_str())),
            super::DependencyItem::Leaf(_) => {}
            super::DependencyItem::Group(g) => required_leaves(g, out),
        }
    }
}

/// Whether selecting plugin `a` makes plugin `b` unreachable through flags:
/// `a` sets a flag to a value, `b`'s step is only shown for another value of
/// that flag, and nothing that can be selected together with `a` sets that
/// other value (setters in `a`'s own exclusive group cannot be chosen with
/// it, and setters that live in steps themselves gated on that value cannot
/// bootstrap it). A later, freely selectable setter keeps the pair possible.
fn excluded_by_flags(ximod: &Ximod, a: (usize, usize, usize), b: (usize, usize, usize)) -> bool {
    let Some(pa) = ximod
        .steps
        .get(a.0)
        .and_then(|s| s.plugin_groups.get(a.1))
        .and_then(|g| g.plugins.get(a.2))
    else {
        return false;
    };
    let a_group_exclusive = ximod
        .steps
        .get(a.0)
        .and_then(|s| s.plugin_groups.get(a.1))
        .is_some_and(|g| {
            matches!(
                g.selection_type,
                SelectionType::SelectExactlyOne | SelectionType::SelectAtMostOne
            )
        });
    let requirements = step_requirements(ximod, b.0);
    for set in &pa.condition_flags {
        for &(flag, wanted) in &requirements {
            if set.name != flag || set.value == wanted {
                continue;
            }
            // Can anything selectable alongside `a` still set `flag = wanted`?
            let mut rescued = false;
            'search: for (si, step) in ximod.steps.iter().enumerate() {
                if step_requirements(ximod, si).contains(&(flag, wanted)) {
                    continue; // gated on the very value it would set
                }
                for (gi, group) in step.plugin_groups.iter().enumerate() {
                    for (pi, plugin) in group.plugins.iter().enumerate() {
                        if (si, gi, pi) == a {
                            continue;
                        }
                        if a_group_exclusive && (si, gi) == (a.0, a.1) {
                            continue; // cannot be chosen together with `a`
                        }
                        if plugin
                            .condition_flags
                            .iter()
                            .any(|f| f.name == flag && f.value == wanted)
                        {
                            rescued = true;
                            break 'search;
                        }
                    }
                }
            }
            if !rescued {
                return true;
            }
        }
    }
    false
}

/// Stable string key for a location (for dedupe/ordering).
fn describe_loc(loc: &RefLoc) -> String {
    match loc {
        RefLoc::Header => "header".into(),
        RefLoc::RequiredFiles => "required".into(),
        RefLoc::ConditionalSet { index } => format!("cond:{index}"),
        RefLoc::Plugin { step, group, plugin } => format!("plugin:{step}:{group}:{plugin}"),
    }
}

/// Last path component of a source (handles both `/` and `\\`).
pub(crate) fn basename(source: &str) -> &str {
    source
        .trim_end_matches(['/', '\\'])
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(source)
}

/// Join a destination and a relative name, then normalize (lowercased,
/// forward-slash, no leading `./`). An empty destination means the Data root.
pub(crate) fn join_norm(destination: &str, name: &str) -> String {
    let dest = destination.trim().replace('\\', "/");
    let dest = dest.trim_matches('/');
    let name = name.replace('\\', "/");
    let name = name.trim_start_matches("./").trim_start_matches('/');
    let joined = if dest.is_empty() {
        name.to_string()
    } else {
        format!("{dest}/{name}")
    };
    joined.to_lowercase()
}

/// Normalize a source path for disk access, keeping the original case.
fn norm_keep_case(raw: &str) -> String {
    raw.replace('\\', "/").trim_start_matches("./").to_string()
}

fn is_absolute_ref(p: &str) -> bool {
    let b = p.as_bytes();
    p.starts_with('/') || p.starts_with('\\') || (b.len() >= 2 && b[1] == b':' && b[0].is_ascii_alphabetic())
}

fn escapes_root(p: &str) -> bool {
    p.replace('\\', "/").split('/').any(|c| c == "..")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{InstallFile, Plugin, PluginGroup, SelectionType, Step, Ximod};

    #[test]
    fn empty_project_has_no_conflicts() {
        let x = Ximod::new("Test");
        let root = std::env::temp_dir();
        assert!(detect_conflicts(&x, &root, ConflictMode::CertainOnly).is_empty());
    }

    #[test]
    fn two_options_same_destination_flagged() {
        // Two plugins in a SelectAny group both install a file to the same
        // destination path -> certain conflict.
        let root = std::env::temp_dir();
        let mut x = Ximod::new("M");

        let mut p1 = Plugin::new("A");
        let mut f1 = InstallFile::new_file("a/skse.ini");
        f1.destination = "SKSE/Plugins".into();
        p1.files.push(f1);

        let mut p2 = Plugin::new("B");
        let mut f2 = InstallFile::new_file("b/skse.ini");
        f2.destination = "SKSE/Plugins".into();
        p2.files.push(f2);

        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p1);
        g.plugins.push(p2);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        x.steps.push(s);

        let c = detect_conflicts(&x, &root, ConflictMode::CertainOnly);
        assert_eq!(c.len(), 1, "exactly one colliding path");
        assert_eq!(c[0].destination, "skse/plugins/skse.ini");
        assert_eq!(c[0].sources.len(), 2);
        assert!(c[0].certain);
    }

    /// A "preset" option (exclusive group, sets `Preset = 1X`) and an option on
    /// a page only shown for `Preset = Custom` can never be installed together:
    /// the only other setters of `Custom` are in the same exclusive group or on
    /// pages themselves gated on `Custom`. A free setter elsewhere (the
    /// "choose your own" option of a SelectAny group) makes the pair possible.
    #[test]
    fn flag_gated_pages_make_preset_conflicts_uncertain() {
        use crate::models::{ConditionFlag, Dependency};
        let root = std::env::temp_dir();
        let file = |src: &str| {
            let mut f = InstallFile::new_file(src);
            f.destination = "".into();
            f
        };
        let build = |presets_exclusive: bool| {
            let mut x = Ximod::new("M");
            let mut presets = PluginGroup::new(
                "Presets",
                if presets_exclusive {
                    SelectionType::SelectExactlyOne
                } else {
                    SelectionType::SelectAny
                },
            );
            let mut custom = Plugin::new("Custom");
            custom.condition_flags.push(ConditionFlag {
                name: "Preset".into(),
                value: "Custom".into(),
            });
            let mut one = Plugin::new("1X");
            one.condition_flags.push(ConditionFlag {
                name: "Preset".into(),
                value: "1X".into(),
            });
            one.files.push(file("Features/1.0/x.jsonc"));
            presets.plugins.push(custom);
            presets.plugins.push(one);
            let mut s1 = Step::new("Presets");
            s1.plugin_groups.push(presets);
            x.steps.push(s1);
            // Page shown only for Custom; one option there sets Custom again
            // (harmless leftover) and ships the same file as the preset.
            let mut s2 = Step::new("Base");
            s2.visibility.push_leaf(Dependency::new_flag("Preset", "Custom"));
            let mut g = PluginGroup::new("Size", SelectionType::SelectAtMostOne);
            let mut radius = Plugin::new("1x Radius");
            radius.condition_flags.push(ConditionFlag {
                name: "Preset".into(),
                value: "Custom".into(),
            });
            radius.files.push(file("Features/1.0/x.jsonc"));
            g.plugins.push(radius);
            s2.plugin_groups.push(g);
            x.steps.push(s2);
            x
        };
        let exclusive = build(true);
        assert!(detect_conflicts(&exclusive, &root, ConflictMode::CertainOnly).is_empty());
        let all = detect_conflicts(&exclusive, &root, ConflictMode::All);
        assert_eq!(all.len(), 1);
        assert!(!all[0].certain);
        // SelectAny presets: "Custom" can be ticked together with "1X", so the
        // gated page is reachable and the conflict is certain.
        let any = build(false);
        let c = detect_conflicts(&any, &root, ConflictMode::CertainOnly);
        assert_eq!(c.len(), 1);
        assert!(c[0].certain);
    }

    #[test]
    fn mutually_exclusive_options_not_certain() {
        // Same two files, but in a SelectExactlyOne group: cannot coexist, so it
        // is filtered out in CertainOnly and kept (not certain) in All.
        let root = std::env::temp_dir();
        let mut x = Ximod::new("M");
        let mut p1 = Plugin::new("A");
        let mut f1 = InstallFile::new_file("a/x.esp");
        f1.destination = "".into();
        p1.files.push(f1);
        let mut p2 = Plugin::new("B");
        let mut f2 = InstallFile::new_file("b/x.esp");
        f2.destination = "".into();
        p2.files.push(f2);
        let mut g = PluginGroup::new("G", SelectionType::SelectExactlyOne);
        g.plugins.push(p1);
        g.plugins.push(p2);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        x.steps.push(s);

        assert!(detect_conflicts(&x, &root, ConflictMode::CertainOnly).is_empty());
        let all = detect_conflicts(&x, &root, ConflictMode::All);
        assert_eq!(all.len(), 1);
        assert!(!all[0].certain);
    }

    #[test]
    fn same_option_no_self_conflict() {
        // One plugin listing two files to the same path is not a cross-option
        // conflict (distinct locations < 2).
        let root = std::env::temp_dir();
        let mut x = Ximod::new("M");
        let mut p = Plugin::new("A");
        let mut f1 = InstallFile::new_file("a/x.esp");
        f1.destination = "".into();
        let mut f2 = InstallFile::new_file("a/x.esp");
        f2.destination = "".into();
        p.files.push(f1);
        p.files.push(f2);
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        x.steps.push(s);
        assert!(detect_conflicts(&x, &root, ConflictMode::All).is_empty());
    }

    /// A project root with two BA2 archives sharing one asset, plus a loose
    /// copy of another asset held by one of them.
    fn archive_root(tag: &str) -> std::path::PathBuf {
        use crate::archive::{Ba2Game, Ba2Options, package_ba2};
        let root = std::env::temp_dir().join(format!("ximod_conf_ba2_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (src, files) in [
            ("srcA", vec!["textures/shared.dds", "meshes/a.nif"]),
            ("srcB", vec!["textures/shared.dds", "meshes/b.nif"]),
        ] {
            for f in files {
                let p = root.join(src).join(f);
                std::fs::create_dir_all(p.parent().unwrap()).unwrap();
                std::fs::write(&p, f.as_bytes()).unwrap();
            }
        }
        let opts = Ba2Options {
            game: Ba2Game::Fallout4,
            general: true,
        };
        std::fs::create_dir_all(root.join("optA")).unwrap();
        std::fs::create_dir_all(root.join("optB/sub")).unwrap();
        package_ba2(&root.join("srcA"), &root.join("optA/Mod - A.ba2"), &opts).unwrap();
        package_ba2(&root.join("srcB"), &root.join("optB/sub/Mod - B.ba2"), &opts).unwrap();
        std::fs::create_dir_all(root.join("loose/meshes")).unwrap();
        std::fs::write(root.join("loose/meshes/a.nif"), b"loose").unwrap();
        std::fs::write(root.join("optA/Broken.ba2"), b"not an archive").unwrap();
        root
    }

    #[test]
    fn archives_conflict_by_content_and_against_loose_files() {
        let root = archive_root("content");
        let mut x = Ximod::new("M");

        // Option A ships its archive as a file, option B as a folder holding
        // the archive in a sub-folder (the entries land under that sub-folder).
        let mut p1 = Plugin::new("A");
        p1.files.push(InstallFile::new_file("optA/Mod - A.ba2"));
        p1.files.push(InstallFile::new_file("optA/Broken.ba2"));
        let mut p2 = Plugin::new("B");
        p2.files.push(InstallFile::new_folder("optB"));
        let mut p3 = Plugin::new("C");
        let mut f3 = InstallFile::new_folder("loose");
        f3.destination = "".into();
        p3.files.push(f3);
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p1);
        g.plugins.push(p2);
        g.plugins.push(p3);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        x.steps.push(s);

        let c = detect_conflicts(&x, &root, ConflictMode::CertainOnly);
        // B's archive sits under sub/, so its textures land under sub/ too:
        // no shared-texture conflict yet; A's meshes/a.nif vs the loose one.
        assert_eq!(c.len(), 1, "{c:#?}");
        assert_eq!(c[0].destination, "meshes/a.nif");
        assert_eq!(c[0].kind, ConflictKind::ArchiveVsLoose);
        assert!(c[0].certain);
        let archived = c[0].sources.iter().find(|s| s.in_archive()).unwrap();
        assert_eq!(archived.source, "optA/Mod - A.ba2::meshes/a.nif");
        assert!(c[0].sources.iter().any(|s| !s.in_archive() && s.source == "loose"));
        assert!(c[0].to_string().contains("loose file wins"), "{}", c[0]);

        // Install B's folder at the Data root level: now both archives pack
        // textures/shared.dds -> archive-vs-archive, and the broken archive
        // never breaks the scan.
        x.steps[0].plugin_groups[0].plugins[1].files[0] = InstallFile::new_folder("optB/sub");
        let c = detect_conflicts(&x, &root, ConflictMode::CertainOnly);
        let shared = c
            .iter()
            .find(|c| c.destination == "textures/shared.dds")
            .expect("shared asset");
        assert_eq!(shared.kind, ConflictKind::ArchiveVsArchive);
        assert!(shared.sources.iter().all(|s| s.in_archive()));
        assert_eq!(shared.sources.len(), 2);
        assert!(shared.to_string().contains("several archives"));
        assert!(c.iter().all(|c| !c.destination.contains("broken")));

        // Two options shipping the same archive name still conflict by name
        // (a plain loose conflict) on top of their contents.
        x.steps[0].plugin_groups[0].plugins[1].files[0] = InstallFile::new_file("optA/Mod - A.ba2");
        let c = detect_conflicts(&x, &root, ConflictMode::CertainOnly);
        let by_name = c
            .iter()
            .find(|c| c.destination == "mod - a.ba2")
            .expect("archive by name");
        assert_eq!(by_name.kind, ConflictKind::Loose);
        assert!(
            c.iter()
                .any(|c| c.destination == "textures/shared.dds" && c.kind == ConflictKind::ArchiveVsArchive)
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn archive_cache_lists_each_archive_once() {
        let root = archive_root("cache");
        let mut cache = ArchiveCache::new();
        let abs = root.join("optA/Mod - A.ba2");
        let a = cached_listing(&mut cache, &abs).unwrap();
        let b = cached_listing(&mut cache, &abs).unwrap();
        assert!(Rc::ptr_eq(&a, &b));
        assert!(cached_listing(&mut cache, &root.join("optA/Broken.ba2")).is_none());
        assert_eq!(cache.len(), 2);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn conflict_kind_follows_the_sources() {
        let src = |s: &str| ConflictSource {
            loc: RefLoc::RequiredFiles,
            source: s.into(),
        };
        assert_eq!(ConflictKind::of(&[src("a.esp"), src("b.esp")]), ConflictKind::Loose);
        assert_eq!(
            ConflictKind::of(&[src("a.ba2::x.nif"), src("b.ba2::x.nif")]),
            ConflictKind::ArchiveVsArchive
        );
        assert_eq!(
            ConflictKind::of(&[src("a.ba2::x.nif"), src("x.nif")]),
            ConflictKind::ArchiveVsLoose
        );
    }

    /// Lot N: a step's requirements come from `And` levels only (nested
    /// `And` groups included); nothing is required by an `Or` branch.
    #[test]
    fn step_requirements_ignore_or_branches() {
        use crate::models::{Dependency, DependencyGroup, LogicalOperator};
        let mut x = Ximod::new("M");
        let mut s = Step::new("S");
        s.visibility.push_leaf(Dependency::new_flag("A", "1"));
        s.visibility.push_leaf(Dependency::new_file("X.esm", "Active"));
        let mut or = DependencyGroup::new(LogicalOperator::Or);
        or.push_leaf(Dependency::new_flag("B", "1"));
        or.push_leaf(Dependency::new_flag("B", "2"));
        s.visibility.push_group(or);
        let mut and = DependencyGroup::new(LogicalOperator::And);
        and.push_leaf(Dependency::new_flag("C", "3"));
        let mut deeper_or = DependencyGroup::new(LogicalOperator::Or);
        deeper_or.push_leaf(Dependency::new_flag("D", "4"));
        and.push_group(deeper_or);
        s.visibility.push_group(and);
        x.steps.push(s);
        assert_eq!(step_requirements(&x, 0), vec![("A", "1"), ("C", "3")]);
        // An Or at the top requires nothing.
        x.steps[0].visibility.operator = LogicalOperator::Or;
        assert!(step_requirements(&x, 0).is_empty());
        assert!(step_requirements(&x, 7).is_empty());
    }
}
