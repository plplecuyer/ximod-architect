//! Multi-document (FOMOD tab) handling: open / switch / close / save of
//! the documents, plus the tab strip and drag-and-drop opening.

use super::main_window::{ConfirmAction, Tab, XimodApp};
use super::selection::Selection;
use crate::config::ScriptMacros;
use crate::models::*;
use crate::xml;
use eframe::egui;
use fluent::FluentArgs;
use std::path::PathBuf;

/// One open FOMOD document (one tab). Snapshots the active working copy so a
/// tab can be left and returned to without losing its data or view.
#[derive(Clone)]
pub struct DocState {
    pub ximod: Ximod,
    pub root_directory: Option<PathBuf>,
    pub modified: bool,
    /// Undo / redo stacks of this document.
    pub history: crate::ui::history::History,
    pub tab: Tab,
    pub selection: Selection,
    pub file: Option<usize>,
    pub flag: Option<usize>,
    pub dependency: Option<usize>,
    pub cond_file: Option<usize>,
    pub req_file: Option<usize>,
    pub plugin_pattern: Option<usize>,
    pub plugin_dep: Option<usize>,
    pub visibility_dep: Option<usize>,
    pub module_dep: Option<usize>,
    /// Constructs of the loaded `ModuleConfig.xml` the model cannot keep
    /// (see `xml::fidelity`); re-reported by every validation run.
    pub import_report: Vec<crate::xml::fidelity::Unmodelled>,
    /// The Problems panel of this document: its findings, whether the panel
    /// is shown/expanded, the measured sizes, and a validation still running
    /// for it (polled again when the tab is active).
    pub validation_issues: Vec<crate::ui::problems::Issue>,
    pub show_problems: bool,
    pub problems_expanded: bool,
    pub sizes: Option<crate::models::simulate::SizeReport>,
    pub validation_rx: Option<crate::ui::jobs::PendingValidation>,
}

