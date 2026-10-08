//! Real-time preview of the FOMOD installer.
//!
//! Opens a window that simulates, as faithfully as possible, what a mod manager
//! (Vortex, MO2, …) shows when the end user runs the generated installer:
//!
//!   * steps shown in order, skipping those whose conditional visibility is not
//!     satisfied by the current selections/flags;
//!   * each group rendered with the widget matching its selection type
//!     (radios for SelectExactlyOne/AtMostOne, checkboxes otherwise), honouring
//!     each plugin's effective type (Required forced on, NotUsable forced off,
//!     Recommended checked by default, …), including dynamic typeDescriptor
//!     patterns evaluated live;
//!   * Back / Next / Install navigation with per-step validity checks;
//!   * a final summary listing every file that would be installed.
//!
//! File-type dependencies (Active/Inactive/Missing of a game file) cannot be
//! known outside a real install, so an "assumptions" panel lets the author set
//! the supposed state of each referenced file; the simulation honours it.

use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use eframe::egui::{self, Color32, RichText};
use fluent::FluentArgs;

use crate::models::simulate::{
    InstallEntry, InstallTree, Scenario, SelKey, TreeNode, build_install_tree, default_selections, effective_type,
    evaluate, format_size, group_valid, install_entries, list_scenarios, referenced_files, sorted_entries,
};
use crate::models::verify::RefLoc;
use crate::models::{FileState, PluginType, SelectionType, Ximod};
use crate::ui::components::ImageDisplay;
use crate::ui::foldnav::{FoldFrame, FoldLabels};
use crate::ui::main_window::XimodApp;

/// Render a description as the preview should show it. When `replace` is set
/// (the "Process newlines in descriptions" option), literal `\n` / `\r\n`
/// sequences typed by the author are turned into real line breaks, so the
/// preview matches what a mod manager displays.
fn process_desc(text: &str, replace: bool) -> String {
    if replace {
        text.replace("\\r\\n", "\n").replace("\\n", "\n").replace("\\r", "\n")
    } else {
        text.to_string()
    }
}

/// Runtime state of one preview session.
/// Sentinel stored in `PreviewState::history` for the mod-information page.
const INFO_PAGE: usize = usize::MAX;

#[derive(Default)]
pub struct PreviewState {
    /// When set, the preview shows this model instead of the active project
    /// (used for the translated preview). `Rc` so a clone is cheap.
    pub model_override: Option<std::rc::Rc<crate::models::Ximod>>,
    /// Which plugins are currently selected.
    pub selections: HashMap<SelKey, bool>,
    /// Assumed state of each game file referenced by a `file` dependency.
    pub file_states: BTreeMap<String, FileState>,
    /// Index (into `ximod.steps`) of the step currently shown.
    pub cursor: usize,
    /// Stack of visited step indices, for the Back button.
    pub history: Vec<usize>,
    /// True when the final install summary is shown.
    pub finished: bool,
    /// True while the first page — the mod information (name, author,
    /// version, header image, description from info.xml) — is shown, as a mod
    /// manager does before the first install step.
    pub on_info: bool,
    /// Whether the file-assumptions panel is expanded.
    pub show_assumptions: bool,
    /// Plugin whose description/image is shown in the detail pane.
    pub focused: Option<SelKey>,
    /// Sorted install list of the summary page, with the fingerprint of the
    /// (unsorted) file sequence it was built from. Lets the summary skip the
    /// per-frame clone + sort while nothing changed; dropped when leaving the
    /// summary page and on Refresh (which rebuilds the whole state).
    install_cache: Option<(u64, Vec<InstallEntry>)>,
    /// Final file tree of the summary page, keyed by the install-list
    /// fingerprint and the root it was expanded under.
    tree_cache: Option<(u64, PathBuf, InstallTree)>,
    /// Newline-processed description of the plugin shown in the detail pane.
    desc_cache: Option<DescCache>,
    /// Whether the "Final file tree" section of the summary is expanded.
    pub show_tree: bool,
    /// Keyboard cursor and fold queue of the final file tree.
    pub tree_nav: crate::ui::foldnav::FoldNav,
    /// Node the user pressed Enter on in the file tree: its installing
    /// component is selected in the main window once the frame is over.
    goto: Option<RefLoc>,
    /// Saved scenarios of the root (refreshed on open, save and delete).
    pub scenario_names: Vec<String>,
    /// Scenario chosen in the combo.
    pub scenario_selected: Option<String>,
    /// Name typed for "Save scenario"; the field shows while `saving`.
    pub scenario_name_buf: String,
    pub scenario_saving: bool,
}

/// What the scenario row asked for this frame (applied once the window
/// closure released the preview state).
enum ScenarioAction {
    Load(String),
    Save(String),
    Delete(String),
}

/// Cached result of [`process_desc`] for one description text.
struct DescCache {
    /// The raw description the result was computed from; comparing it is a
    /// plain `memcmp`, far cheaper than the three `String::replace` passes.
    source: String,
    /// `process_desc(source, true)`.
    processed: String,
}

impl PreviewState {
    /// Description of the focused plugin with literal newlines interpreted,
    /// recomputed only when the text differs from the cached one (another
    /// plugin got the focus, or the author edited the description).
    fn processed_desc(&mut self, text: &str) -> &str {
        let cache = match self.desc_cache.take() {
            Some(c) if c.source == text => c,
            _ => DescCache {
                source: text.to_string(),
                processed: process_desc(text, true),
            },
        };
        self.desc_cache.insert(cache).processed.as_str()
    }

    /// Install list for the summary page; see [`PreviewState::install_cache`].
    ///
    /// `flags` / `visible` must be the result of [`evaluate`] for the current
    /// selections and file states.
    fn install_list(&mut self, ximod: &Ximod, flags: &HashMap<String, String>, visible: &[usize]) -> &[InstallEntry] {
        // Hashing the candidate files by reference allocates nothing, whereas
        // rebuilding the list clones every entry and sorts them.
        let mut h = std::collections::hash_map::DefaultHasher::new();
        for (_, f) in install_entries(ximod, &self.selections, &self.file_states, flags, visible) {
            std::mem::discriminant(&f.file_type).hash(&mut h);
            f.source.hash(&mut h);
            f.destination.hash(&mut h);
            f.priority.hash(&mut h);
        }
        let fp = h.finish();

        let cache = match self.install_cache.take() {
            Some(c) if c.0 == fp => c,
            _ => (
                fp,
                sorted_entries(install_entries(
                    ximod,
                    &self.selections,
                    &self.file_states,
                    flags,
                    visible,
                )),
            ),
        };
        self.install_cache.insert(cache).1.as_slice()
    }

