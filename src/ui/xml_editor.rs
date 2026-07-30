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

/// Which file the XML editor is currently showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
                        self.on_xml_applied();
                    }
                    Err(e) => self.xml_editor_error = Some(format!("{}", e)),
                }
            }
        }
    }

    fn on_xml_applied(&mut self) {
        self.project_modified = true;
        self.xml_editor_editing = false;
        self.xml_editor_error = None;
        self.status_message = self.i18n.t("xml-editor-applied");
        // Reflect the model's canonical formatting back into the editor.
        self.xml_editor_content = self.serialize_xml(self.xml_editor_target);
    }

    /// Paint the read-only view: syntax-highlighted, word-wrapped, with hanging
    /// indentation (wrapped continuation rows align under the start of the line's
    /// content). Not selectable — it's a painted rendering, not a text field.
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

        // First pass: lay out every logical line (coloured, hanging indent).
        struct Block {
            num: usize,
            indent_px: f32,
            galley: std::sync::Arc<egui::text::Galley>,
        }
        let mut blocks: Vec<Block> = Vec::new();
        for (i, line) in self.xml_editor_content.split('\n').enumerate() {
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
            let mut job = crate::ui::xml_highlight::highlight_xml(
                rest, font_id.clone(), dark, text_color, None,
            );
            job.wrap.max_width = (text_avail - indent_px).max(space_w * 6.0);
            let galley = ui.fonts(|f| f.layout_job(job));
            blocks.push(Block { num: i + 1, indent_px, galley });
        }

        let total_h: f32 = blocks
            .iter()
            .map(|b| (b.galley.rows.len().max(1) as f32) * row_h)
            .sum();

        let (rect, _resp) =
            ui.allocate_exact_size(egui::vec2(avail_w, total_h.max(row_h)), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        let mut y = rect.top();
        for b in &blocks {
            // Line number, right-aligned in the gutter.
            painter.text(
                egui::pos2(rect.left() + gutter_w - 4.0, y),
                egui::Align2::RIGHT_TOP,
                b.num.to_string(),
                font_id.clone(),
                weak,
            );
            // Text, painted at its hanging-indent position.
            painter.galley(
                egui::pos2(rect.left() + text_x0 + b.indent_px, y),
                b.galley.clone(),
                text_color,
            );
            y += (b.galley.rows.len().max(1) as f32) * row_h;
        }
    }

    /// Render the XML editor window.
    pub fn render_xml_editor(&mut self, ctx: &egui::Context) {
        if !self.show_xml_editor {
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

        // Live well-formedness check (edit mode only; read-only text, being
        // model-generated, is always valid).
        let live = if editing {
            crate::xml::check_well_formed(&self.xml_editor_content)
        } else {
            None
        };
        let error_byte = live.as_ref().map(|e| e.byte);
        let live_status: Option<(bool, String)> = if editing {
            Some(match &live {
                None => (true, self.i18n.t("xml-editor-wellformed")),
                Some(e) => {
                    let mut args = fluent::FluentArgs::new();
                    args.set("line", e.line as i64);
                    args.set("col", e.column as i64);
                    args.set("msg", e.message.clone());
                    (
                        false,
                        self.i18n.t_with_args("xml-editor-error-at", Some(&args)),
                    )
                }
            })
        } else {
            None
        };

        // Schema validation (edit mode, once the document is well-formed).
        let live_ok = editing && live.is_none();
        let schema_lines: Vec<String> = if live_ok {
            let issues = match self.xml_editor_target {
                XmlTarget::ModuleConfig => {
                    crate::xml::validate::validate_module_config(&self.xml_editor_content)
                }
                XmlTarget::InfoXml => {
                    crate::xml::validate::validate_info(&self.xml_editor_content)
                }
            };
            issues.iter().map(|i| self.translate_schema_issue(i)).collect()
        } else {
            Vec::new()
        };
        let l_schema_ok = self.i18n.t("xml-editor-schema-ok");
        let l_schema_issues = self.i18n.t("xml-editor-schema-issues");

        // Line count for the gutter (no wrap → visual rows == logical lines).
        let line_count = self.xml_editor_content.matches('\n').count() + 1;
        let digits = line_count.to_string().len().max(2);

        let mut do_edit = false;
        let mut do_apply = false;
        let mut do_revert = false;
        let mut do_close = false;

        let vb = self.free_viewport_builder(ctx, "ximod_xml_editor", title, [760.0, 580.0]);
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("ximod_xml_editor"),
            vb,
            |ctx, _class| {
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
                if let Some((ok, msg)) = &live_status {
                    let color = if *ok {
                        egui::Color32::from_rgb(60, 160, 60)
                    } else {
                        egui::Color32::from_rgb(220, 80, 80)
                    };
                    ui.colored_label(color, msg);
                }
                // Schema conformity (shown only when well-formed, in edit mode).
                if live_ok {
                    if schema_lines.is_empty() {
                        ui.colored_label(egui::Color32::from_rgb(60, 160, 60), &l_schema_ok);
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(210, 140, 40),
                            format!("{} {}", l_schema_issues, schema_lines.len()),
                        );
                        egui::ScrollArea::vertical()
                            .id_salt("xml_schema_issues")
                            .max_height(90.0)
                            .show(ui, |ui| {
                                for line in &schema_lines {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label("•");
                                        ui.label(line);
                                    });
                                }
                            });
                    }
                }
                ui.separator();

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if editing {
                            // ---- EDIT MODE: standard editable text field ----
                            ui.horizontal_top(|ui| {
                            // Line-number gutter, built from the previous frame's
                            // real layout: a number on the first visual row of each
                            // logical line, blank on wrapped continuation rows.
                            let mut nums = String::new();
                            if self.xml_editor_gutter.is_empty() {
                                for n in 1..=line_count {
                                    nums.push_str(&format!("{:>width$}\n", n, width = digits));
                                }
                            } else {
                                for entry in &self.xml_editor_gutter {
                                    match entry {
                                        Some(n) => {
                                            nums.push_str(&format!("{:>width$}\n", n, width = digits))
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
                            ui.vertical(|ui| {
                                ui.add_space(2.0); // match the text field's top margin
                                ui.label(
                                    egui::RichText::new(nums)
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
                                let mut layouter =
                                    |ui: &egui::Ui, text: &str, wrap_width: f32| {
                                        let font_id =
                                            egui::TextStyle::Monospace.resolve(ui.style());
                                        let mut job = crate::ui::xml_highlight::highlight_xml(
                                            text,
                                            font_id,
                                            ui.visuals().dark_mode,
                                            ui.visuals().text_color(),
                                            error_byte,
                                        );
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
                    ui.colored_label(
                        egui::Color32::from_rgb(220, 80, 80),
                        format!("{} {}", lbl_error, err),
                    );
                }
                });

                crate::ui::main_window::record_win_geom(&mut self.config, ctx, "ximod_xml_editor");
                // Native window close (X button) or Escape.
                if ctx.input(|i| i.viewport().close_requested())
                    || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
                {
                    do_close = true;
                }
            },
        );

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
