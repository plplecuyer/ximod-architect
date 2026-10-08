//! Installer simulation (V2 lot F3, feature 6): the pure logic behind the
//! preview window, usable from the CLI and the validation as well.
//!
//! * [`evaluate`] replays the installer: which steps are visible and which
//!   flags are set, given the selected options and the assumed state of the
//!   game files referenced by `file` dependencies;
//! * [`install_entries`] / [`sorted_entries`] list what gets installed;
//! * [`Scenario`] saves a set of selections and assumptions by *name* (so it
//!   survives reordering) under `<root>/fomod/scenarios/<name>.json`;
//! * [`build_install_tree`] expands the install list into the final file tree,
//!   applying the destination rules of [`super::conflicts`] and the priority
//!   overwrite rules of the mod managers;
//! * [`unreachable`] finds steps, options and conditional sets whose flag
//!   conditions can never hold;
//! * [`option_size`] sums the size of the files an option installs.
//!
//! Language-neutral like the other `models` modules: the UI translates.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use super::bethesda_archive::{ArchiveListing, is_bethesda_archive, list_archive};
use super::conflicts::{basename, join_norm};
use super::verify::RefLoc;
use super::{
    Dependency, DependencyGroup, DependencyItem, FileState, FileType, InstallFile, LogicalOperator, Plugin,
    PluginGroup, PluginType, SelectionType, Ximod,
};

/// (step, group, plugin) index of a plugin in the project tree.
pub type SelKey = (usize, usize, usize);

// --------------------------------------------------------------------------- //
// Condition evaluation
// --------------------------------------------------------------------------- //

/// Whether one dependency holds for the given flags and file assumptions.
/// A file not listed in `files` is assumed Active; an unset flag counts as
/// the empty string. Game / mod-manager version requirements are assumed
/// met (the simulation has no game to check against).
pub fn dep_satisfied(dep: &Dependency, flags: &HashMap<String, String>, files: &BTreeMap<String, FileState>) -> bool {
    match dep.kind() {
        super::DependencyType::File => {
            let st = files.get(&dep.name).copied().unwrap_or(FileState::Active);
            st.as_str().eq_ignore_ascii_case(&dep.value)
        }
        super::DependencyType::Flag => match flags.get(&dep.name) {
            Some(v) => v == &dep.value,
            None => dep.value.is_empty(),
        },
        super::DependencyType::Game | super::DependencyType::Fomm => true,
    }
}

/// Whether a condition group holds: `And` needs every item, `Or` any one;
/// nested groups are evaluated recursively and an empty group always holds.
pub fn group_satisfied(
    group: &DependencyGroup,
    flags: &HashMap<String, String>,
    files: &BTreeMap<String, FileState>,
) -> bool {
    if group.items.is_empty() {
        return true;
    }
    let item_ok = |item: &DependencyItem| match item {
        DependencyItem::Leaf(d) => dep_satisfied(d, flags, files),
        DependencyItem::Group(g) => group_satisfied(g, flags, files),
    };
    match group.operator {
        LogicalOperator::Or => group.items.iter().any(item_ok),
        LogicalOperator::And => group.items.iter().all(item_ok),
    }
}

/// Whether a flat dependency list holds under `op` (an empty list always
/// holds). Kept for callers that evaluate a list of leaves on its own.
#[allow(dead_code)]
pub fn deps_satisfied(
    op: LogicalOperator,
    deps: &[Dependency],
    flags: &HashMap<String, String>,
    files: &BTreeMap<String, FileState>,
) -> bool {
    if deps.is_empty() {
        return true;
    }
    match op {
        LogicalOperator::Or => deps.iter().any(|d| dep_satisfied(d, flags, files)),
        _ => deps.iter().all(|d| dep_satisfied(d, flags, files)),
    }
}

/// Effective type of a plugin: the first matching `dependency_patterns`
/// (typeDescriptor) entry, otherwise its default type.
pub fn effective_type(
    plugin: &Plugin,
    flags: &HashMap<String, String>,
    files: &BTreeMap<String, FileState>,
) -> PluginType {
    for pat in &plugin.dependency_patterns {
        if group_satisfied(&pat.condition, flags, files) {
            return PluginType::from_str(&pat.pattern_type);
        }
    }
    plugin.default_type
}

/// Result of one forward pass over the steps.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Evaluation {
    /// Flags set by the selected options of the visible steps.
    pub flags: HashMap<String, String>,
    /// Indices (into `ximod.steps`) of the visible steps, in order.
    pub visible_steps: Vec<usize>,
}

/// Forward pass over the steps: a step's flags come from its selected plugins
/// and only count when the step itself is visible given the flags
/// accumulated so far.
pub fn evaluate(ximod: &Ximod, selections: &HashMap<SelKey, bool>, files: &BTreeMap<String, FileState>) -> Evaluation {
    let mut flags: HashMap<String, String> = HashMap::new();
    let mut visible_steps = Vec::new();
    for (si, step) in ximod.steps.iter().enumerate() {
        if !group_satisfied(&step.visibility, &flags, files) {
            continue;
        }
        visible_steps.push(si);
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            for (pi, plugin) in group.plugins.iter().enumerate() {
                if *selections.get(&(si, gi, pi)).unwrap_or(&false) {
                    for cf in &plugin.condition_flags {
                        flags.insert(cf.name.clone(), cf.value.clone());
                    }
                }
            }
        }
    }
    Evaluation { flags, visible_steps }
}

/// Every file that would be installed, in declaration order (required files,
/// then the files of the selected plugins of each visible step, then the
/// conditional sets whose condition holds), with where each comes from.
///
/// `flags` / `visible` must come from [`evaluate`] with the same inputs.
pub fn install_entries<'a>(
    ximod: &'a Ximod,
    selections: &'a HashMap<SelKey, bool>,
    files: &'a BTreeMap<String, FileState>,
    flags: &'a HashMap<String, String>,
    visible: &'a [usize],
) -> impl Iterator<Item = (RefLoc, &'a InstallFile)> {
    let from_plugins = visible.iter().flat_map(move |&si| {
        ximod.steps[si]
            .plugin_groups
            .iter()
            .enumerate()
            .flat_map(move |(gi, group)| {
                group
                    .plugins
                    .iter()
                    .enumerate()
                    .filter(move |(pi, _)| *selections.get(&(si, gi, *pi)).unwrap_or(&false))
                    .flat_map(move |(_, plugin)| {
                        plugin.files.iter().map(move |f| {
                            (
                                RefLoc::Plugin {
                                    step: si + 1,
                                    group: gi + 1,
                                    plugin: plugin.name.clone(),
                                },
                                f,
                            )
                        })
                    })
            })
    });
    let from_conditions = ximod
        .conditional_files
        .iter()
        .enumerate()
        .filter(move |(_, cfs)| group_satisfied(&cfs.condition, flags, files))
        .flat_map(|(ci, cfs)| {
            cfs.files
                .iter()
                .map(move |f| (RefLoc::ConditionalSet { index: ci + 1 }, f))
        });
    ximod
        .required_files
        .iter()
        .map(|f| (RefLoc::RequiredFiles, f))
        .chain(from_plugins)
        .chain(from_conditions)
}

/// [`install_entries`] without the locations (the plain-file API, kept for
/// callers that do not need the origin of each file).
#[allow(dead_code)]
pub fn install_refs<'a>(
    ximod: &'a Ximod,
    selections: &'a HashMap<SelKey, bool>,
    files: &'a BTreeMap<String, FileState>,
    flags: &'a HashMap<String, String>,
    visible: &'a [usize],
) -> impl Iterator<Item = &'a InstallFile> {
    install_entries(ximod, selections, files, flags, visible).map(|(_, f)| f)
}

