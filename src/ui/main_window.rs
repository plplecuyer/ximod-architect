//! Main window implementation
//!
//! Primary UI component containing all tabs and functionality.

use crate::config::{AppConfig, ScriptMacros, Theme};
use crate::i18n::I18n;
use crate::models::*;
use crate::ui::components::*;
use crate::xml;

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

/// One open FOMOD document (one tab). Snapshots the active working copy so a
/// tab can be left and returned to without losing its data or view.
#[derive(Clone)]
pub struct DocState {
    pub ximod: Ximod,
    pub root_directory: Option<PathBuf>,
    pub modified: bool,
    pub tab: Tab,
    pub step: Option<usize>,
    pub group: Option<usize>,
    pub plugin: Option<usize>,
    pub file: Option<usize>,
    pub flag: Option<usize>,
    pub dependency: Option<usize>,
    pub cond_file: Option<usize>,
    pub cond_pattern: Option<usize>,
    pub req_file: Option<usize>,
}

impl DocState {
    /// A pristine blank document (no root, nothing selected).
    fn blank() -> Self {
        Self {
            ximod: Ximod::default(),
            root_directory: None,
            modified: false,
            tab: Tab::Info,
            step: None,
            group: None,
            plugin: None,
            file: None,
            flag: None,
            dependency: None,
            cond_file: None,
            cond_pattern: None,
            req_file: None,
        }
    }
}

/// Which documents a pending close affects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseScope {
    /// Close the active FOMOD only.
    Active,
    /// Close every FOMOD.
    All,
}

/// A pending action awaiting user confirmation.
#[derive(Debug, Clone)]
pub enum ConfirmAction {
    /// Delete an installation step (destructive: removes its groups/plugins).
    DeleteStep(usize),
    /// Save a project that did not pass validation. Carries the ready-to-display
    /// warning text (the list of problems) so the dialog can show it verbatim.
    SaveAnyway(String),
}

/// Keyboard shortcuts of the main menu, so the menu display and the global
/// handler stay in sync.
struct MenuShortcuts {
    new: egui::KeyboardShortcut,
    open: egui::KeyboardShortcut,
    open_file: egui::KeyboardShortcut,
    save: egui::KeyboardShortcut,
    settings: egui::KeyboardShortcut,
    quit: egui::KeyboardShortcut,
    about: egui::KeyboardShortcut,
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
    pub locale_version: u32,  // Incremented on locale change to force menu rebuild

    // UI State
    pub current_tab: Tab,
    pub current_step_index: Option<usize>,
    pub current_group_index: Option<usize>,
    pub current_plugin_index: Option<usize>,
    pub current_file_index: Option<usize>,
    pub current_flag_index: Option<usize>,
    pub current_dependency_index: Option<usize>,
    pub current_cond_file_index: Option<usize>,
    pub current_cond_pattern_index: Option<usize>,
    pub current_req_file_index: Option<usize>,

    // Dialogs
    pub show_settings: bool,
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
    pub show_validation_report: bool,
    pub validation_report: Vec<String>,

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
    pub settings_focus: usize,  // Current focused control index
    pub temp_locale: String,
    pub temp_theme: Theme,
    pub temp_font_size: f32,
    pub temp_replace_newlines: bool,
    pub temp_max_recent_files: usize,
    pub temp_window_width: f32,
    pub temp_window_height: f32,
    
    // Screen info for window positioning
    pub screen_info: crate::ScreenInfo,
    
    // Flag to apply theme on first frame
    pub theme_applied: bool,

    // Font size currently pushed into egui's text styles. Used to apply the
    // size only when it actually changes (live preview while the Settings
    // dialog is open, saved value otherwise). Sentinel < 0 forces a first apply.
    pub applied_font_size: f32,

    // Status
    pub status_message: String,
    pub project_modified: bool,

    // Multi-FOMOD documents (one tab each). `self.ximod` / `root_directory` /
    // `project_modified` and the `current_*` indices are the ACTIVE working copy;
    // `docs[active_doc]` mirrors it (synced on tab switch / close). There is
    // always at least one document.
    pub docs: Vec<DocState>,
    pub active_doc: usize,
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
}

/// Settings dialog tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    General,
    RecentFiles,
}

/// Shorten `text` from the *left* so that it fits within `max_width`, keeping
/// the end visible and prefixing an ellipsis (e.g. "…\MyMod\fomod").
///
/// Sample a free window's current outer position and inner size into the config,
/// keyed by viewport id. Called each frame from inside a viewport closure (with
/// the child context). Takes `&mut AppConfig` so it works both when the closure
/// borrows the whole app and when it only borrows the config field disjointly.
pub(crate) fn record_win_geom(
    config: &mut crate::config::AppConfig,
    ctx: &egui::Context,
    id: &str,
) {
    let (outer, inner) = ctx.input(|i| {
        let vp = i.viewport();
        (vp.outer_rect, vp.inner_rect)
    });
    if let Some(o) = outer {
        config
            .window_positions
            .insert(id.to_string(), (o.min.x, o.min.y));
    }
    if let Some(r) = inner {
        let sz = r.size();
        if sz.x > 1.0 && sz.y > 1.0 {
            config.window_sizes.insert(id.to_string(), (sz.x, sz.y));
        }
    }
}

