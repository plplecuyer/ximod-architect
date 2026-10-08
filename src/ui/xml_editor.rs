//! In-app XML editor (skeleton).
//!
//! Lets advanced users view — and optionally hand-edit — the generated
//! `info.xml` and `ModuleConfig.xml` for the current project.
//!
//! Synchronization model (single source of truth at any moment):
//! - Opening the editor serializes the current model to text (model → text) and
//!   shows it **read-only**.
//! - "Edit" switches to edit mode; while the editor is open the graphical tabs
//!   are locked (the editor window is treated as modal).
//! - "Apply" re-parses the edited text back into the model (text → model). On
//!   success the model is updated and the editor returns to read-only; on
//!   failure the parse error is shown and the text stays editable.
//! - "Cancel" discards the edits and regenerates the text from the model.
//!
//! This is the workflow skeleton: syntax highlighting (colouring) and live
//! validation are added in later steps and plug into this structure.

use crate::models::Ximod;
use crate::ui::main_window::XimodApp;
use eframe::egui;
use std::sync::Arc;

/// Which file the XML editor is currently showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum XmlTarget {
    #[default]
    InfoXml,
    ModuleConfig,
}

impl XmlTarget {
    fn file_name(self) -> &'static str {
        match self {
            XmlTarget::InfoXml => "info.xml",
            XmlTarget::ModuleConfig => "ModuleConfig.xml",
        }
    }
}

// ---------------------------------------------------------------------------
// Per-window caches, kept in egui's temporary storage (the values are cheap to
// clone: a key plus an `Arc`). Each one is keyed by a hash of everything its
// content depends on, so it is recomputed exactly when an input changes —
// typing, resizing, switching theme or language — and never otherwise.
// ---------------------------------------------------------------------------

/// One logical line of the read-only view.
struct RoLine {
    /// Hanging-indent width (the line's leading whitespace).
    indent_px: f32,
    /// Top of the line, relative to the top of the view.
    y: f32,
    /// Number of visual rows once wrapped (at least 1).
    rows: usize,
    /// Highlighted text, with its wrap width already set.
    ///
    /// The job is cached rather than the laid-out galley on purpose: a galley
    /// points into the font atlas, which egui may rebuild at any time, whereas
    /// laying a visible line out again each frame is a hit in egui's own
    /// galley cache.
    job: egui::text::LayoutJob,
}

/// Layout of the whole read-only view (highlighting + wrapping of every line).
#[derive(Clone)]
struct RoLayout {
    key: u64,
    lines: Arc<Vec<RoLine>>,
    total_h: f32,
}

/// Result of the live checks run on the edited text.
struct EditCheckData {
    /// Byte offset of the well-formedness error, if any.
    error_byte: Option<usize>,
    /// Well-formedness status line: (ok, translated message).
    status: (bool, String),
    /// Translated schema issues (empty unless well-formed).
    schema_lines: Vec<String>,
}

#[derive(Clone)]
struct EditCheck {
    key: u64,
    data: Arc<EditCheckData>,
}

/// Text of the line-number gutter (edit mode).
#[derive(Clone)]
struct GutterText {
    key: u64,
    text: Arc<str>,
}

/// Highlighted (not yet wrapped) job of the edited text.
#[derive(Clone)]
struct EditJob {
    key: u64,
    job: Arc<egui::text::LayoutJob>,
}

fn cache_id() -> egui::Id {
    egui::Id::new("ximod_xml_editor_cache")
}

/// Forget every cache of the editor window.
fn drop_caches(ctx: &egui::Context) {
    ctx.data_mut(|d| {
        d.remove::<RoLayout>(cache_id());
        d.remove::<EditCheck>(cache_id());
        d.remove::<GutterText>(cache_id());
        d.remove::<EditJob>(cache_id());
    });
}

impl XimodApp {
    /// Open the XML editor on the given file, in read-only mode.
    pub fn open_xml_editor(&mut self, target: XmlTarget) {
        self.xml_editor_target = target;
        self.xml_editor_editing = false;
        self.xml_editor_error = None;
        self.xml_editor_gutter.clear();
        self.xml_editor_content = self.serialize_xml(target);
        self.show_xml_editor = true;
    }

    /// Serialize the current model to the XML text for `target`.
    fn serialize_xml(&self, target: XmlTarget) -> String {
        let result = match target {
            XmlTarget::InfoXml => crate::xml::info_xml_to_string(&self.ximod),
            XmlTarget::ModuleConfig => crate::xml::module_config_to_string(&self.ximod),
        };
        result.unwrap_or_else(|e| format!("<!-- {} -->", e))
    }

