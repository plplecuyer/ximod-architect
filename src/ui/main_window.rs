//! Main window implementation
//!
//! Primary UI component containing all tabs and functionality.

use crate::config::{AppConfig, Theme};
use crate::i18n::I18n;
use crate::models::*;
use crate::ui::components::*;
use crate::ui::docs::{CloseScope, DocState};
use crate::ui::selection::Selection;

use eframe::egui::{self, RichText, Vec2};
use fluent::FluentArgs;
use std::path::PathBuf;

/// Main application tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Info,
    Steps,
    RequiredInstalls,
    ConditionalInstalls,
}

/// What a project-relative file dialog should ask for.
#[derive(Clone, Copy)]
pub(crate) enum PickWhat {
    Files,
    Folder,
    Image,
}

/// Autocompletion candidates derived from the whole project. Rebuilt only
/// when `project_revision` changes instead of on every frame (four full
/// traversals of the model per frame, with quadratic de-duplication, used to
/// be the single biggest per-frame cost of the editor).
#[derive(Default)]
pub struct AutocompleteCache {
    revision: Option<u64>,
    pub flags: std::rc::Rc<Vec<String>>,
    pub flag_values: std::rc::Rc<Vec<String>>,
    pub dep_names: std::rc::Rc<Vec<String>>,
    /// Names tested by file dependencies (`Skyrim.esm`…).
    pub file_names: std::rc::Rc<Vec<String>>,
}

/// A pending action awaiting user confirmation.
#[derive(Debug, Clone)]
pub(crate) enum ConfirmAction {
    /// Delete an installation step (destructive: removes its groups/plugins).
    DeleteStep(usize),
    /// Save a project that did not pass validation. Carries the ready-to-display
    /// warning text (the list of problems) so the dialog can show it verbatim.
    SaveAnyway(String),
}

/// Application state
pub struct XimodApp {
    // Data
    pub ximod: Ximod,
    pub root_directory: Option<PathBuf>,
    /// Dynamically-loaded games & their category lists (from Categories.json).
    pub games: crate::games::GamesData,

    // Configuration
    pub config: AppConfig,
    pub i18n: I18n,

    // UI State
    pub current_tab: Tab,
    /// Selected step / group / option / conditional set (see `ui/selection.rs`).
    pub selection: Selection,

    /// Edit buffer for the "same destination for a whole group" field.
    pub group_dest_buf: String,
    /// Edit buffer for the "same destination for a whole page/step" field.
    pub page_dest_buf: String,
    /// Paging offset (in pages of 8) for the conditional-install tabs strip.
    pub cond_tab_page: usize,
    pub current_file_index: Option<usize>,
    pub current_flag_index: Option<usize>,
    pub current_dependency_index: Option<usize>,
    pub current_cond_file_index: Option<usize>,
    pub current_req_file_index: Option<usize>,
    /// Selected pattern / dependency inside a plugin's "Plugin dependencies" panel.
    pub current_plugin_pattern_index: Option<usize>,
    pub current_plugin_dep_index: Option<usize>,
    /// Selected dependency inside a step's "Visibility conditions" panel.
    pub current_visibility_dep_index: Option<usize>,
    /// Selected row of the mod requirements (`moduleDependencies`) editor.
    pub current_module_dep_index: Option<usize>,
    /// Constructs of the active document's `ModuleConfig.xml` the model
    /// cannot keep (see `xml::fidelity`), reported in the Problems panel.
    pub import_report: Vec<crate::xml::fidelity::Unmodelled>,

    // Dialogs
    pub show_settings: bool,
    /// First launch: the Settings window opened by itself must stay above the
    /// main window, which is created right after it and would otherwise
    /// cover it. Cleared once the user saves the first-start settings.
    pub settings_on_top: bool,
    /// Frames during which the Settings window is still asked to take the
    /// focus after opening (the main window's own activation comes later and
    /// would steal it).
    pub settings_focus_frames: u8,
    /// Same for the country picker, which opens above the Settings window.
    pub flag_picker_focus_frames: u8,
    /// Pending fold command of the project tree, with the frames left to
    /// apply it (see `render_project_tree`).
    pub tree_fold: Option<(crate::ui::tree::Fold, u8)>,
    /// Keyboard cursor of the project tree (↑ ↓ move it, Enter selects).
    pub tree_cursor: Option<crate::ui::tree::NavKey>,
    /// Row to scroll into view on the next frame.
    pub tree_scroll_to: Option<crate::ui::tree::NavKey>,
    /// Id of the menu bar's ui (keys egui's open-menu state).
    pub menu_bar_id: Option<egui::Id>,
    /// The open menu (its root id) and whether hovering opened it.
    pub menu_open_by_hover: Option<(egui::Id, bool)>,
    pub show_about: bool,
    pub show_script_dialog: bool,
    pub editing_pre_script: bool,
    pub script_content: String,

    // Confirmation dialog state
    pub show_confirm: bool,
    pub confirm_action: Option<ConfirmAction>,

    // Translation editor state
    pub show_translation: bool,
    pub trans_source_lang: String,
    pub trans_target_lang: String,
    pub trans_entries: Vec<crate::ui::translation::TransEntry>,
    /// Country whose languages are offered for translation.
    pub trans_country: String,
    /// Font (relative to assets/fonts) used to preview the target language.
    pub trans_font: String,
    /// Country endonym written in the target language.
    pub trans_endonym: String,
    /// Endonym of the target language itself (e.g. "Français" for `fra`).
    pub trans_lang_endonym: String,
    /// Name of the translator, recorded in the translated file.
    pub trans_author: String,
    /// Baseline values captured when the card is (re)loaded, so a save only
    /// propagates the font / endonyms to the reference JSON when the user has
    /// actually edited them — never merely because a file header differed.
    pub trans_font_loaded: String,
    pub trans_endonym_loaded: String,
    pub trans_lang_endonym_loaded: String,
    /// True when the country endonym came from an authoritative source (the
    /// translation's countryEndonyms.tsv or an exact Countries.json entry),
    /// rather than the French/English fallback. Only authoritative values are
    /// propagated back to Countries.json on save.
    pub trans_endonym_authoritative: bool,
    /// Transient message shown in the translation editor (errors, hints).
    pub trans_message: String,
    /// Keyboard-highlighted row in the translation table, and the number of rows
    /// that fit (measured last frame, so Page Up/Down follow window resizing).
    pub trans_cursor: usize,
    pub trans_visible: usize,

