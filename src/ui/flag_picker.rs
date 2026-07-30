//! Flag picker: a grid of every country flag, used to choose the user's
//! country from the settings window.
//!
//! Selecting a flag sets `temp_country`, which in turn scopes (and enables) the
//! language drop-down and the country-name field.
//!
//! Only the visible rows are built (`ScrollArea::show_rows`), so the ~250 SVG
//! flags are rasterised lazily instead of all at once.

use crate::ui::main_window::XimodApp;
use eframe::egui;

/// Which window asked for the flag picker, i.e. where the choice is stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlagTarget {
    /// Settings window: sets the user's own country.
    #[default]
    Settings,
    /// Translation editor: sets the country whose languages can be translated.
    Translation,
}

/// Flags per row in the grid.
const COLUMNS: usize = 6;
/// Drawn size of one flag.
const CELL_W: f32 = 104.0;
const CELL_H: f32 = 70.0;
/// Row height = flag + caption + spacing.
const ROW_H: f32 = CELL_H + 26.0;

/// Keyboard navigation for the 2-D flag grid. Consumes the keys. Returns:
///   * whether the cursor moved,
///   * the vertical scroll offset to force this frame to keep the cursor visible
///     (None = leave the user's scroll alone),
///   * whether Enter was pressed (select the cursor),
///   * whether Escape was pressed (close the window).
///
/// Grid navigation (arrows / Page / Home / End / Enter) is active only when the
/// filter field is not being edited; Escape always closes.
#[allow(clippy::too_many_arguments)]
fn handle_flag_keys(
    ctx: &egui::Context,
    cursor: &mut usize,
    n: usize,
    cols: usize,
    page_rows: usize,
    editing: bool,
    cur_off: f32,
    vh: f32,
    pitch: f32,
) -> (bool, Option<f32>, bool, bool) {
    let mut enter = false;
    let mut close = false;
    if n == 0 {
        *cursor = 0;
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                close = true;
            }
        });
        return (false, None, false, close);
    }
    if *cursor >= n {
        *cursor = n - 1;
    }
    let start = *cursor;
    let cols = cols.max(1);
    let page = page_rows.max(1) * cols;
    #[derive(PartialEq)]
    enum S {
        No,
        Min,
        Top,
        Bottom,
    }
    let mut intent = S::No;
    let mut tabbed = false;
    ctx.input_mut(|i| {
        use egui::{Key, Modifiers};
        let m = Modifiers::NONE;
        if i.consume_key(m, Key::Escape) {
            close = true;
        }
        // Tab / Shift+Tab: intercept them so egui's native focus traversal does
        // not fight our cursor (it would move the blue focus ring to an unrendered
        // button in the virtualized grid). Treated like Right / Left.
        if i.consume_key(Modifiers::SHIFT, Key::Tab) {
            *cursor = cursor.saturating_sub(1);
            intent = S::Min;
            tabbed = true;
        }
        if i.consume_key(m, Key::Tab) {
            if *cursor + 1 < n {
                *cursor += 1;
            }
            intent = S::Min;
            tabbed = true;
        }
        if !editing {
            if i.consume_key(m, Key::ArrowRight) {
                if *cursor + 1 < n {
                    *cursor += 1;
                }
                intent = S::Min;
            }
            if i.consume_key(m, Key::ArrowLeft) {
                *cursor = cursor.saturating_sub(1);
                intent = S::Min;
            }
            if i.consume_key(m, Key::ArrowDown) {
                if *cursor + cols < n {
                    *cursor += cols;
                }
                intent = S::Min;
            }
            if i.consume_key(m, Key::ArrowUp) {
                if *cursor >= cols {
                    *cursor -= cols;
                }
                intent = S::Min;
            }
            if i.consume_key(m, Key::PageDown) {
                *cursor = (*cursor + page).min(n - 1);
                intent = S::Top;
            }
            if i.consume_key(m, Key::PageUp) {
                *cursor = cursor.saturating_sub(page);
                intent = S::Top;
            }
            if i.consume_key(m, Key::Home) {
                *cursor = 0;
                intent = S::Top;
            }
            if i.consume_key(m, Key::End) {
                *cursor = n - 1;
                intent = S::Bottom;
            }
            if i.consume_key(m, Key::Enter) {
                enter = true;
            }
        }
    });
    // When navigating with Tab, drop any egui keyboard focus (filter field or a
    // button) so only our cursor border shows and the arrow keys work next.
    if tabbed {
        let focused = ctx.memory(|m| m.focused());
        if let Some(id) = focused {
            ctx.memory_mut(|m| m.surrender_focus(id));
        }
    }
    let moved = *cursor != start;
    let off = if !moved || intent == S::No {
        None
    } else {
        let row = (*cursor / cols) as f32;
        // Rows are laid out with a pitch of ROW_H + inter-row spacing, so the
        // scroll offset must use that pitch (using ROW_H alone under-scrolls by
        // one spacing per row — enough to hide the last row at the bottom).
        let row_top = row * pitch;
        let row_bot = row_top + pitch;
        match intent {
            S::Top => Some(row_top),
            S::Bottom => Some((row_bot - vh).max(0.0)),
            S::Min => {
                if row_top < cur_off {
                    Some(row_top)
                } else if row_bot > cur_off + vh {
                    Some(row_bot - vh)
                } else {
                    None
                }
            }
            S::No => None,
        }
    };
    (moved, off, enter, close)
}

