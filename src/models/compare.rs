//! FOMOD version comparison (V2 roadmap, priority 10).
//!
//! Given two versions of a project (e.g. the previous release and the working
//! copy), summarize what changed — steps and options added or removed, files
//! added or removed, and scalar metadata changes — to help write a changelog and
//! to catch accidental drops.
//!
//! This is a structural diff over the in-memory [`Ximod`] model, not a text diff
//! of the XML. It is i18n-free; the UI renders the [`ProjectDiff`].
//!
//! Matching strategy: items are paired by **name** at each level (step → group →
//! option), and install files by their `(source, destination)` pair. Names are
//! assumed reasonably unique within their parent (the usual case for a FOMOD); on
//! duplicate names the last occurrence wins.

use std::collections::BTreeMap;

use super::{InstallFile, Ximod};

/// A single added/removed structural item, identified by a human-readable path
/// such as `"Step «Textures» / Option «4K»"`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffItem {
    /// Breadcrumb-style identifier of the item within the project.
    pub path: String,
}

impl DiffItem {
    fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

/// The result of comparing two projects (`from` → `to`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectDiff {
    /// Steps present in `to` but not in `from`.
    pub steps_added: Vec<DiffItem>,
    /// Steps present in `from` but not in `to`.
    pub steps_removed: Vec<DiffItem>,
    /// Options (plugins) added (within matched steps/groups, or via a new group).
    pub options_added: Vec<DiffItem>,
    /// Options (plugins) removed.
    pub options_removed: Vec<DiffItem>,
    /// Install files added (by source→destination).
    pub files_added: Vec<DiffItem>,
    /// Install files removed.
    pub files_removed: Vec<DiffItem>,
    /// Scalar metadata changes (name, author, version…), as `"field: a → b"`.
    pub meta_changes: Vec<String>,
}

impl ProjectDiff {
    /// True when nothing changed.
    pub fn is_empty(&self) -> bool {
        self.steps_added.is_empty()
            && self.steps_removed.is_empty()
            && self.options_added.is_empty()
            && self.options_removed.is_empty()
            && self.files_added.is_empty()
            && self.files_removed.is_empty()
            && self.meta_changes.is_empty()
    }

    /// Total number of changes across all categories.
    pub fn total(&self) -> usize {
        self.steps_added.len()
            + self.steps_removed.len()
            + self.options_added.len()
            + self.options_removed.len()
            + self.files_added.len()
            + self.files_removed.len()
            + self.meta_changes.len()
    }
}

/// Normalized key for an install file (case-insensitive, forward slashes).
fn file_key(f: &InstallFile) -> String {
    format!(
        "{} \u{2192} {}",
        f.source.replace('\\', "/").to_lowercase(),
        f.destination.replace('\\', "/").to_lowercase()
    )
}

/// Compute the structural diff `from` → `to`.
pub fn diff_projects(from: &Ximod, to: &Ximod) -> ProjectDiff {
    let mut d = ProjectDiff::default();

    // ---- scalar metadata ----
    scalar(&mut d, "name", &from.name, &to.name);
    scalar(&mut d, "author", &from.author, &to.author);
    scalar(&mut d, "version", &from.version, &to.version);
    scalar(&mut d, "game", &from.game, &to.game);
    scalar(&mut d, "url", &from.url, &to.url);
    scalar(
        &mut d,
        "category",
        &format!("{:?}", from.category),
        &format!("{:?}", to.category),
    );
    scalar(
        &mut d,
        "header image",
        from.header_image.as_deref().unwrap_or(""),
        to.header_image.as_deref().unwrap_or(""),
    );
    if from.description != to.description {
        d.meta_changes.push("description changed".to_string());
    }
    scalar(
        &mut d,
        "title position",
        from.title_position.as_deref().unwrap_or(""),
        to.title_position.as_deref().unwrap_or(""),
    );
    scalar(
        &mut d,
        "title colour",
        from.title_colour.as_deref().unwrap_or(""),
        to.title_colour.as_deref().unwrap_or(""),
    );
    let deps_count = |x: &Ximod| x.module_dependencies.as_ref().map(|m| m.leaf_count()).unwrap_or(0);
    scalar(
        &mut d,
        "mod requirements",
        &deps_count(from).to_string(),
        &deps_count(to).to_string(),
    );

    // ---- steps / groups / options / files ----
    let from_steps = index_by(&from.steps, |s| s.name.clone());
    let to_steps = index_by(&to.steps, |s| s.name.clone());

    for (name, s) in &to_steps {
        if !from_steps.contains_key(name) {
            d.steps_added.push(DiffItem::new(format!("Step «{name}»")));
            // All options of a brand-new step count as added options.
            for g in &s.plugin_groups {
                for p in &g.plugins {
                    d.options_added
                        .push(DiffItem::new(format!("Step «{name}» / Option «{}»", p.name)));
                }
            }
        }
    }
    for (name, s) in &from_steps {
        if !to_steps.contains_key(name) {
            d.steps_removed.push(DiffItem::new(format!("Step «{name}»")));
            for g in &s.plugin_groups {
                for p in &g.plugins {
                    d.options_removed
                        .push(DiffItem::new(format!("Step «{name}» / Option «{}»", p.name)));
                }
            }
        }
    }

    // Matched steps: compare their options and files.
    for (sname, sa) in &from_steps {
        let Some(sb) = to_steps.get(sname) else { continue };
        let pa = options_of(sa);
        let pb = options_of(sb);
        for pname in pb.keys() {
            if !pa.contains_key(pname) {
                d.options_added
                    .push(DiffItem::new(format!("Step «{sname}» / Option «{pname}»")));
            }
        }
        for pname in pa.keys() {
            if !pb.contains_key(pname) {
                d.options_removed
                    .push(DiffItem::new(format!("Step «{sname}» / Option «{pname}»")));
            }
        }
        // Matched options: compare files.
        for (pname, pa_plugin) in &pa {
            let Some(pb_plugin) = pb.get(pname) else { continue };
            diff_files(
                &mut d,
                &format!("Step «{sname}» / Option «{pname}»"),
                &pa_plugin.files,
                &pb_plugin.files,
            );
        }
    }

    // ---- required files ----
    diff_files(&mut d, "Required files", &from.required_files, &to.required_files);

    // ---- conditional sets (paired by index) ----
    let n = from.conditional_files.len().max(to.conditional_files.len());
    for i in 0..n {
        let fa = from.conditional_files.get(i).map(|s| &s.files[..]).unwrap_or(&[]);
        let fb = to.conditional_files.get(i).map(|s| &s.files[..]).unwrap_or(&[]);
        diff_files(&mut d, &format!("Conditional set {}", i + 1), fa, fb);
    }

    d
}

