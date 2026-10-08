//! Shared behaviour of every foldable list / tree of the application: the
//! Expand / Collapse context menus on headings and the keyboard cursor
//! (↑ ↓ move, → unfold or enter, ← fold or climb, Enter acts on the row).
//!
//! The project tree has its own, older implementation (`tree.rs`); the
//! condition editor, the preview's file tree and any future tree go through
//! this module so they all behave the same. Usage, once per frame:
//!
//! ```ignore
//! let mut f = nav.frame(&i18n, nested);
//! let resp = f.header(ui, "key", depth, CollapsingHeader::new(..), |ui| { .. });
//! f.leaf(ui, "key/child", depth + 1, ui.label("..").rect);
//! if let Some(key) = nav.end_frame(ctx, f) { /* Enter on `key` */ }
//! ```
//!
//! Rows are recorded in drawing order with their depth; a heading's subtree
//! is the run of deeper rows after it. Folding goes through a queue applied
//! over several frames, because a subtree only yields its own headings once
//! its parent is open.

use eframe::egui::{self, collapsing_header::CollapsingState};

use crate::i18n::I18n;
use crate::ui::theme::Palette;

/// Persistent part: the cursor and the pending fold command.
#[derive(Debug, Default, Clone)]
pub struct FoldNav {
    /// Key of the row under the keyboard cursor.
    pub cursor: Option<String>,
    /// Row to scroll into view on the next frame.
    scroll_to: Option<String>,
    /// Fold command and the number of frames it is still applied on.
    fold: Option<(FoldCmd, u8)>,
}

/// A fold command from a heading's context menu or the keyboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoldCmd {
    /// Every heading.
    All(bool),
    /// One heading only.
    One(String, bool),
    /// A heading and every heading under it.
    Subtree(String, bool),
}

/// One visible row, in drawing order.
#[derive(Debug, Clone)]
struct Row {
    key: String,
    depth: usize,
    /// Collapsing-state id and openness, for headings.
    fold: Option<(egui::Id, bool)>,
}

/// Translated menu labels, fetched once (outside any closure that cannot
/// borrow the application's `I18n`).
#[derive(Debug, Clone)]
pub struct FoldLabels {
    expand: String,
    collapse: String,
    all: [String; 2],
    all_hint: [String; 2],
    sel: [String; 2],
    sel_hint: [String; 2],
    from: [String; 2],
    from_hint: [String; 2],
}

impl FoldLabels {
    pub fn new(i18n: &I18n) -> Self {
        let t = |k: &str| i18n.t(k);
        Self {
            expand: t("tree-expand"),
            collapse: t("tree-collapse"),
            all: [t("tree-collapse-all"), t("tree-expand-all")],
            all_hint: [t("tree-collapse-all-hint"), t("tree-expand-all-hint")],
            sel: [t("tree-collapse-selected"), t("tree-expand-selected")],
            sel_hint: [t("tree-collapse-selected-hint"), t("tree-expand-selected-hint")],
            from: [t("tree-collapse-from"), t("tree-expand-from")],
            from_hint: [t("tree-collapse-from-hint"), t("tree-expand-from-hint")],
        }
    }
}

/// Per-frame collector handed to the drawing code.
pub struct FoldFrame<'a> {
    rows: Vec<Row>,
    cursor: Option<String>,
    scroll_to: Option<String>,
    requested: Option<FoldCmd>,
    /// Whether headings can contain headings ("from Selected" entries).
    nested: bool,
    labels: &'a FoldLabels,
}

