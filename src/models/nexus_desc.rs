//! Nexus Mods description generator (V2 lot F3, feature 8).
//!
//! Writes a mod page description from the project: title and description,
//! the requirements (masters of the shipped plugins that neither the mod
//! nor the game provides, plus the file dependencies the author declared),
//! the installation options (steps → groups → options), one line on the
//! FOMOD installer and, when a previous version is given, a changelog from
//! [`super::compare::diff_projects`].
//!
//! Two formats: Nexus BBCode and Markdown (for GitHub / other sites). The
//! module is i18n-free: section titles arrive in [`DescLabels`], filled by
//! the UI from its locale (or in English by the CLI).

use std::collections::BTreeSet;
use std::path::Path;

use super::compare::diff_projects;
use super::plugin_checks::{collect_plugins, masters_to_add};
use super::plugin_header::EslLimits;
use super::translate::{TranslationDoc, apply_to_model};
use super::{SelectionType, Ximod};
use crate::games::GamesData;

/// Output markup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DescFormat {
    #[default]
    BBCode,
    Markdown,
}

impl DescFormat {
    /// File extension of a saved description.
    pub fn extension(self) -> &'static str {
        match self {
            Self::BBCode => "txt",
            Self::Markdown => "md",
        }
    }
}

/// What to include.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescOptions {
    pub format: DescFormat,
    pub include_requirements: bool,
    pub include_options: bool,
    pub include_changelog: bool,
    pub include_install: bool,
    /// Free line appended under the description (e.g. "French translation").
    pub language_note: Option<String>,
}

impl Default for DescOptions {
    fn default() -> Self {
        Self {
            format: DescFormat::BBCode,
            include_requirements: true,
            include_options: true,
            include_changelog: true,
            include_install: true,
            language_note: None,
        }
    }
}

/// Localised section titles and words, supplied by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescLabels {
    pub requirements: String,
    pub options: String,
    pub changelog: String,
    pub install: String,
    /// Sentence on the FOMOD installer (the "Installation" body).
    pub install_text: String,
    pub requires: String,
    pub step: String,
    pub added: String,
    pub removed: String,
    pub changed: String,
}

impl DescLabels {
    /// English labels (CLI, tests).
    pub fn english() -> Self {
        Self {
            requirements: "Requirements".into(),
            options: "Installation options".into(),
            changelog: "Changelog".into(),
            install: "Installation".into(),
            install_text: "This mod ships a FOMOD installer: install it with a mod manager (Vortex, Mod Organizer 2) \
                           and pick your options in the installer."
                .into(),
            requires: "Requires".into(),
            step: "Step".into(),
            added: "Added".into(),
            removed: "Removed".into(),
            changed: "Changed".into(),
        }
    }
}

/// Markup helpers for the two formats.
struct Writer {
    format: DescFormat,
    out: String,
}

impl Writer {
    fn title(&mut self, text: &str) {
        match self.format {
            DescFormat::BBCode => self.out.push_str(&format!("[size=5][b]{text}[/b][/size]\n")),
            DescFormat::Markdown => self.out.push_str(&format!("# {text}\n")),
        }
    }

    fn section(&mut self, text: &str) {
        self.out.push('\n');
        match self.format {
            DescFormat::BBCode => self.out.push_str(&format!("[size=4][b]{text}[/b][/size]\n")),
            DescFormat::Markdown => self.out.push_str(&format!("## {text}\n")),
        }
    }

    fn para(&mut self, text: &str) {
        if !text.trim().is_empty() {
            self.out.push_str(text.trim());
            self.out.push('\n');
        }
    }

    fn italic(&mut self, text: &str) {
        match self.format {
            DescFormat::BBCode => self.out.push_str(&format!("[i]{text}[/i]\n")),
            DescFormat::Markdown => self.out.push_str(&format!("*{text}*\n")),
        }
    }

    fn link(&mut self, text: &str, url: &str) {
        match self.format {
            DescFormat::BBCode => self.out.push_str(&format!("[url={url}]{text}[/url]\n")),
            DescFormat::Markdown => self.out.push_str(&format!("[{text}]({url})\n")),
        }
    }

