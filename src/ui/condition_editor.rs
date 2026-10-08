//! Visual condition editor (V2 roadmap, priority 9).
//!
//! Flags, dependency patterns and conditional installs are the trickiest part of
//! the FOMOD format and the easiest to get wrong. This window gives an overview
//! of the wiring: for every condition flag, which options **set** it and which
//! step-visibility rules, conditional installs, plugin dependency patterns and
//! mod requirements **use** it — plus the file-state dependencies. Flags that
//! are set but never used (or used but never set) are highlighted, as are the
//! values tested but never set and the values set but never tested.
//!
//! It also *edits*: every "set by" / "used by" line is a link that selects the
//! node in the main window, a flag can be renamed (or all its uses deleted)
//! across the whole project in one undo step, and a **condition builder** at
//! the bottom edits the condition of the node currently selected in the main
//! window, with a live natural-language sentence.
//!
//! The window never touches the model itself: it returns [`CondAction`]s that
//! the main window applies (`XimodApp::apply_cond_actions`), so that every
//! change goes through `mark_modified` and the undo history. The analysis
//! itself lives in [`crate::models::flags`], shared with the validation.

use std::hash::{Hash, Hasher};
use std::rc::Rc;

use eframe::egui::{self, RichText};

use crate::i18n::I18n;
use crate::models::condition_text::{self, ConditionLabels};
use crate::models::flags::{FlagReport, FlagSite, analyze_flags};
use crate::models::{
    Dependency, DependencyGroup, DependencyItem, DependencyType, FileState, LogicalOperator, PluginType, Ximod,
};
use crate::ui::foldnav::{FoldFrame, FoldLabels, FoldNav};
use crate::ui::main_window::{AutocompleteCache, Tab, XimodApp};
use crate::ui::problems::Target;
use crate::ui::theme::{Palette, icon};
use crate::ui::widgets::dependency_editor::{DepCandidates, DepEditorLabels, render_nested_group};

/// The condition list the builder edits (0-based indices).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderTarget {
    /// `<visible>` conditions of a step.
    StepVisibility(usize),
    /// One dependency pattern of an option: `(step, group, plugin, pattern)`.
    /// A `pattern` index equal to the number of patterns creates a new one.
    PluginPattern(usize, usize, usize, usize),
    /// A conditional-install set.
    ConditionalSet(usize),
    /// The mod requirements (`<moduleDependencies>`).
    Module,
}

/// What the editor asks the main window to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CondAction {
    /// Select a node in the project tree.
    Select(Target),
    /// Rename a flag everywhere (one undo step).
    RenameFlag { from: String, to: String },
    /// Remove every setting and every test of a flag.
    DeleteFlagUses { name: String },
    /// Replace the condition group (operator, leaves and nested groups) of
    /// a node.
    ApplyBuilder {
        target: BuilderTarget,
        group: DependencyGroup,
    },
    /// Change the type an option takes when a pattern matches.
    SetPatternType {
        step: usize,
        group: usize,
        plugin: usize,
        pattern: usize,
        pattern_type: String,
    },
    /// Make `pattern` the one shown in the option's inspector (the builder
    /// follows the inspector's pattern selection).
    SelectPattern {
        step: usize,
        group: usize,
        plugin: usize,
        pattern: usize,
    },
}

/// State of the visual condition editor window.
#[derive(Debug, Default)]
pub struct ConditionEditor {
    /// Whether the window is open.
    pub open: bool,
    /// Analysis of the project, with the fingerprint of the model it was built
    /// from. Rebuilt only when the fingerprint changes instead of every frame.
    cache: Option<(u64, Analysis)>,
    /// Flag being renamed: its current name and the text typed so far.
    rename: Option<(String, String)>,
    /// Error shown under the rename field (empty / duplicate name).
    rename_error: Option<String>,
    /// Which pattern of the selected option the builder edits.
    pub builder_pattern: usize,
    /// Builder rows being edited, for which target, and the model's list as
    /// last seen (to tell an external change from our own commit).
    draft: Option<Draft>,
    /// Keyboard cursor and fold queue of the flag list (↑ ↓ move, ← →
    /// fold / unfold, Enter selects the node of the row). Row keys:
    /// `flag:<name>` for a heading, `flag:<name>/<n>` for its n-th "set by"
    /// / "used by" line, `files` and `files/<n>` for the file dependencies.
    pub nav: FoldNav,
}

#[derive(Debug)]
struct Draft {
    target: BuilderTarget,
    /// The group being edited (incomplete leaves included).
    rows: DependencyGroup,
    /// The model's group as last seen / committed.
    snapshot: DependencyGroup,
}

/// Display-ready result of the analysis, cached between frames.
#[derive(Debug, Default)]
struct Analysis {
    report: FlagReport,
    /// One entry per flag, sorted by flag name.
    flags: Vec<FlagView>,
    /// File-state dependencies, already formatted as bullet lines.
    file_deps: Vec<Line>,
}

/// One formatted line of the overview, with the node it points at.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    text: String,
    target: Option<Target>,
}

/// One flag as shown in the window (all strings pre-formatted).
#[derive(Debug)]
struct FlagView {
    name: String,
    /// Collapsing-header text (`⚑ name`).
    header: String,
    /// Stable id salt of the collapsing header (`flag_name`).
    id_salt: String,
    /// Set but never used, or used but never set.
    orphan: bool,
    /// `  • <where> = <value>` lines.
    setters: Vec<Line>,
    /// `  • <where> (needs = <value>)` lines.
    users: Vec<Line>,
    /// Distinct values the flag is set to, and whether each is ever tested.
    set_values: Vec<(String, bool)>,
    /// Distinct values the flag is tested against, and whether each is ever set.
    tested_values: Vec<(String, bool)>,
}

/// The tree node a site lives in.
pub(crate) fn target_of(site: FlagSite) -> Target {
    match site {
        FlagSite::Plugin { step, group, plugin }
        | FlagSite::PluginPattern {
            step, group, plugin, ..
        } => Target::Plugin(step, group, plugin),
        FlagSite::StepVisibility { step } => Target::Step(step),
        FlagSite::CondSet { index } => Target::CondSet(index),
        FlagSite::Module => Target::Info,
    }
}

/// Cheap fingerprint of everything the analysis reads from the project: the
/// tree shape (steps / groups / plugins), plugin names, condition flags,
/// dependency patterns, step visibility rules, conditional-install and
/// mod-wide dependencies. Hashing borrows the strings, so unlike the
/// analysis it allocates nothing.
fn fingerprint(ximod: &Ximod) -> u64 {
    fn hash_group(g: &DependencyGroup, h: &mut impl Hasher) {
        (g.operator == LogicalOperator::Or).hash(h);
        g.items.len().hash(h);
        for item in &g.items {
            match item {
                DependencyItem::Leaf(d) => {
                    0u8.hash(h);
                    d.dep_type.hash(h);
                    d.name.hash(h);
                    d.value.hash(h);
                }
                DependencyItem::Group(sub) => {
                    1u8.hash(h);
                    hash_group(sub, h);
                }
            }
        }
    }

    let mut h = std::collections::hash_map::DefaultHasher::new();
    ximod.steps.len().hash(&mut h);
    for step in &ximod.steps {
        step.plugin_groups.len().hash(&mut h);
        for group in &step.plugin_groups {
            group.plugins.len().hash(&mut h);
            for plugin in &group.plugins {
                plugin.name.hash(&mut h);
                plugin.condition_flags.len().hash(&mut h);
                for cf in &plugin.condition_flags {
                    cf.name.hash(&mut h);
                    cf.value.hash(&mut h);
                }
                plugin.dependency_patterns.len().hash(&mut h);
                for pat in &plugin.dependency_patterns {
                    pat.pattern_type.hash(&mut h);
                    hash_group(&pat.condition, &mut h);
                }
            }
        }
        hash_group(&step.visibility, &mut h);
    }
    ximod.conditional_files.len().hash(&mut h);
    for set in &ximod.conditional_files {
        hash_group(&set.condition, &mut h);
    }
    match &ximod.module_dependencies {
        Some(m) => hash_group(m, &mut h),
        None => 0usize.hash(&mut h),
    }
    h.finish()
}

