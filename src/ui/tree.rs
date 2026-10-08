//! Project tree (left panel) and inspector (central panel).
//!
//! The tree is the single navigation surface of the editor: Mod info,
//! Installation steps → groups → options, Required files, Conditional
//! installs. Selecting a node drives the existing `current_*` indices, and
//! the inspector shows the editor of the selected node. Every node has a
//! context menu (add, duplicate, move, delete) and nodes can be dragged:
//! an option onto a group, a group onto a step, a step onto a step.

use super::main_window::{ConfirmAction, Tab, XimodApp};
use crate::models::{Plugin, PluginGroup, SelectionType, Step};
use crate::ui::components::{section_header, section_header_hint};
use crate::ui::theme::{Palette, icon};
use eframe::egui::{self, RichText};

/// What is being dragged in the tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DragItem {
    Step(usize),
    Group(usize, usize),
    Plugin(usize, usize, usize),
}

/// Where a drop landed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DropTarget {
    Step(usize),
    Group(usize, usize),
    Plugin(usize, usize, usize),
}

/// Pending structural action, applied after the tree is drawn (the tree
/// borrows the model while it renders).
#[derive(Debug, Clone)]
enum TreeAction {
    Select(Tab),
    SelectStep(usize),
    SelectGroup(usize, usize),
    SelectPlugin(usize, usize, usize),
    SelectCond(usize),
    AddStep,
    AddGroup(usize),
    AddPlugin(usize, usize),
    AddCond,
    DuplicateStep(usize),
    DuplicateGroup(usize, usize),
    DuplicatePlugin(usize, usize, usize),
    DeleteStep(usize),
    DeleteGroup(usize, usize),
    DeletePlugin(usize, usize, usize),
    DeleteCond(usize),
    MoveStep(usize, isize),
    MoveGroup(usize, usize, isize),
    MovePlugin(usize, usize, usize, isize),
    SaveTemplate(usize),
    Drop(DragItem, DropTarget),
}

/// A selectable tree label that is also a drag source for `payload`.
///
/// Not `Ui::dnd_drag_source`: that helper lays a drag-only widget over the
/// label, and egui's hit test then routes every click to that top widget —
/// which cannot click — so the label underneath never sees clicks,
/// double-clicks or right-clicks. Here the label itself senses click *and*
/// drag: a press-and-release selects it (and opens its context menu), a
/// press-and-move starts the drag, whose payload the drop targets read with
/// `dnd_release_payload` as before.
fn drag_source_label(
    ui: &mut egui::Ui,
    payload: DragItem,
    selected: bool,
    text: impl Into<egui::WidgetText>,
) -> egui::Response {
    let text: egui::WidgetText = text.into();
    let resp = ui
        .selectable_label(selected, text.clone())
        .interact(egui::Sense::click_and_drag());
    if resp.drag_started() {
        egui::DragAndDrop::set_payload(ui.ctx(), payload);
    }
    if resp.dragged() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        // A ghost of the label follows the pointer, like `dnd_drag_source`.
        if let Some(pos) = ui.ctx().pointer_interact_pos() {
            let layer = egui::LayerId::new(egui::Order::Tooltip, resp.id.with("ghost"));
            let painter = ui.ctx().layer_painter(layer);
            let galley = text.into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                egui::TextStyle::Body,
            );
            let rect = egui::Rect::from_min_size(pos + egui::vec2(8.0, -galley.size().y / 2.0), galley.size());
            painter.rect_filled(rect.expand(4.0), 4.0, ui.visuals().extreme_bg_color.gamma_multiply(0.9));
            painter.galley(rect.min, galley, ui.visuals().text_color());
        }
    } else if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
    }
    resp
}

/// A row of the project tree the keyboard cursor can rest on, in drawing
/// order (see `render_project_tree`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavKey {
    Info,
    StepsHeader,
    Step(usize),
    Group(usize, usize),
    Plugin(usize, usize, usize),
    Required,
    CondHeader,
    CondSet(usize),
}

/// One visible row: its key and, for foldable rows, the collapsing-state id
/// and whether it is open. (The cursor ring and the scrolling are drawn
/// where the row is laid out.)
#[derive(Debug, Clone, Copy)]
pub(crate) struct NavRow {
    pub key: NavKey,
    pub fold: Option<(egui::Id, bool)>,
}

/// The parent row of a foldable or nested node (← on a leaf goes there).
fn nav_parent(key: NavKey) -> Option<NavKey> {
    match key {
        NavKey::Step(_) => Some(NavKey::StepsHeader),
        NavKey::Group(si, _) => Some(NavKey::Step(si)),
        NavKey::Plugin(si, gi, _) => Some(NavKey::Group(si, gi)),
        NavKey::CondSet(_) => Some(NavKey::CondHeader),
        _ => None,
    }
}

/// The tree action that selects a row (Enter).
fn nav_select(key: NavKey) -> TreeAction {
    match key {
        NavKey::Info => TreeAction::Select(Tab::Info),
        NavKey::StepsHeader => TreeAction::Select(Tab::Steps),
        NavKey::Step(si) => TreeAction::SelectStep(si),
        NavKey::Group(si, gi) => TreeAction::SelectGroup(si, gi),
        NavKey::Plugin(si, gi, pi) => TreeAction::SelectPlugin(si, gi, pi),
        NavKey::Required => TreeAction::Select(Tab::RequiredInstalls),
        NavKey::CondHeader => TreeAction::Select(Tab::ConditionalInstalls),
        NavKey::CondSet(ci) => TreeAction::SelectCond(ci),
    }
}

/// A fold command from the tree's context menus (or a double-click).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Fold {
    /// Open (`true`) or close every step and group.
    All(bool),
    /// Open or close one step header only.
    Step(usize, bool),
    /// Open or close a step and every group under it.
    StepTree(usize, bool),
    /// Open or close one group header.
    Group(egui::Id, bool),
}

/// Apply a fold command to the collapsing headers drawn this frame.
pub(crate) fn apply_fold(ctx: &egui::Context, fold: Fold, step_ids: &[egui::Id], group_ids: &[(usize, egui::Id)]) {
    let set = |id: egui::Id, open: bool| {
        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(ctx, id, true);
        state.set_open(open);
        state.store(ctx);
    };
    match fold {
        Fold::All(open) => {
            for &id in step_ids {
                set(id, open);
            }
            for &(_, id) in group_ids {
                set(id, open);
            }
        }
        Fold::Step(si, open) => {
            if let Some(&id) = step_ids.get(si) {
                set(id, open);
            }
        }
        Fold::StepTree(si, open) => {
            if let Some(&id) = step_ids.get(si) {
                set(id, open);
            }
            for &(s, id) in group_ids.iter().filter(|(s, _)| *s == si) {
                let _ = s;
                set(id, open);
            }
        }
        Fold::Group(id, open) => set(id, open),
    }
}

impl XimodApp {
    // ------------------------------------------------------------ actions

    /// Append a new step and select it.
    pub(crate) fn add_step(&mut self) {
        let name = self
            .i18n
            .t_num("default-step-name", (self.ximod.steps.len() + 1) as i64);
        self.ximod.steps.push(Step::new(name));
        let last = self.ximod.steps.len() - 1;
        self.current_tab = Tab::Steps;
        self.select_step(Some(last));
        self.mark_modified();
    }

    fn add_group(&mut self, si: usize) {
        let Some(step) = self.ximod.steps.get_mut(si) else {
            return;
        };
        let name = self
            .i18n
            .t_num("default-group-name", (step.plugin_groups.len() + 1) as i64);
        step.plugin_groups
            .push(PluginGroup::new(name, SelectionType::SelectAny));
        let gi = step.plugin_groups.len() - 1;
        self.current_tab = Tab::Steps;
        self.select_step(Some(si));
        self.select_group(Some(gi));
        self.mark_modified();
    }

    fn add_plugin(&mut self, si: usize, gi: usize) {
        let Some(group) = self.ximod.steps.get_mut(si).and_then(|s| s.plugin_groups.get_mut(gi)) else {
            return;
        };
        let name = self.i18n.t_num("default-plugin-name", (group.plugins.len() + 1) as i64);
        group.plugins.push(Plugin::new(name));
        let pi = group.plugins.len() - 1;
        self.current_tab = Tab::Steps;
        self.select_step(Some(si));
        self.select_group(Some(gi));
        self.select_plugin(Some(pi));
        self.mark_modified();
    }

    fn add_cond_set(&mut self) {
        self.ximod
            .conditional_files
            .push(crate::models::ConditionalFileSet::new());
        let last = self.ximod.conditional_files.len() - 1;
        self.current_tab = Tab::ConditionalInstalls;
        self.select_cond_pattern(Some(last));
        self.mark_modified();
    }

