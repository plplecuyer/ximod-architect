//! XIMOD Architect - Cross-platform mod installer creation tool
//!
//! A Rust port of the original creation tool for building mod installers
//! for Bethesda game mods (Skyrim, Fallout, Starfield, etc.)
//!
//! # Features
//!
//! - Create and edit mod installer packages
//! - Multi-step installation wizard support
//! - Conditional file installation
//! - Plugin dependency patterns
//! - Multi-language support (i18n)
//! - Pre/post save scripting
//! - Cross-platform (Windows, Linux, macOS)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod data;
mod export;
mod fonts;
mod games;
mod i18n;
mod icon;
mod models;
mod splash;
mod ui;
mod update;
mod xml;

// --- V2 roadmap (skeletons) ---
mod archive;
mod archive_open;
mod backups;
mod cli_batch;
mod cli_translate;
mod crash_guard;
mod manual;
mod media;
mod wizard;

use eframe::egui;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application name
pub const APP_NAME: &str = "XIMOD Architect";

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Public page of the application, embedded in the "Created with…" signature
/// comment of generated XML files. Leave empty to omit the URL from the
/// comment; set it to the Nexus/GitHub page once published.
pub const APP_URL: &str = "";

/// Default window size
pub const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;

/// Screen information (size of the primary screen, used to centre windows).
#[derive(Debug, Clone)]
pub struct ScreenInfo {
    pub width: f32,
    pub height: f32,
}

impl Default for ScreenInfo {
    fn default() -> Self {
        Self {
            width: 1920.0,
            height: 1080.0,
        }
    }
}

/// Size of the first connected output according to the kernel DRM subsystem
/// (`/sys/class/drm/<card>-<connector>/modes`, whose first line is the
/// preferred mode, e.g. `1920x1080`). Reading two tiny sysfs files is far
/// cheaper than spawning `xrandr`; `None` when sysfs has nothing usable.
#[cfg(target_os = "linux")]
fn drm_primary_mode() -> Option<(f32, f32)> {
    let mut connectors: Vec<std::path::PathBuf> = std::fs::read_dir("/sys/class/drm")
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    // `read_dir` order is unspecified: sort so the choice is deterministic.
    connectors.sort();
    connectors.into_iter().find_map(|dir| {
        let status = std::fs::read_to_string(dir.join("status")).ok()?;
        if status.trim() != "connected" {
            return None;
        }
        let modes = std::fs::read_to_string(dir.join("modes")).ok()?;
        parse_mode_line(modes.lines().next()?)
    })
}

/// Parse a DRM mode line such as `1920x1080` (or `1920x1080i`) into a size.
#[cfg(any(target_os = "linux", test))]
fn parse_mode_line(line: &str) -> Option<(f32, f32)> {
    let (w, h) = line.trim().split_once('x')?;
    let h = h.trim_end_matches(|c: char| !c.is_ascii_digit());
    let (w, h) = (w.parse::<u32>().ok()?, h.parse::<u32>().ok()?);
    (w > 0 && h > 0).then_some((w as f32, h as f32))
}

/// Detect primary screen dimensions
fn detect_primary_screen() -> ScreenInfo {
    #[allow(unused_mut)]
    let mut info = ScreenInfo::default();

    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

        // Direct Win32 query of the primary monitor: instantaneous, whereas the
        // previous implementation spawned PowerShell at every start-up.
        // SAFETY: `GetSystemMetrics` takes a plain integer index, has no
        // preconditions and does not touch any memory we own.
        let (w, h) = unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) };
        // It returns 0 on failure: keep the defaults in that case.
        if w > 0 && h > 0 {
            info.width = w as f32;
            info.height = h as f32;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some((w, h)) = drm_primary_mode() {
            info.width = w;
            info.height = h;
        } else if let Ok(output) = std::process::Command::new("xrandr").args(["--current"]).output() {
            // Fallback (no usable sysfs entry, e.g. some VMs / remote X servers).
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Look for primary monitor or first connected monitor
            let has_primary = stdout.contains("primary");
            for line in stdout.lines() {
                if line.contains(" connected") && (line.contains("primary") || !has_primary) {
                    // Parse resolution like "1920x1080+0+0"
                    if let Some(res) = line.split_whitespace().find(|s| s.contains('x') && s.contains('+')) {
                        let parts: Vec<&str> = res.split(['x', '+']).collect();
                        if parts.len() >= 2 {
                            info.width = parts[0].parse().unwrap_or(1920.0);
                            info.height = parts[1].parse().unwrap_or(1080.0);
                        }
                    }
                    break;
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        // Use system_profiler to get display info
        if let Ok(output) = Command::new("system_profiler").args(["SPDisplaysDataType"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let line = line.trim();
                if line.starts_with("Resolution:") {
                    // Parse "Resolution: 2560 x 1440" or similar
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        info.width = parts[1].parse().unwrap_or(1920.0);
                        info.height = parts[3].parse().unwrap_or(1080.0);
                    }
                    break;
                }
            }
        }
    }

    tracing::info!("Detected screen: {}x{}", info.width, info.height);
    info
}

/// Calculate centered window position
fn calculate_centered_position(screen: &ScreenInfo, window_width: f32, window_height: f32) -> (f32, f32) {
    let x = (screen.width - window_width) / 2.0;
    let y = (screen.height - window_height) / 2.0;

    // Ensure position is not negative
    let x = x.max(0.0);
    let y = y.max(0.0);

    tracing::info!(
        "Centered position: ({}, {}) for window {}x{}",
        x,
        y,
        window_width,
        window_height
    );
    (x, y)
}

/// Get the path to splash.png in assets/images/ relative to the executable
pub fn get_splash_path() -> Option<std::path::PathBuf> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))?;

    // First, try assets/images/splash.png (standard location)
    let assets_path = exe_dir.join("assets").join("images").join("splash.png");
    if assets_path.exists() {
        return Some(assets_path);
    }

    // Fallback: splash.png next to executable (for backwards compatibility)
    let direct_path = exe_dir.join("splash.png");
    if direct_path.exists() {
        return Some(direct_path);
    }

    // macOS: check in Resources folder of app bundle
    #[cfg(target_os = "macos")]
    {
        if let Some(resources) = exe_dir.parent().and_then(|p| Some(p.join("Resources"))) {
            let bundle_path = resources.join("assets").join("images").join("splash.png");
            if bundle_path.exists() {
                return Some(bundle_path);
            }
            let bundle_direct = resources.join("splash.png");
            if bundle_direct.exists() {
                return Some(bundle_direct);
            }
        }
    }

    None
}

