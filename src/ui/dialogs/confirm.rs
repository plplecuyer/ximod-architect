//! Confirmation dialogs: destructive actions, unsaved changes on exit / close.

use crate::ui::components::*;
use crate::ui::docs::CloseScope;
use crate::ui::main_window::{ConfirmAction, XimodApp};
use eframe::egui::{self, RichText};

impl XimodApp {
    /// Render the confirmation dialog when a `ConfirmAction` is pending.
    /// Uses the reusable `ConfirmDialog` component with translated text.
    pub(crate) fn render_confirm_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_confirm {
            return;
        }
        let action = match self.confirm_action.clone() {
            Some(a) => a,
            None => {
                self.show_confirm = false;
                return;
            }
        };

        let title = self.i18n.t("confirm-title");
        let message = match &action {
            ConfirmAction::DeleteStep(_) => self.i18n.t("confirm-delete"),
            ConfirmAction::SaveAnyway(msg) => msg.clone(),
        };

        let mut dialog = ConfirmDialog::new(title, message);
        // The "save anyway" dialog reads better with explicit Save/Cancel
        // buttons than a bare Yes/No.
        match &action {
            ConfirmAction::SaveAnyway(_) => {
                dialog.confirm_text = self.i18n.t("btn-save");
                dialog.cancel_text = self.i18n.t("btn-cancel");
            }
            _ => {
                dialog.confirm_text = self.i18n.t("btn-yes");
                dialog.cancel_text = self.i18n.t("btn-no");
            }
        }

        let mut open = true;
        match dialog.show(ctx, &mut open) {
            Some(true) => {
                self.execute_confirm_action(action);
                self.show_confirm = false;
                self.confirm_action = None;
            }
            Some(false) => {
                self.show_confirm = false;
                self.confirm_action = None;
            }
            None => {
                if !open {
                    self.show_confirm = false;
                    self.confirm_action = None;
                }
            }
        }
        // Escape cancels the confirmation (never executes the action).
        if self.show_confirm && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.show_confirm = false;
            self.confirm_action = None;
        }
    }

    /// Execute a confirmed action.
    pub(crate) fn execute_confirm_action(&mut self, action: ConfirmAction) {
        match action {
            ConfirmAction::DeleteStep(step_idx) => {
                if step_idx < self.ximod.steps.len() {
                    self.ximod.steps.remove(step_idx);
                    let next = if self.ximod.steps.is_empty() {
                        None
                    } else {
                        Some(step_idx.saturating_sub(1).min(self.ximod.steps.len() - 1))
                    };
                    self.select_step(next);
                    self.mark_modified();
                }
            }
            ConfirmAction::SaveAnyway(_) => {
                self.write_project();
            }
        }
    }

    /// The unsaved-changes prompt shown when closing XIMOD (Yes / No / Cancel).
    pub(crate) fn render_exit_prompt(&mut self, ctx: &egui::Context) {
        if !self.show_exit_prompt {
            return;
        }
        let title = self.i18n.t("exit-title");
        let message = self.i18n.t("exit-unsaved");
        let names = self.modified_doc_names().join(", ");
        let docs_line = self.i18n.t_arg("exit-unsaved-docs", "names", &names);
        let yes = self.i18n.t("btn-yes");
        let no = self.i18n.t("btn-no");
        let cancel = self.i18n.t("btn-cancel");
        let mut choice = 0u8;
        let (enter, esc) = modal_keys(ctx);
        modal_window(ctx, &title, |ui| {
            ui.label(message);
            if !names.is_empty() {
                ui.label(egui::RichText::new(docs_line).weak());
            }
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new(RichText::new(yes).strong())).clicked() || enter {
                    choice = 1;
                }
                if ui.button(no).clicked() {
                    choice = 2;
                }
                if ui.button(cancel).clicked() || esc {
                    choice = 3;
                }
            });
        });
        match choice {
            1 => {
                if self.save_all_modified() {
                    self.show_exit_prompt = false;
                    self.exit_confirmed = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                } else {
                    // The user cancelled a "choose folder" dialog: stay open.
                    self.show_exit_prompt = false;
                }
            }
            2 => {
                self.show_exit_prompt = false;
                self.exit_confirmed = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            3 => self.show_exit_prompt = false,
            _ => {}
        }
    }

    /// Confirmation shown when closing a modified FOMOD (Yes / No / Cancel).
    pub(crate) fn render_close_prompt(&mut self, ctx: &egui::Context) {
        let Some(scope) = self.close_prompt else {
            return;
        };
        let title = self.i18n.t("exit-title");
        let message = self.i18n.t("exit-unsaved");
        let yes = self.i18n.t("btn-yes");
        let no = self.i18n.t("btn-no");
        let cancel = self.i18n.t("btn-cancel");
        let mut choice = 0u8;
        let (enter, esc) = modal_keys(ctx);
        modal_window(ctx, &title, |ui| {
            ui.label(message);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new(RichText::new(yes).strong())).clicked() || enter {
                    choice = 1;
                }
                if ui.button(no).clicked() {
                    choice = 2;
                }
                if ui.button(cancel).clicked() || esc {
                    choice = 3;
                }
            });
        });
        match choice {
            1 => {
                // Save, then close.
                match scope {
                    CloseScope::Active => {
                        if self.ensure_root_for_save() {
                            self.write_project();
                            self.close_active_fomod_force();
                        }
                    }
                    CloseScope::All => {
                        if self.save_all_modified() {
                            self.close_all_fomods_force();
                        }
                    }
                }
                self.close_prompt = None;
            }
            2 => {
                // Close without saving.
                match scope {
                    CloseScope::Active => self.close_active_fomod_force(),
                    CloseScope::All => self.close_all_fomods_force(),
                }
                self.close_prompt = None;
            }
            3 => self.close_prompt = None,
            _ => {}
        }
    }
}