    /// Free-window placement (runtime): ids whose initial on-open geometry has
    /// already been decided this open-session, and the stable position/size
    /// handed to the viewport builder for each (kept constant while open so egui
    /// does not fight the user dragging or resizing). The live geometry is
    /// sampled into the config each frame; the config is what persists.
    pub win_initialized: std::collections::HashSet<String>,
    pub win_pos: std::collections::HashMap<String, egui::Pos2>,
    pub win_size: std::collections::HashMap<String, (f32, f32)>,

    // Country / flag selection (settings)
    pub temp_country: String,
    pub countries: crate::data::CountriesData,
    pub country_languages: crate::data::CountryLanguagesData,
    pub country_names: crate::data::CountryNamesData,
    pub show_flag_picker: bool,
    /// Which window the flag picker is currently serving.
    pub flag_target: crate::ui::flag_picker::FlagTarget,
    pub flag_filter: String,
    /// Keyboard state for the flag picker: highlighted cell, and the scroll
    /// offset / viewport height measured last frame (used to page and to keep the
    /// cursor visible in the virtualized grid).
    pub flag_cursor: usize,
    pub flag_scroll_offset: f32,
    pub flag_viewport_h: f32,
    /// Relative paths of the fonts currently installed in egui, so the atlas is
    /// rebuilt only when the required set actually changes.
    pub loaded_fonts: Vec<String>,
    /// Font currently registered under the preview family (translation editor).
    pub loaded_preview_font: String,

    // FOMOD installer preview state
    pub show_preview: bool,
    pub preview: crate::ui::preview::PreviewState,

    // FOMOD validation report (schema + project checks)
    // --- V2 Lot B: reusable templates ---
    pub show_templates: bool,
    pub templates: Vec<crate::models::templates::Template>,
    pub template_name_buf: String,
    // --- V2 Lot C: FOMOD comparison ---
    pub show_compare: bool,
    pub compare_result: Option<crate::models::compare::ProjectDiff>,
    pub compare_other_name: String,
    // --- V2 Lot D: visual condition editor ---
    pub condition_editor: crate::ui::condition_editor::ConditionEditor,
    /// "Translate a FOMOD" window (see `ui/fomod_translation.rs`).
    pub fomod_trans: crate::ui::fomod_translation::FomodTranslationState,
    /// "Project strings" window (see `ui/project_strings.rs`).
    pub project_strings: crate::ui::project_strings::ProjectStringsState,
    pub validation_issues: Vec<crate::ui::problems::Issue>,
    /// Whether the bottom "Problems" panel is shown / expanded.
    pub show_problems: bool,
    pub problems_expanded: bool,

    // Country/language database explorer
    pub show_properties: bool,
    pub properties: crate::ui::properties::PropertiesState,

    // XML editor state
    pub show_xml_editor: bool,
    pub xml_editor_target: crate::ui::xml_editor::XmlTarget,
    pub xml_editor_content: String,
    pub xml_editor_editing: bool,
    pub xml_editor_error: Option<String>,
    /// Per visual row: the line number, or None for wrapped continuation rows.
    /// Rebuilt from the text field's real layout each frame (used one frame later).
    pub xml_editor_gutter: Vec<Option<usize>>,

    // Settings dialog state
    pub settings_tab: SettingsTab,
    pub settings_focus: usize, // Current focused control index
    pub temp_locale: String,
    pub temp_theme: Theme,
    pub temp_font_size: f32,
    pub temp_replace_newlines: bool,
    pub temp_max_recent_files: usize,
    pub temp_window_width: f32,
    pub temp_window_height: f32,
    pub temp_check_updates: bool,
    pub temp_backup_count: usize,
    pub temp_autosave_minutes: u32,
    pub temp_auto_masters: bool,

    // ---- Update check (query GitHub Releases on startup, once per day) ----
    /// Receiver for the in-flight background check, if one is running.
    pub update_rx: Option<std::sync::mpsc::Receiver<crate::update::UpdateCheck>>,
    /// Newer version to advertise in the banner; `None` = no banner shown.
    pub update_available: Option<String>,
    /// Whether the in-flight check was started manually (Help → Check for
    /// updates). A manual check reports "up to date" and errors; the automatic
    /// startup check stays silent on those outcomes.
    pub update_manual: bool,
    /// Set once the automatic startup check has been considered this session.
    pub update_started: bool,

    // Screen info for window positioning. Only consumed by main.rs today;
    // kept on the app for the window-placement work planned in lot C1.
    #[allow(dead_code)]
    pub screen_info: crate::ScreenInfo,

    // Flag to apply theme on first frame
    pub theme_applied: bool,
    /// Theme currently installed in the context (see `update`).
    pub applied_theme: Option<Theme>,

    // Font size currently pushed into egui's text styles. Used to apply the
    // size only when it actually changes (live preview while the Settings
    // dialog is open, saved value otherwise). Sentinel < 0 forces a first apply.
    pub applied_font_size: f32,

