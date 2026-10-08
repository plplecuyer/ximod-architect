//! "Plugin report" dialog (Tools menu): every plugin file the project
//! installs, with what its header says — kind, light flag, masters (the
//! ones nothing provides in the warning colour), new records against the
//! light-plugin limit and the resulting eligibility.

use crate::models::plugin_checks::{PluginInfo, collect_plugins, file_name_of};
use crate::models::plugin_header::{EslLimits, PluginKind};
use crate::ui::components::{modal_keys, modal_window};
use crate::ui::main_window::XimodApp;
use eframe::egui::{self, RichText};

/// One row of the report, ready to display.
#[derive(Debug, Clone)]
pub struct PluginRow {
    /// Project-relative source path.
    pub source: String,
    pub kind: PluginKind,
    /// `None` when the header could not be read.
    pub light: Option<bool>,
    /// `(master, missing)`: missing = neither shipped, nor base, nor declared.
    pub masters: Vec<(String, bool)>,
    /// `(new records, limit)` when the records were counted.
    pub records: Option<(u32, u32)>,
    /// Light-plugin eligibility, when known.
    pub eligible: Option<bool>,
    pub error: Option<String>,
}

impl PluginRow {
    fn from_info(info: PluginInfo) -> Self {
        let masters = info
            .header
            .as_ref()
            .map(|h| {
                h.masters
                    .iter()
                    .map(|m| (m.clone(), info.missing_masters.iter().any(|x| x == m)))
                    .collect()
            })
            .unwrap_or_default();
        Self {
            source: info.source,
            kind: info.kind,
            light: info.header.as_ref().map(|h| h.light),
            masters,
            records: info.report.as_ref().map(|r| (r.new_forms, r.limit)),
            eligible: info.report.as_ref().map(|r| r.eligible),
            error: info.error,
        }
    }
}

impl XimodApp {
    /// Scan the project's plugins and open the report.
    pub(crate) fn open_plugin_report(&mut self) {
        self.refresh_plugin_report();
        self.show_plugin_report = true;
    }

    /// (Re)scan the plugins of the active project.
    pub(crate) fn refresh_plugin_report(&mut self) {
        let Some(root) = self.root_directory.clone() else {
            self.plugin_report.clear();
            return;
        };
        let base: Vec<String> = self.games.base_masters_for(&self.ximod.game).to_vec();
        let limits = EslLimits::for_game(&self.games, &self.ximod.game);
        self.plugin_report = collect_plugins(&self.ximod, &root, &base, limits)
            .into_iter()
            .map(PluginRow::from_info)
            .collect();
    }