    /// Re-parse the edited text back into the model (text → model).
    ///
    /// The text is parsed into a fresh model, then only the fields owned by the
    /// edited file are copied into the live model — so editing info.xml never
    /// touches the install steps, and editing ModuleConfig.xml never touches the
    /// pure-metadata fields. Parsing a fresh model also means removing an element
    /// correctly clears the corresponding field.
    pub fn xml_editor_validate(&mut self) {
        let content = self.xml_editor_content.clone();
        match self.xml_editor_target {
            XmlTarget::InfoXml => {
                let mut m = Ximod::default();
                match crate::xml::parse_info_xml(&content, &mut m) {
                    Ok(()) => {
                        self.ximod.name = m.name;
                        self.ximod.author = m.author;
                        self.ximod.version = m.version;
                        self.ximod.url = m.url;
                        self.ximod.description = m.description;
                        self.ximod.category = m.category;
                        self.ximod.game = m.game;
                        self.on_xml_applied();
                    }
                    Err(e) => self.xml_editor_error = Some(format!("{}", e)),
                }
            }
            XmlTarget::ModuleConfig => {
                let mut m = Ximod::default();
                match crate::xml::parse_module_config_xml(&content, &mut m) {
                    Ok(()) => {
                        self.ximod.name = m.name;
                        self.ximod.header_image = m.header_image;
                        self.ximod.required_files = m.required_files;
                        self.ximod.steps = m.steps;
                        self.ximod.conditional_files = m.conditional_files;
                        // Lot F1: the header attributes, mod requirements
                        // and authored orders are part of ModuleConfig too.
                        self.ximod.module_dependencies = m.module_dependencies;
                        self.ximod.title_position = m.title_position;
                        self.ximod.title_colour = m.title_colour;
                        self.ximod.image_show_image = m.image_show_image;
                        self.ximod.image_show_fade = m.image_show_fade;
                        self.ximod.image_height = m.image_height;
                        self.ximod.steps_order = m.steps_order;
                        // What the edited text holds that the model drops.
                        self.import_report = crate::xml::fidelity::scan_module_config(&content);
                        self.on_xml_applied();
                    }
                    Err(e) => self.xml_editor_error = Some(format!("{}", e)),
                }
            }
        }
    }

    /// Run the live checks on the edited text: well-formedness, then — only if
    /// well-formed — schema validation. Parses the whole document: callers
    /// cache the result (see `render_xml_editor`).
    fn run_edit_checks(&self) -> EditCheckData {
        match crate::xml::check_well_formed(&self.xml_editor_content) {
            Some(e) => {
                let mut args = fluent::FluentArgs::new();
                args.set("line", e.line as i64);
                args.set("col", e.column as i64);
                args.set("msg", e.message.clone());
                EditCheckData {
                    error_byte: Some(e.byte),
                    status: (false, self.i18n.t_with_args("xml-editor-error-at", Some(&args))),
                    schema_lines: Vec::new(),
                }
            }
            None => {
                let issues = match self.xml_editor_target {
                    XmlTarget::ModuleConfig => crate::xml::validate::validate_module_config(&self.xml_editor_content),
                    XmlTarget::InfoXml => crate::xml::validate::validate_info(&self.xml_editor_content),
                };
                EditCheckData {
                    error_byte: None,
                    status: (true, self.i18n.t("xml-editor-wellformed")),
                    schema_lines: issues.iter().map(|i| self.translate_schema_issue(i)).collect(),
                }
            }
        }
    }

    fn on_xml_applied(&mut self) {
        // The collections were replaced wholesale: every selection index is
        // now meaningless and must not be reused.
        self.reset_selection();
        self.mark_modified();
        self.xml_editor_editing = false;
        self.xml_editor_error = None;
        self.notify_ok(self.i18n.t("xml-editor-applied"));
        // Reflect the model's canonical formatting back into the editor.
        self.xml_editor_content = self.serialize_xml(self.xml_editor_target);
    }

