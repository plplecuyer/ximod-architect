//! Reusable UI components

#![allow(dead_code)]

use eframe::egui::{self, Color32, Response, RichText, Ui};

/// Section header with styled text
pub fn section_header(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).strong().size(14.0));
    ui.add_space(4.0);
}

/// Subsection header
pub fn subsection_header(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).strong());
}

/// Labeled text edit field
pub fn labeled_edit(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(value)
    })
    .inner
}

/// Labeled text edit field with fixed width
pub fn labeled_edit_sized(ui: &mut Ui, label: &str, value: &mut String, width: f32) -> Response {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add_sized([width, 20.0], egui::TextEdit::singleline(value))
    })
    .inner
}

/// Labeled multiline text edit
pub fn labeled_multiline(ui: &mut Ui, label: &str, value: &mut String, height: f32) -> Response {
    ui.vertical(|ui| {
        ui.label(label);
        ui.add_sized(
            [ui.available_width(), height],
            egui::TextEdit::multiline(value),
        )
    })
    .inner
}

/// Confirmation dialog component
pub struct ConfirmDialog {
    pub title: String,
    pub message: String,
    pub confirm_text: String,
    pub cancel_text: String,
}

impl ConfirmDialog {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            confirm_text: "OK".to_string(),
            cancel_text: "Cancel".to_string(),
        }
    }

    /// Show the dialog. Returns Some(true) if confirmed, Some(false) if cancelled, None if open
    pub fn show(&self, ctx: &egui::Context, open: &mut bool) -> Option<bool> {
        let mut result = None;
        let mut should_close = false;

        if *open {
            egui::Window::new(&self.title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(&self.message);
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        if ui.button(&self.confirm_text).clicked() {
                            result = Some(true);
                            should_close = true;
                        }
                        if ui.button(&self.cancel_text).clicked() {
                            result = Some(false);
                            should_close = true;
                        }
                    });
                });

            if should_close {
                *open = false;
            }
        }

        result
    }
}

/// Image display with fallback.
///
/// When an absolute image path is provided and the egui image loaders are
/// installed (see `install_image_loaders`), the image is rendered from disk.
/// Otherwise a bordered placeholder with `fallback_text` is drawn.
pub struct ImageDisplay {
    pub size: [f32; 2],
    pub fallback_text: String,
}

impl ImageDisplay {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: [width, height],
            fallback_text: "No Image".to_string(),
        }
    }

    /// Set the fallback text shown when no image is available (e.g. translated).
    pub fn with_fallback(mut self, text: impl Into<String>) -> Self {
        self.fallback_text = text.into();
        self
    }

    /// Draw the bordered placeholder with the fallback text.
    fn draw_placeholder(&self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(
            egui::Vec2::new(self.size[0], self.size[1]),
            egui::Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            ui.painter().rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(1.0_f32, Color32::from_gray(100)),
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &self.fallback_text,
                egui::FontId::default(),
                Color32::from_gray(150),
            );
        }

        response
    }

    /// Show the component. `abs_path` must be an absolute filesystem path to
    /// an existing image; pass `None` to always show the placeholder.
    pub fn show(&self, ui: &mut Ui, abs_path: Option<&std::path::Path>) -> Response {
        match abs_path {
            Some(path) if path.is_file() => {
                // egui's file loader expects a "file://" URI.
                let uri = format!("file://{}", path.to_string_lossy());
                let image = egui::Image::from_uri(uri)
                    .max_size(egui::Vec2::new(self.size[0], self.size[1]))
                    .maintain_aspect_ratio(true)
                    .fit_to_exact_size(egui::Vec2::new(self.size[0], self.size[1]))
                    .sense(egui::Sense::click());
                ui.add(image)
            }
            _ => self.draw_placeholder(ui),
        }
    }
}

/// File list item for display
pub struct FileListItem {
    pub file_type: String,
    pub source: String,
    pub destination: String,
    pub priority: u32,
}

/// Move item up in a vector
pub fn move_up<T>(vec: &mut Vec<T>, index: usize) -> bool {
    if index > 0 && index < vec.len() {
        vec.swap(index, index - 1);
        true
    } else {
        false
    }
}

/// Move item down in a vector
pub fn move_down<T>(vec: &mut Vec<T>, index: usize) -> bool {
    if index < vec.len().saturating_sub(1) {
        vec.swap(index, index + 1);
        true
    } else {
        false
    }
}