/// Attach to the parent console on Windows so CLI output is visible when the
/// GUI-subsystem binary is launched from a terminal.
#[cfg(windows)]
fn attach_console() {
    use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};
    unsafe {
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
}
#[cfg(not(windows))]
fn attach_console() {}

fn cli_usage() {
    println!(
        "{APP_NAME} {APP_VERSION} — command-line mode\n\
\n\
Usage:\n  ximod-architect <command> [options]\n\
\n\
Commands:\n\
\x20 validate <root> [--sizes]  Validate the FOMOD in <root> (project + ModConfig 5.0 schema);\n\
\x20                            --sizes also measures the install size of the options\n\
\x20 extract  <archive> [-o DIR] [--fomod-only]\n\
\x20                            Extract a .zip / .7z mod archive (default: ./<archive stem>/) and\n\
\x20                            print its FOMOD root; --fomod-only keeps just the fomod folder\n\
\x20 package  <root> [-o FILE]  Write the FOMOD XML, then build a distribution .zip\n\
\x20 build    <root>            (Re)write fomod/info.xml and fomod/ModuleConfig.xml\n\
\x20 batch    <manifest.json>   Validate/build/package several FOMODs and their translations\n\
\x20                            (per entry: \"translations\": [{{\"sidecar\": FILE, \"mode\": package|sibling|inplace}}],\n\
\x20                            \"scenarios\": [NAME, …] runs saved simulator scenarios and fails the\n\
\x20                            entry when one names an option that no longer exists)\n\
\x20 simulate <root> [--scenario <name|file.json>] [--json]\n\
\x20                            Replay the installer: visible steps, selections (the defaults, or a\n\
\x20                            scenario saved under fomod/scenarios/), the final install list with\n\
\x20                            sizes and overwrites, and the total size; --json prints a report\n\
\x20 nexus-desc <root> [--format bbcode|md] [--previous <root>] [--lang <code>] [-o FILE]\n\
\x20                            Write a Nexus Mods page description (requirements, options,\n\
\x20                            installation, changelog against a previous version) in BBCode\n\
\x20                            (default) or Markdown; --lang uses a translation of the root\n\
\x20 inspect  <plugin.esp>      Print a plugin's masters, author, ESL flag and light-plugin report\n\
\x20 strings  <root> [--duplicates] [--csv FILE]\n\
\x20                            List every name and description of the FOMOD (key, field, text),\n\
\x20                            or only the groups of identical strings; --csv writes them as CSV\n\
\x20 backups  <root> [--list | --restore <timestamp> | --prune N]\n\
\x20                            List the rotating backups of the FOMOD XML (fomod/backups/, the\n\
\x20                            default), restore one (its timestamp as listed, YYYYMMDD-HHMMSS)\n\
\x20                            or keep only the N newest\n\
\x20 ba2 <dir> <out.ba2> [game] Pack a folder into a general BA2 (experimental)\n\
\x20 archive <file.bsa|file.ba2> [--json]\n\
\x20                            List the contents of a Bethesda archive (path, size, compressed)\n\
\x20 translate extract <root> --lang <iso3> [--source-lang <iso3>] [-o FILE]\n\
\x20                            List the strings of an existing FOMOD in a translation file\n\
\x20 translate status <FILE> [--strict]\n\
\x20                            Show translation progress and issues (--strict: exit 3 if incomplete)\n\
\x20 translate update <root> <FILE>\n\
\x20                            Refresh a translation file after the FOMOD changed\n\
\x20 translate apply <root> <FILE> [--mode inplace|sibling] [--force-explicit-order] [--force]\n\
\x20                            Write the translated FOMOD (sibling: <root>/fomod_<lang>/, the default;\n\
\x20                            inplace: over the originals, after a .bak copy)\n\
\x20 translate package <root> <FILE> [-o OUT] [--format zip|7z] [--full] [--name-template T]\n\
\x20                            Build an archive of the translation: the translated fomod XML files\n\
\x20                            and a README (default), or the whole translated mod (--full).\n\
\x20                            Name tokens: {{name}} {{version}} {{LANG}} {{lang}} {{lang3}} {{langname}}\n\
\x20 translate export-csv <FILE> <CSV>\n\
\x20                            Write the strings of a translation file as CSV (for a spreadsheet)\n\
\x20 translate import-csv <FILE> <CSV>\n\
\x20                            Read translations, notes and locks back from a CSV (matched by key)\n\
\x20 translate tm-learn <FILE>  Add the accepted translations of a file to the translation memory\n\
\x20 translate tm-apply <FILE>  Pre-fill untranslated strings from the translation memory\n\
\x20 help                       Show this help\n\
\x20 version                    Show the version\n\
\n\
<root> is the mod's root directory (the folder that contains, or will contain,\n\
the 'fomod' sub-directory)."
    );
}

/// Dispatch the command-line mode. Returns the process exit code.
fn run_cli(args: &[String]) -> i32 {
    attach_console();
    match args[0].as_str() {
        "-h" | "--help" | "help" => {
            cli_usage();
            0
        }
        "-V" | "--version" | "version" => {
            println!("{APP_NAME} {APP_VERSION}");
            0
        }
        "validate" => cli_validate(&args[1..]),
        "simulate" => cli_simulate(&args[1..]),
        "nexus-desc" => cli_nexus_desc(&args[1..]),
        "extract" => cli_extract(&args[1..]),
        "package" => cli_package(&args[1..]),
        "build" => cli_build(args.get(1)),
        "batch" => match args.get(1) {
            Some(manifest) => cli_batch::run_batch(std::path::Path::new(manifest)),
            None => {
                eprintln!("batch: missing <manifest.json>\n");
                cli_usage();
                2
            }
        },
        "inspect" => match args.get(1) {
            Some(plugin) => cli_inspect(std::path::Path::new(plugin)),
            None => {
                eprintln!("inspect: missing <plugin.esp>\n");
                cli_usage();
                2
            }
        },
        "ba2" => cli_ba2(&args[1..]),
        "archive" => cli_archive(&args[1..]),
        "backups" => cli_backups(&args[1..]),
        "strings" => cli_strings(&args[1..]),
        "translate" => cli_translate::run(&args[1..]),
        other => {
            eprintln!("Unknown command: {other}\n");
            cli_usage();
            2
        }
    }
}

