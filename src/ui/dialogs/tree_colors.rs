//! Tools → Tree colours…: one swatch per kind of row of the project tree.
//! Clicking a swatch opens the colour picker (an independent window ported
//! from XIMOD Translator); OK writes the colour into the configuration at
//! once, so the tree reflects it immediately. The window edits the set of the
//! theme currently in use and says so; "Reset colours" restores that set.

use crate::tree_colors::{TreeColors, TreeKind};
use crate::ui::color_picker::PickerTarget;
use crate::ui::main_window::XimodApp;
use crate::ui::theme::icon;
use eframe::egui::{self, Color32, Rounding, Sense, Stroke};

const WINDOW_ID: &str = "ximod_tree_colors";
const PICKER_ID: &str = "ximod_color_picker";
const WINDOW_SIZE: [f32; 2] = [440.0, 330.0];
/// Same footprint as the Translator's picker, plus room for the taller tab.
const PICKER_SIZE: [f32; 2] = [560.0, 400.0];

/// The icon shown next to each kind in the editor (the one the tree uses).
fn kind_icon(kind: TreeKind) -> &'static str {
    match kind {
        TreeKind::Info => icon::INFO,
        TreeKind::Step => icon::STEP,
        TreeKind::Group => icon::GROUP,
        TreeKind::Option => icon::OPTION,
        TreeKind::Required => icon::REQUIRED,
        TreeKind::Conditional => icon::CONDITIONAL,
    }
}

impl XimodApp {
    /// Open the tree colours window (Tools menu).
    pub fn open_tree_colors(&mut self) {
        self.show_tree_colors = true;
    }

    /// Render the tree colours window (independent viewport).
    pub fn render_tree_colors(&mut self, ctx: &egui::Context) {
        if !self.show_tree_colors {
            self.free_window_closed(WINDOW_ID);
            return;
        }
        let dark = ctx.style().visuals.dark_mode;
        let title = self.i18n.t("tree-colors-title");
        let hint = self.i18n.t("tree-colors-hint");
        let theme_note = self.i18n.t(if dark {
            "tree-colors-theme-dark"
        } else {
            "tree-colors-theme-light"
        });
        let click = self.i18n.t("tree-colors-click");
        let reset_label = self.i18n.t("tree-colors-reset");
        let close_label = self.i18n.t("btn-close");
        let labels: Vec<(TreeKind, String)> = TreeKind::ALL.iter().map(|k| (*k, self.i18n.t(k.label_key()))).collect();
        let vb = self.free_viewport_builder(ctx, WINDOW_ID, title, WINDOW_SIZE, true);

        let cfg = &mut self.config;
        let picker = &mut self.color_picker;
        let mut do_close = false;
        let mut reset = false;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of(WINDOW_ID), vb, |ctx, _class| {
            egui::TopBottomPanel::bottom("tree_colors_bottom").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui.button(&reset_label).clicked() {
                        reset = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&close_label).clicked() {
                            do_close = true;
                        }
                    });
                });
                ui.add_space(4.0);
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(&hint);
                ui.label(egui::RichText::new(&theme_note).weak().small());
                ui.add_space(8.0);
                let colors: TreeColors = *cfg.tree_colors(dark);
                // Four aligned columns: swatch · icon · label · RGB, as in the
                // Translator's status-colours window.
                egui::Grid::new("tree_colors_grid")
                    .num_columns(4)
                    .spacing([12.0, 6.0])
                    .show(ui, |ui| {
                        for (kind, label) in &labels {
                            let (r, g, b) = colors.get(*kind);
                            let (rect, resp) = ui.allocate_exact_size(egui::vec2(48.0, 20.0), Sense::click());
                            ui.painter()
                                .rect_filled(rect, Rounding::same(3.0), Color32::from_rgb(r, g, b));
                            ui.painter()
                                .rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0_f32, Color32::GRAY));
                            if resp.on_hover_text(&click).clicked() {
                                picker.open_for(PickerTarget::TreeColor { kind: *kind, dark }, (r, g, b));
                            }
                            ui.label(egui::RichText::new(kind_icon(*kind)).color(Color32::from_rgb(r, g, b)));
                            ui.label(label);
                            ui.label(format!("{r}, {g}, {b}"));
                            ui.end_row();
                        }
                    });
            });
            crate::ui::widgets::free_window::record_win_geom(cfg, ctx, WINDOW_ID);
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        if reset {
            *self.config.tree_colors_mut(dark) = TreeColors::defaults(dark);
            let _ = self.config.save();
        }
        if do_close {
            self.show_tree_colors = false;
            self.color_picker.close();
            self.free_window_closed(WINDOW_ID);
            self.free_window_closed(PICKER_ID);
        }
    }

    /// Render the colour picker window (independent viewport, fixed size) when
    /// it is open. The chosen colour is written straight into the configuration.
    pub fn render_color_picker(&mut self, ctx: &egui::Context) {
        if !self.color_picker.is_open() {
            self.free_window_closed(PICKER_ID);
            return;
        }
        let title = self.i18n.t("picker-title");
        let vb = self.free_viewport_builder(ctx, PICKER_ID, title, PICKER_SIZE, true);

        let picker = &mut self.color_picker;
        let cfg = &mut self.config;
        let i18n = &self.i18n;
        let mut committed = None;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of(PICKER_ID), vb, |ctx, _class| {
            picker.needs_center = false;
            egui::CentralPanel::default().show(ctx, |ui| {
                committed = picker.ui(ui, i18n);
            });
            crate::ui::widgets::free_window::record_win_geom(cfg, ctx, PICKER_ID);
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                picker.close();
            }
        });

        if let Some((PickerTarget::TreeColor { kind, dark }, rgb)) = committed {
            self.config.tree_colors_mut(dark).set(kind, rgb);
            let _ = self.config.save();
        }
        if !self.color_picker.is_open() {
            self.free_window_closed(PICKER_ID);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both windows render headless, and a committed colour lands in the right
    /// set of the configuration.
    #[test]
    fn windows_render_headless_and_commit_to_config() {
        let mut app = XimodApp::default();
        app.open_tree_colors();
        app.color_picker.open_for(
            PickerTarget::TreeColor {
                kind: TreeKind::Group,
                dark: true,
            },
            (1, 2, 3),
        );
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            app.render_tree_colors(ctx);
            app.render_color_picker(ctx);
        });
        assert!(app.show_tree_colors);
        assert!(app.color_picker.is_open());
        // Simulate OK: the picker reports its target and colour.
        app.config.tree_colors_mut(true).set(TreeKind::Group, (9, 8, 7));
        assert_eq!(app.config.tree_colors(true).get(TreeKind::Group), (9, 8, 7));
        assert_eq!(
            app.config.tree_colors(false).get(TreeKind::Group),
            TreeColors::LIGHT.get(TreeKind::Group)
        );
    }
}