/// Human label of a site, from the translated templates.
fn site_label(ximod: &Ximod, site: FlagSite, labels: &Labels) -> String {
    let plugin_name = |si: usize, gi: usize, pi: usize| -> &str {
        ximod
            .steps
            .get(si)
            .and_then(|s| s.plugin_groups.get(gi))
            .and_then(|g| g.plugins.get(pi))
            .map(|p| p.name.as_str())
            .unwrap_or_default()
    };
    match site {
        FlagSite::Plugin { step, group, plugin } => fill(
            &labels.setter,
            &[
                ("step", &(step + 1).to_string()),
                ("group", &(group + 1).to_string()),
                ("name", plugin_name(step, group, plugin)),
            ],
        ),
        FlagSite::PluginPattern {
            step,
            group,
            plugin,
            pattern,
        } => {
            let ty = ximod
                .steps
                .get(step)
                .and_then(|s| s.plugin_groups.get(group))
                .and_then(|g| g.plugins.get(plugin))
                .and_then(|p| p.dependency_patterns.get(pattern))
                .map(|p| p.pattern_type.as_str())
                .unwrap_or_default();
            fill(
                &labels.pattern_of,
                &[("name", plugin_name(step, group, plugin)), ("type", ty)],
            )
        }
        FlagSite::StepVisibility { step } => fill(&labels.visibility, &[("step", &(step + 1).to_string())]),
        FlagSite::CondSet { index } => fill(&labels.cond_set, &[("num", &(index + 1).to_string())]),
        FlagSite::Module => labels.module.clone(),
    }
}

/// Run the analysis and format its result once, ready to be drawn every frame.
fn build_analysis(ximod: &Ximod, labels: &Labels) -> Analysis {
    let report = analyze_flags(ximod);
    let flags = report
        .flags
        .iter()
        .map(|f| {
            let never_set = f.values_never_set();
            let never_tested = f.values_never_tested();
            FlagView {
                name: f.name.clone(),
                header: format!("{} {}", icon::FLAG, f.name),
                id_salt: format!("flag_{}", f.name),
                orphan: f.is_orphan(),
                setters: f
                    .setters
                    .iter()
                    .map(|s| Line {
                        text: format!("  • {} = {}", site_label(ximod, s.site, labels), s.value),
                        target: Some(target_of(s.site)),
                    })
                    .collect(),
                users: f
                    .users
                    .iter()
                    .map(|u| Line {
                        text: format!(
                            "  • {}",
                            fill(
                                &labels.needs,
                                &[("ctx", &site_label(ximod, u.site, labels)), ("value", &u.value)]
                            )
                        ),
                        target: Some(target_of(u.site)),
                    })
                    .collect(),
                set_values: f
                    .set_values()
                    .into_iter()
                    .map(|v| (v.to_string(), !never_tested.contains(&v)))
                    .collect(),
                tested_values: f
                    .tested_values()
                    .into_iter()
                    .map(|v| (v.to_string(), !never_set.contains(&v)))
                    .collect(),
            }
        })
        .collect();
    let file_deps = report
        .file_deps
        .iter()
        .map(|f| Line {
            text: format!(
                "• {}",
                fill(
                    &labels.file,
                    &[
                        ("ctx", &site_label(ximod, f.site, labels)),
                        ("name", &f.name),
                        ("state", &f.state)
                    ]
                )
            ),
            target: Some(target_of(f.site)),
        })
        .collect();
    Analysis {
        report,
        flags,
        file_deps,
    }
}

/// Translated texts of the window, resolved once per frame.
struct Texts {
    set_by: String,
    used_by: String,
    filedeps: String,
    empty: String,
    orphan_set: String,
    orphan_used: String,
    rename: String,
    rename_exists: String,
    delete_uses: String,
    values_set: String,
    values_tested: String,
    never_set: String,
    never_tested: String,
    ok: String,
    cancel: String,
    builder: String,
    builder_none: String,
    pattern: String,
    pattern_type: String,
    operator: String,
    add: String,
    add_group: String,
    remove: String,
    dep_flag: String,
    dep_file: String,
    dep_game: String,
    dep_fomm: String,
    add_pattern: String,
    s_if: String,
    s_empty: String,
    /// Translated fragments of the condition sentence.
    cond: ConditionLabels,
    /// Labels of the nested-group frames (shared with the inspector).
    dep_labels: DepEditorLabels,
}

impl Texts {
    fn new(i18n: &I18n) -> Self {
        Self {
            set_by: i18n.t("condeditor-set-by"),
            used_by: i18n.t("condeditor-used-by"),
            filedeps: i18n.t("condeditor-filedeps"),
            empty: i18n.t("condeditor-empty"),
            orphan_set: i18n.t("condeditor-orphan-set"),
            orphan_used: i18n.t("condeditor-orphan-used"),
            rename: i18n.t("condeditor-rename"),
            rename_exists: i18n.t("condeditor-rename-exists"),
            delete_uses: i18n.t("condeditor-delete-uses"),
            values_set: i18n.t("condeditor-values-set"),
            values_tested: i18n.t("condeditor-values-tested"),
            // Fetched with a literal "{value}" argument so `fill` can substitute it per row.
            never_set: i18n.t_arg("condeditor-value-never-set", "value", "{value}"),
            never_tested: i18n.t_arg("condeditor-value-never-tested", "value", "{value}"),
            ok: i18n.t("btn-ok"),
            cancel: i18n.t("btn-cancel"),
            builder: i18n.t("condeditor-builder"),
            builder_none: i18n.t("condeditor-builder-none"),
            pattern: i18n.t("condeditor-builder-pattern"),
            pattern_type: i18n.t("label-pattern-type"),
            operator: i18n.t("label-operator"),
            add: i18n.t("btn-add-condition"),
            add_group: i18n.t("btn-add-group-cond"),
            remove: i18n.t("btn-remove"),
            dep_flag: i18n.t("dep-type-flag"),
            dep_file: i18n.t("dep-type-file"),
            dep_game: i18n.t("dep-type-game"),
            dep_fomm: i18n.t("dep-type-fomm"),
            add_pattern: i18n.t("btn-add-pattern"),
            s_if: i18n.t("condeditor-sentence-if"),
            s_empty: i18n.t("condeditor-sentence-empty"),
            cond: ConditionLabels {
                and: i18n.t("condeditor-sentence-and"),
                or: i18n.t("condeditor-sentence-or"),
                flag: i18n.t("condeditor-sentence-flag"),
                file: i18n.t("condeditor-sentence-file"),
                game: i18n.t("condeditor-sentence-game"),
                fomm: i18n.t("condeditor-sentence-fomm"),
                empty: i18n.t("condeditor-sentence-empty"),
            },
            dep_labels: DepEditorLabels::new(i18n),
        }
    }

    fn kind_label(&self, kind: DependencyType) -> &str {
        match kind {
            DependencyType::Flag => &self.dep_flag,
            DependencyType::File => &self.dep_file,
            DependencyType::Game => &self.dep_game,
            DependencyType::Fomm => &self.dep_fomm,
        }
    }
}

/// What the builder edits this frame, resolved from the model.
struct BuilderView {
    target: BuilderTarget,
    /// The node's condition group (operator, leaves, nested groups).
    group: DependencyGroup,
    /// "THEN …" part of the sentence.
    then: String,
    /// `(number of patterns, type of the edited one)` for an option.
    patterns: Option<(usize, String)>,
}