    fn apply_tree_action(&mut self, action: TreeAction) {
        use crate::ui::components::{move_down, move_up};
        match action {
            TreeAction::Select(tab) => self.current_tab = tab,
            TreeAction::SelectStep(si) => {
                self.current_tab = Tab::Steps;
                if self.selection.step != Some(si) || self.selection.group.is_some() {
                    self.select_step(Some(si));
                }
            }
            TreeAction::SelectGroup(si, gi) => {
                self.current_tab = Tab::Steps;
                if self.selection.step != Some(si) {
                    self.select_step(Some(si));
                }
                if self.selection.group != Some(gi) || self.selection.plugin.is_some() {
                    self.select_group(Some(gi));
                }
            }
            TreeAction::SelectPlugin(si, gi, pi) => {
                self.current_tab = Tab::Steps;
                if self.selection.step != Some(si) {
                    self.select_step(Some(si));
                }
                if self.selection.group != Some(gi) {
                    self.select_group(Some(gi));
                }
                if self.selection.plugin != Some(pi) {
                    self.select_plugin(Some(pi));
                }
            }
            TreeAction::SelectCond(ci) => {
                self.current_tab = Tab::ConditionalInstalls;
                self.select_cond_pattern(Some(ci));
            }
            TreeAction::AddStep => self.add_step(),
            TreeAction::AddGroup(si) => self.add_group(si),
            TreeAction::AddPlugin(si, gi) => self.add_plugin(si, gi),
            TreeAction::AddCond => self.add_cond_set(),
            TreeAction::DuplicateStep(si) => {
                if let Some(step) = self.ximod.steps.get(si).cloned() {
                    self.ximod.steps.insert(si + 1, step);
                    self.current_tab = Tab::Steps;
                    self.select_step(Some(si + 1));
                    self.mark_modified();
                }
            }
            TreeAction::DuplicateGroup(si, gi) => {
                if let Some(step) = self.ximod.steps.get_mut(si)
                    && let Some(group) = step.plugin_groups.get(gi).cloned()
                {
                    step.plugin_groups.insert(gi + 1, group);
                    self.current_tab = Tab::Steps;
                    self.select_step(Some(si));
                    self.select_group(Some(gi + 1));
                    self.mark_modified();
                }
            }
            TreeAction::DuplicatePlugin(si, gi, pi) => {
                if let Some(group) = self.ximod.steps.get_mut(si).and_then(|s| s.plugin_groups.get_mut(gi))
                    && let Some(plugin) = group.plugins.get(pi).cloned()
                {
                    group.plugins.insert(pi + 1, plugin);
                    self.current_tab = Tab::Steps;
                    self.select_step(Some(si));
                    self.select_group(Some(gi));
                    self.select_plugin(Some(pi + 1));
                    self.mark_modified();
                }
            }
            TreeAction::DeleteStep(si) => {
                // A whole step is the one deletion that still asks first.
                self.confirm_action = Some(ConfirmAction::DeleteStep(si));
                self.show_confirm = true;
            }
            TreeAction::DeleteGroup(si, gi) => {
                if let Some(step) = self.ximod.steps.get_mut(si)
                    && gi < step.plugin_groups.len()
                {
                    step.plugin_groups.remove(gi);
                    self.select_step(Some(si));
                    self.mark_modified();
                    let msg = self.i18n.t("msg-deleted-undo");
                    self.notify_info(msg);
                }
            }
            TreeAction::DeletePlugin(si, gi, pi) => {
                if let Some(group) = self.ximod.steps.get_mut(si).and_then(|s| s.plugin_groups.get_mut(gi))
                    && pi < group.plugins.len()
                {
                    group.plugins.remove(pi);
                    self.select_step(Some(si));
                    self.select_group(Some(gi));
                    self.mark_modified();
                    let msg = self.i18n.t("msg-deleted-undo");
                    self.notify_info(msg);
                }
            }
            TreeAction::DeleteCond(ci) => {
                if ci < self.ximod.conditional_files.len() {
                    self.ximod.conditional_files.remove(ci);
                    let n = self.ximod.conditional_files.len();
                    self.select_cond_pattern(if n == 0 { None } else { Some(ci.min(n - 1)) });
                    self.mark_modified();
                    let msg = self.i18n.t("msg-deleted-undo");
                    self.notify_info(msg);
                }
            }
            TreeAction::MoveStep(si, dir) => {
                let moved = if dir < 0 {
                    move_up(&mut self.ximod.steps, si)
                } else {
                    move_down(&mut self.ximod.steps, si)
                };
                if moved {
                    let ni = (si as isize + dir) as usize;
                    self.current_tab = Tab::Steps;
                    self.select_step(Some(ni));
                    self.mark_modified();
                }
            }
            TreeAction::MoveGroup(si, gi, dir) => {
                if let Some(step) = self.ximod.steps.get_mut(si) {
                    let moved = if dir < 0 {
                        move_up(&mut step.plugin_groups, gi)
                    } else {
                        move_down(&mut step.plugin_groups, gi)
                    };
                    if moved {
                        let ngi = (gi as isize + dir) as usize;
                        self.current_tab = Tab::Steps;
                        self.select_step(Some(si));
                        self.select_group(Some(ngi));
                        self.mark_modified();
                    }
                }
            }
            TreeAction::MovePlugin(si, gi, pi, dir) => {
                if let Some(group) = self.ximod.steps.get_mut(si).and_then(|s| s.plugin_groups.get_mut(gi)) {
                    let moved = if dir < 0 {
                        move_up(&mut group.plugins, pi)
                    } else {
                        move_down(&mut group.plugins, pi)
                    };
                    if moved {
                        let npi = (pi as isize + dir) as usize;
                        self.current_tab = Tab::Steps;
                        self.select_step(Some(si));
                        self.select_group(Some(gi));
                        self.select_plugin(Some(npi));
                        self.mark_modified();
                    }
                }
            }
            TreeAction::SaveTemplate(si) => {
                self.select_step(Some(si));
                self.open_templates();
            }
            TreeAction::Drop(item, target) => self.apply_drop(item, target),
        }
    }

    /// Move a dragged node to where it was dropped.
    fn apply_drop(&mut self, item: DragItem, target: DropTarget) {
        match (item, target) {
            // Option → before another option, or to the end of a group.
            (DragItem::Plugin(si, gi, pi), DropTarget::Plugin(ti, tg, tp)) => {
                if (si, gi, pi) == (ti, tg, tp) {
                    return;
                }
                let Some(plugin) = self.take_plugin(si, gi, pi) else {
                    return;
                };
                // Removing shifted the target index when both are in the same group.
                let mut at = tp;
                if (si, gi) == (ti, tg) && pi < tp {
                    at -= 1;
                }
                if let Some(group) = self.ximod.steps.get_mut(ti).and_then(|s| s.plugin_groups.get_mut(tg)) {
                    let at = at.min(group.plugins.len());
                    group.plugins.insert(at, plugin);
                    self.current_tab = Tab::Steps;
                    self.select_step(Some(ti));
                    self.select_group(Some(tg));
                    self.select_plugin(Some(at));
                    self.mark_modified();
                }
            }
            (DragItem::Plugin(si, gi, pi), DropTarget::Group(ti, tg)) => {
                if (si, gi) == (ti, tg) {
                    return;
                }
                let Some(plugin) = self.take_plugin(si, gi, pi) else {
                    return;
                };
                if let Some(group) = self.ximod.steps.get_mut(ti).and_then(|s| s.plugin_groups.get_mut(tg)) {
                    group.plugins.push(plugin);
                    let at = group.plugins.len() - 1;
                    self.current_tab = Tab::Steps;
                    self.select_step(Some(ti));
                    self.select_group(Some(tg));
                    self.select_plugin(Some(at));
                    self.mark_modified();
                }
            }
            // Group → before another group, or to the end of a step.
            (DragItem::Group(si, gi), DropTarget::Group(ti, tg)) => {
                if (si, gi) == (ti, tg) {
                    return;
                }
                let Some(group) = self.take_group(si, gi) else { return };
                let mut at = tg;
                if si == ti && gi < tg {
                    at -= 1;
                }
                if let Some(step) = self.ximod.steps.get_mut(ti) {
                    let at = at.min(step.plugin_groups.len());
                    step.plugin_groups.insert(at, group);
                    self.current_tab = Tab::Steps;
                    self.select_step(Some(ti));
                    self.select_group(Some(at));
                    self.mark_modified();
                }
            }
            (DragItem::Group(si, gi), DropTarget::Step(ti)) => {
                if si == ti {
                    return;
                }
                let Some(group) = self.take_group(si, gi) else { return };
                if let Some(step) = self.ximod.steps.get_mut(ti) {
                    step.plugin_groups.push(group);
                    let at = step.plugin_groups.len() - 1;
                    self.current_tab = Tab::Steps;
                    self.select_step(Some(ti));
                    self.select_group(Some(at));
                    self.mark_modified();
                }
            }
            // Step → before another step.
            (DragItem::Step(si), DropTarget::Step(ti)) => {
                if si == ti || si >= self.ximod.steps.len() {
                    return;
                }
                let step = self.ximod.steps.remove(si);
                let at = if si < ti { ti - 1 } else { ti };
                let at = at.min(self.ximod.steps.len());
                self.ximod.steps.insert(at, step);
                self.current_tab = Tab::Steps;
                self.select_step(Some(at));
                self.mark_modified();
            }
            _ => {}
        }
    }