/// One line of the install list: the file, where it was declared and its
/// declaration rank (ties between equal priorities are broken by it).
#[derive(Debug, Clone)]
pub struct InstallEntry {
    pub loc: RefLoc,
    pub file: InstallFile,
    pub seq: usize,
}

/// Clone the files and order them the way the summary lists them: by priority,
/// then destination (stable, so ties keep their declaration order).
#[allow(dead_code)]
pub fn sorted_install<'a>(files: impl Iterator<Item = &'a InstallFile>) -> Vec<InstallFile> {
    let mut out: Vec<InstallFile> = files.cloned().collect();
    out.sort_by(|a, b| a.priority.cmp(&b.priority).then(a.destination.cmp(&b.destination)));
    out
}

/// [`sorted_install`] keeping the locations.
pub fn sorted_entries<'a>(entries: impl Iterator<Item = (RefLoc, &'a InstallFile)>) -> Vec<InstallEntry> {
    let mut out: Vec<InstallEntry> = entries
        .enumerate()
        .map(|(seq, (loc, f))| InstallEntry {
            loc,
            file: f.clone(),
            seq,
        })
        .collect();
    out.sort_by(|a, b| {
        a.file
            .priority
            .cmp(&b.file.priority)
            .then(a.file.destination.cmp(&b.file.destination))
    });
    out
}

/// Files that would be installed given the current selections (sorted).
#[allow(dead_code)]
pub fn compute_install(
    ximod: &Ximod,
    selections: &HashMap<SelKey, bool>,
    files: &BTreeMap<String, FileState>,
) -> Vec<InstallFile> {
    let ev = evaluate(ximod, selections, files);
    sorted_install(install_refs(ximod, selections, files, &ev.flags, &ev.visible_steps))
}

/// [`compute_install`] with the locations.
pub fn compute_entries(
    ximod: &Ximod,
    selections: &HashMap<SelKey, bool>,
    files: &BTreeMap<String, FileState>,
) -> Vec<InstallEntry> {
    let ev = evaluate(ximod, selections, files);
    sorted_entries(install_entries(ximod, selections, files, &ev.flags, &ev.visible_steps))
}

/// Every game file referenced by a `file` dependency anywhere in the
/// project, each assumed Active (the assumptions panel edits them).
pub fn referenced_files(ximod: &Ximod) -> BTreeMap<String, FileState> {
    let mut fs: BTreeMap<String, FileState> = BTreeMap::new();
    ximod.for_each_condition(|_, group| {
        for d in group.leaves() {
            if d.is_file() && !d.name.is_empty() {
                fs.entry(d.name.clone()).or_insert(FileState::Active);
            }
        }
    });
    fs
}

/// Default selection for every group, seeded from the plugins' default types
/// (what a mod manager shows before the user touches anything).
pub fn default_selections(ximod: &Ximod, files: &BTreeMap<String, FileState>) -> HashMap<SelKey, bool> {
    let empty: HashMap<String, String> = HashMap::new();
    let mut sel = HashMap::new();
    for (si, step) in ximod.steps.iter().enumerate() {
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            let types: Vec<PluginType> = group.plugins.iter().map(|p| effective_type(p, &empty, files)).collect();
            let n = group.plugins.len();
            match group.selection_type {
                SelectionType::SelectAll => {
                    for (pi, t) in types.iter().enumerate().take(n) {
                        sel.insert((si, gi, pi), *t != PluginType::NotUsable);
                    }
                }
                SelectionType::SelectExactlyOne => {
                    let chosen = pick_one(&types);
                    for pi in 0..n {
                        sel.insert((si, gi, pi), Some(pi) == chosen);
                    }
                }
                SelectionType::SelectAtMostOne => {
                    let chosen = types
                        .iter()
                        .position(|t| *t == PluginType::Required)
                        .or_else(|| types.iter().position(|t| *t == PluginType::Recommended));
                    for pi in 0..n {
                        sel.insert((si, gi, pi), Some(pi) == chosen);
                    }
                }
                SelectionType::SelectAny | SelectionType::SelectAtLeastOne => {
                    for (pi, &t) in types.iter().enumerate().take(n) {
                        sel.insert((si, gi, pi), t == PluginType::Required || t == PluginType::Recommended);
                    }
                    if group.selection_type == SelectionType::SelectAtLeastOne
                        && n > 0
                        && !(0..n).any(|pi| sel[&(si, gi, pi)])
                        && let Some(pi) = types.iter().position(|t| *t != PluginType::NotUsable)
                    {
                        sel.insert((si, gi, pi), true);
                    }
                }
            }
        }
    }
    sel
}

/// For SelectExactlyOne: first Required, else first Recommended, else first
/// usable plugin.
pub fn pick_one(types: &[PluginType]) -> Option<usize> {
    types
        .iter()
        .position(|t| *t == PluginType::Required)
        .or_else(|| types.iter().position(|t| *t == PluginType::Recommended))
        .or_else(|| types.iter().position(|t| *t != PluginType::NotUsable))
        .or(if types.is_empty() { None } else { Some(0) })
}

/// Whether a group's selection count satisfies its selection type.
pub fn group_valid(group: &PluginGroup, selections: &HashMap<SelKey, bool>, si: usize, gi: usize) -> bool {
    let count = (0..group.plugins.len())
        .filter(|pi| *selections.get(&(si, gi, *pi)).unwrap_or(&false))
        .count();
    match group.selection_type {
        SelectionType::SelectExactlyOne => count == 1,
        SelectionType::SelectAtLeastOne => count >= 1,
        SelectionType::SelectAtMostOne => count <= 1,
        _ => true,
    }
}

// --------------------------------------------------------------------------- //
// Scenarios
// --------------------------------------------------------------------------- //

/// Sub-folder of `fomod/` holding the saved scenarios.
pub const SCENARIOS_DIR: &str = "scenarios";

/// One selected option, by names (so the scenario survives reordering).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionSpec {
    pub step: String,
    pub group: String,
    pub plugin: String,
}

/// A saved set of selections and file assumptions.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    pub name: String,
    #[serde(default)]
    pub file_states: BTreeMap<String, FileState>,
    #[serde(default)]
    pub selections: Vec<SelectionSpec>,
}

impl Scenario {
    /// Capture the selected options (by name) and the file assumptions.
    pub fn from_state(
        ximod: &Ximod,
        selections: &HashMap<SelKey, bool>,
        file_states: &BTreeMap<String, FileState>,
        name: impl Into<String>,
    ) -> Self {
        let mut specs = Vec::new();
        for (si, step) in ximod.steps.iter().enumerate() {
            for (gi, group) in step.plugin_groups.iter().enumerate() {
                for (pi, plugin) in group.plugins.iter().enumerate() {
                    if *selections.get(&(si, gi, pi)).unwrap_or(&false) {
                        specs.push(SelectionSpec {
                            step: step.name.clone(),
                            group: group.name.clone(),
                            plugin: plugin.name.clone(),
                        });
                    }
                }
            }
        }
        Self {
            name: name.into(),
            file_states: file_states.clone(),
            selections: specs,
        }
    }

