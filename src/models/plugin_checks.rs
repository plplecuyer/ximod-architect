//! Plugin-level checks of a project (V2 features 4 and 5): the masters a
//! plugin requires that nothing provides or declares, and the light-plugin
//! (ESL) flag against what the file really contains.
//!
//! Language-neutral like `verify.rs`: [`PluginIssue`] carries data, the UI
//! translates it. The same scan feeds the "Plugin report" window through
//! [`collect_plugins`]. File access is bounded to the plugin headers, plus
//! one sequential walk of the record headers for files up to
//! [`MAX_COUNT_FILE_SIZE`].

use std::path::Path;

use super::plugin_header::{EslLimits, EslReport, PluginHeader, PluginKind, count_new_records, read_plugin_header};
use super::verify::RefLoc;
use super::{Dependency, InstallFile, Plugin, Ximod};

/// Plugins larger than this get their header read but not their records
/// counted (a sequential walk of a 64 MB file is still fast, beyond that the
/// verdict is not worth the wait during validation).
pub const MAX_COUNT_FILE_SIZE: u64 = 64 * 1024 * 1024;

/// A finding about a plugin referenced by the project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginIssue {
    /// A master of `plugin` is neither in the mod, nor a base master of the
    /// game, nor declared as a file dependency where the plugin is installed.
    MissingMaster {
        loc: RefLoc,
        plugin: String,
        master: String,
    },
    /// `.esl` extension without the light flag.
    EslFlagMismatch { loc: RefLoc, plugin: String },
    /// An `.esp` without the light flag that could carry it.
    EslEligible {
        loc: RefLoc,
        plugin: String,
        new_forms: u32,
        limit: u32,
    },
    /// Light-flagged, but too many new records or an index out of range.
    EslTooBig {
        loc: RefLoc,
        plugin: String,
        new_forms: u32,
        limit: u32,
    },
}

/// Everything known about one plugin reference, for the report window.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub loc: RefLoc,
    /// Project-relative source path as written in the install file.
    pub source: String,
    pub kind: PluginKind,
    /// `None` when the header could not be read (`error` says why).
    pub header: Option<PluginHeader>,
    /// `None` when the file is unreadable or above [`MAX_COUNT_FILE_SIZE`].
    pub report: Option<EslReport>,
    /// Masters not provided by the mod, not base masters, not declared.
    pub missing_masters: Vec<String>,
    pub error: Option<String>,
}

/// The file name part of a source path (either separator).
pub fn file_name_of(source: &str) -> &str {
    source.rsplit(['/', '\\']).next().unwrap_or(source).trim()
}

/// Whether the project installs a file named `name` (case-insensitive file
/// name, anywhere: required, conditional or option files).
pub fn project_provides(ximod: &Ximod, name: &str) -> bool {
    let is_it = |f: &InstallFile| file_name_of(&f.source).eq_ignore_ascii_case(name);
    ximod.required_files.iter().any(is_it)
        || ximod.conditional_files.iter().any(|c| c.files.iter().any(is_it))
        || ximod
            .steps
            .iter()
            .flat_map(|s| &s.plugin_groups)
            .flat_map(|g| &g.plugins)
            .any(|p| p.files.iter().any(is_it))
}

/// A file dependency that asserts the file is present ("Active" / "Exists").
fn is_present_dep(d: &Dependency, name: &str) -> bool {
    d.dep_type.eq_ignore_ascii_case("file")
        && d.name.trim().eq_ignore_ascii_case(name)
        && (d.value.eq_ignore_ascii_case("Active") || d.value.eq_ignore_ascii_case("Exists"))
}