/// `ba2 <src_dir> <out.ba2> [game]` — pack a folder into a general BA2 (experimental).
fn cli_ba2(args: &[String]) -> i32 {
    let (src, out) = match (args.first(), args.get(1)) {
        (Some(s), Some(o)) => (std::path::PathBuf::from(s), std::path::PathBuf::from(o)),
        _ => {
            eprintln!("ba2: usage: ba2 <src_dir> <out.ba2> [sse|fo4|starfield]\n");
            cli_usage();
            return 2;
        }
    };
    let game = match args.get(2).map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("fo4") | Some("fallout4") => archive::Ba2Game::Fallout4,
        Some("starfield") | Some("sf") => archive::Ba2Game::Starfield,
        _ => archive::Ba2Game::SkyrimSE,
    };
    let opts = archive::Ba2Options { game, general: true };
    match archive::package_ba2(&src, &out, &opts) {
        Ok(n) => {
            println!(
                "ba2: wrote {} ({n} files) [EXPERIMENTAL — verify in-game]",
                out.display()
            );
            0
        }
        Err(e) => {
            eprintln!("ba2: {e}");
            1
        }
    }
}

/// `archive <file.bsa|file.ba2> [--json]` — list the contents of a Bethesda
/// archive without reading its file data.
fn cli_archive(args: &[String]) -> i32 {
    let mut path: Option<&String> = None;
    let mut json = false;
    for a in args {
        match a.as_str() {
            "--json" => json = true,
            a if a.len() > 1 && a.starts_with('-') => {
                eprintln!("archive: unknown option '{a}'.\n");
                cli_usage();
                return 2;
            }
            _ if path.is_none() => path = Some(a),
            _ => {
                eprintln!("archive: unexpected argument '{a}'.\n");
                cli_usage();
                return 2;
            }
        }
    }
    let Some(path) = path else {
        eprintln!("archive: missing <file.bsa|file.ba2>.\n");
        cli_usage();
        return 2;
    };
    let path = std::path::Path::new(path);
    let listing = match models::bethesda_archive::list_archive(path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("archive: {}: {e}", path.display());
            return 1;
        }
    };
    if json {
        let entries: Vec<serde_json::Value> = listing
            .entries
            .iter()
            .map(|e| serde_json::json!({ "path": e.path, "size": e.size, "compressed": e.compressed }))
            .collect();
        let report = serde_json::json!({
            "archive": path.display().to_string(),
            "format": listing.format.label(),
            "files": listing.entries.len(),
            "totalSize": listing.total_size(),
            "entries": entries,
        });
        println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    } else {
        use models::simulate::format_size;
        println!(
            "{}: {} — {} file(s), {} unpacked",
            path.display(),
            listing.format.label(),
            listing.entries.len(),
            format_size(listing.total_size())
        );
        for e in &listing.entries {
            println!(
                "  {:>12}  {}  {}",
                e.size,
                if e.compressed { "packed" } else { "stored" },
                e.path
            );
        }
    }
    0
}

/// `inspect <plugin.esp>` — print a Bethesda plugin's masters, author, ESL
/// flag and the light-plugin (ESL) eligibility report.
fn cli_inspect(path: &std::path::Path) -> i32 {
    match models::plugin_header::read_plugin_header(path) {
        Ok(h) => {
            println!("Plugin: {}", path.display());
            if let Some(k) = h.kind {
                println!("  kind:   {k:?}");
            }
            println!("  light (ESL flag): {}", h.light);
            if let Some(a) = &h.author {
                println!("  author: {a}");
            }
            if let Some(d) = &h.description {
                println!("  description: {d}");
            }
            if h.masters.is_empty() {
                println!("  masters: (none)");
            } else {
                println!("  masters:");
                for m in &h.masters {
                    println!("    - {m}");
                }
            }
            match models::plugin_header::count_new_records(path, models::plugin_header::EslLimits::default()) {
                Ok(r) => {
                    println!("  new records: {} (light limit {})", r.new_forms, r.limit);
                    println!(
                        "  light eligible: {}{}",
                        r.eligible,
                        if r.eligible || r.new_forms > r.limit {
                            String::new()
                        } else {
                            " (a new FormID is outside the light range)".to_string()
                        }
                    );
                }
                Err(e) => println!("  new records: unknown ({e})"),
            }
            0
        }
        Err(e) => {
            eprintln!("inspect: {e}");
            1
        }
    }
}

/// Number of backups to keep, from the saved configuration (default 10).
fn cli_backup_count() -> usize {
    config::AppConfig::load_existing()
        .map(|c| c.backup_count)
        .unwrap_or_else(|| config::AppConfig::default().backup_count)
}

/// `backups <root> [--list | --restore <timestamp> | --prune N]`.
fn cli_backups(args: &[String]) -> i32 {
    let Some(root) = args.first() else {
        eprintln!("backups: missing <root>.\n");
        cli_usage();
        return 2;
    };
    let root = std::path::Path::new(root);
    match args.get(1).map(String::as_str) {
        None | Some("--list") => {
            let list = backups::list_backups(root);
            if list.is_empty() {
                println!("No backups under {}", backups::backups_dir(root).display());
            } else {
                println!(
                    "{} backup(s) under {}:",
                    list.len(),
                    backups::backups_dir(root).display()
                );
                for b in &list {
                    println!("  {}  ({})", b.stamp, b.display_time());
                }
            }
            0
        }
        Some("--restore") => {
            let Some(stamp) = args.get(2) else {
                eprintln!("backups: --restore requires a timestamp (see --list).\n");
                cli_usage();
                return 2;
            };
            match backups::restore_to_disk(root, stamp, cli_backup_count()) {
                Ok(x) => {
                    println!(
                        "Restored backup {stamp} of \"{}\" into {}/fomod",
                        x.name,
                        root.display()
                    );
                    0
                }
                Err(e) => {
                    eprintln!("backups: {e}");
                    1
                }
            }
        }
        Some("--prune") => {
            let keep = match args.get(2).map(|s| s.parse::<usize>()) {
                Some(Ok(n)) => n,
                _ => {
                    eprintln!("backups: --prune requires the number of backups to keep.\n");
                    cli_usage();
                    return 2;
                }
            };
            match backups::prune(root, keep) {
                Ok(n) => {
                    println!("Removed {n} backup(s), {} kept.", backups::list_backups(root).len());
                    0
                }
                Err(e) => {
                    eprintln!("backups: {e}");
                    1
                }
            }
        }
        Some(other) => {
            eprintln!("backups: unknown option '{other}'.\n");
            cli_usage();
            2
        }
    }
}

