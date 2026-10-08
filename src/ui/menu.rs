//! Menu bar, toolbar and the global keyboard shortcuts that mirror them.

use super::main_window::{SettingsTab, XimodApp};
use eframe::egui;

/// Keyboard shortcuts of the main menu, so the menu display and the global
/// handler stay in sync.
struct MenuShortcuts {
    new: egui::KeyboardShortcut,
    open: egui::KeyboardShortcut,
    open_file: egui::KeyboardShortcut,
    save: egui::KeyboardShortcut,
    save_as: egui::KeyboardShortcut,
    undo: egui::KeyboardShortcut,
    redo: egui::KeyboardShortcut,
    redo_alt: egui::KeyboardShortcut,
    settings: egui::KeyboardShortcut,
    quit: egui::KeyboardShortcut,
    manual: egui::KeyboardShortcut,
}

impl XimodApp {
    /// Open the Settings dialog, seeding the temporary values from the current
    /// configuration. Shared by the "Settings" menu item and the Ctrl+, shortcut.
    pub(crate) fn open_settings(&mut self) {
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
        self.settings_focus = 0;
        self.settings_tab = SettingsTab::General;
        self.show_settings = true;
    }

    /// Open the reusable-templates window, (re)loading the saved templates.
    pub(crate) fn open_templates(&mut self) {
        if let Some(dir) = crate::models::templates::templates_dir() {
            self.templates = crate::models::templates::load_templates(&dir).unwrap_or_default();
        }
        self.show_templates = true;
    }