/// Resolve the builder target against the model. `None` when the node no
/// longer exists.
fn resolve_builder(ximod: &Ximod, target: BuilderTarget, i18n: &I18n) -> Option<BuilderView> {
    match target {
        BuilderTarget::StepVisibility(si) => {
            let step = ximod.steps.get(si)?;
            Some(BuilderView {
                target,
                group: step.visibility.clone(),
                then: i18n.t("condeditor-sentence-then-visible"),
                patterns: None,
            })
        }
        BuilderTarget::PluginPattern(si, gi, pi, pat_i) => {
            let plugin = ximod.steps.get(si)?.plugin_groups.get(gi)?.plugins.get(pi)?;
            let n = plugin.dependency_patterns.len();
            if n == 0 {
                // Nothing to edit yet: the builder offers to create a pattern.
                return Some(BuilderView {
                    target: BuilderTarget::PluginPattern(si, gi, pi, 0),
                    group: DependencyGroup::default(),
                    then: String::new(),
                    patterns: Some((0, String::new())),
                });
            }
            let pat_i = pat_i.min(n - 1);
            let pat = &plugin.dependency_patterns[pat_i];
            Some(BuilderView {
                target: BuilderTarget::PluginPattern(si, gi, pi, pat_i),
                group: pat.condition.clone(),
                then: i18n.t_arg(
                    "condeditor-sentence-then-type",
                    "type",
                    &crate::ui::labels::plugin_type_name(i18n, &pat.pattern_type),
                ),
                patterns: Some((n, pat.pattern_type.clone())),
            })
        }
        BuilderTarget::ConditionalSet(ci) => {
            let set = ximod.conditional_files.get(ci)?;
            Some(BuilderView {
                target,
                group: set.condition.clone(),
                then: i18n.t("condeditor-sentence-then-install"),
                patterns: None,
            })
        }
        BuilderTarget::Module => Some(BuilderView {
            target,
            group: ximod.module_dependencies.clone().unwrap_or_default(),
            then: i18n.t("condeditor-sentence-then-module"),
            patterns: None,
        }),
    }
}

/// The natural-language sentence of a condition group: nested groups are
/// parenthesised (`IF (flag A = x OR flag A = y) AND flag B = z THEN …`).
fn sentence(t: &Texts, group: &DependencyGroup, then: &str) -> String {
    if !group.leaves().any(Dependency::is_complete) {
        return format!("{} {then}", t.s_empty);
    }
    format!("{} {} {then}", t.s_if, condition_text::describe(group, &t.cond))
}

/// The group as written to the model: complete leaves only (a name for a
/// flag / file, a version for a game / manager leaf), nested groups kept.
fn committed(rows: &DependencyGroup) -> DependencyGroup {
    let mut out = rows.clone();
    out.retain_leaves(Dependency::is_complete);
    out
}

impl ConditionEditor {
    /// Create a closed editor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Draw the editor window. Reads the project to build the overview and
    /// edits the condition of `builder` (the node selected in the main
    /// window). Returns the actions for the main window to apply.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        ximod: &Ximod,
        i18n: &I18n,
        autocomplete: &AutocompleteCache,
        builder: Option<BuilderTarget>,
    ) -> Vec<CondAction> {
        let mut actions = Vec::new();
        if !self.open {
            // Nothing is drawn while closed: do not keep stale state around.
            self.cache = None;
            self.rename = None;
            self.rename_error = None;
            self.draft = None;
            return actions;
        }

        // Re-analyse only when the relevant part of the project (or the UI
        // language, which the labels depend on) changed.
        let fp = {
            let mut h = std::hash::DefaultHasher::new();
            fingerprint(ximod).hash(&mut h);
            i18n.current_locale().hash(&mut h);
            h.finish()
        };
        if !matches!(&self.cache, Some((cached, _)) if *cached == fp) {
            let labels = Labels::new(i18n);
            self.cache = Some((fp, build_analysis(ximod, &labels)));
        }
        let Some((_, analysis)) = &self.cache else {
            return actions; // unreachable: the cache was filled just above
        };

        let title = i18n.t("condeditor-title");
        let t = Texts::new(i18n);
        let view = builder.and_then(|b| {
            let b = match b {
                BuilderTarget::PluginPattern(si, gi, pi, _) => {
                    BuilderTarget::PluginPattern(si, gi, pi, self.builder_pattern)
                }
                other => other,
            };
            resolve_builder(ximod, b, i18n)
        });

        let _ = title;
        let mut rename = self.rename.take();
        let mut rename_error = self.rename_error.take();
        let mut draft = self.draft.take();
        let mut builder_pattern = self.builder_pattern;
        let fold_labels = FoldLabels::new(i18n);
        let mut nav = self.nav.frame(&fold_labels, false);
        // (key, target) of every navigable row drawn this frame.
        let mut targets: Vec<(String, Option<Target>)> = Vec::new();
        egui::CentralPanel::default().show(ctx, |ui| {
            let palette = Palette::from_ui(ui);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                if analysis.flags.is_empty() && analysis.file_deps.is_empty() {
                    ui.label(&t.empty);
                } else {
                    for info in &analysis.flags {
                        render_flag(
                            ui,
                            info,
                            analysis,
                            &t,
                            &palette,
                            &mut rename,
                            &mut rename_error,
                            &mut actions,
                            &mut nav,
                            &mut targets,
                        );
                    }
                    if !analysis.file_deps.is_empty() {
                        ui.separator();
                        let header = egui::CollapsingHeader::new(&t.filedeps).id_salt("cond_filedeps");
                        targets.push(("files".to_string(), analysis.file_deps.iter().find_map(|f| f.target)));
                        nav.header(ui, "files", 0, header, false, |nav, ui| {
                            for (n, f) in analysis.file_deps.iter().enumerate() {
                                let r = render_line(ui, f, &mut actions);
                                let key = format!("files/{n}");
                                nav.leaf(ui, key.as_str(), 1, r.rect);
                                targets.push((key, f.target));
                            }
                        });
                    }
                }
                ui.separator();
                egui::CollapsingHeader::new(RichText::new(&t.builder).strong())
                    .id_salt("cond_builder")
                    .default_open(true)
                    .show(ui, |ui| match &view {
                        Some(view) => render_builder(
                            ui,
                            view,
                            i18n,
                            &t,
                            analysis,
                            autocomplete,
                            &mut draft,
                            &mut builder_pattern,
                            &mut actions,
                        ),
                        None => {
                            ui.label(RichText::new(&t.builder_none).weak());
                        }
                    });
            });
        });

        // ↑ ↓ ← → over the list; Enter shows the row's node in the inspector
        // (a heading: its first site).
        if let Some(key) = self.nav.end_frame(ctx, nav)
            && let Some(target) = targets.iter().find(|(k, _)| *k == key).and_then(|(_, t)| *t)
        {
            actions.push(CondAction::Select(target));
        }

        self.rename = rename;
        self.rename_error = rename_error;
        self.draft = draft;
        self.builder_pattern = builder_pattern;
        actions
    }
}

/// A "set by" / "used by" line: a link to its node.
fn render_line(ui: &mut egui::Ui, line: &Line, actions: &mut Vec<CondAction>) -> egui::Response {
    match line.target {
        Some(target) => {
            let r = ui.link(line.text.as_str());
            if r.clicked() {
                actions.push(CondAction::Select(target));
            }
            r
        }
        None => ui.label(line.text.as_str()),
    }
}

