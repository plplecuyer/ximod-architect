//! Dependency (file / flag / version condition) editor shared by the step
//! visibility, option dependency-pattern, conditional-install and mod
//! requirement panels.
//!
//! One rendering for the four places a [`DependencyGroup`] is edited: a
//! "Dependencies" sub-header, the list of the group's items, then a row
//! with the kind combo (flag / file / game version / mod manager version),
//! the name and value fields (with autocompletion) and the Add / Add group /
//! Remove buttons.
//!
//! A flat condition looks exactly as before lot N: one selectable line per
//! leaf. A nested group (lot N) is drawn in place as an indented frame with
//! its own operator combo, one editable row per leaf (kind, name, value,
//! remove), its nested groups recursively, and "Add condition" / "Add group"
//! / "Remove group" buttons. The top-level selection (`current_*_dep_index`
//! on `XimodApp`) is an index into the top-level items, leaf or group.
//!
//! The caller draws the top-level operator combo above the editor, as it
//! always did.

use crate::models::*;
use crate::ui::components::*;
use crate::ui::main_window::XimodApp;
use crate::ui::theme::icon;
use eframe::egui;
use std::rc::Rc;

/// Which dependency group an editor edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DepTarget {
    /// A step's `<visible>` conditions.
    StepVisibility { step: usize },
    /// One dependency pattern of an option (indices are assumed valid).
    PluginPattern {
        step: usize,
        group: usize,
        plugin: usize,
        pattern: usize,
    },
    /// The conditions of one conditional-install pattern.
    Conditional { pattern: usize },
    /// The mod-wide requirements (`<moduleDependencies>`, Info tab).
    Module,
}

/// Per-instance widget ids of a dependency editor.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DepEditorIds {
    /// `id_salt` of the list's scroll area.
    pub list: &'static str,
    /// Id of the flag / file type combo.
    pub type_combo: &'static str,
    /// Ids of the name / value autocomplete fields.
    pub ac_name: &'static str,
    pub ac_value: &'static str,
}

/// Translated texts of the editor.
pub(crate) struct DepEditorLabels {
    pub hdr_dependencies: String,
    pub btn_add_dep: String,
    pub btn_remove_dep: String,
    pub btn_add_condition: String,
    pub btn_add_group: String,
    pub btn_remove_group: String,
    pub dep_type_flag: String,
    pub dep_type_file: String,
    pub dep_type_game: String,
    pub dep_type_fomm: String,
    pub op_and: String,
    pub op_or: String,
    pub hint_group: String,
}

impl DepEditorLabels {
    pub(crate) fn new(i18n: &crate::i18n::I18n) -> Self {
        Self {
            hdr_dependencies: i18n.t("label-dependencies"),
            btn_add_dep: i18n.t("btn-add-dependency"),
            btn_remove_dep: i18n.t("btn-remove-dependency"),
            btn_add_condition: i18n.t("btn-add-condition"),
            btn_add_group: i18n.t("btn-add-group-cond"),
            btn_remove_group: i18n.t("btn-remove-group-cond"),
            dep_type_flag: i18n.t("dep-type-flag"),
            dep_type_file: i18n.t("dep-type-file"),
            dep_type_game: i18n.t("dep-type-game"),
            dep_type_fomm: i18n.t("dep-type-fomm"),
            op_and: i18n.t("op-and"),
            op_or: i18n.t("op-or"),
            hint_group: i18n.t("dep-group-hint"),
        }
    }

    /// Display label of a leaf kind.
    fn kind_label(&self, kind: DependencyType) -> &str {
        match kind {
            DependencyType::Flag => &self.dep_type_flag,
            DependencyType::File => &self.dep_type_file,
            DependencyType::Game => &self.dep_type_game,
            DependencyType::Fomm => &self.dep_type_fomm,
        }
    }

