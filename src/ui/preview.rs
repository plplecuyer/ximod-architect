//! Real-time preview of the FOMOD installer.
//!
//! Opens a window that simulates, as faithfully as possible, what a mod manager
//! (Vortex, MO2, …) shows when the end user runs the generated installer:
//!
//!   * steps shown in order, skipping those whose conditional visibility is not
//!     satisfied by the current selections/flags;
//!   * each group rendered with the widget matching its selection type
//!     (radios for SelectExactlyOne/AtMostOne, checkboxes otherwise), honouring
//!     each plugin's effective type (Required forced on, NotUsable forced off,
//!     Recommended checked by default, …), including dynamic typeDescriptor
//!     patterns evaluated live;
//!   * Back / Next / Install navigation with per-step validity checks;
//!   * a final summary listing every file that would be installed.
//!
//! File-type dependencies (Active/Inactive/Missing of a game file) cannot be
//! known outside a real install, so an "assumptions" panel lets the author set
//! the supposed state of each referenced file; the simulation honours it.

use std::collections::{BTreeMap, HashMap};

use eframe::egui::{self, Color32, RichText};

use crate::models::{
    Dependency, FileState, InstallFile, LogicalOperator, Plugin, PluginGroup, PluginType,
    SelectionType, Ximod,
};
use crate::ui::components::ImageDisplay;
use crate::ui::main_window::XimodApp;

/// (step, group, plugin) index of a plugin in the project tree.
pub type SelKey = (usize, usize, usize);

/// Runtime state of one preview session.
#[derive(Default)]
pub struct PreviewState {
    /// Which plugins are currently selected.
    pub selections: HashMap<SelKey, bool>,
    /// Assumed state of each game file referenced by a `file` dependency.
    pub file_states: BTreeMap<String, FileState>,
    /// Index (into `ximod.steps`) of the step currently shown.
    pub cursor: usize,
    /// Stack of visited step indices, for the Back button.
    pub history: Vec<usize>,
    /// True when the final install summary is shown.
    pub finished: bool,
    /// Whether the file-assumptions panel is expanded.
    pub show_assumptions: bool,
    /// Plugin whose description/image is shown in the detail pane.
    pub focused: Option<SelKey>,
}

// --------------------------------------------------------------------------- //
// Condition evaluation
// --------------------------------------------------------------------------- //
fn dep_satisfied(
    dep: &Dependency,
    flags: &HashMap<String, String>,
    files: &BTreeMap<String, FileState>,
) -> bool {
    if dep.dep_type == "file" {
        let st = files.get(&dep.name).copied().unwrap_or(FileState::Active);
        st.as_str().eq_ignore_ascii_case(&dep.value)
    } else {
        // flag: satisfied when the flag currently equals the expected value;
        // an unset flag counts as the empty string.
        match flags.get(&dep.name) {
            Some(v) => v == &dep.value,
            None => dep.value.is_empty(),
        }
    }
}

fn deps_satisfied(
    op: LogicalOperator,
    deps: &[Dependency],
    flags: &HashMap<String, String>,
    files: &BTreeMap<String, FileState>,
) -> bool {
    if deps.is_empty() {
        return true; // no condition → always satisfied (e.g. always-visible step)
    }
    match op {
        LogicalOperator::Or => deps.iter().any(|d| dep_satisfied(d, flags, files)),
        _ => deps.iter().all(|d| dep_satisfied(d, flags, files)),
    }
}

/// Effective type of a plugin: the first matching `dependency_patterns`
/// (typeDescriptor) entry, otherwise its default type.
fn effective_type(
    plugin: &Plugin,
    flags: &HashMap<String, String>,
    files: &BTreeMap<String, FileState>,
) -> PluginType {
    for pat in &plugin.dependency_patterns {
        if deps_satisfied(pat.operator, &pat.dependencies, flags, files) {
            return PluginType::from_str(&pat.pattern_type);
        }
    }
    plugin.default_type
}

