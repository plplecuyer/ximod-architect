//! Reusable UI components

#![allow(dead_code)]

use eframe::egui::{self, Color32, Response, RichText, Ui};

/// Section header with styled text
pub fn section_header(ui: &mut Ui, text: &str) {
    // Derive the size from the Heading style so the user's font-size setting
    // applies, instead of a hard-coded 14 px.
    ui.label(RichText::new(text).strong().text_style(egui::TextStyle::Heading));
    ui.add_space(4.0);
}

/// Section header with an "(i)" help marker whose tooltip explains the
/// concept (flags, priorities, destinations…) to non-developer modders.
pub fn section_header_hint(ui: &mut Ui, text: &str, hint: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(text).strong().text_style(egui::TextStyle::Heading));
        ui.label(RichText::new("ⓘ").weak()).on_hover_text(hint);
    });
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
        ui.add_sized([ui.available_width(), height], egui::TextEdit::multiline(value))
    })
    .inner
}

/// Confirmation dialog component
/// Dim everything behind a modal dialog so the user sees at a glance that
/// the rest of the window is blocked. Call once per frame per open modal
/// (several calls share one layer, so stacking does not darken twice).
pub fn modal_veil(ctx: &egui::Context) {
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Middle,
        egui::Id::new("ximod_modal_veil"),
    ));
    painter.rect_filled(ctx.screen_rect(), 0.0, Color32::from_black_alpha(96));
}

/// Keyboard handling shared by every modal: Enter = primary action,
/// Escape = cancel. Consumes the keys so nothing behind reacts to them.
pub fn modal_keys(ctx: &egui::Context) -> (bool, bool) {
    ctx.input_mut(|i| {
        let enter = i.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
        let esc = i.consume_key(egui::Modifiers::NONE, egui::Key::Escape);
        (enter, esc)
    })
}

