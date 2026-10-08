//! About window.

use crate::ui::main_window::XimodApp;
use crate::ui::widgets::free_window::record_win_geom;
use eframe::egui;

/// Fixed size of the "About" window (independent, non-resizable).
const ABOUT_SIZE: [f32; 2] = [430.0, 300.0];

impl XimodApp {
    pub(crate) fn render_about_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_about {
            return;
        }

        let title = self.i18n.t("about-title");
        let app_name = self.i18n.t("app-title");
        let version_str = self.i18n.t_arg("app-version", "version", env!("CARGO_PKG_VERSION"));
        let desc = self.i18n.t("about-description");
        let license = self.i18n.t("about-license");
        let copyright = self.i18n.t("about-copyright");
        let credit = self.i18n.t("about-credit");
        let ok_text = self.i18n.t("btn-ok");

        let mut should_close = false;

        // Independent OS-level window (freely movable, incl. onto another
        // screen), but of FIXED size: not resizable by the user. Only its
        // position is remembered in Config.ini.
        let vb = self
            .free_viewport_builder(ctx, "ximod_about", title, ABOUT_SIZE, true)
            .with_resizable(false)
            .with_min_inner_size(ABOUT_SIZE)
            .with_max_inner_size(ABOUT_SIZE);
        let cfg = &mut self.config;
        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("ximod_about"), vb, |ctx, _class| {
            // The OK button lives in a bottom panel so it is never pushed out
            // of this fixed-size window by the text above it.
            egui::TopBottomPanel::bottom("ximod_about_buttons").show(ctx, |ui| {
                ui.add_space(6.0);
                ui.vertical_centered(|ui| {
                    if ui.button(&ok_text).clicked() {
                        should_close = true;
                    }
                });
                ui.add_space(4.0);
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(6.0);
                        ui.heading(&app_name);
                        ui.label(&version_str);
                        ui.add_space(8.0);
                        ui.label(&desc);
                        ui.add_space(8.0);
                        ui.label(&license);
                        ui.add_space(4.0);
                        ui.label(&copyright);
                        ui.add_space(12.0);
                        // Credit to the original author (as agreed with
                        // Wenderer): a line of text plus a link.
                        ui.label(&credit);
                        ui.hyperlink_to(
                            "Wenderer — FOMOD Creation Tool",
                            "https://www.nexusmods.com/fallout4/mods/6821",
                        );
                        ui.add_space(8.0);
                    });
                });
            });

            record_win_geom(cfg, ctx, "ximod_about");
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                should_close = true;
            }
        });

        if should_close {
            self.show_about = false;
            self.free_window_closed("ximod_about");
        }
    }
}
