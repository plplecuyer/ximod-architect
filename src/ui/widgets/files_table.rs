//! Install-files table shared by the option, required and conditional panels.
//!
//! One rendering for the three places a list of `InstallFile`s is edited:
//! a striped grid (type / source / destination / priority, then the
//! `alwaysInstall` / `installIfUsable` flags) where the destination and the
//! flags are editable in place, followed by the Add file / Add folder /
//! Remove buttons.

use crate::models::*;
use crate::ui::main_window::{PickWhat, XimodApp};
use eframe::egui::{self, RichText};

/// Which file list a table edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FilesTarget {
    /// The files of one option (indices are assumed valid by the caller).
    Plugin { step: usize, group: usize, plugin: usize },
    /// The project's required installs.
    Required,
    /// The files of one conditional-install pattern.
    Conditional { pattern: usize },
}

/// Per-instance identity and layout of a files table.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FilesTableSpec {
    /// `id_salt` of the scroll area.
    pub list_id: &'static str,
    /// Id of the grid.
    pub grid_id: &'static str,
    /// Maximum height of the scroll area.
    pub max_height: f32,
}

/// Translated texts of the table and its buttons.
struct FilesTableLabels {
    btn_add_file: String,
    btn_add_folder: String,
    btn_remove: String,
    col_type: String,
    col_source: String,
    col_destination: String,
    col_priority: String,
    col_always: String,
    col_always_hint: String,
    col_if_usable: String,
    col_if_usable_hint: String,
    /// Hover text of the "view archive contents" button.
    archive_hint: String,
    /// Whether the view button can resolve a source (a root is open).
    can_view_archive: bool,
}

/// A button action that needs the application (file dialog) to complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilesAction {
    AddFiles,
    AddFolder,
    /// Open the archive contents of the row at this index.
    ViewArchive(usize),
}

/// Draw the grid and the buttons. Destination edits and removals are applied
/// to `files` in place; returns whether `files` changed and which "add"
/// action, if any, the user asked for.
fn files_table(
    ui: &mut egui::Ui,
    spec: &FilesTableSpec,
    labels: &FilesTableLabels,
    files: &mut Vec<InstallFile>,
    selected: &mut Option<usize>,
) -> (bool, Option<FilesAction>) {
    let mut modified = false;
    let mut action = None;

    egui::ScrollArea::vertical()
        .id_salt(spec.list_id)
        .max_height(spec.max_height)
        .show(ui, |ui| {
            egui::Grid::new(spec.grid_id)
                .num_columns(6)
                .striped(true)
                .show(ui, |ui| {
                    ui.label(RichText::new(&labels.col_type).strong());
                    ui.label(RichText::new(&labels.col_source).strong());
                    ui.label(RichText::new(&labels.col_destination).strong());
                    ui.label(RichText::new(&labels.col_priority).strong());
                    ui.label(RichText::new(&labels.col_always).strong())
                        .on_hover_text(&labels.col_always_hint);
                    ui.label(RichText::new(&labels.col_if_usable).strong())
                        .on_hover_text(&labels.col_if_usable_hint);
                    ui.end_row();

                    for (idx, f) in files.iter_mut().enumerate() {
                        let is_selected = *selected == Some(idx);
                        if ui.selectable_label(is_selected, f.file_type.as_str()).clicked() {
                            *selected = Some(idx);
                        }
                        let is_archive = f.file_type == FileType::File
                            && crate::models::bethesda_archive::is_bethesda_archive(f.source.trim());
                        if is_archive && labels.can_view_archive {
                            ui.horizontal(|ui| {
                                ui.label(&f.source);
                                if ui
                                    .small_button(crate::ui::theme::icon::PREVIEW)
                                    .on_hover_text(&labels.archive_hint)
                                    .clicked()
                                {
                                    action = Some(FilesAction::ViewArchive(idx));
                                }
                            });
                        } else {
                            ui.label(&f.source);
                        }
                        if ui
                            .add(egui::TextEdit::singleline(&mut f.destination).desired_width(220.0))
                            .changed()
                        {
                            modified = true;
                        }
                        ui.label(f.priority.to_string());
                        if ui
                            .checkbox(&mut f.always_install, "")
                            .on_hover_text(&labels.col_always_hint)
                            .changed()
                        {
                            modified = true;
                        }
                        if ui
                            .checkbox(&mut f.install_if_usable, "")
                            .on_hover_text(&labels.col_if_usable_hint)
                            .changed()
                        {
                            modified = true;
                        }
                        ui.end_row();
                    }
                });
        });

    ui.horizontal(|ui| {
        if ui.button(&labels.btn_add_file).clicked() {
            action = Some(FilesAction::AddFiles);
        }

        if ui.button(&labels.btn_add_folder).clicked() {
            action = Some(FilesAction::AddFolder);
        }

        let can_remove = selected.is_some();
        if ui
            .add_enabled(can_remove, egui::Button::new(&labels.btn_remove))
            .clicked()
            && let Some(file_idx) = *selected
        {
            if file_idx < files.len() {
                files.remove(file_idx);
                modified = true;
            }
            *selected = None;
        }
    });

    (modified, action)
}

