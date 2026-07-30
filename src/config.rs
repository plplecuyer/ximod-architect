//! Application configuration
//!
//! Handles settings persistence and script execution
//! Migrated from TSettings in C++

use std::collections::HashMap;
use std::path::PathBuf;

/// Application theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
            Self::System => "System",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => Self::Light,
            "system" => Self::System,
            _ => Self::Dark,
        }
    }

    pub fn variants() -> &'static [Self] {
        &[Self::Dark, Self::Light, Self::System]
    }

    /// Detect system theme (Windows/Linux/macOS)
    pub fn detect_system_theme() -> Self {
        // Try to detect system theme
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            #[cfg(windows)]
            use std::os::windows::process::CommandExt;
            
            // CREATE_NO_WINDOW flag to prevent console window flash
            #[cfg(windows)]
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            
            // Check Windows dark mode setting via registry
            let mut cmd = Command::new("reg");
            cmd.args([
                "query",
                "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
                "/v",
                "AppsUseLightTheme",
            ]);
            
            #[cfg(windows)]
            cmd.creation_flags(CREATE_NO_WINDOW);
            
            if let Ok(output) = cmd.output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("0x0") {
                    return Self::Dark;
                } else if stdout.contains("0x1") {
                    return Self::Light;
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Check GTK theme or environment variable
            if let Ok(theme) = std::env::var("GTK_THEME") {
                if theme.to_lowercase().contains("dark") {
                    return Self::Dark;
                } else {
                    return Self::Light;
                }
            }
            // Check gsettings for GNOME
            if let Ok(output) = std::process::Command::new("gsettings")
                .args(["get", "org.gnome.desktop.interface", "color-scheme"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("dark") {
                    return Self::Dark;
                } else if stdout.contains("light") {
                    return Self::Light;
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            // Check macOS dark mode
            if let Ok(output) = std::process::Command::new("defaults")
                .args(["read", "-g", "AppleInterfaceStyle"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.to_lowercase().contains("dark") {
                    return Self::Dark;
                }
            }
            return Self::Light;
        }

        // Default to Dark if detection fails
        Self::Dark
    }
}

/// Application configuration (TSettings from C++)
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Language/locale code (ISO 639-3)
    pub locale: String,

    /// Selected country (ISO 3166-1 alpha-3), used to scope the language list
    /// and to display the flag in the settings. Empty when not chosen yet.
    pub country: String,

    /// UI theme
    pub theme: Theme,

    /// Font size for interface text
    pub font_size: f32,

    /// Maximum number of recent files
    pub max_recent_files: usize,

    /// Recent file paths
    pub recent_files: Vec<PathBuf>,

    /// Pre-save script content
    pub pre_save_script: String,

    /// Post-save script content
    pub post_save_script: String,

    /// Window width
    pub window_width: f32,

    /// Window height
    pub window_height: f32,

    /// Replace \r\n sequences in descriptions
    pub replace_newlines: bool,

    /// Splash screen duration in seconds
    pub splash_screen_seconds: u32,

    /// Whether the initial configuration has been completed.
    ///
    /// Stored as `FirstStart` in the INI file: `0` means the program has never
    /// been configured (it then starts in English and opens the settings window
    /// so the user can pick a country and language), `1` means the user has
    /// saved the settings at least once.
    pub first_start_done: bool,

    /// Saved on-screen positions (outer top-left, in points) of the free tool
    /// windows, keyed by viewport id (e.g. "ximod_translation"). A window with
    /// no saved position opens centered on the main window; once the user moves
    /// it, the new position is remembered here. The main window is never stored.
    pub window_positions: std::collections::HashMap<String, (f32, f32)>,

    /// Saved inner sizes (width, height, in points) of the free tool windows,
    /// keyed by viewport id. A window with no saved size opens at its default
    /// size; once resized, the new size is remembered here.
    pub window_sizes: std::collections::HashMap<String, (f32, f32)>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
            country: String::new(),
            theme: Theme::Dark,
            font_size: 14.0,
            max_recent_files: 10,
            recent_files: Vec::new(),
            pre_save_script: String::new(),
            post_save_script: String::new(),
            window_width: 1280.0,
            window_height: 800.0,
            replace_newlines: true,
            splash_screen_seconds: 2,
            first_start_done: false,
            window_positions: std::collections::HashMap::new(),
            window_sizes: std::collections::HashMap::new(),
        }
    }
}

