//! Project → Find missing files…: the references whose file, folder or
//! image no longer exists, each with the candidates found in the mod folder
//! (same name elsewhere → moved; similar name where it was → renamed). The
//! author picks a new location per line (or browses for one) and applies
//! them all at once, as one undoable change.

use crate::models::file_refs::{self, MissingRef, Reason, RefKind};
use crate::ui::main_window::XimodApp;
use crate::ui::theme::icon;
use eframe::egui::{self, RichText};

const WINDOW_ID: &str = "ximod_relink";
const WINDOW_SIZE: [f32; 2] = [820.0, 480.0];

/// State of the window.
#[derive(Debug, Clone, Default)]
pub struct RelinkState {
    pub items: Vec<MissingRef>,
    /// Chosen new location per item (`/` separators), `None` = leave as is.
    pub choice: Vec<Option<String>>,
}

impl RelinkState {
    /// Scan `root` for the missing references of `ximod`; the obvious
    /// candidates (one file of the same name) are pre-selected.
    pub fn scan(ximod: &crate::models::Ximod, root: &std::path::Path) -> Self {
        let items = file_refs::find_missing(ximod, root);
        let choice = items.iter().map(|m| m.obvious().map(|c| c.rel.clone())).collect();
        Self { items, choice }
    }

    /// The choices to apply.
    pub fn chosen(&self) -> Vec<(String, String, RefKind)> {
        self.items
            .iter()
            .zip(&self.choice)
            .filter_map(|(m, c)| c.as_ref().map(|c| (m.path.clone(), c.clone(), m.kind)))
            .collect()
    }
}

fn kind_icon(kind: RefKind) -> &'static str {
    match kind {
        RefKind::File => icon::FILE,
        RefKind::Folder => icon::FOLDER,
        RefKind::Image => icon::IMAGE,
    }
}

impl XimodApp {
    /// Open the window (Project menu), scanning the mod folder.
    pub(crate) fn open_relink(&mut self) {
        let Some(root) = self.root_directory.clone() else {
            let msg = self.i18n.t("msg-no-root-selected");
            self.notify_err(msg);
            return;
        };
        self.relink = Some(RelinkState::scan(&self.ximod, &root));
    }

    /// Apply the chosen locations (one undo step), then rescan.
    pub(crate) fn apply_relink(&mut self) {
        let (Some(state), Some(root)) = (self.relink.as_ref(), self.root_directory.clone()) else {
            return;
        };
        let chosen = state.chosen();
        let n = file_refs::apply_relinks(&mut self.ximod, &chosen);
        if n > 0 {
            self.mark_modified();
        }
        self.notify_ok(self.i18n.t_num("msg-relinked", n as i64));
        self.relink = Some(RelinkState::scan(&self.ximod, &root));
    }

    /// Browse for the new location of item `i`.
    fn browse_relink(&mut self, i: usize) {
        let (Some(root), Some(kind)) = (
            self.root_directory.clone(),
            self.relink.as_ref().and_then(|s| s.items.get(i)).map(|m| m.kind),
        ) else {
            return;
        };
        let start = crate::ui::main_window::pick_start_dir(&root, self.last_pick_dir.as_deref());
        let dialog = rfd::FileDialog::new().set_directory(&start);
        let picked = if kind.is_folder() {
            dialog.pick_folder()
        } else {
            dialog.pick_file()
        };
        let Some(picked) = picked else { return };
        if let Some(dir) = crate::ui::main_window::remembered_pick_dir(std::slice::from_ref(&picked)) {
            self.last_pick_dir = Some(dir);
        }
        match file_refs::relative_to(&root, &picked) {
            Some(rel) => {
                if let Some(c) = self.relink.as_mut().and_then(|s| s.choice.get_mut(i)) {
                    *c = Some(rel);
                }
            }
            None => {
                let msg = self.i18n.t("msg-file-outside-root");
                self.notify_warn(msg);
            }
        }
    }

