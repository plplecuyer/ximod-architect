//! Read-only explorer of the country / language reference database.
//!
//! Opens an independent window (movable to a second screen) that browses the
//! three reference files bidirectionally:
//!   * a "Countries" tab: pick a country → flag, English/French names, official
//!     languages (endonym, font, ISO 639-3/639-1) and every spoken language;
//!   * a "Languages" tab: pick a language → its endonym, ISO codes, font and the
//!     countries where it is spoken (reverse lookup).
//!
//! It never modifies anything — the reference data is edited elsewhere (the
//! translation editor and the `tools/` pipeline).

use eframe::egui::{self, RichText};

use crate::ui::components::ImageDisplay;
use crate::ui::main_window::XimodApp;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum PropTab {
    #[default]
    Countries,
    Languages,
}

#[derive(Default)]
pub struct PropertiesState {
    pub tab: PropTab,
    pub country_filter: String,
    pub country_sel: Option<String>, // ISO 3166-1 alpha-3
    pub country_cursor: usize,       // keyboard-highlighted row in the country list
    pub country_visible: usize,      // fully visible rows (measured last frame)
    pub lang_filter: String,
    pub lang_sel: Option<String>, // ISO 639-3
    pub lang_cursor: usize,       // keyboard-highlighted row in the language list
    pub lang_visible: usize,      // fully visible rows (measured last frame)
    /// Per-window caches (see [`PropCaches`]); dropped when the window closes.
    caches: PropCaches,
}

/// Identity of a source `Vec` (address + length). The reference tables are
/// only ever replaced wholesale (reloaded from disk), never edited in place,
/// so a different identity means "the data may have changed".
type SrcId = (usize, usize);

/// Name of a country for the "Spoken in" list, in the language `loc` the user
/// chose in Settings: its endonym in that language (Countries.json) when the
/// language is official there, else the translated name (CountryNames.json),
/// else the English name.
fn spoken_in_name<'a>(
    names: &'a crate::data::CountryNamesData,
    c: &'a crate::data::CountryEntry,
    loc: &str,
) -> &'a str {
    c.languages
        .iter()
        .find(|l| l.iso3 == loc)
        .map(|l| l.country_endonym.as_str())
        .filter(|s| !s.is_empty())
        .or_else(|| names.name_for(&c.a3, loc))
        .unwrap_or(&c.name_en)
}

fn src_id<T>(v: &[T]) -> SrcId {
    (v.as_ptr() as usize, v.len())
}

/// Everything the window used to recompute on every frame and that only
/// depends on the filter text, the UI locale or the selection.
#[derive(Default)]
struct PropCaches {
    /// Flag folder (`current_exe` + `is_dir`), resolved once per opening.
    flags_dir: Option<Option<std::path::PathBuf>>,
    /// Sorted + filtered country rows.
    countries: Option<CountryList>,
    /// Filtered language rows.
    languages: Option<LangList>,
    /// "Languages spoken" text of the selected country, keyed by its alpha-3.
    country_spoken: Option<(String, String)>,
    /// "Spoken in" text of the selected language, keyed by its ISO 639-3 code,
    /// the identity of the country table it was built from and the language
    /// the country names are written in.
    lang_countries: Option<(String, SrcId, String, String)>,
    /// Index of the selected language in the language table (a hint, always
    /// verified before use).
    lang_sel_idx: usize,
    /// Row heights measured on the previous frame (0 = not measured yet).
    country_row_h: f32,
    lang_row_h: f32,
}

struct CountryList {
    filter: String,
    locale: String,
    src: SrcId,
    /// (alpha-3, localized name), sorted by name.
    rows: Vec<(String, String)>,
}

struct LangList {
    filter: String,
    src: SrcId,
    /// Indices into the language table, in table order.
    rows: Vec<u32>,
}

