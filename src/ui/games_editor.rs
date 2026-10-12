//! Games editor — an independent window (Tools menu) to add, correct or remove
//! the games and their Nexus categories, so the user never has to edit
//! `Categories.json` by hand. Edits are validated (unique non-empty id, a name
//! for every game) and only written to disk — and applied to the running
//! app — when the user clicks Save; the file is saved to the writable location
//! resolved by [`crate::games::GamesData::save`].

use eframe::egui::{self, RichText};

use crate::games::{GameEntry, GamesData};
use crate::ui::main_window::XimodApp;

/// Light-plugin record limit choice for a game.
#[derive(Clone, Copy, PartialEq, Eq)]
enum EslChoice {
    Default,
    L2048,
    L4096,
}

impl EslChoice {
    fn from_limit(v: Option<u32>) -> Self {
        match v {
            Some(2048) => Self::L2048,
            Some(4096) => Self::L4096,
            _ => Self::Default,
        }
    }
    fn to_limit(self) -> Option<u32> {
        match self {
            Self::Default => None,
            Self::L2048 => Some(2048),
            Self::L4096 => Some(4096),
        }
    }
}

/// One editable game row (text buffers the user types into).
struct GameRow {
    id: String,
    name: String,
    nexus_slug: String,
    esl: EslChoice,
    /// One master plugin per line.
    masters_text: String,
    /// One category per line.
    categories_text: String,
}

impl GameRow {
    fn from_entry(id: &str, g: &GameEntry) -> Self {
        Self {
            id: id.to_string(),
            name: g.name.clone(),
            nexus_slug: g.nexus_slug.clone(),
            esl: EslChoice::from_limit(g.esl_limit),
            masters_text: g.masters.join("\n"),
            categories_text: g.categories.join("\n"),
        }
    }

    /// Non-empty, trimmed lines, de-duplicated while keeping order.
    fn lines(text: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for raw in text.lines() {
            let t = raw.trim();
            if !t.is_empty() && !out.iter().any(|x| x == t) {
                out.push(t.to_string());
            }
        }
        out
    }

    fn to_entry(&self) -> GameEntry {
        GameEntry {
            name: self.name.trim().to_string(),
            nexus_slug: self.nexus_slug.trim().to_string(),
            categories: Self::lines(&self.categories_text),
            masters: Self::lines(&self.masters_text),
            esl_limit: self.esl.to_limit(),
        }
    }
}

#[derive(Default)]
pub struct GamesEditorState {
    /// Whether the working copy has been built from the app's current data.
    loaded: bool,
    rows: Vec<GameRow>,
    sel: Option<usize>,
    dirty: bool,
    /// Set when the user asked to close a window with unsaved changes.
    confirm_close: bool,
}

impl GamesEditorState {
    fn load_from(&mut self, data: &GamesData) {
        self.rows = data.games.iter().map(|(id, g)| GameRow::from_entry(id, g)).collect();
        self.sel = if self.rows.is_empty() { None } else { Some(0) };
        self.dirty = false;
        self.confirm_close = false;
        self.loaded = true;
    }

    /// Indices with a blocking problem (empty id, empty name, duplicate id),
    /// each with a translated message.
    fn problems(&self, t: &Texts) -> Vec<(usize, String)> {
        let mut out = Vec::new();
        for (i, r) in self.rows.iter().enumerate() {
            if r.id.trim().is_empty() {
                out.push((i, t.err_no_id.clone()));
            } else if self.rows.iter().take(i).any(|o| o.id.trim() == r.id.trim()) {
                out.push((i, t.err_dup_id.replace("{id}", r.id.trim())));
            }
            if r.name.trim().is_empty() {
                out.push((i, t.err_no_name.replace("{id}", r.id.trim())));
            }
        }
        out
    }

    /// Build a fresh dataset from the working rows (order preserved).
    fn build(&self, version: String) -> GamesData {
        let mut games = indexmap::IndexMap::new();
        for r in &self.rows {
            games.insert(r.id.trim().to_string(), r.to_entry());
        }
        GamesData { version, games }
    }
}