    // Status
    pub status_message: String,
    pub project_modified: bool,
    /// Bumped on every edit of the active model. Lets per-frame consumers
    /// (autocompletion lists, condition analysis, …) cache their result and
    /// recompute only when the project actually changed. Also bumped when the
    /// active document is swapped, since the model it refers to changes.
    pub project_revision: u64,
    /// See [`AutocompleteCache`].
    pub autocomplete: AutocompleteCache,
    /// Undo / redo of the active document (see `ui/history.rs`).
    pub history: crate::ui::history::History,
    /// Inputs of the last `sync_fonts` run (locale, country, preview font,
    /// "all scripts" windows open), so the wanted-font set is rebuilt only
    /// when one of them changes.
    pub fonts_key: Option<(String, String, String, bool, u64, u64)>,
    /// Images re-encoded in place since the last frame: egui's image cache is
    /// keyed by URI, so the old pixels would stay on screen until restart
    /// unless the entry is evicted (done at the top of `update`).
    pub images_to_forget: Vec<PathBuf>,
    /// Distribution export running on a worker thread (see `ui/jobs.rs`).
    pub export_job: Option<crate::ui::jobs::ExportJob>,
    /// Last title sent to the OS window (see `sync_window_title`).
    pub window_title: String,
    /// Transient notifications (see `ui/toasts.rs`).
    pub toasts: crate::ui::toasts::Toasts,
    /// `ctx.input(|i| i.time)` at the start of the current frame, so that
    /// `notify_*` can time-stamp toasts without a `Context` at hand.
    pub frame_time: f64,
    /// Disk-bound validation checks in flight: receiver + the project
    /// revision they were started from.
    pub validation_rx: Option<crate::ui::jobs::PendingValidation>,
    /// Install sizes measured by the last validation (see `size_report`,
    /// which also checks they belong to the current root).
    pub sizes: Option<crate::models::simulate::SizeReport>,
    /// "Nexus description" window (see `ui/dialogs/nexus_desc.rs`).
    pub nexus_desc: crate::ui::dialogs::nexus_desc::NexusDescState,

    // Multi-FOMOD documents (one tab each). `self.ximod` / `root_directory` /
    // `project_modified` and the `current_*` indices are the ACTIVE working copy;
    // `docs[active_doc]` mirrors it (synced on tab switch / close). There is
    // always at least one document.
    pub docs: Vec<DocState>,
    pub active_doc: usize,
    /// Whether the crash-recovery marker from a previous run was looked up.
    pub recovery_checked: bool,
    /// Frame time of the last periodic autosave (see `crash_guard::flush_now`).
    pub autosave_last: f64,
    /// "Restore a backup" dialog (see `ui/dialogs/backups.rs`).
    pub show_backups: bool,
    pub backups: Vec<crate::ui::dialogs::backups::BackupRow>,
    /// First click on "Delete all backups" arms the button; the second deletes.
    pub backups_delete_armed: bool,
    /// "Plugin report" dialog (see `ui/dialogs/plugins.rs`).
    pub show_plugin_report: bool,
    pub plugin_report: Vec<crate::ui::dialogs::plugins::PluginRow>,
    /// "Archive contents" dialog (see `ui/dialogs/archive_view.rs`).
    pub show_archive_view: bool,
    pub archive_view: crate::ui::dialogs::archive_view::ArchiveViewState,
    /// Unsaved-changes prompt shown when closing XIMOD.
    pub show_exit_prompt: bool,
    /// Set once the user chose Save/Don't-save, so the next close is allowed.
    pub exit_confirmed: bool,
    /// A graceful close was requested (menu Exit / Ctrl+Q); handled in `update`.
    pub request_close: bool,
    /// A pending close of a modified document awaiting confirmation.
    pub close_prompt: Option<CloseScope>,

    // Temporary edit values
    pub temp_flag_name: String,
    pub temp_flag_value: String,
    pub temp_dep_type: String,
    pub temp_dep_name: String,
    pub temp_dep_value: String,
    /// Temporary edit values for a plugin's "Plugin dependencies" panel.
    pub temp_pdep_type: String,
    pub temp_pdep_name: String,
    pub temp_pdep_value: String,
    /// Temporary edit values for a step's "Visibility conditions" panel.
    pub temp_vdep_type: String,
    pub temp_vdep_name: String,
    pub temp_vdep_value: String,
    /// Temporary edit values for the "Mod requirements" panel (Info tab).
    pub temp_mdep_type: String,
    pub temp_mdep_name: String,
    pub temp_mdep_value: String,
}

/// Settings dialog tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    General,
    RecentFiles,
}

/// Data read from disk before the application window exists: the
/// configuration and the JSON datasets. Loading them is the slow part of
/// start-up, so `main` runs [`Preloaded::load`] on a worker thread while the
/// splash screen is displayed and hands the result to [`XimodApp::new`].
/// (The i18n bundles are not included: they are not `Send`, and parsing two
/// `.ftl` files is quick.)
pub struct Preloaded {
    pub config: AppConfig,
    pub games: crate::games::GamesData,
    pub countries: crate::data::CountriesData,
    pub country_languages: crate::data::CountryLanguagesData,
    pub country_names: crate::data::CountryNamesData,
}

impl Preloaded {
    /// Read everything from disk (never fails: missing files give empty
    /// datasets, as the individual loaders do).
    pub fn load() -> Self {
        Self::with_config(AppConfig::load().unwrap_or_default())
    }

    /// Same, for a configuration already loaded by the caller.
    pub fn with_config(config: AppConfig) -> Self {
        Self {
            config,
            games: crate::games::GamesData::load(),
            countries: crate::data::CountriesData::load(),
            country_languages: crate::data::CountryLanguagesData::load(),
            country_names: crate::data::CountryNamesData::load(),
        }
    }
}