/// A virtualised list of `selectable_label` rows: only the rows inside the
/// scroll viewport are built. Returns the clicked row (if any) and the number
/// of fully visible rows.
///
/// `scroll` is `Some(align)` on the frame the keyboard cursor moved: the cursor
/// row is then scrolled into view exactly like `Response::scroll_to_me(align)`
/// did, except that its rectangle is computed from the row pitch (the row may
/// be far outside the rendered range, e.g. after End / Page Down).
///
/// Rows must all have the same height, so labels are truncated rather than
/// wrapped. `row_h` carries the height measured on the previous frame (the
/// first frame uses an estimate derived from the style).
fn virtual_list(
    ui: &mut egui::Ui,
    id_salt: &str,
    n: usize,
    cursor: usize,
    scroll: Option<Option<egui::Align>>,
    row_h: &mut f32,
    label: impl Fn(usize) -> String,
) -> (Option<usize>, usize) {
    let h = if *row_h > 0.0 {
        *row_h
    } else {
        let sp = ui.spacing();
        sp.interact_size
            .y
            .max(ui.text_style_height(&egui::TextStyle::Button) + 2.0 * sp.button_padding.y)
    };
    let pitch = h + ui.spacing().item_spacing.y;
    let mut clicked = None;
    let mut measured: Option<f32> = None;
    let out = egui::ScrollArea::vertical()
        .id_salt(id_salt)
        .auto_shrink([false, false])
        .show_rows(ui, h, n, |ui, range| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
            if let Some(align) = scroll {
                // `ui` starts at the top of row `range.start`.
                let top = ui.max_rect().top() + (cursor as f32 - range.start as f32) * pitch;
                let rect = egui::Rect::from_min_size(
                    egui::pos2(ui.max_rect().left(), top),
                    egui::vec2(ui.max_rect().width(), h),
                );
                ui.scroll_to_rect(rect, align);
            }
            for idx in range {
                let resp = ui.selectable_label(idx == cursor, label(idx));
                if measured.is_none() {
                    measured = Some(resp.rect.height());
                }
                if resp.clicked() {
                    clicked = Some(idx);
                }
            }
        });
    if let Some(m) = measured
        && (m - h).abs() > 0.01
    {
        // The style changed (or the estimate was off): lay out again with the
        // real row height.
        *row_h = m;
        ui.ctx().request_repaint();
    }
    let visible = ((out.inner_rect.height() / pitch.max(1.0)).floor() as usize).max(1);
    (clicked, visible)
}

/// Move a list cursor from the keyboard. Consumes the navigation keys (so the
/// filter text field never also reacts to them) and returns:
///   * whether the cursor moved this frame,
///   * whether Enter was pressed,
///   * how the moved cursor should be scrolled into view (the argument to pass
///     to `Response::scroll_to_me`): `None` for single-step arrows (minimal
///     scroll), `Some(TOP)` for Page Up/Down and Home (cursor lands at the top
///     of the page), `Some(BOTTOM)` for End.
///
/// `page` is the number of fully visible rows: Page Down/Up jump by exactly that
/// many, so successive pages do not overlap (last row of a page → first row of
/// the next).
fn handle_list_keys(
    ctx: &egui::Context,
    cursor: &mut usize,
    n: usize,
    page: usize,
) -> (bool, bool, Option<egui::Align>) {
    if n == 0 {
        *cursor = 0;
        return (false, false, None);
    }
    if *cursor >= n {
        *cursor = n - 1;
    }
    let start = *cursor;
    let mut enter = false;
    let mut tabbed = false;
    let mut align: Option<egui::Align> = None;
    let page = page.max(1);
    ctx.input_mut(|i| {
        use egui::{Align, Key, Modifiers};
        let m = Modifiers::NONE;
        // Tab / Shift+Tab: drive the cursor like Down / Up, and consume them so
        // egui's native focus traversal does not move a focus ring onto the
        // filter, the clear button or the detail-panel flag.
        if i.consume_key(Modifiers::SHIFT, Key::Tab) {
            *cursor = cursor.saturating_sub(1);
            tabbed = true;
        }
        if i.consume_key(m, Key::Tab) {
            if *cursor + 1 < n {
                *cursor += 1;
            }
            tabbed = true;
        }
        if i.consume_key(m, Key::ArrowDown) && *cursor + 1 < n {
            *cursor += 1;
            align = None;
        }
        if i.consume_key(m, Key::ArrowUp) {
            *cursor = cursor.saturating_sub(1);
            align = None;
        }
        if i.consume_key(m, Key::PageDown) {
            *cursor = (*cursor + page).min(n - 1);
            align = Some(Align::TOP);
        }
        if i.consume_key(m, Key::PageUp) {
            *cursor = cursor.saturating_sub(page);
            align = Some(Align::TOP);
        }
        if i.consume_key(m, Key::Home) {
            *cursor = 0;
            align = Some(Align::TOP);
        }
        if i.consume_key(m, Key::End) {
            *cursor = n - 1;
            align = Some(Align::BOTTOM);
        }
        if i.consume_key(m, Key::Enter) {
            enter = true;
        }
    });
    // On Tab, drop any egui keyboard focus so no stray focus ring lingers.
    if tabbed {
        let focused = ctx.memory(|m| m.focused());
        if let Some(id) = focused {
            ctx.memory_mut(|m| m.surrender_focus(id));
        }
    }
    (*cursor != start, enter, align)
}

