//! Project → Preset wizard…: an independent window that generates the
//! "preset + customise" pattern (user manual § 7.5) on the current project.
//!
//! Two tabs. **From existing pages** ticks the steps that are feature pages,
//! names the generated elements, lists the presets and shows a choice matrix
//! (rows = every option of every ticked page, columns = presets). **New
//! skeleton** creates feature pages from a short description (name, kind of
//! choice, one variant per line) and hands over to the first tab. Apply runs
//! [`crate::models::presets::apply`] as a single undoable edit.

use crate::models::SelectionType;
use crate::models::presets::{
    self, FeatureSpec, GroupChoice, PresetDef, PresetError, PresetPlan, PresetSettings, PresetSummary,
};
use crate::ui::main_window::XimodApp;
use eframe::egui::{self, RichText};
use std::collections::HashMap;

const WINDOW_ID: &str = "ximod_presets";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WizardTab {
    #[default]
    Existing,
    Skeleton,
}

/// One preset as edited in the matrix: a name and the choices keyed by
/// `(step index, group index)` of the project, so ticking and unticking pages
/// never loses what was chosen on the others.
#[derive(Debug, Clone, Default)]
pub struct PresetRow {
    pub name: String,
    pub choices: HashMap<(usize, usize), GroupChoice>,
}

/// One feature of the skeleton tab.
#[derive(Debug, Clone)]
pub struct SkeletonRow {
    pub name: String,
    pub exclusive: bool,
    pub variants: String,
}

