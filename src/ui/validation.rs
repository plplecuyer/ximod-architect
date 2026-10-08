//! Full validation run and the localisation of validation issues.

use super::main_window::XimodApp;
use crate::models::*;
use eframe::egui;
use fluent::FluentArgs;

impl XimodApp {
    /// Translate a validation error into the current locale.
    /// Maps each `ValidationError` variant to its FTL key and arguments,
    /// keeping the model layer free of any i18n dependency.
    pub(crate) fn translate_validation_error(&self, err: &ValidationError) -> String {
        match err {
            ValidationError::NoName => self.i18n.t("validation-no-name"),
            ValidationError::NoSteps => self.i18n.t("validation-no-steps"),
            ValidationError::EmptyStep { step } => self.i18n.t_num("validation-empty-step", *step as i64),
            ValidationError::EmptyGroup { step, group } => {
                let mut args = FluentArgs::new();
                args.set("step", *step as i64);
                args.set("group", *group as i64);
                self.i18n.t_with_args("validation-empty-group", Some(&args))
            }
            ValidationError::NoPlugins { step, group } => {
                let mut args = FluentArgs::new();
                args.set("step", *step as i64);
                args.set("name", group.clone());
                self.i18n.t_with_args("validation-no-plugins", Some(&args))
            }
        }
    }

    /// Localise a schema validation issue (ModConfig 5.0), prefixed with its
    /// line/column position.
    pub(crate) fn translate_schema_issue(&self, issue: &crate::xml::validate::SchemaIssue) -> String {
        use crate::xml::validate::SchemaIssueKind as K;
        let pair = |k: &str, a: &str, av: String, b: &str, bv: String| {
            let mut args = FluentArgs::new();
            args.set(a, av);
            args.set(b, bv);
            self.i18n.t_with_args(k, Some(&args))
        };
        let msg = match &issue.kind {
            K::WrongRoot { found, expected } => pair(
                "schema-wrong-root",
                "found",
                found.clone(),
                "expected",
                expected.clone(),
            ),
            K::UnknownElement { element, parent } => {
                pair("schema-unknown", "element", element.clone(), "parent", parent.clone())
            }
            K::MissingChild { parent, child } => {
                pair("schema-missing", "parent", parent.clone(), "child", child.clone())
            }
            K::NeedsOne { parent, child } => pair("schema-needs-one", "parent", parent.clone(), "child", child.clone()),
            K::TooMany { parent, child } => pair("schema-too-many", "parent", parent.clone(), "child", child.clone()),
            K::MissingAttr { element, attr } => {
                pair("schema-missing-attr", "element", element.clone(), "attr", attr.clone())
            }
            K::ChooseOne { parent, options } => pair(
                "schema-choose-one",
                "parent",
                parent.clone(),
                "options",
                options.clone(),
            ),
            K::BadEnum {
                element,
                attr,
                value,
                allowed,
            } => {
                let mut args = FluentArgs::new();
                args.set("element", element.clone());
                args.set("attr", attr.clone());
                args.set("value", value.clone());
                args.set("allowed", allowed.clone());
                self.i18n.t_with_args("schema-bad-enum", Some(&args))
            }
        };
        let mut args = FluentArgs::new();
        args.set("line", issue.line as i64);
        args.set("col", issue.column as i64);
        args.set("msg", msg);
        self.i18n.t_with_args("schema-line-col", Some(&args))
    }

