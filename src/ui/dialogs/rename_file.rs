//! "Rename on disk…" (files tables): renames a source file or folder in the
//! mod folder and updates every reference of the project to it — for a
//! folder, the references inside it too (see `models::file_refs`).

use crate::models::file_refs::{self, RenameError};
use crate::ui::main_window::XimodApp;
use eframe::egui::{self, RichText};

const WINDOW_ID: &str = "ximod_rename";
const WINDOW_SIZE: [f32; 2] = [480.0, 240.0];

/// State of the rename window.
#[derive(Debug, Clone, Default)]
pub struct RenameDialog {
    /// The reference being renamed, as stored in the project.
    pub source: String,
    /// The new name being typed (bare name, same folder).
    pub new_name: String,
    /// References that will follow.
    pub uses: usize,
    /// Last refusal, already translated.
    pub error: Option<String>,
    /// Focus the name field on the first frame only.
    focused: bool,
}

/// Last component of a reference (either separator).
fn file_name_of(source: &str) -> &str {
    let s = source.trim().trim_end_matches(['/', '\\']);
    s.rsplit(['/', '\\']).next().unwrap_or(s)
}

impl XimodApp {
    /// Open the rename window for `source` (a reference of the project).
    pub(crate) fn open_rename(&mut self, source: &str) {
        if self.root_directory.is_none() {
            let msg = self.i18n.t("msg-no-root-selected");
            self.notify_err(msg);
            return;
        }
        self.rename_dialog = Some(RenameDialog {
            source: source.to_string(),
            new_name: file_name_of(source).to_string(),
            uses: file_refs::count_uses(&self.ximod, source, true),
            error: None,
            focused: false,
        });
    }

    /// Translated text of a refusal.
    fn rename_error_text(&self, e: &RenameError) -> String {
        match e {
            RenameError::EmptyName => self.i18n.t("rename-err-empty"),
            RenameError::InvalidName => self.i18n.t("rename-err-invalid"),
            RenameError::OutsideRoot => self.i18n.t("rename-err-outside"),
            RenameError::Missing => self.i18n.t("rename-err-missing"),
            RenameError::AlreadyExists => self.i18n.t("rename-err-exists"),
            RenameError::Io(msg) => self.i18n.t_arg("rename-err-io", "error", msg),
        }
    }

    /// Rename on disk, then update the project (one undo step).
    pub(crate) fn commit_rename(&mut self) {
        let (Some(dlg), Some(root)) = (self.rename_dialog.clone(), self.root_directory.clone()) else {
            return;
        };
        match file_refs::rename_on_disk(&mut self.ximod, &root, &dlg.source, &dlg.new_name) {
            Ok(out) => {
                if out.updated > 0 {
                    self.mark_modified();
                }
                let mut args = fluent::FluentArgs::new();
                args.set("path", out.new_rel.clone());
                args.set("num", out.updated as i64);
                let msg = self.i18n.t_with_args("msg-renamed", Some(&args));
                self.notify_ok(msg);
                self.rename_dialog = None;
            }
            Err(e) => {
                let text = self.rename_error_text(&e);
                if let Some(d) = &mut self.rename_dialog {
                    d.error = Some(text);
                }
            }
        }
    }

    /// Render the rename window (independent viewport, fixed size).
    pub(crate) fn render_rename_dialog(&mut self, ctx: &egui::Context) {
        if self.rename_dialog.is_none() {
            self.free_window_closed(WINDOW_ID);
            return;
        }
        let title = self.i18n.t("rename-title");
        let current = self.i18n.t_arg(
            "rename-current",
            "path",
            &self
                .rename_dialog
                .as_ref()
                .map(|d| d.source.clone())
                .unwrap_or_default(),
        );
        let label_name = self.i18n.t("rename-new-name");
        let uses = self
            .i18n
            .t_num("rename-uses", self.rename_dialog.as_ref().map_or(0, |d| d.uses) as i64);
        let warning = self.i18n.t("rename-warning");
        let btn_rename = self.i18n.t("btn-rename");
        let btn_cancel = self.i18n.t("btn-cancel");
        let vb = self.free_viewport_builder(ctx, WINDOW_ID, title, WINDOW_SIZE, true);

        let cfg = &mut self.config;
        let Some(dlg) = self.rename_dialog.as_mut() else { return };
        let mut do_rename = false;
        let mut do_close = false;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of(WINDOW_ID), vb, |ctx, _class| {
            egui::TopBottomPanel::bottom("rename_bottom").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&btn_cancel).clicked() {
                            do_close = true;
                        }
                        let can = !dlg.new_name.trim().is_empty();
                        if ui.add_enabled(can, egui::Button::new(&btn_rename)).clicked() {
                            do_rename = true;
                        }
                    });
                });
                ui.add_space(4.0);
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(&current);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(&label_name);
                    let resp = ui.add(egui::TextEdit::singleline(&mut dlg.new_name).desired_width(300.0));
                    if !dlg.focused {
                        resp.request_focus();
                        dlg.focused = true;
                    }
                    if resp.changed() {
                        dlg.error = None;
                    }
                    if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        do_rename = true;
                    }
                });
                ui.add_space(4.0);
                ui.label(&uses);
                ui.label(RichText::new(&warning).weak().small());
                if let Some(err) = &dlg.error {
                    ui.add_space(4.0);
                    ui.colored_label(ui.visuals().error_fg_color, err);
                }
            });
            crate::ui::widgets::free_window::record_win_geom(cfg, ctx, WINDOW_ID);
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        if do_rename {
            self.commit_rename();
        }
        if do_close {
            self.rename_dialog = None;
            self.free_window_closed(WINDOW_ID);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{InstallFile, Plugin, PluginGroup, SelectionType, Step};

    #[test]
    fn rename_window_renames_and_reports_refusals() {
        let root = std::env::temp_dir().join(format!("ximod_rename_ui_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("Feature/A")).unwrap();
        std::fs::write(root.join("Feature/A/x.esp"), b"x").unwrap();
        std::fs::create_dir_all(root.join("Feature/B")).unwrap();

        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let mut p = Plugin::new("A");
        p.files.push(InstallFile::new_folder("Feature\\A"));
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        app.ximod.steps.push(s);
        app.root_directory = Some(root.clone());

        app.open_rename("Feature\\A");
        let dlg = app.rename_dialog.as_ref().unwrap();
        assert_eq!(dlg.new_name, "A");
        assert_eq!(dlg.uses, 1);
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_rename_dialog(ctx));

        // Refused: the name is taken; the window stays open with the reason.
        app.rename_dialog.as_mut().unwrap().new_name = "B".into();
        app.commit_rename();
        assert!(app.rename_dialog.as_ref().unwrap().error.is_some());

        app.rename_dialog.as_mut().unwrap().new_name = "Alpha".into();
        app.commit_rename();
        assert!(app.rename_dialog.is_none());
        assert!(root.join("Feature/Alpha/x.esp").is_file());
        assert_eq!(
            app.ximod.steps[0].plugin_groups[0].plugins[0].files[0].source,
            "Feature\\Alpha"
        );
        assert!(app.project_modified);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn file_name_of_handles_both_separators() {
        assert_eq!(file_name_of("a\\b\\c.esp"), "c.esp");
        assert_eq!(file_name_of("a/b/"), "b");
        assert_eq!(file_name_of("solo"), "solo");
    }
}