    /// Final file tree of the current install list, expanded under `root`.
    /// Rebuilt only when the install list or the root changed (walking the
    /// folder sources is bounded, see `MAX_TREE_ENTRIES`, so it is fine on
    /// the UI thread).
    fn install_tree(&mut self, root: &std::path::Path) -> Option<&InstallTree> {
        let (fp, entries) = self.install_cache.as_ref()?;
        let fp = *fp;
        let cache = match self.tree_cache.take() {
            Some(c) if c.0 == fp && c.1 == root => c,
            _ => (fp, root.to_path_buf(), build_install_tree(root, entries)),
        };
        Some(&self.tree_cache.insert(cache).2)
    }

    /// Re-read the saved scenarios of `root` (none without a root).
    pub fn refresh_scenarios(&mut self, root: Option<&std::path::Path>) {
        self.scenario_names = root.map(list_scenarios).unwrap_or_default();
        if self
            .scenario_selected
            .as_ref()
            .is_some_and(|s| !self.scenario_names.contains(s))
        {
            self.scenario_selected = None;
        }
    }
}

/// Translated texts of the file tree.
struct TreeLabels<'a> {
    /// "Overwritten by { $plugin }" hover text.
    overwritten: &'a dyn Fn(&str) -> String,
    /// Suffix of a file listed from inside an archive.
    in_archive: String,
    /// "{ $num } entries" header of an archive's contents.
    entries: &'a dyn Fn(usize) -> String,
}

/// One node of the final file tree: folders as collapsing headers, files
/// with their size; overwritten entries struck through with a hover text.
/// An installed BSA / BA2 gets a collapsed header listing its contents.
fn render_tree_node(
    ui: &mut egui::Ui,
    node: &TreeNode,
    labels: &TreeLabels<'_>,
    nav: &mut FoldFrame<'_>,
    depth: usize,
) {
    if node.is_dir() {
        for child in node.children.values() {
            if child.is_dir() {
                let label = format!("{}/  ({})", child.name, format_size(child.size));
                let header = egui::CollapsingHeader::new(label).id_salt(&child.path);
                nav.header(ui, child.path.as_str(), depth, header, false, |nav, ui| {
                    render_tree_node(ui, child, labels, nav, depth + 1);
                });
            } else {
                render_tree_node(ui, child, labels, nav, depth);
            }
        }
        return;
    }
    if node.in_archive {
        let r = ui.horizontal(|ui| {
            ui.label(RichText::new(&node.name).monospace().weak());
            ui.label(RichText::new(format_size(node.size)).small().weak());
            ui.label(RichText::new(&labels.in_archive).small().weak());
        });
        nav.leaf(ui, node.path.as_str(), depth, r.response.rect);
        return;
    }
    let l_overwritten = labels.overwritten;
    let r = ui.horizontal(|ui| {
        let mut text = RichText::new(&node.name).monospace();
        if node.missing {
            text = text.color(Color32::from_rgb(200, 80, 80));
        }
        let resp = ui.label(text);
        if let Some(src) = &node.source {
            resp.on_hover_text(format!("{} (priority {})", src.source, src.priority));
        }
        ui.label(RichText::new(format_size(node.size)).small().weak());
        if let Some(winner) = &node.source {
            let by = match &winner.loc {
                RefLoc::Plugin { plugin, .. } => plugin.clone(),
                other => format!("{other:?}"),
            };
            for lost in &node.overwritten {
                ui.label(RichText::new(&lost.source).small().weak().strikethrough())
                    .on_hover_text(l_overwritten(&by));
            }
        }
    });
    nav.leaf(ui, node.path.as_str(), depth, r.response.rect);
    if !node.children.is_empty() {
        ui.indent(&node.path, |ui| {
            let header = egui::CollapsingHeader::new(RichText::new((labels.entries)(node.children.len())).small())
                .id_salt(&node.path);
            nav.header(
                ui,
                format!("{}#entries", node.path),
                depth + 1,
                header,
                false,
                |nav, ui| {
                    for child in node.children.values() {
                        render_tree_node(ui, child, labels, nav, depth + 2);
                    }
                },
            );
        });
    }
}

/// The node of `path` (a tree key: a folder, a file, or an archive's
/// "entries" heading), searched depth-first.
fn find_tree_node<'a>(node: &'a TreeNode, path: &str) -> Option<&'a TreeNode> {
    let path = path.strip_suffix("#entries").unwrap_or(path);
    if node.path == path {
        return Some(node);
    }
    node.children.values().find_map(|c| find_tree_node(c, path))
}

/// The component installing a node: the file's own winner, or the first
/// winner found under a folder.
fn tree_node_loc(node: &TreeNode) -> Option<RefLoc> {
    if let Some(src) = &node.source {
        return Some(src.loc.clone());
    }
    node.children.values().find_map(tree_node_loc)
}

impl XimodApp {
    /// (Re)initialise the preview from the current project and show the window.
    pub fn open_preview(&mut self) {
        self.open_preview_inner(None);
    }

    /// Preview another model than the active project (the translated copy
    /// produced by the FOMOD translation window). Files are still resolved
    /// against the active root directory.
    pub fn open_preview_of(&mut self, model: crate::models::Ximod) {
        self.open_preview_inner(Some(std::rc::Rc::new(model)));
    }

    fn open_preview_inner(&mut self, model_override: Option<std::rc::Rc<crate::models::Ximod>>) {
        let mut ps = PreviewState::default();
        let ximod: &crate::models::Ximod = model_override.as_deref().unwrap_or(&self.ximod);

        // Every game file referenced by a `file` dependency, so the
        // assumptions panel can offer a state for each (default: Active).
        ps.file_states = referenced_files(ximod);
        ps.selections = default_selections(ximod, &ps.file_states);
        let visible = evaluate(ximod, &ps.selections, &ps.file_states).visible_steps;
        ps.cursor = *visible.first().unwrap_or(&0);
        ps.finished = false;
        ps.on_info = true;
        ps.model_override = model_override;
        ps.show_tree = self.preview.show_tree;
        ps.refresh_scenarios(self.root_directory.as_deref());
        self.preview = ps;
        self.show_preview = true;
    }

    /// Replace the preview's selections and assumptions with a scenario's.
    /// Returns the names the scenario could not resolve.
    pub(crate) fn preview_apply_scenario(&mut self, scenario: &Scenario) -> Result<Vec<String>, String> {
        let model_override = self.preview.model_override.clone();
        let ximod: &Ximod = model_override.as_deref().unwrap_or(&self.ximod);
        let (selections, files, unresolved) = scenario.apply(ximod).map_err(|e| e.to_string())?;
        self.preview.selections = selections;
        self.preview.file_states = files;
        self.preview.install_cache = None;
        self.preview.history.clear();
        self.preview.focused = None;
        let visible = evaluate(ximod, &self.preview.selections, &self.preview.file_states).visible_steps;
        self.preview.cursor = *visible.first().unwrap_or(&0);
        self.preview.finished = false;
        self.preview.on_info = true;
        Ok(unresolved)
    }