fn scalar(d: &mut ProjectDiff, label: &str, a: &str, b: &str) {
    if a != b {
        d.meta_changes.push(format!("{label}: «{a}» \u{2192} «{b}»"));
    }
}

/// Build a name → item map (last occurrence wins on duplicates).
fn index_by<T, F: Fn(&T) -> String>(items: &[T], key: F) -> BTreeMap<String, &T> {
    let mut m = BTreeMap::new();
    for it in items {
        m.insert(key(it), it);
    }
    m
}

/// Flatten a step's options (plugins across all its groups), by option name.
fn options_of(step: &super::Step) -> BTreeMap<String, &super::Plugin> {
    let mut m = BTreeMap::new();
    for g in &step.plugin_groups {
        for p in &g.plugins {
            m.insert(p.name.clone(), p);
        }
    }
    m
}

/// Compare two file lists under `ctx`, appending added/removed entries.
fn diff_files(d: &mut ProjectDiff, ctx: &str, from: &[InstallFile], to: &[InstallFile]) {
    let fa: BTreeMap<String, ()> = from.iter().map(|f| (file_key(f), ())).collect();
    let fb: BTreeMap<String, ()> = to.iter().map(|f| (file_key(f), ())).collect();
    for k in fb.keys() {
        if !fa.contains_key(k) {
            d.files_added.push(DiffItem::new(format!("{ctx} / {k}")));
        }
    }
    for k in fa.keys() {
        if !fb.contains_key(k) {
            d.files_removed.push(DiffItem::new(format!("{ctx} / {k}")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{InstallFile, Plugin, PluginGroup, SelectionType, Step, Ximod};

    #[test]
    fn identical_projects_have_empty_diff() {
        let a = Ximod::new("Mod");
        let b = Ximod::new("Mod");
        assert!(diff_projects(&a, &b).is_empty());
    }

    #[test]
    fn detects_meta_and_structure_changes() {
        let mut a = Ximod::new("Mod");
        a.version = "1.0".into();
        let mut step = Step::new("Textures");
        let mut g = PluginGroup::new("Res", SelectionType::SelectExactlyOne);
        let mut p2k = Plugin::new("2K");
        p2k.files.push(InstallFile::new_folder("2k"));
        g.plugins.push(p2k);
        g.plugins.push(Plugin::new("4K"));
        step.plugin_groups.push(g);
        a.steps.push(step);

        // b: bump version, drop the "4K" option, add an "8K" option, change a file.
        let mut b = a.clone();
        b.version = "1.1".into();
        {
            let g = &mut b.steps[0].plugin_groups[0];
            g.plugins.retain(|p| p.name != "4K");
            g.plugins.push(Plugin::new("8K"));
            g.plugins[0].files[0] = InstallFile::new_folder("2k_hd");
        }

        let d = diff_projects(&a, &b);
        assert!(d.meta_changes.iter().any(|m| m.contains("version")));
        assert!(d.options_added.iter().any(|i| i.path.contains("8K")));
        assert!(d.options_removed.iter().any(|i| i.path.contains("4K")));
        assert!(d.files_added.iter().any(|i| i.path.contains("2k_hd")));
        assert!(
            d.files_removed.iter().any(|i| i.path.contains("2k "))
                || d.files_removed.iter().any(|i| i.path.contains("2k \u{2192}"))
        );
        assert!(!d.is_empty());
        assert!(d.total() >= 4);
    }

    #[test]
    fn added_and_removed_steps() {
        let mut a = Ximod::new("M");
        a.steps.push(Step::new("Keep"));
        a.steps.push(Step::new("Old"));
        let mut b = Ximod::new("M");
        b.steps.push(Step::new("Keep"));
        b.steps.push(Step::new("New"));
        let d = diff_projects(&a, &b);
        assert!(d.steps_added.iter().any(|i| i.path.contains("New")));
        assert!(d.steps_removed.iter().any(|i| i.path.contains("Old")));
    }
}