/// A centred modal window on top of the veil. Returns whatever `body` does.
pub fn modal_window<R>(ctx: &egui::Context, title: &str, body: impl FnOnce(&mut Ui) -> R) -> Option<R> {
    modal_veil(ctx);
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, body)
        .and_then(|r| r.inner)
}

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
            let (enter, esc) = modal_keys(ctx);
            modal_window(ctx, &self.title, |ui| {
                ui.set_max_width(560.0);
                // Long messages (the "save anyway" list of issues) scroll
                // instead of pushing the buttons off screen.
                egui::ScrollArea::vertical()
                    .max_height(320.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        ui.label(&self.message);
                    });
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    let primary = egui::Button::new(RichText::new(&self.confirm_text).strong());
                    if ui.add(primary).clicked() || enter {
                        result = Some(true);
                        should_close = true;
                    }
                    if ui.button(&self.cancel_text).clicked() || esc {
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
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::new(self.size[0], self.size[1]), egui::Sense::click());

        if ui.is_rect_visible(rect) {
            ui.painter()
                .rect_stroke(rect, 0.0, egui::Stroke::new(1.0_f32, Color32::from_gray(100)));
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
    ///
    /// The "does the file exist" answer and the `file://` URI are cached per
    /// path (see [`image_probe`]) instead of costing a `stat` syscall and a
    /// string format per image per frame.
    pub fn show(&self, ui: &mut Ui, abs_path: Option<&std::path::Path>) -> Response {
        let uri = abs_path.and_then(|path| image_probe(ui.ctx(), path));
        match uri {
            Some(uri) => {
                let image = egui::Image::from_uri(&*uri)
                    .max_size(egui::Vec2::new(self.size[0], self.size[1]))
                    .maintain_aspect_ratio(true)
                    .fit_to_exact_size(egui::Vec2::new(self.size[0], self.size[1]))
                    .sense(egui::Sense::click());
                ui.add(image)
            }
            None => self.draw_placeholder(ui),
        }
    }
}

/// The `file://` URI egui's file loader expects for a path on disk.
pub fn file_uri(path: &std::path::Path) -> String {
    format!("file://{}", path.to_string_lossy())
}

/// How long an "is this a file?" answer is trusted, in seconds. Short enough
/// that an image created (or deleted) behind our back shows up promptly.
const IMAGE_PROBE_TTL: f64 = 2.0;
/// Safety valve: the cache is emptied when it grows past this many paths.
const IMAGE_PROBE_MAX: usize = 1024;

#[derive(Clone)]
struct ImageProbe {
    exists: bool,
    uri: std::sync::Arc<str>,
    /// egui time (`InputState::time`, seconds) of the last `stat`.
    checked: f64,
}

/// Path → probe result, shared by every `ImageDisplay` of the application.
type ImageProbeCache = std::sync::Arc<egui::mutex::Mutex<std::collections::HashMap<std::path::PathBuf, ImageProbe>>>;

fn image_probe_cache(ctx: &egui::Context) -> ImageProbeCache {
    ctx.data_mut(|d| {
        d.get_temp_mut_or_default::<ImageProbeCache>(egui::Id::new("ximod_image_probe_cache"))
            .clone()
    })
}

/// Cached `path.is_file()`: returns the image URI when the file exists.
/// The filesystem is queried at most once every [`IMAGE_PROBE_TTL`] seconds
/// per path.
fn image_probe(ctx: &egui::Context, path: &std::path::Path) -> Option<std::sync::Arc<str>> {
    let now = ctx.input(|i| i.time);
    let cache = image_probe_cache(ctx);
    let mut map = cache.lock();
    if let Some(p) = map.get_mut(path) {
        // `now < checked` only if the clock was reset: treat as expired.
        if now < p.checked || now - p.checked >= IMAGE_PROBE_TTL {
            p.exists = path.is_file();
            p.checked = now;
        }
        return p.exists.then(|| p.uri.clone());
    }
    if map.len() >= IMAGE_PROBE_MAX {
        map.clear();
    }
    let probe = ImageProbe {
        exists: path.is_file(),
        uri: file_uri(path).into(),
        checked: now,
    };
    let out = probe.exists.then(|| probe.uri.clone());
    map.insert(path.to_path_buf(), probe);
    out
}

/// Forget everything cached about the image at `path`: the decoded texture
/// held by egui's image loaders and our own existence/URI cache entry. Call it
/// after the file was rewritten in place (same path, new pixels) or created,
/// so the next frame reloads it from disk.
pub fn forget_image_uri(ctx: &egui::Context, path: &std::path::Path) {
    ctx.forget_image(&file_uri(path));
    image_probe_cache(ctx).lock().remove(path);
}

/// File list item for display
pub struct FileListItem {
    pub file_type: String,
    pub source: String,
    pub destination: String,
    pub priority: u32,
}

/// Move item up in a vector
pub fn move_up<T>(vec: &mut [T], index: usize) -> bool {
    if index > 0 && index < vec.len() {
        vec.swap(index, index - 1);
        true
    } else {
        false
    }
}

/// Move item down in a vector
pub fn move_down<T>(vec: &mut [T], index: usize) -> bool {
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
pub fn autocomplete_edit(ui: &mut Ui, id_salt: &str, value: &mut String, candidates: &[String]) -> Response {
    let resp = ui.text_edit_singleline(value);
    let popup_id = ui.make_persistent_id(id_salt);
    if resp.gained_focus() {
        ui.memory_mut(|m| m.open_popup(popup_id));
    }

    // Nothing to filter while the popup is closed (which is almost always).
    if !ui.memory(|m| m.is_popup_open(popup_id)) {
        return resp;
    }

    let needle = value.trim().to_lowercase();
    let matches: Vec<&String> = candidates
        .iter()
        .filter(|c| !c.is_empty() && c.as_str() != value)
        .filter(|c| needle.is_empty() || c.to_lowercase().contains(&needle))
        .take(10)
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
                    if ui.selectable_label(false, m.as_str()).clicked() {
                        *value = (*m).clone();
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
pub fn status_bar(ui: &mut Ui, status: &str, modified: bool, modified_text: &str, summary: &str) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(status).truncate());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if modified {
                // Warning colour from the current visuals: readable on both
                // the dark and the light theme (a hard-coded yellow was not).
                let c = ui.visuals().warn_fg_color;
                ui.label(RichText::new("●").color(c));
                ui.label(modified_text);
                ui.separator();
            }
            if !summary.is_empty() {
                ui.label(RichText::new(summary).weak());
            }
        });
    });
}

/// Small icon button used for every "clear / delete" action (filter clearing,
/// recent-file removal, …). The PNG is embedded in the binary, so it renders
/// identically on every platform and never depends on font glyph coverage.
/// Returns the `Response` so callers can add a tooltip and test `.clicked()`.
pub fn delete_button(ui: &mut Ui) -> Response {
    let color = crate::ui::theme::Palette::from_ui(ui).danger;
    ui.add(egui::Button::new(RichText::new(crate::ui::theme::icon::CLOSE).color(color)).frame(false))
}

/// A small icon-only button (icon font glyph), used for reorder arrows and
/// the like. Returns the widget so callers can wrap it in `add_enabled`.
pub fn icon_button(glyph: &str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(glyph.to_string()).size(15.0)).min_size(egui::vec2(24.0, 22.0))
}