/// `strings <root> [--duplicates] [--csv FILE]` — the names and descriptions
/// of a FOMOD as a table, or its groups of identical strings.
/// Exit codes: 0 ok, 1 error, 2 usage.
fn cli_strings(args: &[String]) -> i32 {
    let mut root: Option<&String> = None;
    let mut dups = false;
    let mut csv: Option<&String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--duplicates" => dups = true,
            "--csv" => {
                i += 1;
                match args.get(i) {
                    Some(v) => csv = Some(v),
                    None => {
                        eprintln!("strings: --csv requires a file.\n");
                        cli_usage();
                        return 2;
                    }
                }
            }
            a if a.len() > 1 && a.starts_with('-') => {
                eprintln!("strings: unknown option '{a}'.\n");
                cli_usage();
                return 2;
            }
            _ if root.is_none() => root = Some(&args[i]),
            other => {
                eprintln!("strings: unexpected argument '{other}'.\n");
                cli_usage();
                return 2;
            }
        }
        i += 1;
    }
    let (_, ximod) = match cli_load(root) {
        Ok(v) => v,
        Err(code) => return code,
    };
    let units = models::strings::model_units(&ximod);
    if let Some(path) = csv {
        let text = models::strings::to_csv(&units);
        if let Err(e) = std::fs::write(path, text) {
            eprintln!("strings: cannot write {path}: {e}");
            return 1;
        }
        println!("{} string(s) written to {path}", units.len());
        return 0;
    }
    if dups {
        let groups = models::strings::duplicates(&units);
        let mut groups: Vec<(String, Vec<usize>)> = groups.into_iter().collect();
        groups.sort();
        for (text, idx) in &groups {
            println!("{text}");
            for &i in idx {
                let u = &units[i];
                println!("    {}\t{}", u.key, models::strings::field_name(u.field));
            }
        }
        println!("{} duplicate group(s)", groups.len());
        return 0;
    }
    for u in &units {
        println!(
            "{}\t{}\t{}",
            u.key,
            models::strings::field_name(u.field),
            u.source.replace(['\r', '\n'], " ")
        );
    }
    println!("{} string(s)", units.len());
    0
}

fn cli_load(root: Option<&String>) -> Result<(std::path::PathBuf, models::Ximod), i32> {
    let root = match root {
        Some(r) => std::path::PathBuf::from(r),
        None => {
            eprintln!("Error: missing <root> directory.");
            return Err(2);
        }
    };
    match xml::load_ximod(&root) {
        Ok(m) => Ok((root, m)),
        Err(e) => {
            eprintln!("Error: cannot load FOMOD from {}: {e}", root.display());
            Err(1)
        }
    }
}

/// `extract <archive> [-o DIR] [--fomod-only]` — unpack a mod archive and
/// print the folder holding its `fomod/`. Exit codes: 0 ok, 1 error, 2 usage.
fn cli_extract(args: &[String]) -> i32 {
    let mut archive: Option<&String> = None;
    let mut out: Option<String> = None;
    let mut fomod_only = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                i += 1;
                match args.get(i) {
                    Some(v) => out = Some(v.clone()),
                    None => {
                        eprintln!("extract: -o requires a directory.\n");
                        cli_usage();
                        return 2;
                    }
                }
            }
            "--fomod-only" => fomod_only = true,
            a if a.len() > 1 && a.starts_with('-') => {
                eprintln!("extract: unknown option '{a}'.\n");
                cli_usage();
                return 2;
            }
            _ => {
                if archive.is_none() {
                    archive = Some(&args[i]);
                } else {
                    eprintln!("extract: unexpected argument '{}'.\n", args[i]);
                    cli_usage();
                    return 2;
                }
            }
        }
        i += 1;
    }
    let Some(archive) = archive else {
        eprintln!("extract: missing <archive>.\n");
        cli_usage();
        return 2;
    };
    let archive = std::path::Path::new(archive);
    let dest = match out {
        Some(o) => std::path::PathBuf::from(o),
        None => {
            let stem = archive.file_stem().and_then(|s| s.to_str()).unwrap_or("archive");
            std::env::current_dir().unwrap_or_default().join(stem)
        }
    };
    match archive_open::extract_archive(archive, &dest, fomod_only, &mut |_, _, _| true) {
        Ok(report) => {
            println!(
                "Extracted {} file(s) into {}\nFOMOD root: {}",
                report.files,
                dest.display(),
                report.root.display()
            );
            0
        }
        Err(e) => {
            eprintln!("extract: {e}");
            1
        }
    }
}