    fn op_label(&self, op: LogicalOperator) -> &str {
        match op {
            LogicalOperator::And => &self.op_and,
            LogicalOperator::Or => &self.op_or,
        }
    }
}

/// Autocomplete sources of the leaf rows.
pub(crate) struct DepCandidates {
    /// Names (flags and files) for the name field.
    pub names: Rc<Vec<String>>,
    /// Values for a flag leaf.
    pub flag_values: Rc<Vec<String>>,
    /// Values for a file leaf (the file states).
    pub file_states: Rc<Vec<String>>,
    /// Nothing to suggest (version leaves).
    pub none: Rc<Vec<String>>,
}

impl DepCandidates {
    pub(crate) fn new(names: Rc<Vec<String>>, flag_values: Rc<Vec<String>>) -> Self {
        Self {
            names,
            flag_values,
            file_states: Rc::new(FileState::variants().iter().map(|s| s.as_str().to_string()).collect()),
            none: Rc::new(Vec::new()),
        }
    }

    fn values_for(&self, kind: DependencyType) -> &Rc<Vec<String>> {
        match kind {
            DependencyType::Flag => &self.flag_values,
            DependencyType::File => &self.file_states,
            DependencyType::Game | DependencyType::Fomm => &self.none,
        }
    }
}

/// Mutable state of one editor: the group, its top-level selection and the
/// draft (kind / name / value) being typed.
struct DepEditorState<'a> {
    group: &'a mut DependencyGroup,
    selected: &'a mut Option<usize>,
    temp_type: &'a mut String,
    temp_name: &'a mut String,
    temp_value: &'a mut String,
}

/// A kind combo for a leaf (draft row or nested row). Returns the newly
/// chosen kind, if any.
fn kind_combo(
    ui: &mut egui::Ui,
    id_salt: impl std::hash::Hash,
    current: DependencyType,
    labels: &DepEditorLabels,
) -> Option<DependencyType> {
    let mut chosen = None;
    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(labels.kind_label(current))
        .show_ui(ui, |ui| {
            for kind in DependencyType::variants() {
                if ui
                    .selectable_label(current == *kind, labels.kind_label(*kind))
                    .clicked()
                    && current != *kind
                {
                    chosen = Some(*kind);
                }
            }
        });
    chosen
}

/// An operator combo for a nested group. Returns the newly chosen operator.
fn operator_combo(
    ui: &mut egui::Ui,
    id_salt: impl std::hash::Hash,
    current: LogicalOperator,
    labels: &DepEditorLabels,
) -> Option<LogicalOperator> {
    let mut chosen = None;
    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(labels.op_label(current))
        .show_ui(ui, |ui| {
            for op in LogicalOperator::variants() {
                if ui.selectable_label(current == *op, labels.op_label(*op)).clicked() && current != *op {
                    chosen = Some(*op);
                }
            }
        });
    chosen
}

/// Name / value fields of a leaf, with autocompletion (the name field is
/// hidden for version leaves, which have none). `id_prefix` makes the
/// autocomplete popups unique per row. Returns whether a field has focus.
fn leaf_fields(
    ui: &mut egui::Ui,
    id_prefix: &str,
    kind: DependencyType,
    name: &mut String,
    value: &mut String,
    cand: &DepCandidates,
) -> bool {
    let mut focus = false;
    if matches!(kind, DependencyType::Flag | DependencyType::File) {
        let r = autocomplete_edit(ui, &format!("{id_prefix}_name"), name, &cand.names);
        focus |= r.has_focus();
        ui.label("=");
    } else {
        ui.label("≥");
    }
    let r = autocomplete_edit(ui, &format!("{id_prefix}_value"), value, cand.values_for(kind));
    focus |= r.has_focus();
    focus
}

/// What a nested group's frame reports to its parent.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GroupOutcome {
    /// The group's content changed this frame.
    pub modified: bool,
    /// The "Remove group" button was clicked (the parent removes it).
    pub remove_self: bool,
    /// A text field of the group (any depth) has keyboard focus.
    pub has_focus: bool,
}