    /// A (possibly nested) bullet list: `(depth, text)` items.
    fn list(&mut self, items: &[(usize, String)]) {
        if items.is_empty() {
            return;
        }
        match self.format {
            DescFormat::Markdown => {
                for (depth, text) in items {
                    self.out.push_str(&"  ".repeat(*depth));
                    self.out.push_str("- ");
                    self.out.push_str(text);
                    self.out.push('\n');
                }
            }
            DescFormat::BBCode => {
                // Nested [list] blocks: open one per depth increase, close on decrease.
                let mut depth = 0usize;
                self.out.push_str("[list]\n");
                for (d, text) in items {
                    while depth < *d {
                        self.out.push_str("[list]\n");
                        depth += 1;
                    }
                    while depth > *d {
                        self.out.push_str("[/list]\n");
                        depth -= 1;
                    }
                    self.out.push_str(&format!("[*]{text}\n"));
                }
                while depth > 0 {
                    self.out.push_str("[/list]\n");
                    depth -= 1;
                }
                self.out.push_str("[/list]\n");
            }
        }
    }
}

/// First sentence of a description (up to the first `.`, `!`, `?` or line
/// break), with literal `\n` sequences treated as line breaks.
pub fn first_sentence(text: &str) -> String {
    let text = text.replace("\\r\\n", "\n").replace("\\n", "\n").replace('\r', "\n");
    let line = text.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("");
    let mut end = line.len();
    for (i, c) in line.char_indices() {
        if matches!(c, '.' | '!' | '?') {
            let next = line[i + c.len_utf8()..].chars().next();
            if next.is_none_or(char::is_whitespace) {
                end = i + c.len_utf8();
                break;
            }
        }
    }
    line[..end].trim().to_string()
}

/// Name of a selection type as shown in the options list.
fn selection_word(t: SelectionType) -> &'static str {
    match t {
        SelectionType::SelectExactlyOne => "exactly one",
        SelectionType::SelectAtMostOne => "at most one",
        SelectionType::SelectAtLeastOne => "at least one",
        SelectionType::SelectAny => "any",
        SelectionType::SelectAll => "all",
    }
}

/// The requirements of the mod: masters of the shipped plugins that neither
/// the mod nor the base game provides (read under `root`), plus the files
/// the author declared as Active/Exists dependencies (module and options).
pub fn requirements(ximod: &Ximod, root: Option<&Path>, games: &GamesData) -> Vec<String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut out: Vec<String> = Vec::new();
    let mut push = |name: &str| {
        let name = name.trim();
        if !name.is_empty() && seen.insert(name.to_ascii_lowercase()) {
            out.push(name.to_string());
        }
    };
    if let Some(root) = root {
        let base: Vec<String> = games.base_masters_for(&ximod.game).to_vec();
        let limits = EslLimits::for_game(games, &ximod.game);
        for info in collect_plugins(ximod, root, &base, limits) {
            if let Some(h) = &info.header {
                for m in masters_to_add(ximod, h, &base) {
                    push(&m);
                }
            }
        }
    }
    let present = |d: &super::Dependency| {
        d.dep_type.eq_ignore_ascii_case("file")
            && (d.value.eq_ignore_ascii_case("Active") || d.value.eq_ignore_ascii_case("Exists"))
            && !super::plugin_checks::project_provides(ximod, d.name.trim())
    };
    // Leaves at every depth of the mod requirements and option patterns.
    if let Some(m) = &ximod.module_dependencies {
        for d in m.leaves().filter(|d| present(d)) {
            push(&d.name);
        }
    }
    for step in &ximod.steps {
        for g in &step.plugin_groups {
            for p in &g.plugins {
                for pat in &p.dependency_patterns {
                    for d in pat.condition.leaves().filter(|d| present(d)) {
                        push(&d.name);
                    }
                }
            }
        }
    }
    out
}