impl AppConfig {
    /// Get the configuration directory path.
    ///
    /// Portable mode: if a `Config.ini` already sits next to the executable, that
    /// folder is used, so a portable/unzipped copy stays self-contained. Otherwise
    /// the per-user configuration directory is used — `%APPDATA%\XIMOD Architect` on
    /// Windows, `~/.config/XIMOD Architect` on Linux, and
    /// `~/Library/Application Support/XIMOD Architect` on macOS — so an application
    /// installed in a read-only location (e.g. `Program Files`, `/usr/local/bin`,
    /// `/Applications`) can still save its settings without administrator rights.
    pub fn config_dir() -> Option<PathBuf> {
        // Portable mode: an existing Config.ini next to the executable wins.
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                if dir.join("Config.ini").is_file() {
                    return Some(dir.to_path_buf());
                }
            }
        }
        // Installed mode (default): per-user configuration directory.
        if let Some(base) = dirs::config_dir() {
            return Some(base.join("XIMOD Architect"));
        }
        // Last resort: next to the executable.
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
    }

    /// Get the configuration file path
    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("Config.ini"))
    }

    /// Load configuration from INI file
    /// Creates the file with default values if it doesn't exist
    pub fn load() -> anyhow::Result<Self> {
        let path =
            Self::config_path().ok_or_else(|| anyhow::anyhow!("Could not determine config path"))?;

        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let config = Self::parse_ini(&content);
            Ok(config)
        } else {
            // Create default config and save it
            let config = Self::default();
            match config.save() {
                Err(e) => tracing::warn!("Failed to create default Config.ini: {}", e),
                _ => tracing::info!("Created default Config.ini at {:?}", path),
            }
            Ok(config)
        }
    }

    /// Parse INI format content
    fn parse_ini(content: &str) -> Self {
        let mut config = Self::default();
        let mut values: HashMap<String, String> = HashMap::new();

        // Tolerate a leading UTF-8 BOM (EF BB BF) left by editors such as
        // Notepad++ when saving "UTF-8 with BOM"; otherwise it would stick to
        // the first key and that setting would be silently ignored.
        let content = content.strip_prefix('\u{feff}').unwrap_or(content);

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }

            // Skip section headers
            if line.starts_with('[') && line.ends_with(']') {
                continue;
            }

            // Parse key=value
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim().to_string();
                let value = line[pos + 1..].trim().to_string();
                values.insert(key, value);
            }
        }

        // Apply values to config
        if let Some(v) = values.get("Country") {
            config.country = v.clone();
        }
        if let Some(v) = values.get("FirstStart") {
            config.first_start_done = v.trim() != "0";
        }
        if let Some(v) = values.get("Locale") {
            config.locale = v.clone();
        }
        if let Some(v) = values.get("Theme") {
            config.theme = Theme::from_str(v);
        }
        if let Some(v) = values.get("FontSize") {
            config.font_size = v.parse().unwrap_or(14.0);
        }
        if let Some(v) = values.get("MaxRecentFiles") {
            config.max_recent_files = v.parse().unwrap_or(10);
        }
        if let Some(v) = values.get("ReplaceNewlines") {
            config.replace_newlines = v == "1" || v.to_lowercase() == "true";
        }
        if let Some(v) = values.get("SplashScreenSeconds") {
            config.splash_screen_seconds = v.parse().unwrap_or(2);
        }
        if let Some(v) = values.get("WindowWidth") {
            config.window_width = v.parse().unwrap_or(1280.0);
        }
        if let Some(v) = values.get("WindowHeight") {
            config.window_height = v.parse().unwrap_or(800.0);
        }
        if let Some(v) = values.get("PreSaveScript") {
            config.pre_save_script = v.replace("\\n", "\n");
        }
        if let Some(v) = values.get("PostSaveScript") {
            config.post_save_script = v.replace("\\n", "\n");
        }

        // Parse saved free-window positions (WinPos_<id>=x,y) and sizes
        // (WinSize_<id>=w,h).
        for (k, v) in &values {
            let parse_pair = || -> Option<(f32, f32)> {
                let (a, b) = v.split_once(',')?;
                Some((a.trim().parse().ok()?, b.trim().parse().ok()?))
            };
            if let Some(id) = k.strip_prefix("WinPos_") {
                if let Some(p) = parse_pair() {
                    config.window_positions.insert(id.to_string(), p);
                }
            } else if let Some(id) = k.strip_prefix("WinSize_") {
                if let Some(p) = parse_pair() {
                    config.window_sizes.insert(id.to_string(), p);
                }
            }
        }

        // Parse recent files (RecentFile0, RecentFile1, etc.)
        config.recent_files.clear();
        for i in 0..config.max_recent_files {
            let key = format!("RecentFile{}", i);
            if let Some(v) = values.get(&key) {
                if !v.is_empty() {
                    config.recent_files.push(PathBuf::from(v));
                }
            }
        }

        config
    }

    /// Save configuration to INI file
    pub fn save(&self) -> anyhow::Result<()> {
        let dir =
            Self::config_dir().ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        std::fs::create_dir_all(&dir)?;

        let path = dir.join("Config.ini");
        let content = self.to_ini();
        std::fs::write(&path, content)?;

        Ok(())
    }

    /// Convert config to INI format
    fn to_ini(&self) -> String {
        let mut lines = Vec::new();

        lines.push("; XIMOD Architect Configuration".to_string());
        lines.push("; Do not edit manually unless you know what you're doing".to_string());
        lines.push(String::new());

        lines.push("[General]".to_string());
        lines.push(format!("Locale={}", self.locale));
        lines.push(format!("Country={}", self.country));
        lines.push(format!(
            "FirstStart={}",
            if self.first_start_done { 1 } else { 0 }
        ));
        lines.push(format!("Theme={}", self.theme.as_str()));
        lines.push(format!("FontSize={}", self.font_size));
        lines.push(format!(
            "ReplaceNewlines={}",
            if self.replace_newlines { "1" } else { "0" }
        ));
        lines.push(format!("MaxRecentFiles={}", self.max_recent_files));
        lines.push(format!("SplashScreenSeconds={}", self.splash_screen_seconds));
        lines.push(String::new());

        lines.push("[Window]".to_string());
        lines.push(format!("WindowWidth={}", self.window_width));
        lines.push(format!("WindowHeight={}", self.window_height));
        lines.push(String::new());

        lines.push("[Scripts]".to_string());
        lines.push(format!(
            "PreSaveScript={}",
            self.pre_save_script.replace('\n', "\\n")
        ));
        lines.push(format!(
            "PostSaveScript={}",
            self.post_save_script.replace('\n', "\\n")
        ));
        lines.push(String::new());

        lines.push("[RecentFiles]".to_string());
        for (i, path) in self.recent_files.iter().enumerate() {
            lines.push(format!("RecentFile{}={}", i, path.display()));
        }
        lines.push(String::new());

        lines.push("[WindowPositions]".to_string());
        let mut pos_ids: Vec<&String> = self.window_positions.keys().collect();
        pos_ids.sort();
        for id in pos_ids {
            let (x, y) = self.window_positions[id];
            lines.push(format!("WinPos_{}={},{}", id, x, y));
        }
        let mut size_ids: Vec<&String> = self.window_sizes.keys().collect();
        size_ids.sort();
        for id in size_ids {
            let (w, h) = self.window_sizes[id];
            lines.push(format!("WinSize_{}={},{}", id, w, h));
        }

        lines.join("\n")
    }

    /// Add a file to recent files list
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);

        // Add to front
        self.recent_files.insert(0, path);

        // Trim to max size
        while self.recent_files.len() > self.max_recent_files {
            self.recent_files.pop();
        }
    }

    /// Remove a file from recent files list
    #[allow(dead_code)]
    pub fn remove_recent_file(&mut self, path: &PathBuf) {
        self.recent_files.retain(|p| p != path);
    }
}

