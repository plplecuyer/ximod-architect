//! "Nexus description" window (Tools menu): generates the mod page text
//! from the project (see `models/nexus_desc.rs`) in BBCode or Markdown,
//! with a copy button and a save-as button. A translation sidecar of the
//! root can be picked to generate the description of a translated FOMOD,
//! and a previous version (another FOMOD root) adds a changelog.

use std::path::PathBuf;

use crate::models::nexus_desc::{DescFormat, DescLabels, DescOptions, generate, generate_translated};
use crate::models::translate::TranslationDoc;
use crate::ui::main_window::XimodApp;
use crate::ui::widgets::free_window::record_win_geom;
use eframe::egui::{self, RichText};

/// Free-window state.
#[derive(Default)]
pub struct NexusDescState {
    pub open: bool,
    pub opts: DescOptions,
    /// Previous version: its root folder and the model loaded from it.
    pub previous: Option<(PathBuf, crate::models::Ximod)>,
    /// Translation sidecars found in the root (path, target language).
    pub sidecars: Vec<(PathBuf, String)>,
    /// Chosen sidecar (`None` = the source language).
    pub language: Option<PathBuf>,
    pub text: String,
    /// Set when an option changed: the text is regenerated on the next frame.
    pub dirty: bool,
}

impl XimodApp {
    /// Localised section titles for the generator.
    pub(crate) fn nexus_labels(&self) -> DescLabels {
        DescLabels {
            requirements: self.i18n.t("nexus-sec-requirements"),
            options: self.i18n.t("nexus-sec-options"),
            changelog: self.i18n.t("nexus-sec-changelog"),
            install: self.i18n.t("nexus-sec-install"),
            install_text: self.i18n.t("nexus-install-text"),
            requires: self.i18n.t("nexus-requires"),
            step: self.i18n.t("nexus-step"),
            added: self.i18n.t("nexus-added"),
            removed: self.i18n.t("nexus-removed"),
            changed: self.i18n.t("nexus-changed"),
        }
    }