    fn take_plugin(&mut self, si: usize, gi: usize, pi: usize) -> Option<Plugin> {
        let group = self.ximod.steps.get_mut(si)?.plugin_groups.get_mut(gi)?;
        (pi < group.plugins.len()).then(|| group.plugins.remove(pi))
    }

    fn take_group(&mut self, si: usize, gi: usize) -> Option<PluginGroup> {
        let step = self.ximod.steps.get_mut(si)?;
        (gi < step.plugin_groups.len()).then(|| step.plugin_groups.remove(gi))
    }

    // --------------------------------------------------------------- tree

    /// The project tree, drawn inside the left side panel.
    pub(crate) fn render_project_tree(&mut self, ui: &mut egui::Ui) {
        let t = |k: &str| self.i18n.t(k);
        let l_info = t("tree-mod-info");
        let l_steps = t("tree-steps");
        let l_required = t("tree-required");
        let l_conditional = t("tree-conditional");
        let l_empty_steps = t("tree-empty-steps");
        let l_untitled = t("tab-untitled");
        let m_add_step = t("btn-add-step");
        let m_add_group = t("btn-add-group");
        let m_add_option = t("btn-add-plugin");
        let m_add_cond = t("btn-add-pattern");
        let m_duplicate = t("tree-duplicate");
        let m_delete = t("tree-delete");
        let m_up = t("reorder-before");
        let m_down = t("reorder-after");
        let m_template = t("tree-save-template");
        let drop_hint = t("tree-drop-hint");
        let m_expand = t("tree-expand");
        let m_collapse = t("tree-collapse");
        let m_expand_all = t("tree-expand-all");
        let m_expand_sel = t("tree-expand-selected");
        let m_expand_from = t("tree-expand-from");
        let m_collapse_all = t("tree-collapse-all");
        let m_collapse_sel = t("tree-collapse-selected");
        let m_collapse_from = t("tree-collapse-from");
        let h_expand_all = t("tree-expand-all-hint");
        let h_expand_sel = t("tree-expand-selected-hint");
        let h_expand_from = t("tree-expand-from-hint");
        let h_collapse_all = t("tree-collapse-all-hint");
        let h_collapse_sel = t("tree-collapse-selected-hint");
        let h_collapse_from = t("tree-collapse-from-hint");

        let palette = Palette::from_ui(ui);
        // Collapsing-header ids of the steps and groups drawn this frame, so a
        // fold command from a context menu can open/close any of them.
        let mut step_ids: Vec<egui::Id> = Vec::with_capacity(self.ximod.steps.len());
        let mut group_ids: Vec<(usize, egui::Id)> = Vec::new();
        let mut fold: Option<Fold> = None;
        // Rows in drawing order, for the keyboard cursor.
        let mut nav: Vec<NavRow> = Vec::new();
        let cursor_key = self.tree_cursor;
        let scroll_key = self.tree_scroll_to.take();
        // Draw the keyboard cursor ring on a row and scroll to it when asked.
        let mark = |ui: &mut egui::Ui, key: NavKey, rect: egui::Rect| {
            if cursor_key == Some(key) {
                ui.painter()
                    .rect_stroke(rect.expand(1.0), 3.0, egui::Stroke::new(1.5_f32, palette.accent));
            }
            if scroll_key == Some(key) {
                ui.scroll_to_rect(rect, None);
            }
        };
        let tab = self.current_tab;
        let sel = (self.selection.step, self.selection.group, self.selection.plugin);
        let cond_sel = self.selection.cond_pattern;
        let mut action: Option<TreeAction> = None;

        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            ui.add_space(4.0);
            // ---- Mod info
            let r = ui.selectable_label(tab == Tab::Info, format!("{} {}", icon::INFO, l_info));
            if r.clicked() {
                action = Some(TreeAction::Select(Tab::Info));
            }
            mark(ui, NavKey::Info, r.rect);
            nav.push(NavRow {
                key: NavKey::Info,
                fold: None,
            });
            ui.add_space(2.0);

            // ---- Steps
            ui.horizontal(|ui| {
                let header = format!("{} {} ({})", icon::STEP, l_steps, self.ximod.steps.len());
                let selected = tab == Tab::Steps && sel.0.is_none();
                let r = ui.selectable_label(selected, header);
                if r.clicked() {
                    action = Some(TreeAction::Select(Tab::Steps));
                }
                mark(ui, NavKey::StepsHeader, r.rect);
                nav.push(NavRow {
                    key: NavKey::StepsHeader,
                    fold: None,
                });
                if ui
                    .add(crate::ui::components::icon_button(icon::ADD))
                    .on_hover_text(&m_add_step)
                    .clicked()
                {
                    action = Some(TreeAction::AddStep);
                }
            });
            if self.ximod.steps.is_empty() {
                ui.indent("steps_empty", |ui| {
                    ui.label(RichText::new(&l_empty_steps).weak().small());
                });
            }
            for (si, step) in self.ximod.steps.iter().enumerate() {
                let id = ui.make_persistent_id(("tree_step", si));
                let step_selected = tab == Tab::Steps && sel.0 == Some(si) && sel.1.is_none();
                let name = if step.name.trim().is_empty() {
                    crate::ui::labels::step_fallback(&self.i18n, si)
                } else {
                    step.name.clone()
                };
                step_ids.push(id);
                let state = egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true);
                let step_open = state.is_open();
                let header = state
                    .show_header(ui, |ui| {
                        let label = format!("{} {}", icon::STEP, name);
                        let resp =
                            drag_source_label(ui, DragItem::Step(si), step_selected, RichText::new(label).strong());
                        mark(ui, NavKey::Step(si), resp.rect);
                        nav.push(NavRow {
                            key: NavKey::Step(si),
                            fold: Some((id, step_open)),
                        });
                        badge(ui, self.issues_at(crate::ui::problems::Target::Step(si)), palette);
                        if resp.clicked() {
                            action = Some(TreeAction::SelectStep(si));
                        }
                        // Double-click: show the step in the inspector and open it.
                        if resp.double_clicked() {
                            action = Some(TreeAction::SelectStep(si));
                            fold = Some(Fold::Step(si, true));
                        }
                        resp.context_menu(|ui| {
                            ui.menu_button(&m_expand, |ui| {
                                if ui.button(&m_expand_all).on_hover_text(&h_expand_all).clicked() {
                                    fold = Some(Fold::All(true));
                                    ui.close_menu();
                                }
                                if ui.button(&m_expand_sel).on_hover_text(&h_expand_sel).clicked() {
                                    fold = Some(Fold::Step(si, true));
                                    ui.close_menu();
                                }
                                if ui.button(&m_expand_from).on_hover_text(&h_expand_from).clicked() {
                                    fold = Some(Fold::StepTree(si, true));
                                    ui.close_menu();
                                }
                            });
                            ui.menu_button(&m_collapse, |ui| {
                                if ui.button(&m_collapse_all).on_hover_text(&h_collapse_all).clicked() {
                                    fold = Some(Fold::All(false));
                                    ui.close_menu();
                                }
                                if ui.button(&m_collapse_sel).on_hover_text(&h_collapse_sel).clicked() {
                                    fold = Some(Fold::Step(si, false));
                                    ui.close_menu();
                                }
                                if ui.button(&m_collapse_from).on_hover_text(&h_collapse_from).clicked() {
                                    fold = Some(Fold::StepTree(si, false));
                                    ui.close_menu();
                                }
                            });
                            ui.separator();
                            if ui.button(&m_add_group).clicked() {
                                action = Some(TreeAction::AddGroup(si));
                                ui.close_menu();
                            }
                            if ui.button(&m_duplicate).clicked() {
                                action = Some(TreeAction::DuplicateStep(si));
                                ui.close_menu();
                            }
                            if ui.button(&m_template).clicked() {
                                action = Some(TreeAction::SaveTemplate(si));
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.add_enabled(si > 0, egui::Button::new(&m_up)).clicked() {
                                action = Some(TreeAction::MoveStep(si, -1));
                                ui.close_menu();
                            }
                            if ui
                                .add_enabled(si + 1 < self.ximod.steps.len(), egui::Button::new(&m_down))
                                .clicked()
                            {
                                action = Some(TreeAction::MoveStep(si, 1));
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button(RichText::new(&m_delete).color(palette.danger)).clicked() {
                                action = Some(TreeAction::DeleteStep(si));
                                ui.close_menu();
                            }
                        });
                        drop_feedback(ui, &resp, &drop_hint, palette);
                        if let Some(item) = resp.dnd_release_payload::<DragItem>() {
                            action = Some(TreeAction::Drop(*item, DropTarget::Step(si)));
                        }
                    })
                    .body(|ui| {
                        for (gi, group) in step.plugin_groups.iter().enumerate() {
                            let gid = ui.make_persistent_id(("tree_group", si, gi));
                            group_ids.push((si, gid));
                            let group_selected =
                                tab == Tab::Steps && sel.0 == Some(si) && sel.1 == Some(gi) && sel.2.is_none();
                            let gname = if group.name.trim().is_empty() {
                                l_untitled.clone()
                            } else {
                                group.name.clone()
                            };
                            let gstate =
                                egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), gid, true);
                            let group_open = gstate.is_open();
                            gstate
                                .show_header(ui, |ui| {
                                    let short = crate::ui::labels::selection_type(&self.i18n, group.selection_type);
                                    let resp = drag_source_label(
                                        ui,
                                        DragItem::Group(si, gi),
                                        group_selected,
                                        format!("{} {}", icon::GROUP, gname),
                                    );
                                    mark(ui, NavKey::Group(si, gi), resp.rect);
                                    nav.push(NavRow {
                                        key: NavKey::Group(si, gi),
                                        fold: Some((gid, group_open)),
                                    });
                                    ui.label(RichText::new(short).small().weak());
                                    badge(ui, self.issues_at(crate::ui::problems::Target::Group(si, gi)), palette);
                                    if resp.clicked() {
                                        action = Some(TreeAction::SelectGroup(si, gi));
                                    }
                                    // Double-click: show the group in the inspector and open it.
                                    if resp.double_clicked() {
                                        action = Some(TreeAction::SelectGroup(si, gi));
                                        fold = Some(Fold::Group(gid, true));
                                    }
                                    resp.context_menu(|ui| {
                                        ui.menu_button(&m_expand, |ui| {
                                            if ui.button(&m_expand_all).on_hover_text(&h_expand_all).clicked() {
                                                fold = Some(Fold::All(true));
                                                ui.close_menu();
                                            }
                                            if ui.button(&m_expand_sel).on_hover_text(&h_expand_sel).clicked() {
                                                fold = Some(Fold::Group(gid, true));
                                                ui.close_menu();
                                            }
                                        });
                                        ui.menu_button(&m_collapse, |ui| {
                                            if ui.button(&m_collapse_all).on_hover_text(&h_collapse_all).clicked() {
                                                fold = Some(Fold::All(false));
                                                ui.close_menu();
                                            }
                                            if ui.button(&m_collapse_sel).on_hover_text(&h_collapse_sel).clicked() {
                                                fold = Some(Fold::Group(gid, false));
                                                ui.close_menu();
                                            }
                                        });
                                        ui.separator();
                                        if ui.button(&m_add_option).clicked() {
                                            action = Some(TreeAction::AddPlugin(si, gi));
                                            ui.close_menu();
                                        }
                                        if ui.button(&m_duplicate).clicked() {
                                            action = Some(TreeAction::DuplicateGroup(si, gi));
                                            ui.close_menu();
                                        }
                                        ui.separator();
                                        if ui.add_enabled(gi > 0, egui::Button::new(&m_up)).clicked() {
                                            action = Some(TreeAction::MoveGroup(si, gi, -1));
                                            ui.close_menu();
                                        }
                                        if ui
                                            .add_enabled(gi + 1 < step.plugin_groups.len(), egui::Button::new(&m_down))
                                            .clicked()
                                        {
                                            action = Some(TreeAction::MoveGroup(si, gi, 1));
                                            ui.close_menu();
                                        }
                                        ui.separator();
                                        if ui.button(RichText::new(&m_delete).color(palette.danger)).clicked() {
                                            action = Some(TreeAction::DeleteGroup(si, gi));
                                            ui.close_menu();
                                        }
                                    });
                                    drop_feedback(ui, &resp, &drop_hint, palette);
                                    if let Some(item) = resp.dnd_release_payload::<DragItem>() {
                                        action = Some(TreeAction::Drop(*item, DropTarget::Group(si, gi)));
                                    }
                                })
                                .body(|ui| {
                                    for (pi, plugin) in group.plugins.iter().enumerate() {
                                        let plugin_selected =
                                            tab == Tab::Steps && sel == (Some(si), Some(gi), Some(pi));
                                        let pname = if plugin.name.trim().is_empty() {
                                            l_untitled.clone()
                                        } else {
                                            plugin.name.clone()
                                        };
                                        let glyph =
                                            if plugin.image_path.as_deref().is_some_and(|p| !p.trim().is_empty()) {
                                                icon::IMAGE
                                            } else {
                                                icon::OPTION
                                            };
                                        let resp = drag_source_label(
                                            ui,
                                            DragItem::Plugin(si, gi, pi),
                                            plugin_selected,
                                            format!("{} {}", glyph, pname),
                                        );
                                        mark(ui, NavKey::Plugin(si, gi, pi), resp.rect);
                                        nav.push(NavRow {
                                            key: NavKey::Plugin(si, gi, pi),
                                            fold: None,
                                        });
                                        if let Some(info) = self.option_size_of((si, gi, pi)) {
                                            ui.label(
                                                RichText::new(crate::models::simulate::format_size(info.bytes))
                                                    .small()
                                                    .weak(),
                                            );
                                        }
                                        badge(
                                            ui,
                                            self.issues_at(crate::ui::problems::Target::Plugin(si, gi, pi)),
                                            palette,
                                        );
                                        if resp.clicked() || resp.double_clicked() {
                                            action = Some(TreeAction::SelectPlugin(si, gi, pi));
                                        }
                                        resp.context_menu(|ui| {
                                            if ui.button(&m_duplicate).clicked() {
                                                action = Some(TreeAction::DuplicatePlugin(si, gi, pi));
                                                ui.close_menu();
                                            }
                                            ui.separator();
                                            if ui.add_enabled(pi > 0, egui::Button::new(&m_up)).clicked() {
                                                action = Some(TreeAction::MovePlugin(si, gi, pi, -1));
                                                ui.close_menu();
                                            }
                                            if ui
                                                .add_enabled(pi + 1 < group.plugins.len(), egui::Button::new(&m_down))
                                                .clicked()
                                            {
                                                action = Some(TreeAction::MovePlugin(si, gi, pi, 1));
                                                ui.close_menu();
                                            }
                                            ui.separator();
                                            if ui.button(RichText::new(&m_delete).color(palette.danger)).clicked() {
                                                action = Some(TreeAction::DeletePlugin(si, gi, pi));
                                                ui.close_menu();
                                            }
                                        });
                                        drop_feedback(ui, &resp, &drop_hint, palette);
                                        if let Some(item) = resp.dnd_release_payload::<DragItem>() {
                                            action = Some(TreeAction::Drop(*item, DropTarget::Plugin(si, gi, pi)));
                                        }
                                    }
                                    if group.plugins.is_empty() {
                                        ui.label(
                                            RichText::new(format!("   {} {}", icon::ADD, m_add_option))
                                                .weak()
                                                .small(),
                                        )
                                        .on_hover_text(&m_add_option);
                                    }
                                });
                        }
                    });
                let _ = header;
            }

