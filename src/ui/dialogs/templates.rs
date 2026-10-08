//! Reusable step templates window.

use crate::ui::main_window::XimodApp;
use eframe::egui;

impl XimodApp {
    /// Save the currently selected step as a reusable template, named from
    /// `template_name_buf` (falls back to the step's name).
    pub(crate) fn save_current_step_as_template(&mut self) {
        let Some(si) = self.selection.step else {
            self.notify_err(self.i18n.t("msg-template-no-step"));
            return;
        };
        let Some(step) = self.ximod.steps.get(si) else {
            self.notify_err(self.i18n.t("msg-template-no-step"));
            return;
        };
        let name = {
            let n = self.template_name_buf.trim();
            if n.is_empty() { step.name.clone() } else { n.to_string() }
        };
        let name = if name.trim().is_empty() {
            crate::ui::labels::step_fallback(&self.i18n, si)
        } else {
            name
        };
        let tpl = crate::models::templates::Template {
            name: name.clone(),
            description: String::new(),
            body: crate::models::templates::TemplateBody::Step(step.clone()),
        };
        let Some(dir) = crate::models::templates::templates_dir() else {
            self.notify_err(self.i18n.t("msg-template-no-dir"));
            return;
        };
        match crate::models::templates::save_template(&dir, &tpl) {
            Ok(_) => {
                self.template_name_buf.clear();
                self.templates = crate::models::templates::load_templates(&dir).unwrap_or_default();
                self.notify_ok(self.i18n.t_arg("msg-template-saved", "name", &name));
            }
            Err(e) => {
                self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string()));
            }
        }
    }

    /// Insert the template at `idx` into the current project (appended).
    pub(crate) fn insert_template(&mut self, idx: usize) {
        if let Some(tpl) = self.templates.get(idx).cloned() {
            crate::models::templates::apply_template(&mut self.ximod, &tpl);
            self.mark_modified();
            self.notify_ok(self.i18n.t("msg-template-inserted"));
        }
    }

    /// Render the reusable-templates window.
    pub(crate) fn render_templates_window(&mut self, ctx: &egui::Context) {
        if !self.show_templates {
            return;
        }
        let title = self.i18n.t("templates-title");
        let empty = self.i18n.t("templates-empty");
        let insert = self.i18n.t("templates-insert");
        let save_step = self.i18n.t("templates-save-step");
        let name_hint = self.i18n.t("templates-name-hint");
        let close = self.i18n.t("btn-ok");

        let mut open = self.show_templates;
        let mut to_insert: Option<usize> = None;
        let mut do_save = false;
        let mut do_close = false;

        egui::Window::new(title)
            .open(&mut open)
            .resizable(true)
            .default_size([460.0, 360.0])
            .show(ctx, |ui| {
                // Save the current step as a template.
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.template_name_buf)
                            .hint_text(&name_hint)
                            .desired_width(220.0),
                    );
                    if ui.button(&save_step).clicked() {
                        do_save = true;
                    }
                });
                ui.separator();

                if self.templates.is_empty() {
                    ui.label(&empty);
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, tpl) in self.templates.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("[{}]", tpl.body.kind_tag()));
                                ui.label(&tpl.name);
                                if ui.button(&insert).clicked() {
                                    to_insert = Some(i);
                                }
                            });
                            if !tpl.description.trim().is_empty() {
                                ui.small(&tpl.description);
                            }
                        }
                    });
                }

                ui.separator();
                if ui.button(&close).clicked() {
                    do_close = true;
                }
            });

        if do_save {
            self.save_current_step_as_template();
        }
        if let Some(i) = to_insert {
            self.insert_template(i);
        }
        if do_close {
            open = false;
        }
        self.show_templates = open;
    }
}