    /// Open the window and generate the text.
    pub(crate) fn open_nexus_desc(&mut self) {
        self.nexus_desc.sidecars = self
            .root_directory
            .as_deref()
            .map(TranslationDoc::find_sidecars)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|p| {
                let lang = TranslationDoc::load(&p).ok()?.target_lang;
                Some((p, lang))
            })
            .collect();
        if self
            .nexus_desc
            .language
            .as_ref()
            .is_some_and(|l| !self.nexus_desc.sidecars.iter().any(|(p, _)| p == l))
        {
            self.nexus_desc.language = None;
        }
        self.regenerate_nexus_desc();
        self.nexus_desc.open = true;
    }

    /// (Re)generate the description from the current options.
    pub(crate) fn regenerate_nexus_desc(&mut self) {
        let labels = self.nexus_labels();
        let state = &self.nexus_desc;
        let previous = state.previous.as_ref().map(|(_, m)| m);
        let doc = state.language.as_ref().and_then(|p| TranslationDoc::load(p).ok());
        let text = match doc {
            Some(doc) => generate_translated(
                &doc,
                &self.ximod,
                self.root_directory.as_deref(),
                &self.games,
                previous,
                &state.opts,
                &labels,
            ),
            None => generate(
                &self.ximod,
                self.root_directory.as_deref(),
                &self.games,
                previous,
                &state.opts,
                &labels,
            ),
        };
        self.nexus_desc.text = text;
        self.nexus_desc.dirty = false;
    }

    /// Load a previous version of the mod from `path` for the changelog.
    pub(crate) fn nexus_set_previous(&mut self, path: PathBuf) {
        match crate::xml::load_ximod(&path) {
            Ok(m) => {
                self.nexus_desc.previous = Some((path, m));
                self.nexus_desc.dirty = true;
            }
            Err(e) => self.notify_err(self.i18n.t_arg("msg-wizard-error", "error", &e.to_string())),
        }
    }

    /// Render the window.
    pub(crate) fn render_nexus_desc(&mut self, ctx: &egui::Context) {
        if !self.nexus_desc.open {
            self.free_window_closed("ximod_nexus");
            return;
        }
        if self.nexus_desc.dirty {
            self.regenerate_nexus_desc();
        }

        let title = self.i18n.t("nexus-title");
        let l_format = self.i18n.t("nexus-format");
        let l_req = self.i18n.t("nexus-include-requirements");
        let l_opts = self.i18n.t("nexus-include-options");
        let l_install = self.i18n.t("nexus-include-install");
        let l_changelog = self.i18n.t("nexus-include-changelog");
        let l_previous = self.i18n.t("nexus-previous");
        let l_previous_none = self.i18n.t("nexus-previous-none");
        let l_language = self.i18n.t("nexus-language");
        let l_language_source = self.i18n.t("nexus-language-source");
        let l_copy = self.i18n.t("btn-copy");
        let l_save_as = self.i18n.t("btn-save-as");
        let l_close = self.i18n.t("btn-close");
        let l_clear = self.i18n.t("btn-clear");
        let msg_copied = self.i18n.t("msg-copied");
        let mod_name = if self.ximod.name.trim().is_empty() {
            "fomod".to_string()
        } else {
            self.ximod.name.trim().to_string()
        };

        let vb = self.free_viewport_builder(ctx, "ximod_nexus", title, [720.0, 600.0], false);
        let state = &mut self.nexus_desc;
        let cfg = &mut self.config;
        let mut do_close = false;
        let mut pick_previous = false;
        let mut save_as = false;
        let mut copied = false;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("ximod_nexus"), vb, |ctx, _class| {
            egui::TopBottomPanel::bottom("nexus_buttons").show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button(&l_copy).clicked() {
                        ui.ctx().copy_text(state.text.clone());
                        copied = true;
                    }
                    if ui.button(&l_save_as).clicked() {
                        save_as = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&l_close).clicked() {
                            do_close = true;
                        }
                    });
                });
                ui.add_space(4.0);
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                // ---- options ----
                ui.horizontal(|ui| {
                    ui.label(&l_format);
                    if ui
                        .radio_value(&mut state.opts.format, DescFormat::BBCode, "BBCode")
                        .changed()
                        || ui
                            .radio_value(&mut state.opts.format, DescFormat::Markdown, "Markdown")
                            .changed()
                    {
                        state.dirty = true;
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    if ui.checkbox(&mut state.opts.include_requirements, &l_req).changed()
                        || ui.checkbox(&mut state.opts.include_options, &l_opts).changed()
                        || ui.checkbox(&mut state.opts.include_install, &l_install).changed()
                        || ui.checkbox(&mut state.opts.include_changelog, &l_changelog).changed()
                    {
                        state.dirty = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(state.opts.include_changelog, |ui| {
                        if ui.button(&l_previous).clicked() {
                            pick_previous = true;
                        }
                        match &state.previous {
                            Some((path, m)) => {
                                let shown = if m.version.trim().is_empty() {
                                    m.name.clone()
                                } else {
                                    format!("{} {}", m.name, m.version)
                                };
                                ui.label(RichText::new(shown).strong())
                                    .on_hover_text(path.display().to_string());
                                if ui.small_button(&l_clear).clicked() {
                                    state.previous = None;
                                    state.dirty = true;
                                }
                            }
                            None => {
                                ui.label(RichText::new(&l_previous_none).weak());
                            }
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label(&l_language);
                    let current = match &state.language {
                        Some(p) => state
                            .sidecars
                            .iter()
                            .find(|(sp, _)| sp == p)
                            .map(|(_, lang)| lang.clone())
                            .unwrap_or_else(|| l_language_source.clone()),
                        None => l_language_source.clone(),
                    };
                    egui::ComboBox::from_id_salt("nexus_language")
                        .selected_text(current)
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_label(state.language.is_none(), &l_language_source)
                                .clicked()
                            {
                                state.language = None;
                                state.dirty = true;
                            }
                            let sidecars = state.sidecars.clone();
                            for (p, lang) in &sidecars {
                                let selected = state.language.as_ref() == Some(p);
                                if ui
                                    .selectable_label(selected, lang)
                                    .on_hover_text(p.display().to_string())
                                    .clicked()
                                {
                                    state.language = Some(p.clone());
                                    state.dirty = true;
                                }
                            }
                        });
                });
                ui.separator();
                // ---- generated text ----
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    let mut text = state.text.as_str();
                    ui.add_sized(
                        [ui.available_width(), ui.available_height()],
                        egui::TextEdit::multiline(&mut text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY),
                    );
                });
            });

            record_win_geom(cfg, ctx, "ximod_nexus");
            if ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                do_close = true;
            }
        });

        if copied {
            self.notify_ok(msg_copied);
        }
        if pick_previous && let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.nexus_set_previous(path);
        }
        if save_as {
            let ext = self.nexus_desc.opts.format.extension();
            let file = rfd::FileDialog::new()
                .set_file_name(format!("{mod_name}-description.{ext}"))
                .add_filter(ext, &[ext])
                .save_file();
            if let Some(path) = file {
                match std::fs::write(&path, &self.nexus_desc.text) {
                    Ok(()) => {
                        let msg = self.i18n.t_arg("msg-saved-to", "path", &path.display().to_string());
                        self.notify_ok(msg);
                    }
                    Err(e) => self.notify_err(e.to_string()),
                }
            }
        }
        if do_close {
            self.nexus_desc.open = false;
            self.free_window_closed("ximod_nexus");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Plugin, PluginGroup, SelectionType, Step, Ximod};

    fn app() -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale("eng");
        app.ximod = Ximod::new("Aurelia");
        app.ximod.version = "1.0".into();
        let mut step = Step::new("Textures");
        let mut group = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        group.plugins.push(Plugin::new("2K"));
        step.plugin_groups.push(group);
        app.ximod.steps.push(step);
        app
    }

    #[test]
    fn window_generates_and_renders_headless() {
        let mut app = app();
        app.open_nexus_desc();
        assert!(app.nexus_desc.open);
        assert!(app.nexus_desc.text.starts_with("[size=5][b]Aurelia 1.0[/b][/size]"));
        // Localised section titles, not raw keys.
        assert!(app.nexus_desc.text.contains("Installation options"));
        assert!(!app.nexus_desc.text.contains("nexus-"));
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_nexus_desc(ctx));
        // Options changes regenerate on the next frame.
        app.nexus_desc.opts.format = DescFormat::Markdown;
        app.nexus_desc.opts.include_options = false;
        app.nexus_desc.dirty = true;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_nexus_desc(ctx));
        assert!(app.nexus_desc.text.starts_with("# Aurelia 1.0"));
        assert!(!app.nexus_desc.text.contains("Installation options"));
        // A previous version adds the changelog.
        let mut prev = app.ximod.clone();
        prev.version = "0.9".into();
        app.nexus_desc.previous = Some((PathBuf::from("prev"), prev));
        app.nexus_desc.dirty = true;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_nexus_desc(ctx));
        assert!(app.nexus_desc.text.contains("## Changelog"));
        app.i18n.set_locale("fra");
        app.regenerate_nexus_desc();
        assert!(
            app.nexus_desc.text.contains("Journal des modifications"),
            "{}",
            app.nexus_desc.text
        );
        app.nexus_desc.open = false;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.render_nexus_desc(ctx));
    }

    #[test]
    fn translation_sidecars_of_the_root_are_listed() {
        let root = std::env::temp_dir().join(format!("ximod_nexus_dlg_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let mut app = app();
        crate::xml::save_ximod(&app.ximod, &root).unwrap();
        app.root_directory = Some(root.clone());
        let mut doc = crate::cli_translate::extract_translation(&root, "eng", "fra").unwrap();
        for u in doc.units.iter_mut().filter(|u| u.source == "Aurelia") {
            u.target = "Aurélia".into();
            u.status = crate::models::translate::TStatus::Translated;
        }
        let sidecar = TranslationDoc::sidecar_path(&root, "Aurelia", "fra");
        doc.save(&sidecar).unwrap();
        app.open_nexus_desc();
        assert_eq!(app.nexus_desc.sidecars.len(), 1);
        assert_eq!(app.nexus_desc.sidecars[0].1, "fra");
        app.nexus_desc.language = Some(sidecar);
        app.regenerate_nexus_desc();
        assert!(app.nexus_desc.text.contains("Aurélia 1.0"));
        // A previous version loaded from disk.
        app.nexus_set_previous(root.clone());
        assert!(app.nexus_desc.previous.is_some());
        app.nexus_set_previous(root.join("nope"));
        let _ = std::fs::remove_dir_all(&root);
    }
}
