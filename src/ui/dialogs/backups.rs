//! "Restore a backup" dialog: the rotating backups of the active project
//! (see `backups.rs`), newest first, each with the number of differences
//! from the current project. Compare opens the diff window, Restore swaps
//! the backup in as the working copy (undoable), and the whole folder can be
//! emptied with a two-click "Delete all backups".

use crate::backups::{self, BackupEntry};
use crate::models::compare::diff_projects;
use crate::ui::components::{modal_keys, modal_window};
use crate::ui::main_window::XimodApp;
use eframe::egui::{self, RichText};

/// One row of the dialog: a backup and its diff summary against the current
/// project (`None` when the backup could not be loaded).
#[derive(Debug, Clone)]
pub struct BackupRow {
    pub entry: BackupEntry,
    pub changes: Option<usize>,
}

/// What the user clicked this frame.
enum BackupAction {
    Compare(usize),
    Restore(usize),
    DeleteAll,
    Close,
}

impl XimodApp {
    /// (Re)read the backup folder of the active project.
    pub(crate) fn refresh_backups(&mut self) {
        self.backups_delete_armed = false;
        let Some(root) = self.root_directory.clone() else {
            self.backups.clear();
            return;
        };
        self.backups = backups::list_backups(&root)
            .into_iter()
            .map(|entry| {
                let changes = backups::load_backup(&entry.path)
                    .ok()
                    .map(|old| diff_projects(&old, &self.ximod).total());
                BackupRow { entry, changes }
            })
            .collect();
    }

    /// File → Restore a backup…
    pub(crate) fn open_backups_dialog(&mut self) {
        if self.root_directory.is_none() {
            self.notify_err(self.i18n.t("msg-no-root-selected"));
            return;
        }
        self.refresh_backups();
        self.show_backups = true;
    }

