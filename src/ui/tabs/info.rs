//! "Info" tab: mod metadata and header image.

use crate::models::*;
use crate::ui::components::*;
use crate::ui::main_window::{PickWhat, XimodApp};
use eframe::egui::{self, RichText};

impl XimodApp {
    /// Optimize the header image in place (downscale/convert to sane bounds).
    pub(crate) fn optimize_header_image(&mut self) {
        let (Some(root), Some(rel)) = (
            self.root_directory.clone(),
            self.ximod.header_image.clone().filter(|s| !s.trim().is_empty()),
        ) else {
            self.notify_err(self.i18n.t("msg-no-header-image"));
            return;
        };
        let abs = root.join(rel.replace('\\', "/"));
        if !abs.is_file() {
            self.notify_err(self.i18n.t("msg-no-header-image"));
            return;
        }
        let constraints = crate::media::ImageConstraints::default();
        match crate::media::process_image(&abs, &abs, &constraints) {
            Ok(true) => {
                self.notify_ok(self.i18n.t("msg-image-optimized"));
                self.images_to_forget.push(abs);
            }
            Ok(false) => self.notify_info(self.i18n.t("msg-image-ok")),
            Err(e) => self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string())),
        }
    }

    pub(crate) fn render_info_tab(&mut self, ui: &mut egui::Ui) {
        let label_workspace = self.i18n.t("label-workspace");
        let label_root = self.i18n.t("label-root-dir");
        let label_name = self.i18n.t("label-mod-name");
        let label_author = self.i18n.t("label-author");
        let label_version = self.i18n.t("label-version");
        let label_game = self.i18n.t("label-game-name");
        let label_category = self.i18n.t("label-category");
        let label_url = self.i18n.t("label-url");
        let label_header = self.i18n.t("label-header-image");
        let label_desc = self.i18n.t("label-description");
        let btn_browse = self.i18n.t("btn-browse");
        let btn_clear = self.i18n.t("btn-clear");
        let label_module_deps = self.i18n.t("info-module-deps");
        let hint_module_deps = self.i18n.t("info-module-deps-hint");
        let label_operator = self.i18n.t("label-operator");
        let hint_operator = self.i18n.t("hint-operator");
        let label_advanced = self.i18n.t("info-header-advanced");
        let label_title_position = self.i18n.t("info-title-position");
        let label_title_colour = self.i18n.t("info-title-colour");
        let label_image_show = self.i18n.t("info-image-show");
        let label_image_fade = self.i18n.t("info-image-fade");
        let label_image_height = self.i18n.t("info-image-height");
        let label_default = self.i18n.t("info-attr-default");
        let hint_colour = self.i18n.t("info-title-colour-hint");
        let placeholder = self.i18n.t("placeholder-select-dir");
        let placeholder_game = self.i18n.t("placeholder-select-game");

        // Pre-compute dynamic game & category data (avoids borrow conflicts inside
        // the ComboBox closures). Both come from the external Categories.json.
        let game_list = self.games.game_list();
        let selected_game_id = self.ximod.game.clone();
        let current_game_name = self.games.name_for(&selected_game_id).unwrap_or("").to_string();
        let game_categories: Vec<String> = self.games.categories_for(&selected_game_id).to_vec();
        // Nexus Mods slug of the selected game (empty when unknown) → direct link.
        let nexus_slug = self.games.nexus_slug_for(&selected_game_id).unwrap_or("").to_string();
        let btn_nexus = self.i18n.t("btn-nexus");
        let nexus_hint = self.i18n.t("nexus-open-hint");

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Workspace section header
            ui.label(RichText::new(&label_workspace).strong());
            ui.add_space(2.0);

            // Root directory
            ui.horizontal(|ui| {
                ui.label(&label_root);
                let root_text = self
                    .root_directory
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| placeholder.clone());
                ui.label(&root_text);
                if ui.button(&btn_browse).clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_folder()
                {
                    // Pointing a pristine project at a folder that already
                    // holds a FOMOD means "open it", not "overwrite it later".
                    let has_fomod = path.join("fomod").join("ModuleConfig.xml").is_file()
                        || path.join("fomod").join("info.xml").is_file();
                    if has_fomod && self.active_is_pristine() {
                        self.load_project(path);
                    } else {
                        self.root_directory = Some(path);
                        self.mark_modified();
                    }
                }
            });

            ui.separator();

            egui::Grid::new("info_grid")
                .num_columns(2)
                .spacing([40.0, 8.0])
                .show(ui, |ui| {
                    ui.label(&label_name);
                    if ui.text_edit_singleline(&mut self.ximod.name).changed() {
                        self.mark_modified();
                    }
                    ui.end_row();

                    ui.label(&label_author);
                    if ui.text_edit_singleline(&mut self.ximod.author).changed() {
                        self.mark_modified();
                    }
                    ui.end_row();

                    ui.label(&label_version);
                    if ui.text_edit_singleline(&mut self.ximod.version).changed() {
                        self.mark_modified();
                    }
                    ui.end_row();

                    // Game selector (dynamic list from Categories.json)
                    ui.label(&label_game);
                    let game_text = if current_game_name.is_empty() {
                        placeholder_game.clone()
                    } else {
                        current_game_name.clone()
                    };
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("game_combo")
                            .selected_text(&game_text)
                            .show_ui(ui, |ui| {
                                for (game_id, game_name) in &game_list {
                                    let is_selected = self.ximod.game == *game_id;
                                    if ui.selectable_label(is_selected, game_name).clicked() {
                                        self.ximod.game = game_id.clone();
                                        self.mark_modified();
                                    }
                                }
                            });
                        // Direct link to the game's Nexus Mods page (uses the slug).
                        if ui
                            .add_enabled(!nexus_slug.is_empty(), egui::Button::new(&btn_nexus))
                            .on_hover_text(&nexus_hint)
                            .clicked()
                        {
                            crate::fonts::open_url(&format!("https://www.nexusmods.com/{}", nexus_slug));
                        }
                    });
                    ui.end_row();

                    ui.label(&label_category);
                    let current_cat = self.ximod.category.as_str().to_string();
                    egui::ComboBox::from_id_salt("category_combo")
                        .selected_text(&current_cat)
                        .show_ui(ui, |ui| {
                            if game_categories.is_empty() {
                                // Fallback to built-in categories when no game is
                                // selected (or its list is unavailable).
                                for cat in ModCategory::predefined() {
                                    let is_selected = self.ximod.category == *cat;
                                    if ui.selectable_label(is_selected, cat.as_str()).clicked() {
                                        self.ximod.category = cat.clone();
                                        self.mark_modified();
                                    }
                                }
                            } else {
                                for cat in &game_categories {
                                    let is_selected = self.ximod.category.as_str() == cat.as_str();
                                    if ui.selectable_label(is_selected, cat).clicked() {
                                        self.ximod.category = ModCategory::from_str(cat);
                                        self.mark_modified();
                                    }
                                }
                            }
                        });
                    ui.end_row();

                    ui.label(&label_url);
                    if ui.text_edit_singleline(&mut self.ximod.url).changed() {
                        self.mark_modified();
                    }
                    ui.end_row();
                });

            ui.separator();

            // Header image
            ui.horizontal(|ui| {
                ui.label(&label_header);
                let img_text = self.ximod.header_image.clone().unwrap_or_default();
                ui.label(&img_text);

                if ui.button(&btn_browse).clicked()
                    && let Some(rel) = self.pick_relative(PickWhat::Image).pop()
                {
                    self.ximod.header_image = Some(rel);
                    self.mark_modified();
                }

                if ui.button(&btn_clear).clicked() {
                    self.ximod.header_image = None;
                    self.mark_modified();
                }

                // Optimize header image (V2): downscale/convert to sane bounds.
                let has_img = self
                    .ximod
                    .header_image
                    .as_ref()
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false)
                    && self.root_directory.is_some();
                if ui
                    .add_enabled(has_img, egui::Button::new(self.i18n.t("btn-optimize-image")))
                    .clicked()
                {
                    self.optimize_header_image();
                }
            });

            // Header image preview (rendered from disk relative to the root dir)
            {
                let abs_path = self.ximod.header_image.as_ref().and_then(|rel| {
                    self.root_directory
                        .as_ref()
                        .map(|root| root.join(rel.replace('\\', "/")))
                });
                let fallback = self.i18n.t("image-no-image");
                ImageDisplay::new(220.0, 110.0)
                    .with_fallback(fallback)
                    .show(ui, abs_path.as_deref());
            }

            // Advanced header attributes (title position / colour, image
            // display flags): rarely used, round-tripped verbatim.
            egui::CollapsingHeader::new(&label_advanced)
                .id_salt("info_header_advanced")
                .default_open(self.has_advanced_header())
                .show(ui, |ui| {
                    self.render_advanced_header(
                        ui,
                        &AdvancedHeaderLabels {
                            title_position: &label_title_position,
                            title_colour: &label_title_colour,
                            image_show: &label_image_show,
                            image_fade: &label_image_fade,
                            image_height: &label_image_height,
                            default: &label_default,
                            colour_hint: &hint_colour,
                        },
                    );
                });

            ui.separator();

            // Description
            ui.label(&label_desc);
            if ui
                .add_sized(
                    [ui.available_width(), 150.0],
                    egui::TextEdit::multiline(&mut self.ximod.description),
                )
                .changed()
            {
                self.mark_modified();
            }

            ui.separator();

            // Mod requirements (<moduleDependencies>): what the whole mod
            // needs before the installer runs. Empty ⇒ omitted on save.
            let has_deps = self.ximod.has_module_dependencies();
            egui::CollapsingHeader::new(&label_module_deps)
                .id_salt("info_module_deps")
                .default_open(has_deps)
                .show(ui, |ui| {
                    ui.label(RichText::new(&hint_module_deps).small().weak());
                    let current_op = self
                        .ximod
                        .module_dependencies
                        .as_ref()
                        .map(|m| m.operator)
                        .unwrap_or_default();
                    ui.horizontal(|ui| {
                        ui.label(&label_operator);
                        egui::ComboBox::from_id_salt("mdep_operator")
                            .selected_text(crate::ui::labels::operator(&self.i18n, current_op))
                            .show_ui(ui, |ui| {
                                for op in LogicalOperator::variants() {
                                    let label = crate::ui::labels::operator(&self.i18n, *op);
                                    if ui.selectable_label(current_op == *op, label).clicked() && current_op != *op {
                                        self.ximod
                                            .module_dependencies
                                            .get_or_insert_with(Default::default)
                                            .operator = *op;
                                        self.mark_modified();
                                    }
                                }
                            })
                            .response
                            .on_hover_text(&hint_operator);
                    });
                    self.render_dependency_editor(
                        ui,
                        crate::ui::widgets::dependency_editor::DepTarget::Module,
                        crate::ui::widgets::dependency_editor::DepEditorIds {
                            list: "mdep_deps_list",
                            type_combo: "mdep_dep_type",
                            ac_name: "ac_mdep_name",
                            ac_value: "ac_mdep_value",
                        },
                    );
                });
        });
    }

    /// Whether any of the rarely used header attributes is set (the
    /// "Advanced header" section then opens by default).
    fn has_advanced_header(&self) -> bool {
        self.ximod.title_position.is_some()
            || self.ximod.title_colour.is_some()
            || self.ximod.image_show_image.is_some()
            || self.ximod.image_show_fade.is_some()
            || self.ximod.image_height.is_some()
    }

    /// The "Advanced header" grid: `moduleName/@position`, `@colour` and
    /// the `moduleImage` display attributes. Each value is optional; the
    /// "default" choice removes the attribute on save.
    fn render_advanced_header(&mut self, ui: &mut egui::Ui, labels: &AdvancedHeaderLabels<'_>) {
        const POSITIONS: [&str; 3] = ["Left", "Right", "RightOfImage"];
        egui::Grid::new("info_advanced_grid")
            .num_columns(2)
            .spacing([40.0, 8.0])
            .show(ui, |ui| {
                // Title position.
                ui.label(labels.title_position);
                let current = self.ximod.title_position.clone();
                egui::ComboBox::from_id_salt("info_title_position")
                    .selected_text(current.as_deref().unwrap_or(labels.default))
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(current.is_none(), labels.default).clicked() && current.is_some() {
                            self.ximod.title_position = None;
                            self.mark_modified();
                        }
                        for pos in POSITIONS {
                            if ui.selectable_label(current.as_deref() == Some(pos), pos).clicked()
                                && current.as_deref() != Some(pos)
                            {
                                self.ximod.title_position = Some(pos.to_string());
                                self.mark_modified();
                            }
                        }
                    });
                ui.end_row();

                // Title colour (hex RRGGBB; soft validation).
                ui.label(labels.title_colour);
                ui.horizontal(|ui| {
                    let mut colour = self.ximod.title_colour.clone().unwrap_or_default();
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut colour)
                            .desired_width(90.0)
                            .hint_text("RRGGBB"),
                    );
                    if resp.changed() {
                        self.ximod.title_colour = if colour.trim().is_empty() { None } else { Some(colour) };
                        self.mark_modified();
                    }
                    if let Some(c) = self.ximod.title_colour.as_deref()
                        && !is_hex_colour(c)
                    {
                        ui.label(
                            RichText::new(labels.colour_hint)
                                .small()
                                .color(ui.visuals().warn_fg_color),
                        );
                    }
                });
                ui.end_row();

                // Image flags.
                ui.label(labels.image_show);
                if tri_checkbox(ui, &mut self.ximod.image_show_image, labels.default) {
                    self.mark_modified();
                }
                ui.end_row();

                ui.label(labels.image_fade);
                if tri_checkbox(ui, &mut self.ximod.image_show_fade, labels.default) {
                    self.mark_modified();
                }
                ui.end_row();

                // Image height.
                ui.label(labels.image_height);
                ui.horizontal(|ui| {
                    let mut set = self.ximod.image_height.is_some();
                    if ui.checkbox(&mut set, "").changed() {
                        self.ximod.image_height = if set { Some(100) } else { None };
                        self.mark_modified();
                    }
                    if let Some(h) = self.ximod.image_height.as_mut() {
                        if ui.add(egui::DragValue::new(h).range(0..=4096)).changed() {
                            self.mark_modified();
                        }
                    } else {
                        ui.label(RichText::new(labels.default).weak());
                    }
                });
                ui.end_row();
            });
    }
}