            ui.add_space(6.0);
            // ---- Required files
            let r = ui.selectable_label(
                tab == Tab::RequiredInstalls,
                format!(
                    "{} {} ({})",
                    icon::REQUIRED,
                    l_required,
                    self.ximod.required_files.len()
                ),
            );
            if r.clicked() {
                action = Some(TreeAction::Select(Tab::RequiredInstalls));
            }
            mark(ui, NavKey::Required, r.rect);
            nav.push(NavRow {
                key: NavKey::Required,
                fold: None,
            });

            ui.add_space(2.0);
            // ---- Conditional installs
            ui.horizontal(|ui| {
                let selected = tab == Tab::ConditionalInstalls && cond_sel.is_none();
                let header = format!(
                    "{} {} ({})",
                    icon::CONDITIONAL,
                    l_conditional,
                    self.ximod.conditional_files.len()
                );
                let r = ui.selectable_label(selected, header);
                if r.clicked() {
                    action = Some(TreeAction::Select(Tab::ConditionalInstalls));
                }
                mark(ui, NavKey::CondHeader, r.rect);
                nav.push(NavRow {
                    key: NavKey::CondHeader,
                    fold: None,
                });
                if ui
                    .add(crate::ui::components::icon_button(icon::ADD))
                    .on_hover_text(&m_add_cond)
                    .clicked()
                {
                    action = Some(TreeAction::AddCond);
                }
            });
            ui.indent("cond_sets", |ui| {
                for ci in 0..self.ximod.conditional_files.len() {
                    let selected = tab == Tab::ConditionalInstalls && cond_sel == Some(ci);
                    let label = self.i18n.t_num("cond-set-label", (ci + 1) as i64);
                    let resp = ui.selectable_label(selected, format!("{} {}", icon::FLAG, label));
                    if resp.clicked() {
                        action = Some(TreeAction::SelectCond(ci));
                    }
                    mark(ui, NavKey::CondSet(ci), resp.rect);
                    nav.push(NavRow {
                        key: NavKey::CondSet(ci),
                        fold: None,
                    });
                    resp.context_menu(|ui| {
                        if ui.button(RichText::new(&m_delete).color(palette.danger)).clicked() {
                            action = Some(TreeAction::DeleteCond(ci));
                            ui.close_menu();
                        }
                    });
                }
            });
            ui.add_space(8.0);
        });

        // ---- Keyboard: ↑ ↓ move the cursor ring, Enter selects the row
        // under it, ← folds (or goes to the parent), → unfolds (or goes to
        // the first child). Inactive while a text field has the keyboard.
        if !ui.ctx().wants_keyboard_input() && !nav.is_empty() {
            use egui::{Key, Modifiers};
            let (up, down, left, right, enter) = ui.ctx().input_mut(|i| {
                let n = Modifiers::NONE;
                (
                    i.consume_key(n, Key::ArrowUp),
                    i.consume_key(n, Key::ArrowDown),
                    i.consume_key(n, Key::ArrowLeft),
                    i.consume_key(n, Key::ArrowRight),
                    i.consume_key(n, Key::Enter),
                )
            });
            if up || down || left || right || enter {
                // The cursor starts on the selected node.
                let current = self
                    .tree_cursor
                    .filter(|k| nav.iter().any(|r| r.key == *k))
                    .unwrap_or_else(|| self.selection_nav_key());
                let pos = nav.iter().position(|r| r.key == current).unwrap_or(0);
                let mut next: Option<NavKey> = None;
                if down && pos + 1 < nav.len() {
                    next = Some(nav[pos + 1].key);
                }
                if up && pos > 0 {
                    next = Some(nav[pos - 1].key);
                }
                let row = nav[pos];
                if right {
                    match row.fold {
                        Some((id, false)) => fold = Some(Fold::Group(id, true)),
                        Some((_, true)) => {
                            if let Some(child) = nav.get(pos + 1).filter(|r| nav_parent(r.key) == Some(row.key)) {
                                next = Some(child.key);
                            }
                        }
                        None => {}
                    }
                }
                if left {
                    match row.fold {
                        Some((id, true)) => fold = Some(Fold::Group(id, false)),
                        _ => {
                            if let Some(parent) = nav_parent(row.key).filter(|p| nav.iter().any(|r| r.key == *p)) {
                                next = Some(parent);
                            }
                        }
                    }
                }
                if enter {
                    action = Some(nav_select(current));
                }
                let key = next.unwrap_or(current);
                self.tree_cursor = Some(key);
                if next.is_some() {
                    self.tree_scroll_to = Some(key);
                }
                ui.ctx().request_repaint();
            }
        }

        // A fold command is applied over a few frames: groups under a step
        // that was closed are only drawn (and get an id) once the step is
        // open again, so "expand all" needs a second pass to reach them.
        if let Some(f) = fold {
            self.tree_fold = Some((f, 3));
        }
        if let Some((f, left)) = self.tree_fold.take() {
            apply_fold(ui.ctx(), f, &step_ids, &group_ids);
            if left > 1 {
                self.tree_fold = Some((f, left - 1));
                ui.ctx().request_repaint();
            }
        }
        if let Some(a) = action {
            self.apply_tree_action(a);
        }
    }

    /// The tree row of the current selection (where the keyboard cursor
    /// starts).
    fn selection_nav_key(&self) -> NavKey {
        match self.current_tab {
            Tab::Info => NavKey::Info,
            Tab::RequiredInstalls => NavKey::Required,
            Tab::ConditionalInstalls => match self.selection.cond_pattern {
                Some(ci) => NavKey::CondSet(ci),
                None => NavKey::CondHeader,
            },
            Tab::Steps => match (self.selection.step, self.selection.group, self.selection.plugin) {
                (Some(si), Some(gi), Some(pi)) => NavKey::Plugin(si, gi, pi),
                (Some(si), Some(gi), None) => NavKey::Group(si, gi),
                (Some(si), None, _) => NavKey::Step(si),
                _ => NavKey::StepsHeader,
            },
        }
    }

    // ---------------------------------------------------------- inspector

    /// The editor of the selected node, drawn inside the central panel.
    pub(crate) fn render_inspector(&mut self, ui: &mut egui::Ui) {
        match self.current_tab {
            Tab::Info => self.render_info_tab(ui),
            Tab::RequiredInstalls => self.render_required_tab(ui),
            Tab::ConditionalInstalls => self.render_conditional_tab(ui),
            Tab::Steps => match (self.selection.step, self.selection.group, self.selection.plugin) {
                (Some(si), Some(gi), Some(pi)) if self.plugin_exists(si, gi, pi) => {
                    self.render_breadcrumb(ui, si, Some(gi), Some(pi));
                    self.render_plugin_details(ui);
                }
                (Some(si), Some(gi), _) if self.group_exists(si, gi) => {
                    self.render_breadcrumb(ui, si, Some(gi), None);
                    self.render_group_inspector(ui, si, gi);
                }
                (Some(si), _, _) if si < self.ximod.steps.len() => {
                    self.render_breadcrumb(ui, si, None, None);
                    self.render_step_inspector(ui, si);
                }
                _ => {
                    ui.add_space(24.0);
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(self.i18n.t("inspector-empty")).weak());
                        ui.add_space(8.0);
                        if ui
                            .button(format!("{} {}", icon::ADD, self.i18n.t("btn-add-step")))
                            .clicked()
                        {
                            self.add_step();
                        }
                    });
                }
            },
        }
    }

    fn group_exists(&self, si: usize, gi: usize) -> bool {
        self.ximod.steps.get(si).is_some_and(|s| gi < s.plugin_groups.len())
    }

    pub(crate) fn plugin_exists(&self, si: usize, gi: usize, pi: usize) -> bool {
        self.ximod
            .steps
            .get(si)
            .and_then(|s| s.plugin_groups.get(gi))
            .is_some_and(|g| pi < g.plugins.len())
    }

    /// "Step › Group › Option" line at the top of the inspector; each part is
    /// clickable to go up one level.
    fn render_breadcrumb(&mut self, ui: &mut egui::Ui, si: usize, gi: Option<usize>, pi: Option<usize>) {
        let step_name = self
            .ximod
            .steps
            .get(si)
            .map(|s| s.name.clone())
            .filter(|n| !n.trim().is_empty())
            .unwrap_or_else(|| crate::ui::labels::step_fallback(&self.i18n, si));
        let group_name = gi
            .and_then(|g| self.ximod.steps[si].plugin_groups.get(g))
            .map(|g| g.name.clone());
        let plugin_name = match (gi, pi) {
            (Some(g), Some(p)) => self.ximod.steps[si].plugin_groups[g]
                .plugins
                .get(p)
                .map(|p| p.name.clone()),
            _ => None,
        };
        let untitled = self.i18n.t("tab-untitled");
        let mut go: Option<TreeAction> = None;
        ui.horizontal(|ui| {
            if ui.link(format!("{} {}", icon::STEP, step_name)).clicked() {
                go = Some(TreeAction::SelectStep(si));
            }
            if let (Some(g), Some(name)) = (gi, group_name) {
                ui.label(RichText::new("›").weak());
                let name = if name.trim().is_empty() { untitled.clone() } else { name };
                if pi.is_some() {
                    if ui.link(format!("{} {}", icon::GROUP, name)).clicked() {
                        go = Some(TreeAction::SelectGroup(si, g));
                    }
                } else {
                    ui.label(RichText::new(format!("{} {}", icon::GROUP, name)).strong());
                }
            }
            if let Some(name) = plugin_name {
                ui.label(RichText::new("›").weak());
                let name = if name.trim().is_empty() { untitled.clone() } else { name };
                ui.label(RichText::new(format!("{} {}", icon::OPTION, name)).strong());
            }
        });
        ui.separator();
        if let Some(a) = go {
            self.apply_tree_action(a);
        }
    }

    /// Editor of a step: name, visibility conditions, page destination, and
    /// the list of its groups with the add/duplicate/delete actions.
    fn render_step_inspector(&mut self, ui: &mut egui::Ui, si: usize) {
        let label_step_name = self.i18n.t("label-step-name");
        let btn_delete_step = self.i18n.t("btn-delete-step");
        let btn_add_group = self.i18n.t("btn-add-group");
        let l_groups = self.i18n.t("section-groups");
        let hint_groups = self.i18n.t("hint-group-type");

        egui::ScrollArea::vertical().id_salt("step_inspector").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(&label_step_name);
                let mut step_name = self.ximod.steps[si].name.clone();
                if ui
                    .add(egui::TextEdit::singleline(&mut step_name).desired_width(320.0))
                    .changed()
                {
                    self.ximod.steps[si].name = step_name;
                    self.mark_modified();
                }
                if ui
                    .button(RichText::new(format!("{} {}", icon::DELETE, btn_delete_step)))
                    .clicked()
                {
                    self.confirm_action = Some(ConfirmAction::DeleteStep(si));
                    self.show_confirm = true;
                }
            });

            let vis_title = self.i18n.t("label-visibility");
            let vis_hint = self.i18n.t("hint-visibility");
            egui::CollapsingHeader::new(&vis_title)
                .id_salt("step_visibility_header")
                .default_open(!self.ximod.steps[si].visibility.is_empty())
                .show(ui, |ui| {
                    self.render_step_visibility(ui, si);
                })
                .header_response
                .on_hover_text(vis_hint);

            let dest_title = self.i18n.t("label-page-dest");
            egui::CollapsingHeader::new(&dest_title)
                .id_salt("step_dest_header")
                .default_open(false)
                .show(ui, |ui| {
                    self.render_page_destination(ui, si);
                });

            ui.add_space(8.0);
            section_header_hint(ui, &l_groups, &hint_groups);
            let mut action: Option<TreeAction> = None;
            for (gi, group) in self.ximod.steps[si].plugin_groups.iter().enumerate() {
                ui.horizontal(|ui| {
                    let name = if group.name.trim().is_empty() {
                        self.i18n.t("tab-untitled")
                    } else {
                        group.name.clone()
                    };
                    if ui.link(format!("{} {}", icon::GROUP, name)).clicked() {
                        action = Some(TreeAction::SelectGroup(si, gi));
                    }
                    ui.label(
                        RichText::new(crate::ui::labels::selection_type(&self.i18n, group.selection_type))
                            .weak()
                            .small(),
                    );
                    ui.label(
                        RichText::new(format!(
                            "· {}",
                            self.i18n.t_num("count-options", group.plugins.len() as i64)
                        ))
                        .weak()
                        .small(),
                    );
                });
            }
            ui.add_space(4.0);
            if ui.button(format!("{} {}", icon::ADD, btn_add_group)).clicked() {
                action = Some(TreeAction::AddGroup(si));
            }
            if let Some(a) = action {
                self.apply_tree_action(a);
            }
        });
    }

    /// Editor of a group: name, selection type, bulk destination, and the
    /// list of its options.
    fn render_group_inspector(&mut self, ui: &mut egui::Ui, si: usize, gi: usize) {
        let label_group_name = self.i18n.t("label-group-name");
        let label_type = self.i18n.t("label-group-type");
        let hint_group_type = self.i18n.t("hint-group-type");
        let label_group_dest = self.i18n.t("label-group-dest");
        let btn_apply_group_dest = self.i18n.t("btn-apply-group-dest");
        let group_dest_hint = self.i18n.t("group-dest-hint");
        let group_dest_nofiles = self.i18n.t("bulk-dest-nofiles");
        let btn_add_plugin = self.i18n.t("btn-add-plugin");
        let btn_remove_group = self.i18n.t("btn-remove-group");
        let l_options = self.i18n.t("section-options");

        egui::ScrollArea::vertical().id_salt("group_inspector").show(ui, |ui| {
            let mut group_name = self.ximod.steps[si].plugin_groups[gi].name.clone();
            let current_sel_type = self.ximod.steps[si].plugin_groups[gi].selection_type;
            ui.horizontal(|ui| {
                ui.label(&label_group_name);
                if ui
                    .add(egui::TextEdit::singleline(&mut group_name).desired_width(320.0))
                    .changed()
                {
                    self.ximod.steps[si].plugin_groups[gi].name = group_name;
                    self.mark_modified();
                }
                if ui
                    .button(RichText::new(format!("{} {}", icon::DELETE, btn_remove_group)))
                    .clicked()
                {
                    self.apply_tree_action(TreeAction::DeleteGroup(si, gi));
                }
            });
            if !self.group_exists(si, gi) {
                return;
            }
            ui.horizontal(|ui| {
                ui.label(&label_type).on_hover_text(&hint_group_type);
                egui::ComboBox::from_id_salt("group_type")
                    .selected_text(crate::ui::labels::selection_type(&self.i18n, current_sel_type))
                    .show_ui(ui, |ui| {
                        for st in SelectionType::variants() {
                            let label = crate::ui::labels::selection_type(&self.i18n, *st);
                            let hint = crate::ui::labels::selection_type_hint(&self.i18n, *st);
                            if ui
                                .selectable_label(current_sel_type == *st, label)
                                .on_hover_text(hint)
                                .clicked()
                            {
                                self.ximod.steps[si].plugin_groups[gi].selection_type = *st;
                                self.mark_modified();
                            }
                        }
                    })
                    .response
                    .on_hover_text(crate::ui::labels::selection_type_hint(&self.i18n, current_sel_type));
            });

            // Bulk destination for the whole group (all options in it).
            let group_file_count: usize = self.ximod.steps[si].plugin_groups[gi]
                .plugins
                .iter()
                .map(|p| p.files.len())
                .sum();
            ui.add_space(4.0);
            ui.label(
                RichText::new(&group_dest_hint)
                    .small()
                    .color(ui.visuals().weak_text_color()),
            );
            ui.horizontal(|ui| {
                ui.label(&label_group_dest);
                ui.text_edit_singleline(&mut self.group_dest_buf);
                if ui
                    .add_enabled(group_file_count > 0, egui::Button::new(&btn_apply_group_dest))
                    .clicked()
                {
                    let dest = self.group_dest_buf.trim().to_string();
                    let n = self.apply_dest_to_group(si, gi, &dest);
                    self.mark_modified();
                    self.notify_ok(self.i18n.t_num("status-dest-applied", n as i64));
                }
            });
            if group_file_count == 0 {
                ui.label(
                    RichText::new(&group_dest_nofiles)
                        .small()
                        .color(ui.visuals().weak_text_color()),
                );
            }

            ui.add_space(8.0);
            section_header(ui, &l_options);
            let mut action: Option<TreeAction> = None;
            for (pi, plugin) in self.ximod.steps[si].plugin_groups[gi].plugins.iter().enumerate() {
                ui.horizontal(|ui| {
                    let name = if plugin.name.trim().is_empty() {
                        self.i18n.t("tab-untitled")
                    } else {
                        plugin.name.clone()
                    };
                    if ui.link(format!("{} {}", icon::OPTION, name)).clicked() {
                        action = Some(TreeAction::SelectPlugin(si, gi, pi));
                    }
                    ui.label(
                        RichText::new(crate::ui::labels::plugin_type(&self.i18n, plugin.default_type))
                            .weak()
                            .small(),
                    );
                    ui.label(
                        RichText::new(format!(
                            "· {}",
                            self.i18n.t_num("count-files", plugin.files.len() as i64)
                        ))
                        .weak()
                        .small(),
                    );
                });
            }
            ui.add_space(4.0);
            if ui.button(format!("{} {}", icon::ADD, btn_add_plugin)).clicked() {
                action = Some(TreeAction::AddPlugin(si, gi));
            }
            if let Some(a) = action {
                self.apply_tree_action(a);
            }
        });
    }
}