impl Default for XimodApp {
    fn default() -> Self {
        Self::from_preloaded(Preloaded::load())
    }
}

impl XimodApp {
    /// Build the application state from preloaded data.
    pub fn from_preloaded(data: Preloaded) -> Self {
        let Preloaded {
            mut config,
            games,
            countries,
            country_languages,
            country_names,
        } = data;
        // Migrate any legacy ISO 639-1 locale (e.g. "fr") to ISO 639-3 ("fra").
        config.locale = crate::i18n::normalize_locale(&config.locale);

        // First launch (FirstStart=0): start in English and open the settings
        // window so the user can pick their country and language. The flag is
        // set to 1 when they save.
        let first_launch = !config.first_start_done;
        if first_launch {
            config.locale = "eng".to_string();
        }

        let mut i18n = I18n::new();
        i18n.set_locale(&config.locale);

        // config.locale is now ISO 639-3; use it directly.
        let temp_locale = config.locale.clone();
        // Cloned before `config` is moved into the struct below.
        let temp_country = config.country.clone();

        Self {
            ximod: Ximod::default(),
            root_directory: None,
            games,
            temp_locale,
            temp_theme: config.theme,
            temp_font_size: config.font_size,
            temp_replace_newlines: config.replace_newlines,
            temp_max_recent_files: config.max_recent_files,
            temp_window_width: config.window_width,
            temp_window_height: config.window_height,
            temp_check_updates: config.check_updates,
            temp_backup_count: config.backup_count,
            temp_autosave_minutes: config.autosave_minutes,
            temp_auto_masters: config.auto_masters,
            update_rx: None,
            update_available: None,
            update_manual: false,
            update_started: false,
            config,
            i18n,
            current_tab: Tab::Info,
            selection: Selection::default(),
            group_dest_buf: String::new(),
            cond_tab_page: 0,
            page_dest_buf: String::new(),
            current_file_index: None,
            current_flag_index: None,
            current_dependency_index: None,
            current_cond_file_index: None,
            current_req_file_index: None,
            current_plugin_pattern_index: None,
            current_plugin_dep_index: None,
            current_visibility_dep_index: None,
            current_module_dep_index: None,
            import_report: Vec::new(),
            show_settings: first_launch,
            settings_on_top: first_launch,
            settings_focus_frames: if first_launch { 30 } else { 0 },
            flag_picker_focus_frames: 0,
            tree_fold: None,
            tree_cursor: None,
            tree_scroll_to: None,
            menu_bar_id: None,
            menu_open_by_hover: None,
            show_about: false,
            show_script_dialog: false,
            editing_pre_script: true,
            script_content: String::new(),
            show_confirm: false,
            confirm_action: None,
            show_translation: false,
            trans_source_lang: "eng".to_string(),
            trans_target_lang: "eng".to_string(),
            trans_entries: Vec::new(),
            trans_country: String::new(),
            trans_font: String::new(),
            trans_endonym: String::new(),
            trans_lang_endonym: String::new(),
            trans_author: String::new(),
            trans_font_loaded: String::new(),
            trans_endonym_loaded: String::new(),
            trans_lang_endonym_loaded: String::new(),
            trans_endonym_authoritative: false,
            win_initialized: std::collections::HashSet::new(),
            win_pos: std::collections::HashMap::new(),
            win_size: std::collections::HashMap::new(),
            trans_message: String::new(),
            trans_cursor: 0,
            trans_visible: 0,
            temp_country,
            countries,
            country_languages,
            country_names,
            show_flag_picker: false,
            flag_target: crate::ui::flag_picker::FlagTarget::Settings,
            flag_filter: String::new(),
            flag_cursor: 0,
            flag_scroll_offset: 0.0,
            flag_viewport_h: 0.0,
            loaded_fonts: Vec::new(),
            loaded_preview_font: String::new(),
            show_preview: false,
            preview: crate::ui::preview::PreviewState::default(),
            show_templates: false,
            templates: Vec::new(),
            template_name_buf: String::new(),
            show_compare: false,
            compare_result: None,
            compare_other_name: String::new(),
            condition_editor: crate::ui::condition_editor::ConditionEditor::new(),
            fomod_trans: Default::default(),
            project_strings: Default::default(),
            validation_issues: Vec::new(),
            show_problems: false,
            problems_expanded: true,
            show_properties: false,
            properties: crate::ui::properties::PropertiesState::default(),
            show_xml_editor: false,
            xml_editor_target: crate::ui::xml_editor::XmlTarget::InfoXml,
            xml_editor_content: String::new(),
            xml_editor_editing: false,
            xml_editor_error: None,
            xml_editor_gutter: Vec::new(),
            settings_tab: SettingsTab::General,
            settings_focus: 0,
            screen_info: crate::ScreenInfo::default(),
            theme_applied: false,
            applied_theme: None,
            applied_font_size: -1.0,
            status_message: String::new(),
            project_modified: false,
            project_revision: 0,
            autocomplete: AutocompleteCache::default(),
            history: Default::default(),
            fonts_key: None,
            images_to_forget: Vec::new(),
            export_job: None,
            window_title: String::new(),
            toasts: crate::ui::toasts::Toasts::default(),
            frame_time: 0.0,
            validation_rx: None,
            sizes: None,
            nexus_desc: Default::default(),
            docs: Vec::new(),
            active_doc: 0,
            recovery_checked: false,
            autosave_last: 0.0,
            show_backups: false,
            backups: Vec::new(),
            backups_delete_armed: false,
            show_plugin_report: false,
            plugin_report: Vec::new(),
            show_archive_view: false,
            archive_view: Default::default(),
            show_exit_prompt: false,
            exit_confirmed: false,
            request_close: false,
            close_prompt: None,
            temp_flag_name: String::new(),
            temp_flag_value: String::new(),
            temp_dep_type: "flag".to_string(),
            temp_dep_name: String::new(),
            temp_dep_value: String::new(),
            temp_pdep_type: "file".to_string(),
            temp_pdep_name: String::new(),
            temp_pdep_value: String::new(),
            temp_vdep_type: "file".to_string(),
            temp_vdep_name: String::new(),
            temp_vdep_value: String::new(),
            temp_mdep_type: "file".to_string(),
            temp_mdep_name: String::new(),
            temp_mdep_value: String::new(),
        }
    }
}

