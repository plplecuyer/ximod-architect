//! CLI extensions — batch build from a manifest (V2 roadmap, priority 11).
//!
//! V1 exposes `validate` / `build` / `package` for a single root. The `batch`
//! command reads a manifest listing several FOMOD roots (plus per-entry options
//! such as output path and archive format) and validates, builds and packages
//! each — the basis for a CI step that runs on every change.
//!
//! Manifest (JSON):
//! ```json
//! {
//!   "entries": [
//!     { "root": "mods/armory",  "format": "7z", "out": "dist/armory.7z" },
//!     { "root": "mods/aurelia", "format": "zip",
//!       "translations": [
//!         { "sidecar": "fomod/translations/Aurelia.fra.ximod-translation" },
//!         { "sidecar": "tr/aurelia.deu.ximod-translation", "mode": "package",
//!           "format": "7z", "full": true, "out": "dist/Aurelia_DE.7z" },
//!         { "sidecar": "tr/aurelia.spa.ximod-translation", "mode": "sibling" }
//!       ] }
//!   ]
//! }
//! ```
//!
//! `scenarios` (optional) names simulator scenarios saved under
//! `<root>/fomod/scenarios/` (or `.json` files); each is replayed and the
//! entry fails when a scenario names a step, group or option that no longer
//! exists — the regression check for a renamed option.
//!
//! `translations` (optional) lists translation sidecars to export once the mod
//! is packaged: as an archive (`"package"`, the default: a patch holding the
//! translated `fomod` XML files, or the whole translated mod with `"full"`),
//! next to the original (`"sibling"`: `<root>/fomod_<lang>/`) or over it
//! (`"inplace"`). A sidecar path that does not exist as given is looked up
//! under the entry's root.
//!
//! Exit code is 0 when every entry succeeds, non-zero otherwise (fails a CI job).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::archive::{ArchiveFormat, CompressionLevel, export_archive};
use crate::cli_translate::{
    ApplyMode, ApplyOptions, PackageKind, PackageOptions, apply_translation, default_package_path,
    package_translation_excluding,
};
use crate::export::default_archive_name_with_ext;
use crate::models::translate::TranslationDoc;
use crate::xml;

/// One FOMOD to process in a batch run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchEntry {
    /// Mod root directory (contains or will contain `fomod/`).
    pub root: String,
    /// Archive format: `"zip"` (default) or `"7z"`.
    #[serde(default)]
    pub format: Option<String>,
    /// Explicit output path; defaulted from the project name when absent.
    #[serde(default)]
    pub out: Option<String>,
    /// Translations to export after the mod is packaged.
    #[serde(default)]
    pub translations: Vec<BatchTranslation>,
    /// Simulator scenarios to replay (names under `fomod/scenarios/`, or
    /// `.json` files); unresolved names fail the entry.
    #[serde(default)]
    pub scenarios: Vec<String>,
}

/// One translation of a batch entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchTranslation {
    /// Translation sidecar (`*.ximod-translation`); looked up as given, then
    /// under the entry's root.
    pub sidecar: String,
    /// `"package"` (default), `"sibling"` or `"inplace"`.
    #[serde(default)]
    pub mode: Option<String>,
    /// Archive to write in package mode; defaulted next to the mod folder,
    /// named after the template remembered in the sidecar.
    #[serde(default)]
    pub out: Option<String>,
    /// Archive format in package mode: `"zip"` (default) or `"7z"`.
    #[serde(default)]
    pub format: Option<String>,
    /// Package the whole translated mod instead of the XML files only.
    #[serde(default)]
    pub full: bool,
}

/// The batch manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchManifest {
    /// FOMODs to process, in order.
    pub entries: Vec<BatchEntry>,
}

/// Parse an archive-format string (case-insensitive). Defaults to ZIP.
fn parse_format(s: Option<&str>) -> ArchiveFormat {
    match s.map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("7z") | Some("sevenzip") | Some("7zip") => ArchiveFormat::SevenZip,
        _ => ArchiveFormat::Zip,
    }
}