    /// Render the modal plugin report.
    pub(crate) fn render_plugin_report(&mut self, ctx: &egui::Context) {
        if !self.show_plugin_report {
            return;
        }
        let title = self.i18n.t("plugins-title");
        let empty = if self.root_directory.is_none() {
            self.i18n.t("verify-no-root")
        } else {
            self.i18n.t("plugins-empty")
        };
        let col_file = self.i18n.t("plugins-file");
        let col_kind = self.i18n.t("plugins-kind");
        let col_light = self.i18n.t("plugins-light");
        let col_masters = self.i18n.t("plugins-masters");
        let col_records = self.i18n.t("plugins-new-records");
        let col_eligible = self.i18n.t("plugins-eligible");
        let unreadable = self.i18n.t("plugins-unreadable");
        let yes = self.i18n.t("btn-yes");
        let no = self.i18n.t("btn-no");
        let close = self.i18n.t("btn-close");
        let rows = std::mem::take(&mut self.plugin_report);

        let (_enter, esc) = modal_keys(ctx);
        let mut do_close = false;
        modal_window(ctx, &title, |ui| {
            ui.set_min_width(640.0);
            if rows.is_empty() {
                ui.label(&empty);
            } else {
                let palette = crate::ui::theme::Palette::from_ui(ui);
                // The masters column is as wide as the longest master name, so
                // names are never broken across lines (one master per line).
                let font_id = egui::TextStyle::Body.resolve(ui.style());
                let masters_w = rows
                    .iter()
                    .flat_map(|r| r.masters.iter())
                    .map(|(m, _)| {
                        ui.fonts(|f| {
                            f.layout_no_wrap(m.clone(), font_id.clone(), egui::Color32::WHITE)
                                .size()
                                .x
                        })
                    })
                    .fold(60.0_f32, f32::max)
                    + 8.0;
                egui::ScrollArea::both()
                    .max_height(360.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        egui::Grid::new("plugin_report_grid")
                            .num_columns(6)
                            .striped(true)
                            .spacing([14.0, 6.0])
                            .show(ui, |ui| {
                                for c in [
                                    &col_file,
                                    &col_kind,
                                    &col_light,
                                    &col_masters,
                                    &col_records,
                                    &col_eligible,
                                ] {
                                    ui.label(RichText::new(c).strong());
                                }
                                ui.end_row();
                                for row in &rows {
                                    ui.label(file_name_of(&row.source)).on_hover_text(&row.source);
                                    ui.label(format!("{:?}", row.kind).to_ascii_lowercase());
                                    match row.light {
                                        Some(true) => ui.label(&yes),
                                        Some(false) => ui.label(&no),
                                        None => ui.label("–"),
                                    };
                                    ui.vertical(|ui| {
                                        ui.set_min_width(masters_w);
                                        if row.masters.is_empty() {
                                            ui.label("–");
                                        }
                                        for (m, missing) in &row.masters {
                                            let text = RichText::new(m);
                                            let text = if *missing { text.color(palette.warning) } else { text };
                                            ui.add(egui::Label::new(text).wrap_mode(egui::TextWrapMode::Extend));
                                        }
                                    });
                                    match row.records {
                                        Some((n, limit)) => ui.label(format!("{n} / {limit}")),
                                        None => ui.label("–"),
                                    };
                                    match (row.eligible, &row.error) {
                                        (_, Some(e)) => ui
                                            .label(RichText::new(&unreadable).color(palette.danger))
                                            .on_hover_text(e),
                                        (Some(true), None) => ui.colored_label(palette.success, &yes),
                                        (Some(false), None) => ui.label(&no),
                                        (None, None) => ui.label("–"),
                                    };
                                    ui.end_row();
                                }
                            });
                    });
            }
            ui.add_space(12.0);
            if ui.button(&close).clicked() || esc {
                do_close = true;
            }
        });
        self.plugin_report = rows;
        if do_close {
            self.show_plugin_report = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::plugin_header::tests::build_plugin;
    use crate::models::{InstallFile, Plugin, PluginGroup, SelectionType, Step, Ximod};

    #[test]
    fn report_rows_and_headless_render() {
        let root = std::env::temp_dir().join(format!("ximod_plugin_report_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("Mod.esp"),
            build_plugin(0x200, &["Skyrim.esm", "Lib.esm"], &[]),
        )
        .unwrap();
        std::fs::write(root.join("Bad.esm"), b"junk").unwrap();

        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let mut x = Ximod::new("T");
        let mut p = Plugin::new("Opt");
        p.files.push(InstallFile::new_file("Mod.esp"));
        p.files.push(InstallFile::new_file("readme.txt"));
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        x.steps.push(s);
        x.required_files.push(InstallFile::new_file("Bad.esm"));
        app.open_loaded(x, root.clone());

        app.open_plugin_report();
        assert!(app.show_plugin_report);
        assert_eq!(app.plugin_report.len(), 2);
        let bad = app.plugin_report.iter().find(|r| r.source == "Bad.esm").unwrap();
        assert!(bad.error.is_some() && bad.light.is_none());
        let good = app.plugin_report.iter().find(|r| r.source == "Mod.esp").unwrap();
        assert_eq!(good.light, Some(true));
        assert_eq!(good.records, Some((0, 2048)));
        assert_eq!(good.eligible, Some(true));
        // Lib.esm is missing; Skyrim.esm only when the games dataset is absent.
        assert!(good.masters.iter().any(|(m, missing)| m == "Lib.esm" && *missing));

        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_plugin_report(ctx));
        assert!(app.show_plugin_report);
        assert_eq!(app.plugin_report.len(), 2, "rows survive a frame");

        // Without a root: an empty report that still renders.
        app.new_project();
        app.open_plugin_report();
        assert!(app.plugin_report.is_empty());
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_plugin_report(ctx));
        let _ = std::fs::remove_dir_all(&root);
    }
}