/// One nested group, drawn in place: operator, buttons, then its items
/// (editable leaf rows, nested groups recursively). `path` is the index
/// path from the top-level group (for stable widget ids under `id_base`).
pub(crate) fn render_nested_group(
    ui: &mut egui::Ui,
    group: &mut DependencyGroup,
    path: &[usize],
    id_base: &str,
    labels: &DepEditorLabels,
    cand: &DepCandidates,
) -> GroupOutcome {
    render_nested_group_inner(ui, group, path, id_base, labels, cand)
}

fn render_nested_group_inner(
    ui: &mut egui::Ui,
    group: &mut DependencyGroup,
    path: &[usize],
    id_base: &str,
    labels: &DepEditorLabels,
    cand: &DepCandidates,
) -> GroupOutcome {
    let mut out = GroupOutcome::default();
    let path_id = path.iter().map(|i| i.to_string()).collect::<Vec<_>>().join("_");
    let prefix = format!("{id_base}_g{path_id}");
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            if let Some(op) = operator_combo(ui, format!("{prefix}_op"), group.operator, labels) {
                group.operator = op;
                out.modified = true;
            }
            ui.label(egui::RichText::new("ⓘ").weak())
                .on_hover_text(&labels.hint_group);
            if ui.small_button(&labels.btn_add_condition).clicked() {
                group.push_leaf(Dependency::new_flag("", ""));
                out.modified = true;
            }
            if ui.small_button(&labels.btn_add_group).clicked() {
                group.push_group(DependencyGroup::new(LogicalOperator::And));
                out.modified = true;
            }
            if ui.small_button(&labels.btn_remove_group).clicked() {
                out.remove_self = true;
            }
        });
        let mut remove: Option<usize> = None;
        for (idx, item) in group.items.iter_mut().enumerate() {
            let mut sub_path = path.to_vec();
            sub_path.push(idx);
            match item {
                DependencyItem::Leaf(dep) => {
                    ui.horizontal(|ui| {
                        let kind = dep.kind();
                        let row_id = format!("{prefix}_l{idx}");
                        if let Some(k) = kind_combo(ui, format!("{row_id}_kind"), kind, labels) {
                            dep.dep_type = k.as_str().to_string();
                            if matches!(k, DependencyType::Game | DependencyType::Fomm) {
                                dep.name.clear();
                            }
                            out.modified = true;
                        }
                        let before = (dep.name.clone(), dep.value.clone());
                        out.has_focus |= leaf_fields(ui, &row_id, dep.kind(), &mut dep.name, &mut dep.value, cand);
                        if (dep.name.as_str(), dep.value.as_str()) != (before.0.as_str(), before.1.as_str()) {
                            out.modified = true;
                        }
                        if ui
                            .add(icon_button(icon::DELETE))
                            .on_hover_text(&labels.btn_remove_dep)
                            .clicked()
                        {
                            remove = Some(idx);
                        }
                    });
                }
                DependencyItem::Group(sub) => {
                    let r = render_nested_group_inner(ui, sub, &sub_path, id_base, labels, cand);
                    out.modified |= r.modified;
                    out.has_focus |= r.has_focus;
                    if r.remove_self {
                        remove = Some(idx);
                    }
                }
            }
        }
        if let Some(idx) = remove
            && idx < group.items.len()
        {
            group.items.remove(idx);
            out.modified = true;
        }
    });
    out
}