/// Process one entry: load → validate → package. Returns `Ok(output_path)`.
fn process_entry(entry: &BatchEntry) -> Result<String, String> {
    let root = Path::new(&entry.root);
    let ximod = xml::load_ximod(root).map_err(|e| format!("load: {e}"))?;

    // Non-fatal validation: report count but keep going (packaging still useful).
    let errors = ximod.validate();
    if !errors.is_empty() {
        eprintln!("  warning: {} validation issue(s)", errors.len());
    }

    let format = parse_format(entry.format.as_deref());
    let out = match &entry.out {
        Some(p) => PathBuf::from(p),
        None => root.join(default_archive_name_with_ext(&ximod, format.extension())),
    };
    let n =
        export_archive(&ximod, root, &out, format, CompressionLevel::Normal).map_err(|e| format!("package: {e}"))?;
    let mut desc = format!("{} ({} files)", out.display(), n);

    // Saved scenarios: replayed before packaging, so a renamed option breaks
    // the build instead of silently changing what the scenario installs.
    for name in &entry.scenarios {
        let report = simulate_report(root, &ximod, Some(name)).map_err(|e| format!("scenario {name}: {e}"))?;
        if !report.unresolved.is_empty() {
            return Err(format!(
                "scenario {name}: {} unresolved name(s): {}",
                report.unresolved.len(),
                report.unresolved.join("; ")
            ));
        }
        desc.push_str(&format!(
            "\n  ok -> scenario {name}: {} file(s), {}",
            report.install.len(),
            crate::models::simulate::format_size(report.total_size)
        ));
    }

    // Archives written so far, kept out of a full translation package when
    // they sit inside the mod folder.
    let mut archives = vec![out];
    for translation in &entry.translations {
        let done = process_translation(root, translation, &mut archives)
            .map_err(|e| format!("translation {}: {e}", translation.sidecar))?;
        desc.push_str("\n  ok -> ");
        desc.push_str(&done);
    }
    Ok(desc)
}

/// Export one translation of an entry. Returns a description of the result.
fn process_translation(
    root: &Path,
    translation: &BatchTranslation,
    archives: &mut Vec<PathBuf>,
) -> Result<String, String> {
    let mut sidecar = PathBuf::from(&translation.sidecar);
    if !sidecar.is_file() && sidecar.is_relative() && root.join(&sidecar).is_file() {
        sidecar = root.join(&sidecar);
    }
    let doc = TranslationDoc::load(&sidecar).map_err(|e| format!("{e:#}"))?;

    let mode = translation.mode.as_deref().map(|m| m.trim().to_ascii_lowercase());
    let apply_mode = match mode.as_deref() {
        None | Some("") | Some("package") => None,
        Some("sibling") => Some(ApplyMode::Sibling),
        Some("inplace") | Some("in-place") => Some(ApplyMode::InPlace),
        Some(other) => {
            return Err(format!("unknown mode '{other}' (package, sibling or inplace)"));
        }
    };

    if let Some(mode) = apply_mode {
        let opts = ApplyOptions {
            mode,
            force_explicit_order: doc.export.force_explicit_order,
            force: false,
        };
        let outcome = apply_translation(root, &doc, &opts).map_err(|e| format!("{e:#}"))?;
        let written: Vec<String> = outcome.written.iter().map(|p| p.display().to_string()).collect();
        return Ok(format!(
            "{} [{}]: {} translation(s) applied, wrote {}",
            doc.target_lang,
            if mode == ApplyMode::Sibling {
                "sibling"
            } else {
                "inplace"
            },
            outcome.applied,
            if written.is_empty() {
                "nothing (already up to date)".to_string()
            } else {
                written.join(", ")
            }
        ));
    }

    let format = parse_format(translation.format.as_deref());
    let out = match &translation.out {
        Some(p) => PathBuf::from(p),
        None => default_package_path(root, &doc, &doc.export.name_template, format),
    };
    let opts = PackageOptions {
        kind: if translation.full {
            PackageKind::Full
        } else {
            PackageKind::PatchOnly
        },
        format,
        out,
        force_explicit_order: doc.export.force_explicit_order,
        readme: None,
    };
    let outcome = package_translation_excluding(root, &doc, &opts, archives, &mut |_, _, _| true)
        .map_err(|e| format!("{e:#}"))?;
    archives.push(outcome.path.clone());
    let mut desc = format!(
        "{} [{}]: {} ({} files, {} translation(s))",
        doc.target_lang,
        if translation.full {
            "full package"
        } else {
            "patch package"
        },
        outcome.path.display(),
        outcome.files,
        outcome.applied
    );
    if outcome.stale > 0 {
        desc.push_str(&format!(", {} stale string(s) skipped", outcome.stale));
    }
    Ok(desc)
}