    /// Run a scenario-row action (load / save / delete) and notify.
    fn preview_scenario_action(&mut self, action: ScenarioAction) {
        let Some(root) = self.root_directory.clone() else {
            return;
        };
        match action {
            ScenarioAction::Load(name) => match crate::models::simulate::load_scenario(&root, &name) {
                Ok(sc) => match self.preview_apply_scenario(&sc) {
                    Ok(unresolved) if unresolved.is_empty() => {}
                    Ok(unresolved) => {
                        let msg = self.i18n.t_num("preview-scenario-unresolved", unresolved.len() as i64);
                        self.notify_warn(msg);
                    }
                    Err(e) => self.notify_err(e),
                },
                Err(e) => self.notify_err(e.to_string()),
            },
            ScenarioAction::Save(name) => {
                let model_override = self.preview.model_override.clone();
                let ximod: &Ximod = model_override.as_deref().unwrap_or(&self.ximod);
                let sc = Scenario::from_state(ximod, &self.preview.selections, &self.preview.file_states, name);
                match crate::models::simulate::save_scenario(&root, &sc) {
                    Ok(path) => {
                        let stem = path
                            .file_stem()
                            .map(|s| s.to_string_lossy().into_owned())
                            .unwrap_or(sc.name.clone());
                        self.preview.refresh_scenarios(Some(&root));
                        self.preview.scenario_selected = Some(stem.clone());
                        self.preview.scenario_name_buf.clear();
                        self.preview.scenario_saving = false;
                        let msg = self.i18n.t_arg("preview-scenario-saved", "name", &stem);
                        self.notify_ok(msg);
                    }
                    Err(e) => self.notify_err(e.to_string()),
                }
            }
            ScenarioAction::Delete(name) => match crate::models::simulate::delete_scenario(&root, &name) {
                Ok(()) => {
                    self.preview.refresh_scenarios(Some(&root));
                    self.preview.scenario_selected = None;
                }
                Err(e) => self.notify_err(e.to_string()),
            },
        }
    }