impl XimodApp {
    /// The file list and the selection index that a `FilesTarget` refers to.
    fn files_target_mut(&mut self, target: FilesTarget) -> (&mut Vec<InstallFile>, &mut Option<usize>) {
        match target {
            FilesTarget::Plugin { step, group, plugin } => (
                &mut self.ximod.steps[step].plugin_groups[group].plugins[plugin].files,
                &mut self.current_file_index,
            ),
            FilesTarget::Required => (&mut self.ximod.required_files, &mut self.current_req_file_index),
            FilesTarget::Conditional { pattern } => (
                &mut self.ximod.conditional_files[pattern].files,
                &mut self.current_cond_file_index,
            ),
        }
    }

    /// Render the files table of `target`: the grid, then the Add file / Add
    /// folder / Remove buttons. Added paths are picked relative to the project
    /// root (see `pick_relative`).
    pub(crate) fn render_files_table(&mut self, ui: &mut egui::Ui, target: FilesTarget, spec: FilesTableSpec) {
        let labels = FilesTableLabels {
            btn_add_file: self.i18n.t("btn-add-file"),
            btn_add_folder: self.i18n.t("btn-add-folder"),
            btn_remove: self.i18n.t("btn-remove-file"),
            col_type: self.i18n.t("label-file-type"),
            col_source: self.i18n.t("label-source"),
            col_destination: self.i18n.t("label-destination"),
            col_priority: self.i18n.t("label-priority"),
            col_always: self.i18n.t("file-always-install"),
            col_always_hint: self.i18n.t("file-always-install-hint"),
            col_if_usable: self.i18n.t("file-install-if-usable"),
            col_if_usable_hint: self.i18n.t("file-install-if-usable-hint"),
            archive_hint: self.i18n.t("archive-view-hint"),
            can_view_archive: self.root_directory.is_some(),
        };

        let (modified, action) = {
            let (files, selected) = self.files_target_mut(target);
            files_table(ui, &spec, &labels, files, selected)
        };
        if modified {
            self.mark_modified();
        }

        match action {
            Some(FilesAction::AddFiles) => {
                let rels = self.pick_relative(PickWhat::Files);
                for rel in &rels {
                    self.files_target_mut(target).0.push(InstallFile::new_file(rel.clone()));
                    self.mark_modified();
                }
                if let FilesTarget::Plugin { step, group, plugin } = target {
                    self.after_plugin_files_added(step, group, plugin, &rels);
                }
            }
            Some(FilesAction::AddFolder) => {
                for rel in self.pick_relative(PickWhat::Folder) {
                    self.files_target_mut(target).0.push(InstallFile::new_folder(rel));
                    self.mark_modified();
                }
            }
            Some(FilesAction::ViewArchive(idx)) => {
                let source = self.files_target_mut(target).0.get(idx).map(|f| f.source.clone());
                if let Some(source) = source {
                    self.open_archive_view_rel(&source);
                }
            }
            None => {}
        }
    }

    /// Plugin files (`.esp` / `.esm` / `.esl`) were just added to an option:
    /// read their headers to fill in the mod author when it is still empty
    /// and, when `auto_masters` is on, declare their masters that neither
    /// the game nor the mod provides as `Active` file dependencies of the
    /// option (see `models::plugin_checks`). Unreadable files are ignored.
    pub(crate) fn after_plugin_files_added(&mut self, step: usize, group: usize, plugin: usize, sources: &[String]) {
        use crate::models::plugin_checks::{add_masters_to_plugin, file_name_of, masters_to_add};
        use crate::models::plugin_header::{PluginKind, read_plugin_header};
        let Some(root) = self.root_directory.clone() else {
            return;
        };
        let base: Vec<String> = self.games.base_masters_for(&self.ximod.game).to_vec();
        for source in sources {
            let abs = root.join(source.replace('\\', "/"));
            if PluginKind::from_path(&abs).is_none() {
                continue;
            }
            let Ok(header) = read_plugin_header(&abs) else { continue };
            if self.ximod.author.trim().is_empty()
                && let Some(author) = header.author.as_deref().map(str::trim).filter(|a| !a.is_empty())
            {
                self.ximod.author = author.to_string();
                self.mark_modified();
                self.notify_info(self.i18n.t_arg("msg-author-from-plugin", "author", author));
            }
            if !self.config.auto_masters {
                continue;
            }
            let wanted = masters_to_add(&self.ximod, &header, &base);
            let Some(opt) = self
                .ximod
                .steps
                .get_mut(step)
                .and_then(|s| s.plugin_groups.get_mut(group))
                .and_then(|g| g.plugins.get_mut(plugin))
            else {
                return;
            };
            let added = add_masters_to_plugin(opt, &wanted);
            if added > 0 {
                self.mark_modified();
                let mut args = fluent::FluentArgs::new();
                args.set("num", added as i64);
                args.set("plugin", file_name_of(source).to_string());
                self.notify_info(self.i18n.t_with_args("msg-masters-added", Some(&args)));
            }
        }
    }