impl Default for SkeletonRow {
    fn default() -> Self {
        Self {
            name: String::new(),
            exclusive: true,
            variants: String::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PresetWizardState {
    pub loaded: bool,
    pub tab: WizardTab,
    pub settings: PresetSettings,
    /// Per project step: is it a feature page?
    pub selected: Vec<bool>,
    pub presets: Vec<PresetRow>,
    pub skeleton: Vec<SkeletonRow>,
    /// Last validation error of the plan, for the status line.
    pub error: Option<PresetError>,
    /// The project as it was before an earlier run of the wizard, when one is
    /// detected: the page list and the matrix refer to it, and Apply rebuilds
    /// the structure instead of stacking a second one on top of the first.
    pub base: Option<crate::models::Ximod>,
    /// Where the installation-mode step goes (kept from an earlier run).
    pub mode_step_at: usize,
}

impl PresetWizardState {
    /// Seed the names from the interface language and the page list from the
    /// project (every step unticked).
    pub fn load(&mut self, i18n: &crate::i18n::I18n, step_count: usize) {
        self.settings = PresetSettings {
            mode_step_name: i18n.t("preset-default-mode-step"),
            preset_group_name: i18n.t("preset-default-group"),
            use_preset_name: i18n.t("preset-default-use-preset"),
            customise_label: i18n.t("preset-default-customise"),
            preset_flag: "preset".to_string(),
            none_name: i18n.t("preset-default-none"),
            no_preset_name: i18n.t("preset-default-no-preset"),
            add_none: true,
        };
        self.mode_step_at = 0;
        self.selected = vec![false; step_count];
        if self.presets.is_empty() {
            self.presets.push(PresetRow {
                name: i18n.t_num("preset-new-name", 1),
                choices: HashMap::new(),
            });
        }
        if self.skeleton.is_empty() {
            self.skeleton.push(SkeletonRow::default());
        }
        self.tab = if step_count == 0 {
            WizardTab::Skeleton
        } else {
            WizardTab::Existing
        };
        self.error = None;
        self.loaded = true;
    }

    /// Prefill the window from the structure an earlier run of the wizard
    /// left in the project (see [`presets::unapply`]): its names, its feature
    /// pages and its presets with their choices.
    pub fn load_existing(&mut self, plan: &PresetPlan, step_count: usize) {
        self.settings = plan.settings.clone();
        self.mode_step_at = plan.mode_step_at;
        self.selected = vec![false; step_count];
        for &si in &plan.feature_steps {
            if si < step_count {
                self.selected[si] = true;
            }
        }
        self.presets = plan
            .presets
            .iter()
            .map(|p| {
                let mut choices = HashMap::new();
                for (f, groups) in p.choices.iter().enumerate() {
                    if let Some(&si) = plan.feature_steps.get(f) {
                        for (gi, c) in groups.iter().enumerate() {
                            choices.insert((si, gi), c.clone());
                        }
                    }
                }
                PresetRow {
                    name: p.name.clone(),
                    choices,
                }
            })
            .collect();
        self.tab = WizardTab::Existing;
    }

    /// Keep the page list in step with the project (steps added or removed
    /// while the window is open).
    pub fn sync(&mut self, step_count: usize) {
        self.selected.resize(step_count, false);
    }

    /// The ticked steps, in project order.
    pub fn feature_steps(&self) -> Vec<usize> {
        self.selected
            .iter()
            .enumerate()
            .filter(|(_, s)| **s)
            .map(|(i, _)| i)
            .collect()
    }

    /// Build the plan the model expects from the matrix.
    pub fn plan(&self, ximod: &crate::models::Ximod) -> PresetPlan {
        let feature_steps = self.feature_steps();
        let presets = self
            .presets
            .iter()
            .map(|p| PresetDef {
                name: p.name.clone(),
                choices: feature_steps
                    .iter()
                    .map(|&si| {
                        let groups = ximod.steps.get(si).map(|s| s.plugin_groups.len()).unwrap_or(0);
                        (0..groups)
                            .map(|gi| p.choices.get(&(si, gi)).cloned().unwrap_or_default())
                            .collect()
                    })
                    .collect(),
            })
            .collect();
        PresetPlan {
            settings: self.settings.clone(),
            feature_steps,
            presets,
            mode_step_at: self.mode_step_at,
        }
    }

    /// The skeleton rows as feature specs (empty rows dropped).
    pub fn skeleton_specs(&self) -> Vec<FeatureSpec> {
        self.skeleton
            .iter()
            .filter(|r| !r.name.trim().is_empty())
            .map(|r| FeatureSpec {
                name: r.name.trim().to_string(),
                exclusive: r.exclusive,
                variants: r
                    .variants
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty())
                    .collect(),
            })
            .filter(|f| !f.variants.is_empty())
            .collect()
    }
}

/// All the strings of the window, fetched once per frame.
struct Texts {
    title: String,
    tab_existing: String,
    tab_skeleton: String,
    pages: String,
    pages_hint: String,
    no_steps: String,
    names: String,
    mode_step: String,
    group: String,
    use_preset_name: String,
    customise_label: String,
    flag: String,
    none_name: String,
    no_preset_name: String,
    add_none: String,
    presets: String,
    add: String,
    remove: String,
    none_row: String,
    matrix_hint: String,
    page_col: String,
    apply: String,
    close: String,
    summary: String,
    err_no_feature: String,
    err_no_preset: String,
    err_empty_name: String,
    err_dup_preset: String,
    err_bad_choice: String,
    skel_hint: String,
    skel_feature: String,
    skel_exclusive: String,
    skel_multiple: String,
    skel_variants: String,
    skel_add: String,
    skel_remove: String,
    skel_create: String,
    skel_none: String,
    rebuild_hint: String,
}

impl Texts {
    fn new(i: &crate::i18n::I18n) -> Self {
        Self {
            title: i.t("preset-title"),
            tab_existing: i.t("preset-tab-existing"),
            tab_skeleton: i.t("preset-tab-skeleton"),
            pages: i.t("preset-pages"),
            pages_hint: i.t("preset-pages-hint"),
            no_steps: i.t("preset-no-steps"),
            names: i.t("preset-names"),
            mode_step: i.t("preset-mode-step"),
            group: i.t("preset-group"),
            use_preset_name: i.t("preset-use-preset-name"),
            customise_label: i.t("preset-customise-label"),
            flag: i.t("preset-flag"),
            none_name: i.t("preset-none-name"),
            no_preset_name: i.t("preset-no-preset-name"),
            add_none: i.t("preset-add-none"),
            presets: i.t("preset-presets"),
            add: i.t("preset-add"),
            remove: i.t("preset-remove"),
            none_row: i.t("preset-none-row"),
            matrix_hint: i.t("preset-matrix-hint"),
            page_col: i.t("preset-page-col"),
            apply: i.t("preset-apply"),
            close: i.t("btn-close"),
            summary: i.t("preset-summary"),
            err_no_feature: i.t("preset-err-no-feature"),
            err_no_preset: i.t("preset-err-no-preset"),
            err_empty_name: i.t("preset-err-empty-name"),
            err_dup_preset: i.t("preset-err-dup-preset"),
            err_bad_choice: i.t("preset-err-bad-choice"),
            skel_hint: i.t("preset-skel-hint"),
            skel_feature: i.t("preset-skel-feature"),
            skel_exclusive: i.t("preset-skel-exclusive"),
            skel_multiple: i.t("preset-skel-multiple"),
            skel_variants: i.t("preset-skel-variants"),
            skel_add: i.t("preset-skel-add"),
            skel_remove: i.t("preset-skel-remove"),
            skel_create: i.t("preset-skel-create"),
            skel_none: i.t("preset-skel-none"),
            rebuild_hint: i.t("preset-rebuild-hint"),
        }
    }

    fn error(&self, e: &PresetError) -> String {
        match e {
            PresetError::NoFeature => self.err_no_feature.clone(),
            PresetError::NoPreset => self.err_no_preset.clone(),
            PresetError::EmptyName => self.err_empty_name.clone(),
            PresetError::DuplicatePreset(n) => self.err_dup_preset.replace("{name}", n),
            PresetError::BadStep(_) | PresetError::BadChoice { .. } => self.err_bad_choice.clone(),
        }
    }

    fn summary(&self, s: &PresetSummary) -> String {
        self.summary
            .replace("{features}", &s.features.to_string())
            .replace("{options}", &s.options.to_string())
            .replace("{sets}", &s.conditional_sets.to_string())
            .replace("{none}", &s.none_options.to_string())
    }
}

/// Format the summary of an applied plan for the status message.
pub fn summary_text(i18n: &crate::i18n::I18n, s: &PresetSummary) -> String {
    Texts::new(i18n).summary(s)
}

impl XimodApp {
    /// Open the preset wizard (Project menu).
    pub fn open_preset_wizard(&mut self) {
        self.show_preset_wizard = true;
        self.preset_wizard.loaded = false;
    }

    /// Render the preset wizard window (independent viewport).
    pub fn render_preset_wizard(&mut self, ctx: &egui::Context) {
        if !self.show_preset_wizard {
            self.preset_wizard.loaded = false;
            self.free_window_closed(WINDOW_ID);
            return;
        }
        if !self.preset_wizard.loaded {
            // A structure left by an earlier run is undone on a copy: the
            // window then edits the original pages and the existing presets.
            let mut restored = self.ximod.clone();
            let existing = presets::unapply(&mut restored);
            let n = restored.steps.len();
            self.preset_wizard.load(&self.i18n, n);
            match existing {
                Some(plan) => {
                    self.preset_wizard.load_existing(&plan, n);
                    self.preset_wizard.base = Some(restored);
                }
                None => self.preset_wizard.base = None,
            }
        }
        // The project the page list and the matrix refer to.
        let mut base = self.preset_wizard.base.take();
        let rebuild = base.is_some();
        self.preset_wizard
            .sync(base.as_ref().map_or(self.ximod.steps.len(), |b| b.steps.len()));

        let t = Texts::new(&self.i18n);
        let vb = self.free_viewport_builder(ctx, WINDOW_ID, t.title.clone(), [960.0, 620.0], false);
        let new_preset_name = self
            .i18n
            .t_num("preset-new-name", (self.preset_wizard.presets.len() + 1) as i64);

        let st = &mut self.preset_wizard;
        let cfg = &mut self.config;
        let ximod = base.as_ref().unwrap_or(&self.ximod);
        let mut do_apply = false;
        let mut do_close = false;
        let mut do_create_pages = false;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of(WINDOW_ID), vb, |ctx, _class| {
            // ---- Tabs
            egui::TopBottomPanel::top("preset_tabs").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut st.tab, WizardTab::Existing, &t.tab_existing);
                    ui.selectable_value(&mut st.tab, WizardTab::Skeleton, &t.tab_skeleton);
                });
                ui.add_space(4.0);
            });

            // ---- Bottom bar: status + Apply / Close
            let plan = st.plan(ximod);
            st.error = presets::validate(ximod, &plan).err();
            egui::TopBottomPanel::bottom("preset_bottom").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    match st.tab {
                        WizardTab::Existing => {
                            if let Some(e) = &st.error {
                                ui.label(RichText::new(t.error(e)).color(ui.visuals().warn_fg_color));
                            } else {
                                let n_opts: usize = plan
                                    .feature_steps
                                    .iter()
                                    .map(|&si| {
                                        ximod.steps[si]
                                            .plugin_groups
                                            .iter()
                                            .map(|g| g.plugins.len())
                                            .sum::<usize>()
                                    })
                                    .sum();
                                ui.label(
                                    RichText::new(format!(
                                        "{} · {} · {}",
                                        plan.feature_steps.len(),
                                        plan.presets.len(),
                                        n_opts
                                    ))
                                    .weak(),
                                );
                            }
                        }
                        WizardTab::Skeleton => {
                            if st.skeleton_specs().is_empty() {
                                ui.label(RichText::new(&t.skel_none).weak());
                            }
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&t.close).clicked() {
                            do_close = true;
                        }
                        match st.tab {
                            WizardTab::Existing => {
                                if ui
                                    .add_enabled(st.error.is_none(), egui::Button::new(&t.apply))
                                    .clicked()
                                {
                                    do_apply = true;
                                }
                            }
                            WizardTab::Skeleton => {
                                if ui
                                    .add_enabled(!st.skeleton_specs().is_empty(), egui::Button::new(&t.skel_create))
                                    .clicked()
                                {
                                    do_create_pages = true;
                                }
                            }
                        }
                    });
                });
                ui.add_space(4.0);
            });

            match st.tab {
                WizardTab::Existing => {
                    // ---- Left: feature pages + names
                    egui::SidePanel::left("preset_left")
                        .default_width(300.0)
                        .show(ctx, |ui| {
                            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                                ui.add_space(4.0);
                                if rebuild {
                                    ui.label(RichText::new(&t.rebuild_hint).color(ui.visuals().warn_fg_color));
                                    ui.add_space(6.0);
                                }
                                ui.strong(&t.pages);
                                ui.label(RichText::new(&t.pages_hint).weak().small());
                                ui.add_space(4.0);
                                if ximod.steps.is_empty() {
                                    ui.label(RichText::new(&t.no_steps).weak());
                                }
                                for (si, step) in ximod.steps.iter().enumerate() {
                                    let name = if step.name.trim().is_empty() {
                                        format!("#{}", si + 1)
                                    } else {
                                        step.name.clone()
                                    };
                                    ui.checkbox(&mut st.selected[si], name);
                                }
                                ui.add_space(8.0);
                                ui.separator();
                                ui.strong(&t.names);
                                egui::Grid::new("preset_names")
                                    .num_columns(2)
                                    .spacing([8.0, 4.0])
                                    .show(ui, |ui| {
                                        ui.label(&t.mode_step);
                                        ui.text_edit_singleline(&mut st.settings.mode_step_name);
                                        ui.end_row();
                                        ui.label(&t.group);
                                        ui.text_edit_singleline(&mut st.settings.preset_group_name);
                                        ui.end_row();
                                        ui.label(&t.use_preset_name);
                                        ui.text_edit_singleline(&mut st.settings.use_preset_name);
                                        ui.end_row();
                                        ui.label(&t.customise_label);
                                        ui.text_edit_singleline(&mut st.settings.customise_label);
                                        ui.end_row();
                                        ui.label(&t.flag);
                                        ui.text_edit_singleline(&mut st.settings.preset_flag);
                                        ui.end_row();
                                        ui.label(&t.none_name);
                                        ui.text_edit_singleline(&mut st.settings.none_name);
                                        ui.end_row();
                                        ui.label(&t.no_preset_name);
                                        ui.text_edit_singleline(&mut st.settings.no_preset_name);
                                        ui.end_row();
                                    });
                                ui.checkbox(&mut st.settings.add_none, &t.add_none);
                            });
                        });

                    // ---- Centre: presets + choice matrix
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.strong(&t.presets);
                            if ui.button(&t.add).clicked() {
                                st.presets.push(PresetRow {
                                    name: new_preset_name.clone(),
                                    choices: HashMap::new(),
                                });
                            }
                        });
                        ui.label(RichText::new(&t.matrix_hint).weak().small());
                        ui.add_space(4.0);
                        let feature_steps = st.feature_steps();
                        let n_presets = st.presets.len();
                        let mut remove: Option<usize> = None;
                        egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                            egui::Grid::new("preset_matrix")
                                .num_columns(n_presets + 1)
                                .spacing([14.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    // Header: page column + one editable name per preset.
                                    ui.strong(&t.page_col);
                                    for (k, p) in st.presets.iter_mut().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.add(egui::TextEdit::singleline(&mut p.name).desired_width(120.0));
                                            if ui
                                                .add_enabled(
                                                    n_presets > 1,
                                                    egui::Button::new(crate::ui::theme::icon::CLOSE).small(),
                                                )
                                                .on_hover_text(&t.remove)
                                                .clicked()
                                            {
                                                remove = Some(k);
                                            }
                                        });
                                    }
                                    ui.end_row();

                                    for &si in &feature_steps {
                                        let step = &ximod.steps[si];
                                        ui.strong(if step.name.trim().is_empty() {
                                            format!("#{}", si + 1)
                                        } else {
                                            step.name.clone()
                                        });
                                        for _ in 0..n_presets {
                                            ui.label("");
                                        }
                                        ui.end_row();
                                        for (gi, group) in step.plugin_groups.iter().enumerate() {
                                            let exclusive = matches!(
                                                group.selection_type,
                                                SelectionType::SelectExactlyOne | SelectionType::SelectAtMostOne
                                            );
                                            if step.plugin_groups.len() > 1 {
                                                ui.label(RichText::new(format!("  {}", group.name)).italics());
                                                for _ in 0..n_presets {
                                                    ui.label("");
                                                }
                                                ui.end_row();
                                            }
                                            if exclusive {
                                                ui.label(format!("    {}", t.none_row));
                                                for p in st.presets.iter_mut() {
                                                    let cur = p.choices.get(&(si, gi)).cloned().unwrap_or_default();
                                                    let is_none = matches!(cur, GroupChoice::None);
                                                    if ui.radio(is_none, "").clicked() {
                                                        p.choices.insert((si, gi), GroupChoice::None);
                                                    }
                                                }
                                                ui.end_row();
                                            }
                                            for (pi, plugin) in group.plugins.iter().enumerate() {
                                                ui.label(format!("    {}", plugin.name));
                                                for p in st.presets.iter_mut() {
                                                    let cur = p.choices.get(&(si, gi)).cloned().unwrap_or_default();
                                                    if exclusive {
                                                        let on = cur == GroupChoice::One(pi);
                                                        if ui.radio(on, "").clicked() {
                                                            p.choices.insert((si, gi), GroupChoice::One(pi));
                                                        }
                                                    } else {
                                                        let mut v = match cur {
                                                            GroupChoice::Many(v) => v,
                                                            GroupChoice::One(x) => vec![x],
                                                            GroupChoice::None => Vec::new(),
                                                        };
                                                        let mut on = v.contains(&pi);
                                                        if ui.checkbox(&mut on, "").changed() {
                                                            if on {
                                                                v.push(pi);
                                                                v.sort_unstable();
                                                                v.dedup();
                                                            } else {
                                                                v.retain(|x| *x != pi);
                                                            }
                                                            p.choices.insert((si, gi), GroupChoice::Many(v));
                                                        }
                                                    }
                                                }
                                                ui.end_row();
                                            }
                                        }
                                    }
                                });
                        });
                        if let Some(k) = remove {
                            st.presets.remove(k);
                        }
                    });
                }
                WizardTab::Skeleton => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add_space(4.0);
                        ui.label(&t.skel_hint);
                        ui.add_space(6.0);
                        let mut remove: Option<usize> = None;
                        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                            for (k, row) in st.skeleton.iter_mut().enumerate() {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(&t.skel_feature);
                                        ui.add(egui::TextEdit::singleline(&mut row.name).desired_width(260.0));
                                        ui.radio_value(&mut row.exclusive, true, &t.skel_exclusive);
                                        ui.radio_value(&mut row.exclusive, false, &t.skel_multiple);
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.button(&t.skel_remove).clicked() {
                                                remove = Some(k);
                                            }
                                        });
                                    });
                                    ui.label(&t.skel_variants);
                                    ui.add(
                                        egui::TextEdit::multiline(&mut row.variants)
                                            .desired_rows(3)
                                            .desired_width(f32::INFINITY),
                                    );
                                });
                                ui.add_space(4.0);
                            }
                            if ui.button(&t.skel_add).clicked() {
                                st.skeleton.push(SkeletonRow::default());
                            }
                        });
                        if let Some(k) = remove {
                            st.skeleton.remove(k);
                            if st.skeleton.is_empty() {
                                st.skeleton.push(SkeletonRow::default());
                            }
                        }
                    });
                }
            }

            crate::ui::widgets::free_window::record_win_geom(cfg, ctx, WINDOW_ID);
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        if do_create_pages {
            let specs = self.preset_wizard.skeleton_specs();
            let mut created = presets::create_feature_pages(&mut self.ximod, &specs);
            if let Some(b) = base.as_mut() {
                // The page list refers to the restored project: add the pages
                // there too, and tick them by their indices in it.
                created = presets::create_feature_pages(b, &specs);
            }
            if !created.is_empty() {
                self.mark_modified();
                self.preset_wizard
                    .sync(base.as_ref().map_or(self.ximod.steps.len(), |b| b.steps.len()));
                for &si in &created {
                    self.preset_wizard.selected[si] = true;
                }
                self.preset_wizard.skeleton = vec![SkeletonRow::default()];
                self.preset_wizard.tab = WizardTab::Existing;
                let msg = self.i18n.t_num("preset-skel-created", created.len() as i64);
                self.notify_ok(msg);
            }
        }
        if do_apply {
            let plan = self.preset_wizard.plan(base.as_ref().unwrap_or(&self.ximod));
            // Rebuild: undo the earlier structure on a working copy, then
            // apply the new plan to it — one undoable edit either way.
            let mut work = self.ximod.clone();
            if rebuild {
                presets::unapply(&mut work);
            }
            match presets::apply(&mut work, &plan) {
                Ok(summary) => {
                    self.ximod = work;
                    self.mark_modified();
                    self.reset_selection();
                    self.select_step(Some(0));
                    let msg = self
                        .i18n
                        .t("preset-applied")
                        .replace("{summary}", &summary_text(&self.i18n, &summary));
                    self.notify_ok(msg);
                    do_close = true;
                }
                Err(e) => {
                    let msg = Texts::new(&self.i18n).error(&e);
                    self.notify_err(msg);
                }
            }
        }
        if do_close {
            self.show_preset_wizard = false;
            self.preset_wizard.loaded = false;
            self.free_window_closed(WINDOW_ID);
        } else {
            self.preset_wizard.base = base;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The window renders headless; the matrix state converts into a plan that
    /// the model accepts, and ticking pages in any order keeps the choices.
    #[test]
    fn window_renders_and_state_builds_a_valid_plan() {
        let mut app = XimodApp::default();
        app.open_preset_wizard();
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_preset_wizard(ctx));
        assert!(app.show_preset_wizard);
        assert_eq!(
            app.preset_wizard.tab,
            WizardTab::Skeleton,
            "no step yet → skeleton tab first"
        );

        // Create two pages from the skeleton, then define a preset.
        app.preset_wizard.skeleton = vec![
            SkeletonRow {
                name: "Lighting".into(),
                exclusive: true,
                variants: "Warm\nCold".into(),
            },
            SkeletonRow {
                name: "Extras".into(),
                exclusive: false,
                variants: "Sounds\nMeshes".into(),
            },
        ];
        let specs = app.preset_wizard.skeleton_specs();
        assert_eq!(specs.len(), 2);
        let created = presets::create_feature_pages(&mut app.ximod, &specs);
        app.preset_wizard.sync(app.ximod.steps.len());
        for si in created {
            app.preset_wizard.selected[si] = true;
        }
        app.preset_wizard.presets[0].choices.insert((0, 0), GroupChoice::One(1));
        app.preset_wizard.presets[0]
            .choices
            .insert((1, 0), GroupChoice::Many(vec![0]));
        let plan = app.preset_wizard.plan(&app.ximod);
        assert_eq!(plan.feature_steps, vec![0, 1]);
        assert_eq!(
            plan.presets[0].choices,
            vec![vec![GroupChoice::One(1)], vec![GroupChoice::Many(vec![0])]]
        );
        assert!(presets::validate(&app.ximod, &plan).is_ok());

        // Unticking the first page drops it from the plan but keeps its choice.
        app.preset_wizard.selected[0] = false;
        let plan = app.preset_wizard.plan(&app.ximod);
        assert_eq!(plan.feature_steps, vec![1]);
        assert_eq!(plan.presets[0].choices, vec![vec![GroupChoice::Many(vec![0])]]);
        assert!(app.preset_wizard.presets[0].choices.contains_key(&(0, 0)));

        // Rendering the matrix with pages ticked still works headless.
        app.preset_wizard.selected[0] = true;
        app.preset_wizard.tab = WizardTab::Existing;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_preset_wizard(ctx));
        assert!(app.preset_wizard.error.is_none());
    }

    /// Reopening the wizard on a project it already transformed loads the
    /// existing presets, and Apply rebuilds instead of stacking.
    #[test]
    fn reopening_loads_the_existing_structure_and_rebuilds() {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let specs = vec![FeatureSpec {
            name: "Lighting".into(),
            exclusive: true,
            variants: vec!["Warm".into(), "Cold".into()],
        }];
        presets::create_feature_pages(&mut app.ximod, &specs);
        let plan = PresetPlan {
            settings: PresetSettings::default(),
            feature_steps: vec![0],
            mode_step_at: 0,
            presets: vec![PresetDef {
                name: "Cosy".into(),
                choices: vec![vec![GroupChoice::One(0)]],
            }],
        };
        presets::apply(&mut app.ximod, &plan).unwrap();
        assert_eq!(app.ximod.steps.len(), 2);

        app.open_preset_wizard();
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_preset_wizard(ctx));
        let st = &app.preset_wizard;
        assert!(st.base.is_some(), "earlier structure detected");
        assert_eq!(st.selected, vec![true]);
        assert_eq!(st.presets.len(), 1);
        assert_eq!(st.presets[0].name, "Cosy");
        assert_eq!(st.presets[0].choices.get(&(0, 0)), Some(&GroupChoice::One(0)));

        // A second preset, then the same rebuild as Apply.
        app.preset_wizard.presets.push(PresetRow {
            name: "Bright".into(),
            choices: HashMap::from([((0, 0), GroupChoice::One(1))]),
        });
        let plan = app.preset_wizard.plan(app.preset_wizard.base.as_ref().unwrap());
        let mut work = app.ximod.clone();
        presets::unapply(&mut work);
        presets::apply(&mut work, &plan).unwrap();
        assert_eq!(work.steps.len(), 2, "still one installation-mode step");
        assert_eq!(
            work.steps[0].plugin_groups[0].plugins.len(),
            3,
            "two presets + No preset"
        );
    }
}