/// Whether `name` is already declared as a present file where `loc` lives:
/// an option's dependency patterns, or the module dependencies (plus the
/// set's own conditions for a conditional file set).
pub fn declared_at(ximod: &Ximod, loc: &RefLoc, name: &str) -> bool {
    let module = ximod
        .module_dependencies
        .as_ref()
        .is_some_and(|m| m.leaves().any(|d| is_present_dep(d, name)));
    match loc {
        RefLoc::Plugin { step, group, plugin } => step
            .checked_sub(1)
            .and_then(|si| ximod.steps.get(si))
            .and_then(|s| group.checked_sub(1).and_then(|gi| s.plugin_groups.get(gi)))
            .and_then(|g| g.plugins.iter().find(|p| &p.name == plugin))
            .is_some_and(|p| {
                p.dependency_patterns
                    .iter()
                    .any(|pat| pat.condition.leaves().any(|d| is_present_dep(d, name)))
            }),
        RefLoc::ConditionalSet { index } => {
            module
                || index
                    .checked_sub(1)
                    .and_then(|ci| ximod.conditional_files.get(ci))
                    .is_some_and(|c| c.condition.leaves().any(|d| is_present_dep(d, name)))
        }
        RefLoc::RequiredFiles | RefLoc::Header => module,
    }
}

/// The masters of `header` that are neither in the mod, nor base masters,
/// nor declared where the plugin lives.
pub fn missing_masters(ximod: &Ximod, loc: &RefLoc, header: &PluginHeader, base_masters: &[String]) -> Vec<String> {
    header
        .masters
        .iter()
        .filter(|m| !m.trim().is_empty())
        .filter(|m| !base_masters.iter().any(|b| b.eq_ignore_ascii_case(m)))
        .filter(|m| !project_provides(ximod, m))
        .filter(|m| !declared_at(ximod, loc, m))
        .cloned()
        .collect()
}

/// The masters of a freshly added plugin that should become file
/// dependencies of the option: not base masters, not provided by the mod.
/// (Declared ones are filtered by [`add_masters_to_plugin`].)
pub fn masters_to_add(ximod: &Ximod, header: &PluginHeader, base_masters: &[String]) -> Vec<String> {
    header
        .masters
        .iter()
        .filter(|m| !m.trim().is_empty())
        .filter(|m| !base_masters.iter().any(|b| b.eq_ignore_ascii_case(m)))
        .filter(|m| !project_provides(ximod, m))
        .cloned()
        .collect()
}

/// Add `Active` file dependencies on `masters` to `plugin`: in its first
/// dependency pattern typed like the option's default type, or in a new
/// pattern of that type. A master already named by a file dependency of the
/// option (any pattern, any state) is not added again. Returns how many
/// were added.
pub fn add_masters_to_plugin(plugin: &mut Plugin, masters: &[String]) -> usize {
    let wanted: Vec<&String> = masters
        .iter()
        .filter(|m| {
            !plugin.dependency_patterns.iter().any(|pat| {
                pat.condition
                    .leaves()
                    .any(|d| d.dep_type.eq_ignore_ascii_case("file") && d.name.trim().eq_ignore_ascii_case(m))
            })
        })
        .collect();
    if wanted.is_empty() {
        return 0;
    }
    let type_name = plugin.default_type.as_str();
    let index = match plugin
        .dependency_patterns
        .iter()
        .position(|pat| pat.pattern_type == type_name)
    {
        Some(i) => i,
        None => {
            plugin.dependency_patterns.push(super::DependencyPattern {
                pattern_type: type_name.to_string(),
                condition: super::DependencyGroup::default(),
            });
            plugin.dependency_patterns.len() - 1
        }
    };
    let pattern = &mut plugin.dependency_patterns[index];
    let mut added = 0;
    for m in wanted {
        if pattern
            .condition
            .leaves()
            .any(|d| d.dep_type.eq_ignore_ascii_case("file") && d.name.trim().eq_ignore_ascii_case(m))
        {
            continue;
        }
        pattern.condition.push_leaf(Dependency::new_file(m.clone(), "Active"));
        added += 1;
    }
    added
}

/// Every plugin file the project installs, with its location.
fn plugin_refs(ximod: &Ximod) -> Vec<(RefLoc, String)> {
    let mut out = Vec::new();
    let mut push = |loc: RefLoc, f: &InstallFile| {
        if f.file_type == super::FileType::File && PluginKind::from_path(Path::new(f.source.trim())).is_some() {
            out.push((loc, f.source.trim().to_string()));
        }
    };
    for f in &ximod.required_files {
        push(RefLoc::RequiredFiles, f);
    }
    for (i, set) in ximod.conditional_files.iter().enumerate() {
        for f in &set.files {
            push(RefLoc::ConditionalSet { index: i + 1 }, f);
        }
    }
    for (si, step) in ximod.steps.iter().enumerate() {
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            for plugin in &group.plugins {
                for f in &plugin.files {
                    push(
                        RefLoc::Plugin {
                            step: si + 1,
                            group: gi + 1,
                            plugin: plugin.name.clone(),
                        },
                        f,
                    );
                }
            }
        }
    }
    out
}