impl XimodApp {
    /// Create a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>, data: Preloaded, screen_info: crate::ScreenInfo) -> Self {
        let mut app = Self::from_preloaded(data);
        app.screen_info = screen_info;

        // Install image loaders so ImageDisplay can render header/plugin images
        // from disk via "file://" URIs (requires egui_extras "image" + "file").
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Apply spacing modifications
        cc.egui_ctx
            .all_styles_mut(|style| style.spacing.item_spacing = Vec2::new(8.0, 6.0));

        // Theme will be applied on first frame in update() to ensure it takes effect
        app.theme_applied = false;

        app.status_message = app.i18n.t("status-ready");

        // Load the fonts for the current interface language / country up front so
        // the very first frame already renders with the right glyphs. (sync_fonts
        // otherwise runs at the end of update(), which would leave frame 1 on the
        // default fonts for a non-Latin interface language.)
        app.sync_fonts(&cc.egui_ctx);
        app
    }

    /// Push a font size into egui's text styles so the "Font size" setting
    /// actually changes the rendered text. Sizes for the other text styles are
    /// derived proportionally from the base (body) size.
    pub(crate) fn apply_font_size_value(&self, ctx: &egui::Context, base: f32) {
        use egui::{FontFamily, FontId, TextStyle};
        let base = base.clamp(8.0, 24.0);
        // Both the dark and the light style, so a system theme switch keeps
        // the chosen size.
        ctx.all_styles_mut(|style| {
            style.text_styles = [
                (
                    TextStyle::Small,
                    FontId::new((base * 0.85).round(), FontFamily::Proportional),
                ),
                (TextStyle::Body, FontId::new(base, FontFamily::Proportional)),
                (TextStyle::Button, FontId::new(base, FontFamily::Proportional)),
                (
                    TextStyle::Monospace,
                    FontId::new((base * 0.95).round(), FontFamily::Monospace),
                ),
                (
                    TextStyle::Heading,
                    FontId::new((base * 1.3).round(), FontFamily::Proportional),
                ),
            ]
            .into();
        });
    }

    /// Autocompletion candidates for the current project revision.
    pub(crate) fn autocomplete(&mut self) -> &AutocompleteCache {
        if self.autocomplete.revision != Some(self.project_revision) {
            self.autocomplete.flags = std::rc::Rc::new(self.ximod.get_all_flags());
            self.autocomplete.flag_values = std::rc::Rc::new(self.ximod.get_all_flag_values());
            self.autocomplete.dep_names = std::rc::Rc::new(self.ximod.get_all_dependency_names());
            self.autocomplete.file_names = std::rc::Rc::new(self.ximod.get_all_file_names());
            self.autocomplete.revision = Some(self.project_revision);
        }
        &self.autocomplete
    }

    /// Record that the active project changed: sets the dirty flag and bumps
    /// the revision counter that invalidates per-frame caches.
    pub(crate) fn mark_modified(&mut self) {
        self.project_modified = true;
        self.project_revision = self.project_revision.wrapping_add(1);
        self.history.record_edit(self.frame_time);
    }

    /// Undo the last edit burst of the active document.
    pub(crate) fn undo(&mut self) {
        if let Some(previous) = self.history.undo(&self.ximod) {
            self.ximod = previous;
            self.project_modified = true;
            self.project_revision = self.project_revision.wrapping_add(1);
            self.clamp_selection();
        }
    }

    /// Redo the last undone edit burst.
    pub(crate) fn redo(&mut self) {
        if let Some(next) = self.history.redo(&self.ximod) {
            self.ximod = next;
            self.project_modified = true;
            self.project_revision = self.project_revision.wrapping_add(1);
            self.clamp_selection();
        }
    }

    /// Keep every selection index inside the current collections (after an
    /// undo/redo the model may have fewer steps, groups or options).
    pub(crate) fn clamp_selection(&mut self) {
        let n_steps = self.ximod.steps.len();
        if self.selection.step.is_some_and(|i| i >= n_steps) {
            self.select_step(if n_steps == 0 { None } else { Some(n_steps - 1) });
            return;
        }
        if let Some(si) = self.selection.step {
            let n_groups = self.ximod.steps[si].plugin_groups.len();
            if self.selection.group.is_some_and(|g| g >= n_groups) {
                self.select_group(None);
                return;
            }
            if let Some(gi) = self.selection.group {
                let n_plugins = self.ximod.steps[si].plugin_groups[gi].plugins.len();
                if self.selection.plugin.is_some_and(|p| p >= n_plugins) {
                    self.select_plugin(None);
                    return;
                }
                if let Some(pi) = self.selection.plugin {
                    let plugin = &self.ximod.steps[si].plugin_groups[gi].plugins[pi];
                    if self
                        .current_flag_index
                        .is_some_and(|i| i >= plugin.condition_flags.len())
                    {
                        self.current_flag_index = None;
                    }
                    if self.current_file_index.is_some_and(|i| i >= plugin.files.len()) {
                        self.current_file_index = None;
                    }
                    if self
                        .current_plugin_pattern_index
                        .is_some_and(|i| i >= plugin.dependency_patterns.len())
                    {
                        self.current_plugin_pattern_index = None;
                        self.current_plugin_dep_index = None;
                    }
                }
            }
        }
        let n_cond = self.ximod.conditional_files.len();
        if self.selection.cond_pattern.is_some_and(|i| i >= n_cond) {
            self.select_cond_pattern(if n_cond == 0 { None } else { Some(n_cond - 1) });
        }
        if self
            .current_req_file_index
            .is_some_and(|i| i >= self.ximod.required_files.len())
        {
            self.current_req_file_index = None;
        }
    }

    /// Forget every selection index without touching the current tab. Called
    /// whenever the model is replaced wholesale (load, XML editor "Apply",
    /// merge) so that no index can outlive the collection it pointed into.
    pub(crate) fn reset_selection(&mut self) {
        self.select_step(if self.ximod.steps.is_empty() { None } else { Some(0) });
        self.select_cond_pattern(None);
        self.current_req_file_index = None;
        self.current_module_dep_index = None;
    }

    // Selection helpers. Every index below a given level is cleared when that
    // level changes: a flag or file index that belonged to plugin A must never
    // be applied to plugin B. With `panic = "abort"` in release builds, a stale
    // index reaching `Vec::remove` used to close the application outright.

    /// Select a step and clear every selection that lives below it.
    pub(crate) fn select_step(&mut self, si: Option<usize>) {
        self.selection.step = si;
        self.current_visibility_dep_index = None;
        self.select_group(None);
    }

    /// Select a group within the current step and clear the plugin selection.
    pub(crate) fn select_group(&mut self, gi: Option<usize>) {
        self.selection.group = gi;
        self.select_plugin(None);
    }

    /// Select a plugin within the current group and clear its sub-selections.
    pub(crate) fn select_plugin(&mut self, pi: Option<usize>) {
        self.selection.plugin = pi;
        self.current_file_index = None;
        self.current_flag_index = None;
        self.current_plugin_pattern_index = None;
        self.current_plugin_dep_index = None;
    }

    /// Select a conditional-install pattern and clear its sub-selections.
    pub(crate) fn select_cond_pattern(&mut self, ci: Option<usize>) {
        self.selection.cond_pattern = ci;
        self.current_dependency_index = None;
        self.current_cond_file_index = None;
    }

    /// Install the fonts required to display the current interface language and
    /// every language offered for the selected country.
    ///
    /// Cheap when nothing changed: the wanted set is compared to the installed
    /// one and `set_fonts` (which rebuilds the glyph atlas) runs only on change.
    pub(crate) fn sync_fonts(&mut self, ctx: &egui::Context) {
        // Cheap fingerprint of everything the wanted-font set depends on. When
        // it is unchanged, skip the rebuild entirely: this runs every frame.
        // Every window that lists language or country names in their own
        // script: settings explorer, translation editor, flag picker, FOMOD
        // translation (source/target language combos), Nexus description
        // (language combo).
        let all_scripts = self.show_properties
            || self.show_translation
            || self.show_flag_picker
            || self.fomod_trans.open
            || self.nexus_desc.open;
        let key = (
            self.config.locale.clone(),
            self.temp_country.clone(),
            self.trans_font.clone(),
            all_scripts,
            self.project_revision,
            self.fomod_trans.font_revision(),
        );
        if self.fonts_key.as_ref() == Some(&key) {
            return;
        }
        self.fonts_key = Some(key);

        let mut wanted: Vec<String> = Vec::new();

        // Fonts for the scripts used by the texts of the project itself (and
        // of the translation being edited): a Japanese or Arabic FOMOD must
        // display its names and descriptions whatever the interface language.
        {
            let mut scripts = std::collections::BTreeSet::new();
            self.ximod
                .for_each_text(|t| crate::fonts::fonts_for_text(t, &mut scripts));
            self.fomod_trans
                .for_each_text(|t| crate::fonts::fonts_for_text(t, &mut scripts));
            wanted.extend(scripts);
        }

        // Font of the language currently displayed in the interface.
        if let Some(f) = self.i18n.font_for(&self.config.locale) {
            wanted.push(f.to_string());
        }
        // Fonts of every language listed for the selected country: those are the
        // entries shown in the settings' language drop-down.
        if !self.temp_country.is_empty() {
            for code in self.country_languages.languages_for(&self.temp_country) {
                if let Some(f) = self.i18n.font_for(code) {
                    wanted.push(f.to_string());
                }
            }
        }
        // Font chosen in the translation editor: also previewed in its own
        // family, so the endonym field renders in the target language typeface.
        let preview = self.trans_font.clone();
        if !preview.is_empty() {
            wanted.push(preview.clone());
        }
        // The Properties window (country/language explorer), the Translation
        // editor (language drop-downs) and the flag picker (country endonyms)
        // all show names in every script — Thai, Amharic, Greek, Georgian… —
        // so load every distinct language font while one of them is open. There
        // are only a few dozen after dedup, each appended to the Proportional
        // fallback chain, so egui picks the right glyph per character.
        if all_scripts {
            wanted.extend(self.i18n.languages().distinct_fonts().iter().cloned());
        }
        wanted.sort();
        wanted.dedup();

        if wanted == self.loaded_fonts && preview == self.loaded_preview_font {
            return;
        }
        tracing::info!("Loading {} font(s)", wanted.len());
        ctx.set_fonts(crate::fonts::build_font_definitions_with_preview(
            &wanted,
            if preview.is_empty() {
                None
            } else {
                Some(preview.as_str())
            },
        ));
        self.loaded_fonts = wanted;
        self.loaded_preview_font = preview;
        // set_fonts only takes effect at the start of the next frame; request one
        // now so the new glyphs appear immediately instead of waiting for the
        // next input event (which would otherwise leave a brief tofu flash).
        ctx.request_repaint();
    }

    // Notifications: every user-facing event goes through one of these so it
    // is both kept in the status bar and shown as a toast.

    pub(crate) fn notify(&mut self, level: crate::ui::toasts::ToastLevel, msg: impl Into<String>) {
        let msg = msg.into();
        self.status_message = msg.clone();
        self.toasts.push(level, msg, self.frame_time);
    }

    pub(crate) fn notify_ok(&mut self, msg: impl Into<String>) {
        self.notify(crate::ui::toasts::ToastLevel::Success, msg);
    }

    pub(crate) fn notify_info(&mut self, msg: impl Into<String>) {
        self.notify(crate::ui::toasts::ToastLevel::Info, msg);
    }

    pub(crate) fn notify_warn(&mut self, msg: impl Into<String>) {
        self.notify(crate::ui::toasts::ToastLevel::Warning, msg);
    }

    pub(crate) fn notify_err(&mut self, msg: impl Into<String>) {
        self.notify(crate::ui::toasts::ToastLevel::Error, msg);
    }

    /// Open a file/folder dialog rooted at the project folder and return the
    /// chosen paths relative to it. Paths outside the root are rejected with
    /// a visible warning (they used to be ignored silently), and a missing
    /// root is reported instead of making the button do nothing.
    pub(crate) fn pick_relative(&mut self, what: PickWhat) -> Vec<String> {
        let Some(root) = self.root_directory.clone() else {
            let msg = self.i18n.t("msg-no-root-selected");
            self.notify_err(msg);
            return Vec::new();
        };
        let dialog = rfd::FileDialog::new().set_directory(&root);
        let paths: Vec<PathBuf> = match what {
            PickWhat::Files => dialog.pick_files().unwrap_or_default(),
            PickWhat::Folder => dialog.pick_folder().into_iter().collect(),
            PickWhat::Image => dialog
                .add_filter(self.i18n.t("filter-images"), &["png", "jpg", "jpeg"])
                .pick_file()
                .into_iter()
                .collect(),
        };
        let mut rels = Vec::new();
        let mut outside = 0usize;
        for path in paths {
            match path.strip_prefix(&root) {
                Ok(rel) => rels.push(rel.to_string_lossy().to_string()),
                Err(_) => outside += 1,
            }
        }
        if outside > 0 {
            let msg = self.i18n.t("msg-file-outside-root");
            self.notify_warn(msg);
        }
        rels
    }
}