    /// Resolve the scenario against `ximod`: the selection map (every option
    /// listed, selected when named by the scenario), the file assumptions,
    /// and the names that no longer match anything (`step / group / plugin`).
    ///
    /// Fails when the scenario names options but none of them exists in the
    /// project (it belongs to another mod).
    #[allow(clippy::type_complexity)]
    pub fn apply(&self, ximod: &Ximod) -> Result<(HashMap<SelKey, bool>, BTreeMap<String, FileState>, Vec<String>)> {
        let mut selections: HashMap<SelKey, bool> = HashMap::new();
        for (si, step) in ximod.steps.iter().enumerate() {
            for (gi, group) in step.plugin_groups.iter().enumerate() {
                for pi in 0..group.plugins.len() {
                    selections.insert((si, gi, pi), false);
                }
            }
        }
        let mut unresolved = Vec::new();
        let mut resolved = 0usize;
        for spec in &self.selections {
            let key = ximod
                .steps
                .iter()
                .enumerate()
                .filter(|(_, s)| s.name == spec.step)
                .find_map(|(si, s)| {
                    s.plugin_groups
                        .iter()
                        .enumerate()
                        .filter(|(_, g)| g.name == spec.group)
                        .find_map(|(gi, g)| {
                            g.plugins
                                .iter()
                                .position(|p| p.name == spec.plugin)
                                .map(|pi| (si, gi, pi))
                        })
                });
            match key {
                Some(k) => {
                    selections.insert(k, true);
                    resolved += 1;
                }
                None => unresolved.push(format!("{} / {} / {}", spec.step, spec.group, spec.plugin)),
            }
        }
        if resolved == 0 && !self.selections.is_empty() {
            bail!(
                "scenario \"{}\" matches no option of this project ({} name(s) unresolved)",
                self.name,
                unresolved.len()
            );
        }
        let mut files = referenced_files(ximod);
        for (k, v) in &self.file_states {
            files.insert(k.clone(), *v);
        }
        Ok((selections, files, unresolved))
    }
}

/// `<root>/fomod/scenarios`, reusing the existing spelling of `fomod`.
pub fn scenarios_dir(root: &Path) -> PathBuf {
    crate::xml::patch::locate_fomod_files(root).dir.join(SCENARIOS_DIR)
}

/// Turn a scenario name into its file name (without the extension).
fn scenario_stem(name: &str) -> String {
    let stem = super::translate::sanitize_file_stem(name);
    if stem.is_empty() { "scenario".to_string() } else { stem }
}

/// Path of the scenario file for `name`.
pub fn scenario_path(root: &Path, name: &str) -> PathBuf {
    scenarios_dir(root).join(format!("{}.json", scenario_stem(name)))
}

/// Names of the scenarios saved under the root, sorted.
pub fn list_scenarios(root: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(scenarios_dir(root))
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.is_file() && p.extension().is_some_and(|e| e.eq_ignore_ascii_case("json")))
                .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// Write the scenario as JSON under `fomod/scenarios/`. Returns its path.
pub fn save_scenario(root: &Path, scenario: &Scenario) -> Result<PathBuf> {
    let path = scenario_path(root, &scenario.name);
    let dir = scenarios_dir(root);
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let mut json = serde_json::to_string_pretty(scenario).context("serializing the scenario")?;
    json.push('\n');
    std::fs::write(&path, json).with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
}

/// Read a scenario: `name` is a saved scenario's name, or the path of a
/// `.json` file.
pub fn load_scenario(root: &Path, name: &str) -> Result<Scenario> {
    let as_path = Path::new(name);
    let path = if as_path.extension().is_some_and(|e| e.eq_ignore_ascii_case("json")) && as_path.is_file() {
        as_path.to_path_buf()
    } else {
        scenario_path(root, name)
    };
    let text = std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let text = text.strip_prefix('\u{FEFF}').unwrap_or(&text);
    let mut scenario: Scenario =
        serde_json::from_str(text).with_context(|| format!("{} is not a valid scenario", path.display()))?;
    if scenario.name.trim().is_empty() {
        scenario.name = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| name.to_string());
    }
    Ok(scenario)
}

/// Remove a saved scenario.
pub fn delete_scenario(root: &Path, name: &str) -> Result<()> {
    let path = scenario_path(root, name);
    std::fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))
}

// --------------------------------------------------------------------------- //
// Final install tree
// --------------------------------------------------------------------------- //

/// Upper bound on the files expanded into a tree (folder sources are walked
/// on disk; a texture mod can hold far more than this).
pub const MAX_TREE_ENTRIES: usize = 20_000;

/// Upper bound on the Bethesda archives (`.bsa` / `.ba2`) opened to list
/// their contents under their node in one tree.
pub const MAX_TREE_ARCHIVES: usize = 50;

/// Where a file of the tree comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallSource {
    /// The option / list that declares the install entry.
    pub loc: RefLoc,
    /// The source path as authored (a folder for expanded entries).
    pub source: String,
    pub priority: u32,
    /// Declaration rank of the entry in the install list.
    pub seq: usize,
    /// Size on disk of the installed file (0 when missing).
    pub size: u64,
}

/// A node of the final file tree: a folder (children) or a file (source).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TreeNode {
    /// Component name, in the case of the first entry that wrote it.
    pub name: String,
    /// Normalized full path (lowercased, forward slashes).
    pub path: String,
    /// File size, or the total of the children for a folder.
    pub size: u64,
    pub children: BTreeMap<String, TreeNode>,
    /// The entry that ends up installed at this path (files only).
    pub source: Option<InstallSource>,
    /// Entries overwritten at this path, in losing order.
    pub overwritten: Vec<InstallSource>,
    /// The winning source does not exist on disk.
    pub missing: bool,
    /// A file *inside* a Bethesda archive, listed under the archive's node
    /// for information: it has no source of its own, its `name` is the path
    /// inside the archive and its `size` the unpacked size. Not counted in
    /// the totals (the archive is, by its on-disk size).
    pub in_archive: bool,
}

impl TreeNode {
    /// True for a folder node.
    pub fn is_dir(&self) -> bool {
        self.source.is_none() && !self.in_archive
    }
}

/// The final install tree with its totals.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstallTree {
    /// The Data folder (its `name` is empty).
    pub root: TreeNode,
    /// Sum of the sizes of the installed files.
    pub total_size: u64,
    /// Number of installed files (after overwrites).
    pub files: usize,
    /// Number of overwritten entries.
    pub overwrites: usize,
    /// Set when [`MAX_TREE_ENTRIES`] was reached while expanding folders.
    pub truncated: bool,
    /// Source paths that do not exist under the root.
    pub missing: Vec<String>,
    /// Unpacked bytes of the files inside the installed Bethesda archives
    /// (informative: `total_size` counts the archives themselves).
    pub archived_bytes: u64,
}

/// Normalize a source path for disk access, keeping the original case.
fn norm_keep_case(raw: &str) -> String {
    raw.trim().replace('\\', "/").trim_start_matches("./").to_string()
}

/// Join a destination and a relative name keeping the case (display form).
fn join_keep_case(destination: &str, name: &str) -> String {
    let dest = destination.trim().replace('\\', "/");
    let dest = dest.trim_matches('/');
    let name = name.replace('\\', "/");
    let name = name.trim_start_matches("./").trim_start_matches('/');
    if dest.is_empty() {
        name.to_string()
    } else {
        format!("{dest}/{name}")
    }
}

