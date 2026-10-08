//! "Conditional installs" tab.

use crate::models::*;
use crate::ui::components::*;
use crate::ui::main_window::XimodApp;
use crate::ui::widgets::dependency_editor::{DepEditorIds, DepTarget};
use crate::ui::widgets::files_table::{FilesTableSpec, FilesTarget};
use eframe::egui;

impl XimodApp {
    pub(crate) fn render_conditional_tab(&mut self, ui: &mut egui::Ui) {
        let title = self.i18n.t("tab-conditional");
        let hint_add = self.i18n.t("btn-add-pattern");
        let hint_remove = self.i18n.t("btn-remove-pattern");
        let label_operator = self.i18n.t("label-operator");
        let hint_operator = self.i18n.t("hint-operator");
        let hdr_files = self.i18n.t("label-files");

        section_header(ui, &title);

        let pattern_count = self.ximod.conditional_files.len();

        // Pre-compute translated pattern labels to avoid borrow conflicts in the closure
        let pattern_labels: Vec<String> = (0..pattern_count)
            .map(|idx| self.i18n.t_num("pattern-label", (idx + 1) as i64))
            .collect();

        // Conditional-install tabs — paginated (8 per page) when there are more than
        // 8, so the row never overflows past the window edge. The « / » buttons page
        // the visible window by 8.
        const COND_PAGE: usize = 8;
        let max_page = if pattern_count == 0 {
            0
        } else {
            (pattern_count - 1) / COND_PAGE
        };
        if self.cond_tab_page > max_page {
            self.cond_tab_page = max_page;
        }
        let paged = pattern_count > COND_PAGE;
        let start = self.cond_tab_page * COND_PAGE;
        let end = (start + COND_PAGE).min(pattern_count);

        ui.horizontal(|ui| {
            if paged
                && ui
                    .add_enabled(
                        self.cond_tab_page > 0,
                        crate::ui::components::icon_button(crate::ui::theme::icon::LEFT),
                    )
                    .clicked()
            {
                self.cond_tab_page -= 1;
            }

            for (idx, label) in pattern_labels.iter().enumerate().take(end).skip(start) {
                let selected = self.selection.cond_pattern == Some(idx);
                if ui.selectable_label(selected, label).clicked() {
                    self.select_cond_pattern(Some(idx));
                }
            }

            if paged {
                if ui
                    .add_enabled(
                        end < pattern_count,
                        crate::ui::components::icon_button(crate::ui::theme::icon::RIGHT),
                    )
                    .clicked()
                {
                    self.cond_tab_page += 1;
                }
                ui.label(format!("{}–{} / {}", start + 1, end, pattern_count));
            }

            if ui
                .add(crate::ui::components::icon_button(crate::ui::theme::icon::ADD))
                .on_hover_text(&hint_add)
                .clicked()
            {
                self.ximod.conditional_files.push(ConditionalFileSet::new());
                let last = self.ximod.conditional_files.len() - 1;
                self.select_cond_pattern(Some(last));
                self.cond_tab_page = last / COND_PAGE;
                self.mark_modified();
            }

            let can_remove = self.selection.cond_pattern.is_some();
            if ui
                .add_enabled(
                    can_remove,
                    crate::ui::components::icon_button(crate::ui::theme::icon::DELETE),
                )
                .on_hover_text(&hint_remove)
                .clicked()
                && let Some(idx) = self.selection.cond_pattern
            {
                if idx < self.ximod.conditional_files.len() {
                    self.ximod.conditional_files.remove(idx);
                    self.mark_modified();
                }
                let next = if self.ximod.conditional_files.is_empty() {
                    None
                } else {
                    Some(idx.saturating_sub(1).min(self.ximod.conditional_files.len() - 1))
                };
                self.select_cond_pattern(next);
            }
        });

        ui.separator();

        if let Some(pattern_idx) = self.selection.cond_pattern {
            if pattern_idx >= self.ximod.conditional_files.len() {
                return;
            }

            let current_op = self.ximod.conditional_files[pattern_idx].condition.operator;
            ui.horizontal(|ui| {
                ui.label(&label_operator);
                egui::ComboBox::from_id_salt("cond_operator")
                    .selected_text(crate::ui::labels::operator(&self.i18n, current_op))
                    .show_ui(ui, |ui| {
                        for op in LogicalOperator::variants() {
                            let label = crate::ui::labels::operator(&self.i18n, *op);
                            if ui.selectable_label(current_op == *op, label).clicked() {
                                self.ximod.conditional_files[pattern_idx].condition.operator = *op;
                                self.mark_modified();
                            }
                        }
                    })
                    .response
                    .on_hover_text(&hint_operator);
            });

            self.render_dependency_editor(
                ui,
                DepTarget::Conditional { pattern: pattern_idx },
                DepEditorIds {
                    list: "cond_deps_list",
                    type_combo: "dep_type",
                    ac_name: "ac_dep_name",
                    ac_value: "ac_dep_value",
                },
            );

            ui.add_space(8.0);
            subsection_header(ui, &hdr_files);

            self.render_files_table(
                ui,
                FilesTarget::Conditional { pattern: pattern_idx },
                FilesTableSpec {
                    list_id: "cond_files_list",
                    grid_id: "cond_files_grid",
                    max_height: 150.0,
                },
            );
        }
    }
}
