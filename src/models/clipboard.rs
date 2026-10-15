//! Copy / cut / paste of installer nodes (2.2.0).
//!
//! A step, a group or an option can be copied from one project and pasted
//! into the same or another open project (or another XIMOD window, through
//! the system clipboard). The payload reuses the template body
//! ([`TemplateBody`]) and its `serde` derives; the text written to the
//! system clipboard starts with a marker line so that ordinary text is never
//! mistaken for a node.
//!
//! Pasting follows the selection: a step goes after the selected step, a
//! group into the selected step (after the selected group), an option into
//! the selected group (after the selected option). Two things are reported
//! because they often need attention after a paste from another mod: files
//! the pasted node uses that do not exist in this mod's folder, and flags it
//! tests that nothing in this project sets.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::file_refs::{RefKind, norm};
use super::templates::TemplateBody;
use super::{FileType, Plugin, Ximod};

/// First line of the clipboard text.
pub const MARKER: &str = "XIMOD-ARCHITECT-CLIPBOARD 1";

/// A node of the project tree, by position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRef {
    Step(usize),
    Group(usize, usize),
    Plugin(usize, usize, usize),
}

/// What the clipboard holds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipItem {
    /// The copied node.
    pub body: TemplateBody,
    /// Mod folder of the project it was copied from, when it had one.
    #[serde(default)]
    pub source_root: Option<PathBuf>,
}

impl ClipItem {
    /// Copy the node `at` of `ximod`.
    pub fn copy(ximod: &Ximod, at: NodeRef, source_root: Option<&Path>) -> Option<Self> {
        let body = match at {
            NodeRef::Step(si) => TemplateBody::Step(ximod.steps.get(si)?.clone()),
            NodeRef::Group(si, gi) => TemplateBody::Group(ximod.steps.get(si)?.plugin_groups.get(gi)?.clone()),
            NodeRef::Plugin(si, gi, pi) => {
                TemplateBody::Plugin(ximod.steps.get(si)?.plugin_groups.get(gi)?.plugins.get(pi)?.clone())
            }
        };
        Some(Self {
            body,
            source_root: source_root.map(Path::to_path_buf),
        })
    }

    /// Display name of the copied node.
    pub fn name(&self) -> &str {
        match &self.body {
            TemplateBody::Step(s) => &s.name,
            TemplateBody::Group(g) => &g.name,
            TemplateBody::Plugin(p) => &p.name,
        }
    }

    /// Text for the system clipboard: the marker line, then JSON.
    pub fn to_text(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap_or_default();
        format!("{MARKER}\n{json}")
    }

    /// Read clipboard text written by [`to_text`](Self::to_text); `None`
    /// for anything else.
    pub fn from_text(text: &str) -> Option<Self> {
        let text = text.trim_start_matches('\u{feff}');
        let (first, rest) = text.split_once('\n')?;
        if first.trim() != MARKER {
            return None;
        }
        serde_json::from_str(rest).ok()
    }

    /// The options of the copied node.
    fn plugins(&self) -> Vec<&Plugin> {
        match &self.body {
            TemplateBody::Step(s) => s.plugin_groups.iter().flat_map(|g| g.plugins.iter()).collect(),
            TemplateBody::Group(g) => g.plugins.iter().collect(),
            TemplateBody::Plugin(p) => vec![p],
        }
    }

    /// Every file, folder and image reference of the copied node.
    pub fn references(&self) -> Vec<(RefKind, String)> {
        let mut out = Vec::new();
        for p in self.plugins() {
            if let Some(img) = p.image_path.as_deref().filter(|s| !s.trim().is_empty()) {
                out.push((RefKind::Image, img.to_string()));
            }
            for f in &p.files {
                let kind = if f.file_type == FileType::Folder {
                    RefKind::Folder
                } else {
                    RefKind::File
                };
                out.push((kind, f.source.clone()));
            }
        }
        out
    }

    /// Flag names the copied node tests (step visibility and option
    /// dependency patterns, at every depth).
    pub fn tested_flags(&self) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        if let TemplateBody::Step(s) = &self.body {
            out.extend(s.visibility.leaves().filter(|d| d.is_flag()).map(|d| d.name.clone()));
        }
        for p in self.plugins() {
            for pat in &p.dependency_patterns {
                out.extend(pat.condition.leaves().filter(|d| d.is_flag()).map(|d| d.name.clone()));
            }
        }
        out
    }
}

/// Why a paste could not be placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasteError {
    /// A group needs a selected step.
    NeedStep,
    /// An option needs a selected group (or a step that has one).
    NeedGroup,
}