fn cli_validate(args: &[String]) -> i32 {
    let mut root: Option<&String> = None;
    let mut sizes = false;
    for a in args {
        match a.as_str() {
            "--sizes" => sizes = true,
            a if a.len() > 1 && a.starts_with('-') => {
                eprintln!("validate: unknown option '{a}'.\n");
                cli_usage();
                return 2;
            }
            _ if root.is_none() => root = Some(a),
            _ => {
                eprintln!("validate: unexpected argument '{a}'.\n");
                cli_usage();
                return 2;
            }
        }
    }
    let root = match root {
        Some(r) => std::path::PathBuf::from(r),
        None => {
            eprintln!("Error: missing <root> directory.");
            return 2;
        }
    };
    let (ximod, unmodelled) = match xml::load_ximod_with_report(&root) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: cannot load FOMOD from {}: {e}", root.display());
            return 1;
        }
    };
    let mut problems = 0;
    // Constructs the editor cannot keep: a warning, not a schema problem.
    if !unmodelled.is_empty() {
        println!(
            "[ModuleConfig.xml] warning: {} construct(s) XIMOD cannot edit (dropped on save):",
            unmodelled.len()
        );
        for u in &unmodelled {
            println!("  - {u}");
        }
    }
    for err in ximod.validate() {
        println!("[project] {err:?}");
        problems += 1;
    }
    if let Ok(xml) = xml::module_config_to_string(&ximod) {
        for issue in xml::validate::validate_module_config(&xml) {
            println!(
                "[ModuleConfig.xml] line {}, col {}: {}",
                issue.line, issue.column, issue.kind
            );
            problems += 1;
        }
    }
    if let Ok(xml) = xml::info_xml_to_string(&ximod) {
        for issue in xml::validate::validate_info(&xml) {
            println!("[info.xml] line {}, col {}: {}", issue.line, issue.column, issue.kind);
            problems += 1;
        }
    }
    // Unreachable nodes, with their condition spelled out (nested groups in
    // parentheses) so the author sees which value is never set.
    let labels = models::condition_text::ConditionLabels::english();
    for u in models::simulate::unreachable(&ximod) {
        let condition = match u {
            models::simulate::Unreachable::Step { index } => ximod.steps.get(index).map(|s| &s.visibility),
            models::simulate::Unreachable::ConditionalSet { index } => {
                ximod.conditional_files.get(index).map(|c| &c.condition)
            }
            models::simulate::Unreachable::Option { .. } => None,
        };
        match condition {
            Some(g) => println!(
                "[project] info: unreachable {u:?} (condition: {})",
                models::condition_text::describe(g, &labels)
            ),
            None => println!("[project] info: unreachable {u:?}"),
        }
    }
    // Destination conflicts (including the contents of BSA / BA2 archives):
    // reported, not counted as problems — they are warnings for the author.
    for c in models::conflicts::detect_conflicts(&ximod, &root, models::conflicts::ConflictMode::CertainOnly) {
        println!("[files] warning: {c}");
    }
    if sizes {
        use models::simulate::format_size;
        let report = models::simulate::SizeReport::compute(&ximod, &root);
        let options = report.options_total();
        let conditional: u64 = report.conditional_sizes.iter().map(|s| s.bytes).sum();
        println!(
            "Install size: options {} (total {}), required {}, conditional sets {} (total {})",
            report.option_sizes.len(),
            format_size(options.bytes),
            format_size(report.required_size.bytes),
            report.conditional_sizes.len(),
            format_size(conditional)
        );
    }
    if problems == 0 {
        println!("OK — the FOMOD conforms to the schema.");
        0
    } else {
        eprintln!("{problems} problem(s) found.");
        1
    }
}

/// `simulate <root> [--scenario <name|file.json>] [--json]`: replay the
/// installer with the default selections or a saved scenario. Exit codes:
/// 0 ok, 1 error (also when a scenario names options that do not exist),
/// 2 usage.
fn cli_simulate(args: &[String]) -> i32 {
    let mut root: Option<&String> = None;
    let mut scenario: Option<String> = None;
    let mut json = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scenario" => {
                i += 1;
                match args.get(i) {
                    Some(v) => scenario = Some(v.clone()),
                    None => {
                        eprintln!("simulate: --scenario requires a name or a .json file.\n");
                        cli_usage();
                        return 2;
                    }
                }
            }
            "--json" => json = true,
            a if a.len() > 1 && a.starts_with('-') => {
                eprintln!("simulate: unknown option '{a}'.\n");
                cli_usage();
                return 2;
            }
            _ if root.is_none() => root = Some(&args[i]),
            a => {
                eprintln!("simulate: unexpected argument '{a}'.\n");
                cli_usage();
                return 2;
            }
        }
        i += 1;
    }
    let (root, ximod) = match cli_load(root) {
        Ok(v) => v,
        Err(c) => return c,
    };
    let report = match cli_batch::simulate_report(&root, &ximod, scenario.as_deref()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("simulate: {e}");
            return 1;
        }
    };
    if json {
        match serde_json::to_string_pretty(&report) {
            Ok(text) => println!("{text}"),
            Err(e) => {
                eprintln!("simulate: {e}");
                return 1;
            }
        }
    } else {
        print!("{}", cli_batch::format_simulation(&report));
    }
    if report.unresolved.is_empty() { 0 } else { 1 }
}