/// Used for recent-file paths, where the tail (mod name) identifies the entry
/// far better than the head (drive and parent folders).
fn elide_start(ui: &egui::Ui, text: &str, max_width: f32) -> String {
    let font_id = egui::TextStyle::Body.resolve(ui.style());
    let width_of = |s: &str| -> f32 {
        ui.fonts(|f| {
            f.layout_no_wrap(s.to_string(), font_id.clone(), egui::Color32::WHITE)
                .rect
                .width()
        })
    };

    if max_width <= 0.0 || width_of(text) <= max_width {
        return text.to_string();
    }

    let chars: Vec<char> = text.chars().collect();
    // Binary search for the smallest start offset whose "…" + tail still fits.
    let (mut lo, mut hi) = (0usize, chars.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        let candidate: String = std::iter::once('…').chain(chars[mid..].iter().copied()).collect();
        if width_of(&candidate) <= max_width {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    std::iter::once('…').chain(chars[lo..].iter().copied()).collect()
}

impl Default for XimodApp {
    fn default() -> Self {
        let mut config = AppConfig::load().unwrap_or_default();
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
            games: crate::games::GamesData::load(),
            temp_locale,
            temp_theme: config.theme,
            temp_font_size: config.font_size,
            temp_replace_newlines: config.replace_newlines,
            temp_max_recent_files: config.max_recent_files,
            temp_window_width: config.window_width,
            temp_window_height: config.window_height,
            config,
            i18n,
            locale_version: 0,
            current_tab: Tab::Info,
            current_step_index: None,
            current_group_index: None,
            current_plugin_index: None,
            current_file_index: None,
            current_flag_index: None,
            current_dependency_index: None,
            current_cond_file_index: None,
            current_cond_pattern_index: None,
            current_req_file_index: None,
            show_settings: first_launch,
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
            countries: crate::data::CountriesData::load(),
            country_languages: crate::data::CountryLanguagesData::load(),
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
            show_validation_report: false,
            validation_report: Vec::new(),
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
            applied_font_size: -1.0,
            status_message: String::new(),
            project_modified: false,
            docs: Vec::new(),
            active_doc: 0,
            show_exit_prompt: false,
            exit_confirmed: false,
            request_close: false,
            close_prompt: None,
            temp_flag_name: String::new(),
            temp_flag_value: String::new(),
            temp_dep_type: "flag".to_string(),
            temp_dep_name: String::new(),
            temp_dep_value: String::new(),
        }
    }
}

impl XimodApp {
    /// Create a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>, _start_with_splash: bool, screen_info: crate::ScreenInfo) -> Self {
        let mut app = Self::default();
        app.screen_info = screen_info;
        
        // Install image loaders so ImageDisplay can render header/plugin images
        // from disk via "file://" URIs (requires egui_extras "image" + "file").
        egui_extras::install_image_loaders(&cc.egui_ctx);
        
        // Apply spacing modifications
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        cc.egui_ctx.set_style(style);
        
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
    ) -> egui::ViewportBuilder {
        if !self.win_initialized.contains(id) {
            // Size: the saved size if the window was resized before, else the
            // caller's default.
            let win_size = self
                .config
                .window_sizes
                .get(id)
                .map(|&(w, h)| [w, h])
                .unwrap_or(size);
            // Position: the saved position if the window was moved before, else
            // centered on the main window.
            let pos = match self.config.window_positions.get(id) {
                Some(&(x, y)) => egui::pos2(x, y),
                None => {
                    let main = ctx.input(|i| i.viewport().outer_rect).unwrap_or_else(|| {
                        egui::Rect::from_min_size(
                            egui::pos2(80.0, 80.0),
                            egui::vec2(1280.0, 800.0),
                        )
                    });
                    let c = main.center();
                    egui::pos2(c.x - win_size[0] / 2.0, c.y - win_size[1] / 2.0)
                }
            };
            self.win_pos.insert(id.to_string(), pos);
            self.win_size.insert(id.to_string(), (win_size[0], win_size[1]));
            self.win_initialized.insert(id.to_string());
        }
        let pos = self
            .win_pos
            .get(id)
            .copied()
            .unwrap_or_else(|| egui::pos2(80.0, 80.0));
        let sz = self.win_size.get(id).copied().unwrap_or((size[0], size[1]));
        egui::ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([sz.0, sz.1])
            .with_position(pos)
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

    fn apply_theme(&self, ctx: &egui::Context) {
        match self.config.theme {
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::System => {
                // Detect system theme
                match Theme::detect_system_theme() {
                    Theme::Light => ctx.set_visuals(egui::Visuals::light()),
                    _ => ctx.set_visuals(egui::Visuals::dark()),
                }
            }
        }
        // Font size is handled by the per-frame live sync in `update()` (which
        // reflects the Settings preview immediately); nothing to do here.
    }

    /// Push a font size into egui's text styles so the "Font size" setting
    /// actually changes the rendered text. Sizes for the other text styles are
    /// derived proportionally from the base (body) size.
    fn apply_font_size_value(&self, ctx: &egui::Context, base: f32) {
        use egui::{FontFamily, FontId, TextStyle};
        let base = base.clamp(8.0, 24.0);
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Small, FontId::new((base * 0.85).round(), FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(base, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(base, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new((base * 0.95).round(), FontFamily::Monospace)),
            (TextStyle::Heading, FontId::new((base * 1.3).round(), FontFamily::Proportional)),
        ]
        .into();
        ctx.set_style(style);
    }

    /// Start a new blank project. With FOMOD tabs this adds a new tab (replacing
    /// the current one only when it is a pristine blank project), so nothing is
    /// discarded and no confirmation is needed.
    fn new_project(&mut self) {
        self.ensure_doc();
        if !self.active_is_pristine() {
            self.commit_active();
            let clone = self.docs[self.active_doc].clone();
            self.docs.push(clone);
            self.active_doc = self.docs.len() - 1;
        }
        self.ximod = Ximod::default();
        self.root_directory = None;
        self.project_modified = false;
        self.reset_navigation();
        self.commit_active();
        self.status_message = self.i18n.t("status-ready");
    }

    /// Shared by the "New" menu item and the Ctrl+N shortcut.
    fn request_new_project(&mut self) {
        self.new_project();
    }

    // ---- Multi-FOMOD document (tab) helpers ----

    /// Snapshot the active working copy into a `DocState`.
    fn make_doc_snapshot(&self) -> DocState {
        DocState {
            ximod: self.ximod.clone(),
            root_directory: self.root_directory.clone(),
            modified: self.project_modified,
            tab: self.current_tab,
            step: self.current_step_index,
            group: self.current_group_index,
            plugin: self.current_plugin_index,
            file: self.current_file_index,
            flag: self.current_flag_index,
            dependency: self.current_dependency_index,
            cond_file: self.current_cond_file_index,
            cond_pattern: self.current_cond_pattern_index,
            req_file: self.current_req_file_index,
        }
    }

    /// Guarantee at least one document, mirroring the initial working copy.
    fn ensure_doc(&mut self) {
        if self.docs.is_empty() {
            self.docs.push(self.make_doc_snapshot());
            self.active_doc = 0;
        } else if self.active_doc >= self.docs.len() {
            self.active_doc = self.docs.len() - 1;
        }
    }

    /// Copy the active working copy into its `docs` slot.
    fn commit_active(&mut self) {
        self.ensure_doc();
        let snap = self.make_doc_snapshot();
        self.docs[self.active_doc] = snap;
    }

    /// Load `docs[active_doc]` into the active working copy.
    fn checkout_active(&mut self) {
        self.ensure_doc();
        let d = self.docs[self.active_doc].clone();
        self.ximod = d.ximod;
        self.root_directory = d.root_directory;
        self.project_modified = d.modified;
        self.current_tab = d.tab;
        self.current_step_index = d.step;
        self.current_group_index = d.group;
        self.current_plugin_index = d.plugin;
        self.current_file_index = d.file;
        self.current_flag_index = d.flag;
        self.current_dependency_index = d.dependency;
        self.current_cond_file_index = d.cond_file;
        self.current_cond_pattern_index = d.cond_pattern;
        self.current_req_file_index = d.req_file;
    }

    /// Switch the active document to `index`.
    fn switch_doc(&mut self, index: usize) {
        if index >= self.docs.len() || index == self.active_doc {
            return;
        }
        self.commit_active();
        self.active_doc = index;
        self.checkout_active();
    }

    /// True when the active document is an untouched blank project.
    fn active_is_pristine(&self) -> bool {
        self.root_directory.is_none()
            && !self.project_modified
            && self.ximod.name.is_empty()
            && self.ximod.steps.is_empty()
            && self.ximod.required_files.is_empty()
            && self.ximod.conditional_files.is_empty()
    }

    /// Reset the navigation selection to a freshly-loaded project's defaults.
    fn reset_navigation(&mut self) {
        self.current_tab = Tab::Info;
        self.current_step_index = if self.ximod.steps.is_empty() { None } else { Some(0) };
        self.current_group_index = None;
        self.current_plugin_index = None;
        self.current_file_index = None;
        self.current_flag_index = None;
        self.current_dependency_index = None;
        self.current_cond_file_index = None;
        self.current_cond_pattern_index = None;
        self.current_req_file_index = None;
    }

    /// Make a freshly-loaded project the active document — a new tab, unless the
    /// current document is a pristine blank project (then it is replaced).
    fn open_loaded(&mut self, ximod: Ximod, root: PathBuf) {
        self.ensure_doc();
        // If this FOMOD is already open, just focus its tab.
        if let Some(i) = self
            .docs
            .iter()
            .position(|d| d.root_directory.as_deref() == Some(root.as_path()))
        {
            self.switch_doc(i);
            return;
        }
        if !self.active_is_pristine() {
            self.commit_active();
            let clone = self.docs[self.active_doc].clone();
            self.docs.push(clone);
            self.active_doc = self.docs.len() - 1;
        }
        self.ximod = ximod;
        self.root_directory = Some(root);
        self.project_modified = false;
        self.reset_navigation();
        self.commit_active();
    }

    /// Full (untruncated) name of a document: its root folder name, else the mod
    /// name, else empty.
    fn doc_full_name(d: &DocState) -> String {
        if let Some(root) = &d.root_directory {
            if let Some(name) = root.file_name().and_then(|n| n.to_str()) {
                return name.to_string();
            }
        }
        if !d.ximod.name.is_empty() {
            return d.ximod.name.clone();
        }
        String::new()
    }

    /// Close the active FOMOD, asking for confirmation first if it has unsaved
    /// changes. A pristine blank tab closes without prompting.
    fn close_active_fomod(&mut self) {
        self.ensure_doc();
        if self.project_modified {
            self.close_prompt = Some(CloseScope::Active);
        } else {
            self.close_active_fomod_force();
        }
    }

    /// Close the active FOMOD unconditionally. The last remaining document
    /// becomes a blank one.
    fn close_active_fomod_force(&mut self) {
        self.ensure_doc();
        if self.docs.len() <= 1 {
            self.docs[0] = DocState::blank();
            self.active_doc = 0;
        } else {
            self.docs.remove(self.active_doc);
            if self.active_doc >= self.docs.len() {
                self.active_doc = self.docs.len() - 1;
            }
        }
        self.checkout_active();
        self.status_message = self.i18n.t("status-ready");
    }

    /// Close every FOMOD, asking for confirmation first if any has unsaved
    /// changes.
    fn close_all_fomods(&mut self) {
        self.commit_active();
        if self.docs.iter().any(|d| d.modified) {
            self.close_prompt = Some(CloseScope::All);
        } else {
            self.close_all_fomods_force();
        }
    }

    /// Close every FOMOD unconditionally, leaving a single blank project.
    fn close_all_fomods_force(&mut self) {
        self.docs.clear();
        self.docs.push(DocState::blank());
        self.active_doc = 0;
        self.checkout_active();
        self.status_message = self.i18n.t("status-ready");
    }

    /// Save every modified document that has a destination folder.
    fn save_all_modified(&mut self) {
        self.commit_active();
        for i in 0..self.docs.len() {
            if self.docs[i].modified && self.docs[i].root_directory.is_some() {
                if i != self.active_doc {
                    self.commit_active();
                    self.active_doc = i;
                    self.checkout_active();
                }
                self.write_project();
                self.commit_active();
            }
        }
    }

    /// Open the Settings dialog, seeding the temporary values from the current
    /// configuration. Shared by the "Settings" menu item and the Ctrl+, shortcut.
    /// Install the fonts required to display the current interface language and
    /// every language offered for the selected country.
    ///
    /// Cheap when nothing changed: the wanted set is compared to the installed
    /// one and `set_fonts` (which rebuilds the glyph atlas) runs only on change.
    fn sync_fonts(&mut self, ctx: &egui::Context) {
        let mut wanted: Vec<String> = Vec::new();

        // Font of the language currently displayed in the interface.
        if let Some(f) = self.i18n.font_for(&self.config.locale) {
            wanted.push(f.to_string());
        }
        // Fonts of every language listed for the selected country: those are the
        // entries shown in the settings' language drop-down.
        if !self.temp_country.is_empty() {
            for code in self.country_languages.languages_for(&self.temp_country) {
                if let Some(f) = self.i18n.font_for(code) {
                    let f = f.to_string();
                    if !wanted.contains(&f) {
                        wanted.push(f);
                    }
                }
            }
        }
        // Font chosen in the translation editor: also previewed in its own
        // family, so the endonym field renders in the target language typeface.
        let preview = self.trans_font.clone();
        if !preview.is_empty() && !wanted.contains(&preview) {
            wanted.push(preview.clone());
        }
        // The Properties window is a country/language explorer that shows
        // language names (endonyms) in every script — Thai, Amharic, Greek,
        // etc. When it is open, load every distinct language font so none of
        // them renders as a tofu box. There are only a few dozen after dedup,
        // and each is appended to the Proportional fallback chain, so egui
        // picks the right glyph per character.
        if self.show_properties {
            for e in &self.i18n.languages().languages {
                if !e.font.is_empty() && !wanted.contains(&e.font) {
                    wanted.push(e.font.clone());
                }
            }
        }
        // The Translation editor not only shows the reference (column 2) and
        // target (column 3) languages, but also lists many language endonyms in
        // its two drop-downs ("displayed language" = every existing translation,
        // "language to translate" = the country's languages), each in its own
        // script. Loading only the source/target fonts left those lists full of
        // tofu boxes, so — as for the Properties window — load every distinct
        // language font while the editor is open (a few dozen after dedup).
        if self.show_translation {
            for e in &self.i18n.languages().languages {
                if !e.font.is_empty() && !wanted.contains(&e.font) {
                    wanted.push(e.font.clone());
                }
            }
        }
        wanted.sort();
        wanted.dedup();

        if wanted == self.loaded_fonts && preview == self.loaded_preview_font {
            return;
        }
        tracing::info!("Loading {} font(s)", wanted.len());
        ctx.set_fonts(crate::fonts::build_font_definitions_with_preview(
            &wanted,
            if preview.is_empty() { None } else { Some(preview.as_str()) },
        ));
        self.loaded_fonts = wanted;
        self.loaded_preview_font = preview;
        // set_fonts only takes effect at the start of the next frame; request one
        // now so the new glyphs appear immediately instead of waiting for the
        // next input event (which would otherwise leave a brief tofu flash).
        ctx.request_repaint();
    }

    fn open_settings(&mut self) {
        self.temp_locale = self.config.locale.clone();
        self.temp_country = self.config.country.clone();
        self.temp_theme = self.config.theme;
        self.temp_font_size = self.config.font_size;
        self.temp_replace_newlines = self.config.replace_newlines;
        self.temp_max_recent_files = self.config.max_recent_files;
        self.temp_window_width = self.config.window_width;
        self.temp_window_height = self.config.window_height;
        self.settings_focus = 0;
        self.settings_tab = SettingsTab::General;
        self.show_settings = true;
    }

    fn open_directory(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.load_project(path);
        }
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(self.i18n.t("filter-xml"), &["xml"])
            .pick_file()
        {
            if let Some(parent) = path.parent() {
                if let Some(ximod_parent) = parent.parent() {
                    self.load_project(ximod_parent.to_path_buf());
                }
            }
        }
    }

    fn load_project(&mut self, path: PathBuf) {
        match xml::load_ximod(&path) {
            Ok(ximod) => {
                // Open in a new tab (or focus/replace as appropriate).
                self.open_loaded(ximod, path.clone());
                self.config.add_recent_file(path);
                let _ = self.config.save();
                self.status_message = self.i18n.t("msg-load-success");
            }
            Err(e) => {
                let error_msg = self.i18n.t("msg-load-error");
                self.status_message = format!("{}: {}", error_msg, e);
            }
        }
    }

    /// Merge a donor FOMOD into the current project.
    ///
    /// Mirrors the original C++ tool's "Merge FOMOD" command: the user picks the
    /// donor's `ModuleConfig.xml`, and its steps, required files and conditional
    /// installs are appended to the end of the current (recipient) project. The
    /// recipient keeps its own metadata (name, author, version, header image…).
    ///
    /// The command is only reachable once a recipient project exists (a root
    /// directory has been chosen), matching the original: "the recipient should
    /// be loaded from file or created".
    fn merge_fomod(&mut self) {
        if self.root_directory.is_none() {
            self.status_message = self.i18n.t("msg-no-root-selected");
            return;
        }
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(self.i18n.t("filter-xml"), &["xml"])
            .pick_file()
        {
            match xml::load_module_config_file(&path) {
                Ok(mut donor) => {
                    // Append the donor's installation data to the end.
                    self.ximod.steps.append(&mut donor.steps);
                    self.ximod.required_files.append(&mut donor.required_files);
                    self.ximod
                        .conditional_files
                        .append(&mut donor.conditional_files);

                    // Re-anchor the selection to a valid, simple state.
                    self.current_step_index = if self.ximod.steps.is_empty() {
                        None
                    } else {
                        Some(0)
                    };
                    self.current_group_index = None;
                    self.current_plugin_index = None;
                    self.project_modified = true;
                    self.status_message = self.i18n.t("msg-merge-success");
                }
                Err(e) => {
                    let error_msg = self.i18n.t("msg-merge-error");
                    self.status_message = format!("{}: {}", error_msg, e);
                }
            }
        }
    }

    /// Entry point for saving (menu and Ctrl+S).
    ///
    /// Rather than block an incomplete project, we validate it and, if there are
    /// problems, open a confirmation dialog listing every one of them and letting
    /// the user save anyway. A valid project is written straight away.
    fn save_project(&mut self) {
        if self.root_directory.is_none() {
            self.status_message = self.i18n.t("msg-no-root-selected");
            return;
        }

        let errors = self.ximod.validate();
        // Schema conformity of the generated ModuleConfig.xml (ModConfig 5.0).
        let schema_issues: Vec<crate::xml::validate::SchemaIssue> =
            crate::xml::module_config_to_string(&self.ximod)
                .map(|xml| crate::xml::validate::validate_module_config(&xml))
                .unwrap_or_default();

        if !errors.is_empty() || !schema_issues.is_empty() {
            // Build a warning that lists every problem, then ask for
            // confirmation instead of refusing the save.
            let mut msg = self.i18n.t("confirm-save-issues");
            for err in &errors {
                msg.push_str("\n• ");
                msg.push_str(&self.translate_validation_error(err));
            }
            for issue in &schema_issues {
                msg.push_str("\n• ");
                msg.push_str(&self.translate_schema_issue(issue));
            }
            msg.push('\n');
            msg.push('\n');
            msg.push_str(&self.i18n.t("confirm-save-anyway"));
            self.confirm_action = Some(ConfirmAction::SaveAnyway(msg));
            self.show_confirm = true;
            return;
        }

        self.write_project();
    }

    /// Write the FOMOD to disk: pre-save script, XML files, post-save script.
    /// Called for a valid project, or after the user confirms saving one that
    /// still has validation warnings.
    fn write_project(&mut self) {
        let root = match &self.root_directory {
            Some(r) => r.clone(),
            None => {
                self.status_message = self.i18n.t("msg-no-root-selected");
                return;
            }
        };

        // Pre-save script
        if !self.config.pre_save_script.is_empty() {
            let macros = ScriptMacros::new(
                &self.ximod.name,
                &self.ximod.author,
                &self.ximod.version,
                root.to_str().unwrap_or(""),
            );
            let _ = crate::config::run_script(&self.config.pre_save_script, &macros);
        }

        match xml::save_ximod(&self.ximod, &root) {
            Ok(()) => {
                self.project_modified = false;
                self.status_message = self.i18n.t("msg-save-success");

                // Record the saved project in the recent-files list. Previously
                // only "Open folder/file" did this, so a project that was created
                // and only ever saved (never re-opened) never appeared in
                // File → Recent and the [RecentFiles] section stayed empty.
                self.config.add_recent_file(root.clone());
                let _ = self.config.save();

                // Post-save script
                if !self.config.post_save_script.is_empty() {
                    let macros = ScriptMacros::new(
                        &self.ximod.name,
                        &self.ximod.author,
                        &self.ximod.version,
                        root.to_str().unwrap_or(""),
                    );
                    let _ = crate::config::run_script(&self.config.post_save_script, &macros);
                }
            }
            Err(e) => {
                let error_msg = self.i18n.t("msg-save-error");
                self.status_message = format!("{}: {}", error_msg, e);
            }
        }
    }

    /// Build a ready-to-upload distribution archive (FOMOD XML + mod files).
    ///
    /// Writes the FOMOD XML into the project root, then zips the whole root into
    /// a single `.zip` whose layout is exactly what a mod manager expects.
    fn export_distribution(&mut self) {
        let root = match &self.root_directory {
            Some(r) => r.clone(),
            None => {
                self.status_message = self.i18n.t("msg-no-root-selected");
                return;
            }
        };
        let default_name = crate::export::default_archive_name(&self.ximod);
        let Some(path) = rfd::FileDialog::new()
            .set_file_name(&default_name)
            .add_filter("Zip", &["zip"])
            .save_file()
        else {
            return;
        };
        match crate::export::build_distribution_archive(&self.ximod, &root, &path) {
            Ok(n) => {
                let mut args = FluentArgs::new();
                args.set("count", n as i64);
                args.set("path", path.display().to_string());
                self.status_message = self.i18n.t_with_args("msg-export-success", Some(&args));
            }
            Err(e) => {
                let mut args = FluentArgs::new();
                args.set("error", e.to_string());
                self.status_message = self.i18n.t_with_args("msg-export-error", Some(&args));
            }
        }
    }

    /// Translate a validation error into the current locale.
    /// Maps each `ValidationError` variant to its FTL key and arguments,
    /// keeping the model layer free of any i18n dependency.
    pub fn translate_validation_error(&self, err: &ValidationError) -> String {
        match err {
            ValidationError::NoName => self.i18n.t("validation-no-name"),
            ValidationError::NoSteps => self.i18n.t("validation-no-steps"),
            ValidationError::EmptyStep { step } => {
                self.i18n.t_num("validation-empty-step", *step as i64)
            }
            ValidationError::EmptyGroup { step, group } => {
                let mut args = FluentArgs::new();
                args.set("step", *step as i64);
                args.set("group", *group as i64);
                self.i18n.t_with_args("validation-empty-group", Some(&args))
            }
            ValidationError::NoPlugins { step, group } => {
                let mut args = FluentArgs::new();
                args.set("step", *step as i64);
                args.set("name", group.clone());
                self.i18n.t_with_args("validation-no-plugins", Some(&args))
            }
        }
    }

    /// Localise a schema validation issue (ModConfig 5.0), prefixed with its
    /// line/column position.
    pub fn translate_schema_issue(&self, issue: &crate::xml::validate::SchemaIssue) -> String {
        use crate::xml::validate::SchemaIssueKind as K;
        let pair = |k: &str, a: &str, av: String, b: &str, bv: String| {
            let mut args = FluentArgs::new();
            args.set(a, av);
            args.set(b, bv);
            self.i18n.t_with_args(k, Some(&args))
        };
        let msg = match &issue.kind {
            K::WrongRoot { found, expected } => {
                pair("schema-wrong-root", "found", found.clone(), "expected", expected.clone())
            }
            K::UnknownElement { element, parent } => {
                pair("schema-unknown", "element", element.clone(), "parent", parent.clone())
            }
            K::MissingChild { parent, child } => {
                pair("schema-missing", "parent", parent.clone(), "child", child.clone())
            }
            K::NeedsOne { parent, child } => {
                pair("schema-needs-one", "parent", parent.clone(), "child", child.clone())
            }
            K::TooMany { parent, child } => {
                pair("schema-too-many", "parent", parent.clone(), "child", child.clone())
            }
            K::MissingAttr { element, attr } => {
                pair("schema-missing-attr", "element", element.clone(), "attr", attr.clone())
            }
            K::ChooseOne { parent, options } => {
                pair("schema-choose-one", "parent", parent.clone(), "options", options.clone())
            }
            K::BadEnum { element, attr, value, allowed } => {
                let mut args = FluentArgs::new();
                args.set("element", element.clone());
                args.set("attr", attr.clone());
                args.set("value", value.clone());
                args.set("allowed", allowed.clone());
                self.i18n.t_with_args("schema-bad-enum", Some(&args))
            }
        };
        let mut args = FluentArgs::new();
        args.set("line", issue.line as i64);
        args.set("col", issue.column as i64);
        args.set("msg", msg);
        self.i18n.t_with_args("schema-line-col", Some(&args))
    }

    /// Run the full validation (project + ModuleConfig/info schema) and open the
    /// report window.
    pub fn run_full_validation(&mut self) {
        let mut report: Vec<String> = Vec::new();
        // Project-level checks.
        for err in self.ximod.validate() {
            report.push(self.translate_validation_error(&err));
        }
        // ModuleConfig.xml schema.
        if let Ok(xml) = crate::xml::module_config_to_string(&self.ximod) {
            for issue in crate::xml::validate::validate_module_config(&xml) {
                report.push(self.translate_schema_issue(&issue));
            }
        }
        // info.xml schema.
        if let Ok(xml) = crate::xml::info_xml_to_string(&self.ximod) {
            for issue in crate::xml::validate::validate_info(&xml) {
                report.push(self.translate_schema_issue(&issue));
            }
        }
        // Referenced-file verification (V2 priority 1): needs the root folder on
        // disk. Skipped with a note when no root is set.
        match &self.root_directory {
            Some(root) => {
                let issues = crate::models::verify::verify_files(&self.ximod, root);
                for issue in &issues {
                    report.push(self.translate_file_issue(issue));
                }
            }
            None => report.push(self.i18n.t("verify-no-root")),
        }
        self.validation_report = report;
        self.show_validation_report = true;
    }

    /// Build a human context string for a referenced-file location.
    fn file_ref_location(&self, loc: &crate::models::verify::RefLoc) -> String {
        use crate::models::verify::RefLoc as L;
        match loc {
            L::Header => self.i18n.t("loc-header"),
            L::RequiredFiles => self.i18n.t("loc-required"),
            L::ConditionalSet { index } => self.i18n.t_num("loc-conditional", *index as i64),
            L::Plugin { step, group, plugin } => {
                let mut args = FluentArgs::new();
                args.set("step", *step as i64);
                args.set("group", *group as i64);
                args.set("plugin", plugin.clone());
                self.i18n.t_with_args("loc-plugin", Some(&args))
            }
        }
    }

    /// Localise a referenced-file issue.
    pub fn translate_file_issue(&self, issue: &crate::models::verify::FileIssue) -> String {
        use crate::models::verify::FileIssue as F;
        let with = |key: &str, loc: &crate::models::verify::RefLoc, path: &str| {
            let mut args = FluentArgs::new();
            args.set("loc", self.file_ref_location(loc));
            args.set("path", path.to_string());
            self.i18n.t_with_args(key, Some(&args))
        };
        match issue {
            F::MissingSource { loc, path, folder } => {
                with(if *folder { "verify-missing-folder" } else { "verify-missing-file" }, loc, path)
            }
            F::MissingImage { loc, path } => with("verify-missing-image", loc, path),
            F::AbsolutePath { loc, path } => with("verify-absolute", loc, path),
            F::OutsideRoot { loc, path } => with("verify-outside", loc, path),
            F::OrphanFile { path } => {
                self.i18n.t_arg("verify-orphan", "path", path)
            }
        }
    }

    /// Render the validation report window.
    fn render_validation_report(&mut self, ctx: &egui::Context) {
        if !self.show_validation_report {
            return;
        }
        let title = self.i18n.t("validate-report-title");
        let ok = self.i18n.t("validate-ok");
        let close = self.i18n.t("btn-ok");
        let report = self.validation_report.clone();
        let mut open = true;
        let mut do_close = false;

        egui::Window::new(&title)
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size([580.0, 400.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                if report.is_empty() {
                    ui.label(
                        egui::RichText::new(&ok).color(egui::Color32::from_rgb(60, 160, 60)),
                    );
                } else {
                    ui.label(
                        egui::RichText::new(format!("{} — {}", title, report.len())).strong(),
                    );
                    ui.add_space(4.0);
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(320.0)
                        .show(ui, |ui| {
                            for line in &report {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label("•");
                                    ui.label(line);
                                });
                            }
                        });
                }
                ui.separator();
                if ui.button(&close).clicked() {
                    do_close = true;
                }
            });

        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            do_close = true;
        }
        if do_close || !open {
            self.show_validation_report = false;
        }
    }

    /// Render the confirmation dialog when a `ConfirmAction` is pending.
    /// Uses the reusable `ConfirmDialog` component with translated text.
    fn render_confirm_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_confirm {
            return;
        }
        let action = match self.confirm_action.clone() {
            Some(a) => a,
            None => {
                self.show_confirm = false;
                return;
            }
        };

        let title = self.i18n.t("confirm-title");
        let message = match &action {
            ConfirmAction::DeleteStep(_) => self.i18n.t("confirm-delete"),
            ConfirmAction::SaveAnyway(msg) => msg.clone(),
        };

        let mut dialog = ConfirmDialog::new(title, message);
        // The "save anyway" dialog reads better with explicit Save/Cancel
        // buttons than a bare Yes/No.
        match &action {
            ConfirmAction::SaveAnyway(_) => {
                dialog.confirm_text = self.i18n.t("btn-save");
                dialog.cancel_text = self.i18n.t("btn-cancel");
            }
            _ => {
                dialog.confirm_text = self.i18n.t("btn-yes");
                dialog.cancel_text = self.i18n.t("btn-no");
            }
        }

        let mut open = true;
        match dialog.show(ctx, &mut open) {
            Some(true) => {
                self.execute_confirm_action(action);
                self.show_confirm = false;
                self.confirm_action = None;
            }
            Some(false) => {
                self.show_confirm = false;
                self.confirm_action = None;
            }
            None => {
                if !open {
                    self.show_confirm = false;
                    self.confirm_action = None;
                }
            }
        }
        // Escape cancels the confirmation (never executes the action).
        if self.show_confirm
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.show_confirm = false;
            self.confirm_action = None;
        }
    }

    /// Execute a confirmed action.
    fn execute_confirm_action(&mut self, action: ConfirmAction) {
        match action {
            ConfirmAction::DeleteStep(step_idx) => {
                if step_idx < self.ximod.steps.len() {
                    self.ximod.steps.remove(step_idx);
                    self.current_step_index = if self.ximod.steps.is_empty() {
                        None
                    } else {
                        Some(step_idx.saturating_sub(1).min(self.ximod.steps.len() - 1))
                    };
                    self.current_group_index = None;
                    self.current_plugin_index = None;
                    self.project_modified = true;
                }
            }
            ConfirmAction::SaveAnyway(_) => {
                self.write_project();
            }
        }
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
            settings: KeyboardShortcut::new(Modifiers::COMMAND, Key::Comma),
            quit: KeyboardShortcut::new(Modifiers::COMMAND, Key::Q),
            about: KeyboardShortcut::new(Modifiers::NONE, Key::F1),
        }
    }

    /// Trigger menu actions from global keyboard shortcuts. Ignored while a modal
    /// dialog is open, so shortcuts don't fire behind a dialog.
    fn handle_menu_shortcuts(&mut self, ctx: &egui::Context) {
        if self.show_settings || self.show_about || self.show_script_dialog || self.show_confirm || self.show_translation || self.show_xml_editor || self.show_validation_report {
            return;
        }
        let sc = Self::menu_shortcuts();

        // Ctrl+Shift+O must be tested before Ctrl+O (more specific first).
        if ctx.input_mut(|i| i.consume_shortcut(&sc.open_file)) {
            self.open_file();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.open)) {
            self.open_directory();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.new)) {
            self.request_new_project();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.save)) {
            if self.root_directory.is_some() {
                self.save_project();
            }
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.settings)) {
            self.open_settings();
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.about)) {
            self.show_about = true;
        } else if ctx.input_mut(|i| i.consume_shortcut(&sc.quit)) {
            self.request_close = true;
        }
    }

    fn render_menu_bar(&mut self, ctx: &egui::Context) {
        // Check if a modal dialog is open
        let modal_open = self.show_settings || self.show_about || self.show_script_dialog || self.show_confirm || (self.show_xml_editor && self.xml_editor_editing);
        
        let menu_file = self.i18n.t("menu-file");
        let menu_new = self.i18n.t("menu-new");
        let menu_open = self.i18n.t("menu-open");
        let menu_open_file = self.i18n.t("menu-open-file");
        let menu_save = self.i18n.t("menu-save");
        let menu_merge = self.i18n.t("menu-merge");
        let menu_export = self.i18n.t("menu-export");
        let menu_close_fomod = self.i18n.t("menu-close-fomod");
        let menu_close_all = self.i18n.t("menu-close-all-fomods");
        let menu_recent = self.i18n.t("menu-recent");
        let menu_exit = self.i18n.t("menu-exit");
        let menu_options = self.i18n.t("menu-options");
        let menu_xml_editor = self.i18n.t("xml-editor-title");
        let menu_settings = self.i18n.t("menu-settings");
        let menu_pre_save = self.i18n.t("menu-pre-save-script");
        let menu_post_save = self.i18n.t("menu-post-save-script");
        let menu_translation = self.i18n.t("menu-translation");
        let menu_preview = self.i18n.t("menu-preview");
        let menu_validate = self.i18n.t("menu-validate");
        let menu_properties = self.i18n.t("menu-properties");
        let menu_help = self.i18n.t("menu-help");
        let menu_about = self.i18n.t("menu-about");

        let has_root = self.root_directory.is_some();
        let recent_files = self.config.recent_files.clone();

        // Shortcut display strings (platform-aware via format_shortcut).
        let sc = Self::menu_shortcuts();
        let sct_new = ctx.format_shortcut(&sc.new);
        let sct_open = ctx.format_shortcut(&sc.open);
        let sct_open_file = ctx.format_shortcut(&sc.open_file);
        let sct_save = ctx.format_shortcut(&sc.save);
        // egui renders the Comma key by name ("Comma"); show the symbol instead
        // so the shortcut reads "Ctrl+," (or "⌘," on macOS) rather than "Ctrl+Comma".
        let sct_settings = ctx.format_shortcut(&sc.settings).replace("Comma", ",");
        let sct_quit = ctx.format_shortcut(&sc.quit);
        let sct_about = ctx.format_shortcut(&sc.about);

        // Use locale_version to create unique IDs for menu recreation
        let lv = self.locale_version;

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            // Disable menu when modal is open
            if modal_open {
                ui.disable();
            }
            
            egui::menu::bar(ui, |ui| {
                // File menu with unique ID based on locale_version
                ui.push_id(("file_menu", lv), |ui| {
                    ui.menu_button(&menu_file, |ui| {
                        let btn_new = egui::Button::new(&menu_new)
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .shortcut_text(&sct_new);
                        if ui.add(btn_new).clicked() {
                            self.request_new_project();
                            ui.close_menu();
                        }

                        // Open Folder menu - always visible
                        let btn_open = egui::Button::new(&menu_open)
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .shortcut_text(&sct_open);
                        if ui.add(btn_open).clicked() {
                            self.open_directory();
                            ui.close_menu();
                        }

                        // Open File menu - always visible
                        let btn_open_file = egui::Button::new(&menu_open_file)
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .shortcut_text(&sct_open_file);
                        if ui.add(btn_open_file).clicked() {
                            self.open_file();
                            ui.close_menu();
                        }

                        let btn_save = egui::Button::new(&menu_save)
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .shortcut_text(&sct_save);
                        if ui.add_enabled(has_root, btn_save).clicked() {
                            self.save_project();
                            ui.close_menu();
                        }

                        // Merge FOMOD - append a donor's steps/files to the
                        // current project. Only usable once a recipient exists.
                        let btn_merge = egui::Button::new(&menu_merge)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add_enabled(has_root, btn_merge).clicked() {
                            self.merge_fomod();
                            ui.close_menu();
                        }

                        // Export a ready-to-upload distribution archive (.zip).
                        let btn_export = egui::Button::new(&menu_export)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add_enabled(has_root, btn_export).clicked() {
                            self.export_distribution();
                            ui.close_menu();
                        }

                        ui.separator();

                        // Close the active FOMOD / all FOMODs.
                        let btn_close = egui::Button::new(&menu_close_fomod)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add(btn_close).clicked() {
                            self.close_active_fomod();
                            ui.close_menu();
                        }
                        let btn_close_all = egui::Button::new(&menu_close_all)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add(btn_close_all).clicked() {
                            self.close_all_fomods();
                            ui.close_menu();
                        }

                        ui.separator();

                        // Recent files submenu with unique ID
                        ui.push_id(("recent_menu", lv), |ui| {
                            ui.menu_button(&menu_recent, |ui| {
                                for path in &recent_files {
                                    let display = path.to_string_lossy().to_string();
                                    let btn_path = egui::Button::new(&display)
                                        .wrap_mode(egui::TextWrapMode::Extend);
                                    if ui.add(btn_path).clicked() {
                                        self.load_project(path.clone());
                                        ui.close_menu();
                                    }
                                }
                                if recent_files.is_empty() {
                                    ui.label(self.i18n.t("label-empty"));
                                }
                            });
                        });

                        ui.separator();

                        let btn_exit = egui::Button::new(&menu_exit)
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .shortcut_text(&sct_quit);
                        if ui.add(btn_exit).clicked() {
                            self.request_close = true;
                            ui.close_menu();
                        }
                    });
                });

                // Options menu with unique ID based on locale_version
                ui.push_id(("options_menu", lv), |ui| {
                    ui.menu_button(&menu_options, |ui| {
                        // Use buttons with explicit wrap_mode to prevent text wrapping
                        let btn = egui::Button::new(&menu_settings)
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .shortcut_text(&sct_settings);
                        if ui.add(btn).clicked() {
                            self.open_settings();
                            ui.close_menu();
                        }

                        ui.separator();

                        let btn_pre = egui::Button::new(&menu_pre_save)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add(btn_pre).clicked() {
                            self.editing_pre_script = true;
                            self.script_content = self.config.pre_save_script.clone();
                            self.show_script_dialog = true;
                            ui.close_menu();
                        }

                        let btn_post = egui::Button::new(&menu_post_save)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add(btn_post).clicked() {
                            self.editing_pre_script = false;
                            self.script_content = self.config.post_save_script.clone();
                            self.show_script_dialog = true;
                            ui.close_menu();
                        }

                        ui.separator();

                        let btn_trans = egui::Button::new(&menu_translation)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add(btn_trans).clicked() {
                            // Translate FROM English by default.
                            self.trans_source_lang = "eng".to_string();
                            // Start from the user's own country, as configured
                            // in the settings.
                            if self.trans_country.is_empty() {
                                self.trans_country = self.config.country.clone();
                            }
                            // Keep the target language consistent with the country
                            // shown by the flag: prefer the current UI language when
                            // it is spoken there, otherwise the country's first
                            // language. Otherwise the flag and the info card can
                            // disagree (e.g. a Japanese flag with French details).
                            let ui_locale = self.i18n.current_locale().to_string();
                            let langs = self.country_languages.languages_for(&self.trans_country);
                            self.trans_target_lang = if langs.iter().any(|l| *l == ui_locale) {
                                ui_locale
                            } else if let Some(first) = langs.first() {
                                first.clone()
                            } else {
                                ui_locale
                            };
                            self.load_translation_entries();
                            self.show_translation = true;
                            ui.close_menu();
                        }

                        ui.separator();

                        // XML editor: view/edit info.xml and ModuleConfig.xml.
                        ui.menu_button(&menu_xml_editor, |ui| {
                            let btn_info = egui::Button::new("info.xml")
                                .wrap_mode(egui::TextWrapMode::Extend);
                            if ui.add(btn_info).clicked() {
                                self.open_xml_editor(crate::ui::xml_editor::XmlTarget::InfoXml);
                                ui.close_menu();
                            }
                            let btn_config = egui::Button::new("ModuleConfig.xml")
                                .wrap_mode(egui::TextWrapMode::Extend);
                            if ui.add(btn_config).clicked() {
                                self.open_xml_editor(crate::ui::xml_editor::XmlTarget::ModuleConfig);
                                ui.close_menu();
                            }
                        });

                        ui.separator();

                        // FOMOD installer preview (interactive simulation).
                        let btn_preview = egui::Button::new(&menu_preview)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add(btn_preview).clicked() {
                            self.open_preview();
                            ui.close_menu();
                        }

                        // Full FOMOD validation (project + ModConfig 5.0 schema).
                        let btn_validate = egui::Button::new(&menu_validate)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add(btn_validate).clicked() {
                            self.run_full_validation();
                            ui.close_menu();
                        }

                        // Country/language database explorer.
                        let btn_properties = egui::Button::new(&menu_properties)
                            .wrap_mode(egui::TextWrapMode::Extend);
                        if ui.add(btn_properties).clicked() {
                            self.open_properties();
                            ui.close_menu();
                        }
                    });
                });

                // Help menu with unique ID based on locale_version
                ui.push_id(("help_menu", lv), |ui| {
                    ui.menu_button(&menu_help, |ui| {
                        let btn_about = egui::Button::new(&menu_about)
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .shortcut_text(&sct_about);
                        if ui.add(btn_about).clicked() {
                            self.show_about = true;
                            ui.close_menu();
                        }
                    });
                });
            });
        });
    }

    fn render_info_tab(&mut self, ui: &mut egui::Ui) {
        let label_workspace = self.i18n.t("label-workspace");
        let label_root = self.i18n.t("label-root-dir");
        let label_name = self.i18n.t("label-mod-name");
        let label_author = self.i18n.t("label-author");
        let label_version = self.i18n.t("label-version");
        let label_game = self.i18n.t("label-game-name");
        let label_category = self.i18n.t("label-category");
        let label_url = self.i18n.t("label-url");
        let label_header = self.i18n.t("label-header-image");
        let label_desc = self.i18n.t("label-description");
        let btn_browse = self.i18n.t("btn-browse");
        let btn_clear = self.i18n.t("btn-clear");
        let placeholder = self.i18n.t("placeholder-select-dir");
        let placeholder_game = self.i18n.t("placeholder-select-game");

        // Pre-compute dynamic game & category data (avoids borrow conflicts inside
        // the ComboBox closures). Both come from the external Categories.json.
        let game_list = self.games.game_list();
        let selected_game_id = self.ximod.game.clone();
        let current_game_name = self
            .games
            .name_for(&selected_game_id)
            .unwrap_or("")
            .to_string();
        let game_categories: Vec<String> =
            self.games.categories_for(&selected_game_id).to_vec();
        // Nexus Mods slug of the selected game (empty when unknown) → direct link.
        let nexus_slug = self
            .games
            .nexus_slug_for(&selected_game_id)
            .unwrap_or("")
            .to_string();
        let btn_nexus = self.i18n.t("btn-nexus");
        let nexus_hint = self.i18n.t("nexus-open-hint");

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Workspace section header
            ui.label(RichText::new(&label_workspace).strong());
            ui.add_space(2.0);

            // Root directory
            ui.horizontal(|ui| {
                ui.label(&label_root);
                let root_text = self
                    .root_directory
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| placeholder.clone());
                ui.label(&root_text);
                if ui.button(&btn_browse).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.root_directory = Some(path);
                        self.project_modified = true;
                    }
                }
            });

            ui.separator();

            egui::Grid::new("info_grid")
                .num_columns(2)
                .spacing([40.0, 8.0])
                .show(ui, |ui| {
                    ui.label(&label_name);
                    if ui.text_edit_singleline(&mut self.ximod.name).changed() {
                        self.project_modified = true;
                    }
                    ui.end_row();

                    ui.label(&label_author);
                    if ui.text_edit_singleline(&mut self.ximod.author).changed() {
                        self.project_modified = true;
                    }
                    ui.end_row();

                    ui.label(&label_version);
                    if ui.text_edit_singleline(&mut self.ximod.version).changed() {
                        self.project_modified = true;
                    }
                    ui.end_row();

                    // Game selector (dynamic list from Categories.json)
                    ui.label(&label_game);
                    let game_text = if current_game_name.is_empty() {
                        placeholder_game.clone()
                    } else {
                        current_game_name.clone()
                    };
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("game_combo")
                            .selected_text(&game_text)
                            .show_ui(ui, |ui| {
                                for (game_id, game_name) in &game_list {
                                    let is_selected = self.ximod.game == *game_id;
                                    if ui.selectable_label(is_selected, game_name).clicked() {
                                        self.ximod.game = game_id.clone();
                                        self.project_modified = true;
                                    }
                                }
                            });
                        // Direct link to the game's Nexus Mods page (uses the slug).
                        if ui
                            .add_enabled(!nexus_slug.is_empty(), egui::Button::new(&btn_nexus))
                            .on_hover_text(&nexus_hint)
                            .clicked()
                        {
                            crate::fonts::open_url(&format!(
                                "https://www.nexusmods.com/{}",
                                nexus_slug
                            ));
                        }
                    });
                    ui.end_row();

                    ui.label(&label_category);
                    let current_cat = self.ximod.category.as_str().to_string();
                    egui::ComboBox::from_id_salt("category_combo")
                        .selected_text(&current_cat)
                        .show_ui(ui, |ui| {
                            if game_categories.is_empty() {
                                // Fallback to built-in categories when no game is
                                // selected (or its list is unavailable).
                                for cat in ModCategory::predefined() {
                                    let is_selected = self.ximod.category == *cat;
                                    if ui.selectable_label(is_selected, cat.as_str()).clicked() {
                                        self.ximod.category = cat.clone();
                                        self.project_modified = true;
                                    }
                                }
                            } else {
                                for cat in &game_categories {
                                    let is_selected = self.ximod.category.as_str() == cat.as_str();
                                    if ui.selectable_label(is_selected, cat).clicked() {
                                        self.ximod.category = ModCategory::from_str(cat);
                                        self.project_modified = true;
                                    }
                                }
                            }
                        });
                    ui.end_row();

                    ui.label(&label_url);
                    if ui.text_edit_singleline(&mut self.ximod.url).changed() {
                        self.project_modified = true;
                    }
                    ui.end_row();
                });

            ui.separator();

            // Header image
            ui.horizontal(|ui| {
                ui.label(&label_header);
                let img_text = self.ximod.header_image.clone().unwrap_or_default();
                ui.label(&img_text);

                if ui.button(&btn_browse).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(self.i18n.t("filter-images"), &["png", "jpg", "jpeg", "bmp"])
                        .pick_file()
                    {
                        if let Some(ref root) = self.root_directory {
                            if let Ok(rel) = path.strip_prefix(root) {
                                self.ximod.header_image =
                                    Some(rel.to_string_lossy().to_string());
                                self.project_modified = true;
                            }
                        }
                    }
                }

                if ui.button(&btn_clear).clicked() {
                    self.ximod.header_image = None;
                    self.project_modified = true;
                }
            });

            // Header image preview (rendered from disk relative to the root dir)
            {
                let abs_path = self.ximod.header_image.as_ref().and_then(|rel| {
                    self.root_directory.as_ref().map(|root| root.join(rel))
                });
                let fallback = self.i18n.t("image-no-image");
                ImageDisplay::new(220.0, 110.0)
                    .with_fallback(fallback)
                    .show(ui, abs_path.as_deref());
            }

            ui.separator();

            // Description
            ui.label(&label_desc);
            if ui
                .add_sized(
                    [ui.available_width(), 150.0],
                    egui::TextEdit::multiline(&mut self.ximod.description),
                )
                .changed()
            {
                self.project_modified = true;
            }
        });
    }

    fn render_steps_tab(&mut self, ui: &mut egui::Ui) {
        let label_step_name = self.i18n.t("label-step-name");
        let btn_delete_step = self.i18n.t("btn-delete-step");
        let hint_before = self.i18n.t("reorder-before");
        let hint_after = self.i18n.t("reorder-after");

        let step_names: Vec<String> = self.ximod.steps.iter().map(|s| s.name.clone()).collect();

        // Step tabs
        ui.horizontal(|ui| {
            for (idx, name) in step_names.iter().enumerate() {
                let selected = self.current_step_index == Some(idx);
                if ui.selectable_label(selected, name).clicked() {
                    self.current_step_index = Some(idx);
                    self.current_group_index = None;
                    self.current_plugin_index = None;
                }
            }

            if ui.button("➕").clicked() {
                let name = self.i18n.t_num("default-step-name", (self.ximod.steps.len() + 1) as i64);
                self.ximod.steps.push(Step::new(name));
                self.current_step_index = Some(self.ximod.steps.len() - 1);
                self.project_modified = true;
            }

            // Reorder the selected step (horizontal tab row → ◀ ▶).
            if let Some(i) = self.current_step_index {
                let n = self.ximod.steps.len();
                if ui
                    .add_enabled(i > 0, crate::ui::components::arrow_left_button())
                    .on_hover_text(&hint_before)
                    .clicked()
                {
                    crate::ui::components::move_up(&mut self.ximod.steps, i);
                    self.current_step_index = Some(i - 1);
                    self.project_modified = true;
                }
                if ui
                    .add_enabled(i + 1 < n, crate::ui::components::arrow_right_button())
                    .on_hover_text(&hint_after)
                    .clicked()
                {
                    crate::ui::components::move_down(&mut self.ximod.steps, i);
                    self.current_step_index = Some(i + 1);
                    self.project_modified = true;
                }
            }
        });

        ui.separator();

        if let Some(step_idx) = self.current_step_index {
            if step_idx < self.ximod.steps.len() {
                // Step name
                ui.horizontal(|ui| {
                    ui.label(&label_step_name);
                    let mut step_name = self.ximod.steps[step_idx].name.clone();
                    if ui.text_edit_singleline(&mut step_name).changed() {
                        self.ximod.steps[step_idx].name = step_name;
                        self.project_modified = true;
                    }

                    if ui.button(&btn_delete_step).clicked() {
                        // Ask for confirmation before deleting a whole step.
                        self.confirm_action = Some(ConfirmAction::DeleteStep(step_idx));
                        self.show_confirm = true;
                        return;
                    }
                });

                ui.separator();

                ui.columns(2, |columns| {
                    self.render_groups_panel(&mut columns[0], step_idx);
                    self.render_plugin_details(&mut columns[1]);
                });
            }
        }
    }

    fn render_groups_panel(&mut self, ui: &mut egui::Ui, step_idx: usize) {
        let label_group = self.i18n.t("label-group-name");
        let btn_add_group = self.i18n.t("btn-add-group");
        let btn_remove_group = self.i18n.t("btn-remove-group");
        let label_plugin = self.i18n.t("label-plugin-name");
        let btn_add_plugin = self.i18n.t("btn-add-plugin");
        let btn_remove_plugin = self.i18n.t("btn-remove-plugin");
        let label_type = self.i18n.t("label-group-type");
        let hint_before = self.i18n.t("reorder-before");
        let hint_after = self.i18n.t("reorder-after");

        section_header(ui, &label_group);

        let groups: Vec<(String, String)> = if step_idx < self.ximod.steps.len() {
            self.ximod.steps[step_idx]
                .plugin_groups
                .iter()
                .map(|g| (g.name.clone(), g.selection_type.as_str().to_string()))
                .collect()
        } else {
            Vec::new()
        };

        egui::ScrollArea::vertical()
            .id_salt("groups_list")
            .max_height(120.0)
            .show(ui, |ui| {
                for (idx, (name, sel_type)) in groups.iter().enumerate() {
                    let selected = self.current_group_index == Some(idx);
                    let display = format!("{} ({})", name, sel_type);
                    if ui.selectable_label(selected, &display).clicked() {
                        self.current_group_index = Some(idx);
                        self.current_plugin_index = None;
                    }
                }
            });

        // Group editing
        if let Some(group_idx) = self.current_group_index {
            if step_idx < self.ximod.steps.len()
                && group_idx < self.ximod.steps[step_idx].plugin_groups.len()
            {
                let mut group_name =
                    self.ximod.steps[step_idx].plugin_groups[group_idx].name.clone();
                let current_sel_type =
                    self.ximod.steps[step_idx].plugin_groups[group_idx].selection_type;

                ui.horizontal(|ui| {
                    if ui.text_edit_singleline(&mut group_name).changed() {
                        self.ximod.steps[step_idx].plugin_groups[group_idx].name = group_name;
                        self.project_modified = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(&label_type);
                    egui::ComboBox::from_id_salt("group_type")
                        .selected_text(current_sel_type.as_str())
                        .show_ui(ui, |ui| {
                            for st in SelectionType::variants() {
                                if ui
                                    .selectable_label(current_sel_type == *st, st.as_str())
                                    .clicked()
                                {
                                    self.ximod.steps[step_idx].plugin_groups[group_idx]
                                        .selection_type = *st;
                                    self.project_modified = true;
                                }
                            }
                        });
                });
            }
        }

        ui.horizontal(|ui| {
            if ui.button(&btn_add_group).clicked() && step_idx < self.ximod.steps.len() {
                let name = self.i18n.t_num(
                    "default-group-name",
                    (self.ximod.steps[step_idx].plugin_groups.len() + 1) as i64,
                );
                self.ximod.steps[step_idx]
                    .plugin_groups
                    .push(PluginGroup::new(name, SelectionType::SelectAny));
                self.project_modified = true;
            }

            let can_remove = self.current_group_index.is_some();
            if ui
                .add_enabled(can_remove, egui::Button::new(&btn_remove_group))
                .clicked()
            {
                if let Some(group_idx) = self.current_group_index {
                    if step_idx < self.ximod.steps.len() {
                        self.ximod.steps[step_idx].plugin_groups.remove(group_idx);
                        self.current_group_index = None;
                        self.current_plugin_index = None;
                        self.project_modified = true;
                    }
                }
            }

            // Reorder the selected group (▲ ▼).
            if let Some(gi) = self.current_group_index {
                if step_idx < self.ximod.steps.len() {
                    let n = self.ximod.steps[step_idx].plugin_groups.len();
                    if ui
                        .add_enabled(gi > 0, crate::ui::components::arrow_up_button())
                        .on_hover_text(&hint_before)
                        .clicked()
                    {
                        crate::ui::components::move_up(
                            &mut self.ximod.steps[step_idx].plugin_groups,
                            gi,
                        );
                        self.current_group_index = Some(gi - 1);
                        self.current_plugin_index = None;
                        self.project_modified = true;
                    }
                    if ui
                        .add_enabled(gi + 1 < n, crate::ui::components::arrow_down_button())
                        .on_hover_text(&hint_after)
                        .clicked()
                    {
                        crate::ui::components::move_down(
                            &mut self.ximod.steps[step_idx].plugin_groups,
                            gi,
                        );
                        self.current_group_index = Some(gi + 1);
                        self.current_plugin_index = None;
                        self.project_modified = true;
                    }
                }
            }
        });

        ui.add_space(16.0);
        section_header(ui, &label_plugin);

        let plugins: Vec<String> = if let Some(group_idx) = self.current_group_index {
            if step_idx < self.ximod.steps.len()
                && group_idx < self.ximod.steps[step_idx].plugin_groups.len()
            {
                self.ximod.steps[step_idx].plugin_groups[group_idx]
                    .plugins
                    .iter()
                    .map(|p| p.name.clone())
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        egui::ScrollArea::vertical()
            .id_salt("plugins_list")
            .max_height(120.0)
            .show(ui, |ui| {
                for (idx, name) in plugins.iter().enumerate() {
                    let selected = self.current_plugin_index == Some(idx);
                    if ui.selectable_label(selected, name).clicked() {
                        self.current_plugin_index = Some(idx);
                    }
                }
            });

        ui.horizontal(|ui| {
            let can_add = self.current_group_index.is_some();
            if ui
                .add_enabled(can_add, egui::Button::new(&btn_add_plugin))
                .clicked()
            {
                if let Some(group_idx) = self.current_group_index {
                    if step_idx < self.ximod.steps.len()
                        && group_idx < self.ximod.steps[step_idx].plugin_groups.len()
                    {
                        let name = self.i18n.t_num(
                            "default-plugin-name",
                            (self.ximod.steps[step_idx].plugin_groups[group_idx].plugins.len() + 1) as i64,
                        );
                        self.ximod.steps[step_idx].plugin_groups[group_idx]
                            .plugins
                            .push(Plugin::new(name));
                        self.project_modified = true;
                    }
                }
            }

            let can_remove = self.current_plugin_index.is_some();
            if ui
                .add_enabled(can_remove, egui::Button::new(&btn_remove_plugin))
                .clicked()
            {
                if let (Some(group_idx), Some(plugin_idx)) =
                    (self.current_group_index, self.current_plugin_index)
                {
                    if step_idx < self.ximod.steps.len()
                        && group_idx < self.ximod.steps[step_idx].plugin_groups.len()
                    {
                        self.ximod.steps[step_idx].plugin_groups[group_idx]
                            .plugins
                            .remove(plugin_idx);
                        self.current_plugin_index = None;
                        self.project_modified = true;
                    }
                }
            }

            // Reorder the selected plugin (▲ ▼).
            if let (Some(group_idx), Some(pi)) =
                (self.current_group_index, self.current_plugin_index)
            {
                if step_idx < self.ximod.steps.len()
                    && group_idx < self.ximod.steps[step_idx].plugin_groups.len()
                {
                    let n = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins.len();
                    if ui
                        .add_enabled(pi > 0, crate::ui::components::arrow_up_button())
                        .on_hover_text(&hint_before)
                        .clicked()
                    {
                        crate::ui::components::move_up(
                            &mut self.ximod.steps[step_idx].plugin_groups[group_idx].plugins,
                            pi,
                        );
                        self.current_plugin_index = Some(pi - 1);
                        self.project_modified = true;
                    }
                    if ui
                        .add_enabled(pi + 1 < n, crate::ui::components::arrow_down_button())
                        .on_hover_text(&hint_after)
                        .clicked()
                    {
                        crate::ui::components::move_down(
                            &mut self.ximod.steps[step_idx].plugin_groups[group_idx].plugins,
                            pi,
                        );
                        self.current_plugin_index = Some(pi + 1);
                        self.project_modified = true;
                    }
                }
            }
        });
    }

    fn render_plugin_details(&mut self, ui: &mut egui::Ui) {
        let step_idx = match self.current_step_index {
            Some(i) => i,
            None => return,
        };
        let group_idx = match self.current_group_index {
            Some(i) => i,
            None => {
                ui.label(self.i18n.t("msg-select-group-first"));
                return;
            }
        };
        let plugin_idx = match self.current_plugin_index {
            Some(i) => i,
            None => {
                ui.label(self.i18n.t("msg-select-plugin-edit"));
                return;
            }
        };

        if step_idx >= self.ximod.steps.len() {
            return;
        }
        if group_idx >= self.ximod.steps[step_idx].plugin_groups.len() {
            return;
        }
        if plugin_idx >= self.ximod.steps[step_idx].plugin_groups[group_idx].plugins.len() {
            return;
        }

        let label_name = self.i18n.t("label-plugin-name");
        let label_desc = self.i18n.t("label-plugin-desc");
        let label_type = self.i18n.t("label-plugin-type");
        let label_image = self.i18n.t("label-plugin-image");
        let btn_browse = self.i18n.t("btn-browse");
        let btn_clear = self.i18n.t("btn-clear");
        let label_flags = self.i18n.t("label-flag-name");
        let label_files = self.i18n.t("label-source");

        egui::ScrollArea::vertical().show(ui, |ui| {
            section_header(ui, &label_name);

            let mut plugin_name = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins
                [plugin_idx]
                .name
                .clone();
            if ui.text_edit_singleline(&mut plugin_name).changed() {
                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].name =
                    plugin_name;
                self.project_modified = true;
            }

            ui.label(&label_desc);
            let mut desc = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                .description
                .clone();
            if ui
                .add_sized([ui.available_width(), 80.0], egui::TextEdit::multiline(&mut desc))
                .changed()
            {
                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                    .description = desc;
                self.project_modified = true;
            }

            let current_type = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins
                [plugin_idx]
                .default_type;
            ui.horizontal(|ui| {
                ui.label(&label_type);
                egui::ComboBox::from_id_salt("plugin_type_combo")
                    .selected_text(current_type.as_str())
                    .show_ui(ui, |ui| {
                        for pt in PluginType::variants() {
                            if ui
                                .selectable_label(current_type == *pt, pt.as_str())
                                .clicked()
                            {
                                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins
                                    [plugin_idx]
                                    .default_type = *pt;
                                self.project_modified = true;
                            }
                        }
                    });
            });

            let img_text = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                .image_path
                .clone()
                .unwrap_or_default();
            ui.horizontal(|ui| {
                ui.label(&label_image);
                ui.label(&img_text);
                if ui.button(&btn_browse).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(self.i18n.t("filter-images"), &["png", "jpg", "jpeg", "bmp"])
                        .pick_file()
                    {
                        if let Some(ref root) = self.root_directory {
                            if let Ok(rel) = path.strip_prefix(root) {
                                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins
                                    [plugin_idx]
                                    .image_path = Some(rel.to_string_lossy().to_string());
                                self.project_modified = true;
                            }
                        }
                    }
                }
                if ui.button(&btn_clear).clicked() {
                    self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                        .image_path = None;
                    self.project_modified = true;
                }
            });

            // Plugin image preview (rendered from disk relative to the root dir)
            {
                let abs_path = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins
                    [plugin_idx]
                    .image_path
                    .as_ref()
                    .and_then(|rel| self.root_directory.as_ref().map(|root| root.join(rel)));
                let fallback = self.i18n.t("image-no-image");
                ImageDisplay::new(220.0, 110.0)
                    .with_fallback(fallback)
                    .show(ui, abs_path.as_deref());
            }

            ui.add_space(8.0);
            section_header(ui, &label_flags);
            self.render_condition_flags(ui, step_idx, group_idx, plugin_idx);

            ui.add_space(8.0);
            section_header(ui, &label_files);
            self.render_plugin_files(ui, step_idx, group_idx, plugin_idx);
        });
    }

    fn render_condition_flags(
        &mut self,
        ui: &mut egui::Ui,
        step_idx: usize,
        group_idx: usize,
        plugin_idx: usize,
    ) {
        let btn_add = self.i18n.t("btn-add-flag");
        let btn_remove = self.i18n.t("btn-remove-flag");

        let flags: Vec<(String, String)> = self.ximod.steps[step_idx].plugin_groups[group_idx]
            .plugins[plugin_idx]
            .condition_flags
            .iter()
            .map(|f| (f.name.clone(), f.value.clone()))
            .collect();

        egui::ScrollArea::vertical()
            .id_salt("flags_list")
            .max_height(80.0)
            .show(ui, |ui| {
                for (idx, (name, value)) in flags.iter().enumerate() {
                    let selected = self.current_flag_index == Some(idx);
                    let text = format!("{} = {}", name, value);
                    if ui.selectable_label(selected, &text).clicked() {
                        self.current_flag_index = Some(idx);
                        self.temp_flag_name = name.clone();
                        self.temp_flag_value = value.clone();
                    }
                }
            });

        let all_flags = self.ximod.get_all_flags();
        let all_flag_values = self.ximod.get_all_flag_values();
        ui.horizontal(|ui| {
            crate::ui::components::autocomplete_edit(
                ui,
                "ac_flag_name",
                &mut self.temp_flag_name,
                &all_flags,
            );
            ui.label("=");
            crate::ui::components::autocomplete_edit(
                ui,
                "ac_flag_value",
                &mut self.temp_flag_value,
                &all_flag_values,
            );
        });

        ui.horizontal(|ui| {
            if ui.button(&btn_add).clicked() && !self.temp_flag_name.is_empty() {
                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                    .condition_flags
                    .push(ConditionFlag::new(
                        self.temp_flag_name.clone(),
                        self.temp_flag_value.clone(),
                    ));
                self.temp_flag_name.clear();
                self.temp_flag_value.clear();
                self.project_modified = true;
            }

            let can_remove = self.current_flag_index.is_some();
            if ui
                .add_enabled(can_remove, egui::Button::new(&btn_remove))
                .clicked()
            {
                if let Some(flag_idx) = self.current_flag_index {
                    self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                        .condition_flags
                        .remove(flag_idx);
                    self.current_flag_index = None;
                    self.project_modified = true;
                }
            }
        });
    }

    fn render_plugin_files(
        &mut self,
        ui: &mut egui::Ui,
        step_idx: usize,
        group_idx: usize,
        plugin_idx: usize,
    ) {
        let btn_add_file = self.i18n.t("btn-add-file");
        let btn_add_folder = self.i18n.t("btn-add-folder");
        let btn_remove = self.i18n.t("btn-remove-file");
        let col_type = self.i18n.t("label-file-type");
        let col_source = self.i18n.t("label-source");
        let col_destination = self.i18n.t("label-destination");
        let col_priority = self.i18n.t("label-priority");

        let files: Vec<(String, String, String, u32)> = self.ximod.steps[step_idx].plugin_groups
            [group_idx]
            .plugins[plugin_idx]
            .files
            .iter()
            .map(|f| {
                (
                    f.file_type.as_str().to_string(),
                    f.source.clone(),
                    f.destination.clone(),
                    f.priority,
                )
            })
            .collect();

        egui::ScrollArea::vertical()
            .id_salt("files_list")
            .max_height(100.0)
            .show(ui, |ui| {
                egui::Grid::new("plugin_files_grid")
                    .num_columns(4)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(RichText::new(&col_type).strong());
                        ui.label(RichText::new(&col_source).strong());
                        ui.label(RichText::new(&col_destination).strong());
                        ui.label(RichText::new(&col_priority).strong());
                        ui.end_row();

                        for (idx, (ftype, src, dst, pri)) in files.iter().enumerate() {
                            let selected = self.current_file_index == Some(idx);
                            if ui.selectable_label(selected, ftype).clicked() {
                                self.current_file_index = Some(idx);
                            }
                            ui.label(src);
                            let mut dst_buf = dst.clone();
                            if ui
                                .add(egui::TextEdit::singleline(&mut dst_buf).desired_width(220.0))
                                .changed()
                            {
                                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins
                                    [plugin_idx]
                                    .files[idx]
                                    .destination = dst_buf;
                                self.project_modified = true;
                            }
                            ui.label(pri.to_string());
                            ui.end_row();
                        }
                    });
            });

        ui.horizontal(|ui| {
            if ui.button(&btn_add_file).clicked() {
                if let Some(paths) = rfd::FileDialog::new().pick_files() {
                    if let Some(ref root) = self.root_directory {
                        for path in paths {
                            if let Ok(rel) = path.strip_prefix(root) {
                                let rel_str = rel.to_string_lossy().to_string();
                                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins
                                    [plugin_idx]
                                    .files
                                    .push(InstallFile::new_file(rel_str));
                                self.project_modified = true;
                            }
                        }
                    }
                }
            }

            if ui.button(&btn_add_folder).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    if let Some(ref root) = self.root_directory {
                        if let Ok(rel) = path.strip_prefix(root) {
                            let rel_str = rel.to_string_lossy().to_string();
                            self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                                .files
                                .push(InstallFile::new_folder(rel_str));
                            self.project_modified = true;
                        }
                    }
                }
            }

            let can_remove = self.current_file_index.is_some();
            if ui
                .add_enabled(can_remove, egui::Button::new(&btn_remove))
                .clicked()
            {
                if let Some(file_idx) = self.current_file_index {
                    self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                        .files
                        .remove(file_idx);
                    self.current_file_index = None;
                    self.project_modified = true;
                }
            }
        });
    }

    fn render_required_tab(&mut self, ui: &mut egui::Ui) {
        let title = self.i18n.t("tab-required");
        let btn_add_file = self.i18n.t("btn-add-file");
        let btn_add_folder = self.i18n.t("btn-add-folder");
        let btn_remove = self.i18n.t("btn-remove-file");
        let col_type = self.i18n.t("label-file-type");
        let col_source = self.i18n.t("label-source");
        let col_destination = self.i18n.t("label-destination");
        let col_priority = self.i18n.t("label-priority");

        section_header(ui, &title);

        let files: Vec<(String, String, String, u32)> = self
            .ximod
            .required_files
            .iter()
            .map(|f| {
                (
                    f.file_type.as_str().to_string(),
                    f.source.clone(),
                    f.destination.clone(),
                    f.priority,
                )
            })
            .collect();

        egui::ScrollArea::vertical()
            .id_salt("req_files_list")
            .max_height(300.0)
            .show(ui, |ui| {
                egui::Grid::new("req_files_grid")
                    .num_columns(4)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(RichText::new(&col_type).strong());
                        ui.label(RichText::new(&col_source).strong());
                        ui.label(RichText::new(&col_destination).strong());
                        ui.label(RichText::new(&col_priority).strong());
                        ui.end_row();

                        for (idx, (ftype, src, dst, pri)) in files.iter().enumerate() {
                            let selected = self.current_req_file_index == Some(idx);
                            if ui.selectable_label(selected, ftype).clicked() {
                                self.current_req_file_index = Some(idx);
                            }
                            ui.label(src);
                            let mut dst_buf = dst.clone();
                            if ui
                                .add(egui::TextEdit::singleline(&mut dst_buf).desired_width(220.0))
                                .changed()
                            {
                                self.ximod.required_files[idx].destination = dst_buf;
                                self.project_modified = true;
                            }
                            ui.label(pri.to_string());
                            ui.end_row();
                        }
                    });
            });

        ui.horizontal(|ui| {
            if ui.button(&btn_add_file).clicked() {
                if let Some(paths) = rfd::FileDialog::new().pick_files() {
                    if let Some(ref root) = self.root_directory {
                        for path in paths {
                            if let Ok(rel) = path.strip_prefix(root) {
                                let rel_str = rel.to_string_lossy().to_string();
                                self.ximod.required_files.push(InstallFile::new_file(rel_str));
                                self.project_modified = true;
                            }
                        }
                    }
                }
            }

            if ui.button(&btn_add_folder).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    if let Some(ref root) = self.root_directory {
                        if let Ok(rel) = path.strip_prefix(root) {
                            let rel_str = rel.to_string_lossy().to_string();
                            self.ximod.required_files.push(InstallFile::new_folder(rel_str));
                            self.project_modified = true;
                        }
                    }
                }
            }

            let can_remove = self.current_req_file_index.is_some();
            if ui
                .add_enabled(can_remove, egui::Button::new(&btn_remove))
                .clicked()
            {
                if let Some(idx) = self.current_req_file_index {
                    self.ximod.required_files.remove(idx);
                    self.current_req_file_index = None;
                    self.project_modified = true;
                }
            }
        });
    }

    fn render_conditional_tab(&mut self, ui: &mut egui::Ui) {
        let title = self.i18n.t("tab-conditional");
        let label_operator = self.i18n.t("label-operator");
        let btn_add_dep = self.i18n.t("btn-add-dependency");
        let btn_remove_dep = self.i18n.t("btn-remove-dependency");
        let btn_add_file = self.i18n.t("btn-add-file");
        let btn_add_folder = self.i18n.t("btn-add-folder");
        let btn_remove_file = self.i18n.t("btn-remove-file");
        let col_type = self.i18n.t("label-file-type");
        let col_source = self.i18n.t("label-source");
        let col_destination = self.i18n.t("label-destination");
        let col_priority = self.i18n.t("label-priority");
        let hdr_dependencies = self.i18n.t("label-dependencies");
        let hdr_files = self.i18n.t("label-files");

        section_header(ui, &title);

        let pattern_count = self.ximod.conditional_files.len();

        // Pre-compute translated pattern labels to avoid borrow conflicts in the closure
        let pattern_labels: Vec<String> = (0..pattern_count)
            .map(|idx| self.i18n.t_num("pattern-label", (idx + 1) as i64))
            .collect();

        ui.horizontal(|ui| {
            for idx in 0..pattern_count {
                let selected = self.current_cond_pattern_index == Some(idx);
                if ui
                    .selectable_label(selected, &pattern_labels[idx])
                    .clicked()
                {
                    self.current_cond_pattern_index = Some(idx);
                }
            }

            if ui.button("➕").clicked() {
                self.ximod.conditional_files.push(ConditionalFileSet::new());
                self.current_cond_pattern_index = Some(self.ximod.conditional_files.len() - 1);
                self.project_modified = true;
            }

            let can_remove = self.current_cond_pattern_index.is_some();
            if ui.add_enabled(can_remove, egui::Button::new("➖")).clicked() {
                if let Some(idx) = self.current_cond_pattern_index {
                    self.ximod.conditional_files.remove(idx);
                    self.current_cond_pattern_index = if self.ximod.conditional_files.is_empty() {
                        None
                    } else {
                        Some(idx.saturating_sub(1).min(self.ximod.conditional_files.len() - 1))
                    };
                    self.project_modified = true;
                }
            }
        });

        ui.separator();

        if let Some(pattern_idx) = self.current_cond_pattern_index {
            if pattern_idx >= self.ximod.conditional_files.len() {
                return;
            }

            let current_op = self.ximod.conditional_files[pattern_idx].operator;
            ui.horizontal(|ui| {
                ui.label(&label_operator);
                egui::ComboBox::from_id_salt("cond_operator")
                    .selected_text(current_op.as_str())
                    .show_ui(ui, |ui| {
                        for op in LogicalOperator::variants() {
                            if ui
                                .selectable_label(current_op == *op, op.as_str())
                                .clicked()
                            {
                                self.ximod.conditional_files[pattern_idx].operator = *op;
                                self.project_modified = true;
                            }
                        }
                    });
            });

            subsection_header(ui, &hdr_dependencies);

            let deps: Vec<String> = self.ximod.conditional_files[pattern_idx]
                .dependencies
                .iter()
                .map(|d| d.display_name())
                .collect();

            egui::ScrollArea::vertical()
                .id_salt("cond_deps_list")
                .max_height(100.0)
                .show(ui, |ui| {
                    for (idx, dep) in deps.iter().enumerate() {
                        let selected = self.current_dependency_index == Some(idx);
                        if ui.selectable_label(selected, dep).clicked() {
                            self.current_dependency_index = Some(idx);
                        }
                    }
                });

            let temp_dep_type = self.temp_dep_type.clone();
            let all_dep_names = self.ximod.get_all_dependency_names();
            let dep_value_candidates: Vec<String> = if temp_dep_type == "file" {
                crate::models::FileState::variants()
                    .iter()
                    .map(|s| s.as_str().to_string())
                    .collect()
            } else {
                self.ximod.get_all_flag_values()
            };
            let dep_type_flag = self.i18n.t("dep-type-flag");
            let dep_type_file = self.i18n.t("dep-type-file");
            // Display label matching the current internal value ("flag" / "file")
            let dep_type_display = if temp_dep_type == "file" {
                dep_type_file.clone()
            } else {
                dep_type_flag.clone()
            };
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("dep_type")
                    .selected_text(&dep_type_display)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(temp_dep_type == "flag", &dep_type_flag)
                            .clicked()
                        {
                            self.temp_dep_type = "flag".to_string();
                        }
                        if ui
                            .selectable_label(temp_dep_type == "file", &dep_type_file)
                            .clicked()
                        {
                            self.temp_dep_type = "file".to_string();
                        }
                    });
                crate::ui::components::autocomplete_edit(
                    ui,
                    "ac_dep_name",
                    &mut self.temp_dep_name,
                    &all_dep_names,
                );
                ui.label("=");
                crate::ui::components::autocomplete_edit(
                    ui,
                    "ac_dep_value",
                    &mut self.temp_dep_value,
                    &dep_value_candidates,
                );

                if ui.button(&btn_add_dep).clicked() && !self.temp_dep_name.is_empty() {
                    let dep = Dependency {
                        dep_type: self.temp_dep_type.clone(),
                        name: self.temp_dep_name.clone(),
                        value: self.temp_dep_value.clone(),
                    };
                    self.ximod.conditional_files[pattern_idx].dependencies.push(dep);
                    self.temp_dep_name.clear();
                    self.temp_dep_value.clear();
                    self.project_modified = true;
                }

                let can_remove = self.current_dependency_index.is_some();
                if ui
                    .add_enabled(can_remove, egui::Button::new(&btn_remove_dep))
                    .clicked()
                {
                    if let Some(dep_idx) = self.current_dependency_index {
                        self.ximod.conditional_files[pattern_idx]
                            .dependencies
                            .remove(dep_idx);
                        self.current_dependency_index = None;
                        self.project_modified = true;
                    }
                }
            });

            ui.add_space(8.0);
            subsection_header(ui, &hdr_files);

            let files: Vec<(String, String, String, u32)> = self.ximod.conditional_files
                [pattern_idx]
                .files
                .iter()
                .map(|f| {
                    (
                        f.file_type.as_str().to_string(),
                        f.source.clone(),
                        f.destination.clone(),
                        f.priority,
                    )
                })
                .collect();

            egui::ScrollArea::vertical()
                .id_salt("cond_files_list")
                .max_height(150.0)
                .show(ui, |ui| {
                    egui::Grid::new("cond_files_grid")
                        .num_columns(4)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label(RichText::new(&col_type).strong());
                            ui.label(RichText::new(&col_source).strong());
                            ui.label(RichText::new(&col_destination).strong());
                            ui.label(RichText::new(&col_priority).strong());
                            ui.end_row();

                            for (idx, (ftype, src, dst, pri)) in files.iter().enumerate() {
                                let selected = self.current_cond_file_index == Some(idx);
                                if ui.selectable_label(selected, ftype).clicked() {
                                    self.current_cond_file_index = Some(idx);
                                }
                                ui.label(src);
                                let mut dst_buf = dst.clone();
                                if ui
                                    .add(egui::TextEdit::singleline(&mut dst_buf).desired_width(220.0))
                                    .changed()
                                {
                                    self.ximod.conditional_files[pattern_idx].files[idx]
                                        .destination = dst_buf;
                                    self.project_modified = true;
                                }
                                ui.label(pri.to_string());
                                ui.end_row();
                            }
                        });
                });

            ui.horizontal(|ui| {
                if ui.button(&btn_add_file).clicked() {
                    if let Some(paths) = rfd::FileDialog::new().pick_files() {
                        if let Some(ref root) = self.root_directory {
                            for path in paths {
                                if let Ok(rel) = path.strip_prefix(root) {
                                    let rel_str = rel.to_string_lossy().to_string();
                                    self.ximod.conditional_files[pattern_idx]
                                        .files
                                        .push(InstallFile::new_file(rel_str));
                                    self.project_modified = true;
                                }
                            }
                        }
                    }
                }

                if ui.button(&btn_add_folder).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        if let Some(ref root) = self.root_directory {
                            if let Ok(rel) = path.strip_prefix(root) {
                                let rel_str = rel.to_string_lossy().to_string();
                                self.ximod.conditional_files[pattern_idx]
                                    .files
                                    .push(InstallFile::new_folder(rel_str));
                                self.project_modified = true;
                            }
                        }
                    }
                }

                let can_remove = self.current_cond_file_index.is_some();
                if ui
                    .add_enabled(can_remove, egui::Button::new(&btn_remove_file))
                    .clicked()
                {
                    if let Some(file_idx) = self.current_cond_file_index {
                        self.ximod.conditional_files[pattern_idx].files.remove(file_idx);
                        self.current_cond_file_index = None;
                        self.project_modified = true;
                    }
                }
            });
        }
    }

    fn render_settings_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }

        // Modal overlay - blocks interaction with main window
        let screen_rect = ctx.screen_rect();
        let modal_layer = egui::LayerId::new(egui::Order::Middle, egui::Id::new("settings_modal_bg"));
        let painter = ctx.layer_painter(modal_layer);
        painter.rect_filled(
            screen_rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 128),
        );
        
        // Capture all mouse events on the modal layer
        let _modal_response = egui::Area::new(egui::Id::new("modal_capture"))
            .order(egui::Order::Middle)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.allocate_response(screen_rect.size(), egui::Sense::click_and_drag());
            });

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
        let label_max_recent = self.i18n.t("settings-max-recent");
        let label_window_width = self.i18n.t("settings-window-width");
        let label_window_height = self.i18n.t("settings-window-height");
        let label_no_recent = self.i18n.t("settings-no-recent-files");
        let btn_save = self.i18n.t("btn-save");
        let btn_cancel = self.i18n.t("btn-cancel");
        let btn_clear = self.i18n.t("btn-clear");
        let _btn_remove = self.i18n.t("btn-remove");

        let available_locales: Vec<String> = self
            .i18n
            .available_locales()
            .iter()
            .map(|s| s.to_string())
            .collect();

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

        // Clear any focus from main window widgets
        ctx.memory_mut(|mem| mem.surrender_focus(egui::Id::NULL));

        // Use input_mut to consume keyboard events
        ctx.input_mut(|i| {
            // Consume and handle Tab navigation
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Tab) {
                // Tab: next
                self.settings_focus += 1;
                if self.settings_focus > max_focus {
                    self.settings_focus = 0;
                }
                // Skip controls not in current tab
                if self.settings_tab == SettingsTab::RecentFiles && self.settings_focus > FOCUS_TAB_RECENT && self.settings_focus < FOCUS_CLEAR {
                    self.settings_focus = FOCUS_CLEAR;
                }
            }
            
            if i.consume_key(egui::Modifiers::SHIFT, egui::Key::Tab) {
                // Shift+Tab: previous
                if self.settings_focus == 0 {
                    self.settings_focus = max_focus;
                } else {
                    self.settings_focus -= 1;
                }
                // Skip controls not in current tab
                if self.settings_tab == SettingsTab::RecentFiles && self.settings_focus > FOCUS_TAB_RECENT && self.settings_focus < FOCUS_CLEAR {
                    self.settings_focus = FOCUS_TAB_RECENT;
                }
            }

            // Arrow key navigation for combo boxes and number fields
            if self.settings_tab == SettingsTab::General {
                match self.settings_focus {
                    FOCUS_LANGUAGE => {
                        if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) || 
                           i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                            let idx = available_locales.iter().position(|l| l == &self.temp_locale).unwrap_or(0);
                            if idx > 0 {
                                self.temp_locale = available_locales[idx - 1].clone();
                            }
                        }
                        if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) || 
                           i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                            let idx = available_locales.iter().position(|l| l == &self.temp_locale).unwrap_or(0);
                            if idx < available_locales.len() - 1 {
                                self.temp_locale = available_locales[idx + 1].clone();
                            }
                        }
                    }
                    FOCUS_THEME => {
                        let themes = Theme::variants();
                        let idx = themes.iter().position(|t| *t == self.temp_theme).unwrap_or(0);
                        if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) || 
                           i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                            if idx > 0 {
                                self.temp_theme = themes[idx - 1];
                            }
                        }
                        if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) || 
                           i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                            if idx < themes.len() - 1 {
                                self.temp_theme = themes[idx + 1];
                            }
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
                    FOCUS_REPLACE_NEWLINES => {
                        if i.consume_key(egui::Modifiers::NONE, egui::Key::Space) {
                            self.temp_replace_newlines = !self.temp_replace_newlines;
                        }
                    }
                    _ => {}
                }
            }

            // Tab switching with Space/Enter
            if self.settings_focus == FOCUS_TAB_GENERAL {
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Space) || 
                   i.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                    self.settings_tab = SettingsTab::General;
                }
            }
            if self.settings_focus == FOCUS_TAB_RECENT {
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Space) || 
                   i.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                    self.settings_tab = SettingsTab::RecentFiles;
                }
            }

            // Enter for buttons
            if self.settings_focus == FOCUS_SAVE {
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                    should_save = true;
                    should_close = true;
                }
            }
            if self.settings_focus == FOCUS_CLEAR && self.settings_tab == SettingsTab::RecentFiles {
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                    clear_recent = true;
                }
            }
            if self.settings_focus == FOCUS_CANCEL {
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                    should_close = true;
                }
            }

            // Escape to close
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                should_close = true;
            }
        });

        // Helper function for focused style
        let focused_stroke = egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(100, 149, 237));
        
        egui::Window::new(&title)
            .collapsible(false)
            // Exact, constant size: the Recent Files tab lists full paths, which
            // would otherwise widen the window (and egui remembers the enlarged
            // size, so it never came back). Long paths are truncated instead.
            .resizable(false)
            .fixed_size([770.0, 495.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
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
                    
                    let tab_rec_response = ui.selectable_label(self.settings_tab == SettingsTab::RecentFiles, &tab_recent);
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

                // Fixed-height content area: both tabs always occupy exactly the
                // same vertical space. The window therefore keeps a constant size
                // instead of growing on the longer tab and never shrinking back
                // (egui remembers a window's enlarged size).
                let content_size = egui::vec2(ui.available_width(), 280.0);
                ui.allocate_ui(content_size, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
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
                                        .and_then(|f| {
                                            crate::data::flags_dir().map(|d| d.join(f))
                                        })
                                        .filter(|p| p.is_file());

                                    let flag_resp = match &flag_path {
                                        Some(p) => ui.add_sized(
                                            flag_size,
                                            egui::ImageButton::new(
                                                egui::Image::from_uri(format!(
                                                    "file://{}",
                                                    p.display()
                                                ))
                                                .fit_to_exact_size(flag_size),
                                            ),
                                        ),
                                        None => ui.add_sized(
                                            flag_size,
                                            egui::Button::new(
                                                egui::RichText::new("\u{1F3F3}").size(28.0),
                                            ),
                                        ),
                                    };
                                    if flag_resp
                                        .on_hover_text(&label_pick_country)
                                        .clicked()
                                    {
                                        self.flag_target =
                                            crate::ui::flag_picker::FlagTarget::Settings;
                                        self.flag_filter.clear();
                                        self.flag_cursor = 0;
                                        self.flag_scroll_offset = 0.0;
                                        self.show_flag_picker = true;
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
                                        let country_langs: Vec<String> = self
                                            .country_languages
                                            .languages_for(&self.temp_country)
                                            .to_vec();
                                        let lang_enabled = !country_langs.is_empty();

                                        ui.horizontal(|ui| {
                                            ui.label(&label_lang);
                                            let current_display =
                                                self.i18n.display_name(&self.temp_locale);
                                            ui.add_enabled_ui(lang_enabled, |ui| {
                                                let combo =
                                                    egui::ComboBox::from_id_salt("lang_combo")
                                                        .selected_text(current_display)
                                                        .height(260.0)
                                                        .show_ui(ui, |ui| {
                                                            egui::ScrollArea::both()
                                                                .auto_shrink([false, false])
                                                                .show(ui, |ui| {
                                                                    for locale in &country_langs {
                                                                        let display = self
                                                                            .i18n
                                                                            .display_name(locale);
                                                                        if ui
                                                                            .selectable_label(
                                                                                self.temp_locale
                                                                                    == *locale,
                                                                                display,
                                                                            )
                                                                            .clicked()
                                                                        {
                                                                            self.temp_locale =
                                                                                locale.clone();
                                                                        }
                                                                    }
                                                                });
                                                        });
                                                if self.settings_focus == FOCUS_LANGUAGE {
                                                    ui.painter().rect_stroke(
                                                        combo.response.rect,
                                                        2.0,
                                                        focused_stroke,
                                                    );
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
                                                .selected_text(self.temp_theme.as_str())
                                                .show_ui(ui, |ui| {
                                                    for theme in Theme::variants() {
                                                        if ui
                                                            .selectable_label(
                                                                self.temp_theme == *theme,
                                                                theme.as_str(),
                                                            )
                                                            .clicked()
                                                        {
                                                            self.temp_theme = *theme;
                                                        }
                                                    }
                                                });
                                            if self.settings_focus == FOCUS_THEME {
                                                ui.painter().rect_stroke(
                                                    combo.response.rect,
                                                    2.0,
                                                    focused_stroke,
                                                );
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
                                                ui.painter().rect_stroke(
                                                    drag.rect,
                                                    2.0,
                                                    focused_stroke,
                                                );
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

                                ui.add_space(12.0);
                                ui.separator();
                                ui.add_space(8.0);

                                // Max recent files
                                ui.horizontal(|ui| {
                                    ui.label(&label_max_recent);
                                    let mut max_recent = self.temp_max_recent_files as i32;
                                    let drag = ui.add(egui::DragValue::new(&mut max_recent)
                                        .range(1..=20)
                                        .speed(0.2));
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
                                    let drag = ui.add(egui::DragValue::new(&mut self.temp_window_width)
                                        .range(800.0..=3840.0)
                                        .speed(10.0));
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
                                    let drag = ui.add(egui::DragValue::new(&mut self.temp_window_height)
                                        .range(600.0..=2160.0)
                                        .speed(10.0));
                                    if self.settings_focus == FOCUS_WINDOW_HEIGHT {
                                        ui.painter().rect_stroke(drag.rect, 2.0, focused_stroke);
                                    }
                                    if drag.clicked() {
                                        self.settings_focus = FOCUS_WINDOW_HEIGHT;
                                    }
                                });
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
                                    let path_w = (content_size.x - 84.0).max(60.0);
                                    for (idx, path) in self.config.recent_files.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{}.", idx + 1));
                                            // Button first: the path label takes
                                            // all remaining width, which would
                                            // otherwise push it out of view.
                                            if crate::ui::components::delete_button(ui).clicked() {
                                                remove_index = Some(idx);
                                            }
                                            // Keep the *end* of the path visible
                                            // (mod name) rather than the drive.
                                            let full = path.display().to_string();
                                            let shown = elide_start(ui, &full, path_w);
                                            ui.add(
                                                egui::Label::new(shown)
                                                    .wrap_mode(egui::TextWrapMode::Truncate),
                                            )
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

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

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
            });

        // Handle recent files modifications
        if let Some(idx) = remove_index {
            if idx < self.config.recent_files.len() {
                self.config.recent_files.remove(idx);
                let _ = self.config.save();
            }
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
            self.config.theme = self.temp_theme;
            self.config.font_size = self.temp_font_size;
            self.config.replace_newlines = self.temp_replace_newlines;
            self.config.max_recent_files = self.temp_max_recent_files;
            self.config.window_width = self.temp_window_width;
            self.config.window_height = self.temp_window_height;

            self.i18n.set_locale(&self.temp_locale);
            self.apply_theme(ctx);
            
            // Force complete UI rebuild when locale changes
            if locale_changed {
                // Increment locale version to create new menu IDs
                self.locale_version = self.locale_version.wrapping_add(1);
                
                // Clear cached UI state - this forces menus to recalculate their sizes
                ctx.memory_mut(|mem| {
                    mem.data.clear();
                });
                
                // Force immediate repaint with new IDs
                ctx.request_repaint();
            }

            match self.config.save() {
                Err(e) => {
                    self.status_message = format!("{}: {}", self.i18n.t("msg-settings-save-error"), e);
                }
                _ => {
                    self.status_message = self.i18n.t("status-settings-saved");
                }
            }
        }

        // Escape closes the settings without saving (same as Cancel).
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            should_close = true;
        }

        if should_close {
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
            }
            self.show_settings = false;
            self.settings_tab = SettingsTab::General;
            self.settings_focus = 0;
        }
    }

    fn render_about_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_about {
            return;
        }

        let title = self.i18n.t("about-title");
        let app_name = self.i18n.t("app-title");
        let version_str = self.i18n.t_arg("app-version", "version", env!("CARGO_PKG_VERSION"));
        let desc = self.i18n.t("about-description");
        let license = self.i18n.t("about-license");
        let copyright = self.i18n.t("about-copyright");
        let credit = self.i18n.t("about-credit");
        let ok_text = self.i18n.t("btn-ok");

        let mut should_close = false;

        egui::Window::new(&title)
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(&app_name);
                    ui.label(&version_str);
                    ui.add_space(8.0);
                    ui.label(&desc);
                    ui.add_space(8.0);
                    ui.label(&license);
                    ui.add_space(4.0);
                    ui.label(&copyright);
                    ui.add_space(12.0);
                    // Credit to the original author (as agreed with Wenderer):
                    // a line of text plus a clickable link to the original tool.
                    ui.label(&credit);
                    ui.hyperlink_to(
                        "Wenderer — FOMOD Creation Tool",
                        "https://www.nexusmods.com/fallout4/mods/6821",
                    );
                    ui.add_space(16.0);
                    if ui.button(&ok_text).clicked() {
                        should_close = true;
                    }
                });
            });

        if should_close
            || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.show_about = false;
        }
    }

    fn render_script_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_script_dialog {
            return;
        }

        let title = if self.editing_pre_script {
            self.i18n.t("menu-pre-save-script")
        } else {
            self.i18n.t("menu-post-save-script")
        };
        let info = self.i18n.t("script-info");
        let macros_title = self.i18n.t("script-macros");
        let btn_save = self.i18n.t("btn-save");
        let btn_cancel = self.i18n.t("btn-cancel");
        let macro_lines = [
            self.i18n.t("macro-modname"),
            self.i18n.t("macro-modauthor"),
            self.i18n.t("macro-modversion"),
            self.i18n.t("macro-modroot"),
            self.i18n.t("macro-date"),
            self.i18n.t("macro-time"),
            self.i18n.t("macro-random"),
        ];

        let mut should_close = false;
        let mut should_save = false;
        let mut content = self.script_content.clone();

        egui::Window::new(&title)
            .collapsible(false)
            .resizable(true)
            .default_width(500.0)
            .show(ctx, |ui| {
                ui.label(&info);

                ui.collapsing(&macros_title, |ui| {
                    for line in &macro_lines {
                        ui.label(line);
                    }
                });

                ui.add_space(8.0);

                ui.add_sized(
                    [ui.available_width(), 200.0],
                    egui::TextEdit::multiline(&mut content).font(egui::TextStyle::Monospace),
                );

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    if ui.button(&btn_save).clicked() {
                        should_save = true;
                        should_close = true;
                    }
                    if ui.button(&btn_cancel).clicked() {
                        should_close = true;
                    }
                });
            });

        self.script_content = content;

        if should_save {
            // Save script content directly to config
            if self.editing_pre_script {
                self.config.pre_save_script = self.script_content.clone();
            } else {
                self.config.post_save_script = self.script_content.clone();
            }
            match self.config.save() {
                Err(e) => {
                    self.status_message = format!("{}: {}", self.i18n.t("msg-script-save-error"), e);
                }
                _ => {
                    self.status_message = self.i18n.t("status-settings-saved");
                }
            }
        }

        // Escape closes the script editor without saving (same as Cancel).
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            should_close = true;
        }
        if should_close {
            self.show_script_dialog = false;
        }
    }

    /// The FOMOD tab strip (one tab per open document).
    fn render_fomod_tabs(&mut self, ctx: &egui::Context, modal_open: bool) {
        let active = self.active_doc;
        let untitled = self.i18n.t("tab-untitled");
        let close_hint = self.i18n.t("tab-close-hint");
        // Precompute (label, tooltip, modified) to avoid borrow issues.
        let entries: Vec<(String, String, bool)> = self
            .docs
            .iter()
            .map(|d| {
                let full = Self::doc_full_name(d);
                let mut label = elide_tab(&full, 22);
                if label.is_empty() {
                    label = untitled.clone();
                }
                let tooltip = d
                    .root_directory
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| if full.is_empty() { untitled.clone() } else { full.clone() });
                (label, tooltip, d.modified)
            })
            .collect();

        let mut switch_to: Option<usize> = None;
        let mut close_tab: Option<usize> = None;
        egui::TopBottomPanel::top("fomod_tabs").show(ctx, |ui| {
            if modal_open {
                ui.disable();
            }
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, (label, tooltip, modified)) in entries.iter().enumerate() {
                        let text = if *modified {
                            format!("● {label}")
                        } else {
                            label.clone()
                        };
                        if ui
                            .selectable_label(i == active, text)
                            .on_hover_text(tooltip)
                            .clicked()
                        {
                            switch_to = Some(i);
                        }
                        // Per-tab close button (delete icon) with save check.
                        if crate::ui::components::delete_button(ui)
                            .on_hover_text(&close_hint)
                            .clicked()
                        {
                            close_tab = Some(i);
                        }
                        ui.separator();
                    }
                });
            });
        });
        // Closing takes precedence over a plain selection click.
        if let Some(i) = close_tab {
            self.switch_doc(i);
            self.close_active_fomod();
        } else if let Some(i) = switch_to {
            self.switch_doc(i);
        }
    }

    /// Open any FOMOD folder / config file dropped onto the window.
    fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        let dropped: Vec<PathBuf> = ctx.input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone())
                .collect()
        });
        for p in dropped {
            match fomod_root_from_drop(&p) {
                Some(root) => self.load_project(root),
                None => self.status_message = self.i18n.t("msg-drop-not-fomod"),
            }
        }
    }

    /// The unsaved-changes prompt shown when closing XIMOD (Yes / No / Cancel).
    fn render_exit_prompt(&mut self, ctx: &egui::Context) {
        if !self.show_exit_prompt {
            return;
        }
        let title = self.i18n.t("exit-title");
        let message = self.i18n.t("exit-unsaved");
        let yes = self.i18n.t("btn-yes");
        let no = self.i18n.t("btn-no");
        let cancel = self.i18n.t("btn-cancel");
        let mut choice = 0u8;
        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(message);
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button(yes).clicked() {
                        choice = 1;
                    }
                    if ui.button(no).clicked() {
                        choice = 2;
                    }
                    if ui.button(cancel).clicked() {
                        choice = 3;
                    }
                });
            });
        match choice {
            1 => {
                self.save_all_modified();
                self.show_exit_prompt = false;
                self.exit_confirmed = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            2 => {
                self.show_exit_prompt = false;
                self.exit_confirmed = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            3 => self.show_exit_prompt = false,
            _ => {}
        }
    }

    /// Confirmation shown when closing a modified FOMOD (Yes / No / Cancel).
    fn render_close_prompt(&mut self, ctx: &egui::Context) {
        let Some(scope) = self.close_prompt else {
            return;
        };
        let title = self.i18n.t("exit-title");
        let message = self.i18n.t("exit-unsaved");
        let yes = self.i18n.t("btn-yes");
        let no = self.i18n.t("btn-no");
        let cancel = self.i18n.t("btn-cancel");
        let mut choice = 0u8;
        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(message);
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button(yes).clicked() {
                        choice = 1;
                    }
                    if ui.button(no).clicked() {
                        choice = 2;
                    }
                    if ui.button(cancel).clicked() {
                        choice = 3;
                    }
                });
            });
        match choice {
            1 => {
                // Save, then close.
                match scope {
                    CloseScope::Active => {
                        if self.root_directory.is_some() {
                            self.write_project();
                        }
                        self.close_active_fomod_force();
                    }
                    CloseScope::All => {
                        self.save_all_modified();
                        self.close_all_fomods_force();
                    }
                }
                self.close_prompt = None;
            }
            2 => {
                // Close without saving.
                match scope {
                    CloseScope::Active => self.close_active_fomod_force(),
                    CloseScope::All => self.close_all_fomods_force(),
                }
                self.close_prompt = None;
            }
            3 => self.close_prompt = None,
            _ => {}
        }
    }
}

