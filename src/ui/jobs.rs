//! Background jobs driven from the UI: distribution export and the on-disk
//! part of the validation.
//!
//! Both used to run synchronously inside a menu handler, freezing the window
//! for as long as the compression or the directory walk took (minutes for a
//! texture mod in LZMA2). They now run on a worker thread and talk back
//! through an `mpsc` channel; the thread requests a repaint after every
//! message so the progress window updates even when the mouse is still.

use super::main_window::XimodApp;
use eframe::egui;
use fluent::FluentArgs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};

/// Messages sent by the export worker.
pub enum ExportMsg {
    /// `(files_done, files_total, current_file)`.
    Progress(usize, usize, String),
    /// Finished: number of files on success, error text otherwise.
    Done(Result<usize, String>),
}

/// What a worker job is for: decides the progress window title and what
/// happens with the result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobPurpose {
    /// Packaging an archive (distribution export, translation package).
    Export,
    /// Extracting a mod archive to open it as a project.
    OpenArchive,
    /// Extracting the `fomod/` of a mod archive for the translation window.
    TranslateArchive,
}

/// An export running on a worker thread.
pub struct ExportJob {
    rx: Receiver<ExportMsg>,
    cancel: Arc<AtomicBool>,
    /// Destination archive (shown in the window, and reported on success).
    pub path: PathBuf,
    pub purpose: JobPurpose,
    done: usize,
    total: usize,
    current: String,
}

/// Result of the disk-bound validation checks, computed off the UI thread.
/// Language-neutral: the UI translates it when it arrives.
/// A validation running for a document: the channel its report arrives on
/// and the project revision it was started from. Shared (`Rc`) so a document
/// snapshot can keep it while the tab is inactive.
pub type PendingValidation = std::rc::Rc<(std::sync::mpsc::Receiver<DiskReport>, u64)>;

pub struct DiskReport {
    pub file_issues: Vec<crate::models::verify::FileIssue>,
    pub conflicts: Vec<crate::models::conflicts::Conflict>,
    pub image_issues: Vec<(String, crate::media::ImageIssue)>,
    pub plugin_issues: Vec<crate::models::plugin_checks::PluginIssue>,
    /// Install size of every option, the required files and the conditional
    /// sets (shown in the inspector and the tree once the report arrives).
    pub sizes: crate::models::simulate::SizeReport,
}

impl XimodApp {
    /// Start packaging `root` into `path` on a worker thread. The FOMOD XML
    /// must already be saved (the caller does that on the UI thread, it is
    /// fast and must see the live model).
    pub(crate) fn spawn_export(
        &mut self,
        ctx: &egui::Context,
        root: PathBuf,
        path: PathBuf,
        format: crate::archive::ArchiveFormat,
    ) {
        let out = path.clone();
        self.spawn_archive_job(ctx, path, move |progress| {
            crate::archive::package_directory(&root, &out, format, crate::archive::CompressionLevel::Normal, progress)
                .map_err(|e| e.to_string())
        });
    }