/// Draw the sub-header, the list and the add / remove row. Returns whether
/// the group changed.
fn dependency_editor(
    ui: &mut egui::Ui,
    ids: &DepEditorIds,
    labels: &DepEditorLabels,
    cand: &DepCandidates,
    st: DepEditorState<'_>,
) -> bool {
    let mut modified = false;

    subsection_header(ui, &labels.hdr_dependencies);

    // Nested groups take room: give the list more height when there are any.
    let max_height = if st.group.has_groups() { 260.0 } else { 100.0 };
    egui::ScrollArea::vertical()
        .id_salt(ids.list)
        .max_height(max_height)
        .show(ui, |ui| {
            let mut remove: Option<usize> = None;
            for idx in 0..st.group.items.len() {
                let selected = *st.selected == Some(idx);
                match &mut st.group.items[idx] {
                    DependencyItem::Leaf(dep) => {
                        if ui.selectable_label(selected, dep.display_name()).clicked() {
                            *st.selected = Some(idx);
                        }
                    }
                    DependencyItem::Group(sub) => {
                        let r = render_nested_group_inner(ui, sub, &[idx], ids.list, labels, cand);
                        modified |= r.modified;
                        if r.remove_self {
                            remove = Some(idx);
                        }
                    }
                }
            }
            if let Some(idx) = remove
                && idx < st.group.items.len()
            {
                st.group.items.remove(idx);
                *st.selected = None;
                modified = true;
            }
        });

    let draft_kind = DependencyType::from_str(st.temp_type);

    ui.horizontal(|ui| {
        if let Some(k) = kind_combo(ui, ids.type_combo, draft_kind, labels) {
            *st.temp_type = k.as_str().to_string();
        }
        let draft_kind = DependencyType::from_str(st.temp_type);
        // The draft fields keep their historical ids (`ac_*`), so the
        // autocomplete popups behave exactly as before.
        if matches!(draft_kind, DependencyType::Flag | DependencyType::File) {
            autocomplete_edit(ui, ids.ac_name, st.temp_name, &cand.names);
            ui.label("=");
        } else {
            ui.label("≥");
        }
        autocomplete_edit(ui, ids.ac_value, st.temp_value, cand.values_for(draft_kind));

        let draft = Dependency {
            dep_type: st.temp_type.clone(),
            name: if draft_kind == DependencyType::Game || draft_kind == DependencyType::Fomm {
                String::new()
            } else {
                st.temp_name.clone()
            },
            value: st.temp_value.clone(),
        };
        if ui.button(&labels.btn_add_dep).clicked() && draft.is_complete() {
            st.group.push_leaf(draft);
            st.temp_name.clear();
            st.temp_value.clear();
            modified = true;
        }

        if ui
            .button(&labels.btn_add_group)
            .on_hover_text(&labels.hint_group)
            .clicked()
        {
            st.group.push_group(DependencyGroup::new(LogicalOperator::And));
            modified = true;
        }

        let can_remove = st.selected.is_some();
        if ui
            .add_enabled(can_remove, egui::Button::new(&labels.btn_remove_dep))
            .clicked()
            && let Some(dep_idx) = *st.selected
        {
            if dep_idx < st.group.items.len() {
                st.group.items.remove(dep_idx);
                modified = true;
            }
            *st.selected = None;
        }
    });

    modified
}