impl DocState {
    /// A pristine blank document (no root, nothing selected).
    pub(crate) fn blank() -> Self {
        Self {
            ximod: Ximod::default(),
            root_directory: None,
            modified: false,
            history: Default::default(),
            tab: Tab::Info,
            selection: Selection::default(),
            file: None,
            flag: None,
            dependency: None,
            cond_file: None,
            req_file: None,
            plugin_pattern: None,
            plugin_dep: None,
            visibility_dep: None,
            module_dep: None,
            import_report: Vec::new(),
            validation_issues: Vec::new(),
            show_problems: false,
            problems_expanded: true,
            sizes: None,
            validation_rx: None,
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

impl XimodApp {
    // ---- Multi-FOMOD document (tab) helpers ----

    /// Start a new blank project. With FOMOD tabs this adds a new tab (replacing
    /// the current one only when it is a pristine blank project), so nothing is
    /// discarded and no confirmation is needed.
    pub(crate) fn new_project(&mut self) {
        self.ensure_doc();
        if !self.active_is_pristine() {
            self.commit_active();
            self.docs.push(DocState::blank());
            self.active_doc = self.docs.len() - 1;
        }
        self.ximod = Ximod::default();
        self.history.reset(&self.ximod);
        self.root_directory = None;
        self.project_modified = false;
        self.import_report.clear();
        self.reset_problems();
        self.reset_navigation();
        self.commit_active();
        self.status_message = self.i18n.t("status-ready");
    }

    /// Shared by the "New" menu item and the Ctrl+N shortcut.
    pub(crate) fn request_new_project(&mut self) {
        self.new_project();
    }

    /// Snapshot the active working copy into a `DocState`.
    pub(crate) fn make_doc_snapshot(&self) -> DocState {
        DocState {
            ximod: self.ximod.clone(),
            root_directory: self.root_directory.clone(),
            modified: self.project_modified,
            // Cloned, not taken: `commit_active` is also called before a
            // save or an exit check, after which the user keeps editing the
            // same document and must keep its undo stack.
            history: self.history.clone(),
            tab: self.current_tab,
            selection: self.selection,
            file: self.current_file_index,
            flag: self.current_flag_index,
            dependency: self.current_dependency_index,
            cond_file: self.current_cond_file_index,
            req_file: self.current_req_file_index,
            plugin_pattern: self.current_plugin_pattern_index,
            plugin_dep: self.current_plugin_dep_index,
            visibility_dep: self.current_visibility_dep_index,
            module_dep: self.current_module_dep_index,
            import_report: self.import_report.clone(),
            validation_issues: self.validation_issues.clone(),
            show_problems: self.show_problems,
            problems_expanded: self.problems_expanded,
            sizes: self.sizes.clone(),
            validation_rx: self.validation_rx.clone(),
        }
    }

    /// Guarantee at least one document, mirroring the initial working copy.
    pub(crate) fn ensure_doc(&mut self) {
        if self.docs.is_empty() {
            self.docs.push(self.make_doc_snapshot());
            self.active_doc = 0;
        } else if self.active_doc >= self.docs.len() {
            self.active_doc = self.docs.len() - 1;
        }
    }

    /// Copy the active working copy into its `docs` slot.
    pub(crate) fn commit_active(&mut self) {
        self.ensure_doc();
        let snap = self.make_doc_snapshot();
        self.docs[self.active_doc] = snap;
    }

    /// Load `docs[active_doc]` into the active working copy.
    pub(crate) fn checkout_active(&mut self) {
        self.ensure_doc();
        // Take the slot's content instead of cloning it: the slot is rewritten
        // by the next `commit_active` anyway, and a deep clone of a large
        // project here is pure waste.
        let d = std::mem::replace(&mut self.docs[self.active_doc], DocState::blank());
        self.project_revision = self.project_revision.wrapping_add(1);
        self.ximod = d.ximod;
        self.history = d.history;
        self.root_directory = d.root_directory;
        self.project_modified = d.modified;
        self.current_tab = d.tab;
        self.selection = d.selection;
        self.current_file_index = d.file;
        self.current_flag_index = d.flag;
        self.current_dependency_index = d.dependency;
        self.current_cond_file_index = d.cond_file;
        self.current_req_file_index = d.req_file;
        self.current_plugin_pattern_index = d.plugin_pattern;
        self.current_plugin_dep_index = d.plugin_dep;
        self.current_visibility_dep_index = d.visibility_dep;
        self.current_module_dep_index = d.module_dep;
        self.import_report = d.import_report;
        self.validation_issues = d.validation_issues;
        self.show_problems = d.show_problems;
        self.problems_expanded = d.problems_expanded;
        self.sizes = d.sizes;
        // A validation still running for this document was started from a
        // revision counter that the tab switches have bumped since: re-stamp
        // it with the current revision so its report is not called stale.
        self.validation_rx = d.validation_rx.map(|rc| match std::rc::Rc::try_unwrap(rc) {
            Ok((rx, _)) => std::rc::Rc::new((rx, self.project_revision)),
            Err(rc) => rc,
        });
    }

    /// Switch the active document to `index`.
    pub(crate) fn switch_doc(&mut self, index: usize) {
        if index >= self.docs.len() || index == self.active_doc {
            return;
        }
        self.commit_active();
        self.active_doc = index;
        self.checkout_active();
    }

    /// True when the active document is an untouched blank project.
    pub(crate) fn active_is_pristine(&self) -> bool {
        self.root_directory.is_none()
            && !self.project_modified
            && self.ximod.name.is_empty()
            && self.ximod.steps.is_empty()
            && self.ximod.required_files.is_empty()
            && self.ximod.conditional_files.is_empty()
    }

    /// A fresh document has no findings and no Problems panel.
    pub(crate) fn reset_problems(&mut self) {
        self.validation_issues.clear();
        self.show_problems = false;
        self.problems_expanded = true;
        self.sizes = None;
        self.validation_rx = None;
    }

    /// Reset the navigation selection to a freshly-loaded project's defaults.
    pub(crate) fn reset_navigation(&mut self) {
        self.current_tab = Tab::Info;
        self.reset_selection();
    }

    /// Make a freshly-loaded project the active document — a new tab, unless the
    /// current document is a pristine blank project (then it is replaced).
    pub(crate) fn open_loaded(&mut self, ximod: Ximod, root: PathBuf) {
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
            self.docs.push(DocState::blank());
            self.active_doc = self.docs.len() - 1;
        }
        self.ximod = ximod;
        self.history.reset(&self.ximod);
        self.root_directory = Some(root);
        self.project_modified = false;
        self.import_report.clear();
        self.reset_problems();
        self.reset_navigation();
        self.commit_active();
    }

    /// Full (untruncated) name of a document: its root folder name, else the mod
    /// name, else empty.
    /// Display name of a document: its root folder name, else the mod name.
    pub(crate) fn display_name(root: &Option<PathBuf>, mod_name: &str) -> String {
        if let Some(root) = root
            && let Some(name) = root.file_name().and_then(|n| n.to_str())
        {
            return name.to_string();
        }
        if !mod_name.is_empty() {
            return mod_name.to_string();
        }
        String::new()
    }

    /// Close the active FOMOD, asking for confirmation first if it has unsaved
    /// changes. A pristine blank tab closes without prompting.
    pub(crate) fn close_active_fomod(&mut self) {
        self.ensure_doc();
        if self.project_modified {
            self.close_prompt = Some(CloseScope::Active);
        } else {
            self.close_active_fomod_force();
        }
    }

    /// Close the active FOMOD unconditionally. The last remaining document
    /// becomes a blank one.
    pub(crate) fn close_active_fomod_force(&mut self) {
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
    pub(crate) fn close_all_fomods(&mut self) {
        self.commit_active();
        if self.docs.iter().any(|d| d.modified) {
            self.close_prompt = Some(CloseScope::All);
        } else {
            self.close_all_fomods_force();
        }
    }

    /// Close every FOMOD unconditionally, leaving a single blank project.
    pub(crate) fn close_all_fomods_force(&mut self) {
        self.docs.clear();
        self.docs.push(DocState::blank());
        self.active_doc = 0;
        self.checkout_active();
        self.status_message = self.i18n.t("status-ready");
    }

    /// Save every modified document that has a destination folder.
    ///
    /// Documents that have no folder yet are brought to the front and the
    /// user is asked where to save them; cancelling that dialog aborts the
    /// whole operation and returns `false`. Silently skipping them, as the
    /// old code did, lost their content on exit.
    pub(crate) fn save_all_modified(&mut self) -> bool {
        self.commit_active();
        for i in 0..self.docs.len() {
            if !self.docs[i].modified {
                continue;
            }
            if i != self.active_doc {
                self.commit_active();
                self.active_doc = i;
                self.checkout_active();
            }
            if !self.ensure_root_for_save() {
                self.commit_active();
                return false;
            }
            self.write_project();
            self.commit_active();
        }
        true
    }

    /// Names of the documents with unsaved changes, for the exit prompt.
    pub(crate) fn modified_doc_names(&self) -> Vec<String> {
        let untitled = self.i18n.t("tab-untitled");
        self.docs
            .iter()
            .enumerate()
            .filter_map(|(i, d)| {
                let (root, name, modified) = if i == self.active_doc {
                    (&self.root_directory, &self.ximod.name, self.project_modified)
                } else {
                    (&d.root_directory, &d.ximod.name, d.modified)
                };
                modified.then(|| {
                    let n = Self::display_name(root, name);
                    if n.is_empty() { untitled.clone() } else { n }
                })
            })
            .collect()
    }

    pub(crate) fn open_directory(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.load_project(path);
        }
    }

    pub(crate) fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(self.i18n.t("filter-xml"), &["xml"])
            .pick_file()
        {
            match fomod_root_from_drop(&path) {
                Some(root) => self.load_project(root),
                None => self.notify_err(self.i18n.t("msg-drop-not-fomod")),
            }
        }
    }

    pub(crate) fn load_project(&mut self, path: PathBuf) {
        if self.load_project_quiet(path) {
            self.notify_ok(self.i18n.t("msg-load-success"));
        }
    }

    /// Load the FOMOD under `path` into a tab and record it in the recent
    /// files. Errors are notified; a success is not (the caller does that,
    /// with its own wording). Constructs the editor cannot keep are listed
    /// in the Problems panel and announced with a warning toast.
    pub(crate) fn load_project_quiet(&mut self, path: PathBuf) -> bool {
        match xml::load_ximod_with_report(&path) {
            Ok((ximod, report)) => {
                // Open in a new tab (or focus/replace as appropriate).
                self.open_loaded(ximod, path.clone());
                self.config.add_recent_file(path);
                let _ = self.config.save();
                if !report.is_empty() {
                    self.import_report = report;
                    self.commit_active();
                    let msg = self.i18n.t_num("msg-import-lossy", self.import_report.len() as i64);
                    self.validation_issues = self.import_issues();
                    self.show_problems = true;
                    self.problems_expanded = true;
                    self.notify_warn(msg);
                }
                true
            }
            Err(e) => {
                let error_msg = self.i18n.t("msg-load-error");
                self.notify_err(format!("{}: {}", error_msg, e));
                false
            }
        }
    }

    /// "Open Archive…": pick a `.zip` / `.7z` and open the FOMOD it holds.
    pub(crate) fn open_archive_dialog(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(self.i18n.t("filter-archive"), &["zip", "7z"])
            .pick_file()
        {
            self.open_archive(ctx, path);
        }
    }

    /// Localise an archive-opening error.
    pub(crate) fn translate_archive_error(&self, e: &crate::archive_open::ArchiveError) -> String {
        use crate::archive_open::ArchiveError as E;
        match e {
            E::UnsupportedArchive { ext } => self.i18n.t_arg("msg-archive-unsupported", "ext", ext),
            E::NoFomod { path } => self
                .i18n
                .t_arg("msg-archive-no-fomod", "path", &path.display().to_string()),
            other => self.i18n.t_arg("msg-archive-error", "error", &other.to_string()),
        }
    }

    /// Open the FOMOD inside a mod archive: a previous extraction of the
    /// same archive is reused as is; otherwise the archive is extracted on
    /// a worker thread (progress window, cancellable) and the project is
    /// loaded when it is done (see `archive_extracted`).
    pub(crate) fn open_archive(&mut self, ctx: &egui::Context, archive: PathBuf) {
        use crate::archive_open::{self, Prepared, WorkKind};
        let dest = match archive_open::prepare(&archive, WorkKind::Full) {
            Ok(Prepared::Extract { dest }) => dest,
            Ok(Prepared::Reused { root }) => {
                if self.load_project_quiet(root.clone()) {
                    self.notify_info(
                        self.i18n
                            .t_arg("msg-archive-reused", "path", &root.display().to_string()),
                    );
                }
                return;
            }
            Err(e) => {
                self.notify_err(self.translate_archive_error(&e));
                return;
            }
        };
        let src = archive.clone();
        self.spawn_job(
            ctx,
            archive,
            crate::ui::jobs::JobPurpose::OpenArchive,
            move |progress| {
                archive_open::extract_archive(&src, &dest, false, progress)
                    .map(|r| r.files)
                    .map_err(|e| e.to_string())
            },
        );
    }

    /// Completion of an "open archive" job: load the extracted FOMOD.
    pub(crate) fn archive_extracted(&mut self, archive: &std::path::Path, files: usize) {
        use crate::archive_open::{self, WorkKind};
        match archive_open::extracted_root(archive, WorkKind::Full) {
            Some(root) => {
                if self.load_project_quiet(root.clone()) {
                    let mut args = FluentArgs::new();
                    args.set("num", files as i64);
                    args.set("path", root.display().to_string());
                    self.notify_ok(self.i18n.t_with_args("msg-archive-opened", Some(&args)));
                }
            }
            None => {
                let e = archive_open::ArchiveError::NoFomod {
                    path: archive.to_path_buf(),
                };
                self.notify_err(self.translate_archive_error(&e));
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
    pub(crate) fn merge_fomod(&mut self) {
        if self.root_directory.is_none() {
            self.notify_err(self.i18n.t("msg-no-root-selected"));
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
                    self.ximod.conditional_files.append(&mut donor.conditional_files);

                    // Re-anchor the selection to a valid, simple state.
                    self.reset_selection();
                    self.mark_modified();
                    self.notify_ok(self.i18n.t("msg-merge-success"));
                }
                Err(e) => {
                    let error_msg = self.i18n.t("msg-merge-error");
                    self.notify_err(format!("{}: {}", error_msg, e));
                }
            }
        }
    }

    /// Entry point for saving (menu and Ctrl+S).
    ///
    /// Rather than block an incomplete project, we validate it and, if there are
    /// problems, open a confirmation dialog listing every one of them and letting
    /// the user save anyway. A valid project is written straight away.
    pub(crate) fn save_project(&mut self) {
        // A brand-new project has no folder yet: ask for one instead of
        // silently doing nothing (which is what Ctrl+S used to do).
        if !self.ensure_root_for_save() {
            return;
        }

        let errors = self.ximod.validate();
        // Schema conformity of the generated ModuleConfig.xml (ModConfig 5.0).
        let schema_issues: Vec<crate::xml::validate::SchemaIssue> = crate::xml::module_config_to_string(&self.ximod)
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

    /// Keep the OS window title in sync with the active document:
    /// "● MyMod — XIMOD Architect v2.0.0" (the dot marks unsaved changes).
    pub(crate) fn sync_window_title(&mut self, ctx: &egui::Context) {
        let name = Self::display_name(&self.root_directory, &self.ximod.name);
        let mut title = String::new();
        if self.project_modified {
            title.push_str("● ");
        }
        if !name.is_empty() {
            title.push_str(&name);
            title.push_str(" — ");
        }
        title.push_str(crate::APP_NAME);
        title.push_str(" v");
        title.push_str(crate::APP_VERSION);
        if self.window_title != title {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
            self.window_title = title;
        }
    }

    /// Make sure the active project has a root folder, asking the user for one
    /// when it has none. Returns `false` when the user cancelled the dialog.
    pub(crate) fn ensure_root_for_save(&mut self) -> bool {
        if self.root_directory.is_some() {
            return true;
        }
        match rfd::FileDialog::new()
            .set_title(self.i18n.t("dialog-choose-root"))
            .pick_folder()
        {
            Some(path) => {
                self.root_directory = Some(path);
                self.mark_modified();
                true
            }
            None => {
                self.notify_err(self.i18n.t("msg-no-root-selected"));
                false
            }
        }
    }

    /// "Save as…": pick a (new) root folder for the active project, then save.
    pub(crate) fn save_project_as(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .set_title(self.i18n.t("dialog-choose-root"))
            .pick_folder()
        else {
            return;
        };
        self.root_directory = Some(path);
        self.mark_modified();
        self.save_project();
    }

    /// Write the FOMOD to disk: pre-save script, XML files, post-save script.
    /// Called for a valid project, or after the user confirms saving one that
    /// still has validation warnings.
    pub(crate) fn write_project(&mut self) {
        let root = match &self.root_directory {
            Some(r) => r.clone(),
            None => {
                self.notify_err(self.i18n.t("msg-no-root-selected"));
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

        match self.save_with_backup(&root) {
            Ok(()) => {
                self.project_modified = false;
                self.notify_ok(self.i18n.t("msg-save-success"));

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
                self.notify_err(format!("{}: {}", error_msg, e));
            }
        }
    }

    /// Write the FOMOD XML of the active project into `root`, keeping a
    /// rotating backup of the previous version first (see `backups.rs`). A
    /// backup that cannot be made is logged, not fatal: the save goes on.
    pub(crate) fn save_with_backup(&self, root: &std::path::Path) -> anyhow::Result<()> {
        let made = match crate::backups::backup_before_save(&self.ximod, root, self.config.backup_count) {
            Ok(made) => made,
            Err(e) => {
                tracing::warn!("Backup skipped: {e}");
                None
            }
        };
        let result = xml::save_ximod(&self.ximod, root);
        if result.is_err()
            && let Some(dir) = made
        {
            // Nothing was replaced: the backup would be a copy of the current files.
            let _ = std::fs::remove_dir_all(dir);
        }
        result
    }

    /// Build a ready-to-upload distribution archive (FOMOD XML + mod files).
    ///
    /// Writes the FOMOD XML into the project root, then zips the whole root into
    /// a single `.zip` whose layout is exactly what a mod manager expects.
    pub(crate) fn export_distribution(&mut self, ctx: &egui::Context) {
        let root = match &self.root_directory {
            Some(r) => r.clone(),
            None => {
                self.notify_err(self.i18n.t("msg-no-root-selected"));
                return;
            }
        };
        let default_name = crate::export::default_archive_name(&self.ximod);
        let Some(path) = rfd::FileDialog::new()
            .set_file_name(&default_name)
            .add_filter("Zip", &["zip"])
            .add_filter("7-Zip", &["7z"])
            .save_file()
        else {
            return;
        };
        // Pick the format from the chosen extension (defaults to ZIP).
        let format = match path
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("7z") => crate::archive::ArchiveFormat::SevenZip,
            _ => crate::archive::ArchiveFormat::Zip,
        };
        // The XML is written here, on the UI thread (fast, and it must see the
        // live model); the compression itself runs on a worker thread with a
        // progress window, see `ui/jobs.rs`.
        if let Err(e) = self.save_with_backup(&root) {
            let mut args = FluentArgs::new();
            args.set("error", e.to_string());
            self.notify_err(self.i18n.t_with_args("msg-export-error", Some(&args)));
            return;
        }
        self.project_modified = false;
        self.spawn_export(ctx, root, path, format);
    }

    /// New from folder (V2 Lot B): pick a folder, scan it and propose a
    /// steps/groups/options skeleton (one subfolder = one option).
    pub(crate) fn new_from_folder(&mut self) {
        let Some(path) = rfd::FileDialog::new().pick_folder() else {
            return;
        };
        match crate::wizard::propose_from_folder(&path, &crate::wizard::WizardOptions::default()) {
            Ok(ximod) => {
                let count = ximod
                    .steps
                    .first()
                    .and_then(|s| s.plugin_groups.first())
                    .map(|g| g.plugins.len())
                    .unwrap_or(0);
                self.open_loaded(ximod, path);
                self.mark_modified();
                self.notify_ok(self.i18n.t_num("msg-wizard-success", count as i64));
            }
            Err(e) => {
                self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string()));
            }
        }
    }

    /// The FOMOD tab strip (one tab per open document).
    pub(crate) fn render_fomod_tabs(&mut self, ctx: &egui::Context, modal_open: bool) {
        let active = self.active_doc;
        let untitled = self.i18n.t("tab-untitled");
        let close_hint = self.i18n.t("tab-close-hint");
        // Precompute (label, tooltip, modified) to avoid borrow issues.
        let entries: Vec<(String, String, bool)> = self
            .docs
            .iter()
            .enumerate()
            .map(|(i, d)| {
                // The active tab reflects the live working copy, not its
                // (possibly stale) `docs` slot.
                let (root, name, modified) = if i == active {
                    (&self.root_directory, &self.ximod.name, self.project_modified)
                } else {
                    (&d.root_directory, &d.ximod.name, d.modified)
                };
                let full = Self::display_name(root, name);
                let mut label = elide_tab(&full, 22);
                if label.is_empty() {
                    label = untitled.clone();
                }
                let tooltip = root.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| {
                    if full.is_empty() {
                        untitled.clone()
                    } else {
                        full.clone()
                    }
                });
                (label, tooltip, modified)
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
                        if ui.selectable_label(i == active, text).on_hover_text(tooltip).clicked() {
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

    /// Handle files dropped onto the window.
    ///
    /// Two behaviors, disambiguated by whether the drop targets the open project:
    /// - if a project is open, an option (plugin) is selected, and every dropped
    ///   path lies inside the project root → assign them to that option as
    ///   file/folder sources (V2 Lot B drag-and-drop);
    /// - otherwise → open any dropped FOMOD folder / config file (V1 behavior).
    pub(crate) fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        let dropped: Vec<PathBuf> = crate::ui::dragdrop::dropped_paths(ctx);
        if dropped.is_empty() {
            return;
        }

        // Try the "assign to selected option" path first.
        if let (Some(root), Some(si), Some(gi), Some(pi)) = (
            self.root_directory.clone(),
            self.selection.step,
            self.selection.group,
            self.selection.plugin,
        ) {
            let root_canon = root.canonicalize().unwrap_or_else(|_| root.clone());
            let all_inside = dropped
                .iter()
                .all(|p| p.canonicalize().map(|c| c.starts_with(&root_canon)).unwrap_or(false));
            if all_inside
                && let Some(plugin) = self
                    .ximod
                    .steps
                    .get_mut(si)
                    .and_then(|s| s.plugin_groups.get_mut(gi))
                    .and_then(|g| g.plugins.get_mut(pi))
            {
                let before = plugin.files.len();
                let (added, rejected) = crate::ui::dragdrop::assign_dropped_to_plugin(plugin, &dropped, &root);
                let new_sources: Vec<String> = plugin.files[before..].iter().map(|f| f.source.clone()).collect();
                self.mark_modified();
                let mut args = FluentArgs::new();
                args.set("added", added as i64);
                args.set("rejected", rejected as i64);
                self.status_message = self.i18n.t_with_args("msg-drop-assigned", Some(&args));
                // Dropped plugins: author and masters (see `files_table.rs`).
                self.after_plugin_files_added(si, gi, pi, &new_sources);
                return;
            }
        }

        // Fall back to opening a dropped FOMOD (folder, XML file or archive).
        for p in dropped {
            if p.is_file() && looks_like_archive(&p) {
                self.open_archive(ctx, p);
                continue;
            }
            match fomod_root_from_drop(&p) {
                Some(root) => self.load_project(root),
                None => self.notify_err(self.i18n.t("msg-drop-not-fomod")),
            }
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

/// Whether a dropped file is a mod archive, supported (`.zip` / `.7z`) or
/// not (`.rar`…): both go through `open_archive`, which explains the latter.
fn looks_like_archive(path: &std::path::Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("zip" | "7z" | "rar")
    )
}

/// Resolve a dropped path to a FOMOD root (the folder that contains `fomod/`).
pub(crate) fn fomod_root_from_drop(path: &std::path::Path) -> Option<PathBuf> {
    if path.is_dir() {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name.eq_ignore_ascii_case("fomod") {
            return path.parent().map(|p| p.to_path_buf());
        }
        if xml::installer_dir(path).is_dir() {
            return Some(path.to_path_buf());
        }
        return None;
    }
    if path.is_file() {
        // A FOMOD xml lives in <root>/fomod/ModuleConfig.xml (or info.xml).
        let parent = path.parent()?;
        let pname = parent.file_name().and_then(|n| n.to_str()).unwrap_or("");
        // Anything else (a random file dropped onto the window) is not a
        // FOMOD: opening its grand-parent as an empty project used to be
        // reported as a successful load and added to the recent files.
        if pname.eq_ignore_ascii_case("fomod") {
            return parent.parent().map(|p| p.to_path_buf());
        }
        return None;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app
    }

    fn project(name: &str, steps: usize) -> Ximod {
        let mut x = Ximod {
            name: name.to_string(),
            ..Default::default()
        };
        for i in 0..steps {
            x.steps.push(Step::new(format!("{name}-{i}")));
        }
        x
    }

    /// A unique scratch folder under the system temp dir (no `tempfile` crate).
    fn scratch_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod-docs-{}-{}", std::process::id(), tag));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn fresh_app_is_pristine_until_edited() {
        let mut app = app();
        assert!(app.active_is_pristine());
        assert!(app.modified_doc_names().is_empty());
        app.ximod.name = "Edited".into();
        app.mark_modified();
        assert!(!app.active_is_pristine());
        assert!(app.project_modified);
    }

    #[test]
    fn open_loaded_replaces_a_pristine_doc_then_adds_tabs() {
        let mut app = app();
        app.open_loaded(project("A", 0), PathBuf::from("/mods/A"));
        // The pristine blank document was replaced, not kept as a tab.
        assert_eq!(app.docs.len(), 1);
        assert_eq!(app.active_doc, 0);
        app.open_loaded(project("B", 2), PathBuf::from("/mods/B"));
        assert_eq!(app.docs.len(), 2);
        assert_eq!(app.active_doc, 1);
        assert_eq!(app.ximod.name, "B");
        // Re-opening an already open root just focuses its tab.
        app.open_loaded(project("A again", 0), PathBuf::from("/mods/A"));
        assert_eq!(app.docs.len(), 2);
        assert_eq!(app.active_doc, 0);
        assert_eq!(app.ximod.name, "A");
    }

    /// Each tab keeps its own Problems panel: findings, visibility and a
    /// validation still running for it.
    #[test]
    fn problems_panel_is_per_document() {
        use crate::ui::problems::{Issue, Severity};
        let mut app = app();
        app.open_loaded(project("A", 1), PathBuf::from("/mods/A"));
        app.validation_issues = vec![Issue::new(Severity::Warning, "only in A", None)];
        app.show_problems = true;
        app.problems_expanded = true;
        let (tx, rx) = std::sync::mpsc::channel::<crate::ui::jobs::DiskReport>();
        app.validation_rx = Some(std::rc::Rc::new((rx, app.project_revision)));

        app.open_loaded(project("B", 1), PathBuf::from("/mods/B"));
        assert!(app.validation_issues.is_empty(), "B starts with no findings");
        assert!(!app.show_problems);
        assert!(app.validation_rx.is_none(), "A's running validation is not B's");
        app.validation_issues = vec![Issue::new(Severity::Error, "only in B", None)];
        app.show_problems = true;

        app.switch_doc(0);
        assert_eq!(app.validation_issues.len(), 1);
        assert_eq!(app.validation_issues[0].message, "only in A");
        assert!(app.show_problems && app.problems_expanded);
        let pending = app.validation_rx.as_ref().expect("A's validation is still pending");
        assert_eq!(pending.1, app.project_revision, "re-stamped with the current revision");
        drop(tx);

        app.switch_doc(1);
        assert_eq!(app.validation_issues[0].message, "only in B");
        assert!(app.validation_rx.is_none());
    }

    #[test]
    fn switch_doc_restores_each_documents_state() {
        let mut app = app();
        app.open_loaded(project("A", 0), PathBuf::from("/mods/A"));
        app.open_loaded(project("B", 2), PathBuf::from("/mods/B"));
        // Loading selects the first step when there is one.
        assert_eq!(app.selection.step, Some(0));
        // Edit B: a different selection and a dirty flag.
        app.select_step(Some(1));
        app.ximod.name = "B edited".into();
        app.mark_modified();
        let rev = app.project_revision;

        app.switch_doc(0);
        assert_eq!(app.active_doc, 0);
        assert_eq!(app.ximod.name, "A");
        assert_eq!(app.root_directory.as_deref(), Some(std::path::Path::new("/mods/A")));
        assert_eq!(app.selection, Selection::default());
        assert!(!app.project_modified);
        assert_ne!(app.project_revision, rev, "swapping the model bumps the revision");

        app.switch_doc(1);
        assert_eq!(app.ximod.name, "B edited");
        assert_eq!(app.root_directory.as_deref(), Some(std::path::Path::new("/mods/B")));
        assert_eq!(app.selection.step, Some(1));
        assert!(app.project_modified);
        assert!(app.history.can_undo());

        // Out-of-range or same index: no-op.
        app.switch_doc(7);
        app.switch_doc(1);
        assert_eq!(app.active_doc, 1);
        assert_eq!(app.ximod.name, "B edited");
    }

    #[test]
    fn closing_a_modified_doc_prompts_and_forcing_activates_the_neighbour() {
        let mut app = app();
        app.open_loaded(project("A", 0), PathBuf::from("/mods/A"));
        app.open_loaded(project("B", 1), PathBuf::from("/mods/B"));
        app.ximod.name = "B edited".into();
        app.mark_modified();

        app.close_active_fomod();
        assert_eq!(app.close_prompt, Some(CloseScope::Active));
        assert_eq!(app.docs.len(), 2);
        assert_eq!(app.ximod.name, "B edited");

        app.close_prompt = None;
        app.close_active_fomod_force();
        assert_eq!(app.docs.len(), 1);
        assert_eq!(app.active_doc, 0);
        assert_eq!(app.ximod.name, "A");
        assert!(!app.project_modified);

        // Closing the last document leaves a blank one.
        app.close_active_fomod_force();
        assert_eq!(app.docs.len(), 1);
        assert!(app.active_is_pristine());
        assert!(app.root_directory.is_none());
    }

    #[test]
    fn close_all_prompts_when_any_doc_is_modified_and_force_leaves_a_blank() {
        let mut app = app();
        app.open_loaded(project("A", 0), PathBuf::from("/mods/A"));
        app.ximod.name = "A edited".into();
        app.mark_modified();
        app.open_loaded(project("B", 0), PathBuf::from("/mods/B"));
        // Active B is clean, but A (committed) is dirty: prompt.
        app.close_all_fomods();
        assert_eq!(app.close_prompt, Some(CloseScope::All));
        assert_eq!(app.docs.len(), 2);

        app.close_prompt = None;
        app.close_all_fomods_force();
        assert_eq!(app.docs.len(), 1);
        assert_eq!(app.active_doc, 0);
        assert!(app.active_is_pristine());
        assert!(app.ximod.name.is_empty() && app.ximod.steps.is_empty());
        assert!(app.close_prompt.is_none());

        // With nothing modified, closing all needs no prompt.
        app.open_loaded(project("C", 0), PathBuf::from("/mods/C"));
        app.close_all_fomods();
        assert!(app.close_prompt.is_none());
        assert!(app.active_is_pristine());
    }

    #[test]
    fn modified_doc_names_lists_only_dirty_docs() {
        let mut app = app();
        app.open_loaded(project("A", 0), PathBuf::from("/mods/A"));
        app.ximod.name = "A edited".into();
        app.mark_modified();
        app.open_loaded(project("B", 0), PathBuf::from("/mods/B"));
        assert_eq!(app.modified_doc_names(), vec!["A".to_string()]);
        // The active document reflects the live working copy (not its stale slot).
        app.mark_modified();
        assert_eq!(app.modified_doc_names(), vec!["A".to_string(), "B".to_string()]);
        // A modified document without a root nor a name is listed as "untitled".
        app.new_project();
        app.mark_modified();
        let names = app.modified_doc_names();
        assert_eq!(names.len(), 3);
        assert_eq!(names[2], app.i18n.t("tab-untitled"));
    }

    #[test]
    fn new_project_reuses_a_pristine_tab() {
        let mut app = app();
        app.new_project();
        assert_eq!(app.docs.len(), 1);
        app.open_loaded(project("A", 1), PathBuf::from("/mods/A"));
        app.new_project();
        assert_eq!(app.docs.len(), 2);
        assert_eq!(app.active_doc, 1);
        assert!(app.active_is_pristine());
        assert_eq!(app.selection, Selection::default());
        assert_eq!(app.current_tab, Tab::Info);
    }

    #[test]
    fn display_name_prefers_the_root_folder() {
        assert_eq!(
            XimodApp::display_name(&Some(PathBuf::from("/mods/My Mod")), "Other"),
            "My Mod"
        );
        assert_eq!(XimodApp::display_name(&None, "Other"), "Other");
        assert_eq!(XimodApp::display_name(&None, ""), "");
    }

    #[test]
    fn elide_tab_keeps_short_names_and_truncates_long_ones() {
        assert_eq!(elide_tab("short", 22), "short");
        assert_eq!(elide_tab("abcdefghij", 5), "abcd…");
        assert_eq!(elide_tab("éèêëéèêë", 4), "éèê…");
    }

    #[test]
    fn looks_like_archive_matches_zip_7z_and_rar_only() {
        assert!(looks_like_archive(std::path::Path::new("/dl/Mod-1.0.ZIP")));
        assert!(looks_like_archive(std::path::Path::new("mod.7z")));
        assert!(looks_like_archive(std::path::Path::new("mod.rar")));
        assert!(!looks_like_archive(std::path::Path::new("fomod/ModuleConfig.xml")));
        assert!(!looks_like_archive(std::path::Path::new("mod.tar.gz")));
    }

    #[test]
    fn import_report_is_per_document_and_feeds_the_problems_panel() {
        let mut app = app();
        app.open_loaded(project("A", 1), PathBuf::from("/mods/A"));
        app.import_report = vec![crate::xml::fidelity::Unmodelled::UnknownElement {
            element: "v1.6".into(),
            parent: "config".into(),
            loc: crate::xml::fidelity::Loc::Module,
        }];
        app.commit_active();
        let issues = app.import_issues();
        assert_eq!(issues.len(), 2, "one header line + one item");
        assert!(issues[0].message.contains('1'), "{}", issues[0].message);
        assert!(issues[1].message.contains("1.6"), "{}", issues[1].message);
        assert_eq!(issues[1].target, Some(crate::ui::problems::Target::Info));
        // Another document has no report; switching back restores it.
        app.open_loaded(project("B", 0), PathBuf::from("/mods/B"));
        assert!(app.import_report.is_empty());
        assert!(app.import_issues().is_empty());
        app.switch_doc(0);
        assert_eq!(app.import_report.len(), 1);
    }

    #[test]
    fn fomod_root_from_drop_resolves_folders_and_xml_files() {
        let root = scratch_dir("drop");
        let fomod = root.join("fomod");
        std::fs::create_dir_all(&fomod).unwrap();
        let cfg = fomod.join("ModuleConfig.xml");
        std::fs::write(&cfg, "<config/>").unwrap();
        let stray = root.join("readme.txt");
        std::fs::write(&stray, "hi").unwrap();

        assert_eq!(fomod_root_from_drop(&root).as_deref(), Some(root.as_path()));
        assert_eq!(fomod_root_from_drop(&fomod).as_deref(), Some(root.as_path()));
        assert_eq!(fomod_root_from_drop(&cfg).as_deref(), Some(root.as_path()));
        assert_eq!(fomod_root_from_drop(&stray), None);
        assert_eq!(fomod_root_from_drop(&root.join("missing")), None);
        let _ = std::fs::remove_dir_all(&root);
    }
}