    /// Render the window (independent viewport).
    pub(crate) fn render_relink(&mut self, ctx: &egui::Context) {
        if self.relink.is_none() {
            self.free_window_closed(WINDOW_ID);
            return;
        }
        let t = |k: &str| self.i18n.t(k);
        let title = t("relink-title");
        let hint = t("relink-hint");
        let none = t("relink-none");
        let col_missing = t("relink-col-missing");
        let col_uses = t("relink-col-uses");
        let col_new = t("relink-col-new");
        let keep = t("relink-keep");
        let moved = t("relink-moved");
        let renamed = t("relink-renamed");
        let browse = t("relink-browse");
        let rescan = t("relink-rescan");
        let close = t("btn-close");
        let chosen_n = self.relink.as_ref().map_or(0, |s| s.choice.iter().flatten().count());
        let apply = self.i18n.t_num("btn-apply-relink", chosen_n as i64);
        let vb = self.free_viewport_builder(ctx, WINDOW_ID, title, WINDOW_SIZE, false);

        let cfg = &mut self.config;
        let Some(state) = self.relink.as_mut() else { return };
        let mut do_close = false;
        let mut do_apply = false;
        let mut do_rescan = false;
        let mut browse_for: Option<usize> = None;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of(WINDOW_ID), vb, |ctx, _class| {
            egui::TopBottomPanel::bottom("relink_bottom").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui.button(&rescan).clicked() {
                        do_rescan = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&close).clicked() {
                            do_close = true;
                        }
                        if ui.add_enabled(chosen_n > 0, egui::Button::new(&apply)).clicked() {
                            do_apply = true;
                        }
                    });
                });
                ui.add_space(4.0);
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(&hint);
                ui.add_space(6.0);
                if state.items.is_empty() {
                    ui.label(RichText::new(&none).strong());
                    return;
                }
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    egui::Grid::new("relink_grid")
                        .num_columns(3)
                        .striped(true)
                        .spacing([12.0, 6.0])
                        .show(ui, |ui| {
                            ui.label(RichText::new(&col_missing).strong());
                            ui.label(RichText::new(&col_uses).strong());
                            ui.label(RichText::new(&col_new).strong());
                            ui.end_row();
                            for (i, item) in state.items.iter().enumerate() {
                                ui.label(format!("{} {}", kind_icon(item.kind), item.path));
                                ui.label(format!("×{}", item.uses));
                                ui.horizontal(|ui| {
                                    let current = state.choice[i].clone();
                                    let shown = current.clone().unwrap_or_else(|| keep.clone());
                                    egui::ComboBox::from_id_salt(("relink_choice", i))
                                        .selected_text(shown)
                                        .width(380.0)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut state.choice[i], None, &keep);
                                            for c in &item.candidates {
                                                let why = match c.reason {
                                                    Reason::Moved => &moved,
                                                    Reason::Renamed => &renamed,
                                                };
                                                ui.selectable_value(
                                                    &mut state.choice[i],
                                                    Some(c.rel.clone()),
                                                    format!("{}  —  {why}", c.rel),
                                                );
                                            }
                                            // A browsed path that is not a candidate stays listed.
                                            if let Some(cur) = &current
                                                && !item.candidates.iter().any(|c| &c.rel == cur)
                                            {
                                                ui.selectable_value(&mut state.choice[i], Some(cur.clone()), cur);
                                            }
                                        });
                                    if ui.small_button(&browse).clicked() {
                                        browse_for = Some(i);
                                    }
                                });
                                ui.end_row();
                            }
                        });
                });
            });
            crate::ui::widgets::free_window::record_win_geom(cfg, ctx, WINDOW_ID);
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        if let Some(i) = browse_for {
            self.browse_relink(i);
        }
        if do_apply {
            self.apply_relink();
        }
        if do_rescan && let Some(root) = self.root_directory.clone() {
            self.relink = Some(RelinkState::scan(&self.ximod, &root));
        }
        if do_close {
            self.relink = None;
            self.free_window_closed(WINDOW_ID);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{InstallFile, Plugin, PluginGroup, SelectionType, Step};

    #[test]
    fn relink_window_scans_preselects_and_applies() {
        let root = std::env::temp_dir().join(format!("ximod_relink_ui_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("Common")).unwrap();
        std::fs::write(root.join("Common/Main.esp"), b"x").unwrap();

        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let mut p = Plugin::new("A");
        p.files.push(InstallFile::new_file("Old\\Main.esp"));
        p.files.push(InstallFile::new_file("Old\\Gone.esp"));
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        app.ximod.steps.push(s);
        app.root_directory = Some(root.clone());

        app.open_relink();
        let state = app.relink.as_ref().unwrap();
        assert_eq!(state.items.len(), 2);
        assert_eq!(state.chosen().len(), 1, "only the obvious match is pre-selected");
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_relink(ctx));

        app.apply_relink();
        assert_eq!(
            app.ximod.steps[0].plugin_groups[0].plugins[0].files[0].source,
            "Common\\Main.esp"
        );
        assert!(app.project_modified);
        // After the rescan, only the unresolved reference is left.
        assert_eq!(app.relink.as_ref().unwrap().items.len(), 1);
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_relink(ctx));
        let _ = std::fs::remove_dir_all(&root);
    }
}