/// Build the final file tree of an install list, resolving folder sources
/// under `root`. When two entries land on the same path the higher priority
/// wins; on a tie the later declaration wins, like the mod managers.
pub fn build_install_tree(root: &Path, entries: &[InstallEntry]) -> InstallTree {
    // Flat list of (normalized path, display path, source) in resolution order.
    let mut order: Vec<&InstallEntry> = entries.iter().collect();
    order.sort_by(|a, b| a.file.priority.cmp(&b.file.priority).then(a.seq.cmp(&b.seq)));

    let mut tree = InstallTree::default();
    let mut count = 0usize;
    // Winner per normalized path, with the losers.
    let mut by_path: BTreeMap<String, (String, InstallSource, Vec<InstallSource>, bool)> = BTreeMap::new();
    // Normalized path -> on-disk archive of the winner, for the `.bsa` /
    // `.ba2` files (the last placement is the winner, like `by_path`).
    let mut archives: BTreeMap<String, PathBuf> = BTreeMap::new();

    'outer: for e in order {
        let source = e.file.source.trim();
        if source.is_empty() {
            continue;
        }
        let mut place = |norm: String, display: String, src: InstallSource, missing: bool| {
            match by_path.get_mut(&norm) {
                Some((_, winner, losers, was_missing)) => {
                    // Later (higher priority or later declaration) wins.
                    let old = std::mem::replace(winner, src);
                    losers.push(old);
                    *was_missing = missing;
                }
                None => {
                    by_path.insert(norm, (display, src, Vec::new(), missing));
                }
            }
        };
        let abs = root.join(norm_keep_case(source));
        match e.file.file_type {
            FileType::File => {
                let size = std::fs::metadata(&abs).ok().filter(|m| m.is_file()).map(|m| m.len());
                if size.is_none() {
                    tree.missing.push(source.to_string());
                }
                let name = basename(source);
                if is_bethesda_archive(name) && size.is_some() {
                    archives.insert(join_norm(&e.file.destination, name), abs.clone());
                }
                place(
                    join_norm(&e.file.destination, name),
                    join_keep_case(&e.file.destination, name),
                    InstallSource {
                        loc: e.loc.clone(),
                        source: source.to_string(),
                        priority: e.file.priority,
                        seq: e.seq,
                        size: size.unwrap_or(0),
                    },
                    size.is_none(),
                );
                count += 1;
            }
            FileType::Folder => {
                if !abs.is_dir() {
                    tree.missing.push(source.to_string());
                    continue;
                }
                for entry in WalkDir::new(&abs).into_iter().filter_map(Result::ok) {
                    if !entry.file_type().is_file() {
                        continue;
                    }
                    if count >= MAX_TREE_ENTRIES {
                        tree.truncated = true;
                        break 'outer;
                    }
                    let Ok(rel) = entry.path().strip_prefix(&abs) else {
                        continue;
                    };
                    let rel = rel.to_string_lossy().replace('\\', "/");
                    let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                    if is_bethesda_archive(entry.path()) {
                        archives.insert(join_norm(&e.file.destination, &rel), entry.path().to_path_buf());
                    }
                    place(
                        join_norm(&e.file.destination, &rel),
                        join_keep_case(&e.file.destination, &rel),
                        InstallSource {
                            loc: e.loc.clone(),
                            source: source.to_string(),
                            priority: e.file.priority,
                            seq: e.seq,
                            size,
                        },
                        false,
                    );
                    count += 1;
                }
            }
        }
        if count >= MAX_TREE_ENTRIES {
            tree.truncated = true;
            break;
        }
    }

    // Grow the tree from the flat map.
    let mut listings: HashMap<PathBuf, Option<ArchiveListing>> = HashMap::new();
    for (norm, (display, winner, losers, missing)) in by_path {
        let comps: Vec<&str> = display.split('/').filter(|c| !c.is_empty()).collect();
        let norm_comps: Vec<&str> = norm.split('/').filter(|c| !c.is_empty()).collect();
        if comps.is_empty() || comps.len() != norm_comps.len() {
            continue;
        }
        let mut node = &mut tree.root;
        let mut path = String::new();
        for (i, (c, nc)) in comps.iter().zip(norm_comps.iter()).enumerate() {
            if !path.is_empty() {
                path.push('/');
            }
            path.push_str(nc);
            let last = i + 1 == comps.len();
            let child = node.children.entry((*nc).to_string()).or_insert_with(|| TreeNode {
                name: (*c).to_string(),
                path: path.clone(),
                ..Default::default()
            });
            if last {
                child.size = winner.size;
                child.missing = missing;
                tree.overwrites += losers.len();
                child.overwritten = losers.clone();
                child.source = Some(winner.clone());
                tree.files += 1;
                tree.total_size += winner.size;
                if let Some(abs) = archives.get(&norm) {
                    tree.archived_bytes += attach_archive_entries(child, abs, &mut listings);
                }
            }
            node = child;
        }
    }
    fold_sizes(&mut tree.root);
    tree
}

/// List the archive at `abs` (once per tree, at most [`MAX_TREE_ARCHIVES`]
/// archives) and hang its entries under `node` as informative children.
/// Returns the unpacked bytes added. Unreadable archives get no children.
fn attach_archive_entries(
    node: &mut TreeNode,
    abs: &Path,
    listings: &mut HashMap<PathBuf, Option<ArchiveListing>>,
) -> u64 {
    if !listings.contains_key(abs) {
        let listing = if listings.len() < MAX_TREE_ARCHIVES {
            list_archive(abs).ok()
        } else {
            None
        };
        listings.insert(abs.to_path_buf(), listing);
    }
    let Some(Some(listing)) = listings.get(abs) else {
        return 0;
    };
    let mut bytes = 0;
    for entry in &listing.entries {
        bytes += entry.size;
        node.children.insert(
            entry.path.clone(),
            TreeNode {
                name: entry.path.clone(),
                path: format!("{}::{}", node.path, entry.path),
                size: entry.size,
                in_archive: true,
                ..Default::default()
            },
        );
    }
    bytes
}

/// Folder sizes = sum of their children.
fn fold_sizes(node: &mut TreeNode) -> u64 {
    if node.source.is_some() {
        return node.size;
    }
    let mut total = 0;
    for child in node.children.values_mut() {
        total += fold_sizes(child);
    }
    node.size = total;
    total
}

/// Human-readable size with one decimal (locale-neutral: `1.5 MB`).
pub fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

// --------------------------------------------------------------------------- //
// Unreachable analysis
// --------------------------------------------------------------------------- //

/// A part of the installer that no selection can ever reach.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unreachable {
    /// A step whose visibility condition tests a flag value no earlier
    /// option sets.
    Step { index: usize },
    /// An option that is NotUsable by default and whose patterns leading to
    /// a usable type test flag values no option of this or an earlier step
    /// sets.
    Option { step: usize, group: usize, plugin: usize },
    /// A conditional file set whose condition tests a flag value no option
    /// sets.
    ConditionalSet { index: usize },
}

/// Every `flag=value` pair set by an option of `steps`.
fn flag_values_of(steps: &[super::Step]) -> HashSet<(String, String)> {
    let mut out = HashSet::new();
    for s in steps {
        for g in &s.plugin_groups {
            for p in &g.plugins {
                for cf in &p.condition_flags {
                    out.insert((cf.name.clone(), cf.value.clone()));
                }
            }
        }
    }
    out
}

/// Whether one dependency can hold at all: file, game and mod-manager
/// dependencies and empty flag values (an unset flag) always can; a flag
/// value must be set somewhere in `setters`.
fn dep_possible(dep: &Dependency, setters: &HashSet<(String, String)>) -> bool {
    if !dep.is_flag() || dep.value.is_empty() {
        return true;
    }
    setters.contains(&(dep.name.clone(), dep.value.clone()))
}

