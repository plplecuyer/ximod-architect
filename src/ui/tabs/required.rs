//! "Required installs" tab.

use crate::ui::components::*;
use crate::ui::main_window::XimodApp;
use crate::ui::widgets::files_table::{FilesTableSpec, FilesTarget};
use eframe::egui;

impl XimodApp {
    pub(crate) fn render_required_tab(&mut self, ui: &mut egui::Ui) {
        let title = self.i18n.t("tab-required");

        section_header(ui, &title);

        self.render_required_destination(ui);
        ui.add_space(6.0);
        self.render_files_table(
            ui,
            FilesTarget::Required,
            FilesTableSpec {
                list_id: "req_files_list",
                grid_id: "req_files_grid",
                max_height: 300.0,
            },
        );
    }

    /// Panel: assign one install destination to every required file at once
    /// (the counterpart of the group / page controls of the Install Steps tab).
    pub(crate) fn render_required_destination(&mut self, ui: &mut egui::Ui) {
        let hint = self.i18n.t("req-dest-hint");
        let btn_apply = self.i18n.t("btn-apply-req-dest");
        let label_dest = self.i18n.t("label-destination");
        let nofiles = self.i18n.t("bulk-dest-nofiles");
        let file_count = self.ximod.required_files.len();

        ui.label(egui::RichText::new(&hint).small().color(ui.visuals().weak_text_color()));
        ui.horizontal(|ui| {
            ui.label(&label_dest);
            ui.text_edit_singleline(&mut self.req_dest_buf);
            if ui.add_enabled(file_count > 0, egui::Button::new(&btn_apply)).clicked() {
                let dest = self.req_dest_buf.trim().to_string();
                let n = self.ximod.set_all_required_destinations(&dest);
                self.mark_modified();
                self.notify_ok(self.i18n.t_num("status-dest-applied", n as i64));
            }
        });
        if file_count == 0 {
            ui.label(
                egui::RichText::new(&nofiles)
                    .small()
                    .color(ui.visuals().weak_text_color()),
            );
        }
    }
}