// ---------------------------------------------------------------------------
// Simulation report (`simulate` command, batch scenarios)
// ---------------------------------------------------------------------------

/// One selected option of a simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimSelection {
    pub step: String,
    pub group: String,
    pub plugin: String,
}

/// One installed file of a simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimFile {
    /// Final path under the game's Data folder.
    pub path: String,
    pub source: String,
    pub from: String,
    pub priority: u32,
    pub size: u64,
    pub missing: bool,
    /// Sources overwritten at this path (`source ← from`).
    pub overwrites: Vec<String>,
}

/// The report of one simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationReport {
    pub name: String,
    pub version: String,
    /// Scenario replayed, or `None` for the default selections.
    pub scenario: Option<String>,
    pub unresolved: Vec<String>,
    pub file_states: std::collections::BTreeMap<String, crate::models::FileState>,
    pub visible_steps: Vec<String>,
    pub selections: Vec<SimSelection>,
    pub install: Vec<SimFile>,
    pub total_size: u64,
    pub files: usize,
    pub overwrites: usize,
    pub truncated: bool,
    pub missing: Vec<String>,
}

/// Describe where an install entry comes from.
fn loc_text(loc: &crate::models::verify::RefLoc) -> String {
    use crate::models::verify::RefLoc as L;
    match loc {
        L::Header => "header".into(),
        L::RequiredFiles => "required files".into(),
        L::ConditionalSet { index } => format!("conditional set {index}"),
        L::Plugin { step, group, plugin } => format!("step {step} / group {group} / {plugin}"),
    }
}

/// Flatten the final tree into the report's file list.
fn flatten_tree(node: &crate::models::simulate::TreeNode, out: &mut Vec<SimFile>) {
    if let Some(src) = &node.source {
        out.push(SimFile {
            path: node.path.clone(),
            source: src.source.clone(),
            from: loc_text(&src.loc),
            priority: src.priority,
            size: node.size,
            missing: node.missing,
            overwrites: node
                .overwritten
                .iter()
                .map(|o| format!("{} ← {}", o.source, loc_text(&o.loc)))
                .collect(),
        });
    }
    for child in node.children.values() {
        flatten_tree(child, out);
    }
}

/// Replay the installer of `ximod` under `root` with the default selections
/// or the named scenario (a saved name or a `.json` path).
pub fn simulate_report(
    root: &Path,
    ximod: &crate::models::Ximod,
    scenario: Option<&str>,
) -> anyhow::Result<SimulationReport> {
    use crate::models::simulate as sim;
    let (selections, file_states, unresolved, scenario_name) = match scenario {
        Some(name) => {
            let sc = sim::load_scenario(root, name)?;
            let (sel, fs, unresolved) = sc.apply(ximod)?;
            (sel, fs, unresolved, Some(sc.name))
        }
        None => {
            let fs = sim::referenced_files(ximod);
            (sim::default_selections(ximod, &fs), fs, Vec::new(), None)
        }
    };
    let ev = sim::evaluate(ximod, &selections, &file_states);
    let entries = sim::compute_entries(ximod, &selections, &file_states);
    let tree = sim::build_install_tree(root, &entries);
    let mut install = Vec::new();
    flatten_tree(&tree.root, &mut install);
    let mut sel_out = Vec::new();
    for &si in &ev.visible_steps {
        let step = &ximod.steps[si];
        for (gi, g) in step.plugin_groups.iter().enumerate() {
            for (pi, p) in g.plugins.iter().enumerate() {
                if *selections.get(&(si, gi, pi)).unwrap_or(&false) {
                    sel_out.push(SimSelection {
                        step: step.name.clone(),
                        group: g.name.clone(),
                        plugin: p.name.clone(),
                    });
                }
            }
        }
    }
    Ok(SimulationReport {
        name: ximod.name.clone(),
        version: ximod.version.clone(),
        scenario: scenario_name,
        unresolved,
        file_states,
        visible_steps: ev
            .visible_steps
            .iter()
            .map(|&si| ximod.steps[si].name.clone())
            .collect(),
        selections: sel_out,
        install,
        total_size: tree.total_size,
        files: tree.files,
        overwrites: tree.overwrites,
        truncated: tree.truncated,
        missing: tree.missing,
    })
}