    /// Run `work` on a worker thread with the export progress window. `work`
    /// receives the throttled, cancellable progress callback and returns the
    /// number of files written. Shared by the distribution export and the
    /// translation package.
    pub(crate) fn spawn_archive_job<F>(&mut self, ctx: &egui::Context, path: PathBuf, work: F)
    where
        F: FnOnce(crate::export::Progress<'_>) -> Result<usize, String> + Send + 'static,
    {
        self.spawn_job(ctx, path, JobPurpose::Export, work);
    }

    /// [`spawn_archive_job`](Self::spawn_archive_job) with an explicit
    /// purpose (extraction jobs hand their result to `load_project` or to
    /// the translation window instead of a toast only).
    pub(crate) fn spawn_job<F>(&mut self, ctx: &egui::Context, path: PathBuf, purpose: JobPurpose, work: F)
    where
        F: FnOnce(crate::export::Progress<'_>) -> Result<usize, String> + Send + 'static,
    {
        let (tx, rx): (Sender<ExportMsg>, Receiver<ExportMsg>) = std::sync::mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let cancel_worker = cancel.clone();
        let ctx_worker = ctx.clone();
        std::thread::Builder::new()
            .name("ximod-export".into())
            .spawn(move || {
                let mut last_report = std::time::Instant::now();
                let mut progress = |done: usize, total: usize, rel: &std::path::Path| -> bool {
                    if cancel_worker.load(Ordering::Relaxed) {
                        return false;
                    }
                    // Throttle UI updates: one message per ~50 ms, plus the
                    // first and last file.
                    let now = std::time::Instant::now();
                    if done == 0 || done == total || now.duration_since(last_report).as_millis() >= 50 {
                        last_report = now;
                        let _ = tx.send(ExportMsg::Progress(done, total, rel.to_string_lossy().into_owned()));
                        ctx_worker.request_repaint();
                    }
                    true
                };
                let result = work(&mut progress);
                let _ = tx.send(ExportMsg::Done(result));
                ctx_worker.request_repaint();
            })
            .ok();
        self.export_job = Some(ExportJob {
            rx,
            cancel,
            path,
            purpose,
            done: 0,
            total: 0,
            current: String::new(),
        });
    }

    /// Poll the export worker and draw its progress window. Called every
    /// frame; a no-op when no export is running.
    pub(crate) fn render_export_job(&mut self, ctx: &egui::Context) {
        let Some(job) = self.export_job.as_mut() else { return };

        // Drain the channel.
        let mut finished: Option<Result<usize, String>> = None;
        while let Ok(msg) = job.rx.try_recv() {
            match msg {
                ExportMsg::Progress(done, total, current) => {
                    job.done = done;
                    job.total = total;
                    job.current = current;
                }
                ExportMsg::Done(result) => finished = Some(result),
            }
        }

        if let Some(result) = finished {
            let cancelled = job.cancel.load(Ordering::Relaxed);
            let path = job.path.clone();
            let purpose = job.purpose.clone();
            self.export_job = None;
            match (purpose, result) {
                (JobPurpose::Export, Ok(n)) => {
                    let mut args = FluentArgs::new();
                    args.set("count", n as i64);
                    args.set("path", path.display().to_string());
                    let msg = self.i18n.t_with_args("msg-export-success", Some(&args));
                    self.notify_ok(msg);
                }
                (JobPurpose::OpenArchive, Ok(n)) => self.archive_extracted(&path, n),
                (JobPurpose::TranslateArchive, Ok(_)) => self.translate_archive_extracted(&path),
                (JobPurpose::Export, Err(_)) if cancelled => {
                    let msg = self.i18n.t("msg-export-cancelled");
                    self.notify_warn(msg);
                }
                (JobPurpose::Export, Err(e)) => {
                    let mut args = FluentArgs::new();
                    args.set("error", e);
                    let msg = self.i18n.t_with_args("msg-export-error", Some(&args));
                    self.notify_err(msg);
                }
                (_, Err(e)) => {
                    let mut args = FluentArgs::new();
                    args.set("error", e);
                    let msg = self.i18n.t_with_args("msg-archive-error", Some(&args));
                    if cancelled {
                        self.notify_warn(msg);
                    } else {
                        self.notify_err(msg);
                    }
                }
            }
            return;
        }

        // Progress window (modal: the menus are disabled while a job runs).
        let title = match job.purpose {
            JobPurpose::Export => self.i18n.t("export-progress-title"),
            JobPurpose::OpenArchive | JobPurpose::TranslateArchive => self.i18n.t("msg-archive-extracting"),
        };
        let cancel_label = self.i18n.t("btn-cancel");
        let fraction = if job.total == 0 {
            0.0
        } else {
            job.done as f32 / job.total as f32
        };
        let mut args = FluentArgs::new();
        args.set("done", job.done as i64);
        args.set("total", job.total as i64);
        let counter = self.i18n.t_with_args("export-progress-files", Some(&args));
        let current = job.current.clone();
        let cancelling = job.cancel.load(Ordering::Relaxed);
        let mut do_cancel = false;

        crate::ui::components::modal_veil(ctx);
        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.set_min_width(420.0);
                ui.label(job.path.display().to_string());
                ui.add_space(6.0);
                ui.add(egui::ProgressBar::new(fraction).text(counter).animate(job.total == 0));
                ui.add_space(4.0);
                ui.label(egui::RichText::new(current).small().weak());
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.add_enabled(!cancelling, egui::Button::new(&cancel_label)).clicked() {
                        do_cancel = true;
                    }
                    if cancelling {
                        ui.spinner();
                    }
                });
            });

        if do_cancel {
            job.cancel.store(true, Ordering::Relaxed);
        }
    }

    /// Start the disk-bound validation checks on a worker thread. The result
    /// is appended to the validation report when it arrives.
    pub(crate) fn spawn_disk_validation(&mut self, ctx: &egui::Context, root: PathBuf) {
        let (tx, rx) = std::sync::mpsc::channel::<DiskReport>();
        let ximod = self.ximod.clone();
        let ctx_worker = ctx.clone();
        let generation = self.project_revision;
        let base_masters: Vec<String> = self.games.base_masters_for(&ximod.game).to_vec();
        let esl_limits = crate::models::plugin_header::EslLimits::for_game(&self.games, &ximod.game);
        std::thread::Builder::new()
            .name("ximod-validate".into())
            .spawn(move || {
                let file_issues = crate::models::verify::verify_files(&ximod, &root);
                let plugin_issues =
                    crate::models::plugin_checks::check_plugins(&ximod, &root, &base_masters, esl_limits);
                let conflicts = crate::models::conflicts::detect_conflicts(
                    &ximod,
                    &root,
                    crate::models::conflicts::ConflictMode::CertainOnly,
                );
                let constraints = crate::media::ImageConstraints::default();
                let mut images: Vec<String> = Vec::new();
                if let Some(h) = ximod.header_image.as_ref().filter(|s| !s.trim().is_empty()) {
                    images.push(h.clone());
                }
                for step in &ximod.steps {
                    for g in &step.plugin_groups {
                        for p in &g.plugins {
                            if let Some(img) = p.image_path.as_ref().filter(|s| !s.trim().is_empty()) {
                                images.push(img.clone());
                            }
                        }
                    }
                }
                let mut image_issues = Vec::new();
                for rel in images {
                    let abs = root.join(rel.replace('\\', "/"));
                    if !abs.is_file() {
                        continue; // missing images are already reported by verify
                    }
                    for issue in crate::media::validate_image(&abs, &constraints) {
                        image_issues.push((rel.clone(), issue));
                    }
                }
                let sizes = crate::models::simulate::SizeReport::compute(&ximod, &root);
                let _ = tx.send(DiskReport {
                    file_issues,
                    conflicts,
                    image_issues,
                    plugin_issues,
                    sizes,
                });
                ctx_worker.request_repaint();
            })
            .ok();
        self.validation_rx = Some(std::rc::Rc::new((rx, generation)));
    }

    /// Poll the validation worker; translate and append its findings.
    pub(crate) fn poll_disk_validation(&mut self) {
        let Some(pending) = self.validation_rx.as_ref() else {
            return;
        };
        let Ok(report) = pending.0.try_recv() else { return };
        let generation = pending.1;
        self.validation_rx = None;
        use crate::models::verify::FileIssue;
        use crate::ui::problems::{Issue, Severity};
        let mut found: Vec<Issue> =
            Vec::with_capacity(report.file_issues.len() + report.conflicts.len() + report.image_issues.len());
        for issue in &report.file_issues {
            let (severity, target) = match issue {
                FileIssue::MissingSource { loc, .. }
                | FileIssue::MissingImage { loc, .. }
                | FileIssue::AbsolutePath { loc, .. }
                | FileIssue::OutsideRoot { loc, .. } => (Severity::Error, self.target_of_loc(loc)),
                FileIssue::OrphanFile { .. } => (Severity::Info, None),
            };
            found.push(Issue::new(severity, self.translate_file_issue(issue), target));
        }
        for c in &report.conflicts {
            use crate::models::conflicts::ConflictKind;
            let target = c.sources.first().and_then(|s| self.target_of_loc(&s.loc));
            // Loose files over archives is the rule of every mod manager:
            // an archive-vs-loose collision is informative, not a warning.
            let severity = match c.kind {
                ConflictKind::ArchiveVsLoose => Severity::Info,
                ConflictKind::Loose | ConflictKind::ArchiveVsArchive if c.certain => Severity::Warning,
                _ => Severity::Info,
            };
            found.push(Issue::new(severity, self.translate_conflict(c), target));
        }
        for (rel, issue) in &report.image_issues {
            found.push(Issue::new(
                Severity::Warning,
                self.translate_image_issue(rel, issue),
                self.target_of_image(rel),
            ));
        }
        for issue in &report.plugin_issues {
            use crate::models::plugin_checks::PluginIssue as P;
            let (severity, loc) = match issue {
                P::EslEligible { loc, .. } => (Severity::Info, loc),
                P::MissingMaster { loc, .. } | P::EslFlagMismatch { loc, .. } | P::EslTooBig { loc, .. } => {
                    (Severity::Warning, loc)
                }
            };
            found.push(Issue::new(
                severity,
                self.translate_plugin_issue(issue),
                self.target_of_loc(loc),
            ));
        }
        // If the project changed while the checks ran, say so rather than
        // presenting stale findings as current.
        if generation != self.project_revision {
            found.push(Issue::new(Severity::Info, self.i18n.t("verify-stale"), None));
        }
        self.validation_issues.extend(found);
        self.sizes = Some(report.sizes);
    }

    /// The measured sizes, when a validation ran for the current root.
    pub(crate) fn size_report(&self) -> Option<&crate::models::simulate::SizeReport> {
        let report = self.sizes.as_ref()?;
        (self.root_directory.as_deref() == Some(report.root.as_path())).then_some(report)
    }

    /// Size of one option as measured by the last validation.
    pub(crate) fn option_size_of(
        &self,
        key: crate::models::simulate::SelKey,
    ) -> Option<crate::models::simulate::SizeInfo> {
        self.size_report()?.option_sizes.get(&key).copied()
    }
}