/// Generate the description.
pub fn generate(
    ximod: &Ximod,
    root: Option<&Path>,
    games: &GamesData,
    previous: Option<&Ximod>,
    opts: &DescOptions,
    labels: &DescLabels,
) -> String {
    let mut w = Writer {
        format: opts.format,
        out: String::new(),
    };

    // ---- title + description ----
    let name = if ximod.name.trim().is_empty() {
        "Untitled mod".to_string()
    } else {
        ximod.name.trim().to_string()
    };
    let title = if ximod.version.trim().is_empty() {
        name
    } else {
        format!("{name} {}", ximod.version.trim())
    };
    w.title(&title);
    if !ximod.url.trim().is_empty() {
        w.link(ximod.url.trim(), ximod.url.trim());
    }
    w.para(&ximod.description);
    if let Some(note) = opts.language_note.as_deref().filter(|n| !n.trim().is_empty()) {
        w.italic(note.trim());
    }

    // ---- requirements ----
    if opts.include_requirements {
        let reqs = requirements(ximod, root, games);
        if !reqs.is_empty() {
            w.section(&labels.requirements);
            let items: Vec<(usize, String)> = reqs.iter().map(|r| (0, format!("{}: {r}", labels.requires))).collect();
            w.list(&items);
        }
    }

    // ---- installation options ----
    if opts.include_options && !ximod.steps.is_empty() {
        w.section(&labels.options);
        let mut items: Vec<(usize, String)> = Vec::new();
        for (si, step) in ximod.steps.iter().enumerate() {
            let sname = if step.name.trim().is_empty() {
                format!("{} {}", labels.step, si + 1)
            } else {
                format!("{} {}: {}", labels.step, si + 1, step.name.trim())
            };
            items.push((0, sname));
            for g in &step.plugin_groups {
                items.push((1, format!("{} ({})", g.name.trim(), selection_word(g.selection_type))));
                for p in &g.plugins {
                    let sentence = first_sentence(&p.description);
                    items.push((
                        2,
                        if sentence.is_empty() {
                            p.name.trim().to_string()
                        } else {
                            format!("{} — {sentence}", p.name.trim())
                        },
                    ));
                }
            }
        }
        w.list(&items);
    }

    // ---- installation ----
    if opts.include_install {
        w.section(&labels.install);
        w.para(&labels.install_text);
    }

    // ---- changelog ----
    if opts.include_changelog
        && let Some(prev) = previous
    {
        let d = diff_projects(prev, ximod);
        if !d.is_empty() {
            w.section(&labels.changelog);
            let mut items: Vec<(usize, String)> = Vec::new();
            let added: Vec<&str> = d
                .steps_added
                .iter()
                .chain(&d.options_added)
                .chain(&d.files_added)
                .map(|i| i.path.as_str())
                .collect();
            let removed: Vec<&str> = d
                .steps_removed
                .iter()
                .chain(&d.options_removed)
                .chain(&d.files_removed)
                .map(|i| i.path.as_str())
                .collect();
            if !added.is_empty() {
                items.push((0, labels.added.clone()));
                items.extend(added.iter().map(|p| (1, (*p).to_string())));
            }
            if !removed.is_empty() {
                items.push((0, labels.removed.clone()));
                items.extend(removed.iter().map(|p| (1, (*p).to_string())));
            }
            if !d.meta_changes.is_empty() {
                items.push((0, labels.changed.clone()));
                items.extend(d.meta_changes.iter().map(|m| (1, m.clone())));
            }
            w.list(&items);
        }
    }

    w.out
}