    /// Localise the location of an unmodelled construct (`xml::fidelity`).
    pub(crate) fn translate_fidelity_loc(&self, loc: &crate::xml::fidelity::Loc) -> String {
        use crate::xml::fidelity::Loc as L;
        match loc {
            L::Module => self.i18n.t("loc-module"),
            L::Step { step, name } => {
                let mut args = FluentArgs::new();
                args.set("step", *step as i64);
                args.set("name", name.clone());
                self.i18n.t_with_args("loc-step", Some(&args))
            }
            L::Plugin {
                step,
                group,
                plugin_name,
                ..
            } => {
                let mut args = FluentArgs::new();
                args.set("step", *step as i64);
                args.set("group", *group as i64);
                args.set("plugin", plugin_name.clone());
                self.i18n.t_with_args("loc-plugin", Some(&args))
            }
            L::Conditional { index } => self.i18n.t_num("loc-conditional", *index as i64),
            L::Other => self.i18n.t("loc-installer"),
        }
    }

    /// Localise a construct the model cannot keep (`xml::fidelity`).
    pub(crate) fn translate_unmodelled(&self, u: &crate::xml::fidelity::Unmodelled) -> String {
        use crate::xml::fidelity::Unmodelled as U;
        let context = self.translate_fidelity_loc(u.loc());
        let mut args = FluentArgs::new();
        args.set("context", context);
        let key = match u {
            U::UnknownElement { element, parent, .. } => {
                args.set("element", element.clone());
                args.set("parent", parent.clone());
                "fidelity-unknown"
            }
        };
        self.i18n.t_with_args(key, Some(&args))
    }

    /// The node an unmodelled construct concerns, when it can be selected.
    pub(crate) fn target_of_unmodelled(
        &self,
        u: &crate::xml::fidelity::Unmodelled,
    ) -> Option<crate::ui::problems::Target> {
        use crate::ui::problems::Target;
        use crate::xml::fidelity::Loc as L;
        match u.loc() {
            L::Module => Some(Target::Info),
            L::Step { step, .. } => {
                let si = step.checked_sub(1)?;
                self.ximod.steps.get(si).map(|_| Target::Step(si))
            }
            L::Plugin {
                step, group, plugin, ..
            } => {
                let si = step.checked_sub(1)?;
                let gi = group.checked_sub(1)?;
                self.ximod
                    .steps
                    .get(si)?
                    .plugin_groups
                    .get(gi)?
                    .plugins
                    .get(*plugin)
                    .map(|_| Target::Plugin(si, gi, *plugin))
            }
            L::Conditional { index } => {
                let ci = index.checked_sub(1)?;
                self.ximod.conditional_files.get(ci).map(|_| Target::CondSet(ci))
            }
            L::Other => None,
        }
    }

    /// The Problems entries for the active document's import report: a
    /// leading summary line, then one warning per dropped construct. Empty
    /// when the FOMOD loaded faithfully.
    pub(crate) fn import_issues(&self) -> Vec<crate::ui::problems::Issue> {
        use crate::ui::problems::{Issue, Severity};
        if self.import_report.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(self.import_report.len() + 1);
        out.push(Issue::new(
            Severity::Warning,
            self.i18n.t_num("msg-import-lossy", self.import_report.len() as i64),
            None,
        ));
        for u in &self.import_report {
            out.push(Issue::new(
                Severity::Warning,
                self.translate_unmodelled(u),
                self.target_of_unmodelled(u),
            ));
        }
        out
    }