/// Whether a condition group can hold at all (empty → always). Deliberately
/// conservative: an `Or` group is possible as soon as one branch is, an
/// `And` group only when every item is, nested groups recursively — so a
/// finding is certain, never a false alarm.
fn group_possible(group: &DependencyGroup, setters: &HashSet<(String, String)>) -> bool {
    if group.items.is_empty() {
        return true;
    }
    let item_ok = |item: &DependencyItem| match item {
        DependencyItem::Leaf(d) => dep_possible(d, setters),
        DependencyItem::Group(g) => group_possible(g, setters),
    };
    match group.operator {
        LogicalOperator::Or => group.items.iter().any(item_ok),
        LogicalOperator::And => group.items.iter().all(item_ok),
    }
}

/// Steps, options and conditional sets that can never be reached. Only the
/// flag rule is applied (file dependencies are assumed satisfiable), so
/// every finding is certain.
pub fn unreachable(ximod: &Ximod) -> Vec<Unreachable> {
    let mut out = Vec::new();
    for (si, step) in ximod.steps.iter().enumerate() {
        let before = flag_values_of(&ximod.steps[..si]);
        if !group_possible(&step.visibility, &before) {
            out.push(Unreachable::Step { index: si });
            continue;
        }
        let upto = flag_values_of(&ximod.steps[..=si]);
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            for (pi, plugin) in group.plugins.iter().enumerate() {
                if plugin.default_type != PluginType::NotUsable {
                    continue;
                }
                let usable: Vec<_> = plugin
                    .dependency_patterns
                    .iter()
                    .filter(|pat| PluginType::from_str(&pat.pattern_type) != PluginType::NotUsable)
                    .collect();
                if usable.is_empty() {
                    continue; // NotUsable by design, not a reachability bug
                }
                if usable.iter().all(|pat| !group_possible(&pat.condition, &upto)) {
                    out.push(Unreachable::Option {
                        step: si,
                        group: gi,
                        plugin: pi,
                    });
                }
            }
        }
    }
    let all = flag_values_of(&ximod.steps);
    for (ci, cfs) in ximod.conditional_files.iter().enumerate() {
        if !group_possible(&cfs.condition, &all) {
            out.push(Unreachable::ConditionalSet { index: ci });
        }
    }
    out
}

// --------------------------------------------------------------------------- //
// Install size per option
// --------------------------------------------------------------------------- //

/// Size of what an option (or a file list) installs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeInfo {
    pub bytes: u64,
    pub files: usize,
    /// Entries whose source does not exist under the root.
    pub missing: usize,
}

impl SizeInfo {
    fn add(&mut self, other: SizeInfo) {
        self.bytes += other.bytes;
        self.files += other.files;
        self.missing += other.missing;
    }
}

/// Per-process memo of the size of one source entry, keyed by root, source
/// and the modification time of the entry itself (a touched folder or file
/// is re-measured; a validation re-run on an untouched mod costs one `stat`
/// per entry).
type SizeKey = (PathBuf, String, Option<SystemTime>);
static SIZE_CACHE: Mutex<Option<HashMap<SizeKey, SizeInfo>>> = Mutex::new(None);

/// Size of one install entry under `root` (recursive for folders).
pub fn entry_size(root: &Path, file: &InstallFile) -> SizeInfo {
    let source = norm_keep_case(&file.source);
    if source.is_empty() {
        return SizeInfo::default();
    }
    let abs = root.join(&source);
    let mtime = std::fs::metadata(&abs).ok().and_then(|m| m.modified().ok());
    let key = (root.to_path_buf(), source.clone(), mtime);
    if let Ok(guard) = SIZE_CACHE.lock()
        && let Some(hit) = guard.as_ref().and_then(|m| m.get(&key))
    {
        return *hit;
    }
    let info = match file.file_type {
        FileType::File => match std::fs::metadata(&abs) {
            Ok(m) if m.is_file() => SizeInfo {
                bytes: m.len(),
                files: 1,
                missing: 0,
            },
            _ => SizeInfo {
                bytes: 0,
                files: 0,
                missing: 1,
            },
        },
        FileType::Folder => {
            if !abs.is_dir() {
                SizeInfo {
                    bytes: 0,
                    files: 0,
                    missing: 1,
                }
            } else {
                let mut info = SizeInfo::default();
                for entry in WalkDir::new(&abs).into_iter().filter_map(Result::ok) {
                    if entry.file_type().is_file() {
                        info.files += 1;
                        info.bytes += entry.metadata().map(|m| m.len()).unwrap_or(0);
                    }
                }
                info
            }
        }
    };
    if let Ok(mut guard) = SIZE_CACHE.lock() {
        let map = guard.get_or_insert_with(HashMap::new);
        // Keep the memo small: a project rarely references thousands of entries.
        if map.len() > 50_000 {
            map.clear();
        }
        map.insert(key, info);
    }
    info
}

/// Size of a list of install entries.
pub fn files_size(root: &Path, files: &[InstallFile]) -> SizeInfo {
    let mut total = SizeInfo::default();
    for f in files {
        total.add(entry_size(root, f));
    }
    total
}

/// Size of what an option installs: its files summed, folders walked.
pub fn option_size(root: &Path, plugin: &Plugin) -> SizeInfo {
    files_size(root, &plugin.files)
}

/// Sizes of every option, the required files and each conditional set of
/// a project (what the validation worker computes for the UI).
#[derive(Debug, Clone, Default)]
pub struct SizeReport {
    pub root: PathBuf,
    pub option_sizes: HashMap<SelKey, SizeInfo>,
    pub required_size: SizeInfo,
    pub conditional_sizes: Vec<SizeInfo>,
}

impl SizeReport {
    /// Measure every file list of the project under `root`.
    pub fn compute(ximod: &Ximod, root: &Path) -> Self {
        let mut option_sizes = HashMap::new();
        for (si, step) in ximod.steps.iter().enumerate() {
            for (gi, group) in step.plugin_groups.iter().enumerate() {
                for (pi, plugin) in group.plugins.iter().enumerate() {
                    option_sizes.insert((si, gi, pi), option_size(root, plugin));
                }
            }
        }
        Self {
            root: root.to_path_buf(),
            option_sizes,
            required_size: files_size(root, &ximod.required_files),
            conditional_sizes: ximod
                .conditional_files
                .iter()
                .map(|c| files_size(root, &c.files))
                .collect(),
        }
    }