    /// Show the diff window between a backup and the current project. The
    /// modal backup list closes so the (non-modal) diff window is reachable.
    pub(crate) fn compare_with_backup(&mut self, index: usize) {
        let Some(row) = self.backups.get(index) else { return };
        match backups::load_backup(&row.entry.path) {
            Ok(old) => {
                let diff = diff_projects(&old, &self.ximod);
                self.compare_other_name = row.entry.display_time();
                self.compare_result = Some(diff);
                self.show_compare = true;
                self.show_backups = false;
                self.backups_delete_armed = false;
            }
            Err(e) => {
                self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string()));
            }
        }
    }

    /// Replace the working copy with a backup. The swap is one undo step and
    /// marks the document modified (nothing is written to disk until the
    /// next save, which backs up the version being replaced).
    pub(crate) fn restore_backup(&mut self, index: usize) {
        let Some(row) = self.backups.get(index) else { return };
        let stamp = row.entry.display_time();
        match backups::load_backup(&row.entry.path) {
            Ok(restored) => {
                // Close any open edit burst so the restore is its own undo step.
                self.history
                    .end_burst(self.frame_time + crate::ui::history::COALESCE_SECS, &self.ximod);
                self.ximod = restored;
                self.reset_selection();
                self.mark_modified();
                self.show_backups = false;
                self.notify_ok(self.i18n.t_arg("msg-backup-restored", "time", &stamp));
            }
            Err(e) => {
                self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string()));
            }
        }
    }

    /// Empty the backup folder of the active project.
    pub(crate) fn delete_all_backups(&mut self) {
        let Some(root) = self.root_directory.clone() else {
            return;
        };
        match backups::delete_all(&root) {
            Ok(n) => {
                self.backups.clear();
                self.backups_delete_armed = false;
                self.show_backups = false;
                self.notify_ok(self.i18n.t_num("msg-backups-deleted", n as i64));
            }
            Err(e) => {
                self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string()));
            }
        }
    }

    /// Render the modal "Restore a backup" dialog.
    pub(crate) fn render_backups_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_backups {
            return;
        }
        let title = self.i18n.t("backups-title");
        let empty = self.i18n.t("backups-empty");
        let unreadable = self.i18n.t("plugins-unreadable");
        let btn_compare = self.i18n.t("btn-compare");
        let btn_restore = self.i18n.t("btn-restore");
        let btn_delete = self.i18n.t("btn-delete-backups");
        let btn_confirm = self.i18n.t("btn-delete-backups-confirm");
        let btn_close = self.i18n.t("btn-close");
        let changes: Vec<String> = self
            .backups
            .iter()
            .map(|r| match r.changes {
                Some(n) => self.i18n.t_num("backups-changes", n as i64),
                None => unreadable.clone(),
            })
            .collect();
        let rows: Vec<(String, String)> = self
            .backups
            .iter()
            .zip(changes)
            .map(|(r, c)| (r.entry.display_time(), c))
            .collect();
        let armed = self.backups_delete_armed;

        let (_enter, esc) = modal_keys(ctx);
        let mut action: Option<BackupAction> = None;
        let mut armed_now = armed;
        modal_window(ctx, &title, |ui| {
            ui.set_min_width(460.0);
            if rows.is_empty() {
                ui.label(&empty);
            } else {
                egui::ScrollArea::vertical()
                    .max_height(320.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        egui::Grid::new("backups_grid")
                            .num_columns(4)
                            .striped(true)
                            .spacing([12.0, 6.0])
                            .show(ui, |ui| {
                                for (i, (time, summary)) in rows.iter().enumerate() {
                                    ui.label(RichText::new(time).monospace());
                                    ui.label(summary);
                                    if ui.button(&btn_compare).clicked() {
                                        action = Some(BackupAction::Compare(i));
                                    }
                                    if ui.button(&btn_restore).clicked() {
                                        action = Some(BackupAction::Restore(i));
                                    }
                                    ui.end_row();
                                }
                            });
                    });
            }
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if !rows.is_empty() {
                    if armed {
                        let warning = crate::ui::theme::Palette::from_ui(ui).warning;
                        if ui.button(RichText::new(&btn_confirm).color(warning).strong()).clicked() {
                            action = Some(BackupAction::DeleteAll);
                        }
                    } else if ui.button(&btn_delete).clicked() {
                        armed_now = true;
                    }
                }
                if ui.button(&btn_close).clicked() || esc {
                    action = Some(BackupAction::Close);
                }
            });
        });
        self.backups_delete_armed = armed_now;

        match action {
            Some(BackupAction::Compare(i)) => self.compare_with_backup(i),
            Some(BackupAction::Restore(i)) => self.restore_backup(i),
            Some(BackupAction::DeleteAll) => self.delete_all_backups(),
            Some(BackupAction::Close) => {
                self.show_backups = false;
                self.backups_delete_armed = false;
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Step, Ximod};
    use std::path::PathBuf;

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app
    }

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod-bkdlg-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn fake_backup(root: &std::path::Path, stamp: &str, ximod: &Ximod) {
        let dir = backups::backups_dir(root).join(stamp);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("info.xml"), crate::xml::info_xml_to_string(ximod).unwrap()).unwrap();
        std::fs::write(
            dir.join("ModuleConfig.xml"),
            crate::xml::module_config_to_string(ximod).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn restore_swaps_the_model_and_is_undoable() {
        let root = scratch("restore");
        let mut old = Ximod::new("Old");
        old.steps.push(Step::new("S1"));
        fake_backup(&root, "20260101-100000", &old);
        let mut app = app();
        let mut current = Ximod::new("Current");
        current.steps.push(Step::new("A"));
        current.steps.push(Step::new("B"));
        app.open_loaded(current, root.clone());
        app.open_backups_dialog();
        assert!(app.show_backups);
        assert_eq!(app.backups.len(), 1);
        // name change + one step removed + two added
        assert_eq!(app.backups[0].changes, Some(4));
        assert_eq!(app.backups[0].entry.display_time(), "2026-01-01 10:00:00");

        app.compare_with_backup(0);
        assert!(app.show_compare);
        assert!(!app.show_backups, "the modal list closes to reveal the diff window");
        assert_eq!(app.compare_result.as_ref().map(|d| d.total()), Some(4));

        app.restore_backup(0);
        assert_eq!(app.ximod.name, "Old");
        assert!(app.project_modified);
        assert!(!app.show_backups);
        assert!(app.history.can_undo());
        app.undo();
        assert_eq!(app.ximod.name, "Current");
        assert_eq!(app.ximod.steps.len(), 2);

        // Two-click delete: the first click only arms the button.
        app.open_backups_dialog();
        app.backups_delete_armed = true;
        app.delete_all_backups();
        assert!(app.backups.is_empty());
        assert!(!backups::has_backups(&root));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn dialog_renders_headless_with_and_without_backups() {
        let root = scratch("render");
        let mut app = app();
        app.open_loaded(Ximod::new("Cur"), root.clone());
        let ctx = egui::Context::default();
        app.open_backups_dialog();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_backups_dialog(ctx));
        assert!(app.show_backups);
        fake_backup(&root, "20260102-120000", &Ximod::new("Older"));
        app.refresh_backups();
        assert_eq!(app.backups.len(), 1);
        app.backups_delete_armed = true;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_backups_dialog(ctx));
        assert!(app.show_backups);
        // Without a root the dialog refuses to open.
        app.show_backups = false;
        app.new_project();
        app.open_backups_dialog();
        assert!(!app.show_backups);
        let _ = std::fs::remove_dir_all(&root);
    }
}
