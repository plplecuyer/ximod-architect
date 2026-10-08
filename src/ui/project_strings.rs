//! "Project strings" window (Tools menu): every name and description of the
//! active project in one table (see `models/strings.rs`), with search, a
//! kind filter, a duplicates filter, find / replace and inline editing. A
//! double-click on a row's location selects the node in the main window.
//!
//! The table is rebuilt only when `project_revision` changes, never while a
//! cell is being typed in.

use std::collections::HashMap;

use crate::models::strings::{duplicates, model_units, replace_all, replace_in, set_unit};
use crate::models::translate::{TField, TUnit, key_indices};
use crate::ui::fomod_translation::field_key;
use crate::ui::main_window::XimodApp;
use crate::ui::problems::Target;
use crate::ui::theme::{Palette, icon};
use crate::ui::widgets::free_window::record_win_geom;
use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};

const WINDOW_ID: &str = "ximod_strings";

/// Kind filter of the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StringKind {
    #[default]
    All,
    Names,
    Descriptions,
}

impl StringKind {
    /// The fields a kind covers (`None` = every field).
    pub fn fields(self) -> Option<&'static [TField]> {
        match self {
            Self::All => None,
            Self::Names => Some(&[
                TField::InfoName,
                TField::InfoAuthor,
                TField::InfoWebsite,
                TField::ModuleName,
                TField::StepName,
                TField::GroupName,
                TField::PluginName,
            ]),
            Self::Descriptions => Some(&[TField::InfoDescription, TField::PluginDescription]),
        }
    }

    fn key(self) -> &'static str {
        match self {
            Self::All => "strings-kind-all",
            Self::Names => "strings-kind-names",
            Self::Descriptions => "strings-kind-descriptions",
        }
    }
}

/// Free-window state.
#[derive(Default)]
pub struct ProjectStringsState {
    pub open: bool,
    /// The strings of the model at `revision`.
    pub units: Vec<TUnit>,
    revision: Option<u64>,
    /// Unit index → indices of the other units with the same text.
    dup_of: HashMap<usize, Vec<usize>>,
    /// Number of duplicate groups.
    pub dup_groups: usize,
    pub search: String,
    pub kind: StringKind,
    pub dups_only: bool,
    pub replace_with: String,
    pub case_sensitive: bool,
    pub whole_word: bool,
    /// Row last clicked or edited.
    pub cursor: Option<usize>,
    /// Indices of the rows passing the filters.
    pub visible: Vec<usize>,
    /// A cell is being typed in: the table is not rebuilt meanwhile.
    editing: bool,
    /// Width of the text column measured on the previous frame, so row
    /// heights can be computed from the real wrapping width.
    text_col_w: f32,
}

impl ProjectStringsState {
    /// Rebuild the units from the model when the project changed (unless a
    /// cell is being edited, which would move the caret).
    pub fn refresh(&mut self, ximod: &crate::models::Ximod, revision: u64) {
        if self.revision == Some(revision) || self.editing {
            return;
        }
        self.units = model_units(ximod);
        let groups = duplicates(&self.units);
        self.dup_groups = groups.len();
        self.dup_of.clear();
        for idx in groups.into_values() {
            for &i in &idx {
                self.dup_of.insert(i, idx.iter().copied().filter(|&j| j != i).collect());
            }
        }
        if self.cursor.is_some_and(|c| c >= self.units.len()) {
            self.cursor = None;
        }
        self.revision = Some(revision);
    }

    /// Recompute the rows passing the search, kind and duplicates filters.
    pub fn refresh_visible(&mut self) {
        let needle = self.search.trim().to_lowercase();
        let fields = self.kind.fields();
        self.visible = self
            .units
            .iter()
            .enumerate()
            .filter(|(_, u)| fields.is_none_or(|f| f.contains(&u.field)))
            .filter(|(i, _)| !self.dups_only || self.dup_of.contains_key(i))
            .filter(|(_, u)| {
                needle.is_empty()
                    || u.source.to_lowercase().contains(&needle)
                    || u.context.to_lowercase().contains(&needle)
                    || u.key.to_lowercase().contains(&needle)
            })
            .map(|(i, _)| i)
            .collect();
    }