struct Texts {
    title: String,
    add: String,
    delete: String,
    up: String,
    down: String,
    save: String,
    close: String,
    id: String,
    id_hint: String,
    name: String,
    slug: String,
    slug_hint: String,
    esl: String,
    esl_default: String,
    masters: String,
    masters_hint: String,
    categories: String,
    categories_hint: String,
    empty: String,
    select_hint: String,
    err_no_id: String,
    err_dup_id: String,
    err_no_name: String,
    unsaved: String,
    discard: String,
    keep_editing: String,
    new_game_name: String,
    new_game_id: String,
}

impl Texts {
    fn new(i18n: &crate::i18n::I18n) -> Self {
        let t = |k: &str| i18n.t(k);
        Self {
            title: t("games-title"),
            add: t("games-add"),
            delete: t("games-delete"),
            up: t("games-move-up"),
            down: t("games-move-down"),
            save: t("games-save"),
            close: t("btn-close"),
            id: t("games-id"),
            id_hint: t("games-id-hint"),
            name: t("games-name"),
            slug: t("games-slug"),
            slug_hint: t("games-slug-hint"),
            esl: t("games-esl"),
            esl_default: t("games-esl-default"),
            masters: t("games-masters"),
            masters_hint: t("games-masters-hint"),
            categories: t("games-categories"),
            categories_hint: t("games-categories-hint"),
            empty: t("games-empty"),
            select_hint: t("games-select-hint"),
            err_no_id: t("games-err-no-id"),
            err_dup_id: t("games-err-dup-id"),
            err_no_name: t("games-err-no-name"),
            unsaved: t("games-unsaved"),
            discard: t("games-discard"),
            keep_editing: t("games-keep-editing"),
            new_game_name: t("games-new-name"),
            new_game_id: t("games-new-id"),
        }
    }
}

impl XimodApp {
    /// Open the games editor (Tools menu).
    pub fn open_games_editor(&mut self) {
        self.show_games_editor = true;
        self.games_editor.loaded = false;
    }

