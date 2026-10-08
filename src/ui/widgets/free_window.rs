//! Free (independent viewport) tool windows: placement and geometry persistence.

use crate::ui::main_window::XimodApp;
use eframe::egui;

/// Sample a free window's current outer position and inner size into the config,
/// keyed by viewport id. Called each frame from inside a viewport closure (with
/// the child context). Takes `&mut AppConfig` so it works both when the closure
/// borrows the whole app and when it only borrows the config field disjointly.
pub(crate) fn record_win_geom(config: &mut crate::config::AppConfig, ctx: &egui::Context, id: &str) {
    let (outer, inner) = ctx.input(|i| {
        let vp = i.viewport();
        (vp.outer_rect, vp.inner_rect)
    });
    if let Some(o) = outer {
        config.window_positions.insert(id.to_string(), (o.min.x, o.min.y));
    }
    if let Some(r) = inner {
        let sz = r.size();
        if sz.x > 1.0 && sz.y > 1.0 {
            config.window_sizes.insert(id.to_string(), (sz.x, sz.y));
        }
    }
}

impl XimodApp {
    /// Build the viewport for a free tool window, placing it on open: at its
    /// saved position if the user moved it before, otherwise centered on the main
    /// window. The position is decided once per open-session and kept stable so
    /// egui does not fight the user dragging the window.
    pub(crate) fn free_viewport_builder(
        &mut self,
        ctx: &egui::Context,
        id: &str,
        title: String,
        size: [f32; 2],
        fixed_size: bool,
    ) -> egui::ViewportBuilder {
        let mut builder = egui::ViewportBuilder::default().with_title(title);
        // Fixed-size windows (Settings, About) must always open at their coded
        // size, so drop any size that a previous session saved in Config.ini —
        // otherwise the stale saved size would override the constant.
        if fixed_size {
            self.config.window_sizes.remove(id);
        }
        // Apply the saved (or centered) geometry ONLY on the opening frame.
        // Re-applying `with_inner_size`/`with_position` every frame made egui/winit
        // keep snapping the window back to that exact size and position — which
        // showed up as a constant tremble and fought the user's manual moves and
        // resizes. After the first frame we leave the geometry to the OS/user, and
        // `record_win_geom` (called each frame in the window body) persists it.
        if !self.win_initialized.contains(id) {
            // Size: the saved size if the window was resized before, else the
            // caller's default.
            let win_size = self.config.window_sizes.get(id).map(|&(w, h)| [w, h]).unwrap_or(size);
            // Position: the saved position if the window was moved before, else
            // centered on the main window.
            let pos = match self.config.window_positions.get(id) {
                Some(&(x, y)) => egui::pos2(x, y),
                None => {
                    let main = ctx.input(|i| i.viewport().outer_rect).unwrap_or_else(|| {
                        egui::Rect::from_min_size(egui::pos2(80.0, 80.0), egui::vec2(1280.0, 800.0))
                    });
                    let c = main.center();
                    egui::pos2(c.x - win_size[0] / 2.0, c.y - win_size[1] / 2.0)
                }
            };
            self.win_pos.insert(id.to_string(), pos);
            self.win_size.insert(id.to_string(), (win_size[0], win_size[1]));
            self.win_initialized.insert(id.to_string());
            builder = builder.with_inner_size(win_size).with_position(pos).with_active(true);
        }
        builder
    }

    /// Called when a free window closes: persist its geometry (the live position
    /// and size are already recorded into the config each frame) and clear the
    /// per-session placement so the next open re-reads it.
    pub(crate) fn free_window_closed(&mut self, id: &str) {
        if self.win_initialized.remove(id) {
            self.win_pos.remove(id);
            self.win_size.remove(id);
            let _ = self.config.save();
        }
    }
}