/// [`generate`] for a translated copy of the model: `doc` is applied to a
/// clone of `ximod` first, and the changelog (if any) compares `previous`
/// to the source model, since the previous version is not translated.
#[allow(clippy::too_many_arguments)]
pub fn generate_translated(
    doc: &TranslationDoc,
    ximod: &Ximod,
    root: Option<&Path>,
    games: &GamesData,
    previous: Option<&Ximod>,
    opts: &DescOptions,
    labels: &DescLabels,
) -> String {
    let translated = apply_to_model(ximod, doc);
    generate(&translated, root, games, previous, opts, labels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::translate::{TField, TStatus, TUnit};
    use crate::models::{
        Dependency, DependencyGroup, DependencyPattern, InstallFile, LogicalOperator, Plugin, PluginGroup, Step,
    };

    fn sample() -> Ximod {
        let mut m = Ximod::new("Aurelia");
        m.version = "1.2".into();
        m.url = "https://example.org/aurelia".into();
        m.description = "A lovely mod.\nMore text.".into();
        m.game = "skyrimSpecialEdition".into();
        let mut step = Step::new("Textures");
        let mut g = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        let mut p2k = Plugin::new("2K");
        p2k.description = "Balanced quality. Fits most rigs.".into();
        p2k.files.push(InstallFile::new_folder("2k"));
        let mut p4k = Plugin::new("4K");
        p4k.description = "For high-end GPUs!".into();
        p4k.dependency_patterns.push(DependencyPattern {
            pattern_type: "Recommended".into(),
            condition: DependencyGroup::from_leaves(
                LogicalOperator::And,
                vec![Dependency::new_file("SkyUI_SE.esp", "Active")],
            ),
        });
        g.plugins.push(p2k);
        g.plugins.push(p4k);
        step.plugin_groups.push(g);
        m.steps.push(step);
        m
    }

    #[test]
    fn first_sentence_cuts_at_punctuation_or_line() {
        assert_eq!(first_sentence("Balanced quality. Fits most rigs."), "Balanced quality.");
        assert_eq!(first_sentence("v1.2 rocks! Really"), "v1.2 rocks!");
        assert_eq!(first_sentence("Line one\\nLine two"), "Line one");
        assert_eq!(first_sentence("  \n  Second line only"), "Second line only");
        assert_eq!(first_sentence(""), "");
    }

    #[test]
    fn bbcode_output_has_every_section() {
        let m = sample();
        let games = GamesData::load();
        let text = generate(&m, None, &games, None, &DescOptions::default(), &DescLabels::english());
        assert!(text.starts_with("[size=5][b]Aurelia 1.2[/b][/size]\n"), "{text}");
        assert!(text.contains("[url=https://example.org/aurelia]https://example.org/aurelia[/url]"));
        assert!(text.contains("A lovely mod."));
        // Requirements from the declared file dependency (no root).
        assert!(text.contains("[size=4][b]Requirements[/b][/size]"));
        assert!(text.contains("[*]Requires: SkyUI_SE.esp"));
        // Options: nested lists.
        assert!(text.contains("[size=4][b]Installation options[/b][/size]"));
        assert!(text.contains("[*]Step 1: Textures"));
        assert!(text.contains("[*]Resolution (exactly one)"));
        assert!(text.contains("[*]2K — Balanced quality."));
        assert!(text.contains("[*]4K — For high-end GPUs!"));
        assert_eq!(text.matches("[list]").count(), text.matches("[/list]").count());
        assert!(text.contains("[size=4][b]Installation[/b][/size]\nThis mod ships a FOMOD installer"));
        // No previous version: no changelog.
        assert!(!text.contains("Changelog"));
    }

    #[test]
    fn markdown_output_and_toggles() {
        let m = sample();
        let games = GamesData::load();
        let opts = DescOptions {
            format: DescFormat::Markdown,
            include_requirements: false,
            include_install: false,
            language_note: Some("French translation".into()),
            ..Default::default()
        };
        let text = generate(&m, None, &games, None, &opts, &DescLabels::english());
        assert!(text.starts_with("# Aurelia 1.2\n[https://example.org/aurelia](https://example.org/aurelia)\n"));
        assert!(text.contains("*French translation*"));
        assert!(!text.contains("Requirements"));
        assert!(!text.contains("## Installation\n"));
        assert!(text.contains(
            "## Installation options\n- Step 1: Textures\n  - Resolution (exactly one)\n    - 2K — Balanced quality.\n"
        ));
        assert!(
            !text.contains("[size=") && !text.contains("[list]") && !text.contains("[*]"),
            "no BBCode in Markdown: {text}"
        );
        let none = DescOptions {
            format: DescFormat::Markdown,
            include_requirements: false,
            include_options: false,
            include_install: false,
            include_changelog: false,
            language_note: None,
        };
        let text = generate(&m, None, &games, None, &none, &DescLabels::english());
        assert_eq!(
            text,
            "# Aurelia 1.2\n[https://example.org/aurelia](https://example.org/aurelia)\nA lovely mod.\nMore text.\n"
        );
    }

    #[test]
    fn changelog_from_previous_version() {
        let m = sample();
        let mut prev = m.clone();
        prev.version = "1.1".into();
        prev.steps[0].plugin_groups[0].plugins.push(Plugin::new("8K"));
        prev.steps[0].plugin_groups[0].plugins.retain(|p| p.name != "4K");
        let games = GamesData::load();
        let labels = DescLabels::english();
        let opts = DescOptions {
            format: DescFormat::Markdown,
            include_requirements: false,
            include_options: false,
            include_install: false,
            ..Default::default()
        };
        let text = generate(&m, None, &games, Some(&prev), &opts, &labels);
        assert!(text.contains("## Changelog\n- Added\n  - Step «Textures» / Option «4K»\n- Removed\n  - Step «Textures» / Option «8K»\n- Changed\n  - version: «1.1» → «1.2»\n"), "{text}");
        // Identical previous version: section omitted.
        let text = generate(&m, None, &games, Some(&m), &opts, &labels);
        assert!(!text.contains("Changelog"));
        let bb = generate(&m, None, &games, Some(&prev), &DescOptions::default(), &labels);
        assert!(bb.contains("[size=4][b]Changelog[/b][/size]\n[list]\n[*]Added\n[list]\n[*]Step «Textures» / Option «4K»\n[/list]\n[*]Removed"), "{bb}");
    }

    #[test]
    fn requirements_read_plugin_masters_under_root() {
        use crate::models::plugin_header::tests::build_plugin;
        let root = std::env::temp_dir().join(format!("ximod_nexus_req_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let mut m = sample();
        m.required_files.push(InstallFile::new_file("Aurelia.esp"));
        m.required_files.push(InstallFile::new_file("Shipped.esm"));
        let bytes = build_plugin(0, &["Skyrim.esm", "Shipped.esm", "Lib.esm"], &[]);
        std::fs::write(root.join("Aurelia.esp"), &bytes).unwrap();
        std::fs::write(root.join("Shipped.esm"), build_plugin(1, &[], &[])).unwrap();
        let games = GamesData::load();
        let reqs = requirements(&m, Some(&root), &games);
        // Base master and shipped master excluded; the declared SkyUI dependency kept.
        assert_eq!(reqs, vec!["Lib.esm".to_string(), "SkyUI_SE.esp".to_string()]);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn translated_variant_uses_the_translation() {
        let m = sample();
        let mut doc = TranslationDoc::new("eng", "fra");
        doc.units.push(TUnit {
            key: "config/step[0]/group[0]/plugin[0]/name".into(),
            field: TField::PluginName,
            source: "2K".into(),
            target: "2K (équilibré)".into(),
            status: TStatus::Translated,
            locked: false,
            context: String::new(),
            note: String::new(),
        });
        doc.units.push(TUnit {
            key: "info/name".into(),
            field: TField::InfoName,
            source: "Aurelia".into(),
            target: "Aurélia".into(),
            status: TStatus::Translated,
            locked: false,
            context: String::new(),
            note: String::new(),
        });
        let games = GamesData::load();
        let opts = DescOptions {
            format: DescFormat::Markdown,
            language_note: Some("Traduction française".into()),
            ..Default::default()
        };
        let text = generate_translated(&doc, &m, None, &games, None, &opts, &DescLabels::english());
        assert!(text.starts_with("# Aurélia 1.2\n"), "{text}");
        assert!(text.contains("- 2K (équilibré) — Balanced quality."));
        assert!(text.contains("*Traduction française*"));
    }
}