    /// Render the games editor window (independent viewport).
    pub fn render_games_editor(&mut self, ctx: &egui::Context) {
        if !self.show_games_editor {
            // Drop the working copy so a reopened window starts from fresh data.
            self.games_editor.loaded = false;
            self.free_window_closed("ximod_games");
            return;
        }
        if !self.games_editor.loaded {
            let data = self.games.clone();
            self.games_editor.load_from(&data);
        }

        let t = Texts::new(&self.i18n);
        let vb = self.free_viewport_builder(ctx, "ximod_games", t.title.clone(), [820.0, 560.0], false);

        let st = &mut self.games_editor;
        let cfg = &mut self.config;
        let mut do_save = false;
        let mut do_close = false;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("ximod_games"), vb, |ctx, _class| {
            let problems = st.problems(&t);
            let blocking = !problems.is_empty();
            let bad: std::collections::HashSet<usize> = problems.iter().map(|(i, _)| *i).collect();

            // ---- Bottom bar: validation summary + Save / Close (always visible) ----
            egui::TopBottomPanel::bottom("games_buttons").show(ctx, |ui| {
                ui.add_space(6.0);
                if st.confirm_close {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&t.unsaved).color(ui.visuals().warn_fg_color));
                        if ui.button(&t.discard).clicked() {
                            do_close = true;
                        }
                        if ui.button(&t.keep_editing).clicked() {
                            st.confirm_close = false;
                        }
                    });
                } else {
                    ui.horizontal(|ui| {
                        if blocking {
                            let msg = problems.first().map(|(_, m)| m.clone()).unwrap_or_default();
                            ui.label(RichText::new(msg).color(ui.visuals().error_fg_color));
                        } else if st.dirty {
                            ui.label(RichText::new(&t.unsaved).weak());
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(&t.close).clicked() {
                                if st.dirty {
                                    st.confirm_close = true;
                                } else {
                                    do_close = true;
                                }
                            }
                            if ui.add_enabled(!blocking, egui::Button::new(&t.save)).clicked() {
                                do_save = true;
                            }
                        });
                    });
                }
                ui.add_space(6.0);
            });

            // ---- Left: the list of games + add / delete / reorder ----
            egui::SidePanel::left("games_list")
                .resizable(true)
                .default_width(240.0)
                .show(ctx, |ui| {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        if ui.button(&t.add).clicked() {
                            let mut n = 1;
                            let mut id = t.new_game_id.clone();
                            while st.rows.iter().any(|r| r.id == id) {
                                n += 1;
                                id = format!("{}{n}", t.new_game_id);
                            }
                            st.rows.push(GameRow {
                                id,
                                name: t.new_game_name.clone(),
                                nexus_slug: String::new(),
                                esl: EslChoice::Default,
                                masters_text: String::new(),
                                categories_text: String::new(),
                            });
                            st.sel = Some(st.rows.len() - 1);
                            st.dirty = true;
                        }
                        if ui.add_enabled(st.sel.is_some(), egui::Button::new(&t.delete)).clicked()
                            && let Some(i) = st.sel
                        {
                            st.rows.remove(i);
                            st.sel = if st.rows.is_empty() {
                                None
                            } else {
                                Some(i.min(st.rows.len() - 1))
                            };
                            st.dirty = true;
                        }
                    });
                    ui.horizontal(|ui| {
                        let i = st.sel.unwrap_or(0);
                        if ui
                            .add_enabled(st.sel.is_some_and(|i| i > 0), egui::Button::new(&t.up))
                            .clicked()
                        {
                            st.rows.swap(i, i - 1);
                            st.sel = Some(i - 1);
                            st.dirty = true;
                        }
                        if ui
                            .add_enabled(
                                st.sel.is_some_and(|i| i + 1 < st.rows.len()),
                                egui::Button::new(&t.down),
                            )
                            .clicked()
                        {
                            st.rows.swap(i, i + 1);
                            st.sel = Some(i + 1);
                            st.dirty = true;
                        }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                        for i in 0..st.rows.len() {
                            let label = {
                                let r = &st.rows[i];
                                let name = if r.name.trim().is_empty() {
                                    r.id.trim()
                                } else {
                                    r.name.trim()
                                };
                                if name.is_empty() {
                                    "—".to_string()
                                } else {
                                    name.to_string()
                                }
                            };
                            let text = if bad.contains(&i) {
                                RichText::new(format!("⚠ {label}")).color(ui.visuals().warn_fg_color)
                            } else {
                                RichText::new(label)
                            };
                            if ui.selectable_label(st.sel == Some(i), text).clicked() {
                                st.sel = Some(i);
                            }
                        }
                    });
                });

            // ---- Centre: the selected game's fields ----
            egui::CentralPanel::default().show(ctx, |ui| {
                if st.rows.is_empty() {
                    ui.add_space(8.0);
                    ui.label(RichText::new(&t.empty).weak());
                    return;
                }
                let Some(i) = st.sel else {
                    ui.add_space(8.0);
                    ui.label(RichText::new(&t.select_hint).weak());
                    return;
                };
                let mut changed = false;
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    let r = &mut st.rows[i];
                    egui::Grid::new("games_fields")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label(&t.id);
                            changed |= ui
                                .add(egui::TextEdit::singleline(&mut r.id).desired_width(260.0))
                                .on_hover_text(&t.id_hint)
                                .changed();
                            ui.end_row();

                            ui.label(&t.name);
                            changed |= ui
                                .add(egui::TextEdit::singleline(&mut r.name).desired_width(320.0))
                                .changed();
                            ui.end_row();

                            ui.label(&t.slug);
                            changed |= ui
                                .add(egui::TextEdit::singleline(&mut r.nexus_slug).desired_width(260.0))
                                .on_hover_text(&t.slug_hint)
                                .changed();
                            ui.end_row();

                            ui.label(&t.esl);
                            ui.horizontal(|ui| {
                                for (choice, lbl) in [
                                    (EslChoice::Default, t.esl_default.as_str()),
                                    (EslChoice::L2048, "2048"),
                                    (EslChoice::L4096, "4096"),
                                ] {
                                    changed |= ui.radio_value(&mut r.esl, choice, lbl).changed();
                                }
                            });
                            ui.end_row();
                        });

                    ui.add_space(8.0);
                    ui.label(RichText::new(&t.masters).strong());
                    ui.label(RichText::new(&t.masters_hint).weak().small());
                    changed |= ui
                        .add(
                            egui::TextEdit::multiline(&mut r.masters_text)
                                .desired_rows(3)
                                .desired_width(f32::INFINITY),
                        )
                        .changed();

                    ui.add_space(8.0);
                    ui.label(RichText::new(&t.categories).strong());
                    ui.label(RichText::new(&t.categories_hint).weak().small());
                    changed |= ui
                        .add(
                            egui::TextEdit::multiline(&mut r.categories_text)
                                .desired_rows(12)
                                .desired_width(f32::INFINITY),
                        )
                        .changed();
                });
                if changed {
                    st.dirty = true;
                }
            });

            crate::ui::widgets::free_window::record_win_geom(cfg, ctx, "ximod_games");
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                if st.dirty && !st.confirm_close {
                    st.confirm_close = true;
                } else {
                    do_close = true;
                }
            }
        });

        if do_save {
            let version = self.games.version.clone();
            let data = self.games_editor.build(version);
            match data.save() {
                Ok(path) => {
                    self.games = data;
                    self.games_editor.dirty = false;
                    self.games_editor.confirm_close = false;
                    self.notify_ok(
                        self.i18n
                            .t("games-saved")
                            .replace("{path}", &path.display().to_string()),
                    );
                }
                Err(e) => {
                    self.notify_err(self.i18n.t("games-save-error").replace("{error}", &e.to_string()));
                }
            }
        }
        if do_close {
            self.show_games_editor = false;
            self.games_editor.loaded = false;
            self.free_window_closed("ximod_games");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i18n() -> crate::i18n::I18n {
        let mut i = crate::i18n::I18n::new();
        i.set_locale("eng");
        i
    }

    fn row(id: &str, name: &str, cats: &str) -> GameRow {
        GameRow {
            id: id.into(),
            name: name.into(),
            nexus_slug: String::new(),
            esl: EslChoice::Default,
            masters_text: String::new(),
            categories_text: cats.into(),
        }
    }

    #[test]
    fn validation_flags_empty_and_duplicate() {
        let t = Texts::new(&i18n());
        let st = GamesEditorState {
            rows: vec![
                row("skyrim", "Skyrim", ""),
                row("", "No id", ""),
                row("skyrim", "Dup", ""),
            ],
            ..Default::default()
        };
        let flagged: Vec<usize> = st.problems(&t).iter().map(|(i, _)| *i).collect();
        assert!(flagged.contains(&1)); // empty id
        assert!(flagged.contains(&2)); // duplicate id
        // A clean set passes.
        let ok = GamesEditorState {
            rows: vec![row("a", "A", ""), row("b", "B", "")],
            ..Default::default()
        };
        assert!(ok.problems(&t).is_empty());
        // A missing name is flagged too.
        let noname = GamesEditorState {
            rows: vec![row("a", "  ", "")],
            ..Default::default()
        };
        assert_eq!(noname.problems(&t).len(), 1);
    }

    #[test]
    fn build_trims_dedups_and_keeps_order() {
        let st = GamesEditorState {
            rows: vec![row("  b ", "B", "  Armour \n Audio \n Armour \n\n"), row("a", "A", "")],
            ..Default::default()
        };
        let d = st.build("1".into());
        let keys: Vec<&str> = d.games.keys().map(|s| s.as_str()).collect();
        assert_eq!(keys, vec!["b", "a"]); // id trimmed, order preserved
        assert_eq!(d.games["b"].categories, vec!["Armour", "Audio"]); // trimmed + de-duplicated
        assert!(d.games["a"].categories.is_empty());
    }

    #[test]
    fn window_renders_headless_and_loads() {
        use eframe::egui;
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.show_games_editor = true;
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_games_editor(ctx));
        assert!(app.games_editor.loaded);
        // Closing the window drops the working copy.
        app.show_games_editor = false;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_games_editor(ctx));
        assert!(!app.games_editor.loaded);
    }
}