/// Plain-text rendering of a simulation report.
pub fn format_simulation(r: &SimulationReport) -> String {
    use crate::models::simulate::format_size;
    use std::fmt::Write;
    let mut out = String::new();
    let _ = writeln!(out, "{} {}", r.name, r.version);
    match &r.scenario {
        Some(s) => {
            let _ = writeln!(out, "Scenario: {s}");
        }
        None => {
            let _ = writeln!(out, "Scenario: (default selections)");
        }
    }
    if !r.unresolved.is_empty() {
        let _ = writeln!(out, "Unresolved names ({}):", r.unresolved.len());
        for u in &r.unresolved {
            let _ = writeln!(out, "  ! {u}");
        }
    }
    if !r.file_states.is_empty() {
        let _ = writeln!(out, "File assumptions:");
        for (f, st) in &r.file_states {
            let _ = writeln!(out, "  {f}: {}", st.as_str());
        }
    }
    let _ = writeln!(out, "Visible steps ({}):", r.visible_steps.len());
    for s in &r.visible_steps {
        let _ = writeln!(out, "  - {s}");
    }
    let _ = writeln!(out, "Selections ({}):", r.selections.len());
    for s in &r.selections {
        let _ = writeln!(out, "  - {} / {} / {}", s.step, s.group, s.plugin);
    }
    let _ = writeln!(out, "Install ({} files):", r.install.len());
    for f in &r.install {
        let _ = writeln!(
            out,
            "  {}  [{}]  <- {} ({}, priority {}){}",
            f.path,
            if f.missing {
                "MISSING".to_string()
            } else {
                format_size(f.size)
            },
            f.source,
            f.from,
            f.priority,
            if f.overwrites.is_empty() {
                String::new()
            } else {
                format!("  overwrites: {}", f.overwrites.join(", "))
            }
        );
    }
    if !r.missing.is_empty() {
        let _ = writeln!(out, "Missing sources ({}):", r.missing.len());
        for m in &r.missing {
            let _ = writeln!(out, "  ! {m}");
        }
    }
    if r.truncated {
        let _ = writeln!(out, "(file list truncated)");
    }
    let _ = writeln!(
        out,
        "Total: {} in {} file(s), {} overwrite(s)",
        format_size(r.total_size),
        r.files,
        r.overwrites
    );
    out
}