#[allow(clippy::too_many_arguments)]
fn render_flag(
    ui: &mut egui::Ui,
    info: &FlagView,
    analysis: &Analysis,
    t: &Texts,
    palette: &Palette,
    rename: &mut Option<(String, String)>,
    rename_error: &mut Option<String>,
    actions: &mut Vec<CondAction>,
    nav: &mut FoldFrame<'_>,
    targets: &mut Vec<(String, Option<Target>)>,
) {
    let orphan = info.orphan;
    let header = if orphan {
        RichText::new(info.header.as_str()).color(palette.warning)
    } else {
        RichText::new(info.header.as_str())
    };
    let flag_key = format!("flag:{}", info.name);
    // Enter on the heading goes to the first site of the flag.
    targets.push((
        flag_key.clone(),
        info.setters.iter().chain(&info.users).find_map(|l| l.target),
    ));
    let mut line_no = 0usize;
    let header = egui::CollapsingHeader::new(header).id_salt(info.id_salt.as_str());
    nav.header(ui, flag_key.as_str(), 0, header, orphan, |nav, ui| {
        // ---- rename / delete ----
        ui.horizontal(|ui| {
            let renaming = rename.as_ref().is_some_and(|(n, _)| n == &info.name);
            if renaming {
                let (_, buf) = rename.as_mut().expect("checked");
                let resp = ui.add(egui::TextEdit::singleline(buf).desired_width(180.0));
                let enter = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                let to = buf.trim().to_string();
                let valid = !to.is_empty() && to != info.name;
                let ok = ui.add_enabled(valid, egui::Button::new(&t.ok)).clicked();
                if (ok || enter) && valid {
                    if analysis.report.flag(&to).is_some() {
                        *rename_error = Some(fill(&t.rename_exists, &[("name", &to)]));
                    } else {
                        actions.push(CondAction::RenameFlag {
                            from: info.name.clone(),
                            to,
                        });
                        *rename = None;
                        *rename_error = None;
                    }
                }
                if ui.button(&t.cancel).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    *rename = None;
                    *rename_error = None;
                }
            } else {
                if ui.small_button(&t.rename).clicked() {
                    *rename = Some((info.name.clone(), info.name.clone()));
                    *rename_error = None;
                }
                if ui
                    .small_button(RichText::new(format!("{} {}", icon::DELETE, t.delete_uses)).color(palette.danger))
                    .clicked()
                {
                    actions.push(CondAction::DeleteFlagUses {
                        name: info.name.clone(),
                    });
                }
            }
        });
        if rename.as_ref().is_some_and(|(n, _)| n == &info.name)
            && let Some(err) = rename_error.as_ref()
        {
            ui.colored_label(palette.danger, err.as_str());
        }
        // ---- setters ----
        ui.label(&t.set_by);
        if info.setters.is_empty() {
            ui.colored_label(palette.warning, format!("  — {}", t.orphan_used));
        } else {
            for s in &info.setters {
                let r = render_line(ui, s, actions);
                let key = format!("{flag_key}/{line_no}");
                line_no += 1;
                nav.leaf(ui, key.as_str(), 1, r.rect);
                targets.push((key, s.target));
            }
        }
        ui.add_space(4.0);
        // ---- users ----
        ui.label(&t.used_by);
        if info.users.is_empty() {
            ui.colored_label(palette.warning, format!("  — {}", t.orphan_set));
        } else {
            for u in &info.users {
                let r = render_line(ui, u, actions);
                let key = format!("{flag_key}/{line_no}");
                line_no += 1;
                nav.leaf(ui, key.as_str(), 1, r.rect);
                targets.push((key, u.target));
            }
        }
        // ---- values ----
        if !info.set_values.is_empty() {
            ui.add_space(4.0);
            ui.label(&t.values_set);
            for (value, tested) in &info.set_values {
                if *tested {
                    ui.label(format!("  • {value}"));
                } else {
                    ui.label(RichText::new(format!("  • {}", fill(&t.never_tested, &[("value", value)]))).weak());
                }
            }
        }
        if !info.tested_values.is_empty() {
            ui.add_space(4.0);
            ui.label(&t.values_tested);
            for (value, set) in &info.tested_values {
                if *set {
                    ui.label(format!("  • {value}"));
                } else {
                    ui.colored_label(
                        palette.warning,
                        format!("  • {}", fill(&t.never_set, &[("value", value)])),
                    );
                }
            }
        }
    });
}

/// The condition builder: sentence, operator, one row per dependency.
#[allow(clippy::too_many_arguments)]
fn render_builder(
    ui: &mut egui::Ui,
    view: &BuilderView,
    i18n: &I18n,
    t: &Texts,
    analysis: &Analysis,
    autocomplete: &AutocompleteCache,
    draft: &mut Option<Draft>,
    builder_pattern: &mut usize,
    actions: &mut Vec<CondAction>,
) {
    // ---- option: pattern picker, "new pattern", type ----
    if let Some((n_patterns, pattern_type)) = &view.patterns {
        let BuilderTarget::PluginPattern(si, gi, pi, pat_i) = view.target else {
            return;
        };
        ui.horizontal(|ui| {
            if *n_patterns > 0 {
                ui.label(&t.pattern);
                let current = (*builder_pattern).min(n_patterns - 1);
                egui::ComboBox::from_id_salt("cb_pattern")
                    .selected_text(i18n.t_num("pattern-label", current as i64 + 1))
                    .show_ui(ui, |ui| {
                        for idx in 0..*n_patterns {
                            if ui
                                .selectable_label(current == idx, i18n.t_num("pattern-label", idx as i64 + 1))
                                .clicked()
                                && current != idx
                            {
                                *builder_pattern = idx;
                                actions.push(CondAction::SelectPattern {
                                    step: si,
                                    group: gi,
                                    plugin: pi,
                                    pattern: idx,
                                });
                            }
                        }
                    });
            }
            if ui.button(format!("{} {}", icon::ADD, t.add_pattern)).clicked() {
                actions.push(CondAction::ApplyBuilder {
                    target: BuilderTarget::PluginPattern(si, gi, pi, *n_patterns),
                    group: DependencyGroup::default(),
                });
                *builder_pattern = *n_patterns;
            }
        });
        if *n_patterns == 0 || pat_i >= *n_patterns {
            return;
        }
        ui.horizontal(|ui| {
            ui.label(&t.pattern_type);
            egui::ComboBox::from_id_salt("cb_pattern_type")
                .selected_text(crate::ui::labels::plugin_type_name(i18n, pattern_type))
                .show_ui(ui, |ui| {
                    for pt in PluginType::variants() {
                        let name = pt.as_str();
                        if ui
                            .selectable_label(pattern_type == name, crate::ui::labels::plugin_type(i18n, *pt))
                            .clicked()
                            && pattern_type != name
                        {
                            actions.push(CondAction::SetPatternType {
                                step: si,
                                group: gi,
                                plugin: pi,
                                pattern: pat_i,
                                pattern_type: name.to_string(),
                            });
                        }
                    }
                });
        });
    }

    // ---- draft rows: (re)loaded from the model unless being edited ----
    let reload = match draft.as_ref() {
        Some(d) => d.target != view.target || d.snapshot != view.group,
        None => true,
    };
    if reload {
        *draft = Some(Draft {
            target: view.target,
            rows: view.group.clone(),
            snapshot: view.group.clone(),
        });
    }
    let d = draft.as_mut().expect("filled above");

    // ---- sentence ----
    ui.add(egui::Label::new(RichText::new(sentence(t, &d.rows, &view.then)).italics()).wrap());
    ui.add_space(4.0);

    // ---- operator ----
    let operator = view.group.operator;
    ui.horizontal(|ui| {
        ui.label(&t.operator);
        egui::ComboBox::from_id_salt("cb_operator")
            .selected_text(crate::ui::labels::operator(i18n, operator))
            .show_ui(ui, |ui| {
                for op in LogicalOperator::variants() {
                    if ui
                        .selectable_label(operator == *op, crate::ui::labels::operator(i18n, *op))
                        .clicked()
                        && operator != *op
                    {
                        d.rows.operator = *op;
                        let mut now = committed(&d.rows);
                        now.operator = *op;
                        actions.push(CondAction::ApplyBuilder {
                            target: view.target,
                            group: now.clone(),
                        });
                        d.snapshot = now;
                    }
                }
            });
    });

    // ---- rows: top-level leaves inline, nested groups as frames ----
    let file_states: Rc<Vec<String>> = Rc::new(FileState::variants().iter().map(|s| s.as_str().to_string()).collect());
    let cand = DepCandidates::new(autocomplete.dep_names.clone(), autocomplete.flag_values.clone());
    let mut any_focus = false;
    let mut remove: Option<usize> = None;
    let mut combo_changed = false;
    for (i, item) in d.rows.items.iter_mut().enumerate() {
        match item {
            DependencyItem::Leaf(dep) => {
                ui.horizontal(|ui| {
                    let kind = dep.kind();
                    egui::ComboBox::from_id_salt(format!("cb_kind_{i}"))
                        .selected_text(t.kind_label(kind))
                        .width(80.0)
                        .show_ui(ui, |ui| {
                            for k in DependencyType::variants() {
                                if ui.selectable_label(kind == *k, t.kind_label(*k)).clicked() && kind != *k {
                                    dep.dep_type = k.as_str().into();
                                    if matches!(k, DependencyType::Game | DependencyType::Fomm) {
                                        dep.name.clear();
                                    }
                                    combo_changed = true;
                                }
                            }
                        });
                    let kind = dep.kind();
                    match kind {
                        DependencyType::Flag | DependencyType::File => {
                            let names: &[String] = if kind == DependencyType::File {
                                &autocomplete.file_names
                            } else {
                                &autocomplete.flags
                            };
                            let r = crate::ui::components::autocomplete_edit(
                                ui,
                                &format!("cb_name_{i}"),
                                &mut dep.name,
                                names,
                            );
                            any_focus |= r.has_focus();
                            ui.label("=");
                        }
                        DependencyType::Game | DependencyType::Fomm => {
                            ui.label("≥");
                        }
                    }
                    let values: Rc<Vec<String>> = match kind {
                        DependencyType::File => file_states.clone(),
                        DependencyType::Flag => match analysis.report.flag(&dep.name) {
                            Some(f) => {
                                let mut v: Vec<String> = f.set_values().into_iter().map(str::to_string).collect();
                                for tv in f.tested_values() {
                                    if !v.iter().any(|x| x == tv) {
                                        v.push(tv.to_string());
                                    }
                                }
                                Rc::new(v)
                            }
                            None => autocomplete.flag_values.clone(),
                        },
                        DependencyType::Game | DependencyType::Fomm => Rc::new(Vec::new()),
                    };
                    let r =
                        crate::ui::components::autocomplete_edit(ui, &format!("cb_value_{i}"), &mut dep.value, &values);
                    any_focus |= r.has_focus();
                    if ui
                        .add(crate::ui::components::icon_button(icon::DELETE))
                        .on_hover_text(&t.remove)
                        .clicked()
                    {
                        remove = Some(i);
                    }
                });
            }
            DependencyItem::Group(sub) => {
                let r = render_nested_group(ui, sub, &[i], "cb", &t.dep_labels, &cand);
                any_focus |= r.has_focus;
                combo_changed |= r.modified && !r.has_focus;
                if r.remove_self {
                    remove = Some(i);
                }
            }
        }
    }
    if let Some(i) = remove {
        d.rows.items.remove(i);
    }
    ui.horizontal(|ui| {
        if ui.button(format!("{} {}", icon::ADD, t.add)).clicked() {
            d.rows.push_leaf(Dependency::new_flag("", ""));
        }
        if ui.button(format!("{} {}", icon::ADD, t.add_group)).clicked() {
            d.rows.push_group(DependencyGroup::new(LogicalOperator::And));
        }
    });

    // ---- commit: when no row is being typed in and the rows differ ----
    if !any_focus || combo_changed {
        let now = committed(&d.rows);
        if now != d.snapshot {
            actions.push(CondAction::ApplyBuilder {
                target: view.target,
                group: now.clone(),
            });
            d.snapshot = now;
        }
    }
}