    /// Paint the read-only view: syntax-highlighted, word-wrapped, with hanging
    /// indentation (wrapped continuation rows align under the start of the line's
    /// content). Not selectable — it's a painted rendering, not a text field.
    ///
    /// Highlighting and wrapping of the whole document are cached (see
    /// [`RoLayout`]); each frame only paints the lines inside the clip rectangle.
    fn paint_readonly_xml(&self, ui: &mut egui::Ui, digits: usize) {
        let font_id = egui::TextStyle::Monospace.resolve(ui.style());
        let row_h = ui.fonts(|f| f.row_height(&font_id));
        let space_w = ui.fonts(|f| f.glyph_width(&font_id, ' '));
        let dark = ui.visuals().dark_mode;
        let text_color = ui.visuals().text_color();
        let weak = ui.visuals().weak_text_color();

        let gutter_w = (digits as f32) * space_w + 8.0;
        let sep_w = 10.0;
        let text_x0 = gutter_w + sep_w;
        let avail_w = ui.available_width();
        let text_avail = (avail_w - text_x0).max(60.0);

        // Everything the layout depends on.
        let key = egui::util::hash((
            self.xml_editor_content.as_str(),
            text_avail.to_bits(),
            dark,
            text_color,
            &font_id,
            ui.ctx().pixels_per_point().to_bits(),
        ));
        let cached = ui
            .ctx()
            .data(|d| d.get_temp::<RoLayout>(cache_id()))
            .filter(|c| c.key == key);
        let layout = match cached {
            Some(c) => c,
            None => {
                // Lay out every logical line (coloured, hanging indent).
                let mut lines: Vec<RoLine> = Vec::new();
                let mut y = 0.0f32;
                for line in self.xml_editor_content.split('\n') {
                    // Leading whitespace (spaces/tabs) → hanging-indent width.
                    let ws_len: usize = line
                        .chars()
                        .take_while(|c| *c == ' ' || *c == '\t')
                        .map(|c| c.len_utf8())
                        .sum();
                    let (ws, rest) = line.split_at(ws_len);
                    let indent_px = if ws.is_empty() {
                        0.0
                    } else {
                        ui.fonts(|f| f.layout_no_wrap(ws.to_string(), font_id.clone(), weak).rect.width())
                    };
                    let mut job =
                        crate::ui::xml_highlight::highlight_xml(rest, font_id.clone(), dark, text_color, None);
                    job.wrap.max_width = (text_avail - indent_px).max(space_w * 6.0);
                    let rows = ui.fonts(|f| f.layout_job(job.clone())).rows.len().max(1);
                    lines.push(RoLine {
                        indent_px,
                        y,
                        rows,
                        job,
                    });
                    y += (rows as f32) * row_h;
                }
                let layout = RoLayout {
                    key,
                    lines: Arc::new(lines),
                    total_h: y,
                };
                ui.ctx().data_mut(|d| d.insert_temp(cache_id(), layout.clone()));
                layout
            }
        };

        let (rect, _resp) =
            ui.allocate_exact_size(egui::vec2(avail_w, layout.total_h.max(row_h)), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Only the lines intersecting the visible area are laid out and painted.
        let clip = ui.clip_rect();
        let lines = &layout.lines;
        let first = lines.partition_point(|l| rect.top() + l.y + (l.rows as f32) * row_h <= clip.top());
        let mut stale = false;
        for (i, l) in lines.iter().enumerate().skip(first) {
            let y = rect.top() + l.y;
            if y >= clip.bottom() {
                break;
            }
            // Line number, right-aligned in the gutter.
            painter.text(
                egui::pos2(rect.left() + gutter_w - 4.0, y),
                egui::Align2::RIGHT_TOP,
                (i + 1).to_string(),
                font_id.clone(),
                weak,
            );
            // Text, painted at its hanging-indent position.
            let galley = ui.fonts(|f| f.layout_job(l.job.clone()));
            stale |= galley.rows.len().max(1) != l.rows;
            painter.galley(egui::pos2(rect.left() + text_x0 + l.indent_px, y), galley, text_color);
        }
        if stale {
            // The fonts changed under us (the cached row counts no longer match
            // the real layout): rebuild on the next frame.
            ui.ctx().data_mut(|d| d.remove::<RoLayout>(cache_id()));
            ui.ctx().request_repaint();
        }
    }

    /// Render the XML editor window.
    pub fn render_xml_editor(&mut self, ctx: &egui::Context) {
        if !self.show_xml_editor {
            drop_caches(ctx);
            self.free_window_closed("ximod_xml_editor");
            return;
        }

        let editing = self.xml_editor_editing;
        let title = format!(
            "{} — {}",
            self.i18n.t("xml-editor-title"),
            self.xml_editor_target.file_name()
        );
        let lbl_edit = self.i18n.t("xml-editor-edit");
        let lbl_apply = self.i18n.t("xml-editor-apply");
        let lbl_revert = self.i18n.t("xml-editor-revert");
        let lbl_readonly = self.i18n.t("xml-editor-readonly");
        let lbl_editing = self.i18n.t("xml-editor-editing");
        let lbl_error = self.i18n.t("xml-editor-error");

        // Live checks (edit mode only; read-only text, being model-generated,
        // is always valid): well-formedness, then schema validation once the
        // document is well-formed. The whole document is re-parsed for both, so
        // the result is cached and recomputed only when the text (or the file
        // shown, or the UI language of the messages) changes.
        let check: Option<Arc<EditCheckData>> = if editing {
            let key = egui::util::hash((
                self.xml_editor_content.as_str(),
                self.xml_editor_target,
                self.i18n.current_locale(),
            ));
            let cached = ctx
                .data(|d| d.get_temp::<EditCheck>(cache_id()))
                .filter(|c| c.key == key);
            Some(match cached {
                Some(c) => c.data,
                None => {
                    let data = Arc::new(self.run_edit_checks());
                    ctx.data_mut(|d| {
                        d.insert_temp(
                            cache_id(),
                            EditCheck {
                                key,
                                data: data.clone(),
                            },
                        )
                    });
                    data
                }
            })
        } else {
            None
        };
        let error_byte = check.as_ref().and_then(|c| c.error_byte);
        let live_status: Option<&(bool, String)> = check.as_ref().map(|c| &c.status);
        let live_ok = check.as_ref().is_some_and(|c| c.error_byte.is_none());
        let schema_lines: &[String] = check.as_ref().map_or(&[], |c| c.schema_lines.as_slice());
        let l_schema_ok = self.i18n.t("xml-editor-schema-ok");
        let l_schema_issues = self.i18n.t("xml-editor-schema-issues");

        // Line count for the gutter (no wrap → visual rows == logical lines).
        let line_count = self.xml_editor_content.matches('\n').count() + 1;
        let digits = line_count.to_string().len().max(2);

        let mut do_edit = false;
        let mut do_apply = false;
        let mut do_revert = false;
        let mut do_close = false;

        let vb = self.free_viewport_builder(ctx, "ximod_xml_editor", title, [760.0, 580.0], false);
        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("ximod_xml_editor"), vb, |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if editing {
                        if ui.button(&lbl_apply).clicked() {
                            do_apply = true;
                        }
                        if ui.button(&lbl_revert).clicked() {
                            do_revert = true;
                        }
                        ui.separator();
                        ui.label(egui::RichText::new(&lbl_editing).italics());
                    } else {
                        if ui.button(&lbl_edit).clicked() {
                            do_edit = true;
                        }
                        ui.separator();
                        ui.label(egui::RichText::new(&lbl_readonly).weak());
                    }
                });