/// Single-line text field with an autocompletion popup drawn from `candidates`.
///
/// Suggestions are filtered case-insensitively by the current text (all shown
/// when it is empty); clicking one fills the field. `id_salt` must be unique per
/// field. Returns the text-edit `Response`.
pub fn autocomplete_edit(
    ui: &mut Ui,
    id_salt: &str,
    value: &mut String,
    candidates: &[String],
) -> Response {
    let resp = ui.text_edit_singleline(value);
    let popup_id = ui.make_persistent_id(id_salt);
    if resp.gained_focus() {
        ui.memory_mut(|m| m.open_popup(popup_id));
    }

    let needle = value.trim().to_lowercase();
    let matches: Vec<String> = candidates
        .iter()
        .filter(|c| !c.is_empty() && c.as_str() != value)
        .filter(|c| needle.is_empty() || c.to_lowercase().contains(&needle))
        .take(10)
        .cloned()
        .collect();

    if !matches.is_empty() {
        egui::popup_below_widget(
            ui,
            popup_id,
            &resp,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                ui.set_min_width(140.0);
                for m in &matches {
                    if ui.selectable_label(false, m).clicked() {
                        *value = m.clone();
                        ui.memory_mut(|mem| mem.close_popup());
                    }
                }
            },
        );
    }
    resp
}

/// Horizontal toolbar helper
pub fn toolbar(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        add_contents(ui);
    });
}

/// Status bar helper
pub fn status_bar(ui: &mut Ui, status: &str, modified: bool, modified_text: &str) {
    ui.horizontal(|ui| {
        ui.label(status);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if modified {
                ui.label(RichText::new("●").color(Color32::YELLOW));
                ui.label(modified_text);
            }
        });
    });
}

/// Small icon button used for every "clear / delete" action (filter clearing,
/// recent-file removal, …). The PNG is embedded in the binary, so it renders
/// identically on every platform and never depends on font glyph coverage.
/// Returns the `Response` so callers can add a tooltip and test `.clicked()`.
pub fn delete_button(ui: &mut Ui) -> Response {
    let img = egui::Image::new(egui::include_image!(
        "../../../assets/images/icons/delete.png"
    ))
    .fit_to_exact_size(egui::vec2(16.0, 16.0));
    ui.add(egui::ImageButton::new(img).frame(false))
}

/// Reorder ("move") buttons. They use embedded PNG arrows instead of the Unicode
/// geometric shapes (▲ ▼ ◀ ▶), which the bundled Noto Sans font does not cover —
/// those rendered as empty squares. Embedding the PNGs guarantees identical
/// rendering on every platform, independent of font glyph coverage. Each helper
/// returns an `ImageButton` widget so callers can wrap it in `ui.add_enabled(..)`.
pub fn arrow_up_button() -> egui::ImageButton<'static> {
    egui::ImageButton::new(
        egui::Image::new(egui::include_image!(
            "../../../assets/images/icons/Arrow_up.png"
        ))
        .fit_to_exact_size(egui::vec2(14.0, 14.0)),
    )
}

pub fn arrow_down_button() -> egui::ImageButton<'static> {
    egui::ImageButton::new(
        egui::Image::new(egui::include_image!(
            "../../../assets/images/icons/Arrow_down.png"
        ))
        .fit_to_exact_size(egui::vec2(14.0, 14.0)),
    )
}

pub fn arrow_left_button() -> egui::ImageButton<'static> {
    egui::ImageButton::new(
        egui::Image::new(egui::include_image!(
            "../../../assets/images/icons/Arrow_left.png"
        ))
        .fit_to_exact_size(egui::vec2(14.0, 14.0)),
    )
}

pub fn arrow_right_button() -> egui::ImageButton<'static> {
    egui::ImageButton::new(
        egui::Image::new(egui::include_image!(
            "../../../assets/images/icons/Arrow_right.png"
        ))
        .fit_to_exact_size(egui::vec2(14.0, 14.0)),
    )
}

#[cfg(test)]
mod tests {
    use super::{move_down, move_up};

    #[test]
    fn reorder_helpers() {
        let mut v = vec![1, 2, 3];
        assert!(move_up(&mut v, 2));
        assert_eq!(v, vec![1, 3, 2]);
        assert!(!move_up(&mut v, 0)); // first can't move up
        assert!(move_down(&mut v, 0));
        assert_eq!(v, vec![3, 1, 2]);
        assert!(!move_down(&mut v, 2)); // last can't move down
    }
}