    /// Replacements for "Replace all": every visible row containing the
    /// search text.
    pub fn replacements_all(&self) -> Vec<(usize, String)> {
        let needle = self.search.as_str();
        replace_all(
            &self.units,
            needle,
            &self.replace_with,
            self.case_sensitive,
            self.whole_word,
            self.kind.fields(),
        )
        .into_iter()
        .filter(|(i, _)| self.visible.contains(i))
        .collect()
    }

    /// Replacement for "Replace": the row under the cursor, when it matches.
    pub fn replacement_current(&self) -> Option<(usize, String)> {
        let i = self.cursor?;
        let u = self.units.get(i)?;
        replace_in(
            &u.source,
            &self.search,
            &self.replace_with,
            self.case_sensitive,
            self.whole_word,
        )
        .map(|t| (i, t))
    }
}

/// The tree node a unit key lives in (`info/…` and `config/moduleName` are
/// the Mod information).
pub fn target_of_key(key: &str) -> Option<Target> {
    if key.starts_with("info/") || key == "config/moduleName" {
        return Some(Target::Info);
    }
    match key_indices(key)[..] {
        [s] => Some(Target::Step(s)),
        [s, g] => Some(Target::Group(s, g)),
        [s, g, p] => Some(Target::Plugin(s, g, p)),
        _ => None,
    }
}

/// Location shown for a unit: its breadcrumb, or the field name for the
/// header strings.
fn location_text(i18n: &crate::i18n::I18n, u: &TUnit) -> String {
    if u.context.is_empty() {
        i18n.t(field_key(u.field))
    } else {
        u.context.clone()
    }
}

impl XimodApp {
    /// Open the window.
    pub(crate) fn open_project_strings(&mut self) {
        self.project_strings.open = true;
    }