/// Script macros for pre/post save scripts
/// Mirrors the macro replacement from RunBatFile in C++
pub struct ScriptMacros {
    pub mod_name: String,
    pub mod_author: String,
    pub mod_version: String,
    pub mod_root: String,
}

impl ScriptMacros {
    pub fn new(name: &str, author: &str, version: &str, root: &str) -> Self {
        Self {
            mod_name: name.to_string(),
            mod_author: author.to_string(),
            mod_version: version.to_string(),
            mod_root: root.to_string(),
        }
    }

    /// Apply macros to a string
    pub fn apply(&self, input: &str) -> String {
        let now = chrono::Local::now();

        input
            .replace("$MODNAME$", &self.mod_name)
            .replace("$MODAUTHOR$", &self.mod_author)
            .replace("$MODVERSION$", &self.mod_version)
            .replace("$MODROOT$", &self.mod_root)
            .replace("$DATE$", &now.format("%Y-%m-%d").to_string())
            .replace("$TIME$", &now.format("%H:%M:%S").to_string())
            .replace("$RANDOM$", &format!("{}", rand::random::<u16>()))
    }
}

/// Run a script with macro replacement
/// Mirrors RunBatFile from C++
pub fn run_script(script_content: &str, macros: &ScriptMacros) -> anyhow::Result<()> {
    if script_content.trim().is_empty() {
        return Ok(());
    }

    // Apply macros
    let processed = macros.apply(script_content);

    // Create temporary script file
    let temp_dir = std::env::temp_dir();
    let temp_script = if cfg!(windows) {
        temp_dir.join("ximod_temp_script.bat")
    } else {
        temp_dir.join("ximod_temp_script.sh")
    };

    std::fs::write(&temp_script, &processed)?;

    // Execute script
    #[cfg(windows)]
    {
        std::process::Command::new("cmd")
            .args(["/c", temp_script.to_str().unwrap()])
            .status()?;
    }

    #[cfg(not(windows))]
    {
        // Make script executable
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&temp_script)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&temp_script, perms)?;

        std::process::Command::new("sh")
            .arg(&temp_script)
            .status()?;
    }

    // Clean up
    let _ = std::fs::remove_file(&temp_script);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_replacement() {
        let macros = ScriptMacros::new("Test Mod", "Author", "1.0", "/path/to/mod");
        let result = macros.apply("echo $MODNAME$ by $MODAUTHOR$");
        assert!(result.contains("Test Mod"));
        assert!(result.contains("Author"));
    }

    #[test]
    fn test_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.locale, "en");
        assert_eq!(config.max_recent_files, 10);
    }

    #[test]
    fn test_ini_parse() {
        let ini = r#"
[General]
Locale=fr
Theme=Light

[RecentFiles]
RecentFile0=C:\path\to\mod1
RecentFile1=C:\path\to\mod2
"#;
        let config = AppConfig::parse_ini(ini);
        assert_eq!(config.locale, "fr");
        assert_eq!(config.theme, Theme::Light);
        assert_eq!(config.recent_files.len(), 2);
    }

    #[test]
    fn test_ini_roundtrip() {
        let mut config = AppConfig::default();
        config.locale = "fr".to_string();
        config.theme = Theme::Light;
        config.recent_files.push(PathBuf::from("/test/path"));

        let ini = config.to_ini();
        let parsed = AppConfig::parse_ini(&ini);

        assert_eq!(parsed.locale, "fr");
        assert_eq!(parsed.theme, Theme::Light);
        assert_eq!(parsed.recent_files.len(), 1);
    }
}