/// Read every plugin the project references under `root`.
pub fn collect_plugins(ximod: &Ximod, root: &Path, base_masters: &[String], limits: EslLimits) -> Vec<PluginInfo> {
    plugin_refs(ximod)
        .into_iter()
        .map(|(loc, source)| {
            let abs = root.join(source.replace('\\', "/"));
            let kind = PluginKind::from_path(&abs).unwrap_or_default();
            let (header, error) = match read_plugin_header(&abs) {
                Ok(h) => (Some(h), None),
                Err(e) => (None, Some(e.to_string())),
            };
            let report = header.as_ref().and_then(|_| {
                let small = std::fs::metadata(&abs).is_ok_and(|m| m.len() <= MAX_COUNT_FILE_SIZE);
                small.then(|| count_new_records(&abs, limits).ok()).flatten()
            });
            let missing = header
                .as_ref()
                .map(|h| missing_masters(ximod, &loc, h, base_masters))
                .unwrap_or_default();
            PluginInfo {
                loc,
                source,
                kind,
                header,
                report,
                missing_masters: missing,
                error,
            }
        })
        .collect()
}

/// The issues of a scanned plugin.
pub fn issues_of(info: &PluginInfo) -> Vec<PluginIssue> {
    let mut out = Vec::new();
    let Some(header) = &info.header else {
        return out; // unreadable: skipped silently
    };
    let plugin = file_name_of(&info.source).to_string();
    for master in &info.missing_masters {
        out.push(PluginIssue::MissingMaster {
            loc: info.loc.clone(),
            plugin: plugin.clone(),
            master: master.clone(),
        });
    }
    if info.kind == PluginKind::Esl && !header.light {
        out.push(PluginIssue::EslFlagMismatch {
            loc: info.loc.clone(),
            plugin: plugin.clone(),
        });
    }
    if let Some(r) = &info.report {
        if info.kind == PluginKind::Esp && !r.light_flag && r.eligible {
            out.push(PluginIssue::EslEligible {
                loc: info.loc.clone(),
                plugin: plugin.clone(),
                new_forms: r.new_forms,
                limit: r.limit,
            });
        } else if r.light_flag && !r.eligible {
            out.push(PluginIssue::EslTooBig {
                loc: info.loc.clone(),
                plugin,
                new_forms: r.new_forms,
                limit: r.limit,
            });
        }
    }
    out
}