/// Forward pass over the steps: returns the accumulated flags and the list of
/// visible step indices. A step's flags come from its selected plugins and only
/// count when the step itself is visible given the flags accumulated so far.
fn compute(
    ximod: &Ximod,
    selections: &HashMap<SelKey, bool>,
    files: &BTreeMap<String, FileState>,
) -> (HashMap<String, String>, Vec<usize>) {
    let mut flags: HashMap<String, String> = HashMap::new();
    let mut visible = Vec::new();
    for (si, step) in ximod.steps.iter().enumerate() {
        if !deps_satisfied(
            step.visibility_operator,
            &step.visibility_dependencies,
            &flags,
            files,
        ) {
            continue;
        }
        visible.push(si);
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
    (flags, visible)
}

/// Files that would be installed given the current selections.
fn compute_install(
    ximod: &Ximod,
    selections: &HashMap<SelKey, bool>,
    files: &BTreeMap<String, FileState>,
) -> Vec<InstallFile> {
    let (flags, visible) = compute(ximod, selections, files);
    let mut out: Vec<InstallFile> = ximod.required_files.clone();
    for &si in &visible {
        let step = &ximod.steps[si];
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            for (pi, plugin) in group.plugins.iter().enumerate() {
                if *selections.get(&(si, gi, pi)).unwrap_or(&false) {
                    out.extend(plugin.files.iter().cloned());
                }
            }
        }
    }
    for cfs in &ximod.conditional_files {
        if deps_satisfied(cfs.operator, &cfs.dependencies, &flags, files) {
            out.extend(cfs.files.iter().cloned());
        }
    }
    out.sort_by(|a, b| a.priority.cmp(&b.priority).then(a.destination.cmp(&b.destination)));
    out
}

/// Default selection for one group, seeded from the plugins' default types.
fn default_selections(
    ximod: &Ximod,
    files: &BTreeMap<String, FileState>,
) -> HashMap<SelKey, bool> {
    let empty: HashMap<String, String> = HashMap::new();
    let mut sel = HashMap::new();
    for (si, step) in ximod.steps.iter().enumerate() {
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            let types: Vec<PluginType> = group
                .plugins
                .iter()
                .map(|p| effective_type(p, &empty, files))
                .collect();
            let n = group.plugins.len();
            match group.selection_type {
                SelectionType::SelectAll => {
                    for pi in 0..n {
                        sel.insert((si, gi, pi), types[pi] != PluginType::NotUsable);
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
                    for pi in 0..n {
                        let t = types[pi];
                        sel.insert(
                            (si, gi, pi),
                            t == PluginType::Required || t == PluginType::Recommended,
                        );
                    }
                    if group.selection_type == SelectionType::SelectAtLeastOne
                        && n > 0
                        && !(0..n).any(|pi| sel[&(si, gi, pi)])
                    {
                        if let Some(pi) = types.iter().position(|t| *t != PluginType::NotUsable) {
                            sel.insert((si, gi, pi), true);
                        }
                    }
                }
            }
        }
    }
    sel
}

/// For SelectExactlyOne: first Required, else first Recommended, else first
/// usable plugin.
fn pick_one(types: &[PluginType]) -> Option<usize> {
    types
        .iter()
        .position(|t| *t == PluginType::Required)
        .or_else(|| types.iter().position(|t| *t == PluginType::Recommended))
        .or_else(|| types.iter().position(|t| *t != PluginType::NotUsable))
        .or(if types.is_empty() { None } else { Some(0) })
}

