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
mod xml;

use eframe::egui;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;

/// Application name
pub const APP_NAME: &str = "XIMOD Architect";

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default window size
pub const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;

/// Screen information
#[derive(Debug, Clone)]
pub struct ScreenInfo {
    pub width: f32,
    pub height: f32,
    pub dpi: f32,
}

impl Default for ScreenInfo {
    fn default() -> Self {
        Self {
            width: 1920.0,
            height: 1080.0,
            dpi: 96.0,
        }
    }
}

/// Detect primary screen dimensions
fn detect_primary_screen() -> ScreenInfo {
    let mut info = ScreenInfo::default();
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        #[cfg(windows)]
        use std::os::windows::process::CommandExt;
        
        // CREATE_NO_WINDOW flag to prevent console window flash
        #[cfg(windows)]
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        
        // Try PowerShell to get screen info (with hidden window)
        let mut cmd = Command::new("powershell");
        cmd.args(["-NoProfile", "-NonInteractive", "-Command", 
            "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Screen]::PrimaryScreen.Bounds | Format-List Width,Height"]);
        
        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW);
        
        if let Ok(output) = cmd.output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let line = line.trim();
                if line.starts_with("Width") {
                    if let Some(val) = line.split(':').nth(1) {
                        info.width = val.trim().parse().unwrap_or(1920.0);
                    }
                } else if line.starts_with("Height") {
                    if let Some(val) = line.split(':').nth(1) {
                        info.height = val.trim().parse().unwrap_or(1080.0);
                    }
                }
            }
        }
        
        // Try to get DPI (with hidden window)
        let mut cmd_dpi = Command::new("powershell");
        cmd_dpi.args(["-NoProfile", "-NonInteractive", "-Command", 
            "(Get-ItemProperty 'HKCU:\\Control Panel\\Desktop\\WindowMetrics' -Name AppliedDPI -ErrorAction SilentlyContinue).AppliedDPI"]);
        
        #[cfg(windows)]
        cmd_dpi.creation_flags(CREATE_NO_WINDOW);
        
        if let Ok(output) = cmd_dpi.output() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(dpi) = stdout.parse::<f32>() {
                info.dpi = dpi;
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        
        // Try xrandr
        if let Ok(output) = Command::new("xrandr")
            .args(["--current"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Look for primary monitor or first connected monitor
            for line in stdout.lines() {
                if line.contains(" connected") && (line.contains("primary") || !stdout.contains("primary")) {
                    // Parse resolution like "1920x1080+0+0"
                    if let Some(res) = line.split_whitespace()
                        .find(|s| s.contains('x') && s.contains('+'))
                    {
                        let parts: Vec<&str> = res.split(|c| c == 'x' || c == '+').collect();
                        if parts.len() >= 2 {
                            info.width = parts[0].parse().unwrap_or(1920.0);
                            info.height = parts[1].parse().unwrap_or(1080.0);
                        }
                    }
                    break;
                }
            }
        }
        
        // Try to get DPI from Xft.dpi
        if let Ok(output) = Command::new("xrdb")
            .args(["-query"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.starts_with("Xft.dpi:") {
                    if let Some(dpi) = line.split(':').nth(1) {
                        info.dpi = dpi.trim().parse().unwrap_or(96.0);
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        // Use system_profiler to get display info
        if let Ok(output) = Command::new("system_profiler")
            .args(["SPDisplaysDataType"])
            .output()
        {
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
        
        info.dpi = 72.0; // macOS default
    }
    
    tracing::info!("Detected screen: {}x{} @ {} DPI", info.width, info.height, info.dpi);
    info
}

/// Calculate centered window position
fn calculate_centered_position(screen: &ScreenInfo, window_width: f32, window_height: f32) -> (f32, f32) {
    let x = (screen.width - window_width) / 2.0;
    let y = (screen.height - window_height) / 2.0;
    
    // Ensure position is not negative
    let x = x.max(0.0);
    let y = y.max(0.0);
    
    tracing::info!("Centered position: ({}, {}) for window {}x{}", x, y, window_width, window_height);
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
    use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
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
\x20 validate <root>            Validate the FOMOD in <root> (project + ModConfig 5.0 schema)\n\
\x20 package  <root> [-o FILE]  Write the FOMOD XML, then build a distribution .zip\n\
\x20 build    <root>            (Re)write fomod/info.xml and fomod/ModuleConfig.xml\n\
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
        "validate" => cli_validate(args.get(1)),
        "package" => cli_package(&args[1..]),
        "build" => cli_build(args.get(1)),
        other => {
            eprintln!("Unknown command: {other}\n");
            cli_usage();
            2
        }
    }
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

fn cli_validate(root: Option<&String>) -> i32 {
    let (_root, ximod) = match cli_load(root) {
        Ok(v) => v,
        Err(c) => return c,
    };
    let mut problems = 0;
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
            println!(
                "[info.xml] line {}, col {}: {}",
                issue.line, issue.column, issue.kind
            );
            problems += 1;
        }
    }
    if problems == 0 {
        println!("OK — the FOMOD conforms to the schema.");
        0
    } else {
        eprintln!("{problems} problem(s) found.");
        1
    }
}

fn cli_build(root: Option<&String>) -> i32 {
    let (root, ximod) = match cli_load(root) {
        Ok(v) => v,
        Err(c) => return c,
    };
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
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ximod_architect=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("{} v{} starting...", APP_NAME, APP_VERSION);

    // Step 1: Load configuration
    let config = config::AppConfig::load().unwrap_or_default();
    let window_width = if config.window_width > 0.0 { config.window_width } else { DEFAULT_WINDOW_WIDTH };
    let window_height = if config.window_height > 0.0 { config.window_height } else { DEFAULT_WINDOW_HEIGHT };
    
    tracing::info!("Window size from config: {}x{}", window_width, window_height);
    
    // Step 2: Detect primary screen dimensions
    let screen = detect_primary_screen();
    
    // Step 3: Show native transparent splash screen BEFORE creating main window
    let splash_enabled = config.splash_screen_seconds > 0;
    let splash_path = get_splash_path();
    let splash_exists = splash_path.as_ref().map(|p| p.exists()).unwrap_or(false);
    
    if splash_enabled && splash_exists {
        if let Some(path) = splash_path {
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
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(move |cc| Ok(Box::new(ui::XimodApp::new(cc, false, screen)))),
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
        let screen = ScreenInfo { width: 1920.0, height: 1080.0, dpi: 96.0 };
        let (x, y) = calculate_centered_position(&screen, 1280.0, 800.0);
        assert_eq!(x, 320.0);
        assert_eq!(y, 140.0);
    }
}
