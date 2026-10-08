//! Problems panel: the validation findings as a clickable list at the bottom
//! of the main window. Each finding knows which node of the project it
//! concerns, so clicking it selects that node in the tree.

use super::main_window::{Tab, XimodApp};
use crate::models::ValidationError;
use crate::models::verify::RefLoc;
use crate::ui::theme::{Palette, icon};
use eframe::egui::{self, RichText};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// The node a finding points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Info,
    Step(usize),
    Group(usize, usize),
    Plugin(usize, usize, usize),
    Required,
    CondSet(usize),
}

#[derive(Debug, Clone)]
pub struct Issue {
    pub severity: Severity,
    pub message: String,
    pub target: Option<Target>,
}

impl Issue {
    pub fn new(severity: Severity, message: impl Into<String>, target: Option<Target>) -> Self {
        Self {
            severity,
            message: message.into(),
            target,
        }
    }
}

/// Counts shown in the panel header and the status bar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IssueCounts {
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
}

pub fn count(issues: &[Issue]) -> IssueCounts {
    let mut c = IssueCounts::default();
    for i in issues {
        match i.severity {
            Severity::Error => c.errors += 1,
            Severity::Warning => c.warnings += 1,
            Severity::Info => c.infos += 1,
        }
    }
    c
}

impl XimodApp {
    /// The node a project validation error concerns (indices are 1-based in
    /// the error, as they are meant for display).
    pub(crate) fn target_of_validation_error(&self, err: &ValidationError) -> Option<Target> {
        match err {
            ValidationError::NoName => Some(Target::Info),
            ValidationError::NoSteps => None,
            ValidationError::EmptyStep { step } => Some(Target::Step(step.checked_sub(1)?)),
            ValidationError::EmptyGroup { step, group } => {
                Some(Target::Group(step.checked_sub(1)?, group.checked_sub(1)?))
            }
            ValidationError::NoPlugins { step, group } => {
                let si = step.checked_sub(1)?;
                let gi = self
                    .ximod
                    .steps
                    .get(si)?
                    .plugin_groups
                    .iter()
                    .position(|g| &g.name == group)?;
                Some(Target::Group(si, gi))
            }
        }
    }

    /// The node a file reference lives in.
    pub(crate) fn target_of_loc(&self, loc: &RefLoc) -> Option<Target> {
        match loc {
            RefLoc::Header => Some(Target::Info),
            RefLoc::RequiredFiles => Some(Target::Required),
            RefLoc::ConditionalSet { index } => Some(Target::CondSet(index.checked_sub(1)?)),
            RefLoc::Plugin { step, group, plugin } => {
                let si = step.checked_sub(1)?;
                let gi = group.checked_sub(1)?;
                let pi = self
                    .ximod
                    .steps
                    .get(si)?
                    .plugin_groups
                    .get(gi)?
                    .plugins
                    .iter()
                    .position(|p| &p.name == plugin)?;
                Some(Target::Plugin(si, gi, pi))
            }
        }
    }

    /// The node whose image is `rel` (header image or an option image).
    pub(crate) fn target_of_image(&self, rel: &str) -> Option<Target> {
        if self.ximod.header_image.as_deref() == Some(rel) {
            return Some(Target::Info);
        }
        for (si, step) in self.ximod.steps.iter().enumerate() {
            for (gi, group) in step.plugin_groups.iter().enumerate() {
                for (pi, plugin) in group.plugins.iter().enumerate() {
                    if plugin.image_path.as_deref() == Some(rel) {
                        return Some(Target::Plugin(si, gi, pi));
                    }
                }
            }
        }
        None
    }

    /// Select the node a finding points at.
    pub(crate) fn select_target(&mut self, target: Target) {
        match target {
            Target::Info => self.current_tab = Tab::Info,
            Target::Step(si) => {
                self.current_tab = Tab::Steps;
                self.select_step(Some(si));
            }
            Target::Group(si, gi) => {
                self.current_tab = Tab::Steps;
                self.select_step(Some(si));
                self.select_group(Some(gi));
            }
            Target::Plugin(si, gi, pi) => {
                self.current_tab = Tab::Steps;
                self.select_step(Some(si));
                self.select_group(Some(gi));
                self.select_plugin(Some(pi));
            }
            Target::Required => self.current_tab = Tab::RequiredInstalls,
            Target::CondSet(ci) => {
                self.current_tab = Tab::ConditionalInstalls;
                self.select_cond_pattern(Some(ci));
            }
        }
        self.clamp_selection();
    }

    /// Number of findings that concern a given node (for tree badges).
    pub(crate) fn issues_at(&self, target: Target) -> (usize, usize) {
        let mut errors = 0;
        let mut warnings = 0;
        for i in &self.validation_issues {
            if i.target == Some(target) {
                match i.severity {
                    Severity::Error => errors += 1,
                    Severity::Warning => warnings += 1,
                    Severity::Info => {}
                }
            }
        }
        (errors, warnings)
    }

