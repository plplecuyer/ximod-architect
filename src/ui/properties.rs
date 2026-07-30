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

use eframe::egui::{self, Color32, RichText};

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
            self.free_window_closed("ximod_properties");
            return;
        }

        let title = self.i18n.t("prop-title");
        let vb = self.free_viewport_builder(ctx, "ximod_properties", title, [820.0, 600.0]);
        let l_tab_countries = self.i18n.t("prop-tab-countries");
        let l_tab_languages = self.i18n.t("prop-tab-languages");
        let l_filter = self.i18n.t("prop-filter");
        let l_official = self.i18n.t("prop-official-langs");
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
        let i18n = &self.i18n;
        // Flags live in the application's asset folder (assets/images/svg),
        // not under the mod project root.
        let flags_dir = crate::data::flags_dir();
        let st = &mut self.properties;
        let cfg = &mut self.config;
        let mut do_close = false;

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("ximod_properties"),
            vb,
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // Tabs.
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut st.tab, PropTab::Countries, &l_tab_countries);
                        ui.selectable_value(&mut st.tab, PropTab::Languages, &l_tab_languages);
                    });
                    ui.separator();

                    match st.tab {
                        PropTab::Countries => {
                            // Filtered country list (a3, French name).
                            let needle = st.country_filter.trim().to_lowercase();
                            let list: Vec<(String, String)> = countries
                                .country_list()
                                .into_iter()
                                .filter(|(a3, name)| {
                                    needle.is_empty()
                                        || name.to_lowercase().contains(&needle)
                                        || a3.to_lowercase().contains(&needle)
                                })
                                .collect();

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
                                let (moved, enter, align) =
                                    handle_list_keys(ctx, &mut st.country_cursor, list.len(), page);
                                if enter {
                                    if let Some((a3, _)) = list.get(st.country_cursor) {
                                        st.country_sel = Some(a3.clone());
                                    }
                                }
                                let out = egui::ScrollArea::vertical()
                                    .id_salt("prop_country_list")
                                    .auto_shrink([false, false])
                                    .show(list_ui, |ui| {
                                        let spacing = ui.spacing().item_spacing.y;
                                        let mut pitch = 0.0f32;
                                        for (idx, (a3, name)) in list.iter().enumerate() {
                                            let is_cursor = idx == st.country_cursor;
                                            let resp = ui.selectable_label(
                                                is_cursor,
                                                format!("{name}  ({a3})"),
                                            );
                                            if idx == 0 {
                                                pitch = resp.rect.height() + spacing;
                                            }
                                            if resp.clicked() {
                                                st.country_cursor = idx;
                                                st.country_sel = Some(a3.clone());
                                            }
                                            if moved && is_cursor {
                                                resp.scroll_to_me(align);
                                            }
                                        }
                                        pitch
                                    });
                                // Remember how many rows fit, for the next frame's
                                // Page Up/Down (recomputed → adapts to resizing).
                                let pitch = out.inner.max(1.0);
                                st.country_visible =
                                    ((out.inner_rect.height() / pitch).floor() as usize).max(1);

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
                                                ui.label(
                                                    RichText::new(&c.name_fr).strong().size(15.0),
                                                );
                                                ui.label(
                                                    RichText::new(format!("{} — {}", c.name_en, c.a3))
                                                        .color(Color32::GRAY),
                                                );

                                                ui.add_space(8.0);
                                                ui.label(RichText::new(&l_official).strong());
                                                egui::Grid::new("prop_official")
                                                    .num_columns(4)
                                                    .striped(true)
                                                    .show(ui, |ui| {
                                                        ui.label(RichText::new("ISO").strong());
                                                        ui.label(RichText::new("Nom").strong());
                                                        ui.label(RichText::new(&l_endonym).strong());
                                                        ui.label(RichText::new(&l_font).strong());
                                                        ui.end_row();
                                                        for cl in &c.languages {
                                                            let name = i18n.display_name(&cl.iso3);
                                                            let iso1 = i18n
                                                                .languages()
                                                                .iso3_to_iso1(&cl.iso3)
                                                                .unwrap_or("");
                                                            let code = if iso1.is_empty() {
                                                                cl.iso3.clone()
                                                            } else {
                                                                format!("{} / {}", cl.iso3, iso1)
                                                            };
                                                            let font = i18n
                                                                .font_for(&cl.iso3)
                                                                .unwrap_or(&l_none);
                                                            ui.label(code);
                                                            ui.label(name);
                                                            ui.label(&cl.country_endonym);
                                                            ui.label(
                                                                RichText::new(font).monospace().small(),
                                                            );
                                                            ui.end_row();
                                                        }
                                                    });

                                                ui.add_space(8.0);
                                                let spoken = country_languages.languages_for(&c.a3);
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{} ({})",
                                                        l_spoken,
                                                        spoken.len()
                                                    ))
                                                    .strong(),
                                                );
                                                let joined: Vec<String> = spoken
                                                    .iter()
                                                    .map(|iso3| {
                                                        format!(
                                                            "{} ({})",
                                                            i18n.display_name(iso3),
                                                            iso3
                                                        )
                                                    })
                                                    .collect();
                                                ui.label(joined.join(", "));
                                            });
                                    }
                                    None => {
                                        ui.label(RichText::new(&l_pick_country).color(Color32::GRAY));
                                    }
                                }
                            });
                        }
                        PropTab::Languages => {
                            let needle = st.lang_filter.trim().to_lowercase();
                            let entries = &i18n.languages().languages;
                            let list: Vec<(String, String)> = entries
                                .iter()
                                .map(|e| (e.iso3.clone(), e.name.clone()))
                                .filter(|(iso3, name)| {
                                    needle.is_empty()
                                        || name.to_lowercase().contains(&needle)
                                        || iso3.to_lowercase().contains(&needle)
                                })
                                .collect();

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
                                let page = if st.lang_visible > 0 {
                                    st.lang_visible
                                } else {
                                    est
                                };
                                let (moved, enter, align) =
                                    handle_list_keys(ctx, &mut st.lang_cursor, list.len(), page);
                                if enter {
                                    if let Some((iso3, _)) = list.get(st.lang_cursor) {
                                        st.lang_sel = Some(iso3.clone());
                                    }
                                }
                                let out = egui::ScrollArea::vertical()
                                    .id_salt("prop_lang_list")
                                    .auto_shrink([false, false])
                                    .show(list_ui, |ui| {
                                        let spacing = ui.spacing().item_spacing.y;
                                        let mut pitch = 0.0f32;
                                        for (idx, (iso3, name)) in list.iter().enumerate() {
                                            let is_cursor = idx == st.lang_cursor;
                                            let resp = ui.selectable_label(
                                                is_cursor,
                                                format!("{name}  ({iso3})"),
                                            );
                                            if idx == 0 {
                                                pitch = resp.rect.height() + spacing;
                                            }
                                            if resp.clicked() {
                                                st.lang_cursor = idx;
                                                st.lang_sel = Some(iso3.clone());
                                            }
                                            if moved && is_cursor {
                                                resp.scroll_to_me(align);
                                            }
                                        }
                                        pitch
                                    });
                                let pitch = out.inner.max(1.0);
                                st.lang_visible =
                                    ((out.inner_rect.height() / pitch).floor() as usize).max(1);

                                let ui = &mut cols[1];
                                let entry = st
                                    .lang_sel
                                    .as_deref()
                                    .and_then(|iso3| entries.iter().find(|e| e.iso3 == iso3));
                                match entry {
                                    Some(e) => {
                                        egui::ScrollArea::vertical()
                                            .id_salt("prop_lang_detail")
                                            .auto_shrink([false, false])
                                            .show(ui, |ui| {
                                                ui.label(
                                                    RichText::new(&e.name).strong().size(15.0),
                                                );
                                                let code = if e.iso1.is_empty() {
                                                    format!("ISO 639-3 : {}", e.iso3)
                                                } else {
                                                    format!(
                                                        "ISO 639-3 : {} · ISO 639-1 : {}",
                                                        e.iso3, e.iso1
                                                    )
                                                };
                                                ui.label(
                                                    RichText::new(code).color(Color32::GRAY),
                                                );
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
                                                    RichText::new(format!(
                                                        "{} ({})",
                                                        l_spoken_in,
                                                        e.countries.len()
                                                    ))
                                                    .strong(),
                                                );
                                                let names: Vec<String> = e
                                                    .countries
                                                    .iter()
                                                    .map(|a3| match countries.by_a3(a3) {
                                                        Some(c) => {
                                                            format!("{} ({})", c.name_fr, a3)
                                                        }
                                                        None => a3.clone(),
                                                    })
                                                    .collect();
                                                ui.label(names.join(", "));
                                            });
                                    }
                                    None => {
                                        ui.label(RichText::new(&l_pick_lang).color(Color32::GRAY));
                                    }
                                }
                            });
                        }
                    }
                });

                crate::ui::main_window::record_win_geom(cfg, ctx, "ximod_properties");
                if ctx.input(|i| i.viewport().close_requested())
                    || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
                {
                    do_close = true;
                }
            },
        );

        if do_close {
            self.show_properties = false;
            self.free_window_closed("ximod_properties");
        }
    }
}
