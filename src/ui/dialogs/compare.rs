//! FOMOD comparison (diff) window.

use crate::ui::main_window::XimodApp;
use eframe::egui;

impl XimodApp {
    /// Compare the current project with another FOMOD picked from disk.
    /// The diff reads as "changes from the picked version to the current one":
    /// added = present here but not there.
    pub(crate) fn compare_with(&mut self) {
        let Some(path) = rfd::FileDialog::new().pick_folder() else {
            return;
        };
        match crate::xml::load_ximod(&path) {
            Ok(other) => {
                let diff = crate::models::compare::diff_projects(&other, &self.ximod);
                self.compare_other_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.to_string_lossy().to_string());
                self.compare_result = Some(diff);
                self.show_compare = true;
            }
            Err(e) => {
                self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string()));
            }
        }
    }

    /// Render the FOMOD comparison window.
    pub(crate) fn render_compare_window(&mut self, ctx: &egui::Context) {
        if !self.show_compare {
            return;
        }
        let title = self.i18n.t("compare-title");
        let none = self.i18n.t("compare-none");
        let close = self.i18n.t("btn-ok");
        let other = self.compare_other_name.clone();
        let diff = &self.compare_result;
        let mut open = self.show_compare;
        let mut do_close = false;

        egui::Window::new(title)
            .open(&mut open)
            .resizable(true)
            .default_size([600.0, 460.0])
            .show(ctx, |ui| {
                match &diff {
                    Some(d) => ui.small(format!("↔ {other} · Δ {}", d.total())),
                    None => ui.small(format!("↔ {other}")),
                };
                ui.separator();
                match &diff {
                    Some(d) if d.is_empty() => {
                        ui.label(&none);
                    }
                    Some(d) => {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for m in &d.meta_changes {
                                ui.label(format!("~ {m}"));
                            }
                            for it in d.steps_added.iter().chain(&d.options_added).chain(&d.files_added) {
                                ui.colored_label(
                                    crate::ui::theme::Palette::from_ui(ui).success,
                                    format!("+ {}", it.path),
                                );
                            }
                            for it in d.steps_removed.iter().chain(&d.options_removed).chain(&d.files_removed) {
                                ui.colored_label(
                                    crate::ui::theme::Palette::from_ui(ui).danger,
                                    format!("− {}", it.path),
                                );
                            }
                        });
                    }
                    None => {}
                }
                ui.separator();
                if ui.button(&close).clicked() {
                    do_close = true;
                }
            });

        if do_close {
            open = false;
        }
        self.show_compare = open;
    }
}