/// Elide a string to `max` characters, appending an ellipsis when truncated.
fn elide_tab(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        s.to_string()
    } else {
        let mut t: String = chars[..max.saturating_sub(1)].iter().collect();
        t.push('…');
        t
    }
}

/// Resolve a dropped path to a FOMOD root (the folder that contains `fomod/`).
fn fomod_root_from_drop(path: &std::path::Path) -> Option<PathBuf> {
    if path.is_dir() {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name.eq_ignore_ascii_case("fomod") {
            return path.parent().map(|p| p.to_path_buf());
        }
        if path.join("fomod").is_dir() {
            return Some(path.to_path_buf());
        }
        return None;
    }
    if path.is_file() {
        // A FOMOD xml lives in <root>/fomod/ModuleConfig.xml (or info.xml).
        let parent = path.parent()?;
        let pname = parent.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if pname.eq_ignore_ascii_case("fomod") {
            return parent.parent().map(|p| p.to_path_buf());
        }
        return parent.parent().map(|p| p.to_path_buf());
    }
    None
}

impl eframe::App for XimodApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme on first frame (after eframe initialization is complete)
        if !self.theme_applied {
            self.apply_theme(ctx);
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

        // Keep the active document's slot in sync so the tab strip, save-state
        // and exit check all see the live state.
        self.ensure_doc();
        self.commit_active();

        // A graceful close requested from the menu / Ctrl+Q.
        if self.request_close {
            self.request_close = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Files dropped onto the window open the corresponding FOMOD(s).
        self.handle_dropped_files(ctx);

        // Check if a modal dialog is open
        let modal_open = self.show_settings || self.show_about || self.show_script_dialog || self.show_confirm || self.show_exit_prompt || self.close_prompt.is_some() || (self.show_xml_editor && self.xml_editor_editing);

        self.render_menu_bar(ctx);

        // Document (FOMOD) tab strip.
        self.render_fomod_tabs(ctx, modal_open);

        // Global keyboard shortcuts (Ctrl+N, Ctrl+S, …) matching the menu.
        self.handle_menu_shortcuts(ctx);

        let tab_info = self.i18n.t("tab-info");
        let tab_steps = self.i18n.t("tab-steps");
        let tab_required = self.i18n.t("tab-required");
        let tab_conditional = self.i18n.t("tab-conditional");

        egui::CentralPanel::default().show(ctx, |ui| {
            // Disable all main window controls when modal is open
            if modal_open {
                ui.disable();
            }
            
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.current_tab == Tab::Info, &tab_info)
                    .clicked()
                {
                    self.current_tab = Tab::Info;
                }
                if ui
                    .selectable_label(self.current_tab == Tab::Steps, &tab_steps)
                    .clicked()
                {
                    self.current_tab = Tab::Steps;
                }
                if ui
                    .selectable_label(self.current_tab == Tab::RequiredInstalls, &tab_required)
                    .clicked()
                {
                    self.current_tab = Tab::RequiredInstalls;
                }
                if ui
                    .selectable_label(self.current_tab == Tab::ConditionalInstalls, &tab_conditional)
                    .clicked()
                {
                    self.current_tab = Tab::ConditionalInstalls;
                }
            });

            ui.separator();

            match self.current_tab {
                Tab::Info => self.render_info_tab(ui),
                Tab::Steps => self.render_steps_tab(ui),
                Tab::RequiredInstalls => self.render_required_tab(ui),
                Tab::ConditionalInstalls => self.render_conditional_tab(ui),
            }
        });

        let status = self.status_message.clone();
        let modified = self.project_modified;
        let modified_text = self.i18n.t("status-modified");

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            if modal_open {
                ui.disable();
            }
            status_bar(ui, &status, modified, &modified_text);
        });

        self.render_settings_dialog(ctx);
        self.render_about_dialog(ctx);
        self.render_script_dialog(ctx);
        self.render_confirm_dialog(ctx);
        self.render_translation_window(ctx);
        self.render_xml_editor(ctx);
        self.render_preview(ctx);
        self.render_validation_report(ctx);
        self.render_properties(ctx);
        self.render_flag_picker(ctx);
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

        // Update window size in config (for saving on exit)
        let screen_rect = ctx.screen_rect();
        self.config.window_width = screen_rect.width();
        self.config.window_height = screen_rect.height();
    }

    /// Called when the application is about to close
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
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