/// Translated sentence templates used by the analysis to describe where a
/// flag is set or used. Resolved once per analysis, not per line.
struct Labels {
    setter: String,     // "Step {step} / Group {group} / «{name}»"
    pattern_of: String, // "Pattern of «{name}» → {type}"
    visibility: String, // "Visibility of step {step}"
    cond_set: String,   // "Conditional set {num}"
    needs: String,      // "{ctx} (needs = {value})"
    file: String,       // "{ctx}: file «{name}» ({state})"
    module: String,     // "Mod requirements"
}

impl Labels {
    fn new(i18n: &I18n) -> Self {
        Self {
            setter: i18n.t("condeditor-setter-loc"),
            pattern_of: i18n.t("condeditor-pattern-of"),
            visibility: i18n.t("condeditor-visibility-of"),
            cond_set: i18n.t("condeditor-cond-set"),
            needs: i18n.t("condeditor-needs"),
            file: i18n.t("condeditor-file-dep"),
            module: i18n.t("info-module-deps"),
        }
    }

    /// English fallbacks, for the unit tests which run without locale files.
    #[cfg(test)]
    fn english() -> Self {
        Self {
            setter: "Step {step} / Group {group} / «{name}»".into(),
            pattern_of: "Pattern of «{name}» → {type}".into(),
            visibility: "Visibility of step {step}".into(),
            cond_set: "Conditional set {num}".into(),
            needs: "{ctx} (needs = {value})".into(),
            file: "{ctx}: file «{name}» ({state})".into(),
            module: "Mod requirements".into(),
        }
    }
}

