//! "Archive contents" dialog (Tools menu, and the view button of the file
//! tables): the directory of a Bethesda archive (`.bsa` / `.ba2`) — format
//! and version, entry count, total unpacked size, then a searchable table
//! of the entries (path, size, compressed) read without touching the file
//! data (see `models::bethesda_archive`).

use std::path::{Path, PathBuf};

use crate::models::bethesda_archive::{ArchiveListing, list_archive};
use crate::models::simulate::format_size;
use crate::ui::components::{modal_keys, modal_window};
use crate::ui::main_window::XimodApp;
use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};

/// Rows shown at most: a texture archive can hold far more.
pub const MAX_VISIBLE_ROWS: usize = 5_000;

/// State of the dialog.
#[derive(Default)]
pub struct ArchiveViewState {
    /// The archive on disk.
    pub path: PathBuf,
    /// Its directory, when it could be read.
    pub listing: Option<ArchiveListing>,
    /// Why it could not be read.
    pub error: Option<String>,
    /// Filter typed in the search field (case-insensitive substring).
    pub search: String,
}

impl ArchiveViewState {
    /// Indices of the entries matching the search, in archive order.
    pub fn visible(&self) -> Vec<usize> {
        let Some(listing) = &self.listing else {
            return Vec::new();
        };
        let needle = self.search.trim().to_lowercase();
        listing
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| needle.is_empty() || e.path.contains(&needle))
            .map(|(i, _)| i)
            .collect()
    }
}

impl XimodApp {
    /// Ask for a `.bsa` / `.ba2` and open the dialog on it.
    pub(crate) fn open_archive_view_pick(&mut self) {
        let mut dialog = rfd::FileDialog::new().add_filter(self.i18n.t("filter-bethesda-archive"), &["bsa", "ba2"]);
        if let Some(root) = &self.root_directory {
            dialog = dialog.set_directory(root);
        }
        if let Some(path) = dialog.pick_file() {
            self.open_archive_view(&path);
        }
    }

    /// Open the dialog on the archive at `path` (an unreadable archive opens
    /// the dialog with the error).
    pub(crate) fn open_archive_view(&mut self, path: &Path) {
        let mut st = ArchiveViewState {
            path: path.to_path_buf(),
            ..Default::default()
        };
        match list_archive(path) {
            Ok(listing) => st.listing = Some(listing),
            Err(e) => st.error = Some(e.to_string()),
        }
        self.archive_view = st;
        self.show_archive_view = true;
    }

    /// Open the dialog on a project-relative source (the view button of the
    /// file tables). Without a root the button is not shown.
    pub(crate) fn open_archive_view_rel(&mut self, source: &str) {
        let Some(root) = self.root_directory.clone() else {
            return;
        };
        let abs = root.join(source.trim().replace('\\', "/").trim_start_matches("./"));
        self.open_archive_view(&abs);
    }