    /// Files table of one option ("Files" section of the option inspector).
    pub(crate) fn render_plugin_files(
        &mut self,
        ui: &mut egui::Ui,
        step_idx: usize,
        group_idx: usize,
        plugin_idx: usize,
    ) {
        self.render_files_table(
            ui,
            FilesTarget::Plugin {
                step: step_idx,
                group: group_idx,
                plugin: plugin_idx,
            },
            FilesTableSpec {
                list_id: "files_list",
                grid_id: "plugin_files_grid",
                max_height: 100.0,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let mut step = Step::new("S");
        let mut group = PluginGroup::new("G", SelectionType::SelectAny);
        let mut p = Plugin::new("P");
        p.files.push(InstallFile::new_file("a.esp"));
        p.files.push(InstallFile::new_folder("textures"));
        group.plugins.push(p);
        step.plugin_groups.push(group);
        app.ximod.steps.push(step);
        app.ximod.required_files.push(InstallFile::new_file("req.esp"));
        app.ximod.conditional_files.push(ConditionalFileSet::new());
        app.ximod.conditional_files[0]
            .files
            .push(InstallFile::new_file("cond.esp"));
        app
    }

    #[test]
    fn targets_resolve_to_their_lists() {
        let mut app = app();
        let (files, sel) = app.files_target_mut(FilesTarget::Plugin {
            step: 0,
            group: 0,
            plugin: 0,
        });
        assert_eq!(files.len(), 2);
        assert!(sel.is_none());
        let (files, _) = app.files_target_mut(FilesTarget::Required);
        assert_eq!(files[0].source, "req.esp");
        let (files, _) = app.files_target_mut(FilesTarget::Conditional { pattern: 0 });
        assert_eq!(files[0].source, "cond.esp");
    }

    /// Adding a plugin to an option declares its missing masters once, fills
    /// the empty author, and respects the `auto_masters` switch.
    #[test]
    fn plugin_added_declares_masters_and_author() {
        use crate::models::plugin_header::tests::build_plugin;
        let root = std::env::temp_dir().join(format!("ximod_ft_masters_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("Mod.esp"),
            build_plugin(0, &["Skyrim.esm", "Lib.esm", "req.esp"], &[]),
        )
        .unwrap();
        std::fs::write(root.join("notes.txt"), b"x").unwrap();

        let mut app = app();
        app.root_directory = Some(root.clone());
        app.ximod.game = "skyrimSpecialEdition".into();
        app.ximod.author.clear();
        app.config.auto_masters = true;
        // The games dataset may be absent in the test environment: inject it.
        if app.games.base_masters_for("skyrimSpecialEdition").is_empty() {
            app.games.games.insert(
                "skyrimSpecialEdition".into(),
                crate::games::GameEntry {
                    name: "SSE".into(),
                    nexus_slug: String::new(),
                    categories: Vec::new(),
                    masters: vec!["Skyrim.esm".into()],
                    esl_limit: None,
                },
            );
        }
        let sources = vec!["Mod.esp".to_string(), "notes.txt".to_string()];
        app.after_plugin_files_added(0, 0, 0, &sources);
        assert_eq!(app.ximod.author, "Tester");
        let opt = &app.ximod.steps[0].plugin_groups[0].plugins[0];
        // Skyrim.esm is a base master, req.esp is shipped by the mod: only Lib.esm.
        assert_eq!(opt.dependency_patterns.len(), 1);
        assert_eq!(opt.dependency_patterns[0].pattern_type, "Optional");
        assert_eq!(
            opt.dependency_patterns[0].condition,
            DependencyGroup::from_leaves(LogicalOperator::And, vec![Dependency::new_file("Lib.esm", "Active")])
        );
        assert!(app.project_modified);
        // Again: nothing is duplicated.
        app.after_plugin_files_added(0, 0, 0, &sources);
        assert_eq!(
            app.ximod.steps[0].plugin_groups[0].plugins[0].dependency_patterns[0]
                .condition
                .leaf_count(),
            1
        );
        // Switched off: a fresh option gets no dependency.
        app.config.auto_masters = false;
        app.ximod.steps[0].plugin_groups[0].plugins.push(Plugin::new("P2"));
        app.after_plugin_files_added(0, 0, 1, &sources);
        assert!(
            app.ximod.steps[0].plugin_groups[0].plugins[1]
                .dependency_patterns
                .is_empty()
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// The three tables lay out headless without panicking and a plain
    /// render never dirties the project.
    #[test]
    fn tables_render_headless() {
        let mut app = app();
        let ctx = egui::Context::default();
        app.current_req_file_index = Some(0);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                app.render_plugin_files(ui, 0, 0, 0);
                app.render_files_table(
                    ui,
                    FilesTarget::Required,
                    FilesTableSpec {
                        list_id: "req_files_list",
                        grid_id: "req_files_grid",
                        max_height: 300.0,
                    },
                );
                app.render_files_table(
                    ui,
                    FilesTarget::Conditional { pattern: 0 },
                    FilesTableSpec {
                        list_id: "cond_files_list",
                        grid_id: "cond_files_grid",
                        max_height: 150.0,
                    },
                );
            });
        });
        assert!(!app.project_modified);
        assert_eq!(app.current_req_file_index, Some(0));
    }
}