/// `nexus-desc <root> [--format bbcode|md] [--previous <root>] [--lang <code>] [-o FILE]`.
fn cli_nexus_desc(args: &[String]) -> i32 {
    use models::nexus_desc::{DescFormat, DescLabels, DescOptions};
    let mut root: Option<&String> = None;
    let mut format = DescFormat::BBCode;
    let mut previous: Option<String> = None;
    let mut lang: Option<String> = None;
    let mut out: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        let value = |i: &mut usize, what: &str| -> Option<String> {
            *i += 1;
            let v = args.get(*i).cloned();
            if v.is_none() {
                eprintln!("nexus-desc: {what} requires a value.\n");
            }
            v
        };
        match args[i].as_str() {
            "--format" => match value(&mut i, "--format").map(|v| v.to_ascii_lowercase()) {
                Some(v) if v == "bbcode" || v == "bb" => format = DescFormat::BBCode,
                Some(v) if v == "md" || v == "markdown" => format = DescFormat::Markdown,
                Some(v) => {
                    eprintln!("nexus-desc: unknown format '{v}' (bbcode or md).\n");
                    cli_usage();
                    return 2;
                }
                None => {
                    cli_usage();
                    return 2;
                }
            },
            "--previous" => match value(&mut i, "--previous") {
                Some(v) => previous = Some(v),
                None => {
                    cli_usage();
                    return 2;
                }
            },
            "--lang" => match value(&mut i, "--lang") {
                Some(v) => lang = Some(v),
                None => {
                    cli_usage();
                    return 2;
                }
            },
            "-o" | "--output" => match value(&mut i, "-o") {
                Some(v) => out = Some(v),
                None => {
                    cli_usage();
                    return 2;
                }
            },
            a if a.len() > 1 && a.starts_with('-') => {
                eprintln!("nexus-desc: unknown option '{a}'.\n");
                cli_usage();
                return 2;
            }
            _ if root.is_none() => root = Some(&args[i]),
            a => {
                eprintln!("nexus-desc: unexpected argument '{a}'.\n");
                cli_usage();
                return 2;
            }
        }
        i += 1;
    }
    let (root, ximod) = match cli_load(root) {
        Ok(v) => v,
        Err(c) => return c,
    };
    let previous = match previous {
        Some(p) if !std::path::Path::new(&p).is_dir() => {
            eprintln!("nexus-desc: the previous version '{p}' is not a folder");
            return 1;
        }
        Some(p) => match xml::load_ximod(std::path::Path::new(&p)) {
            Ok(m) => Some(m),
            Err(e) => {
                eprintln!("nexus-desc: cannot load the previous version from {p}: {e}");
                return 1;
            }
        },
        None => None,
    };
    let games = games::GamesData::load();
    let opts = DescOptions {
        format,
        ..Default::default()
    };
    let labels = DescLabels::english();
    let text = match lang {
        Some(code) => {
            let code = i18n::normalize_locale(&code);
            let found = models::translate::TranslationDoc::find_sidecars(&root)
                .into_iter()
                .filter_map(|p| models::translate::TranslationDoc::load(&p).ok())
                .find(|d| i18n::normalize_locale(&d.target_lang) == code);
            match found {
                Some(doc) => models::nexus_desc::generate_translated(
                    &doc,
                    &ximod,
                    Some(&root),
                    &games,
                    previous.as_ref(),
                    &opts,
                    &labels,
                ),
                None => {
                    eprintln!(
                        "nexus-desc: no translation for '{code}' under {}",
                        models::translate::TranslationDoc::sidecar_path(&root, &ximod.name, &code)
                            .parent()
                            .map(|p| p.display().to_string())
                            .unwrap_or_default()
                    );
                    return 1;
                }
            }
        }
        None => models::nexus_desc::generate(&ximod, Some(&root), &games, previous.as_ref(), &opts, &labels),
    };
    match out {
        Some(path) => match std::fs::write(&path, &text) {
            Ok(()) => {
                println!("Wrote {path}");
                0
            }
            Err(e) => {
                eprintln!("nexus-desc: cannot write {path}: {e}");
                1
            }
        },
        None => {
            print!("{text}");
            0
        }
    }
}