/// Small error/warning counter next to a node, from the last validation.
fn badge(ui: &mut egui::Ui, (errors, warnings): (usize, usize), palette: Palette) {
    if errors > 0 {
        ui.label(
            RichText::new(format!("{} {errors}", icon::ERROR))
                .small()
                .color(palette.danger),
        );
    }
    if warnings > 0 {
        ui.label(
            RichText::new(format!("{} {warnings}", icon::WARNING))
                .small()
                .color(palette.warning),
        );
    }
}

/// Highlight a node while a compatible item is dragged over it.
fn drop_feedback(ui: &mut egui::Ui, resp: &egui::Response, hint: &str, palette: Palette) {
    if resp.dnd_hover_payload::<DragItem>().is_some() {
        let rect = resp.rect.expand(2.0);
        ui.painter()
            .rect_stroke(rect, 4.0, egui::Stroke::new(1.5_f32, palette.accent));
        resp.clone().on_hover_text(hint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app_with_sample() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        let mut step = Step::new("Textures");
        let mut group = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        group.plugins.push(Plugin::new("2K"));
        group.plugins.push(Plugin::new("4K"));
        step.plugin_groups.push(group);
        app.ximod.steps.push(step);
        app.ximod.steps.push(Step::new("Options"));
        app.history.reset(&app.ximod);
        app
    }

    /// The tree and every inspector must lay out without panicking, for each
    /// kind of selection.
    #[test]
    fn tree_and_inspectors_render_headless() {
        let mut app = app_with_sample();
        let ctx = egui::Context::default();
        let selections = [
            TreeAction::Select(Tab::Info),
            TreeAction::SelectStep(0),
            TreeAction::SelectGroup(0, 0),
            TreeAction::SelectPlugin(0, 0, 1),
            TreeAction::Select(Tab::RequiredInstalls),
            TreeAction::Select(Tab::ConditionalInstalls),
        ];
        for sel in selections {
            app.apply_tree_action(sel.clone());
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::SidePanel::left("t").show(ctx, |ui| app.render_project_tree(ui));
                egui::CentralPanel::default().show(ctx, |ui| app.render_inspector(ui));
            });
        }
        assert_eq!(app.current_tab, Tab::ConditionalInstalls);
    }

    /// Drives the tree with real pointer events.
    struct Mouse {
        ctx: egui::Context,
        time: f64,
    }

    impl Mouse {
        fn new() -> Self {
            let ctx = egui::Context::default();
            // Widget labels are only recorded with this debug option on.
            ctx.all_styles_mut(|s| s.debug.show_interactive_widgets = true);
            Self { ctx, time: 0.0 }
        }

        fn frame(&mut self, app: &mut XimodApp, events: Vec<egui::Event>) {
            self.time += 1.0 / 60.0;
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(900.0, 700.0))),
                time: Some(self.time),
                events,
                ..Default::default()
            };
            let _ = self.ctx.run(input, |ctx| {
                egui::SidePanel::left("t").show(ctx, |ui| app.render_project_tree(ui));
                egui::CentralPanel::default().show(ctx, |ui| app.render_inspector(ui));
            });
        }

        /// Centre of the first widget whose label contains `text` (debug
        /// builds record the label of every widget).
        fn find(&self, text: &str) -> Option<egui::Pos2> {
            self.find_left_of(text, f32::INFINITY)
        }

        /// Same, limited to widgets left of `max_x` (the tree panel).
        fn find_tree(&self, text: &str) -> Option<egui::Pos2> {
            self.find_left_of(text, 228.0)
        }

        fn find_left_of(&self, text: &str, max_x: f32) -> Option<egui::Pos2> {
            self.ctx.viewport(|v| {
                let w = &v.prev_pass.widgets;
                let mut found: Vec<(egui::Rect, egui::Order)> = Vec::new();
                for (layer, rects) in w.layers() {
                    for r in rects {
                        if r.rect.right() <= max_x
                            && w.info(r.id)
                                .and_then(|i| i.label.as_deref())
                                .is_some_and(|l| l.contains(text))
                        {
                            found.push((r.rect, layer.order));
                        }
                    }
                }
                // Prefer menus/popups (drawn above the panels), then the
                // left panel (the tree) over the inspector.
                found.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.left().total_cmp(&b.0.left())));
                found.first().map(|(r, _)| r.center())
            })
        }

        fn click(&mut self, app: &mut XimodApp, pos: egui::Pos2, button: egui::PointerButton) {
            self.frame(app, vec![egui::Event::PointerMoved(pos)]);
            self.frame(
                app,
                vec![egui::Event::PointerButton {
                    pos,
                    button,
                    pressed: true,
                    modifiers: Default::default(),
                }],
            );
            self.frame(
                app,
                vec![egui::Event::PointerButton {
                    pos,
                    button,
                    pressed: false,
                    modifiers: Default::default(),
                }],
            );
            self.frame(app, vec![]);
        }
    }

    #[test]
    fn mouse_harness_clicks_a_plain_button() {
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|s| s.debug.show_interactive_widgets = true);
        let mut clicks = 0;
        let mut time = 0.0;
        let mut frame = |events: Vec<egui::Event>, clicks: &mut i32| {
            time += 1.0 / 60.0;
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(900.0, 700.0))),
                time: Some(time),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if ui.button("Press me").clicked() {
                        *clicks += 1;
                    }
                });
            });
        };
        frame(vec![], &mut clicks);
        frame(vec![], &mut clicks);
        let pos = ctx.viewport(|v| {
            let w = &v.prev_pass.widgets;
            w.layers()
                .flat_map(|(_, rs)| rs.iter())
                .find(|r| w.info(r.id).and_then(|i| i.label.as_deref()) == Some("Press me"))
                .map(|r| r.rect.center())
                .expect("button drawn")
        });
        frame(vec![egui::Event::PointerMoved(pos)], &mut clicks);
        frame(
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            }],
            &mut clicks,
        );
        frame(
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Default::default(),
            }],
            &mut clicks,
        );
        frame(vec![], &mut clicks);
        assert_eq!(clicks, 1, "pos {pos:?}");
    }

    /// Right-click menus keep working after a fold command, and a
    /// double-click selects the node.
    #[test]
    fn mouse_fold_menus_and_double_click() {
        let mut app = app_with_sample();
        let mut m = Mouse::new();
        m.frame(&mut app, vec![]);
        m.frame(&mut app, vec![]);
        let step = m.find_tree("Textures").expect("step label drawn");
        assert!(m.find_tree("Resolution").is_some(), "group drawn under the open step");

        // A primary click selects the step.
        m.click(&mut app, step, egui::PointerButton::Primary);
        assert_eq!(
            app.selection.step,
            Some(0),
            "primary click selects the step at {step:?}"
        );
        // Right-click the step: the menu with the fold submenus appears.
        m.click(&mut app, step, egui::PointerButton::Secondary);
        let labels: Vec<String> = m.ctx.viewport(|v| {
            let w = &v.prev_pass.widgets;
            w.layers()
                .flat_map(|(_, rs)| rs.iter())
                .filter_map(|r| w.info(r.id).and_then(|i| i.label.clone()))
                .collect()
        });
        let collapse = m
            .find("Collapse")
            .unwrap_or_else(|| panic!("context menu open after the first right-click; labels = {labels:?}"));
        // Hover the submenu title to open it, then click "Collapse All".
        m.frame(&mut app, vec![egui::Event::PointerMoved(collapse)]);
        m.frame(&mut app, vec![]);
        m.frame(&mut app, vec![]);
        let all = m.find("Collapse All").expect("submenu open on hover");
        m.frame(&mut app, vec![egui::Event::PointerMoved(all)]);
        m.frame(
            &mut app,
            vec![egui::Event::PointerButton {
                pos: all,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            }],
        );
        m.frame(
            &mut app,
            vec![egui::Event::PointerButton {
                pos: all,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Default::default(),
            }],
        );
        // The fold animates over a few frames.
        for _ in 0..30 {
            m.frame(&mut app, vec![]);
        }
        assert!(
            m.find_tree("Resolution").is_none(),
            "the step is folded: its group is no longer drawn"
        );
        assert!(m.find("Collapse All").is_none(), "the menu closed");

        // A second right-click must open the menu again.
        let step = m.find_tree("Textures").expect("step label still drawn");
        m.click(&mut app, step, egui::PointerButton::Secondary);
        assert!(m.find("Expand").is_some(), "context menu opens again after a fold");
        // Expand All reopens the group.
        let expand = m.find("Expand").unwrap();
        m.frame(&mut app, vec![egui::Event::PointerMoved(expand)]);
        m.frame(&mut app, vec![]);
        m.frame(&mut app, vec![]);
        let all = m.find("Expand All").expect("expand submenu open");
        m.click(&mut app, all, egui::PointerButton::Primary);
        for _ in 0..30 {
            m.frame(&mut app, vec![]);
        }
        assert!(m.find_tree("Resolution").is_some(), "the group is drawn again");

        // Double-click on an option selects it and shows it.
        app.apply_tree_action(TreeAction::Select(Tab::Info));
        m.frame(&mut app, vec![]);
        let opt = m.find_tree("4K").expect("option drawn");
        m.click(&mut app, opt, egui::PointerButton::Primary);
        m.click(&mut app, opt, egui::PointerButton::Primary);
        assert_eq!(app.current_tab, Tab::Steps);
        assert_eq!(app.selection.plugin, Some(1));
    }

    /// Press-and-move on an option still drags it onto another row.
    #[test]
    fn mouse_drag_reorders_options() {
        let mut app = app_with_sample();
        let mut m = Mouse::new();
        m.frame(&mut app, vec![]);
        m.frame(&mut app, vec![]);
        let from = m.find_tree("4K").expect("4K drawn");
        let to = m.find_tree("2K").expect("2K drawn");
        m.frame(&mut app, vec![egui::Event::PointerMoved(from)]);
        m.frame(
            &mut app,
            vec![egui::Event::PointerButton {
                pos: from,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            }],
        );
        // Move in steps so egui sees a drag, not a click.
        for i in 1..=8 {
            let t = i as f32 / 8.0;
            let pos = from + (to - from) * t;
            m.frame(&mut app, vec![egui::Event::PointerMoved(pos)]);
        }
        m.frame(
            &mut app,
            vec![egui::Event::PointerButton {
                pos: to,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Default::default(),
            }],
        );
        m.frame(&mut app, vec![]);
        let names: Vec<&str> = app.ximod.steps[0].plugin_groups[0]
            .plugins
            .iter()
            .map(|p| p.name.as_str())
            .collect();
        assert_eq!(names, vec!["4K", "2K"], "4K dropped before 2K");
    }

    /// ↑ ↓ move the cursor ring, Enter selects, ← folds / goes up, → unfolds
    /// / goes down.
    #[test]
    fn keyboard_navigation_in_the_tree() {
        let mut app = app_with_sample();
        let mut m = Mouse::new();
        let key = |k: egui::Key| egui::Event::Key {
            key: k,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        };
        m.frame(&mut app, vec![]);
        m.frame(&mut app, vec![]);
        // Starts on the selected node (Info), ↓ goes to the steps header, then step 1.
        assert_eq!(app.current_tab, Tab::Info);
        m.frame(&mut app, vec![key(egui::Key::ArrowDown)]);
        assert_eq!(app.tree_cursor, Some(NavKey::StepsHeader));
        m.frame(&mut app, vec![key(egui::Key::ArrowDown)]);
        assert_eq!(app.tree_cursor, Some(NavKey::Step(0)));
        assert_eq!(app.current_tab, Tab::Info, "moving the cursor does not select");
        // Enter selects the step.
        m.frame(&mut app, vec![key(egui::Key::Enter)]);
        m.frame(&mut app, vec![]);
        assert_eq!(app.current_tab, Tab::Steps);
        assert_eq!(app.selection.step, Some(0));
        // → on an open step goes to its first group; ← on the open group folds
        // it (its options disappear), a second ← goes back to the step.
        m.frame(&mut app, vec![key(egui::Key::ArrowRight)]);
        assert_eq!(app.tree_cursor, Some(NavKey::Group(0, 0)));
        m.frame(&mut app, vec![key(egui::Key::ArrowLeft)]);
        for _ in 0..30 {
            m.frame(&mut app, vec![]);
        }
        assert_eq!(app.tree_cursor, Some(NavKey::Group(0, 0)));
        assert!(m.find_tree("4K").is_none(), "group folded");
        m.frame(&mut app, vec![key(egui::Key::ArrowLeft)]);
        assert_eq!(app.tree_cursor, Some(NavKey::Step(0)));
        // → on the step (open) goes to the group again; → on the closed group unfolds it.
        m.frame(&mut app, vec![key(egui::Key::ArrowRight)]);
        m.frame(&mut app, vec![key(egui::Key::ArrowRight)]);
        for _ in 0..30 {
            m.frame(&mut app, vec![]);
        }
        assert!(m.find_tree("4K").is_some(), "group unfolded");
        m.frame(&mut app, vec![key(egui::Key::ArrowLeft)]);
        for _ in 0..30 {
            m.frame(&mut app, vec![]);
        }
        m.frame(&mut app, vec![key(egui::Key::ArrowLeft)]);
        assert_eq!(app.tree_cursor, Some(NavKey::Step(0)));
        // ← on an open step folds it: the group is no longer drawn, ↓ skips it.
        m.frame(&mut app, vec![key(egui::Key::ArrowLeft)]);
        for _ in 0..30 {
            m.frame(&mut app, vec![]);
        }
        assert!(m.find_tree("Resolution").is_none(), "step folded");
        m.frame(&mut app, vec![key(egui::Key::ArrowDown)]);
        assert_eq!(app.tree_cursor, Some(NavKey::Step(1)));
        // → on a closed step unfolds it.
        m.frame(&mut app, vec![key(egui::Key::ArrowUp)]);
        m.frame(&mut app, vec![key(egui::Key::ArrowRight)]);
        for _ in 0..30 {
            m.frame(&mut app, vec![]);
        }
        assert!(m.find_tree("Resolution").is_some(), "step unfolded");
        // Down to the (folded) group, → unfolds it, down to the option, Enter selects it.
        m.frame(&mut app, vec![key(egui::Key::ArrowDown)]);
        assert_eq!(app.tree_cursor, Some(NavKey::Group(0, 0)));
        m.frame(&mut app, vec![key(egui::Key::ArrowRight)]);
        for _ in 0..30 {
            m.frame(&mut app, vec![]);
        }
        m.frame(&mut app, vec![key(egui::Key::ArrowDown)]);
        assert_eq!(app.tree_cursor, Some(NavKey::Plugin(0, 0, 0)));
        m.frame(&mut app, vec![key(egui::Key::Enter)]);
        m.frame(&mut app, vec![]);
        assert_eq!(app.selection.plugin, Some(0));
    }

    #[test]
    fn structural_actions_keep_selection_valid() {
        let mut app = app_with_sample();
        app.apply_tree_action(TreeAction::AddGroup(1));
        assert_eq!((app.selection.step, app.selection.group), (Some(1), Some(0)));
        app.apply_tree_action(TreeAction::AddPlugin(1, 0));
        assert_eq!(app.selection.plugin, Some(0));
        app.apply_tree_action(TreeAction::DuplicatePlugin(0, 0, 1));
        assert_eq!(app.ximod.steps[0].plugin_groups[0].plugins.len(), 3);
        assert_eq!(app.selection.plugin, Some(2));
        app.apply_tree_action(TreeAction::DeleteGroup(0, 0));
        assert!(app.ximod.steps[0].plugin_groups.is_empty());
        assert_eq!(app.selection.group, None);
        // Undo brings the group back and keeps indices in range.
        app.undo();
        assert_eq!(app.ximod.steps[0].plugin_groups.len(), 1);
        app.redo();
        assert!(app.ximod.steps[0].plugin_groups.is_empty());
    }

    #[test]
    fn drag_and_drop_moves_nodes() {
        let mut app = app_with_sample();
        // Option 4K dropped before 2K in the same group.
        app.apply_drop(DragItem::Plugin(0, 0, 1), DropTarget::Plugin(0, 0, 0));
        let names: Vec<&str> = app.ximod.steps[0].plugin_groups[0]
            .plugins
            .iter()
            .map(|p| p.name.as_str())
            .collect();
        assert_eq!(names, ["4K", "2K"]);
        // Group dropped onto the second step: moves to its end.
        app.apply_drop(DragItem::Group(0, 0), DropTarget::Step(1));
        assert!(app.ximod.steps[0].plugin_groups.is_empty());
        assert_eq!(app.ximod.steps[1].plugin_groups.len(), 1);
        assert_eq!((app.selection.step, app.selection.group), (Some(1), Some(0)));
        // Step 1 dropped before step 0.
        app.apply_drop(DragItem::Step(1), DropTarget::Step(0));
        assert_eq!(app.ximod.steps[0].name, "Options");
        assert_eq!(app.selection.step, Some(0));
        // Dropping onto itself is a no-op.
        let before = app.project_revision;
        app.apply_drop(DragItem::Step(0), DropTarget::Step(0));
        assert_eq!(app.project_revision, before);
    }
}