/// Scan every plugin of the project and report the issues found.
pub fn check_plugins(ximod: &Ximod, root: &Path, base_masters: &[String], limits: EslLimits) -> Vec<PluginIssue> {
    collect_plugins(ximod, root, base_masters, limits)
        .iter()
        .flat_map(issues_of)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::plugin_header::tests::{build_plugin, write_plugin};
    use crate::models::{ConditionalFileSet, DependencyGroup, PluginGroup, PluginType, SelectionType, Step};
    use std::path::PathBuf;

    fn root(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod_pchecks_{}_{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn plugin_file(dir: &Path, name: &str, flags: u32, masters: &[&str], body: &[u8]) {
        std::fs::write(dir.join(name), build_plugin(flags, masters, body)).unwrap();
    }

    /// One record new to the plugin (index = number of masters).
    fn one_record(masters: usize, object: u32) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"MISC");
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&(((masters as u32) << 24) | object).to_le_bytes());
        b.extend_from_slice(&[0u8; 8]);
        b
    }

    fn project_with_option(source: &str) -> Ximod {
        let mut x = Ximod::new("T");
        x.game = "skyrimSpecialEdition".into();
        let mut p = Plugin::new("Opt");
        p.files.push(InstallFile::new_file(source));
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        g.plugins.push(p);
        let mut s = Step::new("S");
        s.plugin_groups.push(g);
        x.steps.push(s);
        x
    }

    const BASE: [&str; 2] = ["Skyrim.esm", "Update.esm"];
    fn base() -> Vec<String> {
        BASE.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn file_name_and_project_provides() {
        assert_eq!(file_name_of("plugins/Sub\\Mod.esp"), "Mod.esp");
        assert_eq!(file_name_of("Mod.esp"), "Mod.esp");
        let mut x = project_with_option("plugins/Mod.esp");
        assert!(project_provides(&x, "MOD.ESP"));
        assert!(!project_provides(&x, "Other.esp"));
        x.required_files.push(InstallFile::new_file("Other.esp"));
        assert!(project_provides(&x, "other.esp"));
    }

    #[test]
    fn masters_are_added_once_to_the_default_type_pattern() {
        let mut p = Plugin::new("Opt");
        p.default_type = PluginType::Recommended;
        // A pattern of another type comes first and must be left alone.
        let mut other = crate::models::DependencyPattern::new();
        other.pattern_type = "Required".into();
        p.dependency_patterns.push(other);
        let masters = vec!["Dawnguard.esm".to_string(), "Lib.esm".to_string()];
        assert_eq!(add_masters_to_plugin(&mut p, &masters), 2);
        assert_eq!(p.dependency_patterns.len(), 2);
        assert!(p.dependency_patterns[0].condition.is_empty());
        let pat = &p.dependency_patterns[1];
        assert_eq!(pat.pattern_type, "Recommended");
        assert_eq!(pat.condition.leaf_count(), 2);
        assert_eq!(
            pat.condition.leaves().next().unwrap(),
            &Dependency::new_file("Dawnguard.esm", "Active")
        );
        // Adding again (any case) changes nothing.
        assert_eq!(add_masters_to_plugin(&mut p, &["LIB.ESM".to_string()]), 0);
        assert_eq!(p.dependency_patterns[1].condition.leaf_count(), 2);
    }

    #[test]
    fn masters_to_add_skips_base_and_project_files() {
        let mut x = project_with_option("Mod.esp");
        x.required_files.push(InstallFile::new_file("libs/Lib.esm"));
        let header = PluginHeader {
            kind: Some(PluginKind::Esp),
            masters: vec!["Skyrim.esm".into(), "lib.esm".into(), "Other.esm".into()],
            ..Default::default()
        };
        assert_eq!(masters_to_add(&x, &header, &base()), vec!["Other.esm".to_string()]);
    }

    #[test]
    fn declared_dependencies_count_per_location() {
        let mut x = project_with_option("Mod.esp");
        x.conditional_files.push(ConditionalFileSet::new());
        x.module_dependencies = Some(DependencyGroup::from_leaves(
            crate::models::LogicalOperator::And,
            vec![Dependency::new_file("ModuleDep.esm", "exists")],
        ));
        // Declared inside a nested group: still a declaration.
        let mut nested = DependencyGroup::new(crate::models::LogicalOperator::Or);
        nested.push_leaf(Dependency::new_file("SetDep.esm", "Active"));
        x.conditional_files[0].condition.push_group(nested);
        x.steps[0].plugin_groups[0].plugins[0]
            .dependency_patterns
            .push(crate::models::DependencyPattern {
                pattern_type: "Optional".into(),
                condition: DependencyGroup::from_leaves(
                    crate::models::LogicalOperator::And,
                    vec![
                        Dependency::new_file("OptDep.esm", "Active"),
                        Dependency::new_file("Missing.esm", "Missing"),
                    ],
                ),
            });
        let opt = RefLoc::Plugin {
            step: 1,
            group: 1,
            plugin: "Opt".into(),
        };
        assert!(declared_at(&x, &opt, "optdep.esm"));
        assert!(
            !declared_at(&x, &opt, "Missing.esm"),
            "a 'Missing' state is not a provision"
        );
        assert!(
            !declared_at(&x, &opt, "ModuleDep.esm"),
            "module deps do not cover options"
        );
        assert!(declared_at(&x, &RefLoc::RequiredFiles, "ModuleDep.esm"));
        assert!(!declared_at(&x, &RefLoc::RequiredFiles, "SetDep.esm"));
        let set = RefLoc::ConditionalSet { index: 1 };
        assert!(declared_at(&x, &set, "SetDep.esm"));
        assert!(declared_at(&x, &set, "ModuleDep.esm"));
        assert!(!declared_at(&x, &set, "OptDep.esm"));
    }

    #[test]
    fn check_plugins_reports_masters_and_esl_findings() {
        let dir = root("issues");
        // Option plugin: needs a master nobody provides, and is a small .esp
        // without the light flag → eligible.
        plugin_file(&dir, "Mod.esp", 0, &["Skyrim.esm", "Lib.esm"], &one_record(2, 0x800));
        // Required: .esl extension without the flag.
        plugin_file(&dir, "Patch.esl", 0, &["Skyrim.esm"], &[]);
        // Conditional: light-flagged with an index below 0x800 → too big.
        plugin_file(&dir, "Light.esp", 0x200, &[], &one_record(0, 0x10));
        // A plugin that provides Lib.esm's sibling... and one that is unreadable.
        std::fs::write(dir.join("Broken.esp"), b"nope").unwrap();

        let mut x = project_with_option("Mod.esp");
        x.required_files.push(InstallFile::new_file("Patch.esl"));
        x.required_files.push(InstallFile::new_file("Broken.esp"));
        x.required_files.push(InstallFile::new_file("textures"));
        x.conditional_files.push(ConditionalFileSet::new());
        x.conditional_files[0].files.push(InstallFile::new_file("Light.esp"));

        let issues = check_plugins(&x, &dir, &base(), EslLimits::default());
        let opt = RefLoc::Plugin {
            step: 1,
            group: 1,
            plugin: "Opt".into(),
        };
        assert!(issues.contains(&PluginIssue::MissingMaster {
            loc: opt.clone(),
            plugin: "Mod.esp".into(),
            master: "Lib.esm".into(),
        }));
        assert!(
            !issues
                .iter()
                .any(|i| matches!(i, PluginIssue::MissingMaster { master, .. } if master == "Skyrim.esm"))
        );
        assert!(issues.contains(&PluginIssue::EslEligible {
            loc: opt.clone(),
            plugin: "Mod.esp".into(),
            new_forms: 1,
            limit: 2048,
        }));
        assert!(issues.contains(&PluginIssue::EslFlagMismatch {
            loc: RefLoc::RequiredFiles,
            plugin: "Patch.esl".into(),
        }));
        assert!(issues.contains(&PluginIssue::EslTooBig {
            loc: RefLoc::ConditionalSet { index: 1 },
            plugin: "Light.esp".into(),
            new_forms: 1,
            limit: 2048,
        }));
        // The unreadable plugin is skipped silently; the report lists it.
        let infos = collect_plugins(&x, &dir, &base(), EslLimits::default());
        assert_eq!(infos.len(), 4);
        let broken = infos.iter().find(|i| i.source == "Broken.esp").unwrap();
        assert!(broken.header.is_none() && broken.error.is_some());
        assert!(issues_of(broken).is_empty());

        // Declaring the master, or shipping it, clears the warning.
        let mut declared = x.clone();
        add_masters_to_plugin(
            &mut declared.steps[0].plugin_groups[0].plugins[0],
            &["Lib.esm".to_string()],
        );
        let issues = check_plugins(&declared, &dir, &base(), EslLimits::default());
        assert!(!issues.iter().any(|i| matches!(i, PluginIssue::MissingMaster { .. })));
        let mut shipped = x.clone();
        shipped.required_files.push(InstallFile::new_file("libs/LIB.ESM"));
        let issues = check_plugins(&shipped, &dir, &base(), EslLimits::default());
        assert!(!issues.iter().any(|i| matches!(i, PluginIssue::MissingMaster { .. })));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn scratch_plugins_from_the_header_tests_are_readable() {
        let p = write_plugin("Shared.esp", &build_plugin(0, &["A.esm"], &[]));
        let h = read_plugin_header(&p).unwrap();
        assert_eq!(h.masters, vec!["A.esm".to_string()]);
    }
}