impl eframe::App for XimodApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_time = ctx.input(|i| i.time);
        self.history.end_burst(self.frame_time, &self.ximod);
        // Theme: applied on the first frame (after eframe initialization), and
        // previewed live while the Settings dialog is open — exactly like the
        // font size below. Cancelling the dialog reverts on the next frame.
        let desired_theme = if self.show_settings {
            self.temp_theme
        } else {
            self.config.theme
        };
        // "System" is pinned to the probed OS theme when the windowing layer
        // cannot report it; the probe is cached, so re-resolving it every
        // frame is cheap and re-applies the theme when the OS switches.
        let pinned = desired_theme == Theme::System && ctx.system_theme().is_none();
        let desired_key = if pinned {
            crate::ui::theme::concrete(ctx, desired_theme)
        } else {
            desired_theme
        };
        if !self.theme_applied || self.applied_theme != Some(desired_key) {
            crate::ui::theme::apply(ctx, desired_theme);
            self.apply_font_size_value(ctx, self.applied_font_size.max(8.0));
            self.applied_theme = Some(desired_key);
            self.theme_applied = true;
        }

        // Live font size: while the Settings dialog is open, preview the value
        // being edited (temp_font_size); otherwise use the saved size. Applied
        // only when it actually changes, so cancelling the dialog automatically
        // reverts the preview to the saved size on the next frame.
        let desired_font = if self.show_settings {
            self.temp_font_size
        } else {
            self.config.font_size
        };
        if (desired_font - self.applied_font_size).abs() > f32::EPSILON {
            self.apply_font_size_value(ctx, desired_font);
            self.applied_font_size = desired_font;
        }

        // Evict images that were rewritten on disk so the new pixels show up.
        for path in self.images_to_forget.drain(..) {
            crate::ui::components::forget_image_uri(ctx, &path);
        }

        // The active document lives in the working copy (`self.ximod`…); its
        // `docs` slot is only refreshed by `commit_active()` when the tab
        // changes, on close and on exit. Committing every frame used to deep
        // clone the whole project 60 times per second.
        self.ensure_doc();

        // Crash protection: keep a throttled copy of every modified document
        // for the panic hook (see crash_guard.rs). The clone only happens
        // every couple of seconds, and only when there is unsaved work.
        if crate::crash_guard::wants_snapshot() {
            let active = self.active_doc;
            let snapshots: Vec<crate::crash_guard::Snapshot> = self
                .docs
                .iter()
                .enumerate()
                .filter_map(|(i, d)| {
                    if i == active {
                        self.project_modified.then(|| crate::crash_guard::Snapshot {
                            root: self.root_directory.clone(),
                            ximod: self.ximod.clone(),
                        })
                    } else {
                        d.modified.then(|| crate::crash_guard::Snapshot {
                            root: d.root_directory.clone(),
                            ximod: d.ximod.clone(),
                        })
                    }
                })
                .collect();
            crate::crash_guard::update(snapshots);
        }

        // Periodic autosave of the modified documents (recovery copies, see
        // crash_guard.rs). The repaint request keeps the timer running while
        // the window is idle.
        let autosave_secs = self.config.autosave_minutes as f64 * 60.0;
        if autosave_secs > 0.0 {
            let elapsed = self.frame_time - self.autosave_last;
            if elapsed >= autosave_secs {
                self.autosave_last = self.frame_time;
                let n = crate::crash_guard::flush_now();
                if n > 0 {
                    tracing::info!("Autosaved {n} recovery cop{}", if n == 1 { "y" } else { "ies" });
                }
                ctx.request_repaint_after(std::time::Duration::from_secs_f64(autosave_secs));
            } else {
                ctx.request_repaint_after(std::time::Duration::from_secs_f64(autosave_secs - elapsed));
            }
        }

        // Tell the user once where the previous run's crash backup went.
        if !self.recovery_checked {
            self.recovery_checked = true;
            if let Some(dir) = crate::crash_guard::take_pending_recovery() {
                self.status_message = self
                    .i18n
                    .t_arg("msg-crash-recovery", "path", &dir.display().to_string());
            }
        }

        // A graceful close requested from the menu / Ctrl+Q.
        if self.request_close {
            self.request_close = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Files dropped onto the window open the corresponding FOMOD(s).
        self.handle_dropped_files(ctx);

        // Update check: kick off the automatic (once-per-day) check on the first
        // frame, then poll the in-flight result every frame.
        if !self.update_started {
            self.update_started = true;
            self.start_update_check(ctx, false);
        }
        self.poll_update_check(ctx);

        // Check if a modal dialog is open. The Settings, About and Script
        // windows are now independent, freely movable OS-level windows, so they
        // no longer block the main window.
        let modal_open = self.show_confirm
            || self.show_exit_prompt
            || self.close_prompt.is_some()
            || self.export_job.is_some()
            || self.show_backups
            || self.show_plugin_report
            || self.show_archive_view
            || (self.show_xml_editor && self.xml_editor_editing);

        self.render_menu_bar(ctx);

        // "New version available" banner, just below the menu bar.
        self.render_update_banner(ctx);

        // Document (FOMOD) tab strip.
        self.render_fomod_tabs(ctx, modal_open);

        // Global keyboard shortcuts (Ctrl+N, Ctrl+S, …) matching the menu.
        self.handle_menu_shortcuts(ctx);

        // Bottom panels first (egui lays out the central panel last): the
        // status bar, then the problems list above it.
        let modified = self.project_modified;
        let modified_text = self.i18n.t("status-modified");
        // Durable information lives in the status bar: the step/option counts.
        // The root folder is not repeated here (it is shown in the Workspace
        // section of the inspector, and the mod name in the window title and
        // the document tab). Events go through the toasts.
        let summary = {
            let mut args = FluentArgs::new();
            args.set("steps", self.ximod.steps.len() as i64);
            args.set("options", self.ximod.plugin_count() as i64);
            self.i18n.t_with_args("status-summary", Some(&args))
        };
        let status = &self.status_message;

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            if modal_open {
                ui.disable();
            }
            status_bar(ui, status, modified, &modified_text, &summary);
        });
        self.toasts.show(ctx);
        self.poll_disk_validation();
        self.render_problems_panel(ctx, modal_open);

        // Left: the project tree. Right: the inspector of the selected node.
        let tree_title = self.i18n.t("tree-title");
        egui::SidePanel::left("project_tree")
            .resizable(true)
            .default_width(280.0)
            .min_width(180.0)
            .show(ctx, |ui| {
                if modal_open {
                    ui.disable();
                }
                ui.add_space(4.0);
                ui.label(RichText::new(tree_title).strong().small());
                ui.separator();
                self.render_project_tree(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Disable all main window controls when modal is open
            if modal_open {
                ui.disable();
            }
            self.render_inspector(ui);
        });

        self.render_settings_dialog(ctx);
        self.render_about_dialog(ctx);
        self.render_script_dialog(ctx);
        self.render_confirm_dialog(ctx);
        self.render_translation_window(ctx);
        self.render_xml_editor(ctx);
        self.render_preview(ctx);
        self.render_export_job(ctx);
        self.render_templates_window(ctx);
        self.render_compare_window(ctx);
        self.render_backups_dialog(ctx);
        self.render_plugin_report(ctx);
        self.render_archive_view(ctx);
        self.render_nexus_desc(ctx);
        self.render_condition_editor(ctx);
        self.render_project_strings(ctx);
        self.render_properties(ctx);
        self.render_flag_picker(ctx);
        self.render_fomod_translation(ctx);
        self.render_exit_prompt(ctx);
        self.render_close_prompt(ctx);

        // Intercept the window's close request: if any FOMOD has unsaved changes,
        // veto the close and ask the user (Yes / No / Cancel).
        if ctx.input(|i| i.viewport().close_requested()) && !self.exit_confirmed {
            self.commit_active();
            if self.docs.iter().any(|d| d.modified) {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_exit_prompt = true;
            }
        }

        // Install the fonts needed by the current state (no-op when the required
        // set has not changed). Run *after* the windows have rendered so that a
        // language switched this frame in the translation editor or a country
        // selected in Properties is picked up immediately, not one frame later.
        self.sync_fonts(ctx);
        self.sync_window_title(ctx);

        // Update window size in config (for saving on exit)
        let screen_rect = ctx.screen_rect();
        self.config.window_width = screen_rect.width();
        self.config.window_height = screen_rect.height();
    }

    /// Called when the application is about to close
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // A clean exit: the autosave copies are not needed (the user saved or
        // chose not to), so the next start must not offer to recover them.
        crate::crash_guard::clear_autosave();
        // Free-window geometry (position + size) is sampled into the config each
        // frame while a window is open, so the last state before closing XIMOD is
        // already there; just persist it.
        match self.config.save() {
            Err(e) => {
                tracing::error!("Failed to save configuration on exit: {}", e);
            }
            _ => {
                tracing::info!("Configuration saved on exit");
            }
        }
    }
}
