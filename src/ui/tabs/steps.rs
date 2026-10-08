//! "Steps" tab inspectors: page destination, step visibility, option details.

use crate::models::*;
use crate::ui::components::*;
use crate::ui::main_window::{PickWhat, XimodApp};
use crate::ui::widgets::dependency_editor::{DepEditorIds, DepTarget};
use eframe::egui;

impl XimodApp {
    /// Optimize a plugin's image in place (downscale/convert to sane bounds),
    /// mirroring the header-image action but for the option's own image.
    pub(crate) fn optimize_plugin_image(&mut self, si: usize, gi: usize, pi: usize) {
        let rel = self
            .ximod
            .steps
            .get(si)
            .and_then(|s| s.plugin_groups.get(gi))
            .and_then(|g| g.plugins.get(pi))
            .and_then(|p| p.image_path.clone())
            .filter(|s| !s.trim().is_empty());
        let (Some(root), Some(rel)) = (self.root_directory.clone(), rel) else {
            self.notify_err(self.i18n.t("msg-img-none"));
            return;
        };
        let abs = root.join(rel.replace('\\', "/"));
        if !abs.is_file() {
            self.notify_err(self.i18n.t("msg-img-none"));
            return;
        }
        let constraints = crate::media::ImageConstraints::default();
        match crate::media::process_image(&abs, &abs, &constraints) {
            Ok(true) => {
                self.notify_ok(self.i18n.t("msg-img-optimized"));
                self.images_to_forget.push(abs);
            }
            Ok(false) => self.notify_info(self.i18n.t("msg-img-ok")),
            Err(e) => self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string())),
        }
    }

    /// Set `dest` as the destination of every file of every plugin in one
    /// group, in a single action. Returns the number of files updated.
    ///
    /// This backs the "same destination for a whole group" control: instead of
    /// editing the destination on each option's Files table, the author assigns
    /// one install destination to the entire group at once.
    pub(crate) fn apply_dest_to_group(&mut self, step_idx: usize, group_idx: usize, dest: &str) -> usize {
        self.ximod
            .steps
            .get_mut(step_idx)
            .and_then(|s| s.plugin_groups.get_mut(group_idx))
            .map(|g| g.set_all_destinations(dest))
            .unwrap_or(0)
    }

    /// Set `dest` as the destination of every file of every plugin on one
    /// step/page (all its groups), in a single action. Returns the number of
    /// files updated.
    pub(crate) fn apply_dest_to_step(&mut self, step_idx: usize, dest: &str) -> usize {
        self.ximod
            .steps
            .get_mut(step_idx)
            .map(|s| s.set_all_destinations(dest))
            .unwrap_or(0)
    }

    /// Panel: assign one install destination to every plugin on the current
    /// page/step at once.
    pub(crate) fn render_page_destination(&mut self, ui: &mut egui::Ui, step_idx: usize) {
        let hint = self.i18n.t("page-dest-hint");
        let btn_apply = self.i18n.t("btn-apply-page-dest");
        let label_dest = self.i18n.t("label-destination");
        let nofiles = self.i18n.t("bulk-dest-nofiles");

        let file_count: usize = self
            .ximod
            .steps
            .get(step_idx)
            .map(|s| {
                s.plugin_groups
                    .iter()
                    .flat_map(|g| &g.plugins)
                    .map(|p| p.files.len())
                    .sum()
            })
            .unwrap_or(0);

        ui.label(egui::RichText::new(&hint).small().color(ui.visuals().weak_text_color()));
        ui.horizontal(|ui| {
            ui.label(&label_dest);
            ui.text_edit_singleline(&mut self.page_dest_buf);
        });
        if ui.add_enabled(file_count > 0, egui::Button::new(&btn_apply)).clicked() {
            let dest = self.page_dest_buf.trim().to_string();
            let n = self.apply_dest_to_step(step_idx, &dest);
            self.mark_modified();
            self.notify_ok(self.i18n.t_num("status-dest-applied", n as i64));
        }
        if file_count == 0 {
            ui.label(
                egui::RichText::new(&nofiles)
                    .small()
                    .color(ui.visuals().weak_text_color()),
            );
        }
    }

    /// "Visibility conditions" panel for a step: edits the step's `<visible>`
    /// block (`visibility`, a `DependencyGroup`). When it holds
    /// conditions, the step is shown in the wizard only when they are met (for
    /// example only if a given `.esp` is Active, or a flag was set earlier).
    pub(crate) fn render_step_visibility(&mut self, ui: &mut egui::Ui, step_idx: usize) {
        let label_operator = self.i18n.t("label-operator");
        let hint_operator = self.i18n.t("hint-operator");

        let current_op = self.ximod.steps[step_idx].visibility.operator;
        ui.horizontal(|ui| {
            ui.label(&label_operator);
            egui::ComboBox::from_id_salt("vis_operator")
                .selected_text(crate::ui::labels::operator(&self.i18n, current_op))
                .show_ui(ui, |ui| {
                    for op in LogicalOperator::variants() {
                        let label = crate::ui::labels::operator(&self.i18n, *op);
                        if ui.selectable_label(current_op == *op, label).clicked() {
                            self.ximod.steps[step_idx].visibility.operator = *op;
                            self.mark_modified();
                        }
                    }
                })
                .response
                .on_hover_text(&hint_operator);
        });

        self.render_dependency_editor(
            ui,
            DepTarget::StepVisibility { step: step_idx },
            DepEditorIds {
                list: "vis_deps_list",
                type_combo: "vis_dep_type",
                ac_name: "ac_vdep_name",
                ac_value: "ac_vdep_value",
            },
        );
    }

    pub(crate) fn render_plugin_details(&mut self, ui: &mut egui::Ui) {
        let step_idx = match self.selection.step {
            Some(i) => i,
            None => return,
        };
        let group_idx = match self.selection.group {
            Some(i) => i,
            None => {
                ui.label(self.i18n.t("msg-select-group-first"));
                return;
            }
        };
        let plugin_idx = match self.selection.plugin {
            Some(i) => i,
            None => {
                ui.label(self.i18n.t("msg-select-plugin-edit"));
                return;
            }
        };

        if step_idx >= self.ximod.steps.len() {
            return;
        }
        if group_idx >= self.ximod.steps[step_idx].plugin_groups.len() {
            return;
        }
        if plugin_idx >= self.ximod.steps[step_idx].plugin_groups[group_idx].plugins.len() {
            return;
        }

        let label_name = self.i18n.t("label-plugin-name");
        let label_desc = self.i18n.t("label-plugin-desc");
        let label_type = self.i18n.t("label-plugin-type");
        let label_image = self.i18n.t("label-plugin-image");
        let btn_browse = self.i18n.t("btn-browse");
        let btn_clear = self.i18n.t("btn-clear");
        let label_flags = self.i18n.t("section-flags");
        let label_files = self.i18n.t("section-files");
        let hint_default_type = self.i18n.t("hint-default-type");
        let hint_flags = self.i18n.t("hint-flags");
        let hint_files = self.i18n.t("hint-files");
        let label_deps = self.i18n.t("label-plugin-dependencies");
        let hint_deps = self.i18n.t("hint-plugin-dependencies");

        egui::ScrollArea::vertical().show(ui, |ui| {
            section_header(ui, &label_name);

            let mut plugin_name = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                .name
                .clone();
            if ui.text_edit_singleline(&mut plugin_name).changed() {
                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].name = plugin_name;
                self.mark_modified();
            }

            ui.label(&label_desc);
            let mut desc = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                .description
                .clone();
            if ui
                .add_sized([ui.available_width(), 80.0], egui::TextEdit::multiline(&mut desc))
                .changed()
            {
                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].description = desc;
                self.mark_modified();
            }

            // Plugin image: label on its own line, then the path + Browse/Clear,
            // then the preview — kept directly under the description so the image
            // assigned to the option is clearly visible in this pane.
            ui.add_space(6.0);
            ui.label(&label_image);
            let img_text = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                .image_path
                .clone()
                .unwrap_or_default();
            ui.horizontal(|ui| {
                if ui.button(&btn_browse).clicked()
                    && let Some(rel) = self.pick_relative(PickWhat::Image).pop()
                {
                    self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].image_path = Some(rel);
                    self.mark_modified();
                }
                if ui.button(&btn_clear).clicked() {
                    self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].image_path = None;
                    self.mark_modified();
                }
                if !img_text.is_empty() {
                    ui.label(&img_text);
                }
            });

            // Plugin image preview (rendered from disk relative to the root dir)
            {
                let abs_path = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                    .image_path
                    .as_ref()
                    .and_then(|rel| {
                        self.root_directory
                            .as_ref()
                            .map(|root| root.join(rel.replace('\\', "/")))
                    });
                let fallback = self.i18n.t("image-no-image");
                ImageDisplay::new(240.0, 135.0)
                    .with_fallback(fallback)
                    .show(ui, abs_path.as_deref());
            }

            // Optimize the plugin image (same action as the header image).
            let has_plugin_img = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                .image_path
                .as_ref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
                && self.root_directory.is_some();
            if ui
                .add_enabled(has_plugin_img, egui::Button::new(self.i18n.t("btn-optimize-image")))
                .clicked()
            {
                self.optimize_plugin_image(step_idx, group_idx, plugin_idx);
            }

            // Default option type.
            let current_type = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].default_type;
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(&label_type);
                egui::ComboBox::from_id_salt("plugin_type_combo")
                    .selected_text(crate::ui::labels::plugin_type(&self.i18n, current_type))
                    .show_ui(ui, |ui| {
                        for pt in PluginType::variants() {
                            let label = crate::ui::labels::plugin_type(&self.i18n, *pt);
                            let hint = crate::ui::labels::plugin_type_hint(&self.i18n, *pt);
                            if ui
                                .selectable_label(current_type == *pt, label)
                                .on_hover_text(hint)
                                .clicked()
                            {
                                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].default_type =
                                    *pt;
                                self.mark_modified();
                            }
                        }
                    })
                    .response
                    .on_hover_text(&hint_default_type);
            });

            ui.add_space(8.0);
            section_header_hint(ui, &label_flags, &hint_flags);
            self.render_condition_flags(ui, step_idx, group_idx, plugin_idx);

            ui.add_space(8.0);
            section_header_hint(ui, &label_deps, &hint_deps);
            self.render_plugin_dependencies(ui, step_idx, group_idx, plugin_idx);

            ui.add_space(8.0);
            section_header_hint(ui, &label_files, &hint_files);
            self.render_option_size(ui, (step_idx, group_idx, plugin_idx));
            self.render_plugin_files(ui, step_idx, group_idx, plugin_idx);
        });
    }

    /// Install size of an option as measured by the last validation
    /// (`size-unknown` before one ran), with the missing-source count in the
    /// warning colour.
    pub(crate) fn render_option_size(&self, ui: &mut egui::Ui, key: (usize, usize, usize)) {
        match self.option_size_of(key) {
            Some(info) => {
                let mut args = fluent::FluentArgs::new();
                args.set("size", crate::models::simulate::format_size(info.bytes));
                args.set("num", info.files as i64);
                let text = self.i18n.t_with_args("size-option", Some(&args));
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(text).small().weak());
                    if info.missing > 0 {
                        let warn = crate::ui::theme::Palette::from_ui(ui).warning;
                        ui.label(
                            egui::RichText::new(self.i18n.t_num("size-missing", info.missing as i64))
                                .small()
                                .color(warn),
                        );
                    }
                });
            }
            None => {
                ui.label(egui::RichText::new(self.i18n.t("size-unknown")).small().weak());
            }
        }
    }

    pub(crate) fn render_condition_flags(
        &mut self,
        ui: &mut egui::Ui,
        step_idx: usize,
        group_idx: usize,
        plugin_idx: usize,
    ) {
        let btn_add = self.i18n.t("btn-add-flag");
        let btn_remove = self.i18n.t("btn-remove-flag");

        let flags: Vec<(String, String)> = self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
            .condition_flags
            .iter()
            .map(|f| (f.name.clone(), f.value.clone()))
            .collect();

        egui::ScrollArea::vertical()
            .id_salt("flags_list")
            .max_height(80.0)
            .show(ui, |ui| {
                for (idx, (name, value)) in flags.iter().enumerate() {
                    let selected = self.current_flag_index == Some(idx);
                    let text = format!("{} = {}", name, value);
                    if ui.selectable_label(selected, &text).clicked() {
                        self.current_flag_index = Some(idx);
                        self.temp_flag_name = name.clone();
                        self.temp_flag_value = value.clone();
                    }
                }
            });

        let ac = self.autocomplete();
        let all_flags = ac.flags.clone();
        let all_flag_values = ac.flag_values.clone();
        ui.horizontal(|ui| {
            crate::ui::components::autocomplete_edit(ui, "ac_flag_name", &mut self.temp_flag_name, &all_flags);
            ui.label("=");
            crate::ui::components::autocomplete_edit(ui, "ac_flag_value", &mut self.temp_flag_value, &all_flag_values);
        });

        ui.horizontal(|ui| {
            if ui.button(&btn_add).clicked() && !self.temp_flag_name.is_empty() {
                self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx]
                    .condition_flags
                    .push(ConditionFlag::new(
                        self.temp_flag_name.clone(),
                        self.temp_flag_value.clone(),
                    ));
                self.temp_flag_name.clear();
                self.temp_flag_value.clear();
                self.mark_modified();
            }

            let can_remove = self.current_flag_index.is_some();
            if ui.add_enabled(can_remove, egui::Button::new(&btn_remove)).clicked()
                && let Some(flag_idx) = self.current_flag_index
            {
                let flags =
                    &mut self.ximod.steps[step_idx].plugin_groups[group_idx].plugins[plugin_idx].condition_flags;
                if flag_idx < flags.len() {
                    flags.remove(flag_idx);
                    self.mark_modified();
                }
                self.current_flag_index = None;
            }
        });
    }
}
