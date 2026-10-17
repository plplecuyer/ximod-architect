//! Cut / copy / paste of steps, groups and options (Edit menu, tree context
//! menus, Ctrl+X / Ctrl+C / Ctrl+V in the tree).
//!
//! The clipboard belongs to the application, not to a document, so a node
//! copied in one FOMOD tab can be pasted in another. Copying also puts the
//! node on the system clipboard (see `models::clipboard::MARKER`), which lets
//! two XIMOD windows exchange nodes and makes Ctrl+V work: egui only reports
//! a paste when the system clipboard holds text.

use crate::models::clipboard::{self, ClipItem, NodeRef, PasteError};
use crate::ui::main_window::{Tab, XimodApp};
use eframe::egui;

impl XimodApp {
    /// The node selected in the tree, when the installation steps are shown.
    pub(crate) fn selected_node(&self) -> Option<NodeRef> {
        if self.current_tab != Tab::Steps {
            return None;
        }
        let sel = self.selection;
        match (sel.step, sel.group, sel.plugin) {
            (Some(si), Some(gi), Some(pi)) => Some(NodeRef::Plugin(si, gi, pi)),
            (Some(si), Some(gi), None) => Some(NodeRef::Group(si, gi)),
            (Some(si), None, _) => Some(NodeRef::Step(si)),
            _ => None,
        }
    }

    /// Select a node in the tree (and show the installation steps).
    pub(crate) fn select_node(&mut self, at: NodeRef) {
        self.current_tab = Tab::Steps;
        match at {
            NodeRef::Step(si) => self.select_step(Some(si)),
            NodeRef::Group(si, gi) => {
                self.select_step(Some(si));
                self.select_group(Some(gi));
            }
            NodeRef::Plugin(si, gi, pi) => {
                self.select_step(Some(si));
                self.select_group(Some(gi));
                self.select_plugin(Some(pi));
            }
        }
    }

    /// Copy `at` to the clipboard (and to the system clipboard through `ctx`).
    pub(crate) fn copy_node(&mut self, ctx: Option<&egui::Context>, at: NodeRef) -> bool {
        let Some(item) = ClipItem::copy(&self.ximod, at, self.root_directory.as_deref()) else {
            return false;
        };
        if let Some(ctx) = ctx {
            ctx.copy_text(item.to_text());
        }
        let msg = self.i18n.t_arg("msg-node-copied", "name", item.name());
        self.node_clipboard = Some(item);
        self.notify_info(msg);
        true
    }

    /// Cut `at`: copy it, then remove it from the project (one undo step).
    pub(crate) fn cut_node(&mut self, ctx: Option<&egui::Context>, at: NodeRef) {
        let Some(item) = ClipItem::copy(&self.ximod, at, self.root_directory.as_deref()) else {
            return;
        };
        if !clipboard::remove(&mut self.ximod, at) {
            return;
        }
        if let Some(ctx) = ctx {
            ctx.copy_text(item.to_text());
        }
        // Select what is left around the removed node.
        match at {
            NodeRef::Step(si) => {
                let n = self.ximod.steps.len();
                self.select_step(if n == 0 { None } else { Some(si.min(n - 1)) });
            }
            NodeRef::Group(si, _) => self.select_step(Some(si)),
            NodeRef::Plugin(si, gi, _) => {
                self.select_step(Some(si));
                self.select_group(Some(gi));
            }
        }
        self.mark_modified();
        let msg = crate::ui::theme::ui_symbols(&self.i18n.t_arg("msg-node-cut", "name", item.name()));
        self.node_clipboard = Some(item);
        self.notify_info(msg);
    }

    /// Paste `item` relative to `anchor`, select it, and report what needs
    /// attention (files absent from this mod, flags nothing sets).
    pub(crate) fn paste_item(&mut self, anchor: Option<NodeRef>, item: ClipItem) {
        match clipboard::paste(&mut self.ximod, anchor, &item) {
            Ok(at) => {
                self.select_node(at);
                self.mark_modified();
                self.notify_ok(self.i18n.t_arg("msg-node-pasted", "name", item.name()));
                let report = clipboard::report(&self.ximod, self.root_directory.as_deref(), &item);
                if report.missing_files > 0 {
                    let msg = self.i18n.t_num("msg-paste-missing-files", report.missing_files as i64);
                    self.notify_warn(msg);
                }
                if !report.unset_flags.is_empty() {
                    let msg = self
                        .i18n
                        .t_arg("msg-paste-unset-flags", "flags", &report.unset_flags.join(", "));
                    self.notify_warn(msg);
                }
            }
            Err(PasteError::NeedStep) => self.notify_warn(self.i18n.t("msg-paste-need-step")),
            Err(PasteError::NeedGroup) => self.notify_warn(self.i18n.t("msg-paste-need-group")),
        }
    }

    /// Paste the application clipboard relative to `anchor`.
    pub(crate) fn paste_node(&mut self, anchor: Option<NodeRef>) {
        match self.node_clipboard.clone() {
            Some(item) => self.paste_item(anchor, item),
            None => self.notify_info(self.i18n.t("msg-clipboard-empty")),
        }
    }