/// Whether a group's selection count satisfies its selection type.
fn group_valid(
    group: &PluginGroup,
    selections: &HashMap<SelKey, bool>,
    si: usize,
    gi: usize,
) -> bool {
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

impl XimodApp {
    /// (Re)initialise the preview from the current project and show the window.
    pub fn open_preview(&mut self) {
        let mut ps = PreviewState::default();

        // Collect every game file referenced by a `file` dependency, so the
        // assumptions panel can offer a state for each (default: Active).
        let mut fs: BTreeMap<String, FileState> = BTreeMap::new();
        let add = |deps: &[Dependency], fs: &mut BTreeMap<String, FileState>| {
            for d in deps {
                if d.dep_type == "file" && !d.name.is_empty() {
                    fs.entry(d.name.clone()).or_insert(FileState::Active);
                }
            }
        };
        for step in &self.ximod.steps {
            add(&step.visibility_dependencies, &mut fs);
            for g in &step.plugin_groups {
                for p in &g.plugins {
                    for pat in &p.dependency_patterns {
                        add(&pat.dependencies, &mut fs);
                    }
                }
            }
        }
        for cfs in &self.ximod.conditional_files {
            add(&cfs.dependencies, &mut fs);
        }
        ps.file_states = fs;

        ps.selections = default_selections(&self.ximod, &ps.file_states);
        let (_flags, visible) = compute(&self.ximod, &ps.selections, &ps.file_states);
        ps.cursor = *visible.first().unwrap_or(&0);
        ps.finished = visible.is_empty();
        self.preview = ps;
        self.show_preview = true;
    }

    /// Render the preview window.
    pub fn render_preview(&mut self, ctx: &egui::Context) {
        if !self.show_preview {
            self.free_window_closed("ximod_preview");
            return;
        }

        // Labels (fetched before borrowing fields, so i18n isn't captured).
        let title = self.i18n.t("preview-title");
        let vb = self.free_viewport_builder(ctx, "ximod_preview", title, [860.0, 620.0]);
        let l_refresh = self.i18n.t("preview-refresh");
        let l_assume = self.i18n.t("preview-assumptions");
        let l_back = self.i18n.t("preview-back");
        let l_next = self.i18n.t("preview-next");
        let l_install = self.i18n.t("preview-install");
        let l_close = self.i18n.t("preview-close");
        let l_restart = self.i18n.t("preview-restart");
        let l_summary = self.i18n.t("preview-summary-title");
        let l_empty = self.i18n.t("preview-empty");
        let l_none = self.i18n.t("preview-none-option");
        let l_invalid = self.i18n.t("preview-invalid");
        let l_nosteps = self.i18n.t("preview-no-steps");
        let l_col_src = self.i18n.t("preview-col-source");
        let l_col_dst = self.i18n.t("preview-col-dest");
        let l_col_prio = self.i18n.t("preview-col-priority");
        let l_desc_none = self.i18n.t("preview-select-hint");
        let l_details = self.i18n.t("preview-details");
        let sel_hint = |t: SelectionType| -> String {
            match t {
                SelectionType::SelectExactlyOne => self.i18n.t("preview-sel-exactlyone"),
                SelectionType::SelectAtMostOne => self.i18n.t("preview-sel-atmostone"),
                SelectionType::SelectAny => self.i18n.t("preview-sel-any"),
                SelectionType::SelectAll => self.i18n.t("preview-sel-all"),
                SelectionType::SelectAtLeastOne => self.i18n.t("preview-sel-atleastone"),
            }
        };
        let type_label = |t: PluginType| -> Option<&'static str> {
            match t {
                PluginType::Required => Some("Required"),
                PluginType::Recommended => Some("Recommended"),
                PluginType::NotUsable => Some("NotUsable"),
                PluginType::CouldBeUsable => Some("CouldBeUsable"),
                PluginType::Optional => None,
            }
        };

        // Disjoint field borrows for the closure.
        let ximod = &self.ximod;
        let preview = &mut self.preview;
        let root = self.root_directory.clone();
        let cfg = &mut self.config;
        let mut do_refresh = false;
        let mut do_close = false;

        let (flags, visible) = compute(ximod, &preview.selections, &preview.file_states);

        // Keep the cursor on a visible step while navigating.
        if !preview.finished {
            if visible.is_empty() {
                preview.finished = true;
            } else if !visible.contains(&preview.cursor) {
                preview.cursor = *visible
                    .iter()
                    .find(|&&s| s >= preview.cursor)
                    .unwrap_or(visible.first().unwrap());
            }
        }

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("ximod_preview"),
            vb,
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
                // ---- Top bar: header, refresh, assumptions toggle ----
                ui.horizontal(|ui| {
                    ui.heading(if ximod.name.is_empty() {
                        "FOMOD"
                    } else {
                        ximod.name.as_str()
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&l_refresh).clicked() {
                            do_refresh = true;
                        }
                        let n_files = preview.file_states.len();
                        if n_files > 0 {
                            let lbl = format!("{} ({})", l_assume, n_files);
                            ui.toggle_value(&mut preview.show_assumptions, lbl);
                        }
                    });
                });

                // ---- File-state assumptions panel ----
                if preview.show_assumptions && !preview.file_states.is_empty() {
                    ui.group(|ui| {
                        ui.label(RichText::new(&l_assume).strong());
                        egui::ScrollArea::vertical()
                            .max_height(120.0)
                            .show(ui, |ui| {
                                let names: Vec<String> =
                                    preview.file_states.keys().cloned().collect();
                                for name in names {
                                    ui.horizontal(|ui| {
                                        let cur = preview.file_states[&name];
                                        egui::ComboBox::from_id_salt(("fs", &name))
                                            .selected_text(cur.as_str())
                                            .show_ui(ui, |ui| {
                                                for st in FileState::variants() {
                                                    let mut c = cur;
                                                    if ui
                                                        .selectable_value(&mut c, *st, st.as_str())
                                                        .clicked()
                                                    {
                                                        preview
                                                            .file_states
                                                            .insert(name.clone(), *st);
                                                    }
                                                }
                                            });
                                        ui.label(&name);
                                    });
                                }
                            });
                    });
                }

                ui.separator();

                if preview.finished {
                    // ================= INSTALL SUMMARY =================
                    ui.label(RichText::new(&l_summary).heading());
                    let install = compute_install(ximod, &preview.selections, &preview.file_states);
                    if install.is_empty() {
                        ui.label(&l_empty);
                    } else {
                        ui.label(format!("{} : {}", l_summary, install.len()));
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .max_height(360.0)
                            .show(ui, |ui| {
                                egui::Grid::new("preview_install")
                                    .num_columns(3)
                                    .striped(true)
                                    .min_col_width(120.0)
                                    .show(ui, |ui| {
                                        ui.label(RichText::new(&l_col_src).strong());
                                        ui.label(RichText::new(&l_col_dst).strong());
                                        ui.label(RichText::new(&l_col_prio).strong());
                                        ui.end_row();
                                        for f in &install {
                                            let src = if f.source.is_empty() {
                                                "—"
                                            } else {
                                                f.source.as_str()
                                            };
                                            ui.label(RichText::new(src).monospace());
                                            ui.label(
                                                RichText::new(f.destination.as_str()).monospace(),
                                            );
                                            ui.label(f.priority.to_string());
                                            ui.end_row();
                                        }
                                    });
                            });
                    }
                } else if visible.is_empty() {
                    ui.label(&l_nosteps);
                } else {
                    // ================= CURRENT STEP =================
                    let si = preview.cursor;
                    let step = &ximod.steps[si];
                    let pos = visible.iter().position(|&s| s == si).unwrap_or(0);
                    ui.label(
                        RichText::new(format!(
                            "{} — {}/{}",
                            if step.name.is_empty() {
                                "Step".to_string()
                            } else {
                                step.name.clone()
                            },
                            pos + 1,
                            visible.len()
                        ))
                        .strong()
                        .size(15.0),
                    );
                    ui.add_space(4.0);

                    ui.columns(2, |cols| {
                        // ---- Left: groups & options ----
                        egui::ScrollArea::vertical()
                            .id_salt("preview_opts")
                            .auto_shrink([false, false])
                            .show(&mut cols[0], |ui| {
                                for (gi, group) in step.plugin_groups.iter().enumerate() {
                                    ui.group(|ui| {
                                        ui.label(RichText::new(&group.name).strong());
                                        ui.label(
                                            RichText::new(sel_hint(group.selection_type))
                                                .small()
                                                .color(Color32::GRAY),
                                        );
                                        ui.add_space(2.0);

                                        let radio = matches!(
                                            group.selection_type,
                                            SelectionType::SelectExactlyOne
                                                | SelectionType::SelectAtMostOne
                                        );

                                        // "(none)" option for SelectAtMostOne.
                                        if group.selection_type == SelectionType::SelectAtMostOne {
                                            let any = (0..group.plugins.len()).any(|pi| {
                                                *preview
                                                    .selections
                                                    .get(&(si, gi, pi))
                                                    .unwrap_or(&false)
                                            });
                                            if ui.radio(!any, &l_none).clicked() {
                                                for pi in 0..group.plugins.len() {
                                                    preview.selections.insert((si, gi, pi), false);
                                                }
                                            }
                                        }

                                        for (pi, plugin) in group.plugins.iter().enumerate() {
                                            let key = (si, gi, pi);
                                            let et = effective_type(plugin, &flags, &preview.file_states);
                                            let forced_on = et == PluginType::Required
                                                || group.selection_type == SelectionType::SelectAll;
                                            let forced_off = et == PluginType::NotUsable;
                                            let mut checked = *preview
                                                .selections
                                                .get(&key)
                                                .unwrap_or(&false);
                                            if forced_on {
                                                checked = true;
                                            }
                                            if forced_off {
                                                checked = false;
                                            }
                                            preview.selections.insert(key, checked);
                                            let enabled = !(forced_on || forced_off);

                                            ui.horizontal(|ui| {
                                                let resp = ui.add_enabled_ui(enabled, |ui| {
                                                    if radio {
                                                        ui.radio(checked, &plugin.name)
                                                    } else {
                                                        let mut c = checked;
                                                        ui.checkbox(&mut c, &plugin.name)
                                                    }
                                                });
                                                let clicked = resp.inner.clicked();
                                                if resp.inner.hovered() || clicked {
                                                    preview.focused = Some(key);
                                                }
                                                if clicked {
                                                    if radio {
                                                        for other in 0..group.plugins.len() {
                                                            preview.selections.insert(
                                                                (si, gi, other),
                                                                other == pi,
                                                            );
                                                        }
                                                    } else {
                                                        preview
                                                            .selections
                                                            .insert(key, !checked);
                                                    }
                                                }
                                                if let Some(tl) = type_label(et) {
                                                    ui.label(
                                                        RichText::new(tl)
                                                            .small()
                                                            .color(Color32::from_rgb(150, 120, 60)),
                                                    );
                                                }
                                            });
                                        }
                                    });
                                }
                            });

                        // ---- Right: detail of the focused plugin ----
                        let ui = &mut cols[1];
                        ui.label(RichText::new(&l_details).strong());
                        ui.separator();
                        let focus = preview.focused.filter(|&(fs, fg, fp)| {
                            fs == si
                                && fg < step.plugin_groups.len()
                                && fp < step.plugin_groups[fg].plugins.len()
                        });
                        match focus {
                            Some((_, fg, fp)) => {
                                let plugin = &step.plugin_groups[fg].plugins[fp];
                                ui.label(RichText::new(&plugin.name).strong());
                                ui.add_space(4.0);
                                let abs = plugin
                                    .image_path
                                    .as_ref()
                                    .and_then(|rel| root.as_ref().map(|r| r.join(rel)));
                                ImageDisplay::new(260.0, 150.0)
                                    .with_fallback(" ")
                                    .show(ui, abs.as_deref());
                                ui.add_space(6.0);
                                egui::ScrollArea::vertical()
                                    .id_salt("preview_desc")
                                    .max_height(220.0)
                                    .show(ui, |ui| {
                                        ui.label(&plugin.description);
                                    });
                            }
                            None => {
                                ui.label(
                                    RichText::new(&l_desc_none).color(Color32::GRAY),
                                );
                            }
                        }
                    });
                }

                ui.separator();

                // ---- Navigation ----
                let step_ok = preview.finished
                    || visible.is_empty()
                    || {
                        let step = &ximod.steps[preview.cursor];
                        step.plugin_groups
                            .iter()
                            .enumerate()
                            .all(|(gi, g)| group_valid(g, &preview.selections, preview.cursor, gi))
                    };

                ui.horizontal(|ui| {
                    let can_back = !preview.history.is_empty();
                    if ui.add_enabled(can_back, egui::Button::new(&l_back)).clicked() {
                        if let Some(prev) = preview.history.pop() {
                            preview.finished = false;
                            preview.cursor = prev;
                        }
                    }

                    if preview.finished {
                        if ui.button(&l_restart).clicked() {
                            do_refresh = true;
                        }
                        if ui.button(&l_close).clicked() {
                            do_close = true;
                        }
                    } else {
                        // Is there another visible step after the current one?
                        let has_next = visible.iter().any(|&s| s > preview.cursor);
                        let label = if has_next { &l_next } else { &l_install };
                        if ui
                            .add_enabled(step_ok, egui::Button::new(label))
                            .clicked()
                        {
                            preview.history.push(preview.cursor);
                            match visible.iter().copied().find(|&s| s > preview.cursor) {
                                Some(n) => preview.cursor = n,
                                None => preview.finished = true,
                            }
                        }
                    }

                    if !step_ok {
                        ui.label(
                            RichText::new(&l_invalid).color(Color32::from_rgb(200, 80, 80)),
                        );
                    }
                });
                });

                crate::ui::main_window::record_win_geom(cfg, ctx, "ximod_preview");
                if ctx.input(|i| i.viewport().close_requested())
                    || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
                {
                    do_close = true;
                }
            },
        );

        if do_refresh {
            self.open_preview();
        } else if do_close {
            self.show_preview = false;
            self.free_window_closed("ximod_preview");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        ConditionFlag, ConditionalFileSet, Dependency, InstallFile, Plugin, PluginGroup, Step,
        Ximod,
    };

    /// Build a project: step 1 chooses a texture resolution (sets flag `res`),
    /// step 2 is visible only when `res = 4K`, and a conditional file set installs
    /// an extra file when `res = 4K`.
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
        s2.visibility_dependencies
            .push(Dependency::new_flag("res", "4K"));
        m.steps.push(s1);
        m.steps.push(s2);

        let mut cfs = ConditionalFileSet::new();
        cfs.dependencies.push(Dependency::new_flag("res", "4K"));
        cfs.files.push(InstallFile::new_file("patch4k.esp"));
        m.conditional_files.push(cfs);
        m
    }

    #[test]
    fn default_picks_first_and_hides_conditional_step() {
        let m = sample();
        let files = BTreeMap::new();
        let sel = default_selections(&m, &files);
        // SelectExactlyOne → first plugin (2K) chosen.
        assert_eq!(sel.get(&(0, 0, 0)), Some(&true));
        assert_eq!(sel.get(&(0, 0, 1)), Some(&false));

        let (flags, visible) = compute(&m, &sel, &files);
        assert_eq!(flags.get("res").map(String::as_str), Some("2K"));
        assert_eq!(visible, vec![0]); // step 2 hidden (res != 4K)

        let install: Vec<String> = compute_install(&m, &sel, &files)
            .into_iter()
            .map(|f| f.source)
            .collect();
        assert!(install.iter().any(|d| d == "base.esp"));
        assert!(install.iter().any(|d| d.contains("a.dds")));
        assert!(!install.iter().any(|d| d == "patch4k.esp")); // conditional not applied
    }

    #[test]
    fn selecting_4k_reveals_step_and_conditional_file() {
        let m = sample();
        let files = BTreeMap::new();
        let mut sel = default_selections(&m, &files);
        // Switch the exclusive choice to 4K.
        sel.insert((0, 0, 0), false);
        sel.insert((0, 0, 1), true);

        let (flags, visible) = compute(&m, &sel, &files);
        assert_eq!(flags.get("res").map(String::as_str), Some("4K"));
        assert_eq!(visible, vec![0, 1]); // step 2 now visible

        let install: Vec<String> = compute_install(&m, &sel, &files)
            .into_iter()
            .map(|f| f.source)
            .collect();
        assert!(install.iter().any(|d| d == "patch4k.esp")); // conditional applied
        assert!(install.iter().any(|d| d.contains("b.dds")));
    }

    #[test]
    fn file_dependency_assumption_is_honoured() {
        // A step visible only when a game file is Active.
        let mut m = Ximod::new("F");
        let mut s = Step::new("needs file");
        s.visibility_dependencies
            .push(Dependency::new_file("Skyrim.esm", "Active"));
        m.steps.push(s);
        let sel = HashMap::new();

        let mut files = BTreeMap::new();
        files.insert("Skyrim.esm".to_string(), FileState::Active);
        assert_eq!(compute(&m, &sel, &files).1, vec![0]);

        files.insert("Skyrim.esm".to_string(), FileState::Missing);
        assert_eq!(compute(&m, &sel, &files).1, Vec::<usize>::new());
    }
}
