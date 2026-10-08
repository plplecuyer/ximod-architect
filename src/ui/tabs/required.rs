//! "Required installs" tab.

use crate::ui::components::*;
use crate::ui::main_window::XimodApp;
use crate::ui::widgets::files_table::{FilesTableSpec, FilesTarget};
use eframe::egui;

impl XimodApp {
    pub(crate) fn render_required_tab(&mut self, ui: &mut egui::Ui) {
        let title = self.i18n.t("tab-required");

        section_header(ui, &title);

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
}
