//! Transient notifications ("toasts") stacked in the top-right corner of the
//! main window.
//!
//! The status bar keeps the last message as a permanent line, but a single
//! grey line at the bottom of a 1280×800 window is easy to miss: users
//! reported that "nothing happens" when a save or an export failed. Toasts
//! make successes visible for a few seconds and keep errors on screen until
//! dismissed. Deliberately dependency-free (about a hundred lines) so the
//! look can follow the application's own theme.

use eframe::egui;

/// Visual category of a toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastLevel {
    /// How long the toast stays visible, in seconds. `None` = until closed.
    fn lifetime(self) -> Option<f64> {
        match self {
            ToastLevel::Info => Some(4.0),
            ToastLevel::Success => Some(4.0),
            ToastLevel::Warning => Some(7.0),
            ToastLevel::Error => None,
        }
    }

    fn glyph(self) -> &'static str {
        use crate::ui::theme::icon;
        match self {
            ToastLevel::Info => icon::INFO,
            ToastLevel::Success => icon::VALIDATE,
            ToastLevel::Warning => icon::WARNING,
            ToastLevel::Error => icon::ERROR,
        }
    }

    /// Accent colour, readable on both the dark and the light theme.
    fn color(self, dark: bool) -> egui::Color32 {
        match (self, dark) {
            (ToastLevel::Info, true) => egui::Color32::from_rgb(120, 170, 255),
            (ToastLevel::Info, false) => egui::Color32::from_rgb(30, 90, 190),
            (ToastLevel::Success, true) => egui::Color32::from_rgb(110, 200, 120),
            (ToastLevel::Success, false) => egui::Color32::from_rgb(30, 130, 50),
            (ToastLevel::Warning, true) => egui::Color32::from_rgb(240, 190, 80),
            (ToastLevel::Warning, false) => egui::Color32::from_rgb(170, 110, 0),
            (ToastLevel::Error, true) => egui::Color32::from_rgb(240, 110, 110),
            (ToastLevel::Error, false) => egui::Color32::from_rgb(190, 40, 40),
        }
    }
}

struct Toast {
    level: ToastLevel,
    text: String,
    /// `ctx.input(|i| i.time)` when the toast was created.
    created: f64,
    id: u64,
}

/// The toast stack. Owned by the application; `show` is called once per frame.
#[derive(Default)]
pub struct Toasts {
    items: Vec<Toast>,
    next_id: u64,
}

impl Toasts {
    /// Queue a toast. An identical message already on screen is refreshed
    /// instead of duplicated (repeated clicks on the same action).
    pub fn push(&mut self, level: ToastLevel, text: impl Into<String>, now: f64) {
        let text = text.into();
        if let Some(existing) = self.items.iter_mut().find(|t| t.text == text && t.level == level) {
            existing.created = now;
            return;
        }
        self.items.push(Toast {
            level,
            text,
            created: now,
            id: self.next_id,
        });
        self.next_id = self.next_id.wrapping_add(1);
        // Never let the stack grow without bound.
        if self.items.len() > 6 {
            self.items.remove(0);
        }
    }

    /// Draw the stack and drop expired toasts.
    pub fn show(&mut self, ctx: &egui::Context) {
        if self.items.is_empty() {
            return;
        }
        let now = ctx.input(|i| i.time);
        self.items.retain(|t| match t.level.lifetime() {
            Some(life) => now - t.created < life,
            None => true,
        });
        if self.items.is_empty() {
            return;
        }
        // A timed toast must disappear even if nothing else repaints.
        let next_expiry = self
            .items
            .iter()
            .filter_map(|t| t.level.lifetime().map(|l| (t.created + l - now).max(0.0)))
            .fold(f64::INFINITY, f64::min);
        if next_expiry.is_finite() {
            ctx.request_repaint_after(std::time::Duration::from_secs_f64(next_expiry.min(1.0)));
        }

        let dark = ctx.style().visuals.dark_mode;
        let mut close: Option<u64> = None;
        egui::Area::new(egui::Id::new("ximod_toasts"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 44.0))
            .interactable(true)
            .show(ctx, |ui| {
                ui.set_max_width(420.0);
                for t in &self.items {
                    let accent = t.level.color(dark);
                    let frame = egui::Frame::popup(ui.style())
                        .stroke(egui::Stroke::new(1.0_f32, accent))
                        .inner_margin(egui::Margin::symmetric(10.0, 8.0));
                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(t.level.glyph()).color(accent).strong());
                            ui.add(egui::Label::new(egui::RichText::new(&t.text)).wrap_mode(egui::TextWrapMode::Wrap));
                            if t.level == ToastLevel::Error && ui.small_button(crate::ui::theme::icon::CLOSE).clicked()
                            {
                                close = Some(t.id);
                            }
                        });
                    });
                    ui.add_space(4.0);
                }
            });
        if let Some(id) = close {
            self.items.retain(|t| t.id != id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates_are_refreshed_not_stacked() {
        let mut t = Toasts::default();
        t.push(ToastLevel::Success, "Saved", 0.0);
        t.push(ToastLevel::Success, "Saved", 1.0);
        assert_eq!(t.items.len(), 1);
        assert_eq!(t.items[0].created, 1.0);
        t.push(ToastLevel::Error, "Saved", 1.0);
        assert_eq!(t.items.len(), 2, "same text, different level is a new toast");
    }

    #[test]
    fn stack_is_bounded() {
        let mut t = Toasts::default();
        for i in 0..20 {
            t.push(ToastLevel::Info, format!("m{i}"), i as f64);
        }
        assert_eq!(t.items.len(), 6);
        assert_eq!(t.items[0].text, "m14");
    }
}