impl XimodApp {
    /// Open the country/language explorer.
    pub fn open_properties(&mut self) {
        self.show_properties = true;
    }

    /// Render the Properties window (independent viewport).
    pub fn render_properties(&mut self, ctx: &egui::Context) {
        if !self.show_properties {
            // Drop the caches so a reopened window starts from fresh data.
            if self.properties.caches.flags_dir.is_some() {
                self.properties.caches = PropCaches::default();
            }
            self.free_window_closed("ximod_properties");
            return;
        }

        let title = self.i18n.t("prop-title");
        let vb = self.free_viewport_builder(ctx, "ximod_properties", title, [820.0, 600.0], false);
        let l_tab_countries = self.i18n.t("prop-tab-countries");
        let l_tab_languages = self.i18n.t("prop-tab-languages");
        let l_filter = self.i18n.t("prop-filter");
        let l_official = self.i18n.t("prop-official-langs");
        let l_name = self.i18n.t("prop-col-name");
        let l_spoken = self.i18n.t("prop-spoken-langs");
        let l_endonym = self.i18n.t("prop-endonym");
        let l_font = self.i18n.t("prop-font");
        let l_spoken_in = self.i18n.t("prop-spoken-in");
        let l_pick_country = self.i18n.t("prop-select-country");
        let l_pick_lang = self.i18n.t("prop-select-lang");
        let l_none = self.i18n.t("trans-no-font");
        let l_clear = self.i18n.t("btn-clear");

        let countries = &self.countries;
        let country_languages = &self.country_languages;
        let country_names = &self.country_names;
        let i18n = &self.i18n;
        // Current UI locale (ISO 639-3): country names are shown in this
        // language when available, otherwise in English.
        let loc = i18n.current_locale();
        let st = &mut self.properties;
        // Flags live in the application's asset folder (assets/images/svg),
        // not under the mod project root. Resolved once per opening.
        let flags_dir: &Option<std::path::PathBuf> = st.caches.flags_dir.get_or_insert_with(crate::data::flags_dir);
        let cfg = &mut self.config;
        // "Spoken in" names every country in the language the user chose in
        // Settings (flag → country → language): the country's endonym in that
        // language from Countries.json when the language is official there,
        // else the translated name from CountryNames.json, else English.
        let spoken_loc = loc.to_string();
        let mut do_close = false;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("ximod_properties"), vb, |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // Tabs.
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut st.tab, PropTab::Countries, &l_tab_countries);
                    ui.selectable_value(&mut st.tab, PropTab::Languages, &l_tab_languages);
                });
                ui.separator();

                match st.tab {
                    PropTab::Countries => {
                        // Filtered country list (a3, localized name) — names
                        // shown in the current UI locale, English fallback.
                        // Rebuilt only when the filter, the locale or the
                        // country table changes.
                        let src = src_id(&countries.countries);
                        let fresh = st
                            .caches
                            .countries
                            .as_ref()
                            .is_some_and(|c| c.filter == st.country_filter && c.locale == loc && c.src == src);
                        if !fresh {
                            let needle = st.country_filter.trim().to_lowercase();
                            let mut rows: Vec<(String, String)> = countries
                                .countries
                                .iter()
                                .map(|c| {
                                    let name = country_names.name_for(&c.a3, loc).unwrap_or(&c.name_en).to_string();
                                    (c.a3.clone(), name)
                                })
                                .collect();
                            rows.sort_by_cached_key(|a| a.1.to_lowercase());
                            rows.retain(|(a3, name)| {
                                needle.is_empty()
                                    || name.to_lowercase().contains(&needle)
                                    || a3.to_lowercase().contains(&needle)
                            });
                            st.caches.countries = Some(CountryList {
                                filter: st.country_filter.clone(),
                                locale: loc.to_string(),
                                src,
                                rows,
                            });
                        }
                        let list: &[(String, String)] =
                            st.caches.countries.as_ref().map(|c| c.rows.as_slice()).unwrap_or(&[]);

                        ui.horizontal(|ui| {
                            ui.label(&l_filter);
                            ui.text_edit_singleline(&mut st.country_filter);
                            if crate::ui::components::delete_button(ui)
                                .on_hover_text(&l_clear)
                                .clicked()
                            {
                                st.country_filter.clear();
                            }
                        });
                        ui.add_space(4.0);

                        ui.columns(2, |cols| {
                            let list_ui = &mut cols[0];
                            // Keyboard navigation for the list (arrows / Page
                            // Up-Down / Home / End / Enter). The page size is
                            // the number of fully visible rows, measured on the
                            // previous frame (so it follows window resizing);
                            // a font-based estimate seeds the first frame.
                            let est = {
                                let rh = list_ui.text_style_height(&egui::TextStyle::Body)
                                    + list_ui.spacing().item_spacing.y;
                                ((list_ui.available_height() / rh).floor() as usize).max(1)
                            };
                            let page = if st.country_visible > 0 {
                                st.country_visible
                            } else {
                                est
                            };
                            let (moved, enter, align) = handle_list_keys(ctx, &mut st.country_cursor, list.len(), page);
                            if enter && let Some((a3, _)) = list.get(st.country_cursor) {
                                st.country_sel = Some(a3.clone());
                            }
                            let (clicked, visible) = virtual_list(
                                list_ui,
                                "prop_country_list",
                                list.len(),
                                st.country_cursor,
                                moved.then_some(align),
                                &mut st.caches.country_row_h,
                                |idx| {
                                    let (a3, name) = &list[idx];
                                    format!("{name}  ({a3})")
                                },
                            );
                            if let Some(idx) = clicked
                                && let Some((a3, _)) = list.get(idx)
                            {
                                st.country_cursor = idx;
                                st.country_sel = Some(a3.clone());
                            }
                            // Remember how many rows fit, for the next frame's
                            // Page Up/Down (recomputed → adapts to resizing).
                            st.country_visible = visible;

                            let ui = &mut cols[1];
                            match st.country_sel.as_deref().and_then(|a3| countries.by_a3(a3)) {
                                Some(c) => {
                                    egui::ScrollArea::vertical()
                                        .id_salt("prop_country_detail")
                                        .auto_shrink([false, false])
                                        .show(ui, |ui| {
                                            let abs = countries
                                                .flag_for(&c.a3)
                                                .and_then(|f| flags_dir.as_ref().map(|d| d.join(f)));
                                            ImageDisplay::new(160.0, 100.0)
                                                .with_fallback(" ")
                                                .show(ui, abs.as_deref());
                                            ui.add_space(4.0);
                                            let disp_name = country_names.name_for(&c.a3, loc).unwrap_or(&c.name_en);
                                            ui.label(RichText::new(disp_name).strong().size(15.0));
                                            ui.label(
                                                RichText::new(format!("{} — {}", c.name_en, c.a3))
                                                    .color(ui.visuals().weak_text_color()),
                                            );

                                            ui.add_space(8.0);
                                            ui.label(RichText::new(&l_official).strong());
                                            egui::Grid::new("prop_official").num_columns(4).striped(true).show(
                                                ui,
                                                |ui| {
                                                    ui.label(RichText::new("ISO").strong());
                                                    ui.label(RichText::new(&l_name).strong());
                                                    ui.label(RichText::new(&l_endonym).strong());
                                                    ui.label(RichText::new(&l_font).strong());
                                                    ui.end_row();
                                                    for cl in &c.languages {
                                                        let name = i18n.display_name(&cl.iso3);
                                                        let iso1 =
                                                            i18n.languages().iso3_to_iso1(&cl.iso3).unwrap_or("");
                                                        let code = if iso1.is_empty() {
                                                            cl.iso3.clone()
                                                        } else {
                                                            format!("{} / {}", cl.iso3, iso1)
                                                        };
                                                        let font = i18n.font_for(&cl.iso3).unwrap_or(&l_none);
                                                        ui.label(code);
                                                        ui.label(name);
                                                        ui.label(&cl.country_endonym);
                                                        ui.label(RichText::new(font).monospace().small());
                                                        ui.end_row();
                                                    }
                                                },
                                            );

                                            ui.add_space(8.0);
                                            let spoken = country_languages.languages_for(&c.a3);
                                            ui.label(
                                                RichText::new(format!("{} ({})", l_spoken, spoken.len())).strong(),
                                            );
                                            // Built once per selected country
                                            // (it can list hundreds of languages).
                                            let cached = &mut st.caches.country_spoken;
                                            if cached.as_ref().is_none_or(|(k, _)| *k != c.a3) {
                                                let joined: Vec<String> = spoken
                                                    .iter()
                                                    .map(|iso3| format!("{} ({})", i18n.display_name(iso3), iso3))
                                                    .collect();
                                                *cached = Some((c.a3.clone(), joined.join(", ")));
                                            }
                                            if let Some((_, text)) = cached.as_ref() {
                                                ui.label(text.as_str());
                                            }
                                        });
                                }
                                None => {
                                    ui.label(RichText::new(&l_pick_country).color(ui.visuals().weak_text_color()));
                                }
                            }
                        });
                    }
                    PropTab::Languages => {
                        // Filtered rows, as indices into the language table.
                        // Rebuilt only when the filter (or the table) changes.
                        let entries = &i18n.languages().languages;
                        let src = src_id(entries);
                        let fresh = st
                            .caches
                            .languages
                            .as_ref()
                            .is_some_and(|c| c.filter == st.lang_filter && c.src == src);
                        if !fresh {
                            let needle = st.lang_filter.trim().to_lowercase();
                            let rows: Vec<u32> = entries
                                .iter()
                                .enumerate()
                                .filter(|(_, e)| {
                                    needle.is_empty()
                                        || e.name.to_lowercase().contains(&needle)
                                        || e.iso3.to_lowercase().contains(&needle)
                                })
                                .map(|(i, _)| i as u32)
                                .collect();
                            st.caches.languages = Some(LangList {
                                filter: st.lang_filter.clone(),
                                src,
                                rows,
                            });
                        }
                        let list: &[u32] = st.caches.languages.as_ref().map(|c| c.rows.as_slice()).unwrap_or(&[]);

                        ui.horizontal(|ui| {
                            ui.label(&l_filter);
                            ui.text_edit_singleline(&mut st.lang_filter);
                            if crate::ui::components::delete_button(ui)
                                .on_hover_text(&l_clear)
                                .clicked()
                            {
                                st.lang_filter.clear();
                            }
                        });
                        ui.add_space(4.0);

                        ui.columns(2, |cols| {
                            let list_ui = &mut cols[0];
                            let est = {
                                let rh = list_ui.text_style_height(&egui::TextStyle::Body)
                                    + list_ui.spacing().item_spacing.y;
                                ((list_ui.available_height() / rh).floor() as usize).max(1)
                            };
                            let page = if st.lang_visible > 0 { st.lang_visible } else { est };
                            let (moved, enter, align) = handle_list_keys(ctx, &mut st.lang_cursor, list.len(), page);
                            let (clicked, visible) = virtual_list(
                                list_ui,
                                "prop_lang_list",
                                list.len(),
                                st.lang_cursor,
                                moved.then_some(align),
                                &mut st.caches.lang_row_h,
                                |idx| match entries.get(list[idx] as usize) {
                                    Some(e) => format!("{}  ({})", e.name, e.iso3),
                                    None => String::new(),
                                },
                            );
                            if let Some(idx) = clicked {
                                st.lang_cursor = idx;
                            }
                            if (enter || clicked.is_some())
                                && let Some(&ei) = list.get(st.lang_cursor)
                                && let Some(e) = entries.get(ei as usize)
                            {
                                st.lang_sel = Some(e.iso3.clone());
                                st.caches.lang_sel_idx = ei as usize;
                            }
                            st.lang_visible = visible;

                            let ui = &mut cols[1];
                            // Selected entry: try the remembered index first,
                            // fall back to a scan of the table.
                            let entry = st.lang_sel.as_deref().and_then(|iso3| {
                                entries
                                    .get(st.caches.lang_sel_idx)
                                    .filter(|e| e.iso3 == iso3)
                                    .or_else(|| entries.iter().find(|e| e.iso3 == iso3))
                            });
                            match entry {
                                Some(e) => {
                                    egui::ScrollArea::vertical()
                                        .id_salt("prop_lang_detail")
                                        .auto_shrink([false, false])
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(&e.name).strong().size(15.0));
                                            let code = if e.iso1.is_empty() {
                                                format!("ISO 639-3 : {}", e.iso3)
                                            } else {
                                                format!("ISO 639-3 : {} · ISO 639-1 : {}", e.iso3, e.iso1)
                                            };
                                            ui.label(RichText::new(code).color(ui.visuals().weak_text_color()));
                                            ui.add_space(4.0);
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(&l_font).strong());
                                                let font = if e.font.is_empty() {
                                                    l_none.clone()
                                                } else {
                                                    e.font.clone()
                                                };
                                                ui.label(RichText::new(font).monospace().small());
                                            });

                                            ui.add_space(8.0);
                                            ui.label(
                                                RichText::new(format!("{} ({})", l_spoken_in, e.countries.len()))
                                                    .strong(),
                                            );
                                            // Built once per selected language.
                                            let csrc = src_id(&countries.countries);
                                            let cached = &mut st.caches.lang_countries;
                                            if cached.as_ref().is_none_or(|(k, s, l, _)| {
                                                *k != e.iso3 || *s != csrc || *l != spoken_loc
                                            }) {
                                                let names: Vec<String> = e
                                                    .countries
                                                    .iter()
                                                    .map(|a3| match countries.by_a3(a3) {
                                                        Some(c) => {
                                                            let n = spoken_in_name(country_names, c, &spoken_loc);
                                                            format!("{n} ({a3})")
                                                        }
                                                        None => a3.clone(),
                                                    })
                                                    .collect();
                                                *cached =
                                                    Some((e.iso3.clone(), csrc, spoken_loc.clone(), names.join(", ")));
                                            }
                                            if let Some((_, _, _, text)) = cached.as_ref() {
                                                ui.label(text.as_str());
                                            }
                                        });
                                }
                                None => {
                                    ui.label(RichText::new(&l_pick_lang).color(ui.visuals().weak_text_color()));
                                }
                            }
                        });
                    }
                }
            });

            crate::ui::widgets::free_window::record_win_geom(cfg, ctx, "ximod_properties");
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        if do_close {
            self.show_properties = false;
            self.free_window_closed("ximod_properties");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{spoken_in_name, virtual_list};
    use eframe::egui;
    use std::cell::RefCell;

    /// Run one frame showing a 5 000-row list; returns the rows that were built.
    fn frame(ctx: &egui::Context, cursor: usize, scroll: Option<Option<egui::Align>>, row_h: &mut f32) -> Vec<usize> {
        let built = RefCell::new(Vec::new());
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 300.0))),
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                virtual_list(ui, "test_list", 5000, cursor, scroll, row_h, |idx| {
                    built.borrow_mut().push(idx);
                    format!("row {idx}")
                });
            });
        });
        built.into_inner()
    }

    /// Request a scroll to `cursor`, then let egui apply it (the new offset is
    /// used for layout two frames after the request) and return the rows built.
    fn scroll_to(ctx: &egui::Context, cursor: usize, align: Option<egui::Align>, row_h: &mut f32) -> Vec<usize> {
        let _ = frame(ctx, cursor, Some(align), row_h);
        let _ = frame(ctx, cursor, None, row_h);
        frame(ctx, cursor, None, row_h)
    }

    /// "Spoken in" names a country in the language chosen in Settings: the
    /// endonym from Countries.json when that language is official there,
    /// else the translated name from CountryNames.json, else English.
    #[test]
    fn spoken_in_uses_the_users_language() {
        let countries = crate::data::CountriesData::load();
        let names = crate::data::CountryNamesData::load();
        let che = countries.by_a3("CHE").expect("Switzerland in Countries.json");
        let zaf = countries.by_a3("ZAF").expect("South Africa in Countries.json");
        assert_eq!(spoken_in_name(&names, che, "ita"), "Confederazione Svizzera");
        assert_eq!(spoken_in_name(&names, che, "fra"), "Confédération suisse");
        assert_eq!(spoken_in_name(&names, zaf, "eng"), "Republic of South Africa");
        // German is not official in South Africa: translated name.
        assert_eq!(spoken_in_name(&names, zaf, "deu"), "Südafrika");
        assert_eq!(spoken_in_name(&names, zaf, "fra"), "Afrique du Sud");
        // Unknown language: English name.
        assert_eq!(spoken_in_name(&names, zaf, "xxx"), zaf.name_en);
    }

    #[test]
    fn virtual_list_builds_only_visible_rows_and_follows_the_cursor() {
        let ctx = egui::Context::default();
        ctx.style_mut(|s| s.scroll_animation = egui::style::ScrollAnimation::none());
        let mut row_h = 0.0;

        // Initially: the top of the list, and far fewer rows than the total.
        let _ = frame(&ctx, 0, None, &mut row_h);
        let rows = frame(&ctx, 0, None, &mut row_h);
        assert_eq!(rows.first(), Some(&0));
        assert!(rows.len() < 40, "built {} rows", rows.len());

        // End: the cursor row is far outside the built range, yet it must be
        // scrolled into view.
        let rows = scroll_to(&ctx, 4999, Some(egui::Align::BOTTOM), &mut row_h);
        assert_eq!(rows.last(), Some(&4999));

        // Page-style jump: the cursor lands at the top of the page.
        let rows = scroll_to(&ctx, 2000, Some(egui::Align::TOP), &mut row_h);
        // (egui aligns with one item spacing of margin, so the row just above
        // may still be built although only its spacing is in view.)
        assert!(matches!(rows.first(), Some(&(1999 | 2000))), "{rows:?}");
        assert!(rows.contains(&2000));

        // Single step below the last visible row: minimal scroll, the cursor
        // row becomes visible.
        let next = rows.last().copied().unwrap_or(2000) + 1;
        let rows = scroll_to(&ctx, next, None, &mut row_h);
        assert!(rows.contains(&next));
        // … at the bottom of the page, not at its top.
        assert!(
            rows.first().is_some_and(|&f| f + 5 < next),
            "minimal scroll moved too far: {rows:?}"
        );
    }
}