/// Run a batch from a manifest file. Returns a process exit code (0 = all OK).
pub fn run_batch(manifest: &Path) -> i32 {
    let text = match std::fs::read_to_string(manifest) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("batch: cannot read {}: {e}", manifest.display());
            return 2;
        }
    };
    let parsed: BatchManifest = match serde_json::from_str(&text) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("batch: invalid manifest: {e}");
            return 2;
        }
    };

    let mut failures = 0;
    for (i, entry) in parsed.entries.iter().enumerate() {
        println!("[{}/{}] {}", i + 1, parsed.entries.len(), entry.root);
        match process_entry(entry) {
            Ok(desc) => println!("  ok -> {desc}"),
            Err(e) => {
                eprintln!("  FAILED: {e}");
                failures += 1;
            }
        }
    }
    println!("batch: {} ok, {} failed", parsed.entries.len() - failures, failures);
    if failures == 0 { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_parses() {
        let json = r#"{"entries":[{"root":"a"},{"root":"b","format":"7z","out":"b.7z"}]}"#;
        let m: BatchManifest = serde_json::from_str(json).unwrap();
        assert_eq!(m.entries.len(), 2);
        assert_eq!(m.entries[1].format.as_deref(), Some("7z"));
    }

    #[test]
    fn parse_format_defaults_to_zip() {
        assert_eq!(parse_format(None), ArchiveFormat::Zip);
        assert_eq!(parse_format(Some("ZIP")), ArchiveFormat::Zip);
        assert_eq!(parse_format(Some("7z")), ArchiveFormat::SevenZip);
    }

    #[test]
    fn batch_packages_a_minimal_fomod() {
        use std::fs;
        let dir = std::env::temp_dir().join(format!("ximod_batch_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let root = dir.join("mod_a");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("Plugin.esp"), b"x").unwrap();

        // Build a valid minimal project so load_ximod works.
        let mut x = crate::models::Ximod::new("BatchMod");
        x.required_files
            .push(crate::models::InstallFile::new_file("Plugin.esp"));
        xml::save_ximod(&x, &root).unwrap();

        let manifest = dir.join("batch.json");
        let out = dir.join("out.zip");
        let m = format!(
            r#"{{"entries":[{{"root":{:?},"format":"zip","out":{:?}}}]}}"#,
            root.to_string_lossy(),
            out.to_string_lossy()
        );
        fs::write(&manifest, m).unwrap();

        let code = run_batch(&manifest);
        assert_eq!(code, 0);
        assert!(out.is_file());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn manifest_with_translations_parses() {
        let json = r#"{"entries":[
            {"root":"a"},
            {"root":"b","format":"7z","translations":[
                {"sidecar":"b.fra.ximod-translation"},
                {"sidecar":"b.deu.ximod-translation","mode":"sibling"},
                {"sidecar":"b.spa.ximod-translation","mode":"package","out":"b_ES.7z","format":"7z","full":true}
            ]}
        ]}"#;
        let m: BatchManifest = serde_json::from_str(json).unwrap();
        assert!(m.entries[0].translations.is_empty(), "old manifests still parse");
        let t = &m.entries[1].translations;
        assert_eq!(t.len(), 3);
        assert_eq!(
            (
                t[0].mode.as_deref(),
                t[0].out.as_deref(),
                t[0].format.as_deref(),
                t[0].full
            ),
            (None, None, None, false)
        );
        assert_eq!(t[1].mode.as_deref(), Some("sibling"));
        assert_eq!(t[2].sidecar, "b.spa.ximod-translation");
        assert_eq!(
            (t[2].out.as_deref(), t[2].format.as_deref(), t[2].full),
            (Some("b_ES.7z"), Some("7z"), true)
        );
        // A translation needs its sidecar.
        assert!(serde_json::from_str::<BatchManifest>(r#"{"entries":[{"root":"a","translations":[{}]}]}"#).is_err());
    }

    #[test]
    fn batch_exports_translations() {
        use std::fs;
        let dir = std::env::temp_dir().join(format!("ximod_batch_tr_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let root = dir.join("mod_a");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("Plugin.esp"), b"x").unwrap();
        let mut x = crate::models::Ximod::new("BatchTr");
        x.required_files
            .push(crate::models::InstallFile::new_file("Plugin.esp"));
        xml::save_ximod(&x, &root).unwrap();

        // A French translation of the mod name, stored in the mod folder.
        let mut doc = crate::cli_translate::extract_translation(&root, "eng", "fra").unwrap();
        for u in doc.units.iter_mut().filter(|u| u.source == "BatchTr") {
            u.target = "LotTrad".into();
            u.status = crate::models::translate::TStatus::Translated;
        }
        let sidecar = TranslationDoc::sidecar_path(&root, &doc.mod_name, "fra");
        doc.save(&sidecar).unwrap();
        let sidecar_rel = sidecar
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");

        // The main archive is written inside the mod folder (as by default).
        let main = root.join("BatchTr.zip");
        let full = dir.join("out").join("full.7z");
        let patch = dir.join("BatchTr_FR.zip");
        let manifest = dir.join("batch.json");
        let m = serde_json::json!({ "entries": [{
            "root": root.to_string_lossy(),
            "out": main.to_string_lossy(),
            "translations": [
                { "sidecar": sidecar_rel },
                { "sidecar": sidecar.to_string_lossy(), "mode": "package", "format": "7z", "full": true,
                  "out": full.to_string_lossy() },
                { "sidecar": sidecar_rel, "mode": "sibling" },
            ],
        }]});
        fs::write(&manifest, m.to_string()).unwrap();
        assert_eq!(run_batch(&manifest), 0);

        assert!(main.is_file(), "the mod itself is packaged first");
        // Patch package, at the default location next to the mod folder.
        let zip = zip::ZipArchive::new(fs::File::open(&patch).unwrap()).unwrap();
        let mut names: Vec<&str> = zip.file_names().collect();
        names.sort();
        assert_eq!(names, vec!["README_FR.txt", "fomod/ModuleConfig.xml", "fomod/info.xml"]);
        // Full package: the mod files and the translated XML, not the archive
        // written a moment ago.
        let back = dir.join("unpacked");
        sevenz_rust::decompress_file(&full, &back).unwrap();
        assert!(back.join("Plugin.esp").is_file());
        assert!(back.join("README_FR.txt").is_file());
        assert!(!back.join("BatchTr.zip").exists());
        assert!(
            fs::read_to_string(back.join("fomod/info.xml"))
                .unwrap()
                .contains("LotTrad")
        );
        // Sibling copy.
        assert!(
            fs::read_to_string(root.join("fomod_fra/info.xml"))
                .unwrap()
                .contains("LotTrad")
        );
        assert!(
            fs::read_to_string(root.join("fomod/info.xml"))
                .unwrap()
                .contains("BatchTr")
        );

        // Scenarios: a resolvable one passes, a stale one fails the entry.
        let sc = crate::models::simulate::Scenario {
            name: "all".into(),
            file_states: Default::default(),
            selections: Vec::new(),
        };
        crate::models::simulate::save_scenario(&root, &sc).unwrap();
        let stale = crate::models::simulate::Scenario {
            name: "stale".into(),
            file_states: Default::default(),
            selections: vec![crate::models::simulate::SelectionSpec {
                step: "S".into(),
                group: "G".into(),
                plugin: "Gone".into(),
            }],
        };
        crate::models::simulate::save_scenario(&root, &stale).unwrap();
        for (names, code) in [(vec!["all"], 0), (vec!["all", "stale"], 1), (vec!["missing"], 1)] {
            let m = serde_json::json!({ "entries": [{ "root": root.to_string_lossy(), "out": main.to_string_lossy(), "scenarios": names }] });
            fs::write(&manifest, m.to_string()).unwrap();
            assert_eq!(run_batch(&manifest), code, "{names:?}");
        }
        let m: BatchManifest =
            serde_json::from_str(r#"{"entries":[{"root":"a","scenarios":["x","y.json"]}]}"#).unwrap();
        assert_eq!(m.entries[0].scenarios, vec!["x".to_string(), "y.json".to_string()]);

        // A broken translation fails the entry like any other error.
        for bad in [
            serde_json::json!({ "sidecar": "missing.ximod-translation" }),
            serde_json::json!({ "sidecar": sidecar_rel, "mode": "elsewhere" }),
        ] {
            let m = serde_json::json!({ "entries": [{ "root": root.to_string_lossy(), "translations": [bad] }] });
            fs::write(&manifest, m.to_string()).unwrap();
            assert_eq!(run_batch(&manifest), 1);
        }

        let _ = fs::remove_dir_all(&dir);
    }
}
