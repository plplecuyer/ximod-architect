//! Update check (GitHub Releases) and the "new version" banner.

use crate::ui::main_window::XimodApp;
use eframe::egui;

impl XimodApp {
    /// Today's date as `YYYY-MM-DD` (local), used to rate-limit the update check.
    pub(crate) fn today_str() -> String {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    }

    /// Start a background update check.
    ///
    /// `manual` = the user asked for it (Help → Check for updates): it always
    /// runs and reports the outcome. Otherwise it is the automatic startup check,
    /// which runs at most once per day and stays silent when up to date or on
    /// error.
    pub(crate) fn start_update_check(&mut self, ctx: &egui::Context, manual: bool) {
        // Never run two checks at once.
        if self.update_rx.is_some() {
            return;
        }
        if !manual {
            if !self.config.check_updates {
                return;
            }
            if self.config.last_update_check == Self::today_str() {
                return;
            }
        }
        self.update_manual = manual;
        self.update_rx = Some(crate::update::spawn_check(ctx.clone()));
        if manual {
            self.notify_info(self.i18n.t("update-checking"));
        }
    }

    /// Poll the in-flight update check (called once per frame). Applies the
    /// result: shows the banner, updates the status line, and records the
    /// last-check date in the config.
    pub(crate) fn poll_update_check(&mut self, ctx: &egui::Context) {
        let Some(rx) = &self.update_rx else { return };
        let msg = match rx.try_recv() {
            Ok(m) => m,
            Err(std::sync::mpsc::TryRecvError::Empty) => return,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.update_rx = None;
                return;
            }
        };
        self.update_rx = None;
        let manual = self.update_manual;

        match msg {
            crate::update::UpdateCheck::Available(version) => {
                // An automatic check honors a previously "skipped" version; a
                // manual check always shows the result.
                if manual || self.config.skip_update_version != version {
                    self.update_available = Some(version.clone());
                }
                if manual {
                    self.status_message = self.i18n.t_arg("update-available-status", "version", &version);
                }
                // Record a successful check (only the automatic one is rate-limited,
                // but stamping the date here is harmless for both).
                self.config.last_update_check = Self::today_str();
                let _ = self.config.save();
                ctx.request_repaint();
            }
            crate::update::UpdateCheck::UpToDate => {
                if manual {
                    self.notify_ok(self.i18n.t("update-up-to-date"));
                }
                self.config.last_update_check = Self::today_str();
                let _ = self.config.save();
            }
            crate::update::UpdateCheck::Failed(_e) => {
                if manual {
                    self.notify_err(self.i18n.t("update-check-failed"));
                }
                // A failed automatic check is not stamped, so it retries next launch.
            }
        }
    }

    /// Draw the "new version available" banner at the top of the window, when one
    /// is pending. Returns nothing; mutates state on the user's actions.
    pub(crate) fn render_update_banner(&mut self, ctx: &egui::Context) {
        let Some(version) = self.update_available.clone() else {
            return;
        };

        let msg = self.i18n.t_arg("update-banner-text", "version", &version);
        let btn_download = self.i18n.t("update-download");
        let btn_skip = self.i18n.t("update-skip");
        let btn_later = self.i18n.t("update-later");

        let mut dismiss = false;
        let mut skip = false;

        egui::TopBottomPanel::top("update_banner")
            .frame(
                egui::Frame::none()
                    .fill(crate::ui::theme::Palette::of(&ctx.style().visuals).info_bg)
                    .inner_margin(egui::Margin::symmetric(10.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new(&msg).strong());
                    ui.label(&btn_download);
                    ui.hyperlink_to("Nexus", crate::update::NEXUS_URL);
                    ui.hyperlink_to("GitHub", crate::update::RELEASES_URL);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&btn_later).clicked() {
                            dismiss = true;
                        }
                        if ui.button(&btn_skip).clicked() {
                            skip = true;
                        }
                    });
                });
            });

        if skip {
            self.config.skip_update_version = version;
            let _ = self.config.save();
            self.update_available = None;
        } else if dismiss {
            // Hide for this session only; it may reappear on the next launch.
            self.update_available = None;
        }
    }
}