/// Fill a `{placeholder}` template. Plain brace placeholders rather than
/// Fluent arguments so the templates stay cheap to apply thousands of times.
fn fill(template: &str, pairs: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (k, v) in pairs {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

// ---------------------------------------------------------------------------
// Main-window glue
// ---------------------------------------------------------------------------

impl XimodApp {
    /// The condition list of the node selected in the main window, if any.
    pub(crate) fn builder_target(&self) -> Option<BuilderTarget> {
        match self.current_tab {
            Tab::Info => Some(BuilderTarget::Module),
            Tab::ConditionalInstalls => self.selection.cond_pattern.map(BuilderTarget::ConditionalSet),
            Tab::RequiredInstalls => None,
            Tab::Steps => {
                match (self.selection.step, self.selection.group, self.selection.plugin) {
                    (Some(si), Some(gi), Some(pi)) if self.plugin_exists(si, gi, pi) => Some(
                        BuilderTarget::PluginPattern(si, gi, pi, self.current_plugin_pattern_index.unwrap_or(0)),
                    ),
                    (Some(si), _, _) if si < self.ximod.steps.len() => Some(BuilderTarget::StepVisibility(si)),
                    _ => None,
                }
            }
        }
    }

    /// Draw the condition editor and apply what it asked for.
    pub(crate) fn render_condition_editor(&mut self, ctx: &egui::Context) {
        if !self.condition_editor.open {
            return;
        }
        let builder = self.builder_target();
        // Follow the pattern selected in the inspector.
        if let (Some(BuilderTarget::PluginPattern(..)), Some(idx)) = (builder, self.current_plugin_pattern_index) {
            self.condition_editor.builder_pattern = idx;
        }
        self.autocomplete();
        // An independent, freely movable window (like the preview).
        let title = self.i18n.t("condeditor-title");
        let vb = self.free_viewport_builder(ctx, "ximod_condeditor", title, [720.0, 600.0], false);
        // Taken out to avoid a double borrow of self.
        let mut ed = std::mem::take(&mut self.condition_editor);
        let mut actions = Vec::new();
        let mut close = false;
        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("ximod_condeditor"), vb, |ctx, _class| {
            actions = ed.show(ctx, &self.ximod, &self.i18n, &self.autocomplete, builder);
            crate::ui::widgets::free_window::record_win_geom(&mut self.config, ctx, "ximod_condeditor");
            if ctx.input(|i| i.viewport().close_requested()) {
                close = true;
            }
        });
        self.condition_editor = ed;
        if close {
            self.condition_editor.open = false;
            self.free_window_closed("ximod_condeditor");
        }
        self.apply_cond_actions(actions);
    }

    /// Apply the editor's actions to the model, one undo step each.
    pub(crate) fn apply_cond_actions(&mut self, actions: Vec<CondAction>) {
        for action in actions {
            match action {
                CondAction::Select(target) => {
                    self.select_target(target);
                }
                CondAction::RenameFlag { from, to } => {
                    let to = to.trim().to_string();
                    if to.is_empty() || to == from {
                        continue;
                    }
                    let report = analyze_flags(&self.ximod);
                    if report.flag(&to).is_some() {
                        let msg = self.i18n.t_arg("condeditor-rename-exists", "name", &to);
                        self.notify_warn(msg);
                        continue;
                    }
                    let n = crate::models::flags::rename_flag(&mut self.ximod, &from, &to);
                    if n > 0 {
                        self.mark_modified();
                        let mut args = fluent::FluentArgs::new();
                        args.set("from", from);
                        args.set("to", to);
                        args.set("num", n as i64);
                        let msg = self.i18n.t_with_args("condeditor-renamed", Some(&args));
                        self.notify_ok(msg);
                    }
                }
                CondAction::DeleteFlagUses { name } => {
                    let n = crate::models::flags::delete_flag_uses(&mut self.ximod, &name);
                    if n > 0 {
                        self.mark_modified();
                        self.clamp_selection();
                        let mut args = fluent::FluentArgs::new();
                        args.set("name", name);
                        args.set("num", n as i64);
                        let msg = self.i18n.t_with_args("condeditor-deleted-uses", Some(&args));
                        self.notify_ok(msg);
                    }
                }
                CondAction::ApplyBuilder { target, group } => {
                    if self.apply_builder(target, group) {
                        self.mark_modified();
                    }
                }
                CondAction::SetPatternType {
                    step,
                    group,
                    plugin,
                    pattern,
                    pattern_type,
                } => {
                    if let Some(pat) = self
                        .ximod
                        .steps
                        .get_mut(step)
                        .and_then(|s| s.plugin_groups.get_mut(group))
                        .and_then(|g| g.plugins.get_mut(plugin))
                        .and_then(|p| p.dependency_patterns.get_mut(pattern))
                        && pat.pattern_type != pattern_type
                    {
                        pat.pattern_type = pattern_type;
                        self.mark_modified();
                    }
                }
                CondAction::SelectPattern {
                    step,
                    group,
                    plugin,
                    pattern,
                } => {
                    if self.selection.step == Some(step)
                        && self.selection.group == Some(group)
                        && self.selection.plugin == Some(plugin)
                        && self.plugin_exists(step, group, plugin)
                    {
                        self.current_plugin_pattern_index = Some(pattern);
                        self.current_plugin_dep_index = None;
                        self.condition_editor.builder_pattern = pattern;
                    }
                }
            }
        }
    }

    /// Write a condition group to `target`. Returns whether the model
    /// changed. A pattern index equal to the option's number of patterns
    /// appends a new pattern.
    fn apply_builder(&mut self, target: BuilderTarget, group: DependencyGroup) -> bool {
        match target {
            BuilderTarget::StepVisibility(si) => {
                let Some(step) = self.ximod.steps.get_mut(si) else {
                    return false;
                };
                if step.visibility == group {
                    return false;
                }
                step.visibility = group;
                true
            }
            BuilderTarget::PluginPattern(si, gi, pi, pat_i) => {
                let Some(plugin) = self
                    .ximod
                    .steps
                    .get_mut(si)
                    .and_then(|s| s.plugin_groups.get_mut(gi))
                    .and_then(|g| g.plugins.get_mut(pi))
                else {
                    return false;
                };
                if pat_i == plugin.dependency_patterns.len() {
                    let mut pat = crate::models::DependencyPattern::new();
                    pat.condition = group;
                    plugin.dependency_patterns.push(pat);
                    self.current_plugin_pattern_index = Some(pat_i);
                    return true;
                }
                let Some(pat) = plugin.dependency_patterns.get_mut(pat_i) else {
                    return false;
                };
                if pat.condition == group {
                    return false;
                }
                pat.condition = group;
                true
            }
            BuilderTarget::ConditionalSet(ci) => {
                let Some(set) = self.ximod.conditional_files.get_mut(ci) else {
                    return false;
                };
                if set.condition == group {
                    return false;
                }
                set.condition = group;
                true
            }
            BuilderTarget::Module => {
                let unchanged = match &self.ximod.module_dependencies {
                    Some(m) => *m == group,
                    None => group.is_empty() && group.operator == LogicalOperator::And,
                };
                if unchanged {
                    return false;
                }
                self.ximod.module_dependencies = Some(group);
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        ConditionFlag, ConditionalFileSet, DependencyPattern, Plugin, PluginGroup, SelectionType, Step,
    };
    use crate::ui::foldnav::FoldCmd;

    fn sample() -> Ximod {
        let mut m = Ximod::new("Sample");
        let mut p = Plugin::new("4K");
        p.condition_flags.push(ConditionFlag::new("res", "4K"));
        let mut q = Plugin::new("2K");
        q.condition_flags.push(ConditionFlag::new("res", "2K"));
        let mut pat = DependencyPattern::new();
        pat.condition.push_leaf(Dependency::new_flag("res", "4K"));
        q.dependency_patterns.push(pat);
        let mut g = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        g.plugins.push(p);
        g.plugins.push(q);
        let mut s1 = Step::new("Resolution");
        s1.plugin_groups.push(g);
        let mut s2 = Step::new("Extras");
        s2.visibility.push_leaf(Dependency::new_flag("res", "4K"));
        s2.visibility.push_leaf(Dependency::new_flag("hd", "1"));
        m.steps.push(s1);
        m.steps.push(s2);
        let mut cs = ConditionalFileSet::new();
        cs.condition.push_leaf(Dependency::new_flag("res", "8K"));
        cs.condition.push_leaf(Dependency::new_file("Skyrim.esm", "Active"));
        m.conditional_files.push(cs);
        m
    }

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.ximod = sample();
        app.history.reset(&app.ximod);
        app
    }

    #[test]
    fn fingerprint_changes_with_the_analysed_data_only() {
        let mut m = sample();
        let fp = fingerprint(&m);
        assert_eq!(fingerprint(&m), fp);

        // Not read by the analysis: must not invalidate the cache.
        m.steps[0].name = "Renamed".to_string();
        assert_eq!(fingerprint(&m), fp);

        // Read by the analysis: must invalidate it.
        m.steps[0].plugin_groups[0].plugins[0].condition_flags[0].value = "2K".to_string();
        assert_ne!(fingerprint(&m), fp);
        let fp2 = fingerprint(&m);
        m.module_dependencies = Some(DependencyGroup::from_leaves(
            LogicalOperator::Or,
            vec![Dependency::new_flag("res", "2K")],
        ));
        assert_ne!(fingerprint(&m), fp2);
        // A change inside a nested group invalidates it too.
        let fp3 = fingerprint(&m);
        let mut nested = DependencyGroup::new(LogicalOperator::Or);
        nested.push_leaf(Dependency::new_flag("res", "4K"));
        m.steps[1].visibility.push_group(nested);
        assert_ne!(fingerprint(&m), fp3);
        let fp4 = fingerprint(&m);
        if let Some(DependencyItem::Group(g)) = m.steps[1].visibility.items.last_mut() {
            g.operator = LogicalOperator::And;
        }
        assert_ne!(fingerprint(&m), fp4);
    }

    #[test]
    fn analysis_is_formatted_and_carries_targets() {
        let a = build_analysis(&sample(), &Labels::english());
        assert_eq!(a.flags.len(), 2);
        let f = &a.flags[1];
        assert_eq!(f.name, "res");
        assert_eq!(f.header, format!("{} res", icon::FLAG));
        assert_eq!(f.id_salt, "flag_res");
        assert!(!f.orphan);
        assert_eq!(
            f.setters[0],
            Line {
                text: "  • Step 1 / Group 1 / «4K» = 4K".into(),
                target: Some(Target::Plugin(0, 0, 0))
            }
        );
        assert_eq!(f.setters[1].target, Some(Target::Plugin(0, 0, 1)));
        let users: Vec<(String, Option<Target>)> = f.users.iter().map(|u| (u.text.clone(), u.target)).collect();
        assert_eq!(
            users,
            vec![
                (
                    "  • Pattern of «2K» → Optional (needs = 4K)".to_string(),
                    Some(Target::Plugin(0, 0, 1))
                ),
                (
                    "  • Visibility of step 2 (needs = 4K)".to_string(),
                    Some(Target::Step(1))
                ),
                (
                    "  • Conditional set 1 (needs = 8K)".to_string(),
                    Some(Target::CondSet(0))
                ),
            ]
        );
        // Values: 2K is set but never tested, 8K tested but never set.
        assert_eq!(f.set_values, vec![("2K".to_string(), false), ("4K".to_string(), true)]);
        assert_eq!(
            f.tested_values,
            vec![("4K".to_string(), true), ("8K".to_string(), false)]
        );
        // The orphan flag.
        assert!(a.flags[0].orphan);
        assert_eq!(a.flags[0].users[0].target, Some(Target::Step(1)));
        assert_eq!(a.file_deps.len(), 1);
        assert_eq!(a.file_deps[0].target, Some(Target::CondSet(0)));
        assert_eq!(a.file_deps[0].text, "• Conditional set 1: file «Skyrim.esm» (Active)");
    }

    #[test]
    fn rename_touches_every_site_and_refuses_duplicates_in_one_undo_step() {
        let mut app = app();
        // Duplicate: refused, nothing changes.
        app.apply_cond_actions(vec![CondAction::RenameFlag {
            from: "res".into(),
            to: "hd".into(),
        }]);
        assert!(!app.project_modified);
        assert!(app.status_message.contains("hd"));
        // Empty / unchanged: ignored.
        app.apply_cond_actions(vec![
            CondAction::RenameFlag {
                from: "res".into(),
                to: "  ".into(),
            },
            CondAction::RenameFlag {
                from: "res".into(),
                to: "res".into(),
            },
        ]);
        assert!(!app.project_modified);
        // A real rename.
        app.frame_time = 10.0;
        app.apply_cond_actions(vec![CondAction::RenameFlag {
            from: "res".into(),
            to: "resolution".into(),
        }]);
        assert!(app.project_modified);
        let r = analyze_flags(&app.ximod);
        assert!(r.flag("res").is_none());
        let f = r.flag("resolution").unwrap();
        assert_eq!(f.setters.len(), 2);
        assert_eq!(f.users.len(), 3);
        assert!(app.status_message.contains("5"), "{}", app.status_message);
        // One undo step brings everything back.
        app.undo();
        assert!(analyze_flags(&app.ximod).flag("res").is_some());
        assert!(analyze_flags(&app.ximod).flag("resolution").is_none());
    }

    #[test]
    fn delete_uses_and_navigation() {
        let mut app = app();
        app.apply_cond_actions(vec![CondAction::Select(Target::CondSet(0))]);
        assert_eq!(app.current_tab, Tab::ConditionalInstalls);
        assert_eq!(app.selection.cond_pattern, Some(0));
        app.apply_cond_actions(vec![CondAction::DeleteFlagUses { name: "res".into() }]);
        assert!(app.project_modified);
        assert!(analyze_flags(&app.ximod).flag("res").is_none());
        assert_eq!(app.ximod.conditional_files[0].condition.leaf_count(), 1);
        assert_eq!(app.ximod.steps[1].visibility.leaf_count(), 1);
    }

    #[test]
    fn validation_reports_values_never_set_and_flags_never_used() {
        let mut app = app();
        let ctx = egui::Context::default();
        app.run_full_validation(&ctx);
        let never_set: Vec<&crate::ui::problems::Issue> = app
            .validation_issues
            .iter()
            .filter(|i| i.message.contains("8K") || i.message.contains("\"hd\""))
            .collect();
        assert_eq!(never_set.len(), 2, "{:?}", app.validation_issues);
        assert!(
            never_set
                .iter()
                .all(|i| i.severity == crate::ui::problems::Severity::Warning)
        );
        assert!(never_set.iter().any(|i| i.target == Some(Target::CondSet(0))));
        assert!(never_set.iter().any(|i| i.target == Some(Target::Step(1))));
        // "res" is tested, so no never-used entry; make one.
        assert!(
            !app.validation_issues
                .iter()
                .any(|i| i.severity == crate::ui::problems::Severity::Info && i.message.contains("\"res\""))
        );
        app.ximod.steps[0].plugin_groups[0].plugins[0]
            .condition_flags
            .push(ConditionFlag::new("lonely", "1"));
        app.run_full_validation(&ctx);
        let info = app
            .validation_issues
            .iter()
            .find(|i| i.message.contains("\"lonely\""))
            .expect("never-used flag reported");
        assert_eq!(info.severity, crate::ui::problems::Severity::Info);
        assert_eq!(info.target, Some(Target::Plugin(0, 0, 0)));
        assert!(!info.message.starts_with("issue-"));
    }

    #[test]
    fn builder_applies_to_every_kind_of_target() {
        let mut app = app();
        let mut group = DependencyGroup::from_leaves(
            LogicalOperator::Or,
            vec![
                Dependency::new_flag("res", "2K"),
                Dependency::new_file("A.esp", "Missing"),
            ],
        );
        group.push_group(DependencyGroup::from_leaves(
            LogicalOperator::And,
            vec![Dependency::new_game("1.6")],
        ));
        app.apply_cond_actions(vec![CondAction::ApplyBuilder {
            target: BuilderTarget::StepVisibility(1),
            group: group.clone(),
        }]);
        assert_eq!(app.ximod.steps[1].visibility, group);
        // Same content again: no change recorded.
        app.project_modified = false;
        app.apply_cond_actions(vec![CondAction::ApplyBuilder {
            target: BuilderTarget::StepVisibility(1),
            group: group.clone(),
        }]);
        assert!(!app.project_modified);
        // Existing pattern, then a new one appended (index == len).
        app.apply_cond_actions(vec![
            CondAction::ApplyBuilder {
                target: BuilderTarget::PluginPattern(0, 0, 1, 0),
                group: DependencyGroup::default(),
            },
            CondAction::ApplyBuilder {
                target: BuilderTarget::PluginPattern(0, 0, 1, 1),
                group: group.clone(),
            },
            CondAction::SetPatternType {
                step: 0,
                group: 0,
                plugin: 1,
                pattern: 1,
                pattern_type: "Required".into(),
            },
        ]);
        let plugin = &app.ximod.steps[0].plugin_groups[0].plugins[1];
        assert!(plugin.dependency_patterns[0].condition.is_empty());
        assert_eq!(plugin.dependency_patterns.len(), 2);
        assert_eq!(plugin.dependency_patterns[1].condition, group);
        assert_eq!(plugin.dependency_patterns[1].pattern_type, "Required");
        assert_eq!(app.current_plugin_pattern_index, Some(1));
        // Conditional set and mod requirements.
        app.apply_cond_actions(vec![
            CondAction::ApplyBuilder {
                target: BuilderTarget::ConditionalSet(0),
                group: DependencyGroup::default(),
            },
            CondAction::ApplyBuilder {
                target: BuilderTarget::Module,
                group: group.clone(),
            },
        ]);
        assert!(app.ximod.conditional_files[0].condition.is_empty());
        assert_eq!(app.ximod.module_dependencies.as_ref().unwrap(), &group);
        // Out of range: ignored.
        app.project_modified = false;
        app.apply_cond_actions(vec![CondAction::ApplyBuilder {
            target: BuilderTarget::ConditionalSet(9),
            group,
        }]);
        assert!(!app.project_modified);
    }

    #[test]
    fn selecting_a_pattern_in_the_builder_drives_the_inspector() {
        let mut app = app();
        app.select_target(Target::Plugin(0, 0, 1));
        app.apply_cond_actions(vec![CondAction::SelectPattern {
            step: 0,
            group: 0,
            plugin: 1,
            pattern: 0,
        }]);
        assert_eq!(app.current_plugin_pattern_index, Some(0));
        assert_eq!(app.condition_editor.builder_pattern, 0);
        // Another node selected: ignored.
        app.select_target(Target::Plugin(0, 0, 0));
        app.apply_cond_actions(vec![CondAction::SelectPattern {
            step: 0,
            group: 0,
            plugin: 1,
            pattern: 0,
        }]);
        assert_eq!(app.current_plugin_pattern_index, None);
        assert!(!app.project_modified);
    }

    #[test]
    fn builder_target_follows_the_selection() {
        let mut app = app();
        app.current_tab = Tab::Info;
        assert_eq!(app.builder_target(), Some(BuilderTarget::Module));
        app.select_target(Target::Step(1));
        assert_eq!(app.builder_target(), Some(BuilderTarget::StepVisibility(1)));
        app.select_target(Target::Plugin(0, 0, 1));
        app.current_plugin_pattern_index = Some(0);
        assert_eq!(app.builder_target(), Some(BuilderTarget::PluginPattern(0, 0, 1, 0)));
        app.select_target(Target::CondSet(0));
        assert_eq!(app.builder_target(), Some(BuilderTarget::ConditionalSet(0)));
        app.select_target(Target::Required);
        assert_eq!(app.builder_target(), None);
    }

    #[test]
    fn value_templates_substitute_the_value() {
        let mut app = app();
        for locale in ["eng", "fra"] {
            app.i18n.set_locale(locale);
            let t = Texts::new(&app.i18n);
            let a = fill(&t.never_set, &[("value", "4K")]);
            let b = fill(&t.never_tested, &[("value", "4K")]);
            assert!(a.starts_with("4K"), "{locale}: {a}");
            assert!(b.starts_with("4K"), "{locale}: {b}");
            assert!(!a.contains("value") && !b.contains("value"));
        }
    }

    #[test]
    fn sentence_reads_naturally() {
        let mut app = app();
        let t = Texts::new(&app.i18n);
        let deps = vec![
            Dependency::new_flag("res", "4K"),
            Dependency::new_file("Skyrim.esm", "Active"),
        ];
        let s = sentence(
            &t,
            &DependencyGroup::from_leaves(LogicalOperator::And, deps.clone()),
            "THEN the step is shown",
        );
        assert_eq!(
            s,
            "IF flag res = 4K AND file Skyrim.esm is Active THEN the step is shown"
        );
        let s = sentence(
            &t,
            &DependencyGroup::from_leaves(LogicalOperator::Or, deps[..1].to_vec()),
            "x",
        );
        assert!(s.starts_with("IF flag res = 4K x"));
        let s = sentence(&t, &DependencyGroup::default(), "x");
        assert!(!s.starts_with("IF"));
        // Lot N: nested groups in parentheses, every leaf kind, incomplete
        // leaves skipped (Aurelia-style And(Or(a, b), c)).
        let mut inner = DependencyGroup::new(LogicalOperator::Or);
        inner.push_leaf(Dependency::new_flag("DataVersion", "Standard"));
        inner.push_leaf(Dependency::new_flag("DataVersion", "Undelayed"));
        inner.push_leaf(Dependency::new_flag("", ""));
        let mut g = DependencyGroup::new(LogicalOperator::And);
        g.push_group(inner);
        g.push_leaf(Dependency::new_flag("DataPreset", "Aurelia"));
        g.push_leaf(Dependency::new_game("1.6.1170"));
        g.push_leaf(Dependency::new_fomm("0.13"));
        g.push_group(DependencyGroup::new(LogicalOperator::Or));
        assert_eq!(
            sentence(&t, &g, "THEN"),
            "IF (flag DataVersion = Standard OR flag DataVersion = Undelayed) AND flag DataPreset = Aurelia \
             AND game version ≥ 1.6.1170 AND mod manager version ≥ 0.13 THEN"
        );
        // Only incomplete leaves: no condition.
        let mut blank = DependencyGroup::new(LogicalOperator::And);
        blank.push_leaf(Dependency::new_game(""));
        assert!(!sentence(&t, &blank, "x").starts_with("IF"));
        // `committed` drops the incomplete leaves at every depth, keeps the groups.
        let c = committed(&g);
        assert_eq!(c.leaf_count(), 5);
        assert_eq!(c.items.len(), 5);
        assert_eq!(c.depth(), 1);
        app.i18n.set_locale("fra");
        let t = Texts::new(&app.i18n);
        assert!(!t.s_if.starts_with("condeditor-"));
        let v = resolve_builder(&app.ximod, BuilderTarget::PluginPattern(0, 0, 1, 0), &app.i18n).unwrap();
        assert!(!v.then.contains("condeditor-"), "{}", v.then);
        assert_eq!(v.patterns, Some((1, "Optional".into())));
        // A missing pattern resolves to "append".
        let v = resolve_builder(&app.ximod, BuilderTarget::PluginPattern(0, 0, 0, 3), &app.i18n).unwrap();
        assert_eq!(v.target, BuilderTarget::PluginPattern(0, 0, 0, 0));
        assert!(resolve_builder(&app.ximod, BuilderTarget::ConditionalSet(4), &app.i18n).is_none());
    }

    /// The window lays out headless with and without a builder target and a
    /// plain render never dirties the project.
    #[test]
    fn window_renders_headless() {
        let mut app = app();
        app.condition_editor.open = true;
        let ctx = egui::Context::default();
        for target in [
            Target::Info,
            Target::Step(1),
            Target::Plugin(0, 0, 1),
            Target::CondSet(0),
            Target::Required,
        ] {
            app.select_target(target);
            let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_condition_editor(ctx));
        }
        assert!(!app.project_modified);
        assert!(app.condition_editor.cache.is_some());
        // The draft follows the selected node.
        app.select_target(Target::Step(1));
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_condition_editor(ctx));
        let d = app.condition_editor.draft.as_ref().unwrap();
        assert_eq!(d.target, BuilderTarget::StepVisibility(1));
        assert_eq!(d.rows.items.len(), 2);
        // A nested group in the edited node lays out too.
        let mut nested = DependencyGroup::new(LogicalOperator::Or);
        nested.push_leaf(Dependency::new_flag("res", "2K"));
        nested.push_leaf(Dependency::new_fomm("0.13"));
        app.ximod.steps[1].visibility.push_group(nested);
        app.project_modified = false;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_condition_editor(ctx));
        let d = app.condition_editor.draft.as_ref().unwrap();
        assert_eq!(d.rows.items.len(), 3);
        assert!(!app.project_modified);
        // Closing drops the state.
        app.condition_editor.open = false;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_condition_editor(ctx));
        let mut ed = std::mem::take(&mut app.condition_editor);
        assert!(ed.show(&ctx, &app.ximod, &app.i18n, &app.autocomplete, None).is_empty());
        assert!(ed.cache.is_none());
        assert!(ed.draft.is_none());
    }

    /// ↑ ↓ move the cursor over flags and their lines, ← → fold / unfold a
    /// flag (or climb to it), Enter selects the node of the row.
    #[test]
    fn keyboard_navigation_in_the_flag_list() {
        use crate::ui::main_window::Tab;
        let mut app = app();
        app.condition_editor.open = true;
        let ctx = egui::Context::default();
        let frame = |app: &mut XimodApp, keys: &[egui::Key]| {
            let events = keys
                .iter()
                .map(|&key| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                })
                .collect();
            let _ = ctx.run(
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                |ctx| app.render_condition_editor(ctx),
            );
            // Let the fold animation (0.1 s at ~60 fps) finish.
            for _ in 0..12 {
                let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_condition_editor(ctx));
            }
        };
        frame(&mut app, &[]);
        let flags: Vec<String> = app
            .condition_editor
            .cache
            .as_ref()
            .unwrap()
            .1
            .flags
            .iter()
            .map(|f| f.name.clone())
            .collect();
        assert_eq!(flags, ["hd", "res"]);
        // "hd" is an orphan: open by default, so ↓ lands on its first line.
        frame(&mut app, &[egui::Key::ArrowDown]);
        assert_eq!(app.condition_editor.nav.cursor.as_deref(), Some("flag:hd/0"));
        // Enter shows that line's node (the step testing hd) in the inspector.
        app.select_target(Target::Info);
        frame(&mut app, &[egui::Key::Enter]);
        assert_eq!(app.current_tab, Tab::Steps);
        assert_eq!(app.selection.step, Some(1));
        // ← climbs to the flag, ← again folds it.
        frame(&mut app, &[egui::Key::ArrowLeft]);
        assert_eq!(app.condition_editor.nav.cursor.as_deref(), Some("flag:hd"));
        frame(&mut app, &[egui::Key::ArrowLeft]);
        frame(&mut app, &[egui::Key::ArrowDown]);
        assert_eq!(app.condition_editor.nav.cursor.as_deref(), Some("flag:res"));
        // → unfolds "res", ↓ enters it, Enter selects the option setting it.
        frame(&mut app, &[egui::Key::ArrowRight]);
        frame(&mut app, &[egui::Key::ArrowDown]);
        assert_eq!(app.condition_editor.nav.cursor.as_deref(), Some("flag:res/0"));
        frame(&mut app, &[egui::Key::Enter]);
        assert_eq!(app.selection.plugin, Some(0));
        // Enter on a flag header selects its first site.
        frame(&mut app, &[egui::Key::ArrowUp]);
        app.select_target(Target::Info);
        frame(&mut app, &[egui::Key::Enter]);
        assert_eq!(app.current_tab, Tab::Steps);
        assert_eq!(app.selection.plugin, Some(0));
        // The context menus' "Expand all" / "Collapse all" go through the
        // same fold queue: everything open, then everything closed.
        app.condition_editor.nav.request(FoldCmd::All(true));
        frame(&mut app, &[]);
        frame(&mut app, &[egui::Key::ArrowDown]);
        assert_eq!(app.condition_editor.nav.cursor.as_deref(), Some("flag:res/0"));
        app.condition_editor.nav.request(FoldCmd::All(false));
        frame(&mut app, &[]);
        frame(&mut app, &[egui::Key::ArrowLeft]);
        frame(&mut app, &[egui::Key::ArrowUp]);
        assert_eq!(app.condition_editor.nav.cursor.as_deref(), Some("flag:hd"));
        frame(&mut app, &[egui::Key::ArrowDown]);
        assert_eq!(app.condition_editor.nav.cursor.as_deref(), Some("flag:res"));
    }
}