    /// Ctrl+X / Ctrl+C / Ctrl+V on the tree selection. Text fields keep these
    /// keys: nothing happens while one has the keyboard focus, or outside the
    /// installation steps.
    pub(crate) fn handle_clipboard_events(&mut self, ctx: &egui::Context) {
        if ctx.wants_keyboard_input() || self.current_tab != Tab::Steps {
            return;
        }
        let (copy, cut, pasted) = ctx.input(|i| {
            let mut copy = false;
            let mut cut = false;
            let mut pasted: Option<String> = None;
            for e in &i.events {
                match e {
                    egui::Event::Copy => copy = true,
                    egui::Event::Cut => cut = true,
                    egui::Event::Paste(text) => pasted = Some(text.clone()),
                    _ => {}
                }
            }
            (copy, cut, pasted)
        });
        let selected = self.selected_node();
        if cut {
            if let Some(at) = selected {
                self.cut_node(Some(ctx), at);
            }
        } else if copy {
            if let Some(at) = selected {
                self.copy_node(Some(ctx), at);
            }
        } else if let Some(text) = pasted {
            // A node from this or another XIMOD window wins; other text
            // falls back on the application clipboard.
            match ClipItem::from_text(&text) {
                Some(item) => {
                    self.node_clipboard = Some(item.clone());
                    self.paste_item(selected, item);
                }
                None if self.node_clipboard.is_some() => self.paste_node(selected),
                None => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Plugin, PluginGroup, SelectionType, Step};

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        for s in 0..2 {
            let mut step = Step::new(format!("S{s}"));
            let mut g = PluginGroup::new(format!("G{s}"), SelectionType::SelectAny);
            g.plugins.push(Plugin::new(format!("P{s}")));
            step.plugin_groups.push(g);
            app.ximod.steps.push(step);
        }
        app
    }

    #[test]
    fn copy_cut_paste_between_documents_and_undo() {
        let mut app = app();
        app.select_node(NodeRef::Plugin(0, 0, 0));
        assert_eq!(app.selected_node(), Some(NodeRef::Plugin(0, 0, 0)));
        assert!(app.copy_node(None, NodeRef::Plugin(0, 0, 0)));

        // Paste after the selected option of step 2.
        app.select_node(NodeRef::Plugin(1, 0, 0));
        app.paste_node(app.selected_node());
        assert_eq!(app.ximod.steps[1].plugin_groups[0].plugins[1].name, "P0");
        assert_eq!(app.selected_node(), Some(NodeRef::Plugin(1, 0, 1)));

        // The clipboard survives a switch to another document.
        app.new_project();
        assert!(app.node_clipboard.is_some());
        app.paste_node(None);
        assert!(app.ximod.steps.is_empty(), "an option needs a group");
        app.ximod.steps.push(Step::new("Only"));
        app.ximod.steps[0]
            .plugin_groups
            .push(PluginGroup::new("G", SelectionType::SelectAny));
        app.select_node(NodeRef::Group(0, 0));
        app.paste_node(app.selected_node());
        assert_eq!(app.ximod.steps[0].plugin_groups[0].plugins[0].name, "P0");
    }

    #[test]
    fn cut_removes_and_undo_restores() {
        let mut app = app();
        app.history.reset(&app.ximod);
        app.select_node(NodeRef::Step(0));
        app.cut_node(None, NodeRef::Step(0));
        assert_eq!(app.ximod.steps.len(), 1);
        assert_eq!(app.ximod.steps[0].name, "S1");
        app.history.end_burst(f64::MAX, &app.ximod);
        app.undo();
        assert_eq!(app.ximod.steps.len(), 2);
        // Paste the cut step at the end.
        app.paste_node(None);
        assert_eq!(app.ximod.steps.len(), 3);
        assert_eq!(app.ximod.steps[2].name, "S0");
    }

    #[test]
    fn keyboard_events_follow_the_tree_selection_only() {
        let mut app = app();
        let ctx = egui::Context::default();
        app.select_node(NodeRef::Group(0, 0));

        let run = |app: &mut XimodApp, events: Vec<egui::Event>| {
            let input = egui::RawInput {
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| app.handle_clipboard_events(ctx));
        };
        run(&mut app, vec![egui::Event::Copy]);
        let item = app.node_clipboard.clone().expect("copied");
        assert_eq!(item.name(), "G0");

        // Paste of a node text (another XIMOD window): into the selected step.
        app.select_node(NodeRef::Step(1));
        run(&mut app, vec![egui::Event::Paste(item.to_text())]);
        assert_eq!(app.ximod.steps[1].plugin_groups.len(), 2);
        assert_eq!(app.ximod.steps[1].plugin_groups[1].name, "G0");

        // Outside the steps, the keys do nothing.
        app.current_tab = Tab::Info;
        run(&mut app, vec![egui::Event::Cut]);
        assert_eq!(app.ximod.steps.len(), 2);
    }
}