    /// Render the window and apply its edits to the model.
    pub(crate) fn render_project_strings(&mut self, ctx: &egui::Context) {
        if !self.project_strings.open {
            self.free_window_closed(WINDOW_ID);
            return;
        }
        let revision = self.project_revision;
        self.project_strings.refresh(&self.ximod, revision);
        let title = self.i18n.t("strings-title");
        let vb = self.free_viewport_builder(ctx, WINDOW_ID, title, [980.0, 640.0], false);
        let st = &mut self.project_strings;
        let i18n = &self.i18n;
        let cfg = &mut self.config;

        let l_search = i18n.t("strings-search");
        let l_dups_only = i18n.t("strings-duplicates-only");
        let l_replace_with = i18n.t("strings-replace-with");
        let l_case = i18n.t("strings-case");
        let l_whole = i18n.t("strings-whole-word");
        let l_replace = i18n.t("strings-replace-current");
        let l_replace_all = i18n.t("strings-replace-all");
        let l_close = i18n.t("btn-close");
        let l_col_loc = i18n.t("strings-col-location");
        let l_col_field = i18n.t("strings-col-field");
        let l_col_text = i18n.t("strings-col-text");
        let l_empty = i18n.t("ftr-empty-filter");
        let l_dup_hover = i18n.t("strings-dup-hover");

        let mut do_close = false;
        let mut go: Option<Target> = None;
        // (unit index, new text) edits to write to the model.
        let mut edits: Vec<(usize, String)> = Vec::new();
        let mut replaced: Option<usize> = None;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of(WINDOW_ID), vb, |ctx, _class| {
            // ---- filters and find / replace ----
            egui::TopBottomPanel::top("strings_filters").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(icon::SEARCH);
                    ui.add(
                        egui::TextEdit::singleline(&mut st.search)
                            .hint_text(&l_search)
                            .desired_width(260.0),
                    );
                    egui::ComboBox::from_id_salt("strings_kind")
                        .selected_text(i18n.t(st.kind.key()))
                        .show_ui(ui, |ui| {
                            for kind in [StringKind::All, StringKind::Names, StringKind::Descriptions] {
                                ui.selectable_value(&mut st.kind, kind, i18n.t(kind.key()));
                            }
                        });
                    ui.checkbox(&mut st.dups_only, &l_dups_only);
                });
                ui.horizontal(|ui| {
                    ui.label(&l_replace_with);
                    ui.add(egui::TextEdit::singleline(&mut st.replace_with).desired_width(220.0));
                    ui.checkbox(&mut st.case_sensitive, &l_case);
                    ui.checkbox(&mut st.whole_word, &l_whole);
                    let can = !st.search.is_empty();
                    st.refresh_visible();
                    if ui.add_enabled(can, egui::Button::new(&l_replace)).clicked()
                        && let Some(r) = st.replacement_current()
                    {
                        edits.push(r);
                        replaced = Some(1);
                    }
                    if ui.add_enabled(can, egui::Button::new(&l_replace_all)).clicked() {
                        let all = st.replacements_all();
                        replaced = Some(all.len());
                        edits.extend(all);
                    }
                });
                ui.add_space(4.0);
            });
            // ---- footer ----
            egui::TopBottomPanel::bottom("strings_footer").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let mut args = fluent::FluentArgs::new();
                    args.set("num", st.units.len() as i64);
                    args.set("dups", st.dup_groups as i64);
                    ui.label(RichText::new(i18n.t_with_args("strings-count", Some(&args))).weak());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&l_close).clicked() {
                            do_close = true;
                        }
                    });
                });
                ui.add_space(4.0);
            });
            // ---- table ----
            egui::CentralPanel::default().show(ctx, |ui| {
                st.refresh_visible();
                let visible = st.visible.clone();
                let palette = Palette::from_ui(ui);
                let row_h = ui.text_style_height(&egui::TextStyle::Body) + 8.0;
                // Row heights follow the wrapped text: each multi-line value is
                // laid out at the text column's width (measured last frame, the
                // window width otherwise) and the row takes the resulting height.
                let wrap_w = if st.text_col_w > 20.0 {
                    st.text_col_w
                } else {
                    (ui.available_width() - 260.0 - 150.0 - 40.0).max(160.0)
                };
                let font_id = egui::TextStyle::Body.resolve(ui.style());
                let heights: Vec<f32> = visible
                    .iter()
                    .map(|&i| {
                        let u = &st.units[i];
                        if u.field.is_multiline() {
                            let galley = ui.fonts(|f| {
                                f.layout(u.source.clone(), font_id.clone(), egui::Color32::WHITE, wrap_w - 12.0)
                            });
                            (galley.size().y + 10.0).max(row_h)
                        } else {
                            row_h
                        }
                    })
                    .collect();
                let mut measured_w: Option<f32> = None;
                let mut editing = false;
                let mut new_cursor: Option<usize> = None;
                // Keyboard: ↑ ↓ Page Up/Down Home End move the selected row
                // (not while a cell is being typed in).
                let page = ((ui.available_height() - row_h * 2.0) / row_h.max(1.0))
                    .floor()
                    .max(1.0) as usize;
                let cur_pos = st.cursor.and_then(|c| visible.iter().position(|&i| i == c));
                let mut scroll_to: Option<usize> = None;
                if let Some(p) =
                    crate::ui::components::table_nav_keys(ctx, cur_pos, visible.len(), page, ctx.wants_keyboard_input())
                {
                    st.cursor = Some(visible[p]);
                    scroll_to = Some(p);
                }
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .sense(egui::Sense::click())
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(260.0).clip(true).at_least(100.0))
                    .column(Column::initial(150.0).clip(true).at_least(80.0))
                    .column(Column::remainder().clip(true).at_least(160.0))
                    .min_scrolled_height(0.0);
                if let Some(p) = scroll_to {
                    table = table.scroll_to_row(p, Some(egui::Align::Center));
                }
                let cursor = st.cursor;
                let dup_of = &st.dup_of;
                let units = &mut st.units;
                table
                    .header(row_h, |mut h| {
                        h.col(|ui| {
                            ui.label(RichText::new(&l_col_loc).strong());
                        });
                        h.col(|ui| {
                            ui.label(RichText::new(&l_col_field).strong());
                        });
                        h.col(|ui| {
                            ui.label(RichText::new(&l_col_text).strong());
                        });
                    })
                    .body(|body| {
                        if visible.is_empty() {
                            body.rows(row_h, 1, |mut row| {
                                row.col(|ui| {
                                    ui.label(RichText::new(&l_empty).weak());
                                });
                                row.col(|_| {});
                                row.col(|_| {});
                            });
                            return;
                        }
                        body.heterogeneous_rows(heights.into_iter(), |mut row| {
                            let pos = row.index();
                            let idx = visible[pos];
                            row.set_selected(cursor == Some(idx));
                            let (loc, field_label, dup_tip) = {
                                let u = &units[idx];
                                let dups = dup_of.get(&idx);
                                let tip = dups.map(|others| {
                                    let mut tip = l_dup_hover.clone();
                                    for &j in others {
                                        tip.push('\n');
                                        tip.push_str(&location_text(i18n, &units[j]));
                                    }
                                    (others.len() + 1, tip)
                                });
                                (location_text(i18n, u), i18n.t(field_key(u.field)), tip)
                            };
                            row.col(|ui| {
                                ui.style_mut().interaction.selectable_labels = false;
                                let resp = ui.add(
                                    egui::Label::new(RichText::new(&loc).weak())
                                        .truncate()
                                        .sense(egui::Sense::click()),
                                );
                                if resp.double_clicked()
                                    && let Some(t) = target_of_key(&units[idx].key)
                                {
                                    go = Some(t);
                                }
                                if let Some((n, tip)) = &dup_tip {
                                    ui.label(
                                        RichText::new(i18n.t_num("strings-dup-badge", *n as i64))
                                            .small()
                                            .color(palette.accent),
                                    )
                                    .on_hover_text(tip);
                                }
                            });
                            row.col(|ui| {
                                ui.style_mut().interaction.selectable_labels = false;
                                ui.add(egui::Label::new(RichText::new(&field_label).weak()).truncate());
                            });
                            row.col(|ui| {
                                measured_w = Some(ui.available_width());
                                let unit = &mut units[idx];
                                let resp = if unit.field.is_multiline() {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut unit.source)
                                            .desired_width(f32::INFINITY)
                                            .desired_rows(1),
                                    )
                                } else {
                                    ui.add(egui::TextEdit::singleline(&mut unit.source).desired_width(f32::INFINITY))
                                };
                                if resp.has_focus() {
                                    editing = true;
                                }
                                if resp.gained_focus() {
                                    new_cursor = Some(idx);
                                }
                                if resp.changed() {
                                    edits.push((idx, unit.source.clone()));
                                }
                            });
                            if row.response().clicked() {
                                new_cursor = Some(idx);
                            }
                        });
                    });
                st.editing = editing;
                if let Some(w) = measured_w
                    && (w - st.text_col_w).abs() > 0.5
                {
                    // The column width changed (window resized, column dragged):
                    // the heights computed above were for the old width.
                    st.text_col_w = w;
                    ctx.request_repaint();
                }
                if let Some(c) = new_cursor {
                    st.cursor = Some(c);
                }
            });

            record_win_geom(cfg, ctx, WINDOW_ID);
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        // ---- write the edits to the model ----
        let mut changed = false;
        for (idx, text) in edits {
            let Some(u) = self.project_strings.units.get_mut(idx) else {
                continue;
            };
            if set_unit(&mut self.ximod, &u.key, u.field, &text) {
                u.source = text;
                changed = true;
            }
        }
        if changed {
            self.mark_modified();
        }
        if let Some(n) = replaced {
            let msg = self.i18n.t_num("strings-replaced", n as i64);
            self.notify_ok(msg);
        }
        if let Some(t) = go {
            self.select_target(t);
        }
        if do_close {
            self.project_strings.open = false;
            self.free_window_closed(WINDOW_ID);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Plugin, PluginGroup, SelectionType, Step, Ximod};

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.ximod = Ximod::new("Aurelia");
        app.ximod.description = "Big mod".into();
        let mut step = Step::new("Textures");
        let mut group = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        let mut p = Plugin::new("4K");
        p.description = "High resolution".into();
        group.plugins.push(p);
        let mut q = Plugin::new("2K");
        q.description = "high  resolution".into();
        group.plugins.push(q);
        step.plugin_groups.push(group);
        app.ximod.steps.push(step);
        app.history.reset(&app.ximod);
        app
    }

    #[test]
    fn keys_map_to_tree_targets() {
        assert_eq!(target_of_key("info/Name"), Some(Target::Info));
        assert_eq!(target_of_key("config/moduleName"), Some(Target::Info));
        assert_eq!(target_of_key("config/step[2]/name"), Some(Target::Step(2)));
        assert_eq!(target_of_key("config/step[0]/group[1]/name"), Some(Target::Group(0, 1)));
        assert_eq!(
            target_of_key("config/step[0]/group[1]/plugin[3]/description"),
            Some(Target::Plugin(0, 1, 3))
        );
        assert_eq!(target_of_key("bogus"), None);
    }

    #[test]
    fn table_is_rebuilt_per_revision_and_filtered() {
        let mut app = app();
        let st = &mut app.project_strings;
        st.refresh(&app.ximod, 1);
        assert_eq!(st.units.len(), 9);
        assert_eq!(st.dup_groups, 1);
        assert_eq!(st.dup_of.get(&6), Some(&vec![8]));
        assert_eq!(st.dup_of.get(&0), None);
        st.refresh_visible();
        assert_eq!(st.visible.len(), 9);
        st.kind = StringKind::Descriptions;
        st.refresh_visible();
        assert_eq!(st.visible.len(), 3);
        st.dups_only = true;
        st.refresh_visible();
        assert_eq!(st.visible, vec![6, 8]);
        st.kind = StringKind::All;
        st.dups_only = false;
        st.search = "RESOL".into();
        st.refresh_visible();
        assert_eq!(st.visible.len(), 5); // group name, 2 plugin names (context), 2 descriptions
        // Same revision: no rebuild.
        app.ximod.name = "Changed".into();
        let st = &mut app.project_strings;
        st.refresh(&app.ximod, 1);
        assert_eq!(st.units[0].source, "Aurelia");
        st.refresh(&app.ximod, 2);
        assert_eq!(st.units[0].source, "Changed");
    }

    #[test]
    fn replace_current_and_all_follow_the_filters() {
        let mut app = app();
        let st = &mut app.project_strings;
        st.refresh(&app.ximod, 1);
        st.search = "resolution".into();
        st.replace_with = "res".into();
        st.refresh_visible();
        assert_eq!(st.replacements_all().len(), 3);
        st.kind = StringKind::Names;
        st.refresh_visible();
        let all = st.replacements_all();
        assert_eq!(all, vec![(4, "res".to_string())]);
        st.case_sensitive = true;
        st.kind = StringKind::All;
        st.search = "High".into();
        st.refresh_visible();
        assert_eq!(st.replacements_all(), vec![(6, "res resolution".to_string())]);
        assert_eq!(st.replacement_current(), None);
        st.cursor = Some(6);
        assert_eq!(st.replacement_current(), Some((6, "res resolution".to_string())));
        st.cursor = Some(8);
        assert_eq!(st.replacement_current(), None);
    }

    /// The window lays out headless and a plain render never dirties the
    /// project; an edit goes through `set_unit` and is undoable.
    #[test]
    fn window_renders_headless_and_edits_write_through() {
        let mut app = app();
        app.open_project_strings();
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_project_strings(ctx));
        assert!(!app.project_modified);
        assert_eq!(app.project_strings.units.len(), 9);
        // Simulate an inline edit of the plugin description.
        let u = &app.project_strings.units[6];
        assert!(set_unit(&mut app.ximod, &u.key, u.field, "Ultra"));
        app.mark_modified();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_project_strings(ctx));
        assert_eq!(app.project_strings.units[6].source, "Ultra");
        assert_eq!(app.project_strings.dup_groups, 0);
        app.undo();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_project_strings(ctx));
        assert_eq!(app.project_strings.units[6].source, "High resolution");
        app.project_strings.open = false;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_project_strings(ctx));
    }
}