    /// Run the full validation (project + ModuleConfig/info schema) and open the
    /// report window.
    pub(crate) fn run_full_validation(&mut self, ctx: &egui::Context) {
        use crate::ui::problems::{Issue, Severity};
        // What the import dropped stays reported until the file is reloaded.
        let mut found: Vec<Issue> = self.import_issues();
        // Project-level checks.
        for err in self.ximod.validate() {
            let target = self.target_of_validation_error(&err);
            found.push(Issue::new(
                Severity::Error,
                self.translate_validation_error(&err),
                target,
            ));
        }
        // ModuleConfig.xml schema.
        if let Ok(xml) = crate::xml::module_config_to_string(&self.ximod) {
            for issue in crate::xml::validate::validate_module_config(&xml) {
                found.push(Issue::new(Severity::Error, self.translate_schema_issue(&issue), None));
            }
        }
        // info.xml schema.
        if let Ok(xml) = crate::xml::info_xml_to_string(&self.ximod) {
            for issue in crate::xml::validate::validate_info(&xml) {
                found.push(Issue::new(Severity::Error, self.translate_schema_issue(&issue), None));
            }
        }
        // Steps, options and conditional sets no selection can reach.
        for u in crate::models::simulate::unreachable(&self.ximod) {
            found.push(self.unreachable_issue(&u));
        }
        // Flag wiring: values tested but never set, flags set but never tested.
        found.extend(self.flag_issues());
        self.validation_issues = found;
        self.show_problems = true;
        self.problems_expanded = true;
        // Sizes are measured again by the worker below.
        self.sizes = None;
        // Referenced-file verification, destination conflicts and image
        // checks all walk the mod folder on disk: they run on a worker thread
        // (see `ui/jobs.rs`) and are appended to the report when ready.
        // Skipped with a note when no root is set.
        match self.root_directory.clone() {
            Some(root) => self.spawn_disk_validation(ctx, root),
            None => {
                self.validation_rx = None;
                self.validation_issues.push(crate::ui::problems::Issue::new(
                    crate::ui::problems::Severity::Info,
                    self.i18n.t("verify-no-root"),
                    None,
                ));
            }
        }
    }

    /// Flag findings (shared analysis with the condition editor, see
    /// `models::flags`): a Warning per condition that tests a flag value no
    /// option sets, pointing at the node holding the condition, and an Info
    /// per flag that is set but never tested anywhere, pointing at the first
    /// option setting it.
    pub(crate) fn flag_issues(&self) -> Vec<crate::ui::problems::Issue> {
        use crate::models::flags::analyze_flags;
        use crate::ui::condition_editor::target_of;
        use crate::ui::problems::{Issue, Severity, Target};
        let mut out = Vec::new();
        for flag in &analyze_flags(&self.ximod).flags {
            let mut seen: std::collections::HashSet<(Target, &str)> = std::collections::HashSet::new();
            for u in flag.users_of_unset_values() {
                let target = target_of(u.site);
                if !seen.insert((target, u.value.as_str())) {
                    continue;
                }
                let mut args = FluentArgs::new();
                args.set("flag", flag.name.clone());
                args.set("value", u.value.clone());
                out.push(Issue::new(
                    Severity::Warning,
                    self.i18n.t_with_args("issue-flag-value-never-set", Some(&args)),
                    Some(target),
                ));
            }
            if flag.users.is_empty()
                && let Some(first) = flag.setters.first()
            {
                out.push(Issue::new(
                    Severity::Info,
                    self.i18n.t_arg("issue-flag-never-used", "flag", &flag.name),
                    Some(target_of(first.site)),
                ));
            }
        }
        out
    }

    /// The Problems entry for an unreachable part of the installer.
    pub(crate) fn unreachable_issue(&self, u: &crate::models::simulate::Unreachable) -> crate::ui::problems::Issue {
        use crate::models::simulate::Unreachable as U;
        use crate::ui::problems::{Issue, Severity, Target};
        let (message, target) = match u {
            U::Step { index } => {
                let name = match self.ximod.steps.get(*index) {
                    Some(s) if !s.name.trim().is_empty() => s.name.clone(),
                    _ => crate::ui::labels::step_fallback(&self.i18n, *index),
                };
                (
                    self.i18n.t_arg("issue-unreachable-step", "step", &name),
                    Some(Target::Step(*index)),
                )
            }
            U::Option { step, group, plugin } => {
                let name = self
                    .ximod
                    .steps
                    .get(*step)
                    .and_then(|s| s.plugin_groups.get(*group))
                    .and_then(|g| g.plugins.get(*plugin))
                    .map(|p| p.name.clone())
                    .unwrap_or_default();
                (
                    self.i18n.t_arg("issue-unreachable-option", "plugin", &name),
                    Some(Target::Plugin(*step, *group, *plugin)),
                )
            }
            U::ConditionalSet { index } => (
                self.i18n.t_num("issue-unreachable-cond", *index as i64 + 1),
                Some(Target::CondSet(*index)),
            ),
        };
        Issue::new(Severity::Info, message, target)
    }