/// Reorder ("move") buttons, drawn with the icon font (always available, see
/// `theme::add_icons`). Each helper returns a widget so callers can wrap it in
/// `ui.add_enabled(..)`.
pub fn arrow_up_button() -> egui::Button<'static> {
    icon_button(crate::ui::theme::icon::UP)
}

pub fn arrow_down_button() -> egui::Button<'static> {
    icon_button(crate::ui::theme::icon::DOWN)
}

pub fn arrow_left_button() -> egui::Button<'static> {
    icon_button(crate::ui::theme::icon::LEFT)
}

pub fn arrow_right_button() -> egui::Button<'static> {
    icon_button(crate::ui::theme::icon::RIGHT)
}

/// Keyboard navigation of a row cursor in a table: ↑ / ↓ move by one row,
/// Page Up / Page Down by `page` rows, Home / End to the ends. Only acts when
/// no text field has the keyboard (`editing` false). `pos` is the cursor's
/// position among the `n` visible rows. Returns the new position when the
/// cursor moved (the caller selects that row and scrolls to it).
pub fn table_nav_keys(ctx: &egui::Context, pos: Option<usize>, n: usize, page: usize, editing: bool) -> Option<usize> {
    if n == 0 || editing {
        return None;
    }
    let page = page.max(1);
    let last = n - 1;
    ctx.input_mut(|i| {
        use egui::{Key, Modifiers};
        let none = Modifiers::NONE;
        let cur = pos.unwrap_or(0).min(last);
        let mut next: Option<usize> = None;
        if i.consume_key(none, Key::ArrowDown) {
            next = Some(if pos.is_none() { 0 } else { (cur + 1).min(last) });
        }
        if i.consume_key(none, Key::ArrowUp) {
            next = Some(if pos.is_none() { 0 } else { cur.saturating_sub(1) });
        }
        if i.consume_key(none, Key::PageDown) {
            next = Some((cur + page).min(last));
        }
        if i.consume_key(none, Key::PageUp) {
            next = Some(cur.saturating_sub(page));
        }
        if i.consume_key(none, Key::Home) {
            next = Some(0);
        }
        if i.consume_key(none, Key::End) {
            next = Some(last);
        }
        next.filter(|&p| pos != Some(p))
    })
}

#[cfg(test)]
mod tests {
    use super::{file_uri, forget_image_uri, image_probe, move_down, move_up};

    #[test]
    fn image_probe_caches_until_forgotten() {
        let ctx = eframe::egui::Context::default();
        let path = std::env::temp_dir().join(format!("ximod_image_probe_test_{}.png", std::process::id()));
        let _ = std::fs::remove_file(&path);

        // Missing file: no URI, and the negative answer is cached.
        assert!(image_probe(&ctx, &path).is_none());
        std::fs::write(&path, b"x").expect("write temp file");
        assert!(image_probe(&ctx, &path).is_none());

        // Forgetting the path makes the next probe hit the disk again.
        forget_image_uri(&ctx, &path);
        let uri = image_probe(&ctx, &path);
        assert_eq!(uri.as_deref(), Some(file_uri(&path).as_str()));

        let _ = std::fs::remove_file(&path);
    }

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

    #[test]
    fn table_nav_keys_move_the_cursor() {
        use super::table_nav_keys;
        use eframe::egui::{self, Event, Key, Modifiers};
        let ctx = egui::Context::default();
        let press = |key: Key| egui::RawInput {
            events: vec![Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::NONE,
            }],
            ..Default::default()
        };
        let mut moved = None;
        let _ = ctx.run(press(Key::ArrowDown), |ctx| {
            moved = table_nav_keys(ctx, Some(3), 10, 4, false)
        });
        assert_eq!(moved, Some(4));
        let _ = ctx.run(press(Key::PageDown), |ctx| {
            moved = table_nav_keys(ctx, Some(3), 10, 4, false)
        });
        assert_eq!(moved, Some(7));
        let _ = ctx.run(press(Key::PageUp), |ctx| {
            moved = table_nav_keys(ctx, Some(3), 10, 4, false)
        });
        assert_eq!(moved, Some(0));
        let _ = ctx.run(press(Key::End), |ctx| moved = table_nav_keys(ctx, None, 10, 4, false));
        assert_eq!(moved, Some(9));
        let _ = ctx.run(press(Key::ArrowUp), |ctx| {
            moved = table_nav_keys(ctx, Some(0), 10, 4, false)
        });
        assert_eq!(moved, None, "already on the first row");
        let _ = ctx.run(press(Key::ArrowDown), |ctx| {
            moved = table_nav_keys(ctx, Some(3), 10, 4, true)
        });
        assert_eq!(moved, None, "no navigation while typing");
    }
}
