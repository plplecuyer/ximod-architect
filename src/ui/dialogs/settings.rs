//! Settings window (independent, fixed-size viewport).

use crate::config::Theme;
use crate::ui::main_window::{SettingsTab, XimodApp};
use crate::ui::widgets::free_window::record_win_geom;
use eframe::egui;

/// Fixed size of the "Settings" window (independent, movable, non-resizable:
/// no automatic and no manual sizing).
const SETTINGS_SIZE: [f32; 2] = [770.0, 480.0];

impl XimodApp {
    pub(crate) fn apply_theme(&self, ctx: &egui::Context) {
        // Visuals, palette and spacing live in `ui/theme.rs`. Font size is
        // handled by the per-frame live sync in `update()` (which reflects the
        // Settings preview immediately), and re-applied here because
        // `theme::apply` resets the style.
        crate::ui::theme::apply(ctx, self.config.theme);
        self.apply_font_size_value(ctx, self.applied_font_size.max(8.0));
    }

    pub(crate) fn render_settings_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }

        // Pre-translate all strings
        let title = self.i18n.t("settings-title");
        let tab_general = self.i18n.t("settings-tab-general");
        let tab_recent = self.i18n.t("settings-tab-recent-files");
        let label_lang = self.i18n.t("settings-language");
        let label_country_name = self.i18n.t("settings-country-name");
        let label_pick_country = self.i18n.t("settings-pick-country");
        let label_theme = self.i18n.t("settings-theme");
        let label_font_size = self.i18n.t("settings-font-size");
        let label_replace = self.i18n.t("settings-replace-newlines");
        let label_check_updates = self.i18n.t("settings-check-updates");
        let label_max_recent = self.i18n.t("settings-max-recent");
        let label_window_width = self.i18n.t("settings-window-width");
        let label_window_height = self.i18n.t("settings-window-height");
        let label_no_recent = self.i18n.t("settings-no-recent-files");
        let label_backup_count = self.i18n.t("settings-backup-count");
        let hint_backup_count = self.i18n.t("settings-backup-count-hint");
        let label_autosave = self.i18n.t("settings-autosave-minutes");
        let hint_autosave = self.i18n.t("settings-autosave-minutes-hint");
        let label_auto_masters = self.i18n.t("settings-auto-masters");
        let hint_auto_masters = self.i18n.t("settings-auto-masters-hint");
        let btn_save = self.i18n.t("btn-save");
        let btn_cancel = self.i18n.t("btn-cancel");
        let btn_clear = self.i18n.t("btn-clear");
        let _btn_remove = self.i18n.t("btn-remove");

        let available_locales: Vec<String> = self.i18n.available_locales().iter().map(|s| s.to_string()).collect();

        // Define focusable controls for General tab
        // 0: Tab General, 1: Tab Recent Files
        // 2: Language, 3: Theme, 4: Font Size
        // 5: Replace Newlines, 6: Max Recent Files
        // 7: Window Width, 8: Window Height
        // 9: Save Button, 10: Cancel Button
        const FOCUS_TAB_GENERAL: usize = 0;
        const FOCUS_TAB_RECENT: usize = 1;
        const FOCUS_LANGUAGE: usize = 2;
        const FOCUS_THEME: usize = 3;
        const FOCUS_FONT_SIZE: usize = 4;
        const FOCUS_REPLACE_NEWLINES: usize = 5;
        const FOCUS_MAX_RECENT: usize = 6;
        const FOCUS_WINDOW_WIDTH: usize = 7;
        const FOCUS_WINDOW_HEIGHT: usize = 8;
        const FOCUS_SAVE: usize = 9;
        const FOCUS_CANCEL: usize = 10;
        const MAX_FOCUS_GENERAL: usize = 10;

        // For Recent Files tab: 0, 1, 9 (Clear), 9 (Save), 10 (Cancel)
        const FOCUS_CLEAR: usize = 9;

        let mut should_close = false;
        let mut should_save = false;
        let mut clear_recent = false;
        let mut remove_index: Option<usize> = None;

        // Handle keyboard navigation - consume events to prevent propagation to main window
        let max_focus = if self.settings_tab == SettingsTab::General {
            MAX_FOCUS_GENERAL
        } else {
            FOCUS_CANCEL
        };

        // Independent, freely movable OS-level window of FIXED size (not
        // resizable): no automatic and no manual sizing. Only its position is
        // remembered in Config.ini.
        let vb = self
            .free_viewport_builder(ctx, "ximod_settings", title, SETTINGS_SIZE, true)
            .with_resizable(false)
            .with_min_inner_size(SETTINGS_SIZE)
            .with_max_inner_size(SETTINGS_SIZE)
            // First launch: stay above the main window (see `settings_on_top`).
            .with_window_level(if self.settings_on_top {
                egui::WindowLevel::AlwaysOnTop
            } else {
                egui::WindowLevel::Normal
            });
        let settings_vp = egui::ViewportId::from_hash_of("ximod_settings");
        if self.settings_focus_frames > 0 {
            self.settings_focus_frames -= 1;
            ctx.send_viewport_cmd_to(settings_vp, egui::ViewportCommand::Focus);
            ctx.request_repaint();
        }
        ctx.show_viewport_immediate(settings_vp, vb, |ctx, _class| {
            // Use input_mut to consume keyboard events (child viewport context)
            ctx.input_mut(|i| {
                // Consume and handle Tab navigation
                // Shift+Tab must be tested before plain Tab: `consume_key` ignores
                // extra modifiers, so testing Tab first swallowed Shift+Tab too.
                if i.consume_key(egui::Modifiers::SHIFT, egui::Key::Tab) {
                    // Shift+Tab: previous
                    if self.settings_focus == 0 {
                        self.settings_focus = max_focus;
                    } else {
                        self.settings_focus -= 1;
                    }
                    // Skip controls not in current tab
                    if self.settings_tab == SettingsTab::RecentFiles
                        && self.settings_focus > FOCUS_TAB_RECENT
                        && self.settings_focus < FOCUS_CLEAR
                    {
                        self.settings_focus = FOCUS_TAB_RECENT;
                    }
                } else if i.consume_key(egui::Modifiers::NONE, egui::Key::Tab) {
                    // Tab: next
                    self.settings_focus += 1;
                    if self.settings_focus > max_focus {
                        self.settings_focus = 0;
                    }
                    // Skip controls not in current tab
                    if self.settings_tab == SettingsTab::RecentFiles
                        && self.settings_focus > FOCUS_TAB_RECENT
                        && self.settings_focus < FOCUS_CLEAR
                    {
                        self.settings_focus = FOCUS_CLEAR;
                    }
                }

                // Arrow key navigation for combo boxes and number fields
                if self.settings_tab == SettingsTab::General {
                    match self.settings_focus {
                        FOCUS_LANGUAGE => {
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)
                                || i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)
                            {
                                let idx = available_locales
                                    .iter()
                                    .position(|l| l == &self.temp_locale)
                                    .unwrap_or(0);
                                if idx > 0 {
                                    self.temp_locale = available_locales[idx - 1].clone();
                                }
                            }
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
                                || i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight)
                            {
                                let idx = available_locales
                                    .iter()
                                    .position(|l| l == &self.temp_locale)
                                    .unwrap_or(0);
                                if idx < available_locales.len() - 1 {
                                    self.temp_locale = available_locales[idx + 1].clone();
                                }
                            }
                        }
                        FOCUS_THEME => {
                            let themes = Theme::variants();
                            let idx = themes.iter().position(|t| *t == self.temp_theme).unwrap_or(0);
                            if (i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)
                                || i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft))
                                && idx > 0
                            {
                                self.temp_theme = themes[idx - 1];
                            }
                            if (i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
                                || i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight))
                                && idx < themes.len() - 1
                            {
                                self.temp_theme = themes[idx + 1];
                            }
                        }
                        FOCUS_FONT_SIZE => {
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                                self.temp_font_size = (self.temp_font_size - 0.5).max(8.0);
                            }
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                                self.temp_font_size = (self.temp_font_size + 0.5).min(24.0);
                            }
                        }
                        FOCUS_MAX_RECENT => {
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                                self.temp_max_recent_files = self.temp_max_recent_files.saturating_sub(1).max(1);
                            }
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                                self.temp_max_recent_files = (self.temp_max_recent_files + 1).min(20);
                            }
                        }
                        FOCUS_WINDOW_WIDTH => {
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                                self.temp_window_width = (self.temp_window_width - 10.0).max(800.0);
                            }
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                                self.temp_window_width = (self.temp_window_width + 10.0).min(3840.0);
                            }
                        }
                        FOCUS_WINDOW_HEIGHT => {
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                                self.temp_window_height = (self.temp_window_height - 10.0).max(600.0);
                            }
                            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                                self.temp_window_height = (self.temp_window_height + 10.0).min(2160.0);
                            }
                        }
                        // Checkbox toggle with Space
                        FOCUS_REPLACE_NEWLINES if i.consume_key(egui::Modifiers::NONE, egui::Key::Space) => {
                            self.temp_replace_newlines = !self.temp_replace_newlines;
                        }
                        _ => {}
                    }
                }

                // Tab switching with Space/Enter
                if self.settings_focus == FOCUS_TAB_GENERAL
                    && (i.consume_key(egui::Modifiers::NONE, egui::Key::Space)
                        || i.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
                {
                    self.settings_tab = SettingsTab::General;
                }
                if self.settings_focus == FOCUS_TAB_RECENT
                    && (i.consume_key(egui::Modifiers::NONE, egui::Key::Space)
                        || i.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
                {
                    self.settings_tab = SettingsTab::RecentFiles;
                }

                // Enter for buttons
                if self.settings_focus == FOCUS_SAVE && i.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                    should_save = true;
                    should_close = true;
                }
                if self.settings_focus == FOCUS_CLEAR
                    && self.settings_tab == SettingsTab::RecentFiles
                    && i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                {
                    clear_recent = true;
                }
                if self.settings_focus == FOCUS_CANCEL && i.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                    should_close = true;
                }

                // Escape to close
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                    should_close = true;
                }
            });

            // Helper function for focused style
            let focused_stroke = egui::Stroke::new(2.0_f32, crate::ui::theme::Palette::of(&ctx.style().visuals).accent);

            // Save / Cancel: a bottom panel, so the buttons stay visible whatever
            // the height of the tab content (which scrolls above them).
            egui::TopBottomPanel::bottom("ximod_settings_buttons").show(ctx, |ui| {
                ui.add_space(6.0);
                // Buttons with focus indication
                ui.horizontal(|ui| {
                    let save_btn = ui.button(&btn_save);
                    if self.settings_focus == FOCUS_SAVE {
                        ui.painter().rect_stroke(save_btn.rect, 2.0, focused_stroke);
                    }
                    if save_btn.clicked() {
                        should_save = true;
                        should_close = true;
                        self.settings_focus = FOCUS_SAVE;
                    }

                    let cancel_btn = ui.button(&btn_cancel);
                    if self.settings_focus == FOCUS_CANCEL {
                        ui.painter().rect_stroke(cancel_btn.rect, 2.0, focused_stroke);
                    }
                    if cancel_btn.clicked() {
                        should_close = true;
                        self.settings_focus = FOCUS_CANCEL;
                    }
                });
                ui.add_space(4.0);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                // Tabs with focus indication
                ui.horizontal(|ui| {
                    let tab_gen_response = ui.selectable_label(self.settings_tab == SettingsTab::General, &tab_general);
                    if self.settings_focus == FOCUS_TAB_GENERAL {
                        ui.painter().rect_stroke(tab_gen_response.rect, 2.0, focused_stroke);
                    }
                    if tab_gen_response.clicked() {
                        self.settings_tab = SettingsTab::General;
                        self.settings_focus = FOCUS_TAB_GENERAL;
                    }

                    let tab_rec_response =
                        ui.selectable_label(self.settings_tab == SettingsTab::RecentFiles, &tab_recent);
                    if self.settings_focus == FOCUS_TAB_RECENT {
                        ui.painter().rect_stroke(tab_rec_response.rect, 2.0, focused_stroke);
                    }
                    if tab_rec_response.clicked() {
                        self.settings_tab = SettingsTab::RecentFiles;
                        self.settings_focus = FOCUS_TAB_RECENT;
                    }
                });

                ui.separator();
                ui.add_space(8.0);

                // The content area takes all the height left above the button
                // panel (the window itself is fixed-size, so this is constant);
                // the longer tab scrolls.
                let content_size = egui::vec2(ui.available_width(), ui.available_height());
                ui.allocate_ui(content_size, |ui| {
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                        match self.settings_tab {
                            SettingsTab::General => {
                                // ---- Two columns: flag + country name (left),
                                // language / theme / font size (right) ----
                                ui.horizontal_top(|ui| {
                                    // --- Left: clickable flag opening the picker
                                    let flag_size = egui::vec2(132.0, 88.0);
                                    let flag_path = self
                                        .countries
                                        .flag_for(&self.temp_country)
                                        .and_then(|f| crate::data::flags_dir().map(|d| d.join(f)))
                                        .filter(|p| p.is_file());

                                    let flag_resp = match &flag_path {
                                        Some(p) => ui.add_sized(
                                            flag_size,
                                            egui::ImageButton::new(
                                                egui::Image::from_uri(format!("file://{}", p.display()))
                                                    .fit_to_exact_size(flag_size),
                                            ),
                                        ),
                                        None => ui.add_sized(
                                            flag_size,
                                            egui::Button::new(egui::RichText::new("\u{1F3F3}").size(28.0)),
                                        ),
                                    };
                                    if flag_resp.on_hover_text(&label_pick_country).clicked() {
                                        self.flag_target = crate::ui::flag_picker::FlagTarget::Settings;
                                        self.flag_filter.clear();
                                        self.flag_cursor = 0;
                                        self.flag_scroll_offset = 0.0;
                                        self.show_flag_picker = true;
                                        self.flag_picker_focus_frames = 30;
                                    }

                                    ui.add_space(10.0);

                                    // --- Country name (read-only, two lines)
                                    ui.vertical(|ui| {
                                        ui.label(&label_country_name);
                                        let mut endonym = self
                                            .countries
                                            .endonym_for(&self.temp_country, &self.temp_locale)
                                            .unwrap_or("")
                                            .to_string();
                                        // Multiline so long names ("Royaume-Uni de
                                        // Grande-Bretagne et d'Irlande du Nord",
                                        // 51 chars) wrap at word boundaries instead
                                        // of being cut off.
                                        ui.add_enabled(
                                            false,
                                            egui::TextEdit::multiline(&mut endonym)
                                                .desired_width(240.0)
                                                .desired_rows(2),
                                        );
                                    });

                                    ui.add_space(14.0);

                                    // --- Right: language / theme / font size
                                    ui.vertical(|ui| {
                                        // Language: only enabled once a country is set.
                                        let country_langs: Vec<String> =
                                            self.country_languages.languages_for(&self.temp_country).to_vec();
                                        let lang_enabled = !country_langs.is_empty();

                                        ui.horizontal(|ui| {
                                            ui.label(&label_lang);
                                            let current_display = self.i18n.display_name(&self.temp_locale);
                                            ui.add_enabled_ui(lang_enabled, |ui| {
                                                let combo = egui::ComboBox::from_id_salt("lang_combo")
                                                    .selected_text(current_display)
                                                    .height(260.0)
                                                    .show_ui(ui, |ui| {
                                                        egui::ScrollArea::both().auto_shrink([false, false]).show(
                                                            ui,
                                                            |ui| {
                                                                for locale in &country_langs {
                                                                    let display = self.i18n.display_name(locale);
                                                                    if ui
                                                                        .selectable_label(
                                                                            self.temp_locale == *locale,
                                                                            display,
                                                                        )
                                                                        .clicked()
                                                                    {
                                                                        self.temp_locale = locale.clone();
                                                                    }
                                                                }
                                                            },
                                                        );
                                                    });
                                                if self.settings_focus == FOCUS_LANGUAGE {
                                                    ui.painter().rect_stroke(combo.response.rect, 2.0, focused_stroke);
                                                }
                                                if combo.response.clicked() {
                                                    self.settings_focus = FOCUS_LANGUAGE;
                                                }
                                            });
                                        });

                                        ui.add_space(4.0);

                                        // Theme selection
                                        ui.horizontal(|ui| {
                                            ui.label(&label_theme);
                                            let combo = egui::ComboBox::from_id_salt("theme_combo")
                                                .selected_text(crate::ui::labels::theme(&self.i18n, self.temp_theme))
                                                .show_ui(ui, |ui| {
                                                    for theme in Theme::variants() {
                                                        if ui
                                                            .selectable_label(
                                                                self.temp_theme == *theme,
                                                                crate::ui::labels::theme(&self.i18n, *theme),
                                                            )
                                                            .clicked()
                                                        {
                                                            self.temp_theme = *theme;
                                                        }
                                                    }
                                                });
                                            if self.settings_focus == FOCUS_THEME {
                                                ui.painter().rect_stroke(combo.response.rect, 2.0, focused_stroke);
                                            }
                                            if combo.response.clicked() {
                                                self.settings_focus = FOCUS_THEME;
                                            }
                                        });

                                        ui.add_space(4.0);

                                        // Font size
                                        ui.horizontal(|ui| {
                                            ui.label(&label_font_size);
                                            let drag = ui.add(
                                                egui::DragValue::new(&mut self.temp_font_size)
                                                    .range(8.0..=24.0)
                                                    .speed(0.5),
                                            );
                                            if self.settings_focus == FOCUS_FONT_SIZE {
                                                ui.painter().rect_stroke(drag.rect, 2.0, focused_stroke);
                                            }
                                            if drag.clicked() {
                                                self.settings_focus = FOCUS_FONT_SIZE;
                                            }
                                        });
                                    });
                                });

                                ui.add_space(12.0);
                                ui.separator();
                                ui.add_space(8.0);

                                // Checkbox with focus indication
                                let cb = ui.checkbox(&mut self.temp_replace_newlines, &label_replace);
                                if self.settings_focus == FOCUS_REPLACE_NEWLINES {
                                    ui.painter().rect_stroke(cb.rect, 2.0, focused_stroke);
                                }
                                if cb.clicked() {
                                    self.settings_focus = FOCUS_REPLACE_NEWLINES;
                                }

                                ui.add_space(8.0);

                                // Check for updates on startup
                                ui.checkbox(&mut self.temp_check_updates, &label_check_updates);

                                ui.add_space(12.0);
                                ui.separator();
                                ui.add_space(8.0);

                                // Max recent files
                                ui.horizontal(|ui| {
                                    ui.label(&label_max_recent);
                                    let mut max_recent = self.temp_max_recent_files as i32;
                                    let drag = ui.add(egui::DragValue::new(&mut max_recent).range(1..=20).speed(0.2));
                                    if drag.changed() {
                                        self.temp_max_recent_files = max_recent as usize;
                                    }
                                    if self.settings_focus == FOCUS_MAX_RECENT {
                                        ui.painter().rect_stroke(drag.rect, 2.0, focused_stroke);
                                    }
                                    if drag.clicked() {
                                        self.settings_focus = FOCUS_MAX_RECENT;
                                    }
                                });

                                ui.add_space(4.0);

                                // Window size
                                ui.horizontal(|ui| {
                                    ui.label(&label_window_width);
                                    let drag = ui.add(
                                        egui::DragValue::new(&mut self.temp_window_width)
                                            .range(800.0..=3840.0)
                                            .speed(10.0),
                                    );
                                    if self.settings_focus == FOCUS_WINDOW_WIDTH {
                                        ui.painter().rect_stroke(drag.rect, 2.0, focused_stroke);
                                    }
                                    if drag.clicked() {
                                        self.settings_focus = FOCUS_WINDOW_WIDTH;
                                    }
                                });

                                ui.add_space(4.0);

                                ui.horizontal(|ui| {
                                    ui.label(&label_window_height);
                                    let drag = ui.add(
                                        egui::DragValue::new(&mut self.temp_window_height)
                                            .range(600.0..=2160.0)
                                            .speed(10.0),
                                    );
                                    if self.settings_focus == FOCUS_WINDOW_HEIGHT {
                                        ui.painter().rect_stroke(drag.rect, 2.0, focused_stroke);
                                    }
                                    if drag.clicked() {
                                        self.settings_focus = FOCUS_WINDOW_HEIGHT;
                                    }
                                });

                                ui.add_space(12.0);
                                ui.separator();
                                ui.add_space(8.0);

                                // Rotating backups of the FOMOD XML (0 = off).
                                ui.horizontal(|ui| {
                                    ui.label(&label_backup_count).on_hover_text(&hint_backup_count);
                                    let mut count = self.temp_backup_count as i32;
                                    let drag = ui
                                        .add(egui::DragValue::new(&mut count).range(0..=100).speed(0.2))
                                        .on_hover_text(&hint_backup_count);
                                    if drag.changed() {
                                        self.temp_backup_count = count.max(0) as usize;
                                    }
                                });

                                ui.add_space(4.0);

                                // Recovery autosave period in minutes (0 = off).
                                ui.horizontal(|ui| {
                                    ui.label(&label_autosave).on_hover_text(&hint_autosave);
                                    let mut minutes = self.temp_autosave_minutes as i32;
                                    let drag = ui
                                        .add(egui::DragValue::new(&mut minutes).range(0..=120).speed(0.2))
                                        .on_hover_text(&hint_autosave);
                                    if drag.changed() {
                                        self.temp_autosave_minutes = minutes.max(0) as u32;
                                    }
                                });

                                ui.add_space(8.0);

                                // Masters of an added plugin become file dependencies.
                                ui.checkbox(&mut self.temp_auto_masters, &label_auto_masters)
                                    .on_hover_text(&hint_auto_masters);
                            }

                            SettingsTab::RecentFiles => {
                                if self.config.recent_files.is_empty() {
                                    ui.label(&label_no_recent);
                                } else {
                                    // Stable width for the path column, derived
                                    // from the fixed content width rather than
                                    // ui.available_width() — the latter shifts by
                                    // a few pixels as the scrollbar toggles, which
                                    // changed the elided path every frame and made
                                    // the whole list tremble. Reserve room for the
                                    // index, the ✕ button, spacing and scrollbar.
                                    for (idx, path) in self.config.recent_files.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{}.", idx + 1));
                                            // Button first: the label takes all
                                            // remaining width, which would
                                            // otherwise push it out of view.
                                            if crate::ui::components::delete_button(ui).clicked() {
                                                remove_index = Some(idx);
                                            }
                                            // Show only the folder name (the mod's
                                            // root); the full path stays as a
                                            // tooltip. Displaying the short name
                                            // avoids the width-dependent elision
                                            // that made the longest line flicker
                                            // between two truncations each frame.
                                            let full = path.display().to_string();
                                            let name = path
                                                .file_name()
                                                .map(|n| n.to_string_lossy().to_string())
                                                .unwrap_or_else(|| full.clone());
                                            ui.add(egui::Label::new(name).wrap_mode(egui::TextWrapMode::Truncate))
                                                .on_hover_text(full);
                                        });
                                    }

                                    ui.add_space(8.0);

                                    let clear_btn = ui.button(&btn_clear);
                                    if self.settings_focus == FOCUS_CLEAR {
                                        ui.painter().rect_stroke(clear_btn.rect, 2.0, focused_stroke);
                                    }
                                    if clear_btn.clicked() {
                                        clear_recent = true;
                                        self.settings_focus = FOCUS_CLEAR;
                                    }
                                }
                            }
                        }
                    });
                });
            });

            record_win_geom(&mut self.config, ctx, "ximod_settings");
            if ctx.input(|i| i.viewport().close_requested()) {
                should_close = true;
            }
        });

        // Handle recent files modifications
        if let Some(idx) = remove_index
            && idx < self.config.recent_files.len()
        {
            self.config.recent_files.remove(idx);
            let _ = self.config.save();
        }

        if clear_recent {
            self.config.recent_files.clear();
            let _ = self.config.save();
        }

        // Apply and save settings
        if should_save {
            // config now stores ISO 639-3 directly (temp_locale is ISO 639-3).
            let locale_changed = self.config.locale != self.temp_locale;

            self.config.locale = self.temp_locale.clone();
            self.config.country = self.temp_country.clone();
            // Initial configuration is now done: the program will start in the
            // chosen language from now on (FirstStart=1).
            self.config.first_start_done = true;
            self.settings_on_top = false;
            self.settings_focus_frames = 0;
            self.config.theme = self.temp_theme;
            self.config.font_size = self.temp_font_size;
            self.config.replace_newlines = self.temp_replace_newlines;
            self.config.max_recent_files = self.temp_max_recent_files;
            self.config.window_width = self.temp_window_width;
            self.config.window_height = self.temp_window_height;
            self.config.check_updates = self.temp_check_updates;
            self.config.backup_count = self.temp_backup_count;
            self.config.autosave_minutes = self.temp_autosave_minutes;
            self.config.auto_masters = self.temp_auto_masters;

            self.i18n.set_locale(&self.temp_locale);
            self.apply_theme(ctx);

            // Force complete UI rebuild when locale changes
            if locale_changed {
                // Increment locale version to create new menu IDs

                // Clear cached UI state - this forces menus to recalculate their sizes
                ctx.memory_mut(|mem| {
                    mem.data.clear();
                });

                // Force immediate repaint with new IDs
                ctx.request_repaint();
            }

            match self.config.save() {
                Err(e) => {
                    self.notify_err(format!("{}: {}", self.i18n.t("msg-settings-save-error"), e));
                }
                _ => {
                    self.notify_ok(self.i18n.t("status-settings-saved"));
                }
            }
        }

        if should_close {
            // Whatever the outcome, the first-launch "stay on top" ends here.
            self.settings_on_top = false;
            self.settings_focus_frames = 0;
            // Reset temp values to current config on cancel
            if !should_save {
                // config.locale is ISO 639-3.
                self.temp_locale = self.config.locale.clone();
                self.temp_country = self.config.country.clone();
                self.temp_theme = self.config.theme;
                self.temp_font_size = self.config.font_size;
                self.temp_replace_newlines = self.config.replace_newlines;
                self.temp_max_recent_files = self.config.max_recent_files;
                self.temp_window_width = self.config.window_width;
                self.temp_window_height = self.config.window_height;
                self.temp_check_updates = self.config.check_updates;
                self.temp_backup_count = self.config.backup_count;
                self.temp_autosave_minutes = self.config.autosave_minutes;
                self.temp_auto_masters = self.config.auto_masters;
            }
            self.show_settings = false;
            self.settings_tab = SettingsTab::General;
            self.settings_focus = 0;
            self.free_window_closed("ximod_settings");
        }
    }
}