    /// Localise an image validation issue for `rel` (project-relative path).
    pub(crate) fn translate_image_issue(&self, rel: &str, issue: &crate::media::ImageIssue) -> String {
        use crate::media::ImageIssue as I;
        match issue {
            I::TooLarge { width, height } => {
                let mut args = FluentArgs::new();
                args.set("path", rel.to_string());
                args.set("width", *width as i64);
                args.set("height", *height as i64);
                self.i18n.t_with_args("verify-image-large", Some(&args))
            }
            I::UnsupportedFormat { ext } => {
                let mut args = FluentArgs::new();
                args.set("path", rel.to_string());
                args.set("ext", ext.clone());
                self.i18n.t_with_args("verify-image-format", Some(&args))
            }
            I::Unreadable => self.i18n.t_arg("verify-image-unreadable", "path", rel),
        }
    }

    /// Localise a destination conflict into a single report line.
    pub(crate) fn translate_conflict(&self, c: &crate::models::conflicts::Conflict) -> String {
        let locs: Vec<String> = c.sources.iter().map(|s| self.file_ref_location(&s.loc)).collect();
        let mut args = FluentArgs::new();
        args.set("path", c.destination.clone());
        args.set("count", c.sources.len() as i64);
        args.set("locs", locs.join(", "));
        use crate::models::conflicts::ConflictKind;
        let key = match (c.kind, c.certain) {
            (ConflictKind::ArchiveVsLoose, _) => "issue-conflict-archive-loose",
            (ConflictKind::ArchiveVsArchive, _) => "issue-conflict-archive",
            (ConflictKind::Loose, true) => "conflict-certain",
            (ConflictKind::Loose, false) => "conflict-potential",
        };
        self.i18n.t_with_args(key, Some(&args))
    }

    /// Build a human context string for a referenced-file location.
    pub(crate) fn file_ref_location(&self, loc: &crate::models::verify::RefLoc) -> String {
        use crate::models::verify::RefLoc as L;
        match loc {
            L::Header => self.i18n.t("loc-header"),
            L::RequiredFiles => self.i18n.t("loc-required"),
            L::ConditionalSet { index } => self.i18n.t_num("loc-conditional", *index as i64),
            L::Plugin { step, group, plugin } => {
                let mut args = FluentArgs::new();
                args.set("step", *step as i64);
                args.set("group", *group as i64);
                args.set("plugin", plugin.clone());
                self.i18n.t_with_args("loc-plugin", Some(&args))
            }
        }
    }

    /// Localise a plugin finding (missing master, light-plugin flag).
    pub(crate) fn translate_plugin_issue(&self, issue: &crate::models::plugin_checks::PluginIssue) -> String {
        use crate::models::plugin_checks::PluginIssue as P;
        let mut args = FluentArgs::new();
        let key = match issue {
            P::MissingMaster { plugin, master, .. } => {
                args.set("plugin", plugin.clone());
                args.set("master", master.clone());
                "issue-missing-master"
            }
            P::EslFlagMismatch { plugin, .. } => {
                args.set("plugin", plugin.clone());
                "issue-esl-mismatch-flag"
            }
            P::EslEligible {
                plugin,
                new_forms,
                limit,
                ..
            } => {
                args.set("plugin", plugin.clone());
                args.set("num", *new_forms as i64);
                args.set("limit", *limit as i64);
                "issue-esl-eligible"
            }
            P::EslTooBig {
                plugin,
                new_forms,
                limit,
                ..
            } => {
                args.set("plugin", plugin.clone());
                args.set("num", *new_forms as i64);
                args.set("limit", *limit as i64);
                "issue-esl-too-big"
            }
        };
        self.i18n.t_with_args(key, Some(&args))
    }