impl XimodApp {
    /// The dependency group, its selection index and its draft fields for a
    /// `DepTarget`.
    fn dep_target_mut(&mut self, target: DepTarget) -> DepEditorState<'_> {
        match target {
            DepTarget::StepVisibility { step } => DepEditorState {
                group: &mut self.ximod.steps[step].visibility,
                selected: &mut self.current_visibility_dep_index,
                temp_type: &mut self.temp_vdep_type,
                temp_name: &mut self.temp_vdep_name,
                temp_value: &mut self.temp_vdep_value,
            },
            DepTarget::PluginPattern {
                step,
                group,
                plugin,
                pattern,
            } => DepEditorState {
                group: &mut self.ximod.steps[step].plugin_groups[group].plugins[plugin].dependency_patterns[pattern]
                    .condition,
                selected: &mut self.current_plugin_dep_index,
                temp_type: &mut self.temp_pdep_type,
                temp_name: &mut self.temp_pdep_name,
                temp_value: &mut self.temp_pdep_value,
            },
            DepTarget::Conditional { pattern } => DepEditorState {
                group: &mut self.ximod.conditional_files[pattern].condition,
                selected: &mut self.current_dependency_index,
                temp_type: &mut self.temp_dep_type,
                temp_name: &mut self.temp_dep_name,
                temp_value: &mut self.temp_dep_value,
            },
            // An empty group is written as "no <moduleDependencies>", so
            // materialising the container here changes nothing on save.
            DepTarget::Module => DepEditorState {
                group: self.ximod.module_dependencies.get_or_insert_with(Default::default),
                selected: &mut self.current_module_dep_index,
                temp_type: &mut self.temp_mdep_type,
                temp_name: &mut self.temp_mdep_name,
                temp_value: &mut self.temp_mdep_value,
            },
        }
    }

    /// Render the "Dependencies" sub-section for `target`: header, list
    /// (nested groups in place) and the add / add group / remove row. Names
    /// and flag values are autocompleted from the whole project.
    pub(crate) fn render_dependency_editor(&mut self, ui: &mut egui::Ui, target: DepTarget, ids: DepEditorIds) {
        let labels = DepEditorLabels::new(&self.i18n);
        let ac = self.autocomplete();
        let cand = DepCandidates::new(ac.dep_names.clone(), ac.flag_values.clone());

        let modified = {
            let st = self.dep_target_mut(target);
            dependency_editor(ui, &ids, &labels, &cand, st)
        };
        if modified {
            self.mark_modified();
        }
    }

    /// "Plugin dependencies" panel: edits a plugin's dynamic type
    /// (`dependency_patterns`). Each pattern gives the option a `Type name`
    /// (Optional / Required / Recommended / NotUsable / CouldBeUsable) that
    /// applies when its file/flag conditions are met, otherwise the plugin's
    /// Default Type is used. This mirrors the "Plugin dependencies" tab of the
    /// original FOMOD Creation Tool.
    pub(crate) fn render_plugin_dependencies(
        &mut self,
        ui: &mut egui::Ui,
        step_idx: usize,
        group_idx: usize,
        plugin_idx: usize,
    ) {
        let label_operator = self.i18n.t("label-pattern-operator");
        let hint_operator = self.i18n.t("hint-operator");
        let label_type_name = self.i18n.t("label-pattern-type");
        let btn_new_pattern = self.i18n.t("btn-add-pattern");
        let btn_delete_pattern = self.i18n.t("btn-remove-pattern");

        let pattern_count = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
            .dependency_patterns
            .len();

        let pattern_labels: Vec<String> = (0..pattern_count)
            .map(|idx| self.i18n.t_num("pattern-label", (idx + 1) as i64))
            .collect();

        ui.horizontal(|ui| {
            for (idx, label) in pattern_labels.iter().enumerate().take(pattern_count) {
                let selected = self.current_plugin_pattern_index == Some(idx);
                if ui.selectable_label(selected, label).clicked() {
                    self.current_plugin_pattern_index = Some(idx);
                    self.current_plugin_dep_index = None;
                }
            }

            if ui.button(&btn_new_pattern).clicked() {
                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                    .dependency_patterns
                    .push(DependencyPattern::new());
                let n = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                    .dependency_patterns
                    .len();
                self.current_plugin_pattern_index = Some(n - 1);
                self.current_plugin_dep_index = None;
                self.mark_modified();
            }

            let can_remove = self.current_plugin_pattern_index.is_some();
            if ui
                .add_enabled(can_remove, egui::Button::new(&btn_delete_pattern))
                .clicked()
                && let Some(idx) = self.current_plugin_pattern_index
            {
                let patterns =
                    &mut self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].dependency_patterns;
                if idx < patterns.len() {
                    patterns.remove(idx);
                    self.current_plugin_pattern_index = if patterns.is_empty() {
                        None
                    } else {
                        Some(idx.saturating_sub(1).min(patterns.len() - 1))
                    };
                    self.current_plugin_dep_index = None;
                    self.mark_modified();
                }
            }
        });

        let pattern_idx = match self.current_plugin_pattern_index {
            Some(i) if i < pattern_count => i,
            _ => return,
        };

        ui.separator();

        // Operator (And / Or) and Type name (the type applied when the pattern matches).
        let current_op = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].dependency_patterns
            [pattern_idx]
            .condition
            .operator;
        ui.horizontal(|ui| {
            ui.label(&label_operator);
            egui::ComboBox::from_id_salt("pdep_operator")
                .selected_text(crate::ui::labels::operator(&self.i18n, current_op))
                .show_ui(ui, |ui| {
                    for op in LogicalOperator::variants() {
                        let label = crate::ui::labels::operator(&self.i18n, *op);
                        if ui.selectable_label(current_op == *op, label).clicked() {
                            self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                                .dependency_patterns[pattern_idx]
                                .condition
                                .operator = *op;
                            self.mark_modified();
                        }
                    }
                })
                .response
                .on_hover_text(&hint_operator);
        });

        let current_type = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].dependency_patterns
            [pattern_idx]
            .pattern_type
            .clone();
        ui.horizontal(|ui| {
            ui.label(&label_type_name);
            egui::ComboBox::from_id_salt("pdep_type_name")
                .selected_text(crate::ui::labels::plugin_type_name(&self.i18n, &current_type))
                .show_ui(ui, |ui| {
                    for pt in PluginType::variants() {
                        let name = pt.as_str();
                        let label = crate::ui::labels::plugin_type(&self.i18n, *pt);
                        let hint = crate::ui::labels::plugin_type_hint(&self.i18n, *pt);
                        if ui
                            .selectable_label(current_type == name, label)
                            .on_hover_text(hint)
                            .clicked()
                        {
                            self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                                .dependency_patterns[pattern_idx]
                                .pattern_type = name.to_string();
                            self.mark_modified();
                        }
                    }
                });
        });

        self.render_dependency_editor(
            ui,
            DepTarget::PluginPattern {
                step: step_idx,
                group: group_idx,
                plugin: plugin_idx,
                pattern: pattern_idx,
            },
            DepEditorIds {
                list: "pdep_deps_list",
                type_combo: "pdep_dep_type",
                ac_name: "ac_pdep_name",
                ac_value: "ac_pdep_value",
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let mut step = Step::new("S");
        let mut group = PluginGroup::new("G", SelectionType::SelectAny);
        let mut p = Plugin::new("P");
        p.dependency_patterns.push(DependencyPattern::new());
        group.plugins.push(p);
        step.plugin_groups.push(group);
        app.ximod.steps.push(step);
        app.ximod.conditional_files.push(ConditionalFileSet::new());
        app
    }

    #[test]
    fn targets_resolve_to_their_lists_and_drafts() {
        let mut app = app();
        app.temp_vdep_name = "v".into();
        app.temp_pdep_name = "p".into();
        app.temp_dep_name = "c".into();
        let st = app.dep_target_mut(DepTarget::StepVisibility { step: 0 });
        assert_eq!(st.temp_name, "v");
        let st = app.dep_target_mut(DepTarget::PluginPattern {
            step: 0,
            group: 0,
            plugin: 0,
            pattern: 0,
        });
        assert_eq!(st.temp_name, "p");
        let st = app.dep_target_mut(DepTarget::Conditional { pattern: 0 });
        assert_eq!(st.temp_name, "c");
        assert!(st.group.is_empty());
        app.temp_mdep_name = "m".into();
        let st = app.dep_target_mut(DepTarget::Module);
        assert_eq!(st.temp_name, "m");
        st.group.push_leaf(Dependency::new_file("Skyrim.esm", "Active"));
        assert!(app.ximod.has_module_dependencies());
    }

    /// The mod-requirements editor lays out headless on a project without
    /// any `<moduleDependencies>` yet, and does not dirty it.
    #[test]
    fn module_editor_renders_headless() {
        let mut app = app();
        assert!(app.ximod.module_dependencies.is_none());
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                app.render_dependency_editor(
                    ui,
                    DepTarget::Module,
                    DepEditorIds {
                        list: "mdep_deps_list",
                        type_combo: "mdep_dep_type",
                        ac_name: "ac_mdep_name",
                        ac_value: "ac_mdep_value",
                    },
                );
                app.render_info_tab(ui);
            });
        });
        assert!(!app.project_modified);
        assert!(!app.ximod.has_module_dependencies());
    }

    /// The three editors lay out headless and a plain render never dirties
    /// the project.
    #[test]
    fn editors_render_headless() {
        let mut app = app();
        app.current_plugin_pattern_index = Some(0);
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                app.render_plugin_dependencies(ui, 0, 0, 0);
                app.render_dependency_editor(
                    ui,
                    DepTarget::StepVisibility { step: 0 },
                    DepEditorIds {
                        list: "vis_deps_list",
                        type_combo: "vis_dep_type",
                        ac_name: "ac_vdep_name",
                        ac_value: "ac_vdep_value",
                    },
                );
                app.render_dependency_editor(
                    ui,
                    DepTarget::Conditional { pattern: 0 },
                    DepEditorIds {
                        list: "cond_deps_list",
                        type_combo: "dep_type",
                        ac_name: "ac_dep_name",
                        ac_value: "ac_dep_value",
                    },
                );
            });
        });
        assert!(!app.project_modified);
    }

    /// Lot N: a visibility group with a nested group (Aurelia style) and
    /// every leaf kind lays out headless, keeps its shape and does not
    /// dirty the project; the labels resolve in both reference locales.
    #[test]
    fn nested_group_renders_headless() {
        let mut app = app();
        let mut inner = DependencyGroup::new(LogicalOperator::Or);
        inner.push_leaf(Dependency::new_flag("DataVersion", "Standard"));
        inner.push_leaf(Dependency::new_flag("DataVersion", "Undelayed"));
        let mut deeper = DependencyGroup::new(LogicalOperator::And);
        deeper.push_leaf(Dependency::new_game("1.6"));
        deeper.push_leaf(Dependency::new_fomm("0.13"));
        deeper.push_leaf(Dependency::new_file("Skyrim.esm", "Active"));
        inner.push_group(deeper);
        app.ximod.steps[0].visibility.push_group(inner);
        app.ximod.steps[0]
            .visibility
            .push_leaf(Dependency::new_flag("DataPreset", "Aurelia"));
        let before = app.ximod.steps[0].visibility.clone();
        let ctx = egui::Context::default();
        for locale in ["eng", "fra"] {
            app.i18n.set_locale(locale);
            let labels = DepEditorLabels::new(&app.i18n);
            for text in [
                &labels.btn_add_group,
                &labels.btn_remove_group,
                &labels.dep_type_game,
                &labels.dep_type_fomm,
                &labels.hint_group,
            ] {
                assert!(
                    !text.starts_with("btn-") && !text.starts_with("dep-"),
                    "{locale}: {text}"
                );
            }
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    app.render_dependency_editor(
                        ui,
                        DepTarget::StepVisibility { step: 0 },
                        DepEditorIds {
                            list: "vis_deps_list",
                            type_combo: "vis_dep_type",
                            ac_name: "ac_vdep_name",
                            ac_value: "ac_vdep_value",
                        },
                    );
                });
            });
        }
        assert!(!app.project_modified);
        assert_eq!(app.ximod.steps[0].visibility, before);
        assert_eq!(app.ximod.steps[0].visibility.depth(), 2);
    }
}