    /// The bottom "Problems" panel. Hidden until a validation ran; the header
    /// toggles the list open/closed.
    pub(crate) fn render_problems_panel(&mut self, ctx: &egui::Context, modal_open: bool) {
        if !self.show_problems {
            return;
        }
        let counts = count(&self.validation_issues);
        let title = self.i18n.t("problems-title");
        let l_ok = self.i18n.t("validate-ok");
        let l_running = self.i18n.t("verify-running");
        let l_errors = self.i18n.t_num("problems-errors", counts.errors as i64);
        let l_warnings = self.i18n.t_num("problems-warnings", counts.warnings as i64);
        let l_close = self.i18n.t("btn-close");
        let l_rerun = self.i18n.t("menu-validate");
        let running = self.validation_rx.is_some();
        let mut go: Option<Target> = None;
        let mut close = false;
        let mut rerun = false;
        let expanded = self.problems_expanded;
        let mut toggle = false;

        egui::TopBottomPanel::bottom("problems")
            .resizable(expanded)
            .default_height(160.0)
            .min_height(28.0)
            .show(ctx, |ui| {
                if modal_open {
                    ui.disable();
                }
                let palette = Palette::from_ui(ui);
                ui.horizontal(|ui| {
                    let caret = if expanded { icon::EXPANDED } else { icon::COLLAPSED };
                    if ui.selectable_label(false, format!("{caret} {title}")).clicked() {
                        toggle = true;
                    }
                    if running {
                        ui.spinner();
                        ui.label(RichText::new(&l_running).weak());
                    } else if counts.errors == 0 && counts.warnings == 0 {
                        ui.label(RichText::new(format!("{} {}", icon::VALIDATE, l_ok)).color(palette.success));
                    } else {
                        if counts.errors > 0 {
                            ui.label(RichText::new(format!("{} {}", icon::ERROR, l_errors)).color(palette.danger));
                        }
                        if counts.warnings > 0 {
                            ui.label(RichText::new(format!("{} {}", icon::WARNING, l_warnings)).color(palette.warning));
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button(icon::CLOSE).on_hover_text(&l_close).clicked() {
                            close = true;
                        }
                        if ui.small_button(icon::REDO).on_hover_text(&l_rerun).clicked() {
                            rerun = true;
                        }
                    });
                });
                if !expanded {
                    return;
                }
                ui.separator();
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    for issue in &self.validation_issues {
                        let (glyph, color) = match issue.severity {
                            Severity::Error => (icon::ERROR, palette.danger),
                            Severity::Warning => (icon::WARNING, palette.warning),
                            Severity::Info => (icon::INFO, palette.accent),
                        };
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(glyph).color(color));
                            match issue.target {
                                Some(t) => {
                                    if ui.link(&issue.message).clicked() {
                                        go = Some(t);
                                    }
                                }
                                None => {
                                    ui.add(egui::Label::new(&issue.message).wrap());
                                }
                            }
                        });
                    }
                });
            });

        if toggle {
            self.problems_expanded = !self.problems_expanded;
        }
        if let Some(t) = go {
            self.select_target(t);
        }
        if close {
            self.show_problems = false;
        }
        if rerun {
            self.run_full_validation(ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Plugin, PluginGroup, SelectionType, Step};

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let mut step = Step::new("S");
        let mut group = PluginGroup::new("G", SelectionType::SelectAny);
        let mut p = Plugin::new("P");
        p.image_path = Some("fomod/img/p.png".into());
        group.plugins.push(p);
        step.plugin_groups.push(group);
        app.ximod.steps.push(step);
        app
    }

    #[test]
    fn targets_are_resolved() {
        let app = app();
        assert_eq!(
            app.target_of_validation_error(&ValidationError::EmptyGroup { step: 1, group: 1 }),
            Some(Target::Group(0, 0))
        );
        assert_eq!(
            app.target_of_validation_error(&ValidationError::NoPlugins {
                step: 1,
                group: "G".into()
            }),
            Some(Target::Group(0, 0))
        );
        assert_eq!(
            app.target_of_loc(&RefLoc::Plugin {
                step: 1,
                group: 1,
                plugin: "P".into()
            }),
            Some(Target::Plugin(0, 0, 0))
        );
        assert_eq!(
            app.target_of_loc(&RefLoc::ConditionalSet { index: 2 }),
            Some(Target::CondSet(1))
        );
        assert_eq!(app.target_of_image("fomod/img/p.png"), Some(Target::Plugin(0, 0, 0)));
        assert_eq!(app.target_of_image("nope.png"), None);
    }

    #[test]
    fn select_target_drives_the_selection() {
        let mut app = app();
        app.select_target(Target::Plugin(0, 0, 0));
        assert_eq!(app.current_tab, Tab::Steps);
        assert_eq!(app.selection.plugin, Some(0));
        app.select_target(Target::CondSet(5)); // out of range → clamped
        assert_eq!(app.current_tab, Tab::ConditionalInstalls);
        assert_eq!(app.selection.cond_pattern, None);
    }

    #[test]
    fn counts() {
        let issues = vec![
            Issue::new(Severity::Error, "a", None),
            Issue::new(Severity::Warning, "b", None),
            Issue::new(Severity::Warning, "c", None),
        ];
        assert_eq!(
            count(&issues),
            IssueCounts {
                errors: 1,
                warnings: 2,
                infos: 0
            }
        );
    }
}
