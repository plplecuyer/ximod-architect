//! Pre/post-save script editor window.

use crate::ui::main_window::XimodApp;
use crate::ui::widgets::free_window::record_win_geom;
use eframe::egui;

impl XimodApp {
    pub(crate) fn render_script_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_script_dialog {
            return;
        }

        let title = if self.editing_pre_script {
            self.i18n.t("menu-pre-save-script")
        } else {
            self.i18n.t("menu-post-save-script")
        };
        let info = self.i18n.t("script-info");
        let macros_title = self.i18n.t("script-macros");
        let btn_save = self.i18n.t("btn-save");
        let btn_cancel = self.i18n.t("btn-cancel");
        let macro_lines = [
            self.i18n.t("macro-modname"),
            self.i18n.t("macro-modauthor"),
            self.i18n.t("macro-modversion"),
            self.i18n.t("macro-modroot"),
            self.i18n.t("macro-date"),
            self.i18n.t("macro-time"),
            self.i18n.t("macro-random"),
        ];

        let mut should_close = false;
        let mut should_save = false;
        // Independent, freely movable OS-level window; resizable, with its
        // position and size remembered in Config.ini.
        let vb = self.free_viewport_builder(ctx, "ximod_script", title, [500.0, 420.0], false);
        let content = &mut self.script_content;
        let cfg = &mut self.config;
        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("ximod_script"), vb, |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(&info);

                ui.collapsing(&macros_title, |ui| {
                    for line in &macro_lines {
                        ui.label(line);
                    }
                });

                ui.add_space(8.0);

                let editor_height = (ui.available_height() - 48.0).max(80.0);
                ui.add_sized(
                    [ui.available_width(), editor_height],
                    egui::TextEdit::multiline(content).font(egui::TextStyle::Monospace),
                );

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    if ui.button(&btn_save).clicked() {
                        should_save = true;
                        should_close = true;
                    }
                    if ui.button(&btn_cancel).clicked() {
                        should_close = true;
                    }
                });
            });

            record_win_geom(cfg, ctx, "ximod_script");
            // Escape closes the script editor without saving (same as Cancel).
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                should_close = true;
            }
        });

        if should_save {
            // Save script content directly to config
            if self.editing_pre_script {
                self.config.pre_save_script = self.script_content.clone();
            } else {
                self.config.post_save_script = self.script_content.clone();
            }
            match self.config.save() {
                Err(e) => {
                    self.notify_err(format!("{}: {}", self.i18n.t("msg-script-save-error"), e));
                }
                _ => {
                    self.notify_ok(self.i18n.t("status-settings-saved"));
                }
            }
        }

        if should_close {
            self.show_script_dialog = false;
            self.free_window_closed("ximod_script");
        }
    }
}