/// Translated texts of the "Advanced header" section.
struct AdvancedHeaderLabels<'a> {
    title_position: &'a str,
    title_colour: &'a str,
    image_show: &'a str,
    image_fade: &'a str,
    image_height: &'a str,
    default: &'a str,
    colour_hint: &'a str,
}

/// A `RRGGBB` hex colour (six hex digits, optional leading `#`).
fn is_hex_colour(s: &str) -> bool {
    let s = s.trim().trim_start_matches('#');
    s.len() == 6 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// An optional boolean attribute as "default" (unset) + a checkbox for the
/// explicit value. Returns whether the value changed.
fn tri_checkbox(ui: &mut egui::Ui, value: &mut Option<bool>, default_label: &str) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let mut set = value.is_some();
        if ui.checkbox(&mut set, "").changed() {
            *value = if set { Some(true) } else { None };
            changed = true;
        }
        match value {
            Some(v) => {
                if ui
                    .add(egui::Checkbox::new(v, if *v { "true" } else { "false" }))
                    .changed()
                {
                    changed = true;
                }
            }
            None => {
                ui.label(RichText::new(default_label).weak());
            }
        }
    });
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_colour_validation() {
        assert!(is_hex_colour("FFAA00"));
        assert!(is_hex_colour("#ffaa00"));
        assert!(!is_hex_colour("FFAA0"));
        assert!(!is_hex_colour("GGGGGG"));
        assert!(!is_hex_colour(""));
    }

    /// The Info tab lays out headless with the advanced attributes set and
    /// a plain render never dirties the project.
    #[test]
    fn info_tab_renders_headless_with_advanced_header() {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.ximod.title_position = Some("Right".into());
        app.ximod.title_colour = Some("12AB".into());
        app.ximod.image_show_fade = Some(false);
        app.ximod.image_height = Some(200);
        app.ximod.module_dependencies = Some(DependencyGroup::from_leaves(
            LogicalOperator::Or,
            vec![Dependency::new_file("Skyrim.esm", "Active")],
        ));
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                app.render_info_tab(ui);
            });
        });
        assert!(!app.project_modified);
        assert_eq!(app.ximod.image_height, Some(200));
    }
}