    /// Sum over every option.
    pub fn options_total(&self) -> SizeInfo {
        let mut total = SizeInfo::default();
        for s in self.option_sizes.values() {
            total.add(*s);
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        ConditionFlag, ConditionalFileSet, Dependency, DependencyPattern, InstallFile, Plugin, PluginGroup, Step, Ximod,
    };

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod_simulate_{}_{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn touch(root: &Path, rel: &str, bytes: usize) {
        let p = root.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(p, vec![b'x'; bytes]).unwrap();
    }

    /// Step 1 chooses a texture resolution (sets flag `res`), step 2 is
    /// visible only when `res = 4K`, and a conditional set installs an extra
    /// file when `res = 4K`.
    fn sample() -> Ximod {
        let mut m = Ximod::new("Sample");
        m.required_files.push(InstallFile::new_file("base.esp"));
        let mut g = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        let mut a = Plugin::new("2K");
        a.condition_flags.push(ConditionFlag::new("res", "2K"));
        a.files.push(InstallFile::new_file("tex2k/a.dds"));
        let mut b = Plugin::new("4K");
        b.condition_flags.push(ConditionFlag::new("res", "4K"));
        b.files.push(InstallFile::new_file("tex4k/b.dds"));
        g.plugins.push(a);
        g.plugins.push(b);
        let mut s1 = Step::new("Resolution");
        s1.plugin_groups.push(g);
        let mut s2 = Step::new("Extras (4K only)");
        s2.visibility.push_leaf(Dependency::new_flag("res", "4K"));
        m.steps.push(s1);
        m.steps.push(s2);
        let mut cfs = ConditionalFileSet::new();
        cfs.condition.push_leaf(Dependency::new_flag("res", "4K"));
        cfs.files.push(InstallFile::new_file("patch4k.esp"));
        m.conditional_files.push(cfs);
        m
    }

    #[test]
    fn default_picks_first_and_hides_conditional_step() {
        let m = sample();
        let files = BTreeMap::new();
        let sel = default_selections(&m, &files);
        assert_eq!(sel.get(&(0, 0, 0)), Some(&true));
        assert_eq!(sel.get(&(0, 0, 1)), Some(&false));
        let ev = evaluate(&m, &sel, &files);
        assert_eq!(ev.flags.get("res").map(String::as_str), Some("2K"));
        assert_eq!(ev.visible_steps, vec![0]);
        let install: Vec<String> = compute_install(&m, &sel, &files)
            .into_iter()
            .map(|f| f.source)
            .collect();
        assert!(install.iter().any(|d| d == "base.esp"));
        assert!(install.iter().any(|d| d.contains("a.dds")));
        assert!(!install.iter().any(|d| d == "patch4k.esp"));
    }

    #[test]
    fn selecting_4k_reveals_step_and_conditional_file() {
        let m = sample();
        let files = BTreeMap::new();
        let mut sel = default_selections(&m, &files);
        sel.insert((0, 0, 0), false);
        sel.insert((0, 0, 1), true);
        let ev = evaluate(&m, &sel, &files);
        assert_eq!(ev.flags.get("res").map(String::as_str), Some("4K"));
        assert_eq!(ev.visible_steps, vec![0, 1]);
        let entries = compute_entries(&m, &sel, &files);
        let sources: Vec<&str> = entries.iter().map(|e| e.file.source.as_str()).collect();
        assert!(sources.contains(&"patch4k.esp"));
        assert!(sources.iter().any(|d| d.contains("b.dds")));
        assert!(entries.iter().any(|e| e.loc == RefLoc::ConditionalSet { index: 1 }));
        assert!(
            entries
                .iter()
                .any(|e| matches!(&e.loc, RefLoc::Plugin { plugin, .. } if plugin == "4K"))
        );
    }

    #[test]
    fn file_dependency_assumption_is_honoured() {
        let mut m = Ximod::new("F");
        let mut s = Step::new("needs file");
        s.visibility.push_leaf(Dependency::new_file("Skyrim.esm", "Active"));
        m.steps.push(s);
        let sel = HashMap::new();
        let mut files = referenced_files(&m);
        assert_eq!(files.get("Skyrim.esm"), Some(&FileState::Active));
        assert_eq!(evaluate(&m, &sel, &files).visible_steps, vec![0]);
        files.insert("Skyrim.esm".to_string(), FileState::Missing);
        assert!(evaluate(&m, &sel, &files).visible_steps.is_empty());
    }

    /// Lot N: `And(Or(a, b), c)` and its reachability.
    #[test]
    fn nested_group_evaluation_and_reachability() {
        let mut inner = DependencyGroup::new(LogicalOperator::Or);
        inner.push_leaf(Dependency::new_flag("DataVersion", "Standard"));
        inner.push_leaf(Dependency::new_flag("DataVersion", "Undelayed"));
        let mut group = DependencyGroup::new(LogicalOperator::And);
        group.push_group(inner.clone());
        group.push_leaf(Dependency::new_flag("DataPreset", "Aurelia"));
        group.push_leaf(Dependency::new_game("1.6"));
        let files = BTreeMap::new();
        let flags = |pairs: &[(&str, &str)]| -> HashMap<String, String> {
            pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
        };
        assert!(group_satisfied(
            &group,
            &flags(&[("DataVersion", "Undelayed"), ("DataPreset", "Aurelia")]),
            &files
        ));
        assert!(group_satisfied(
            &group,
            &flags(&[("DataVersion", "Standard"), ("DataPreset", "Aurelia")]),
            &files
        ));
        // The Or branch fails.
        assert!(!group_satisfied(
            &group,
            &flags(&[("DataVersion", "Other"), ("DataPreset", "Aurelia")]),
            &files
        ));
        // The And leaf fails.
        assert!(!group_satisfied(
            &group,
            &flags(&[("DataVersion", "Standard"), ("DataPreset", "Nope")]),
            &files
        ));
        // Empty groups hold, including nested ones.
        let mut with_empty = DependencyGroup::new(LogicalOperator::And);
        with_empty.push_group(DependencyGroup::new(LogicalOperator::Or));
        assert!(group_satisfied(&DependencyGroup::default(), &flags(&[]), &files));
        assert!(group_satisfied(&with_empty, &flags(&[]), &files));
        // The flat helper agrees with the flat case.
        let (op, leaves) = inner.flat().unwrap();
        let leaves: Vec<Dependency> = leaves.into_iter().cloned().collect();
        let f = flags(&[("DataVersion", "Standard")]);
        assert_eq!(
            deps_satisfied(op, &leaves, &f, &files),
            group_satisfied(&inner, &f, &files)
        );

        // Reachability: a nested Or is possible when one branch is; an And
        // with an impossible nested group is not.
        let setters: HashSet<(String, String)> = [("DataVersion".to_string(), "Standard".to_string())]
            .into_iter()
            .collect();
        assert!(group_possible(&inner, &setters));
        assert!(!group_possible(&group, &setters), "DataPreset is never set");
        let mut m = sample();
        let mut s = Step::new("Nested");
        let mut or = DependencyGroup::new(LogicalOperator::Or);
        or.push_leaf(Dependency::new_flag("res", "8K"));
        or.push_leaf(Dependency::new_flag("res", "2K"));
        s.visibility.push_group(or);
        s.visibility.push_leaf(Dependency::new_flag("res", "4K"));
        m.steps.push(s);
        assert!(unreachable(&m).is_empty());
        let mut dead = Step::new("Dead");
        let mut or = DependencyGroup::new(LogicalOperator::Or);
        or.push_leaf(Dependency::new_flag("res", "8K"));
        or.push_leaf(Dependency::new_flag("res", "16K"));
        dead.visibility.push_group(or);
        m.steps.push(dead);
        assert_eq!(unreachable(&m), vec![Unreachable::Step { index: 3 }]);
        // Referenced files come from every depth.
        let mut deep = ConditionalFileSet::new();
        let mut g = DependencyGroup::new(LogicalOperator::Or);
        g.push_leaf(Dependency::new_file("Deep.esm", "Missing"));
        deep.condition.push_group(g);
        m.conditional_files.push(deep);
        assert!(referenced_files(&m).contains_key("Deep.esm"));
    }

    #[test]
    fn group_validity_and_pick_one() {
        let g = PluginGroup::new("G", SelectionType::SelectExactlyOne);
        let mut g = g;
        g.plugins.push(Plugin::new("A"));
        g.plugins.push(Plugin::new("B"));
        let mut sel = HashMap::new();
        sel.insert((0, 0, 0), true);
        assert!(group_valid(&g, &sel, 0, 0));
        sel.insert((0, 0, 1), true);
        assert!(!group_valid(&g, &sel, 0, 0));
        assert_eq!(pick_one(&[PluginType::NotUsable, PluginType::Recommended]), Some(1));
        assert_eq!(pick_one(&[PluginType::NotUsable]), Some(0));
        assert_eq!(pick_one(&[]), None);
    }

    #[test]
    fn scenario_round_trip_and_unresolved_names() {
        let m = sample();
        let root = scratch("scenario");
        let files = referenced_files(&m);
        let mut sel = default_selections(&m, &files);
        sel.insert((0, 0, 0), false);
        sel.insert((0, 0, 1), true);
        let mut fs = files.clone();
        fs.insert("Skyrim.esm".into(), FileState::Missing);
        let sc = Scenario::from_state(&m, &sel, &fs, "Ultra: 4K?");
        assert_eq!(sc.selections.len(), 1);
        assert_eq!(sc.selections[0].plugin, "4K");

        let path = save_scenario(&root, &sc).unwrap();
        assert!(path.ends_with("fomod/scenarios/Ultra_ 4K_.json"), "{}", path.display());
        assert_eq!(list_scenarios(&root), vec!["Ultra_ 4K_".to_string()]);
        let back = load_scenario(&root, "Ultra_ 4K_").unwrap();
        assert_eq!(back, sc);
        // By file path too.
        assert_eq!(load_scenario(&root, &path.to_string_lossy()).unwrap(), sc);

        let (sel2, fs2, unresolved) = back.apply(&m).unwrap();
        assert!(unresolved.is_empty());
        assert_eq!(sel2.get(&(0, 0, 1)), Some(&true));
        assert_eq!(sel2.get(&(0, 0, 0)), Some(&false));
        assert_eq!(fs2.get("Skyrim.esm"), Some(&FileState::Missing));

        // The option is renamed: its selection no longer resolves.
        let mut renamed = m.clone();
        renamed.steps[0].plugin_groups[0].plugins[1].name = "8K".into();
        let mut sc2 = sc.clone();
        sc2.selections.push(SelectionSpec {
            step: "Resolution".into(),
            group: "Resolution".into(),
            plugin: "2K".into(),
        });
        let (_, _, unresolved) = sc2.apply(&renamed).unwrap();
        assert_eq!(unresolved, vec!["Resolution / Resolution / 4K".to_string()]);
        // Nothing resolves at all: an error.
        assert!(sc.apply(&renamed).is_err());
        assert!(sc.apply(&Ximod::new("Other")).is_err());
        // Reordering does not matter: the scenario is by name.
        let mut reordered = m.clone();
        reordered.steps[0].plugin_groups[0].plugins.reverse();
        let (sel3, _, unresolved) = sc.apply(&reordered).unwrap();
        assert!(unresolved.is_empty());
        assert_eq!(sel3.get(&(0, 0, 0)), Some(&true));

        delete_scenario(&root, "Ultra_ 4K_").unwrap();
        assert!(list_scenarios(&root).is_empty());
        assert!(load_scenario(&root, "nope").is_err());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn install_tree_applies_priority_and_tie_rules() {
        let root = scratch("tree");
        touch(&root, "a/skse.ini", 10);
        touch(&root, "b/skse.ini", 20);
        touch(&root, "c/skse.ini", 30);
        touch(&root, "tex/one.dds", 100);
        touch(&root, "tex/sub/two.dds", 200);
        touch(&root, "tex2/sub/two.dds", 50);

        let loc = |p: &str| RefLoc::Plugin {
            step: 1,
            group: 1,
            plugin: p.into(),
        };
        let mut f_a = InstallFile::new_file("a/skse.ini");
        f_a.destination = "SKSE/Plugins".into();
        let mut f_b = InstallFile::new_file("b\\skse.ini");
        f_b.destination = "skse\\plugins".into();
        let mut f_c = InstallFile::new_file("c/skse.ini");
        f_c.destination = "SKSE/Plugins".into();
        f_c.priority = 5;
        let mut d_tex = InstallFile::new_folder("tex");
        d_tex.destination = "textures".into();
        let mut d_tex2 = InstallFile::new_folder("tex2");
        d_tex2.destination = "textures".into();
        let missing = InstallFile::new_file("gone.esp");
        let mut entries = vec![
            // c has the highest priority but is declared first: it must win.
            InstallEntry {
                loc: loc("C"),
                file: f_c,
                seq: 0,
            },
            InstallEntry {
                loc: loc("A"),
                file: f_a,
                seq: 1,
            },
            InstallEntry {
                loc: loc("B"),
                file: f_b,
                seq: 2,
            },
            InstallEntry {
                loc: RefLoc::RequiredFiles,
                file: d_tex,
                seq: 3,
            },
            InstallEntry {
                loc: loc("T2"),
                file: d_tex2,
                seq: 4,
            },
            InstallEntry {
                loc: loc("M"),
                file: missing,
                seq: 5,
            },
        ];
        let tree = build_install_tree(&root, &entries);
        assert!(!tree.truncated);
        assert_eq!(tree.missing, vec!["gone.esp".to_string()]);
        let ini = &tree.root.children["skse"].children["plugins"].children["skse.ini"];
        assert_eq!(ini.name, "skse.ini");
        assert_eq!(ini.size, 30);
        assert!(matches!(&ini.source, Some(s) if s.source == "c/skse.ini"));
        // Losers in losing order: A (priority 0, first) then B (tie, later wins over A).
        let losers: Vec<&str> = ini.overwritten.iter().map(|s| s.source.as_str()).collect();
        assert_eq!(losers, vec!["a/skse.ini", "b\\skse.ini"]);
        // Folder expansion with a later tie: tex2's two.dds overwrites tex's.
        let two = &tree.root.children["textures"].children["sub"].children["two.dds"];
        assert_eq!(two.size, 50);
        assert!(matches!(&two.source, Some(s) if s.source == "tex2"));
        assert_eq!(two.overwritten.len(), 1);
        let textures = &tree.root.children["textures"];
        assert_eq!(textures.size, 150);
        assert!(textures.is_dir());
        // Totals: skse.ini (30) + one.dds (100) + two.dds (50) + gone (0).
        assert_eq!(tree.total_size, 180);
        assert_eq!(tree.files, 4);
        assert_eq!(tree.overwrites, 3);
        assert!(tree.root.children["gone.esp"].missing);
        assert_eq!(tree.root.size, 180);

        // Same entries, given in summary order (by priority then destination).
        entries.sort_by(|a, b| a.file.priority.cmp(&b.file.priority).then(a.seq.cmp(&b.seq)));
        assert_eq!(build_install_tree(&root, &entries), tree);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Installed BSA / BA2 archives list their contents under their node
    /// (unpacked sizes, `in_archive`), counted apart from the install size.
    #[test]
    fn install_tree_lists_archive_contents_without_double_counting() {
        use crate::archive::{Ba2Game, Ba2Options, package_ba2};
        let root = scratch("tree_ba2");
        touch(&root, "pack/textures/a.dds", 300);
        touch(&root, "pack/meshes/b.nif", 44);
        touch(&root, "opt/Mod - Main.esp", 10);
        std::fs::create_dir_all(root.join("opt")).unwrap();
        let archive = root.join("opt/Mod - Main.ba2");
        package_ba2(
            &root.join("pack"),
            &archive,
            &Ba2Options {
                game: Ba2Game::Fallout4,
                general: true,
            },
        )
        .unwrap();
        let archive_size = std::fs::metadata(&archive).unwrap().len();
        touch(&root, "opt/Bad.bsa", 5);

        let entries = vec![
            InstallEntry {
                loc: RefLoc::RequiredFiles,
                file: InstallFile::new_file("opt/Mod - Main.ba2"),
                seq: 0,
            },
            InstallEntry {
                loc: RefLoc::RequiredFiles,
                file: InstallFile::new_file("opt/Mod - Main.esp"),
                seq: 1,
            },
            InstallEntry {
                loc: RefLoc::RequiredFiles,
                file: InstallFile::new_file("opt/Bad.bsa"),
                seq: 2,
            },
        ];
        let tree = build_install_tree(&root, &entries);
        assert_eq!(tree.files, 3);
        assert_eq!(
            tree.total_size,
            archive_size + 10 + 5,
            "archive contents are not counted"
        );
        assert_eq!(tree.archived_bytes, 344);
        let node = &tree.root.children["mod - main.ba2"];
        assert!(!node.is_dir() && !node.in_archive);
        assert_eq!(node.size, archive_size);
        assert_eq!(node.children.len(), 2);
        let inner = &node.children["textures/a.dds"];
        assert!(inner.in_archive && !inner.is_dir());
        assert_eq!(inner.size, 300);
        assert_eq!(inner.path, "mod - main.ba2::textures/a.dds");
        assert!(inner.source.is_none());
        assert_eq!(node.children["meshes/b.nif"].size, 44);
        // An unreadable archive is a plain file.
        assert!(tree.root.children["bad.bsa"].children.is_empty());
        assert_eq!(tree.root.size, archive_size + 15);

        // The same archive expanded from a folder source lands under the
        // destination folder, with its contents.
        let mut folder = InstallFile::new_folder("opt");
        folder.destination = "Data".into();
        let tree = build_install_tree(
            &root,
            &[InstallEntry {
                loc: RefLoc::RequiredFiles,
                file: folder,
                seq: 0,
            }],
        );
        let node = &tree.root.children["data"].children["mod - main.ba2"];
        assert_eq!(node.children.len(), 2);
        assert_eq!(tree.archived_bytes, 344);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn install_tree_truncates_huge_folders() {
        let root = scratch("trunc");
        for i in 0..40 {
            touch(&root, &format!("big/f{i}.bin"), 1);
        }
        let entries: Vec<InstallEntry> = (0..600)
            .map(|seq| InstallEntry {
                loc: RefLoc::RequiredFiles,
                file: InstallFile::new_folder("big"),
                seq,
            })
            .collect();
        // 600 × 40 = 24 000 candidates > MAX_TREE_ENTRIES.
        let tree = build_install_tree(&root, &entries);
        assert!(tree.truncated);
        assert_eq!(tree.files, 40);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn format_size_units() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(1023), "1023 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(5 * 1024 * 1024), "5.0 MB");
        assert_eq!(format_size(3 * 1024 * 1024 * 1024 + 512 * 1024 * 1024), "3.5 GB");
        assert_eq!(format_size(5000 * 1024 * 1024 * 1024), "5000.0 GB");
    }

    #[test]
    fn unreachable_detection() {
        let mut m = sample();
        // Reachable: step 2 needs res=4K, which the 4K option sets.
        assert!(unreachable(&m).is_empty());

        // A step testing a value nobody sets.
        let mut s3 = Step::new("Never");
        s3.visibility.push_leaf(Dependency::new_flag("res", "8K"));
        m.steps.push(s3);
        // An Or with one satisfiable branch stays reachable.
        let mut s4 = Step::new("Maybe");
        s4.visibility.operator = LogicalOperator::Or;
        s4.visibility.push_leaf(Dependency::new_flag("res", "8K"));
        s4.visibility.push_leaf(Dependency::new_flag("res", "2K"));
        m.steps.push(s4);
        // A step testing a flag set only in a LATER step.
        let mut s5 = Step::new("Too early");
        s5.visibility.push_leaf(Dependency::new_flag("late", "yes"));
        m.steps.push(s5);
        let mut s6 = Step::new("Setter");
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        let mut p = Plugin::new("Late");
        p.condition_flags.push(ConditionFlag::new("late", "yes"));
        // An option NotUsable by default, usable only when res=8K: unreachable.
        let mut dead = Plugin::new("Dead");
        dead.default_type = PluginType::NotUsable;
        dead.dependency_patterns.push(DependencyPattern {
            pattern_type: "Optional".into(),
            condition: DependencyGroup::from_leaves(LogicalOperator::And, vec![Dependency::new_flag("res", "8K")]),
        });
        // Same shape but on a value that exists: reachable.
        let mut alive = Plugin::new("Alive");
        alive.default_type = PluginType::NotUsable;
        alive.dependency_patterns.push(DependencyPattern {
            pattern_type: "Optional".into(),
            condition: DependencyGroup::from_leaves(
                LogicalOperator::And,
                vec![
                    Dependency::new_flag("res", "4K"),
                    Dependency::new_file("X.esm", "Active"),
                ],
            ),
        });
        // NotUsable with no usable pattern: by design, not reported.
        let mut by_design = Plugin::new("Design");
        by_design.default_type = PluginType::NotUsable;
        g.plugins.push(p);
        g.plugins.push(dead);
        g.plugins.push(alive);
        g.plugins.push(by_design);
        s6.plugin_groups.push(g);
        m.steps.push(s6);
        // Conditional sets: one on an unknown value, one with an empty value.
        let mut bad = ConditionalFileSet::new();
        bad.condition.push_leaf(Dependency::new_flag("res", "16K"));
        m.conditional_files.push(bad);
        let mut unset = ConditionalFileSet::new();
        unset.condition.push_leaf(Dependency::new_flag("res", ""));
        m.conditional_files.push(unset);

        let found = unreachable(&m);
        assert_eq!(
            found,
            vec![
                Unreachable::Step { index: 2 },
                Unreachable::Step { index: 4 },
                Unreachable::Option {
                    step: 5,
                    group: 0,
                    plugin: 1
                },
                Unreachable::ConditionalSet { index: 1 },
            ]
        );
    }

    #[test]
    fn option_size_counts_files_folders_and_missing() {
        let root = scratch("size");
        touch(&root, "Plugin.esp", 100);
        touch(&root, "meshes/a.nif", 10);
        touch(&root, "meshes/deep/b.nif", 20);
        let mut p = Plugin::new("P");
        p.files.push(InstallFile::new_file("Plugin.esp"));
        p.files.push(InstallFile::new_folder("meshes"));
        p.files.push(InstallFile::new_file("missing.esp"));
        p.files.push(InstallFile::new_folder("nowhere"));
        let s = option_size(&root, &p);
        assert_eq!(
            s,
            SizeInfo {
                bytes: 130,
                files: 3,
                missing: 2
            }
        );
        // The memo is keyed by mtime: a grown file is re-measured.
        let s2 = option_size(&root, &p);
        assert_eq!(s2, s);
        let mut m = Ximod::new("M");
        m.required_files.push(InstallFile::new_file("Plugin.esp"));
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p);
        let mut st = Step::new("S");
        st.plugin_groups.push(g);
        m.steps.push(st);
        let mut c = ConditionalFileSet::new();
        c.files.push(InstallFile::new_folder("meshes"));
        m.conditional_files.push(c);
        let report = SizeReport::compute(&m, &root);
        assert_eq!(report.option_sizes[&(0, 0, 0)], s);
        assert_eq!(report.required_size.bytes, 100);
        assert_eq!(report.conditional_sizes[0].files, 2);
        assert_eq!(report.options_total().bytes, 130);
        let _ = std::fs::remove_dir_all(&root);
    }
}