    /// Keyboard shortcuts shared by the menu bar (for display) and the global
    /// handler (for action). COMMAND maps to Ctrl on Windows/Linux and Cmd on macOS.
    fn menu_shortcuts() -> MenuShortcuts {
        use egui::{Key, KeyboardShortcut, Modifiers};
        MenuShortcuts {
            new: KeyboardShortcut::new(Modifiers::COMMAND, Key::N),
            open: KeyboardShortcut::new(Modifiers::COMMAND, Key::O),
            open_file: KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::SHIFT, Key::O),
            save: KeyboardShortcut::new(Modifiers::COMMAND, Key::S),
            save_as: KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::SHIFT, Key::S),
            undo: KeyboardShortcut::new(Modifiers::COMMAND, Key::Z),
            redo: KeyboardShortcut::new(Modifiers::COMMAND, Key::Y),
            redo_alt: KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::SHIFT, Key::Z),
            settings: KeyboardShortcut::new(Modifiers::COMMAND, Key::Comma),
            quit: KeyboardShortcut::new(Modifiers::COMMAND, Key::Q),
            manual: KeyboardShortcut::new(Modifiers::NONE, Key::F1),
        }
    }

    /// Trigger menu actions from global keyboard shortcuts. Ignored while a modal
    /// dialog is open, so shortcuts don't fire behind a dialog.
    pub(crate) fn handle_menu_shortcuts(&mut self, ctx: &egui::Context) {
        // Only real modal states swallow the shortcuts. Tool windows (About,
        // validation report, scripts, translation, read-only XML view) are
        // independent, non-modal windows: Ctrl+S must keep working with the
        // validation report open.
        if self.show_settings
            || self.show_confirm
            || self.show_exit_prompt
            || self.close_prompt.is_some()
            || self.export_job.is_some()
            || self.show_backups
            || self.show_plugin_report
            || self.show_archive_view
            || (self.show_xml_editor && self.xml_editor_editing)
        {
            return;
        }
        let sc = Self::menu_shortcuts();

        // Ctrl+Shift+O / Ctrl+Shift+S must be tested before Ctrl+O / Ctrl+S
        // (more specific first).
        if ctx.input_mut(|i| i.consume_shortcut(&sc.open_file)) {
            self.open_file();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.open)) {
            self.open_directory();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.new)) {
            self.request_new_project();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.redo_alt) || i.consume_shortcut(&sc.redo)) {
            self.redo();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.undo)) {
            self.undo();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.save_as)) {
            self.save_project_as();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.save)) {
            self.save_project();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.settings)) {
            self.open_settings();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.manual)) {
            self.open_manual();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.quit)) {
            self.request_close = true;
        }
    }

    /// egui keeps a menu open until something is clicked. Sliding along the
    /// bar opens each menu in turn by hover; the last one would then stay
    /// open after the pointer left the bar without choosing anything. A menu
    /// that was opened by *hover* (not by a click) is therefore closed as
    /// soon as the pointer is outside both the bar and the menu itself.
    fn close_menu_left_by_hover(&mut self, ctx: &egui::Context, bar_rect: egui::Rect) {
        let Some(bar_id) = self.menu_bar_id else { return };
        let mut state = egui::menu::BarState::load(ctx, bar_id);
        let Some(root) = state.as_ref() else {
            self.menu_open_by_hover = None;
            return;
        };
        let root_id = root.id;
        let clicked_now = ctx.input(|i| i.pointer.any_click());
        match self.menu_open_by_hover {
            // A new menu root while one was already open: opened by hover,
            // unless this very frame carried a click.
            Some((prev, _)) if prev != root_id => self.menu_open_by_hover = Some((root_id, !clicked_now)),
            Some(_) => {}
            None => self.menu_open_by_hover = Some((root_id, false)),
        }
        let by_hover = self.menu_open_by_hover.is_some_and(|(_, h)| h);
        if !by_hover {
            return;
        }
        let Some(pos) = ctx.input(|i| i.pointer.latest_pos()) else {
            return;
        };
        let inside = bar_rect.contains(pos) || root.menu_state.read().area_contains(pos);
        if !inside {
            **state = None;
            state.store(ctx, bar_id);
            self.menu_open_by_hover = None;
            ctx.request_repaint();
        }
    }

    pub(crate) fn render_menu_bar(&mut self, ctx: &egui::Context) {
        // Check if a modal dialog is open. The Settings, About and Script
        // windows are independent, freely movable OS-level windows, so they
        // do not block the main window.
        let modal_open = self.show_confirm
            || self.export_job.is_some()
            || self.show_backups
            || self.show_plugin_report
            || self.show_archive_view
            || (self.show_xml_editor && self.xml_editor_editing);

        let t = |k: &str| self.i18n.t(k);
        let menu_file = t("menu-file");
        let menu_new = t("menu-new");
        let menu_open = t("menu-open");
        let menu_open_file = t("menu-open-file");
        let menu_open_archive = t("menu-open-archive");
        let menu_save = t("menu-save");
        let menu_save_as = t("menu-save-as");
        let menu_merge = t("menu-merge");
        let menu_export = t("menu-export");
        let menu_restore_backup = t("menu-restore-backup");
        let menu_plugin_report = t("menu-plugin-report");
        let menu_archive_contents = t("menu-archive-contents");
        let menu_nexus_desc = t("menu-nexus-desc");
        let menu_project_strings = t("menu-project-strings");
        let menu_new_from_folder = t("menu-new-from-folder");
        let menu_templates = t("menu-templates");
        let menu_compare = t("menu-compare");
        let menu_condition_editor = t("menu-condition-editor");
        let menu_translate_fomod = t("menu-translate-fomod");
        let menu_close_fomod = t("menu-close-fomod");
        let menu_close_all = t("menu-close-all-fomods");
        let menu_recent = t("menu-recent");
        let menu_exit = t("menu-exit");
        let menu_project = t("menu-project");
        let menu_edit = t("menu-edit");
        let menu_undo = t("menu-undo");
        let menu_redo = t("menu-redo");
        let menu_tools = t("menu-tools");
        let menu_xml_editor = t("xml-editor-title");
        let menu_settings = t("menu-settings");
        let menu_pre_save = t("menu-pre-save-script");
        let menu_post_save = t("menu-post-save-script");
        let menu_translation = t("menu-translation");
        let menu_preview = t("menu-preview");
        let menu_validate = t("menu-validate");
        let menu_properties = t("menu-properties");
        let menu_help = t("menu-help");
        let menu_manual = t("menu-manual");
        let menu_about = t("menu-about");
        let menu_check_updates = t("menu-check-updates");
        let label_empty = t("label-empty");
        let tb_new = crate::ui::theme::with_icon(crate::ui::theme::icon::NEW, &t("toolbar-new"));
        let tb_open = crate::ui::theme::with_icon(crate::ui::theme::icon::OPEN, &t("toolbar-open"));
        let tb_save = crate::ui::theme::with_icon(crate::ui::theme::icon::SAVE, &t("toolbar-save"));
        let tb_validate = crate::ui::theme::with_icon(crate::ui::theme::icon::VALIDATE, &t("toolbar-validate"));
        let tb_preview = crate::ui::theme::with_icon(crate::ui::theme::icon::PREVIEW, &t("toolbar-preview"));
        let tb_export = crate::ui::theme::with_icon(crate::ui::theme::icon::EXPORT, &t("toolbar-export"));

        let has_root = self.root_directory.is_some();
        let recent_files = self.config.recent_files.clone();

        // Shortcut display strings (platform-aware via format_shortcut).
        let sc = Self::menu_shortcuts();
        let sct_new = ctx.format_shortcut(&sc.new);
        let sct_open = ctx.format_shortcut(&sc.open);
        let sct_open_file = ctx.format_shortcut(&sc.open_file);
        let sct_save_as = ctx.format_shortcut(&sc.save_as);
        let sct_save = ctx.format_shortcut(&sc.save);
        // egui renders the Comma key by name ("Comma"); show the symbol instead
        // so the shortcut reads "Ctrl+," (or "⌘," on macOS) rather than "Ctrl+Comma".
        let sct_settings = ctx.format_shortcut(&sc.settings).replace("Comma", ",");
        let sct_quit = ctx.format_shortcut(&sc.quit);
        let sct_manual = ctx.format_shortcut(&sc.manual);
        let sct_undo = ctx.format_shortcut(&sc.undo);
        let sct_redo = ctx.format_shortcut(&sc.redo);
        let can_undo = self.history.can_undo();
        let can_redo = self.history.can_redo();

        // A menu entry: wide label, optional shortcut hint, optional enabling.
        fn item(ui: &mut egui::Ui, label: &str, shortcut: Option<&str>, enabled: bool) -> bool {
            let mut btn = egui::Button::new(label).wrap_mode(egui::TextWrapMode::Extend);
            if let Some(sc) = shortcut {
                btn = btn.shortcut_text(sc);
            }
            let clicked = ui.add_enabled(enabled, btn).clicked();
            if clicked {
                ui.close_menu();
            }
            clicked
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            // Disable menu when modal is open
            if modal_open {
                ui.disable();
            }

            let bar = egui::menu::bar(ui, |ui| {
                // The bar's own id keys egui's menu state (see below).
                self.menu_bar_id = Some(ui.id());
                // ---------------------------------------------------- File
                ui.menu_button(&menu_file, |ui| {
                    if item(ui, &menu_new, Some(&sct_new), true) {
                        self.request_new_project();
                    }
                    if item(ui, &menu_new_from_folder, None, true) {
                        self.new_from_folder();
                    }
                    if item(ui, &menu_open, Some(&sct_open), true) {
                        self.open_directory();
                    }
                    if item(ui, &menu_open_file, Some(&sct_open_file), true) {
                        self.open_file();
                    }
                    if item(ui, &menu_open_archive, None, true) {
                        self.open_archive_dialog(&ui.ctx().clone());
                    }
                    ui.separator();
                    // Save is always available: without a root folder it asks
                    // for one first (a greyed-out entry gave no clue why).
                    if item(ui, &menu_save, Some(&sct_save), true) {
                        self.save_project();
                    }
                    if item(ui, &menu_save_as, Some(&sct_save_as), true) {
                        self.save_project_as();
                    }
                    // One `read_dir` of `fomod/backups`, only while the menu is open.
                    let has_backups = self.root_directory.as_deref().is_some_and(crate::backups::has_backups);
                    if item(ui, &menu_restore_backup, None, has_root && has_backups) {
                        self.open_backups_dialog();
                    }
                    ui.separator();
                    if item(ui, &menu_merge, None, has_root) {
                        self.merge_fomod();
                    }
                    if item(ui, &menu_export, None, has_root) {
                        self.export_distribution(&ui.ctx().clone());
                    }
                    ui.separator();
                    if item(ui, &menu_close_fomod, None, true) {
                        self.close_active_fomod();
                    }
                    if item(ui, &menu_close_all, None, true) {
                        self.close_all_fomods();
                    }
                    ui.separator();
                    ui.menu_button(&menu_recent, |ui| {
                        for path in &recent_files {
                            // Show only the last path component (the mod's root
                            // folder name); the full path stays as a tooltip.
                            let display = path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| path.to_string_lossy().to_string());
                            let btn_path = egui::Button::new(&display).wrap_mode(egui::TextWrapMode::Extend);
                            if ui.add(btn_path).on_hover_text(path.to_string_lossy()).clicked() {
                                self.load_project(path.clone());
                                ui.close_menu();
                            }
                        }
                        if recent_files.is_empty() {
                            ui.label(&label_empty);
                        }
                    });
                    ui.separator();
                    if item(ui, &menu_exit, Some(&sct_quit), true) {
                        self.request_close = true;
                    }
                });

                // ---------------------------------------------------- Edit
                ui.menu_button(&menu_edit, |ui| {
                    if item(ui, &menu_undo, Some(&sct_undo), can_undo) {
                        self.undo();
                    }
                    if item(ui, &menu_redo, Some(&sct_redo), can_redo) {
                        self.redo();
                    }
                });

                // ------------------------------------------------- Project
                ui.menu_button(&menu_project, |ui| {
                    if item(ui, &menu_validate, None, true) {
                        self.run_full_validation(&ui.ctx().clone());
                    }
                    if item(ui, &menu_preview, None, true) {
                        self.open_preview();
                    }
                    ui.separator();
                    if item(ui, &menu_condition_editor, None, true) {
                        self.condition_editor.open = true;
                    }
                    if item(ui, &menu_translate_fomod, None, true) {
                        self.open_fomod_translation(true);
                    }
                    if item(ui, &menu_compare, None, has_root) {
                        self.compare_with();
                    }
                    if item(ui, &menu_templates, None, true) {
                        self.open_templates();
                    }
                    ui.separator();
                    // XML editor: view/edit info.xml and ModuleConfig.xml.
                    ui.menu_button(&menu_xml_editor, |ui| {
                        if item(ui, "info.xml", None, true) {
                            self.open_xml_editor(crate::ui::xml_editor::XmlTarget::InfoXml);
                        }
                        if item(ui, "ModuleConfig.xml", None, true) {
                            self.open_xml_editor(crate::ui::xml_editor::XmlTarget::ModuleConfig);
                        }
                    });
                });

                // --------------------------------------------------- Tools
                ui.menu_button(&menu_tools, |ui| {
                    if item(ui, &menu_settings, Some(&sct_settings), true) {
                        self.open_settings();
                    }
                    ui.separator();
                    if item(ui, &menu_pre_save, None, true) {
                        self.editing_pre_script = true;
                        self.script_content = self.config.pre_save_script.clone();
                        self.show_script_dialog = true;
                    }
                    if item(ui, &menu_post_save, None, true) {
                        self.editing_pre_script = false;
                        self.script_content = self.config.post_save_script.clone();
                        self.show_script_dialog = true;
                    }
                    ui.separator();
                    if item(ui, &menu_plugin_report, None, true) {
                        self.open_plugin_report();
                    }
                    if item(ui, &menu_archive_contents, None, true) {
                        self.open_archive_view_pick();
                    }
                    if item(ui, &menu_nexus_desc, None, true) {
                        self.open_nexus_desc();
                    }
                    if item(ui, &menu_project_strings, None, true) {
                        self.open_project_strings();
                    }
                    ui.separator();
                    if item(ui, &menu_translation, None, true) {
                        self.open_translation_editor();
                    }
                    if item(ui, &menu_properties, None, true) {
                        self.open_properties();
                    }
                });

                // ---------------------------------------------------- Help
                ui.menu_button(&menu_help, |ui| {
                    if item(ui, &menu_manual, Some(&sct_manual), true) {
                        self.open_manual();
                    }
                    if item(ui, &menu_check_updates, None, true) {
                        self.start_update_check(&ui.ctx().clone(), true);
                    }
                    ui.separator();
                    if item(ui, &menu_about, None, true) {
                        self.show_about = true;
                    }
                });
            });
            self.close_menu_left_by_hover(ctx, bar.response.rect);

            // ------------------------------------------------------ Toolbar
            // The everyday actions, one click away. Labels come from the
            // locale; the icons will follow with the icon font (lot U6).
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                let tool = |ui: &mut egui::Ui, label: &str, hint: &str, enabled: bool| -> bool {
                    ui.add_enabled(enabled, egui::Button::new(label))
                        .on_hover_text(hint)
                        .clicked()
                };
                if tool(ui, &tb_new, &format!("{menu_new}  ({sct_new})"), true) {
                    self.request_new_project();
                }
                if tool(ui, &tb_open, &format!("{menu_open}  ({sct_open})"), true) {
                    self.open_directory();
                }
                if tool(ui, &tb_save, &format!("{menu_save}  ({sct_save})"), true) {
                    self.save_project();
                }
                ui.separator();
                if tool(
                    ui,
                    crate::ui::theme::icon::UNDO,
                    &format!("{menu_undo}  ({sct_undo})"),
                    can_undo,
                ) {
                    self.undo();
                }
                if tool(
                    ui,
                    crate::ui::theme::icon::REDO,
                    &format!("{menu_redo}  ({sct_redo})"),
                    can_redo,
                ) {
                    self.redo();
                }
                ui.separator();
                if tool(ui, &tb_validate, &menu_validate, true) {
                    self.run_full_validation(&ui.ctx().clone());
                }
                if tool(ui, &tb_preview, &menu_preview, true) {
                    self.open_preview();
                }
                if tool(ui, &tb_export, &menu_export, has_root) {
                    self.export_distribution(&ui.ctx().clone());
                }
            });
            ui.add_space(2.0);
        });
    }

    /// Open the interface-translation editor (Tools menu), translating from
    /// English into the language matching the configured country flag.
    pub(crate) fn open_translation_editor(&mut self) {
        self.trans_source_lang = "eng".to_string();
        if self.trans_country.is_empty() {
            self.trans_country = self.config.country.clone();
        }
        // Keep the target language consistent with the country flag.
        let ui_locale = self.i18n.current_locale().to_string();
        let langs = self.country_languages.languages_for(&self.trans_country);
        self.trans_target_lang = if langs.contains(&ui_locale) {
            ui_locale
        } else if let Some(first) = langs.first() {
            first.clone()
        } else {
            ui_locale
        };
        self.load_translation_entries();
        self.show_translation = true;
    }

    /// Open the user manual (PDF) for the current language with the system
    /// viewer. Looks next to the executable and in the usual `Manuals/`
    /// folders of a source checkout; falls back to the English manual, then
    /// to the About window when no manual ships with this build.
    pub(crate) fn open_manual(&mut self) {
        match crate::manual::find_manual(self.i18n.current_locale()) {
            Some(path) => crate::fonts::open_path(&path),
            None => {
                let msg = self.i18n.t("msg-manual-missing");
                self.notify_warn(msg);
                self.show_about = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal pointer harness around the menu bar.
    struct Mouse {
        ctx: egui::Context,
        time: f64,
    }

    impl Mouse {
        fn new() -> Self {
            let ctx = egui::Context::default();
            ctx.all_styles_mut(|s| s.debug.show_interactive_widgets = true);
            Self { ctx, time: 0.0 }
        }

        fn frame(&mut self, app: &mut XimodApp, events: Vec<egui::Event>) {
            self.time += 1.0 / 60.0;
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(900.0, 700.0))),
                time: Some(self.time),
                events,
                ..Default::default()
            };
            let _ = self.ctx.run(input, |ctx| {
                app.render_menu_bar(ctx);
                egui::CentralPanel::default().show(ctx, |_ui| {});
            });
        }

        fn find(&self, text: &str) -> Option<egui::Pos2> {
            self.ctx.viewport(|v| {
                let w = &v.prev_pass.widgets;
                w.layers()
                    .flat_map(|(_, rs)| rs.iter())
                    .find(|r| w.info(r.id).and_then(|i| i.label.as_deref()) == Some(text))
                    .map(|r| r.rect.center())
            })
        }

        fn click(&mut self, app: &mut XimodApp, pos: egui::Pos2) {
            self.frame(app, vec![egui::Event::PointerMoved(pos)]);
            for pressed in [true, false] {
                self.frame(
                    app,
                    vec![egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: Default::default(),
                    }],
                );
            }
            self.frame(app, vec![]);
        }
    }

    /// A menu opened by sliding along the bar closes when the pointer leaves;
    /// a menu opened by a click stays until something is clicked.
    #[test]
    fn hover_opened_menu_closes_when_the_pointer_leaves() {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let mut m = Mouse::new();
        m.frame(&mut app, vec![]);
        m.frame(&mut app, vec![]);
        let file = m.find("File").expect("File menu title");
        let help = m.find("Help").expect("Help menu title");
        let far = egui::pos2(600.0, 500.0);

        // Click File: open, and it survives the pointer leaving.
        m.click(&mut app, file);
        assert!(m.find("Open Folder…").is_some(), "File menu open after click");
        m.frame(&mut app, vec![egui::Event::PointerMoved(far)]);
        m.frame(&mut app, vec![]);
        assert!(m.find("Open Folder…").is_some(), "click-opened menu stays open");

        // Slide to Help: its menu opens by hover…
        m.frame(&mut app, vec![egui::Event::PointerMoved(help)]);
        m.frame(&mut app, vec![]);
        m.frame(&mut app, vec![]);
        assert!(m.find("About").is_some(), "Help menu opened by hover");
        // …and closes once the pointer is outside the bar and the menu.
        m.frame(&mut app, vec![egui::Event::PointerMoved(far)]);
        m.frame(&mut app, vec![]);
        m.frame(&mut app, vec![]);
        assert!(m.find("About").is_none(), "hover-opened menu closed");
        assert!(m.find("Open Folder…").is_none());
    }
}