    /// Localise a referenced-file issue.
    pub(crate) fn translate_file_issue(&self, issue: &crate::models::verify::FileIssue) -> String {
        use crate::models::verify::FileIssue as F;
        let with = |key: &str, loc: &crate::models::verify::RefLoc, path: &str| {
            let mut args = FluentArgs::new();
            args.set("loc", self.file_ref_location(loc));
            args.set("path", path.to_string());
            self.i18n.t_with_args(key, Some(&args))
        };
        match issue {
            F::MissingSource { loc, path, folder } => with(
                if *folder {
                    "verify-missing-folder"
                } else {
                    "verify-missing-file"
                },
                loc,
                path,
            ),
            F::MissingImage { loc, path } => with("verify-missing-image", loc, path),
            F::AbsolutePath { loc, path } => with("verify-absolute", loc, path),
            F::OutsideRoot { loc, path } => with("verify-outside", loc, path),
            F::OrphanFile { path } => self.i18n.t_arg("verify-orphan", "path", path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::fidelity::{Loc, Unmodelled};

    fn app(locale: &str) -> XimodApp {
        let mut app = XimodApp::default();
        app.i18n.set_locale(locale);
        app
    }

    /// Every fidelity / archive message resolves (no raw key leaks) in the
    /// two reference locales, with its arguments filled in.
    #[test]
    fn fidelity_and_archive_messages_render_in_eng_and_fra() {
        for locale in ["eng", "fra"] {
            let app = app(locale);
            let items = [
                Unmodelled::UnknownElement {
                    element: "odd".into(),
                    parent: "dependencies".into(),
                    loc: Loc::Conditional { index: 3 },
                },
                Unmodelled::UnknownElement {
                    element: "weird".into(),
                    parent: "moduleDependencies".into(),
                    loc: Loc::Module,
                },
                Unmodelled::UnknownElement {
                    element: "strange".into(),
                    parent: "visible".into(),
                    loc: Loc::Step {
                        step: 2,
                        name: "Textures".into(),
                    },
                },
                Unmodelled::UnknownElement {
                    element: "bogus".into(),
                    parent: "plugin".into(),
                    loc: Loc::Plugin {
                        step: 1,
                        group: 2,
                        plugin: 0,
                        plugin_name: "P".into(),
                    },
                },
            ];
            for u in &items {
                let text = app.translate_unmodelled(u);
                assert!(!text.starts_with("fidelity-"), "{locale}: raw key {text}");
                assert!(!text.contains("loc-"), "{locale}: raw loc key {text}");
            }
            assert!(app.translate_unmodelled(&items[1]).contains("weird"));
            assert!(app.translate_unmodelled(&items[2]).contains("Textures"));
            assert!(app.translate_unmodelled(&items[3]).contains("bogus"));
            let e = crate::archive_open::ArchiveError::UnsupportedArchive { ext: "rar".into() };
            let text = app.translate_archive_error(&e);
            assert!(text.contains(".rar") && text.contains("7-Zip"), "{locale}: {text}");
            assert!(!app.i18n.t("msg-import-lossy").starts_with("msg-"));
        }
    }

    /// Plugin findings render in both reference locales with their arguments.
    #[test]
    fn plugin_issue_messages_render_in_eng_and_fra() {
        use crate::models::plugin_checks::PluginIssue as P;
        use crate::models::verify::RefLoc;
        for locale in ["eng", "fra"] {
            let app = app(locale);
            let loc = RefLoc::RequiredFiles;
            let items = [
                P::MissingMaster {
                    loc: loc.clone(),
                    plugin: "Mod.esp".into(),
                    master: "Lib.esm".into(),
                },
                P::EslFlagMismatch {
                    loc: loc.clone(),
                    plugin: "Patch.esl".into(),
                },
                P::EslEligible {
                    loc: loc.clone(),
                    plugin: "Small.esp".into(),
                    new_forms: 12,
                    limit: 2048,
                },
                P::EslTooBig {
                    loc,
                    plugin: "Huge.esp".into(),
                    new_forms: 5000,
                    limit: 4096,
                },
            ];
            for it in &items {
                let text = app.translate_plugin_issue(it);
                assert!(!text.starts_with("issue-"), "{locale}: raw key {text}");
            }
            assert!(app.translate_plugin_issue(&items[0]).contains("Lib.esm"));
            assert!(app.translate_plugin_issue(&items[2]).contains("12"));
            assert!(app.translate_plugin_issue(&items[3]).contains("4096"));
        }
    }

    /// Unreachable findings are Info issues pointing at their node.
    #[test]
    fn unreachable_issues_render_in_eng_and_fra() {
        use crate::models::simulate::Unreachable as U;
        use crate::ui::problems::{Severity, Target};
        for locale in ["eng", "fra"] {
            let mut app = app(locale);
            let mut step = Step::new("Never");
            step.visibility.push_leaf(Dependency::new_flag("x", "1"));
            let mut group = PluginGroup::new("G", SelectionType::SelectAny);
            group.plugins.push(Plugin::new("Dead"));
            step.plugin_groups.push(group);
            app.ximod.steps.push(step);
            let items = [
                U::Step { index: 0 },
                U::Option {
                    step: 0,
                    group: 0,
                    plugin: 0,
                },
                U::ConditionalSet { index: 2 },
            ];
            let issues: Vec<_> = items.iter().map(|u| app.unreachable_issue(u)).collect();
            for i in &issues {
                assert_eq!(i.severity, Severity::Info);
                assert!(!i.message.starts_with("issue-"), "{locale}: {}", i.message);
            }
            assert!(issues[0].message.contains("Never"));
            assert_eq!(issues[0].target, Some(Target::Step(0)));
            assert!(issues[1].message.contains("Dead"));
            assert_eq!(issues[1].target, Some(Target::Plugin(0, 0, 0)));
            assert!(issues[2].message.contains('3'));
            assert_eq!(issues[2].target, Some(Target::CondSet(2)));
            // The full validation reports the unreachable step.
            let ctx = egui::Context::default();
            app.run_full_validation(&ctx);
            assert!(app.validation_issues.iter().any(|i| i.target == Some(Target::Step(0))));
        }
    }

    /// Targets resolve only when the node exists in the model.
    #[test]
    fn unmodelled_targets_follow_the_model() {
        use crate::ui::problems::Target;
        let mut app = app("eng");
        let mut step = Step::new("S");
        let mut group = PluginGroup::new("G", SelectionType::SelectAny);
        group.plugins.push(Plugin::new("P"));
        step.plugin_groups.push(group);
        app.ximod.steps.push(step);
        let at = |loc: Loc| Unmodelled::UnknownElement {
            element: "x".into(),
            parent: "y".into(),
            loc,
        };
        assert_eq!(app.target_of_unmodelled(&at(Loc::Module)), Some(Target::Info));
        assert_eq!(
            app.target_of_unmodelled(&at(Loc::Step {
                step: 1,
                name: "S".into()
            })),
            Some(Target::Step(0))
        );
        assert_eq!(
            app.target_of_unmodelled(&at(Loc::Step {
                step: 2,
                name: "x".into()
            })),
            None
        );
        assert_eq!(
            app.target_of_unmodelled(&at(Loc::Plugin {
                step: 1,
                group: 1,
                plugin: 0,
                plugin_name: "P".into()
            })),
            Some(Target::Plugin(0, 0, 0))
        );
        assert_eq!(app.target_of_unmodelled(&at(Loc::Conditional { index: 1 })), None);
        assert_eq!(app.target_of_unmodelled(&at(Loc::Other)), None);
    }
}