impl FoldNav {
    /// Start a frame. `nested` adds the "from Selected" menu entries, for
    /// trees whose headings contain headings.
    pub fn frame<'a>(&mut self, labels: &'a FoldLabels, nested: bool) -> FoldFrame<'a> {
        FoldFrame {
            rows: Vec::new(),
            cursor: self.cursor.clone(),
            scroll_to: self.scroll_to.take(),
            requested: None,
            nested,
            labels,
        }
    }

    /// Queue a fold command (menus call this through the frame; callers may
    /// also use it directly, e.g. from a toolbar button).
    pub fn request(&mut self, cmd: FoldCmd) {
        // Deep enough for any realistic nesting; each frame opens one level.
        self.fold = Some((cmd, 8));
    }

    /// Handle the keyboard and apply pending folds. Returns the key of the
    /// row on which Enter was pressed.
    pub fn end_frame(&mut self, ctx: &egui::Context, frame: FoldFrame<'_>) -> Option<String> {
        let FoldFrame { rows, requested, .. } = frame;
        let mut entered = None;
        let mut fold = requested;

        if !ctx.wants_keyboard_input() && !rows.is_empty() {
            use egui::{Key, Modifiers};
            let (up, down, left, right, enter, home, end) = ctx.input_mut(|i| {
                let n = Modifiers::NONE;
                (
                    i.consume_key(n, Key::ArrowUp),
                    i.consume_key(n, Key::ArrowDown),
                    i.consume_key(n, Key::ArrowLeft),
                    i.consume_key(n, Key::ArrowRight),
                    i.consume_key(n, Key::Enter),
                    i.consume_key(n, Key::Home),
                    i.consume_key(n, Key::End),
                )
            });
            if up || down || left || right || enter || home || end {
                let pos = self
                    .cursor
                    .as_ref()
                    .and_then(|c| rows.iter().position(|r| &r.key == c))
                    .unwrap_or(0);
                let mut next: Option<usize> = None;
                if down && pos + 1 < rows.len() {
                    next = Some(pos + 1);
                }
                if up && pos > 0 {
                    next = Some(pos - 1);
                }
                if home {
                    next = Some(0);
                }
                if end {
                    next = Some(rows.len() - 1);
                }
                let row = &rows[pos];
                if right {
                    match row.fold {
                        Some((_, false)) => fold = Some(FoldCmd::One(row.key.clone(), true)),
                        Some((_, true)) if rows.get(pos + 1).is_some_and(|r| r.depth > row.depth) => {
                            next = Some(pos + 1);
                        }
                        _ => {}
                    }
                }
                if left {
                    match row.fold {
                        Some((_, true)) => fold = Some(FoldCmd::One(row.key.clone(), false)),
                        _ => {
                            // Climb to the nearest shallower row above.
                            next = rows[..pos].iter().rposition(|r| r.depth < row.depth);
                        }
                    }
                }
                if enter {
                    entered = Some(row.key.clone());
                }
                let key = next.map(|p| rows[p].key.clone()).unwrap_or_else(|| row.key.clone());
                if next.is_some() {
                    self.scroll_to = Some(key.clone());
                }
                self.cursor = Some(key);
                ctx.request_repaint();
            }
        }

        if let Some(cmd) = fold {
            self.request(cmd);
        }
        if let Some((cmd, left)) = self.fold.take() {
            let set = |id: egui::Id, open: bool| {
                let mut st = CollapsingState::load_with_default_open(ctx, id, false);
                st.set_open(open);
                st.store(ctx);
            };
            let headings = rows.iter().filter(|r| r.fold.is_some());
            match &cmd {
                FoldCmd::All(open) => {
                    for r in headings {
                        set(r.fold.unwrap().0, *open);
                    }
                }
                FoldCmd::One(key, open) => {
                    if let Some(r) = rows.iter().find(|r| &r.key == key)
                        && let Some((id, _)) = r.fold
                    {
                        set(id, *open);
                    }
                }
                FoldCmd::Subtree(key, open) => {
                    if let Some(pos) = rows.iter().position(|r| &r.key == key) {
                        let depth = rows[pos].depth;
                        let sub = rows[pos + 1..].iter().take_while(|r| r.depth > depth);
                        for r in std::iter::once(&rows[pos]).chain(sub) {
                            if let Some((id, _)) = r.fold {
                                set(id, *open);
                            }
                        }
                    }
                }
            }
            if left > 1 {
                self.fold = Some((cmd, left - 1));
            }
            ctx.request_repaint();
        }
        entered
    }
}