impl XimodApp {
    /// Render the flag picker window.
    pub fn render_flag_picker(&mut self, ctx: &egui::Context) {
        if !self.show_flag_picker {
            self.free_window_closed("ximod_flag_picker");
            return;
        }

        let title = self.i18n.t("flags-title");
        let lbl_filter = self.i18n.t("flags-filter");
        let lbl_none = self.i18n.t("flags-none");

        // Countries that actually have a flag file, filtered by the search box.
        let dir = crate::data::flags_dir();
        let needle = self.flag_filter.trim().to_lowercase();
        let entries: Vec<(String, String, std::path::PathBuf)> = match &dir {
            Some(d) => self
                .countries
                .countries
                .iter()
                .filter(|c| !c.flag.is_empty())
                .filter(|c| {
                    needle.is_empty()
                        || c.name_fr.to_lowercase().contains(&needle)
                        || c.name_en.to_lowercase().contains(&needle)
                        || c.a3.to_lowercase().contains(&needle)
                })
                .map(|c| {
                    let label = if c.name_fr.is_empty() {
                        c.name_en.clone()
                    } else {
                        c.name_fr.clone()
                    };
                    (c.a3.clone(), label, d.join(&c.flag))
                })
                .filter(|(_, _, p)| p.is_file())
                .collect(),
            None => Vec::new(),
        };

        let mut do_close = false;
        let mut chosen: Option<String> = None;

        // Independent OS-level window (viewport): freely movable, including onto
        // a second screen and over the (also free) translation editor window.
        let vb = self.free_viewport_builder(ctx, "ximod_flag_picker", title, [720.0, 520.0]);
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("ximod_flag_picker"),
            vb,
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&lbl_filter);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.flag_filter)
                            .desired_width(220.0),
                    );
                    if crate::ui::components::delete_button(ui).clicked() {
                        self.flag_filter.clear();
                    }
                });
                ui.separator();

                // Keyboard navigation (2-D grid). Active before the empty check so
                // Escape still closes when there are no results.
                let n = entries.len();
                let editing = ctx.wants_keyboard_input();
                // Row pitch as used by ScrollArea::show_rows (row height + spacing).
                let pitch = ROW_H + ui.spacing().item_spacing.y;
                let page_rows = if self.flag_viewport_h > 0.0 {
                    (self.flag_viewport_h / pitch).floor() as usize
                } else {
                    (ui.available_height() / pitch).floor() as usize
                };
                let (_moved, target_off, key_enter, key_close) = handle_flag_keys(
                    ctx,
                    &mut self.flag_cursor,
                    n,
                    COLUMNS,
                    page_rows.max(1),
                    editing,
                    self.flag_scroll_offset,
                    self.flag_viewport_h,
                    pitch,
                );
                if key_close {
                    do_close = true;
                }
                if key_enter {
                    if let Some((a3, _, _)) = entries.get(self.flag_cursor) {
                        chosen = Some(a3.clone());
                    }
                }

                if entries.is_empty() {
                    ui.label(&lbl_none);
                    return;
                }

                let cursor_col = ui.visuals().selection.bg_fill;
                let rows = (entries.len() + COLUMNS - 1) / COLUMNS;
                let mut area = egui::ScrollArea::both().auto_shrink([false, false]);
                if let Some(off) = target_off {
                    area = area.vertical_scroll_offset(off);
                }
                let out = area.show_rows(ui, ROW_H, rows, |ui, range| {
                    for row in range {
                        ui.horizontal(|ui| {
                            for col in 0..COLUMNS {
                                let idx = row * COLUMNS + col;
                                let Some((a3, label, path)) = entries.get(idx) else {
                                    break;
                                };
                                ui.vertical(|ui| {
                                    let current = match self.flag_target {
                                        FlagTarget::Settings => &self.temp_country,
                                        FlagTarget::Translation => &self.trans_country,
                                    };
                                    let selected = current == a3;
                                    let is_cursor = idx == self.flag_cursor;
                                    let img = egui::Image::from_uri(format!(
                                        "file://{}",
                                        path.display()
                                    ))
                                    .fit_to_exact_size(egui::vec2(CELL_W, CELL_H));
                                    let resp = ui
                                        .add_sized(
                                            egui::vec2(CELL_W, CELL_H),
                                            egui::ImageButton::new(img).selected(selected),
                                        )
                                        .on_hover_text(label);
                                    if resp.clicked() {
                                        self.flag_cursor = idx;
                                        chosen = Some(a3.clone());
                                    }
                                    // Keyboard cursor: a distinct border.
                                    if is_cursor {
                                        ui.painter().rect_stroke(
                                            resp.rect.expand(1.0),
                                            egui::Rounding::same(4.0),
                                            egui::Stroke::new(2.5_f32, cursor_col),
                                        );
                                    }
                                    // Caption, truncated to the cell width.
                                    ui.allocate_ui(egui::vec2(CELL_W, 18.0), |ui| {
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(label).small(),
                                            )
                                            .wrap_mode(egui::TextWrapMode::Truncate),
                                        );
                                    });
                                });
                            }
                        });
                    }
                });
                self.flag_scroll_offset = out.state.offset.y;
                self.flag_viewport_h = out.inner_rect.height();
                });

                crate::ui::main_window::record_win_geom(&mut self.config, ctx, "ximod_flag_picker");
                if ctx.input(|i| i.viewport().close_requested()) {
                    do_close = true;
                }
            },
        );

        if let Some(a3) = chosen {
            match self.flag_target {
                FlagTarget::Settings => {
                    self.temp_country = a3;
                    // If the current language is not spoken in the new country,
                    // fall back to its first one so the drop-down never shows a
                    // stale value.
                    let langs = self.country_languages.languages_for(&self.temp_country);
                    if !langs.iter().any(|l| *l == self.temp_locale) {
                        if let Some(first) = langs.first() {
                            self.temp_locale = first.clone();
                        }
                    }
                }
                FlagTarget::Translation => {
                    self.trans_country = a3;
                    let langs = self.country_languages.languages_for(&self.trans_country);
                    if !langs.iter().any(|l| *l == self.trans_target_lang) {
                        if let Some(first) = langs.first() {
                            self.trans_target_lang = first.clone();
                            self.load_translation_entries();
                        }
                    }
                    self.refresh_translation_meta();
                }
            }
            self.show_flag_picker = false;
            self.free_window_closed("ximod_flag_picker");
        }
        if do_close {
            self.show_flag_picker = false;
            self.free_window_closed("ximod_flag_picker");
        }
    }
}