/// Insert a copy of `item` relative to `anchor` (the selected node) and
/// return where it went.
pub fn paste(ximod: &mut Ximod, anchor: Option<NodeRef>, item: &ClipItem) -> Result<NodeRef, PasteError> {
    let step_of = |a: NodeRef| match a {
        NodeRef::Step(si) | NodeRef::Group(si, _) | NodeRef::Plugin(si, _, _) => si,
    };
    match &item.body {
        TemplateBody::Step(step) => {
            let at = anchor
                .map(|a| step_of(a) + 1)
                .unwrap_or(ximod.steps.len())
                .min(ximod.steps.len());
            ximod.steps.insert(at, step.clone());
            Ok(NodeRef::Step(at))
        }
        TemplateBody::Group(group) => {
            let anchor = anchor.ok_or(PasteError::NeedStep)?;
            let si = step_of(anchor);
            let step = ximod.steps.get_mut(si).ok_or(PasteError::NeedStep)?;
            let at = match anchor {
                NodeRef::Step(_) => step.plugin_groups.len(),
                NodeRef::Group(_, gi) | NodeRef::Plugin(_, gi, _) => (gi + 1).min(step.plugin_groups.len()),
            };
            step.plugin_groups.insert(at, group.clone());
            Ok(NodeRef::Group(si, at))
        }
        TemplateBody::Plugin(plugin) => {
            let anchor = anchor.ok_or(PasteError::NeedGroup)?;
            let (si, gi, at) = match anchor {
                NodeRef::Step(si) => {
                    let n = ximod.steps.get(si).map_or(0, |s| s.plugin_groups.len());
                    if n == 0 {
                        return Err(PasteError::NeedGroup);
                    }
                    (si, n - 1, usize::MAX)
                }
                NodeRef::Group(si, gi) => (si, gi, usize::MAX),
                NodeRef::Plugin(si, gi, pi) => (si, gi, pi + 1),
            };
            let group = ximod
                .steps
                .get_mut(si)
                .and_then(|s| s.plugin_groups.get_mut(gi))
                .ok_or(PasteError::NeedGroup)?;
            let at = at.min(group.plugins.len());
            group.plugins.insert(at, plugin.clone());
            Ok(NodeRef::Plugin(si, gi, at))
        }
    }
}

/// Remove the node `at` (the "cut" half of cut-and-paste).
pub fn remove(ximod: &mut Ximod, at: NodeRef) -> bool {
    match at {
        NodeRef::Step(si) if si < ximod.steps.len() => {
            ximod.steps.remove(si);
            true
        }
        NodeRef::Group(si, gi) => match ximod.steps.get_mut(si) {
            Some(s) if gi < s.plugin_groups.len() => {
                s.plugin_groups.remove(gi);
                true
            }
            _ => false,
        },
        NodeRef::Plugin(si, gi, pi) => match ximod.steps.get_mut(si).and_then(|s| s.plugin_groups.get_mut(gi)) {
            Some(g) if pi < g.plugins.len() => {
                g.plugins.remove(pi);
                true
            }
            _ => false,
        },
        _ => false,
    }
}

/// What deserves a look after a paste.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PasteReport {
    /// Distinct references of the pasted node missing from `root`.
    pub missing_files: usize,
    /// Flags the pasted node tests that no option of the project sets.
    pub unset_flags: Vec<String>,
}