    /// Render the modal dialog.
    pub(crate) fn render_archive_view(&mut self, ctx: &egui::Context) {
        if !self.show_archive_view {
            return;
        }
        let title = self.i18n.t("archive-view-title");
        let l_format = self.i18n.t("archive-view-format");
        let l_search = self.i18n.t("archive-view-search");
        let col_path = self.i18n.t("archive-view-col-path");
        let col_size = self.i18n.t("archive-view-col-size");
        let col_compressed = self.i18n.t("archive-view-col-compressed");
        let yes = self.i18n.t("btn-yes");
        let no = self.i18n.t("btn-no");
        let close = self.i18n.t("btn-close");
        let mut st = std::mem::take(&mut self.archive_view);
        let file_name = st
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let header = st.listing.as_ref().map(|l| {
            let mut args = fluent::FluentArgs::new();
            args.set("num", l.entries.len() as i64);
            let entries = self.i18n.t_with_args("archive-view-entries", Some(&args));
            let mut args = fluent::FluentArgs::new();
            args.set("size", format_size(l.total_size()));
            let size = self.i18n.t_with_args("archive-view-size", Some(&args));
            (l.format.label(), entries, size)
        });
        let error = st
            .error
            .as_ref()
            .map(|e| self.i18n.t_arg("archive-view-error", "error", e));
        let visible = st.visible();
        let truncated = (visible.len() > MAX_VISIBLE_ROWS).then(|| {
            let mut args = fluent::FluentArgs::new();
            args.set("num", MAX_VISIBLE_ROWS as i64);
            self.i18n.t_with_args("archive-view-truncated", Some(&args))
        });

        let (_enter, esc) = modal_keys(ctx);
        let mut do_close = false;
        modal_window(ctx, &title, |ui| {
            ui.set_min_width(640.0);
            ui.label(RichText::new(&file_name).strong())
                .on_hover_text(st.path.display().to_string());
            if let Some(error) = &error {
                let palette = crate::ui::theme::Palette::from_ui(ui);
                ui.colored_label(palette.danger, error);
            }
            if let Some((format, entries, size)) = &header {
                ui.horizontal_wrapped(|ui| {
                    ui.label(&l_format);
                    ui.label(RichText::new(format).monospace());
                    ui.label("·");
                    ui.label(entries);
                    ui.label("·");
                    ui.label(size);
                });
                ui.add(
                    egui::TextEdit::singleline(&mut st.search)
                        .hint_text(&l_search)
                        .desired_width(f32::INFINITY),
                );
                if let Some(t) = &truncated {
                    ui.label(RichText::new(t).small().weak());
                }
                let listing = st.listing.as_ref().expect("header implies a listing");
                let rows = visible.len().min(MAX_VISIBLE_ROWS);
                let row_h = ui.text_style_height(&egui::TextStyle::Body) + 6.0;
                egui::ScrollArea::horizontal()
                    .id_salt("archive_view_hscroll")
                    .show(ui, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::remainder().clip(true).at_least(240.0))
                            .column(Column::initial(90.0).at_least(60.0))
                            .column(Column::initial(90.0).at_least(60.0))
                            .min_scrolled_height(120.0)
                            .max_scroll_height(360.0)
                            .header(row_h, |mut h| {
                                h.col(|ui| {
                                    ui.label(RichText::new(&col_path).strong());
                                });
                                h.col(|ui| {
                                    ui.label(RichText::new(&col_size).strong());
                                });
                                h.col(|ui| {
                                    ui.label(RichText::new(&col_compressed).strong());
                                });
                            })
                            .body(|body| {
                                body.rows(row_h, rows, |mut row| {
                                    let entry = &listing.entries[visible[row.index()]];
                                    row.col(|ui| {
                                        ui.label(RichText::new(&entry.path).monospace())
                                            .on_hover_text(&entry.path);
                                    });
                                    row.col(|ui| {
                                        ui.label(format_size(entry.size));
                                    });
                                    row.col(|ui| {
                                        ui.label(if entry.compressed { &yes } else { &no });
                                    });
                                });
                            });
                    });
            }
            ui.add_space(12.0);
            if ui.button(&close).clicked() || esc {
                do_close = true;
            }
        });
        self.archive_view = st;
        if do_close {
            self.show_archive_view = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archive::{Ba2Game, Ba2Options, package_ba2};
    use crate::models::bethesda_archive::ArchiveFormat;

    #[test]
    fn archive_view_lists_filters_and_renders_headless() {
        let root = std::env::temp_dir().join(format!("ximod_archive_view_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("src/textures")).unwrap();
        std::fs::write(root.join("src/textures/a.dds"), vec![0u8; 2048]).unwrap();
        std::fs::write(root.join("src/b.nif"), b"nif").unwrap();
        let archive = root.join("Mod - Main.ba2");
        package_ba2(
            &root.join("src"),
            &archive,
            &Ba2Options {
                game: Ba2Game::Fallout4,
                general: true,
            },
        )
        .unwrap();
        std::fs::write(root.join("Bad.bsa"), b"nope").unwrap();

        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.root_directory = Some(root.clone());
        app.open_archive_view_rel("Mod - Main.ba2");
        assert!(app.show_archive_view);
        let listing = app.archive_view.listing.as_ref().expect("readable");
        assert_eq!(listing.format, ArchiveFormat::Ba2Gnrl { version: 1 });
        assert_eq!(listing.entries.len(), 2);
        assert_eq!(listing.total_size(), 2051);
        assert_eq!(app.archive_view.visible().len(), 2);
        app.archive_view.search = "TEX".into();
        assert_eq!(
            app.archive_view.visible(),
            vec![listing.entries.iter().position(|e| e.path == "textures/a.dds").unwrap()]
        );

        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_archive_view(ctx));
        assert!(app.show_archive_view, "a frame keeps the dialog open");
        assert!(app.archive_view.listing.is_some(), "state survives a frame");
        assert_eq!(app.archive_view.search, "TEX");

        // Escape closes it.
        let mut input = egui::RawInput::default();
        input.events.push(egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        });
        let _ = ctx.run(input, |ctx| app.render_archive_view(ctx));
        assert!(!app.show_archive_view);

        // An unreadable archive opens with its error and still renders.
        app.open_archive_view(&root.join("Bad.bsa"));
        assert!(app.show_archive_view);
        assert!(app.archive_view.listing.is_none());
        assert!(app.archive_view.error.is_some());
        assert!(app.archive_view.visible().is_empty());
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_archive_view(ctx));
        assert!(app.archive_view.error.is_some());

        // Without a root the relative opener does nothing.
        app.show_archive_view = false;
        app.root_directory = None;
        app.open_archive_view_rel("Mod - Main.ba2");
        assert!(!app.show_archive_view);
        let _ = std::fs::remove_dir_all(&root);
    }
}