                // Live validation status (edit mode).
                if let Some((ok, msg)) = live_status {
                    let palette = crate::ui::theme::Palette::from_ui(ui);
                    let color = if *ok { palette.success } else { palette.danger };
                    ui.colored_label(color, msg);
                }
                // Schema conformity (shown only when well-formed, in edit mode).
                if live_ok {
                    if schema_lines.is_empty() {
                        ui.colored_label(crate::ui::theme::Palette::from_ui(ui).success, &l_schema_ok);
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(210, 140, 40),
                            format!("{} {}", l_schema_issues, schema_lines.len()),
                        );
                        egui::ScrollArea::vertical()
                            .id_salt("xml_schema_issues")
                            .max_height(90.0)
                            .show(ui, |ui| {
                                for line in schema_lines {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label("•");
                                        ui.label(line);
                                    });
                                }
                            });
                    }
                }
                ui.separator();

                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    if editing {
                        // ---- EDIT MODE: standard editable text field ----
                        ui.horizontal_top(|ui| {
                            // Line-number gutter, built from the previous frame's
                            // real layout: a number on the first visual row of each
                            // logical line, blank on wrapped continuation rows.
                            // The text is rebuilt only when that layout (or the
                            // line count) changes.
                            let gkey = egui::util::hash((&self.xml_editor_gutter, line_count, digits));
                            let cached = ui
                                .ctx()
                                .data(|d| d.get_temp::<GutterText>(cache_id()))
                                .filter(|c| c.key == gkey);
                            let nums: Arc<str> = match cached {
                                Some(c) => c.text,
                                None => {
                                    use std::fmt::Write as _;
                                    let mut nums = String::new();
                                    if self.xml_editor_gutter.is_empty() {
                                        for n in 1..=line_count {
                                            let _ = writeln!(nums, "{:>width$}", n, width = digits);
                                        }
                                    } else {
                                        for entry in &self.xml_editor_gutter {
                                            match entry {
                                                Some(n) => {
                                                    let _ = writeln!(nums, "{:>width$}", n, width = digits);
                                                }
                                                None => {
                                                    for _ in 0..digits {
                                                        nums.push(' ');
                                                    }
                                                    nums.push('\n');
                                                }
                                            }
                                        }
                                    }
                                    let text: Arc<str> = nums.into();
                                    ui.ctx().data_mut(|d| {
                                        d.insert_temp(
                                            cache_id(),
                                            GutterText {
                                                key: gkey,
                                                text: text.clone(),
                                            },
                                        )
                                    });
                                    text
                                }
                            };
                            ui.vertical(|ui| {
                                ui.add_space(2.0); // match the text field's top margin
                                ui.label(
                                    egui::RichText::new(&*nums)
                                        .monospace()
                                        .color(ui.visuals().weak_text_color()),
                                );
                            });
                            ui.separator();

                            // Text field: syntax-highlighted, word-wrapped. The
                            // layouter also rebuilds the gutter model from the real
                            // (wrapped) galley for the next frame.
                            let mut new_gutter: Vec<Option<usize>> = Vec::new();
                            {
                                let ng = &mut new_gutter;
                                let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                                    let font_id = egui::TextStyle::Monospace.resolve(ui.style());
                                    let dark = ui.visuals().dark_mode;
                                    let text_color = ui.visuals().text_color();
                                    // The layouter runs every frame: reuse the
                                    // highlighted job while nothing it depends
                                    // on has changed.
                                    let jkey = egui::util::hash((text, &font_id, dark, text_color, error_byte));
                                    let cached = ui
                                        .ctx()
                                        .data(|d| d.get_temp::<EditJob>(cache_id()))
                                        .filter(|c| c.key == jkey);
                                    let mut job = match cached {
                                        Some(c) => (*c.job).clone(),
                                        None => {
                                            let job = crate::ui::xml_highlight::highlight_xml(
                                                text, font_id, dark, text_color, error_byte,
                                            );
                                            let shared = Arc::new(job.clone());
                                            ui.ctx().data_mut(|d| {
                                                d.insert_temp(cache_id(), EditJob { key: jkey, job: shared })
                                            });
                                            job
                                        }
                                    };
                                    job.wrap.max_width = wrap_width; // wrap on
                                    let galley = ui.fonts(|f| f.layout_job(job));
                                    // One entry per visual row.
                                    let mut g = Vec::with_capacity(galley.rows.len());
                                    let mut line = 1usize;
                                    let mut start_of_line = true;
                                    for row in &galley.rows {
                                        g.push(if start_of_line { Some(line) } else { None });
                                        if row.ends_with_newline {
                                            line += 1;
                                            start_of_line = true;
                                        } else {
                                            start_of_line = false;
                                        }
                                    }
                                    *ng = g;
                                    galley
                                };

                                ui.add(
                                    egui::TextEdit::multiline(&mut self.xml_editor_content)
                                        .code_editor()
                                        .desired_rows(24)
                                        .desired_width(f32::INFINITY)
                                        .interactive(true)
                                        .layouter(&mut layouter),
                                );
                            }
                            self.xml_editor_gutter = new_gutter;
                        });
                    } else {
                        // ---- READ-ONLY MODE: painted, hanging-indent view ----
                        self.paint_readonly_xml(ui, digits);
                    }
                });

                if let Some(err) = self.xml_editor_error.clone() {
                    ui.separator();
                    ui.colored_label(egui::Color32::from_rgb(220, 80, 80), format!("{} {}", lbl_error, err));
                }
            });

            crate::ui::widgets::free_window::record_win_geom(&mut self.config, ctx, "ximod_xml_editor");
            // Native window close (X button) or Escape.
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        if do_edit {
            self.xml_editor_editing = true;
            self.xml_editor_error = None;
            // Start editing from the current model state (in case the graphical
            // tabs were changed while the editor was open read-only).
            self.xml_editor_content = self.serialize_xml(self.xml_editor_target);
        }
        if do_revert {
            self.xml_editor_editing = false;
            self.xml_editor_error = None;
            self.xml_editor_content = self.serialize_xml(self.xml_editor_target);
        }
        if do_apply {
            self.xml_editor_validate();
        }
        if do_close {
            self.show_xml_editor = false;
            self.xml_editor_editing = false;
            self.xml_editor_error = None;
            self.free_window_closed("ximod_xml_editor");
        }
    }
}