fn cli_build(root: Option<&String>) -> i32 {
    let (root, ximod) = match cli_load(root) {
        Ok(v) => v,
        Err(c) => return c,
    };
    if let Err(e) = backups::backup_before_save(&ximod, &root, cli_backup_count()) {
        eprintln!("Warning: backup skipped: {e}");
    }
    match xml::save_ximod(&ximod, &root) {
        Ok(()) => {
            println!("Wrote the FOMOD XML into {}/fomod", root.display());
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

fn cli_package(args: &[String]) -> i32 {
    let mut root: Option<&String> = None;
    let mut out: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                i += 1;
                match args.get(i) {
                    Some(v) => out = Some(v.clone()),
                    None => {
                        eprintln!("Error: -o requires a file path.");
                        return 2;
                    }
                }
            }
            _ => {
                if root.is_none() {
                    root = Some(&args[i]);
                } else {
                    eprintln!("Error: unexpected argument '{}'.", args[i]);
                    return 2;
                }
            }
        }
        i += 1;
    }
    let (root, ximod) = match cli_load(root) {
        Ok(v) => v,
        Err(c) => return c,
    };
    let out_path = match out {
        Some(o) => std::path::PathBuf::from(o),
        None => std::env::current_dir()
            .unwrap_or_default()
            .join(export::default_archive_name(&ximod)),
    };
    if let Err(e) = backups::backup_before_save(&ximod, &root, cli_backup_count()) {
        eprintln!("Warning: backup skipped: {e}");
    }
    match export::build_distribution_archive(&ximod, &root, &out_path) {
        Ok(n) => {
            println!("Packaged {n} file(s) into {}", out_path.display());
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

fn main() -> eframe::Result<()> {
    // Headless command-line mode: any argument switches XIMOD to the CLI, used
    // for automated / CI builds (validate, package, build) without the GUI.
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        std::process::exit(run_cli(&args));
    }

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "ximod_architect=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("{} v{} starting...", APP_NAME, APP_VERSION);

    // Write every modified project to a recovery folder if we ever panic
    // (release builds abort on panic, so this is the only chance to save).
    crash_guard::install_hook();

    // Step 1: Load configuration
    let config = config::AppConfig::load().unwrap_or_default();
    let window_width = if config.window_width > 0.0 {
        config.window_width
    } else {
        DEFAULT_WINDOW_WIDTH
    };
    let window_height = if config.window_height > 0.0 {
        config.window_height
    } else {
        DEFAULT_WINDOW_HEIGHT
    };

    tracing::info!("Window size from config: {}x{}", window_width, window_height);

    // Step 2: Detect primary screen dimensions
    let screen = detect_primary_screen();

    // Read the datasets the application needs on a worker thread, so the
    // splash screen covers the start-up work instead of adding to it.
    let preload = {
        let config = config.clone();
        std::thread::spawn(move || ui::main_window::Preloaded::with_config(config))
    };

    // Step 3: Show native transparent splash screen BEFORE creating main window
    let splash_enabled = config.splash_screen_seconds > 0;
    let splash_path = get_splash_path();
    let splash_exists = splash_path.as_ref().map(|p| p.exists()).unwrap_or(false);

    if splash_enabled
        && splash_exists
        && let Some(path) = splash_path
    {
        let splash_config = splash::SplashConfig {
            image_path: path,
            display_duration: Duration::from_secs(config.splash_screen_seconds as u64),
            fade_duration: Duration::from_millis(500),
            screen_width: screen.width,
            screen_height: screen.height,
        };

        match splash::show_splash(splash_config) {
            Ok(()) => tracing::info!("Splash screen completed"),
            Err(e) => tracing::warn!("Splash screen error (continuing): {}", e),
        }
    }

    // Step 4: Create main window (splash has finished, so show decorations)
    let (win_x, win_y) = calculate_centered_position(&screen, window_width, window_height);

    // Load application icon for window
    let window_icon = icon::create_viewport_icon();
    if window_icon.is_some() {
        tracing::info!("Application icon loaded successfully");
    } else {
        tracing::info!("No application icon found, using default");
    }

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([window_width, window_height])
        .with_position([win_x, win_y])
        .with_min_inner_size([800.0, 600.0])
        .with_title(format!("{} v{}", APP_NAME, APP_VERSION));

    // Add icon if available
    if let Some(icon) = window_icon {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    // Step 5: Run the application (splash already done, no splash in XimodApp)
    let preloaded = preload
        .join()
        .unwrap_or_else(|_| ui::main_window::Preloaded::with_config(config));
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(move |cc| Ok(Box::new(ui::XimodApp::new(cc, preloaded, screen)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_constants() {
        assert!(!APP_NAME.is_empty());
        assert!(!APP_VERSION.is_empty());
    }

    #[test]
    fn test_centered_position() {
        let screen = ScreenInfo {
            width: 1920.0,
            height: 1080.0,
        };
        let (x, y) = calculate_centered_position(&screen, 1280.0, 800.0);
        assert_eq!(x, 320.0);
        assert_eq!(y, 140.0);
    }

    #[test]
    fn cli_strings_lists_and_exports() {
        assert_eq!(cli_strings(&[]), 2, "missing <root>");
        assert_eq!(cli_strings(&["r".into(), "--csv".into()]), 2);
        assert_eq!(cli_strings(&["r".into(), "--bogus".into()]), 2);
        assert_eq!(cli_strings(&["r".into(), "s".into()]), 2);
        let root = std::env::temp_dir().join(format!("ximod_cli_strings_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let mut m = models::Ximod::new("Aurelia");
        let mut step = models::Step::new("Textures");
        let mut group = models::PluginGroup::new("Textures", models::SelectionType::SelectAny);
        group.plugins.push(models::Plugin::new("4K"));
        step.plugin_groups.push(group);
        m.steps.push(step);
        xml::save_ximod(&m, &root).unwrap();
        let r = root.to_string_lossy().into_owned();
        assert_eq!(cli_strings(std::slice::from_ref(&r)), 0);
        assert_eq!(cli_strings(&[r.clone(), "--duplicates".into()]), 0);
        let csv = root.join("strings.csv");
        assert_eq!(cli_strings(&[r, "--csv".into(), csv.to_string_lossy().into_owned()]), 0);
        let text = std::fs::read_to_string(&csv).unwrap();
        assert!(text.starts_with("key,field,context,text\n"));
        assert!(text.contains("config/step[0]/group[0]/plugin[0]/name,pluginName,Textures › Textures › 4K,4K\n"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn cli_extract_usage_and_unsupported_archive_exit_codes() {
        assert_eq!(cli_extract(&[]), 2, "missing <archive>");
        assert_eq!(cli_extract(&["a.zip".into(), "-o".into()]), 2, "-o without a value");
        assert_eq!(cli_extract(&["a.zip".into(), "--bogus".into()]), 2);
        assert_eq!(cli_extract(&["a.zip".into(), "b.zip".into()]), 2);
        let rar = std::env::temp_dir().join("ximod_cli_extract.rar");
        assert_eq!(
            cli_extract(&[rar.to_string_lossy().into_owned()]),
            1,
            ".rar is unsupported"
        );
    }

    /// `archive`: usage errors exit 2, an unreadable archive exits 1, a
    /// BA2 built by the packer lists (plain and `--json`).
    #[test]
    fn cli_archive_lists_a_ba2() {
        let root = std::env::temp_dir().join(format!("ximod_cli_archive_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("src/meshes")).unwrap();
        std::fs::write(root.join("src/meshes/a.nif"), b"abc").unwrap();
        let out = root.join("Mod - Main.ba2");
        archive::package_ba2(
            &root.join("src"),
            &out,
            &archive::Ba2Options {
                game: archive::Ba2Game::Fallout4,
                general: true,
            },
        )
        .unwrap();
        let o = out.to_string_lossy().into_owned();
        assert_eq!(cli_archive(&[]), 2);
        assert_eq!(cli_archive(&[o.clone(), "--bogus".into()]), 2);
        assert_eq!(cli_archive(&[o.clone(), "other".into()]), 2);
        assert_eq!(cli_archive(&[root.join("nope.bsa").to_string_lossy().into_owned()]), 1);
        assert_eq!(cli_archive(std::slice::from_ref(&o)), 0);
        assert_eq!(cli_archive(&[o.clone(), "--json".into()]), 0);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// `backups`: usage errors exit 2, a missing backup exits 1, and a
    /// restore through the CLI writes the backup back as the project.
    #[test]
    fn cli_backups_list_restore_and_prune() {
        let root = std::env::temp_dir().join(format!("ximod_cli_backups_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let r = root.to_string_lossy().into_owned();
        assert_eq!(cli_backups(&[]), 2);
        assert_eq!(cli_backups(&[r.clone(), "--bogus".into()]), 2);
        assert_eq!(cli_backups(&[r.clone(), "--restore".into()]), 2);
        assert_eq!(cli_backups(&[r.clone(), "--prune".into(), "x".into()]), 2);
        assert_eq!(
            cli_backups(std::slice::from_ref(&r)),
            0,
            "an empty list is not an error"
        );
        assert_eq!(
            cli_backups(&[r.clone(), "--restore".into(), "20000101-000000".into()]),
            1
        );

        // Two backups holding older versions, and a current version on disk.
        for (stamp, name) in [("20260101-100000", "One"), ("20260102-100000", "Two")] {
            let dir = backups::backups_dir(&root).join(stamp);
            std::fs::create_dir_all(&dir).unwrap();
            let x = models::Ximod::new(name);
            std::fs::write(dir.join("info.xml"), xml::info_xml_to_string(&x).unwrap()).unwrap();
            std::fs::write(dir.join("ModuleConfig.xml"), xml::module_config_to_string(&x).unwrap()).unwrap();
        }
        xml::save_ximod(&models::Ximod::new("Current"), &root).unwrap();
        assert_eq!(cli_backups(&[r.clone(), "--list".into()]), 0);
        assert_eq!(
            cli_backups(&[r.clone(), "--restore".into(), "20260101-100000".into()]),
            0
        );
        assert_eq!(xml::load_ximod(&root).unwrap().name, "One");
        // The replaced version was backed up: three backups now, prune to one.
        assert_eq!(backups::list_backups(&root).len(), 3);
        assert_eq!(cli_backups(&[r.clone(), "--prune".into(), "1".into()]), 0);
        assert_eq!(backups::list_backups(&root).len(), 1);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// `simulate` / `nexus-desc` / `validate`: usage errors exit 2, a
    /// missing project exits 1, and a saved scenario is replayed.
    #[test]
    fn cli_simulate_nexus_desc_and_validate_sizes() {
        let root = std::env::temp_dir().join(format!("ximod_cli_sim_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("tex")).unwrap();
        std::fs::write(root.join("Plugin.esp"), b"12345").unwrap();
        std::fs::write(root.join("tex/a.dds"), b"1234567890").unwrap();
        let r = root.to_string_lossy().into_owned();
        assert_eq!(cli_simulate(&[]), 2);
        assert_eq!(cli_simulate(&[r.clone(), "--scenario".into()]), 2);
        assert_eq!(cli_simulate(&[r.clone(), "--bogus".into()]), 2);
        assert_eq!(cli_simulate(&[r.clone(), "x".into()]), 2);
        // An unreadable ModuleConfig.xml (a folder in its place): exit 1.
        std::fs::create_dir_all(root.join("fomod/ModuleConfig.xml")).unwrap();
        assert_eq!(cli_simulate(std::slice::from_ref(&r)), 1, "unloadable FOMOD");
        assert_eq!(cli_nexus_desc(&[]), 2);
        assert_eq!(cli_nexus_desc(&[r.clone(), "--format".into()]), 2);
        assert_eq!(cli_nexus_desc(&[r.clone(), "--format".into(), "html".into()]), 2);
        assert_eq!(cli_nexus_desc(&[r.clone(), "--previous".into()]), 2);
        assert_eq!(cli_nexus_desc(&[r.clone(), "--lang".into()]), 2);
        assert_eq!(cli_nexus_desc(&[r.clone(), "-o".into()]), 2);
        assert_eq!(cli_nexus_desc(&[r.clone(), "--nope".into()]), 2);
        assert_eq!(cli_nexus_desc(std::slice::from_ref(&r)), 1, "unloadable FOMOD");
        std::fs::remove_dir_all(root.join("fomod")).unwrap();
        assert_eq!(cli_validate(&[r.clone(), "--nope".into()]), 2);
        assert_eq!(cli_validate(&[]), 2);

        let mut x = models::Ximod::new("SimMod");
        x.version = "1.0".into();
        x.required_files.push(models::InstallFile::new_file("Plugin.esp"));
        let mut g = models::PluginGroup::new("Textures", models::SelectionType::SelectExactlyOne);
        let mut p = models::Plugin::new("HD");
        p.files.push(models::InstallFile::new_folder("tex"));
        g.plugins.push(p);
        g.plugins.push(models::Plugin::new("None"));
        let mut s = models::Step::new("Main");
        s.plugin_groups.push(g);
        x.steps.push(s);
        xml::save_ximod(&x, &root).unwrap();

        assert_eq!(cli_validate(&[r.clone(), "--sizes".into()]), 0);
        assert_eq!(cli_simulate(std::slice::from_ref(&r)), 0);
        assert_eq!(cli_simulate(&[r.clone(), "--json".into()]), 0);
        assert_eq!(cli_simulate(&[r.clone(), "--scenario".into(), "nope".into()]), 1);
        // A saved scenario choosing "None", and one naming a gone option.
        let sc = models::simulate::Scenario {
            name: "lite".into(),
            file_states: Default::default(),
            selections: vec![models::simulate::SelectionSpec {
                step: "Main".into(),
                group: "Textures".into(),
                plugin: "None".into(),
            }],
        };
        models::simulate::save_scenario(&root, &sc).unwrap();
        assert_eq!(cli_simulate(&[r.clone(), "--scenario".into(), "lite".into()]), 0);
        let mut stale = sc.clone();
        stale.name = "stale".into();
        stale.selections.push(models::simulate::SelectionSpec {
            step: "Main".into(),
            group: "Textures".into(),
            plugin: "Gone".into(),
        });
        let path = models::simulate::save_scenario(&root, &stale).unwrap();
        assert_eq!(
            cli_simulate(&[r.clone(), "--scenario".into(), path.to_string_lossy().into_owned()]),
            1,
            "unresolved names fail"
        );
        let report = cli_batch::simulate_report(&root, &x, Some("lite")).unwrap();
        assert_eq!(report.scenario.as_deref(), Some("lite"));
        assert_eq!(report.total_size, 5);
        assert_eq!(report.install.len(), 1);
        let report = cli_batch::simulate_report(&root, &x, None).unwrap();
        assert_eq!(report.total_size, 15);
        assert!(report.selections.iter().any(|s| s.plugin == "HD"));
        let text = cli_batch::format_simulation(&report);
        assert!(
            text.contains("Main") && text.contains("HD") && text.contains("15 B"),
            "{text}"
        );

        let out = root.join("desc.md");
        assert_eq!(
            cli_nexus_desc(&[
                r.clone(),
                "--format".into(),
                "md".into(),
                "--previous".into(),
                r.clone(),
                "-o".into(),
                out.to_string_lossy().into_owned()
            ]),
            0
        );
        let text = std::fs::read_to_string(&out).unwrap();
        assert!(text.starts_with("# SimMod 1.0"), "{text}");
        assert!(text.contains("- Step 1: Main"));
        assert_eq!(
            cli_nexus_desc(&[r.clone(), "--lang".into(), "fr".into()]),
            1,
            "no sidecar"
        );
        assert_eq!(cli_nexus_desc(&[r.clone(), "--previous".into(), "/nope".into()]), 1);
        assert_eq!(cli_nexus_desc(std::slice::from_ref(&r)), 0);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn test_parse_mode_line() {
        assert_eq!(parse_mode_line("1920x1080"), Some((1920.0, 1080.0)));
        assert_eq!(parse_mode_line("1920x1080i\n"), Some((1920.0, 1080.0)));
        assert_eq!(parse_mode_line(""), None);
        assert_eq!(parse_mode_line("0x0"), None);
        assert_eq!(parse_mode_line("garbage"), None);
    }
}