/// Check the pasted `item` against the project it landed in.
pub fn report(ximod: &Ximod, root: Option<&Path>, item: &ClipItem) -> PasteReport {
    let missing_files = match root {
        Some(root) => {
            let mut seen = BTreeSet::new();
            item.references()
                .into_iter()
                .filter(|(kind, raw)| {
                    let rel = raw.trim().replace('\\', "/");
                    let abs = root.join(rel.trim_start_matches("./"));
                    let exists = if kind.is_folder() { abs.is_dir() } else { abs.is_file() };
                    !exists && seen.insert(norm(raw))
                })
                .count()
        }
        None => 0,
    };
    let set: BTreeSet<&str> = ximod
        .steps
        .iter()
        .flat_map(|s| s.plugin_groups.iter())
        .flat_map(|g| g.plugins.iter())
        .flat_map(|p| p.condition_flags.iter())
        .map(|f| f.name.as_str())
        .collect();
    let unset_flags = item
        .tested_flags()
        .into_iter()
        .filter(|f| !set.contains(f.as_str()))
        .collect();
    PasteReport {
        missing_files,
        unset_flags,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        ConditionFlag, Dependency, DependencyGroup, DependencyItem, DependencyPattern, InstallFile, PluginGroup,
        SelectionType, Step,
    };

    fn sample() -> Ximod {
        let mut x = Ximod::new("Src");
        for s in 0..2 {
            let mut step = Step::new(format!("S{s}"));
            for g in 0..2 {
                let mut group = PluginGroup::new(format!("S{s}G{g}"), SelectionType::SelectAny);
                for p in 0..2 {
                    group.plugins.push(Plugin::new(format!("S{s}G{g}P{p}")));
                }
                step.plugin_groups.push(group);
            }
            x.steps.push(step);
        }
        x
    }

    #[test]
    fn text_round_trip_and_foreign_text() {
        let x = sample();
        let item = ClipItem::copy(&x, NodeRef::Group(1, 0), Some(Path::new("/mods/a"))).unwrap();
        let back = ClipItem::from_text(&item.to_text()).unwrap();
        assert_eq!(back.name(), "S1G0");
        assert_eq!(back.source_root.as_deref(), Some(Path::new("/mods/a")));
        assert!(ClipItem::from_text("just some text").is_none());
        assert!(ClipItem::from_text("XIMOD-ARCHITECT-CLIPBOARD 1\nnot json").is_none());
        assert!(ClipItem::copy(&x, NodeRef::Plugin(5, 0, 0), None).is_none());
    }

    #[test]
    fn paste_follows_the_selection() {
        let src = sample();
        let mut x = sample();
        let step = ClipItem::copy(&src, NodeRef::Step(0), None).unwrap();
        let group = ClipItem::copy(&src, NodeRef::Group(0, 1), None).unwrap();
        let opt = ClipItem::copy(&src, NodeRef::Plugin(1, 1, 1), None).unwrap();

        // Step: after the anchor's step, or at the end without a selection.
        assert_eq!(
            paste(&mut x, Some(NodeRef::Plugin(0, 1, 0)), &step),
            Ok(NodeRef::Step(1))
        );
        assert_eq!(paste(&mut x, None, &step), Ok(NodeRef::Step(3)));
        assert_eq!(x.steps.len(), 4);

        // Group: end of a selected step, or after the selected group.
        assert_eq!(paste(&mut x, Some(NodeRef::Step(0)), &group), Ok(NodeRef::Group(0, 2)));
        assert_eq!(
            paste(&mut x, Some(NodeRef::Group(0, 0)), &group),
            Ok(NodeRef::Group(0, 1))
        );
        assert_eq!(x.steps[0].plugin_groups[1].name, "S0G1");
        assert_eq!(paste(&mut x, None, &group), Err(PasteError::NeedStep));

        // Option: end of the group, after the option, last group of a step.
        assert_eq!(
            paste(&mut x, Some(NodeRef::Group(2, 0)), &opt),
            Ok(NodeRef::Plugin(2, 0, 2))
        );
        assert_eq!(
            paste(&mut x, Some(NodeRef::Plugin(2, 0, 0)), &opt),
            Ok(NodeRef::Plugin(2, 0, 1))
        );
        let last = x.steps[2].plugin_groups.len() - 1;
        assert_eq!(
            paste(&mut x, Some(NodeRef::Step(2)), &opt),
            Ok(NodeRef::Plugin(2, last, 2))
        );
        x.steps.push(Step::new("Empty"));
        let empty = x.steps.len() - 1;
        assert_eq!(
            paste(&mut x, Some(NodeRef::Step(empty)), &opt),
            Err(PasteError::NeedGroup)
        );
        assert_eq!(paste(&mut x, None, &opt), Err(PasteError::NeedGroup));

        // Cut = copy + remove.
        assert!(remove(&mut x, NodeRef::Plugin(2, 0, 1)));
        assert!(!remove(&mut x, NodeRef::Plugin(9, 0, 0)));
        assert!(remove(&mut x, NodeRef::Step(empty)));
    }

    #[test]
    fn report_counts_missing_files_and_unset_flags() {
        let root = std::env::temp_dir().join(format!("ximod_clip_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("present")).unwrap();
        std::fs::write(root.join("present/a.esp"), b"x").unwrap();

        let mut p = Plugin::new("Opt");
        p.files.push(InstallFile::new_file("present\\a.esp"));
        p.files.push(InstallFile::new_file("absent\\b.esp"));
        p.files.push(InstallFile::new_file("ABSENT/b.esp")); // same file, counted once
        p.image_path = Some("fomod/pic.png".into());
        let mut cond = DependencyGroup::default();
        cond.items
            .push(DependencyItem::Leaf(Dependency::new_flag("feat_a", "on")));
        cond.items
            .push(DependencyItem::Leaf(Dependency::new_flag("shared", "on")));
        let mut pat = DependencyPattern::new();
        pat.condition = cond;
        p.dependency_patterns.push(pat);
        let item = ClipItem {
            body: TemplateBody::Plugin(p),
            source_root: None,
        };

        let mut target = sample();
        target.steps[0].plugin_groups[0].plugins[0]
            .condition_flags
            .push(ConditionFlag::new("shared", "on"));
        let r = report(&target, Some(&root), &item);
        assert_eq!(r.missing_files, 2);
        assert_eq!(r.unset_flags, vec!["feat_a".to_string()]);
        assert_eq!(report(&target, None, &item).missing_files, 0);
        let _ = std::fs::remove_dir_all(&root);
    }
}