    /// Render the preview window.
    pub fn render_preview(&mut self, ctx: &egui::Context) {
        if !self.show_preview {
            self.free_window_closed("ximod_preview");
            return;
        }

        // Labels (fetched before borrowing fields, so i18n isn't captured).
        let title = self.i18n.t("preview-title");
        let vb = self.free_viewport_builder(ctx, "ximod_preview", title, [860.0, 620.0], false);
        let l_refresh = self.i18n.t("preview-refresh");
        let l_assume = self.i18n.t("preview-assumptions");
        let l_back = self.i18n.t("preview-back");
        let l_install = self.i18n.t("preview-install");
        let l_close = self.i18n.t("preview-close");
        let l_restart = self.i18n.t("preview-restart");
        let l_summary = self.i18n.t("preview-summary-title");
        let l_empty = self.i18n.t("preview-empty");
        let l_none = self.i18n.t("preview-none-option");
        let l_invalid = self.i18n.t("preview-invalid");
        let l_nosteps = self.i18n.t("preview-no-steps");
        let l_info_title = self.i18n.t("tree-mod-info");
        let l_author = self.i18n.t("label-author");
        let l_version = self.i18n.t("label-version");
        let l_url = self.i18n.t("label-url");
        let l_col_src = self.i18n.t("preview-col-source");
        let l_col_dst = self.i18n.t("preview-col-dest");
        let l_col_prio = self.i18n.t("preview-col-priority");
        let l_desc_none = self.i18n.t("preview-select-hint");
        let untitled = self.i18n.t("tab-untitled");
        let l_tree = self.i18n.t("preview-tree");
        let l_tree_truncated = self.i18n.t("preview-tree-truncated");
        let l_scenario = self.i18n.t("preview-scenario");
        let l_scenario_none = self.i18n.t("preview-scenario-none");
        let l_scenario_load = self.i18n.t("preview-scenario-load");
        let l_scenario_save = self.i18n.t("preview-scenario-save");
        let l_scenario_delete = self.i18n.t("preview-scenario-delete");
        let l_scenario_name = self.i18n.t("preview-scenario-name");
        let l_no_root = self.i18n.t("msg-no-root-selected");
        let l_overwritten = |plugin: &str| -> String {
            let mut args = FluentArgs::new();
            args.set("plugin", plugin.to_string());
            self.i18n.t_with_args("preview-overwritten-by", Some(&args))
        };
        let l_total = |size: &str| -> String {
            let mut args = FluentArgs::new();
            args.set("size", size.to_string());
            self.i18n.t_with_args("preview-total-size", Some(&args))
        };
        let l_archived = |size: &str| -> String {
            let mut args = FluentArgs::new();
            args.set("size", size.to_string());
            self.i18n.t_with_args("preview-archived-size", Some(&args))
        };
        let l_entries = |num: usize| -> String { self.i18n.t_num("archive-view-entries", num as i64) };
        let fold_labels = FoldLabels::new(&self.i18n);
        let tree_labels = TreeLabels {
            overwritten: &l_overwritten,
            in_archive: self.i18n.t("preview-in-archive"),
            entries: &l_entries,
        };
        let i18n = &self.i18n;
        let sel_hint = |t: SelectionType| -> String {
            match t {
                SelectionType::SelectExactlyOne => self.i18n.t("preview-sel-exactlyone"),
                SelectionType::SelectAtMostOne => self.i18n.t("preview-sel-atmostone"),
                SelectionType::SelectAny => self.i18n.t("preview-sel-any"),
                SelectionType::SelectAll => self.i18n.t("preview-sel-all"),
                SelectionType::SelectAtLeastOne => self.i18n.t("preview-sel-atleastone"),
            }
        };
        let type_label = |t: PluginType| -> Option<String> {
            match t {
                PluginType::Required | PluginType::Recommended | PluginType::NotUsable | PluginType::CouldBeUsable => {
                    Some(crate::ui::labels::plugin_type(i18n, t))
                }
                PluginType::Optional => None,
            }
        };

        // Whether to interpret literal "\n" sequences in descriptions as line
        // breaks, matching the "Process newlines in descriptions" setting so the
        // preview renders descriptions the way a manager will.
        let replace_newlines = self.config.replace_newlines;

        // How many steps are currently hidden by unmet visibility conditions.
        // Shown as a hint so the author understands why later pages don't appear
        // ("the preview only shows the first screen") instead of it looking like
        // a bug.
        //
        // Single forward pass per frame: the same result drives this hint, the
        // navigation bar and the option list below.
        // A translated copy may stand in for the active project.
        let model_override = self.preview.model_override.clone();
        let ximod_ref: &crate::models::Ximod = model_override.as_deref().unwrap_or(&self.ximod);
        let ev = evaluate(ximod_ref, &self.preview.selections, &self.preview.file_states);
        let (flags, visible) = (ev.flags, ev.visible_steps);
        let hidden_count = ximod_ref.steps.len().saturating_sub(visible.len());
        let l_hidden = if hidden_count > 0 {
            self.i18n.t_num("preview-hidden-steps", hidden_count as i64)
        } else {
            String::new()
        };

        // Disjoint field borrows for the closure.
        let ximod = ximod_ref;
        let preview = &mut self.preview;
        let root = self.root_directory.clone();
        let cfg = &mut self.config;
        let mut do_refresh = false;
        let mut do_close = false;
        let mut scenario_action: Option<ScenarioAction> = None;
        // Set when the file-state assumptions are edited during this frame, in
        // which case `flags` / `visible` above no longer match them.
        let mut assumptions_changed = false;

        // Keep the cursor on a visible step while navigating.
        if !preview.finished && !preview.on_info {
            if visible.is_empty() {
                preview.finished = true;
            } else if !visible.contains(&preview.cursor) {
                preview.cursor = *visible
                    .iter()
                    .find(|&&s| s >= preview.cursor)
                    .unwrap_or(visible.first().unwrap());
            }
        }

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("ximod_preview"), vb, |ctx, _class| {
            // ---- Bottom: navigation bar, in its own panel so it is ALWAYS
            // visible and can never be pushed off-screen by the options list
            // (which previously happened: the preview then looked stuck on the
            // first page because Back / Next / Install were below the window). ----
            egui::TopBottomPanel::bottom("preview_nav").show(ctx, |ui| {
                ui.add_space(6.0);
                let step_ok = preview.finished || preview.on_info || visible.is_empty() || {
                    let step = &ximod.steps[preview.cursor];
                    step.plugin_groups
                        .iter()
                        .enumerate()
                        .all(|(gi, g)| group_valid(g, &preview.selections, preview.cursor, gi))
                };
                // Vortex-style bar: the left button names the step you go back to,
                // the right button names the step Next leads to (or "Install" on the
                // last page), and a progress bar fills the space between them.
                let name_of = |s: usize| -> String {
                    match ximod.steps.get(s) {
                        Some(st) if !st.name.is_empty() => st.name.clone(),
                        _ => crate::ui::labels::step_fallback(i18n, s),
                    }
                };
                let pos = visible.iter().position(|&s| s == preview.cursor).unwrap_or(0);
                // Pages: the mod-information page, then the visible steps.
                let page_count = visible.len() + 1;
                ui.horizontal(|ui| {
                    // ---- Left: back to the previously visited page ----
                    // `INFO_PAGE` in the history stands for the information page.
                    let back_target = if preview.on_info {
                        None
                    } else {
                        preview.history.last().copied()
                    };
                    let back_label = match back_target {
                        Some(INFO_PAGE) => format!("◀  {}", l_info_title),
                        Some(s) => format!("◀  {}", name_of(s)),
                        None => l_back.clone(),
                    };
                    if ui
                        .add_enabled(back_target.is_some(), egui::Button::new(back_label))
                        .clicked()
                        && let Some(prev) = preview.history.pop()
                    {
                        preview.finished = false;
                        if prev == INFO_PAGE {
                            preview.on_info = true;
                        } else {
                            preview.cursor = prev;
                        }
                    }

                    // ---- Right side (filled right-to-left), progress in the middle ----
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if preview.finished {
                            if ui.button(&l_close).clicked() {
                                do_close = true;
                            }
                            if ui.button(&l_restart).clicked() {
                                do_refresh = true;
                            }
                        } else {
                            let next_target = if preview.on_info {
                                visible.first().copied()
                            } else {
                                visible.iter().copied().find(|&s| s > preview.cursor)
                            };
                            let next_label = match next_target {
                                Some(s) => format!("{}  ▶", name_of(s)),
                                None => l_install.clone(),
                            };
                            if ui.add_enabled(step_ok, egui::Button::new(next_label)).clicked() {
                                preview
                                    .history
                                    .push(if preview.on_info { INFO_PAGE } else { preview.cursor });
                                preview.on_info = false;
                                match next_target {
                                    Some(n) => preview.cursor = n,
                                    None => preview.finished = true,
                                }
                            }
                            if !step_ok {
                                ui.label(RichText::new(&l_invalid).small().color(Color32::from_rgb(200, 80, 80)));
                            }
                        }
                        // Progress bar fills the gap between the two buttons.
                        {
                            let shown = if preview.finished {
                                page_count
                            } else if preview.on_info {
                                1
                            } else {
                                pos + 2
                            };
                            let frac = shown as f32 / page_count as f32;
                            let avail = ui.available_width();
                            if avail > 60.0 {
                                ui.add(
                                    egui::ProgressBar::new(frac)
                                        .desired_width(avail)
                                        .text(format!("{} / {}", shown, page_count)),
                                );
                            }
                        }
                    });
                });
                ui.add_space(4.0);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                // ---- Top bar: header, refresh, assumptions toggle ----
                ui.horizontal(|ui| {
                    ui.heading(if ximod.name.is_empty() {
                        untitled.as_str()
                    } else {
                        ximod.name.as_str()
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&l_refresh).clicked() {
                            do_refresh = true;
                        }
                        let n_files = preview.file_states.len();
                        if n_files > 0 {
                            let lbl = format!("{} ({})", l_assume, n_files);
                            ui.toggle_value(&mut preview.show_assumptions, lbl);
                        }
                    });
                });

                // ---- File-state assumptions panel ----
                if preview.show_assumptions && !preview.file_states.is_empty() {
                    ui.group(|ui| {
                        ui.label(RichText::new(&l_assume).strong());
                        egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                            let names: Vec<String> = preview.file_states.keys().cloned().collect();
                            for name in names {
                                ui.horizontal(|ui| {
                                    let cur = preview.file_states[&name];
                                    egui::ComboBox::from_id_salt(("fs", &name))
                                        .selected_text(cur.as_str())
                                        .show_ui(ui, |ui| {
                                            for st in FileState::variants() {
                                                let mut c = cur;
                                                if ui.selectable_value(&mut c, *st, st.as_str()).clicked() {
                                                    preview.file_states.insert(name.clone(), *st);
                                                    assumptions_changed = true;
                                                }
                                            }
                                        });
                                    ui.label(&name);
                                });
                            }
                        });
                    });
                }

                ui.separator();

                if preview.on_info {
                    // ================= MOD INFORMATION =================
                    // What a mod manager shows before the first step: the
                    // header image, the title and the info.xml metadata.
                    preview.install_cache = None;
                    ui.label(RichText::new(&l_info_title).heading());
                    ui.add_space(6.0);
                    egui::ScrollArea::vertical().id_salt("preview_info").show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            if ximod.image_show_image != Some(false)
                                && let Some(rel) = ximod.header_image.as_ref()
                                && let Some(r) = root.as_ref()
                            {
                                let abs = r.join(rel.replace('\\', "/"));
                                let img_w = ui.available_width().clamp(200.0, 760.0);
                                let img_h = match ximod.image_height {
                                    Some(h) if h > 0 => (h as f32).clamp(60.0, 400.0),
                                    _ => (img_w * 0.4).clamp(120.0, 320.0),
                                };
                                ImageDisplay::new(img_w, img_h)
                                    .with_fallback(" ")
                                    .show(ui, Some(abs.as_path()));
                                ui.add_space(8.0);
                            }
                            let mut title = RichText::new(if ximod.name.is_empty() {
                                untitled.as_str()
                            } else {
                                ximod.name.as_str()
                            })
                            .heading();
                            if let Some(colour) = ximod
                                .title_colour
                                .as_deref()
                                .and_then(|c| u32::from_str_radix(c.trim_start_matches('#'), 16).ok())
                            {
                                title = title.color(Color32::from_rgb(
                                    ((colour >> 16) & 0xFF) as u8,
                                    ((colour >> 8) & 0xFF) as u8,
                                    (colour & 0xFF) as u8,
                                ));
                            }
                            ui.label(title);
                            ui.add_space(4.0);
                            if !ximod.author.is_empty() {
                                ui.label(format!("{} {}", l_author, ximod.author));
                            }
                            if !ximod.version.is_empty() {
                                ui.label(format!("{} {}", l_version, ximod.version));
                            }
                            if !ximod.url.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.label(&l_url);
                                    ui.hyperlink(&ximod.url);
                                });
                            }
                            if !ximod.description.is_empty() {
                                ui.add_space(8.0);
                                ui.separator();
                                ui.add_space(4.0);
                                if replace_newlines {
                                    ui.label(preview.processed_desc(&ximod.description));
                                } else {
                                    ui.label(ximod.description.as_str());
                                }
                            }
                        });
                    });
                } else if preview.finished {
                    // ================= INSTALL SUMMARY =================
                    ui.label(RichText::new(&l_summary).heading());
                    // The frame's forward pass is reused unless an assumption
                    // was just edited above, which may change which steps are
                    // visible and which conditional sets apply.
                    let recomputed =
                        assumptions_changed.then(|| evaluate(ximod, &preview.selections, &preview.file_states));
                    let (sum_flags, sum_visible) = match &recomputed {
                        Some(ev) => (&ev.flags, ev.visible_steps.as_slice()),
                        None => (&flags, visible.as_slice()),
                    };

                    // ---- Scenarios row (needs the root folder) ----
                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(root.is_some(), |ui| {
                            ui.label(&l_scenario);
                            let current = preview
                                .scenario_selected
                                .clone()
                                .unwrap_or_else(|| l_scenario_none.clone());
                            let names = preview.scenario_names.clone();
                            egui::ComboBox::from_id_salt("preview_scenario")
                                .selected_text(current)
                                .show_ui(ui, |ui| {
                                    if names.is_empty() {
                                        ui.label(RichText::new(&l_scenario_none).weak());
                                    }
                                    for n in &names {
                                        let selected = preview.scenario_selected.as_deref() == Some(n.as_str());
                                        if ui.selectable_label(selected, n).clicked() {
                                            preview.scenario_selected = Some(n.clone());
                                        }
                                    }
                                });
                            let has_sel = preview.scenario_selected.is_some();
                            if ui.add_enabled(has_sel, egui::Button::new(&l_scenario_load)).clicked()
                                && let Some(n) = preview.scenario_selected.clone()
                            {
                                scenario_action = Some(ScenarioAction::Load(n));
                            }
                            if ui.add_enabled(has_sel, egui::Button::new(&l_scenario_delete)).clicked()
                                && let Some(n) = preview.scenario_selected.clone()
                            {
                                scenario_action = Some(ScenarioAction::Delete(n));
                            }
                            if preview.scenario_saving {
                                let edit = ui.add(
                                    egui::TextEdit::singleline(&mut preview.scenario_name_buf)
                                        .hint_text(&l_scenario_name)
                                        .desired_width(160.0),
                                );
                                let enter = edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                                let name = preview.scenario_name_buf.trim().to_string();
                                if (ui
                                    .add_enabled(!name.is_empty(), egui::Button::new(&l_scenario_save))
                                    .clicked()
                                    || enter)
                                    && !name.is_empty()
                                {
                                    scenario_action = Some(ScenarioAction::Save(name));
                                }
                            } else if ui.button(&l_scenario_save).clicked() {
                                preview.scenario_saving = true;
                            }
                        });
                        if root.is_none() {
                            ui.label(RichText::new("ⓘ").weak()).on_hover_text(&l_no_root);
                        }
                    });
                    ui.add_space(4.0);

                    let install = preview.install_list(ximod, sum_flags, sum_visible);
                    let n_install = install.len();
                    if n_install == 0 {
                        ui.label(&l_empty);
                    } else {
                        ui.label(format!("{} : {}", l_summary, n_install));
                        egui::ScrollArea::vertical()
                            .id_salt("preview_install_scroll")
                            .auto_shrink([false, true])
                            .max_height(240.0)
                            .show(ui, |ui| {
                                egui::Grid::new("preview_install")
                                    .num_columns(3)
                                    .striped(true)
                                    .min_col_width(120.0)
                                    .show(ui, |ui| {
                                        ui.label(RichText::new(&l_col_src).strong());
                                        ui.label(RichText::new(&l_col_dst).strong());
                                        ui.label(RichText::new(&l_col_prio).strong());
                                        ui.end_row();
                                        for e in install {
                                            let f = &e.file;
                                            let src = if f.source.is_empty() { "—" } else { f.source.as_str() };
                                            ui.label(RichText::new(src).monospace());
                                            ui.label(RichText::new(f.destination.as_str()).monospace());
                                            ui.label(f.priority.to_string());
                                            ui.end_row();
                                        }
                                    });
                            });

                        // ---- Final file tree (needs the root folder) ----
                        if let Some(root) = root.as_deref() {
                            ui.add_space(4.0);
                            let header = egui::CollapsingHeader::new(RichText::new(&l_tree).strong())
                                .id_salt("preview_tree")
                                .open(Some(preview.show_tree));
                            let show = preview.show_tree;
                            let mut tree_nav = std::mem::take(&mut preview.tree_nav);
                            let mut goto: Option<RefLoc> = None;
                            let resp = header.show(ui, |ui| {
                                if let Some(tree) = preview.install_tree(root) {
                                    let total = l_total(&format_size(tree.total_size));
                                    ui.label(format!("{total}  ·  {} files", tree.files));
                                    if tree.archived_bytes > 0 {
                                        ui.label(
                                            RichText::new(l_archived(&format_size(tree.archived_bytes)))
                                                .small()
                                                .weak(),
                                        );
                                    }
                                    if tree.truncated {
                                        ui.label(
                                            RichText::new(&l_tree_truncated)
                                                .small()
                                                .color(Color32::from_rgb(150, 120, 60)),
                                        );
                                    }
                                    let mut f = tree_nav.frame(&fold_labels, true);
                                    egui::ScrollArea::vertical()
                                        .id_salt("preview_tree_scroll")
                                        .auto_shrink([false, true])
                                        .max_height(260.0)
                                        .show(ui, |ui| {
                                            render_tree_node(ui, &tree.root, &tree_labels, &mut f, 0);
                                        });
                                    // ↑ ↓ ← → over the tree; Enter selects the
                                    // component installing the row.
                                    if let Some(key) = tree_nav.end_frame(ui.ctx(), f) {
                                        goto = find_tree_node(&tree.root, &key).and_then(tree_node_loc);
                                    }
                                }
                            });
                            preview.tree_nav = tree_nav;
                            if goto.is_some() {
                                preview.goto = goto;
                            }
                            if resp.header_response.clicked() {
                                preview.show_tree = !show;
                            }
                        }
                    }
                } else if visible.is_empty() {
                    preview.install_cache = None;
                    ui.label(&l_nosteps);
                } else {
                    // Left the summary page: its cached list is no longer needed.
                    preview.install_cache = None;

                    // ================= CURRENT STEP =================
                    let si = preview.cursor;
                    let step = &ximod.steps[si];
                    let pos = visible.iter().position(|&s| s == si).unwrap_or(0);
                    ui.label(
                        RichText::new(format!(
                            "{} — {}/{}",
                            if step.name.is_empty() {
                                crate::ui::labels::step_fallback(i18n, pos)
                            } else {
                                step.name.clone()
                            },
                            pos + 1,
                            visible.len()
                        ))
                        .strong()
                        .size(15.0),
                    );
                    if !l_hidden.is_empty() {
                        ui.label(RichText::new(&l_hidden).small().color(Color32::from_rgb(150, 120, 60)));
                    }
                    ui.add_space(4.0);

                    ui.columns(2, |cols| {
                        // ---- Left: groups & options ----
                        egui::ScrollArea::vertical()
                            .id_salt("preview_opts")
                            .auto_shrink([false, false])
                            .show(&mut cols[0], |ui| {
                                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                    for (gi, group) in step.plugin_groups.iter().enumerate() {
                                        ui.group(|ui| {
                                            ui.label(RichText::new(&group.name).strong());
                                            ui.label(
                                                RichText::new(sel_hint(group.selection_type))
                                                    .small()
                                                    .color(ui.visuals().weak_text_color()),
                                            );
                                            ui.add_space(2.0);

                                            let radio = matches!(
                                                group.selection_type,
                                                SelectionType::SelectExactlyOne | SelectionType::SelectAtMostOne
                                            );

                                            // "(none)" option for SelectAtMostOne.
                                            if group.selection_type == SelectionType::SelectAtMostOne {
                                                let any = (0..group.plugins.len())
                                                    .any(|pi| *preview.selections.get(&(si, gi, pi)).unwrap_or(&false));
                                                if ui.radio(!any, &l_none).clicked() {
                                                    for pi in 0..group.plugins.len() {
                                                        preview.selections.insert((si, gi, pi), false);
                                                    }
                                                }
                                            }

                                            for (pi, plugin) in group.plugins.iter().enumerate() {
                                                let key = (si, gi, pi);
                                                let et = effective_type(plugin, &flags, &preview.file_states);
                                                let forced_on = et == PluginType::Required
                                                    || group.selection_type == SelectionType::SelectAll;
                                                let forced_off = et == PluginType::NotUsable;
                                                let mut checked = *preview.selections.get(&key).unwrap_or(&false);
                                                if forced_on {
                                                    checked = true;
                                                }
                                                if forced_off {
                                                    checked = false;
                                                }
                                                preview.selections.insert(key, checked);
                                                let enabled = !(forced_on || forced_off);

                                                ui.horizontal(|ui| {
                                                    let resp = ui.add_enabled_ui(enabled, |ui| {
                                                        if radio {
                                                            ui.radio(checked, &plugin.name)
                                                        } else {
                                                            let mut c = checked;
                                                            ui.checkbox(&mut c, &plugin.name)
                                                        }
                                                    });
                                                    let clicked = resp.inner.clicked();
                                                    if resp.inner.hovered() || clicked {
                                                        preview.focused = Some(key);
                                                    }
                                                    if clicked {
                                                        if radio {
                                                            for other in 0..group.plugins.len() {
                                                                preview.selections.insert((si, gi, other), other == pi);
                                                            }
                                                        } else {
                                                            preview.selections.insert(key, !checked);
                                                        }
                                                    }
                                                    if let Some(tl) = type_label(et) {
                                                        ui.label(
                                                            RichText::new(tl)
                                                                .small()
                                                                .color(Color32::from_rgb(150, 120, 60)),
                                                        );
                                                    }
                                                });
                                            }
                                        });
                                    }
                                });
                            });

                        // ---- Right: detail of the focused plugin ----
                        // `ui.columns` gives each column a *justified* layout, which
                        // stretches wrapped description text edge-to-edge (the ugly
                        // "R E Q U I R E M E N T S" spacing). A mod manager like Vortex
                        // left-aligns it, so render the whole detail pane in a plain
                        // top-down, left-aligned layout.
                        let ui = &mut cols[1];
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            // No "Details" header and no repeated option name: like Vortex,
                            // the pane shows just the image of the focused option and its
                            // description (the option itself is highlighted in the list).
                            let focus = preview.focused.filter(|&(fs, fg, fp)| {
                                fs == si && fg < step.plugin_groups.len() && fp < step.plugin_groups[fg].plugins.len()
                            });
                            match focus {
                                Some((_, fg, fp)) => {
                                    let plugin = &step.plugin_groups[fg].plugins[fp];
                                    let abs = plugin
                                        .image_path
                                        .as_ref()
                                        .and_then(|rel| root.as_ref().map(|r| r.join(rel.replace('\\', "/"))));
                                    // Prominent banner image like Vortex: fill the pane
                                    // width (capped), keeping aspect within the box.
                                    let img_w = ui.available_width().clamp(200.0, 560.0);
                                    let img_h = (img_w * 0.42).clamp(120.0, 260.0);
                                    ImageDisplay::new(img_w, img_h)
                                        .with_fallback(" ")
                                        .show(ui, abs.as_deref());
                                    ui.add_space(6.0);
                                    egui::ScrollArea::vertical()
                                        .id_salt("preview_desc")
                                        .auto_shrink([false, false])
                                        .show(ui, |ui| {
                                            if replace_newlines {
                                                ui.label(preview.processed_desc(&plugin.description));
                                            } else {
                                                ui.label(plugin.description.as_str());
                                            }
                                        });
                                }
                                None => {
                                    ui.label(RichText::new(&l_desc_none).color(ui.visuals().weak_text_color()));
                                }
                            }
                        });
                    });
                }
            });

            crate::ui::widgets::free_window::record_win_geom(cfg, ctx, "ximod_preview");
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        if let Some(action) = scenario_action {
            self.preview_scenario_action(action);
        }
        if let Some(loc) = self.preview.goto.take()
            && let Some(target) = self.target_of_loc(&loc)
        {
            self.select_target(target);
        }
        if do_refresh {
            self.open_preview();
        } else if do_close {
            self.show_preview = false;
            self.free_window_closed("ximod_preview");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::simulate::compute_install;
    use crate::models::{ConditionFlag, ConditionalFileSet, Dependency, InstallFile, Plugin, PluginGroup, Step, Ximod};

    /// Build a project: step 1 chooses a texture resolution (sets flag `res`),
    /// step 2 is visible only when `res = 4K`, and a conditional file set installs
    /// an extra file when `res = 4K`.
    fn sample() -> Ximod {
        let mut m = Ximod::new("Sample");
        m.required_files.push(InstallFile::new_file("base.esp"));

        let mut g = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        let mut a = Plugin::new("2K");
        a.condition_flags.push(ConditionFlag::new("res", "2K"));
        a.files.push(InstallFile::new_file("tex2k/a.dds"));
        let mut b = Plugin::new("4K");
        b.condition_flags.push(ConditionFlag::new("res", "4K"));
        b.files.push(InstallFile::new_file("tex4k/b.dds"));
        g.plugins.push(a);
        g.plugins.push(b);
        let mut s1 = Step::new("Resolution");
        s1.plugin_groups.push(g);

        let mut s2 = Step::new("Extras (4K only)");
        s2.visibility.push_leaf(Dependency::new_flag("res", "4K"));
        m.steps.push(s1);
        m.steps.push(s2);

        let mut cfs = ConditionalFileSet::new();
        cfs.condition.push_leaf(Dependency::new_flag("res", "4K"));
        cfs.files.push(InstallFile::new_file("patch4k.esp"));
        m.conditional_files.push(cfs);
        m
    }

    #[test]
    fn multi_step_navigation_reaches_every_page() {
        // Four always-visible steps (no <visible> conditions), like the real
        // "Lodecs Custom Armory" FOMOD. Every page must be reachable with Next.
        let mut m = Ximod::new("Multi");
        for i in 0..4 {
            let mut g = PluginGroup::new(format!("G{i}"), SelectionType::SelectExactlyOne);
            g.plugins.push(Plugin::new("A"));
            g.plugins.push(Plugin::new("B"));
            let mut s = Step::new(format!("Step {i}"));
            s.plugin_groups.push(g);
            m.steps.push(s);
        }
        let files = BTreeMap::new();
        let sel = default_selections(&m, &files);
        let visible = evaluate(&m, &sel, &files).visible_steps;
        assert_eq!(visible, vec![0, 1, 2, 3]);
        // Simulate the Next progression used by the navigation bar.
        let mut cursor = *visible.first().unwrap();
        let mut seen = vec![cursor];
        while let Some(n) = visible.iter().copied().find(|&s| s > cursor) {
            cursor = n;
            seen.push(cursor);
        }
        assert_eq!(seen, vec![0, 1, 2, 3]);
    }

    #[test]
    fn process_desc_interprets_literal_newlines_when_enabled() {
        // Off: text is untouched.
        assert_eq!(process_desc("a\\nb", false), "a\\nb");
        // On: literal \n / \r\n become real line breaks.
        assert_eq!(process_desc("a\\nb", true), "a\nb");
        assert_eq!(process_desc("a\\r\\nb", true), "a\nb");
        // Real newlines already present are preserved either way.
        assert_eq!(process_desc("a\nb", true), "a\nb");
    }

    #[test]
    fn cached_install_list_follows_selection_changes() {
        let m = sample();
        let mut ps = PreviewState {
            selections: default_selections(&m, &BTreeMap::new()),
            ..Default::default()
        };
        let sources = |ps: &mut PreviewState| -> Vec<String> {
            let ev = evaluate(&m, &ps.selections, &ps.file_states);
            ps.install_list(&m, &ev.flags, &ev.visible_steps)
                .iter()
                .map(|e| e.file.source.clone())
                .collect()
        };
        let expected = |ps: &PreviewState| -> Vec<String> {
            compute_install(&m, &ps.selections, &ps.file_states)
                .into_iter()
                .map(|f| f.source)
                .collect()
        };

        let first = sources(&mut ps);
        assert_eq!(first, expected(&ps));
        let fp = ps.install_cache.as_ref().map(|c| c.0);
        // Unchanged inputs: same fingerprint, same list.
        assert_eq!(sources(&mut ps), first);
        assert_eq!(ps.install_cache.as_ref().map(|c| c.0), fp);

        // Switching to 4K must invalidate the cached list.
        ps.selections.insert((0, 0, 0), false);
        ps.selections.insert((0, 0, 1), true);
        let second = sources(&mut ps);
        assert_eq!(second, expected(&ps));
        assert_ne!(second, first);
    }

    #[test]
    fn cached_tree_follows_install_list_and_root() {
        let root = std::env::temp_dir().join(format!("ximod_preview_tree_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("tex4k")).unwrap();
        std::fs::write(root.join("base.esp"), b"12345").unwrap();
        std::fs::write(root.join("tex4k/b.dds"), b"1234567").unwrap();
        let m = sample();
        let mut ps = PreviewState {
            selections: default_selections(&m, &BTreeMap::new()),
            ..Default::default()
        };
        // No install list yet: no tree.
        assert!(ps.install_tree(&root).is_none());
        let ev = evaluate(&m, &ps.selections, &ps.file_states);
        let _ = ps.install_list(&m, &ev.flags, &ev.visible_steps);
        let t1 = ps.install_tree(&root).unwrap();
        assert_eq!(t1.total_size, 5); // base.esp; a.dds missing
        assert_eq!(t1.missing, vec!["tex2k/a.dds".to_string()]);
        ps.selections.insert((0, 0, 0), false);
        ps.selections.insert((0, 0, 1), true);
        let ev = evaluate(&m, &ps.selections, &ps.file_states);
        let _ = ps.install_list(&m, &ev.flags, &ev.visible_steps);
        let t2 = ps.install_tree(&root).unwrap();
        assert_eq!(t2.total_size, 12);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn cached_description_tracks_its_source_text() {
        let mut ps = PreviewState::default();
        assert_eq!(ps.processed_desc("a\\nb"), "a\nb");
        assert_eq!(ps.processed_desc("a\\nb"), "a\nb");
        assert_eq!(ps.processed_desc("c\\r\\nd"), "c\nd");
    }

    #[test]
    fn scenario_round_trip_through_the_preview() {
        let root = std::env::temp_dir().join(format!("ximod_preview_sc_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.ximod = sample();
        app.root_directory = Some(root.clone());
        app.open_preview();
        assert!(app.preview.scenario_names.is_empty());
        // Pick 4K, save, reset, load back.
        app.preview.selections.insert((0, 0, 0), false);
        app.preview.selections.insert((0, 0, 1), true);
        app.preview_scenario_action(ScenarioAction::Save("Ultra".into()));
        assert_eq!(app.preview.scenario_names, vec!["Ultra".to_string()]);
        assert_eq!(app.preview.scenario_selected.as_deref(), Some("Ultra"));
        app.open_preview();
        assert_eq!(app.preview.selections.get(&(0, 0, 1)), Some(&false));
        app.preview_scenario_action(ScenarioAction::Load("Ultra".into()));
        assert_eq!(app.preview.selections.get(&(0, 0, 1)), Some(&true));
        assert!(!app.preview.finished);
        // A scenario of another project reports its unresolved names.
        let sc = Scenario {
            name: "x".into(),
            file_states: BTreeMap::new(),
            selections: vec![
                crate::models::simulate::SelectionSpec {
                    step: "Resolution".into(),
                    group: "Resolution".into(),
                    plugin: "2K".into(),
                },
                crate::models::simulate::SelectionSpec {
                    step: "Gone".into(),
                    group: "G".into(),
                    plugin: "P".into(),
                },
            ],
        };
        assert_eq!(
            app.preview_apply_scenario(&sc).unwrap(),
            vec!["Gone / G / P".to_string()]
        );
        app.preview_scenario_action(ScenarioAction::Delete("Ultra".into()));
        assert!(app.preview.scenario_names.is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    /// The preview opens on the mod-information page, even for a project
    /// without any step (which then goes straight to the summary).
    #[test]
    fn preview_starts_on_the_mod_information_page() {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.ximod = sample();
        app.open_preview();
        assert!(app.preview.on_info);
        assert!(!app.preview.finished);
        app.ximod = Ximod::new("Empty");
        app.open_preview();
        assert!(app.preview.on_info);
        assert!(!app.preview.finished);
        // Loading a scenario returns to the first page too.
        app.ximod = sample();
        app.open_preview();
        app.preview.on_info = false;
        let sc = Scenario {
            name: "x".into(),
            file_states: BTreeMap::new(),
            selections: Vec::new(),
        };
        app.preview_apply_scenario(&sc).unwrap();
        assert!(app.preview.on_info);
    }

    /// The final file tree of the summary page: ↑ ↓ ← → move and fold like
    /// the project tree, Enter selects the component installing the row.
    #[test]
    fn keyboard_navigation_in_the_file_tree() {
        use crate::ui::main_window::Tab;
        use crate::ui::problems::Target;
        let root = std::env::temp_dir().join(format!("ximod_preview_nav_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("tex2k")).unwrap();
        std::fs::write(root.join("base.esp"), b"12345").unwrap();
        std::fs::write(root.join("tex2k/a.dds"), b"1234567").unwrap();
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.ximod = sample();
        // Installed under textures/: the tree gets a folder heading.
        app.ximod.steps[0].plugin_groups[0].plugins[0].files[0].destination = "textures/a.dds".into();
        app.root_directory = Some(root.clone());
        app.open_preview();
        app.preview.on_info = false;
        app.preview.finished = true;
        app.preview.show_tree = true;
        let ctx = egui::Context::default();
        let frame = |app: &mut XimodApp, keys: &[egui::Key]| {
            let events = keys
                .iter()
                .map(|&key| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                })
                .collect();
            let _ = ctx.run(
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                |ctx| app.render_preview(ctx),
            );
            for _ in 0..12 {
                let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_preview(ctx));
            }
        };
        frame(&mut app, &[]);
        // Rows: base.esp, textures/ (closed).
        frame(&mut app, &[egui::Key::ArrowDown]);
        assert_eq!(app.preview.tree_nav.cursor.as_deref(), Some("textures"));
        frame(&mut app, &[egui::Key::ArrowRight]);
        frame(&mut app, &[egui::Key::ArrowDown]);
        assert_eq!(app.preview.tree_nav.cursor.as_deref(), Some("textures/a.dds"));
        // Enter: the option installing a.dds ("2K", first option of step 1).
        app.select_target(Target::Info);
        frame(&mut app, &[egui::Key::Enter]);
        assert_eq!(app.current_tab, Tab::Steps);
        assert_eq!(app.selection.plugin, Some(0));
        // ← climbs to the folder, ← folds it, ↑ reaches base.esp; Enter goes
        // to the required files.
        frame(&mut app, &[egui::Key::ArrowLeft]);
        assert_eq!(app.preview.tree_nav.cursor.as_deref(), Some("textures"));
        frame(&mut app, &[egui::Key::ArrowLeft]);
        frame(&mut app, &[egui::Key::ArrowUp]);
        assert_eq!(app.preview.tree_nav.cursor.as_deref(), Some("base.esp"));
        frame(&mut app, &[egui::Key::Enter]);
        assert_eq!(app.current_tab, Tab::RequiredInstalls);
        frame(&mut app, &[egui::Key::ArrowDown, egui::Key::ArrowDown]);
        assert_eq!(app.preview.tree_nav.cursor.as_deref(), Some("textures"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn file_dependency_assumption_is_honoured() {
        // A step visible only when a game file is Active.
        let mut m = Ximod::new("F");
        let mut s = Step::new("needs file");
        s.visibility.push_leaf(Dependency::new_file("Skyrim.esm", "Active"));
        m.steps.push(s);
        let sel = HashMap::new();

        let mut files = BTreeMap::new();
        files.insert("Skyrim.esm".to_string(), FileState::Active);
        assert_eq!(evaluate(&m, &sel, &files).visible_steps, vec![0]);

        files.insert("Skyrim.esm".to_string(), FileState::Missing);
        assert_eq!(evaluate(&m, &sel, &files).visible_steps, Vec::<usize>::new());
    }
}