impl FoldFrame<'_> {
    /// Draw the cursor ring on a row and scroll to it when asked.
    fn mark(&self, ui: &mut egui::Ui, key: &str, rect: egui::Rect) {
        if self.cursor.as_deref() == Some(key) {
            let accent = Palette::from_ui(ui).accent;
            ui.painter()
                .rect_stroke(rect.expand(1.0), 3.0, egui::Stroke::new(1.5_f32, accent));
        }
        if self.scroll_to.as_deref() == Some(key) {
            ui.scroll_to_rect(rect, None);
        }
    }

    /// A row without children: records it and draws the cursor on `rect`.
    pub fn leaf(&mut self, ui: &mut egui::Ui, key: impl Into<String>, depth: usize, rect: egui::Rect) {
        let key = key.into();
        self.mark(ui, &key, rect);
        self.rows.push(Row { key, depth, fold: None });
    }

    /// A heading: shows the collapsing header, records the row with the
    /// header's real id and openness, draws the cursor and attaches the
    /// Expand / Collapse context menu.
    pub fn header<R>(
        &mut self,
        ui: &mut egui::Ui,
        key: impl Into<String>,
        depth: usize,
        header: egui::CollapsingHeader,
        default_open: bool,
        body: impl FnOnce(&mut Self, &mut egui::Ui) -> R,
    ) -> egui::collapsing_header::CollapsingResponse<R> {
        let key = key.into();
        // The heading row precedes its subtree; its fold state is filled in
        // once the header is drawn (egui derives the id from a child `Ui`).
        let row = self.rows.len();
        self.rows.push(Row {
            key: key.clone(),
            depth,
            fold: None,
        });
        let resp = header.default_open(default_open).show(ui, |ui| body(self, ui));
        let id = resp.header_response.id;
        let open = CollapsingState::load_with_default_open(ui.ctx(), id, default_open).is_open();
        self.rows[row].fold = Some((id, open));
        self.mark(ui, &key, resp.header_response.rect);
        let l = self.labels;
        let nested = self.nested;
        let mut requested = None;
        resp.header_response.context_menu(|ui| {
            for (open, title) in [(true, &l.expand), (false, &l.collapse)] {
                let i = usize::from(open);
                ui.menu_button(title, |ui| {
                    if ui.button(&l.all[i]).on_hover_text(&l.all_hint[i]).clicked() {
                        requested = Some(FoldCmd::All(open));
                        ui.close_menu();
                    }
                    if ui.button(&l.sel[i]).on_hover_text(&l.sel_hint[i]).clicked() {
                        requested = Some(FoldCmd::One(key.clone(), open));
                        ui.close_menu();
                    }
                    if nested && ui.button(&l.from[i]).on_hover_text(&l.from_hint[i]).clicked() {
                        requested = Some(FoldCmd::Subtree(key.clone(), open));
                        ui.close_menu();
                    }
                });
            }
        });
        if requested.is_some() {
            self.requested = requested;
        }
        resp
    }

    /// Keys of the rows drawn so far, in order (tests).
    #[cfg(test)]
    pub fn keys(&self) -> Vec<String> {
        self.rows.iter().map(|r| r.key.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i18n() -> FoldLabels {
        let mut i = I18n::new();
        i.set_locale("eng");
        FoldLabels::new(&i)
    }

    /// Draws a two-level tree: a/ (a1, a2), b/ (b1).
    fn draw(ctx: &egui::Context, nav: &mut FoldNav, i18n: &FoldLabels) -> (Vec<String>, Option<String>) {
        let mut keys = Vec::new();
        let mut entered = None;
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut f = nav.frame(i18n, true);
                for (dir, files) in [("a", ["a1", "a2"]), ("b", ["b1", "b2"])] {
                    f.header(
                        ui,
                        dir,
                        0,
                        egui::CollapsingHeader::new(dir).id_salt(dir),
                        false,
                        |f, ui| {
                            for file in files {
                                let r = ui.label(file);
                                f.leaf(ui, format!("{dir}/{file}"), 1, r.rect);
                            }
                        },
                    );
                }
                keys = f.keys();
                entered = nav.end_frame(ctx, f);
            });
        });
        (keys, entered)
    }

    fn press(ctx: &egui::Context, nav: &mut FoldNav, i18n: &FoldLabels, key: egui::Key) -> Option<String> {
        let mut entered = None;
        let _ = ctx.run(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut f = nav.frame(i18n, true);
                    for (dir, files) in [("a", ["a1", "a2"]), ("b", ["b1", "b2"])] {
                        f.header(
                            ui,
                            dir,
                            0,
                            egui::CollapsingHeader::new(dir).id_salt(dir),
                            false,
                            |f, ui| {
                                for file in files {
                                    let r = ui.label(file);
                                    f.leaf(ui, format!("{dir}/{file}"), 1, r.rect);
                                }
                            },
                        );
                    }
                    entered = nav.end_frame(ctx, f);
                });
            },
        );
        // Let the fold animation finish.
        for _ in 0..12 {
            draw(ctx, nav, i18n);
        }
        entered
    }

    #[test]
    fn keyboard_moves_folds_and_enters() {
        let ctx = egui::Context::default();
        let i18n = i18n();
        let mut nav = FoldNav::default();
        let (keys, _) = draw(&ctx, &mut nav, &i18n);
        assert_eq!(keys, ["a", "b"]);
        use egui::Key;
        press(&ctx, &mut nav, &i18n, Key::ArrowDown);
        assert_eq!(nav.cursor.as_deref(), Some("b"));
        press(&ctx, &mut nav, &i18n, Key::ArrowUp);
        assert_eq!(nav.cursor.as_deref(), Some("a"));
        // → unfolds, → again enters, ← climbs, ← folds.
        press(&ctx, &mut nav, &i18n, Key::ArrowRight);
        assert_eq!(draw(&ctx, &mut nav, &i18n).0, ["a", "a/a1", "a/a2", "b"]);
        press(&ctx, &mut nav, &i18n, Key::ArrowRight);
        assert_eq!(nav.cursor.as_deref(), Some("a/a1"));
        assert_eq!(press(&ctx, &mut nav, &i18n, Key::Enter).as_deref(), Some("a/a1"));
        press(&ctx, &mut nav, &i18n, Key::ArrowLeft);
        assert_eq!(nav.cursor.as_deref(), Some("a"));
        press(&ctx, &mut nav, &i18n, Key::ArrowLeft);
        assert_eq!(draw(&ctx, &mut nav, &i18n).0, ["a", "b"]);
        // End / Home.
        press(&ctx, &mut nav, &i18n, Key::End);
        assert_eq!(nav.cursor.as_deref(), Some("b"));
        press(&ctx, &mut nav, &i18n, Key::Home);
        assert_eq!(nav.cursor.as_deref(), Some("a"));
    }

    #[test]
    fn fold_commands_reach_every_heading() {
        let ctx = egui::Context::default();
        let i18n = i18n();
        let mut nav = FoldNav::default();
        draw(&ctx, &mut nav, &i18n);
        nav.request(FoldCmd::All(true));
        for _ in 0..14 {
            draw(&ctx, &mut nav, &i18n);
        }
        assert_eq!(
            draw(&ctx, &mut nav, &i18n).0,
            ["a", "a/a1", "a/a2", "b", "b/b1", "b/b2"]
        );
        nav.request(FoldCmd::Subtree("a".into(), false));
        for _ in 0..14 {
            draw(&ctx, &mut nav, &i18n);
        }
        assert_eq!(draw(&ctx, &mut nav, &i18n).0, ["a", "b", "b/b1", "b/b2"]);
        nav.request(FoldCmd::One("b".into(), false));
        for _ in 0..14 {
            draw(&ctx, &mut nav, &i18n);
        }
        assert_eq!(draw(&ctx, &mut nav, &i18n).0, ["a", "b"]);
    }
}
