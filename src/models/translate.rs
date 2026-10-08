//! Translation of an existing FOMOD (data model, language-neutral).
//!
//! A translator does not own the installer being translated, so the work is
//! kept apart from it, in a *sidecar* file (`*.ximod-translation`, JSON) stored
//! under `fomod/translations/`. The sidecar lists every translatable string of
//! the FOMOD as a [`TUnit`] addressed by a stable key (see
//! [`crate::xml::patch`]), with its translation and review status.
//!
//! This module holds the document model, its validation, the merge performed
//! when the source FOMOD is updated, and the projection of a translation onto
//! the in-memory project model. It has no dependency on the UI or on i18n:
//! issues are plain enums that the UI maps to messages.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

/// Version of the sidecar file format written by this build.
pub const SIDECAR_FORMAT: u32 = 1;

/// File extension of a sidecar (without the dot).
pub const SIDECAR_EXTENSION: &str = "ximod-translation";

/// Which of the two FOMOD files a string belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TFile {
    /// `fomod/info.xml`
    Info,
    /// `fomod/ModuleConfig.xml`
    Config,
}

/// Kind of translatable string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TField {
    InfoName,
    InfoAuthor,
    InfoWebsite,
    InfoDescription,
    ModuleName,
    StepName,
    GroupName,
    PluginName,
    PluginDescription,
}

impl TField {
    /// File the field is stored in.
    pub fn file(self) -> TFile {
        match self {
            Self::InfoName | Self::InfoAuthor | Self::InfoWebsite | Self::InfoDescription => TFile::Info,
            _ => TFile::Config,
        }
    }

    /// Descriptions may span several lines; names may not.
    pub fn is_multiline(self) -> bool {
        matches!(self, Self::InfoDescription | Self::PluginDescription)
    }

    /// Fields that are normally kept as they are (author, website).
    pub fn locked_by_default(self) -> bool {
        matches!(self, Self::InfoAuthor | Self::InfoWebsite)
    }

    /// `true` when the string is stored in an XML attribute (step, group and
    /// plugin names) rather than as the text content of an element.
    pub fn is_attribute(self) -> bool {
        matches!(self, Self::StepName | Self::GroupName | Self::PluginName)
    }
}

/// Review status of a unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TStatus {
    /// Not translated yet.
    #[default]
    Untranslated,
    /// Translated and accepted by the translator.
    Translated,
    /// Filled in automatically (duplicate, moved string); to be reviewed.
    Auto,
    /// The source changed since the translation was written.
    Fuzzy,
    /// The string no longer exists in the FOMOD (kept in `obsolete`).
    Obsolete,
}

/// One translatable string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TUnit {
    /// Stable address of the string in the FOMOD (e.g. `config/step[0]/name`).
    pub key: String,
    pub field: TField,
    /// Text found in the source FOMOD.
    pub source: String,
    /// Translation (empty while untranslated).
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub status: TStatus,
    /// A locked unit is left as it is unless it has an explicit target.
    #[serde(default)]
    pub locked: bool,
    /// Breadcrumb locating the string ("Step › Group › Plugin").
    #[serde(default)]
    pub context: String,
    /// Free translator note.
    #[serde(default)]
    pub note: String,
}

impl TUnit {
    /// Text to write in the translated FOMOD: the target, or the source while
    /// there is no target.
    pub fn effective_target(&self) -> &str {
        if self.target.is_empty() {
            &self.source
        } else {
            &self.target
        }
    }

    /// A unit needs no more work once it is translated, or when it is locked
    /// without a target (kept as in the source on purpose).
    pub fn is_done(&self) -> bool {
        self.status == TStatus::Translated || (self.locked && self.target.is_empty())
    }
}

/// Identity of one source file at the time the units were extracted.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFingerprint {
    pub crc32: u32,
    pub bytes: u64,
    /// Encoding label (see `xml::patch::Encoding::label`).
    pub encoding: String,
}

/// Fingerprints of the two source files (absent files are `None`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFingerprint {
    #[serde(default)]
    pub info: Option<FileFingerprint>,
    #[serde(default)]
    pub config: Option<FileFingerprint>,
}

/// Where the translated FOMOD is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ExportMode {
    /// Next to the original (`fomod_<lang>/`), leaving the original untouched.
    #[default]
    Sibling,
    /// Over the original files (after a backup).
    InPlace,
    /// As a packaged archive.
    Package,
}

/// Export preferences remembered with the translation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ExportPrefs {
    pub mode: ExportMode,
    /// Name of the exported package; `{name}` and `{LANG}` are substituted.
    pub name_template: String,
    /// Set existing `order` attributes to `Explicit` in the translated file.
    pub force_explicit_order: bool,
    /// Append ` [LANG]` to the mod name (`Name` and `moduleName`).
    pub suffix_name: bool,
}

impl Default for ExportPrefs {
    fn default() -> Self {
        Self {
            mode: ExportMode::Sibling,
            name_template: "{name}_{LANG}".to_string(),
            force_explicit_order: true,
            suffix_name: false,
        }
    }
}

/// Counters over the units of a document.
///
/// `translated + auto + fuzzy + untranslated == total`: `translated` counts
/// the units that are done ([`TUnit::is_done`], which includes locked units
/// deliberately left as in the source), the three others split the remaining
/// units by status. `locked` is independent of that partition.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TStats {
    pub total: usize,
    pub translated: usize,
    pub auto: usize,
    pub fuzzy: usize,
    pub untranslated: usize,
    pub locked: usize,
}

impl TStats {
    /// Completion in percent (100 for an empty document), rounded down so that
    /// 100 % is only shown when everything is done.
    pub fn percent(&self) -> u32 {
        (self.translated * 100)
            .checked_div(self.total)
            .map_or(100, |p| p as u32)
    }
}

/// A translation sidecar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationDoc {
    /// Sidecar format version ([`SIDECAR_FORMAT`]).
    pub format: u32,
    /// Tool that wrote the file.
    pub tool: String,
    pub source_lang: String,
    pub target_lang: String,
    pub translator: String,
    pub mod_name: String,
    pub mod_version: String,
    pub source: SourceFingerprint,
    /// RFC 3339 timestamps.
    pub created: String,
    pub updated: String,
    pub units: Vec<TUnit>,
    /// Translations whose source string disappeared, kept for reuse.
    #[serde(default)]
    pub obsolete: Vec<TUnit>,
    #[serde(default)]
    pub export: ExportPrefs,
}

/// Current time as an RFC 3339 UTC timestamp.
fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Turn free text into a safe file-name component.
pub(crate) fn sanitize_file_stem(name: &str) -> String {
    let cleaned: String = name
        .trim()
        .chars()
        .map(|c| {
            if c.is_control() || r#"\/:*?"<>|"#.contains(c) {
                '_'
            } else {
                c
            }
        })
        .collect();
    cleaned
        .trim_matches(|c: char| c == '.' || c.is_whitespace())
        .to_string()
}

impl TranslationDoc {
    /// Empty document for a language pair.
    pub fn new(source_lang: impl Into<String>, target_lang: impl Into<String>) -> Self {
        let now = now_rfc3339();
        Self {
            format: SIDECAR_FORMAT,
            tool: format!("{} {}", crate::APP_NAME, crate::APP_VERSION),
            source_lang: source_lang.into(),
            target_lang: target_lang.into(),
            translator: String::new(),
            mod_name: String::new(),
            mod_version: String::new(),
            source: SourceFingerprint::default(),
            created: now.clone(),
            updated: now,
            units: Vec::new(),
            obsolete: Vec::new(),
            export: ExportPrefs::default(),
        }
    }

    /// Progress counters (see [`TStats`]).
    pub fn stats(&self) -> TStats {
        let mut s = TStats {
            total: self.units.len(),
            ..TStats::default()
        };
        for u in &self.units {
            if u.locked {
                s.locked += 1;
            }
            if u.is_done() {
                s.translated += 1;
            } else {
                match u.status {
                    TStatus::Auto => s.auto += 1,
                    TStatus::Fuzzy => s.fuzzy += 1,
                    _ => s.untranslated += 1,
                }
            }
        }
        s
    }

    /// Key → text to write, for every unit that has a target. Units without a
    /// target (untranslated, or locked and left as in the source) are absent,
    /// so the patcher does not touch them.
    pub fn target_map(&self) -> HashMap<String, String> {
        self.units
            .iter()
            .filter(|u| !u.target.is_empty())
            .map(|u| (u.key.clone(), u.effective_target().to_string()))
            .collect()
    }

    /// [`Self::target_map`] with the export preferences applied: when
    /// `export.suffix_name` is set, ` [LANG]` (upper-cased target language) is
    /// appended to the mod name in both files, translated or not.
    pub fn export_targets(&self) -> HashMap<String, String> {
        let mut map = self.target_map();
        if self.export.suffix_name && !self.target_lang.trim().is_empty() {
            let suffix = format!(" [{}]", self.target_lang.trim().to_uppercase());
            for u in &self.units {
                if matches!(u.field, TField::InfoName | TField::ModuleName) && !u.effective_target().ends_with(&suffix)
                {
                    map.insert(u.key.clone(), format!("{}{suffix}", u.effective_target()));
                }
            }
        }
        map
    }

    /// Read a sidecar file.
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
        let text = text.strip_prefix('\u{FEFF}').unwrap_or(&text);
        let doc: Self = serde_json::from_str(text)
            .with_context(|| format!("{} is not a valid translation file", path.display()))?;
        if doc.format > SIDECAR_FORMAT {
            bail!(
                "{} was written by a newer version (format {}, this version reads up to {})",
                path.display(),
                doc.format,
                SIDECAR_FORMAT
            );
        }
        Ok(doc)
    }

    /// Write the sidecar as pretty JSON, stamping `updated` with the current
    /// time. The file is written next to its destination and renamed into
    /// place, so an interrupted save never truncates an existing translation.
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut doc = self.clone();
        doc.updated = now_rfc3339();
        let mut json = serde_json::to_string_pretty(&doc).context("serializing the translation")?;
        json.push('\n');

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        write_atomic(path, json.as_bytes())
    }

    /// Default location of the sidecar of a mod:
    /// `<root>/fomod/translations/<mod name>.<lang>.ximod-translation`
    /// (`fomod` when the mod has no usable name). An existing `fomod` folder
    /// spelled with another case is reused.
    pub fn sidecar_path(root: &Path, mod_name: &str, target_lang: &str) -> PathBuf {
        let mut stem = sanitize_file_stem(mod_name);
        if stem.is_empty() {
            stem = "fomod".to_string();
        }
        let mut lang: String = target_lang
            .trim()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        if lang.is_empty() {
            lang = "und".to_string();
        }
        translations_dir(root).join(format!("{stem}.{lang}.{SIDECAR_EXTENSION}"))
    }

    /// Sidecars found in `<root>/fomod/translations`, sorted by path.
    pub fn find_sidecars(root: &Path) -> Vec<PathBuf> {
        let suffix = format!(".{SIDECAR_EXTENSION}");
        let mut found: Vec<PathBuf> = std::fs::read_dir(translations_dir(root))
            .map(|entries| {
                entries
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| {
                        p.is_file()
                            && p.file_name()
                                .map(|n| n.to_string_lossy().to_lowercase().ends_with(&suffix))
                                .unwrap_or(false)
                    })
                    .collect()
            })
            .unwrap_or_default();
        found.sort();
        found
    }
}

/// `<root>/fomod/translations`, reusing the existing spelling of `fomod`.
fn translations_dir(root: &Path) -> PathBuf {
    crate::xml::patch::locate_fomod_files(root).dir.join("translations")
}

/// Write `bytes` to `path` through a temporary sibling and a rename.
pub(crate) fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    use std::io::Write;
    let file_name = path
        .file_name()
        .with_context(|| format!("{} has no file name", path.display()))?
        .to_string_lossy()
        .into_owned();
    let tmp = path.with_file_name(format!(".{file_name}.{}.tmp", std::process::id()));
    let result = (|| -> Result<()> {
        let mut f = std::fs::File::create(&tmp).with_context(|| format!("creating {}", tmp.display()))?;
        f.write_all(bytes)?;
        f.sync_all().ok();
        drop(f);
        std::fs::rename(&tmp, path).with_context(|| format!("writing {}", path.display()))
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// A problem found in a translation. Language-neutral: the UI maps each kind
/// to a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum TIssue {
    /// Marked as translated (or auto/fuzzy) but the target is empty.
    EmptyTarget,
    /// The target only contains whitespace.
    WhitespaceOnly,
    /// Leading/trailing whitespace differs from the source.
    EdgeWhitespaceMismatch,
    /// Protected tokens (see [`tokens`]) differ between source and target.
    TokenMismatch { missing: Vec<String>, extra: Vec<String> },
    /// A line break in a single-line field (step, group, plugin, mod name…).
    NewlineInName,
    /// Characters that XML cannot represent.
    ControlChars,
    /// The target is suspiciously longer or shorter than the source.
    LengthRatio { ratio: f32 },
    /// The target is the same text as the source.
    IdenticalToSource,
    /// Another unit with the same source has a different translation.
    InconsistentDuplicate { other_key: String },
    /// `]]>` in text content (cannot be written in a CDATA section).
    CdataTerminator,
    /// The source contains a glossary term (`term` is its source form) but
    /// the target lacks the expected translation (see [`glossary_issues`]).
    GlossaryViolation { term: String },
}

impl TIssue {
    /// Blocking issues would produce a broken installer; the others are hints.
    pub fn is_blocking(&self) -> bool {
        match self {
            Self::NewlineInName | Self::ControlChars | Self::CdataTerminator => true,
            Self::TokenMismatch { missing, .. } => missing.iter().any(|t| t == "\\n" || t == "\\r\\n"),
            _ => false,
        }
    }

    /// Stable identifier of the kind (the `kind` tag of the JSON form), handy
    /// for counting and for building i18n keys.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::EmptyTarget => "emptyTarget",
            Self::WhitespaceOnly => "whitespaceOnly",
            Self::EdgeWhitespaceMismatch => "edgeWhitespaceMismatch",
            Self::TokenMismatch { .. } => "tokenMismatch",
            Self::NewlineInName => "newlineInName",
            Self::ControlChars => "controlChars",
            Self::LengthRatio { .. } => "lengthRatio",
            Self::IdenticalToSource => "identicalToSource",
            Self::InconsistentDuplicate { .. } => "inconsistentDuplicate",
            Self::CdataTerminator => "cdataTerminator",
            Self::GlossaryViolation { .. } => "glossaryViolation",
        }
    }
}

/// File extensions whose names must be copied verbatim.
const FILE_EXTENSIONS: [&str; 7] = [".esp", ".esm", ".esl", ".bsa", ".ba2", ".dds", ".nif"];

/// Does `chars[at..]` start with the ASCII string `pat` (case-insensitive)?
fn starts_with_at(chars: &[char], at: usize, pat: &str) -> bool {
    let mut i = at;
    for p in pat.chars() {
        match chars.get(i) {
            Some(c) if c.eq_ignore_ascii_case(&p) => i += 1,
            _ => return false,
        }
    }
    true
}

/// Characters that may be part of a file name written in running text.
fn is_file_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '.')
}

/// Protected tokens of a string, in order of appearance (duplicates kept):
/// things a translation must reproduce exactly.
///
/// * the escape sequences `\n` and `\r\n` written literally (a backslash
///   followed by a letter), which some installers use as line breaks;
/// * URLs (`http://…`, `https://…`);
/// * numbers with at least two digits (`2048`, `1.5.97`);
/// * game file names (`*.esp`, `*.esm`, `*.esl`, `*.bsa`, `*.ba2`, `*.dds`,
///   `*.nif`);
/// * markup tags (`<b>`, `</font>`, `<br/>`);
/// * placeholders in braces (`{name}`).
pub fn tokens(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let collect = |from: usize, to: usize| chars[from..to].iter().collect::<String>();
    let mut out = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Literal escape sequences.
        if starts_with_at(&chars, i, "\\r\\n") {
            out.push("\\r\\n".to_string());
            i += 4;
            continue;
        }
        if c == '\\' && chars.get(i + 1) == Some(&'n') {
            out.push("\\n".to_string());
            i += 2;
            continue;
        }

        // URLs.
        if starts_with_at(&chars, i, "http://") || starts_with_at(&chars, i, "https://") {
            let mut end = i;
            while end < chars.len()
                && !chars[end].is_whitespace()
                && !matches!(chars[end], '<' | '>' | '"' | '\'' | '[' | ']' | '{' | '}')
            {
                end += 1;
            }
            // Sentence punctuation right after a URL is not part of it.
            while end > i && matches!(chars[end - 1], '.' | ',' | ';' | ':' | '!' | '?' | ')' | '(') {
                end -= 1;
            }
            out.push(collect(i, end));
            i = end.max(i + 1);
            continue;
        }

        // Markup tags.
        if c == '<' && chars.get(i + 1).is_some_and(|n| n.is_ascii_alphabetic() || *n == '/') {
            let limit = (i + 80).min(chars.len());
            let close = (i + 1..limit).find(|&j| matches!(chars[j], '>' | '<' | '\n'));
            if let Some(j) = close
                && chars[j] == '>'
            {
                out.push(collect(i, j + 1));
                i = j + 1;
                continue;
            }
        }

        // Placeholders.
        if c == '{' {
            let limit = (i + 40).min(chars.len());
            let close =
                (i + 1..limit).find(|&j| !(chars[j].is_alphanumeric() || matches!(chars[j], '_' | '-' | '.' | '$')));
            if let Some(j) = close
                && chars[j] == '}'
                && j > i + 1
            {
                out.push(collect(i, j + 1));
                i = j + 1;
                continue;
            }
        }

        // File names (checked at the start of a word).
        if is_file_char(c) && c != '.' && (i == 0 || !is_file_char(chars[i - 1])) {
            let mut end = i;
            while end < chars.len() && is_file_char(chars[end]) {
                end += 1;
            }
            while end > i && chars[end - 1] == '.' {
                end -= 1;
            }
            let word = collect(i, end);
            let lower = word.to_lowercase();
            if FILE_EXTENSIONS
                .iter()
                .any(|ext| lower.ends_with(ext) && lower.len() > ext.len())
            {
                out.push(word);
                i = end;
                continue;
            }
        }

        // Numbers.
        if c.is_ascii_digit() {
            let mut end = i;
            let mut digits = 0;
            while end < chars.len() {
                if chars[end].is_ascii_digit() {
                    digits += 1;
                    end += 1;
                } else if matches!(chars[end], '.' | ',') && chars.get(end + 1).is_some_and(|n| n.is_ascii_digit()) {
                    end += 1;
                } else {
                    break;
                }
            }
            if digits >= 2 {
                out.push(collect(i, end));
            }
            i = end;
            continue;
        }

        i += 1;
    }
    out
}

/// Multiset difference `a − b`, keeping the order of `a`.
fn multiset_minus(a: &[String], b: &[String]) -> Vec<String> {
    let mut pool: HashMap<&str, usize> = HashMap::new();
    for t in b {
        *pool.entry(t.as_str()).or_default() += 1;
    }
    let mut out = Vec::new();
    for t in a {
        match pool.get_mut(t.as_str()) {
            Some(n) if *n > 0 => *n -= 1,
            _ => out.push(t.clone()),
        }
    }
    out
}

/// Leading and trailing whitespace of a string.
fn edge_whitespace(s: &str) -> (&str, &str) {
    let lead = &s[..s.len() - s.trim_start().len()];
    let trail = &s[s.trim_end().len()..];
    (lead, trail)
}

/// `true` for characters that cannot appear in an XML 1.0 document.
fn is_xml_illegal(c: char) -> bool {
    matches!(c, '\u{0}'..='\u{8}' | '\u{B}' | '\u{C}' | '\u{E}'..='\u{1F}' | '\u{FFFE}' | '\u{FFFF}')
}

/// Checks on a single unit.
///
/// A unit without a target has nothing to check: it yields no issue, except
/// [`TIssue::EmptyTarget`] when its status claims there is a translation
/// (translated, auto or fuzzy) and it is not locked.
pub fn validate_unit(u: &TUnit) -> Vec<TIssue> {
    let mut issues = Vec::new();

    if u.target.is_empty() {
        if !u.locked && !matches!(u.status, TStatus::Untranslated | TStatus::Obsolete) {
            issues.push(TIssue::EmptyTarget);
        }
        return issues;
    }
    if u.target.trim().is_empty() {
        issues.push(TIssue::WhitespaceOnly);
        return issues;
    }

    if u.target.chars().any(is_xml_illegal) {
        issues.push(TIssue::ControlChars);
    }
    if !u.field.is_multiline() && u.target.contains(['\n', '\r']) {
        issues.push(TIssue::NewlineInName);
    }
    if !u.field.is_attribute() && u.target.contains("]]>") {
        issues.push(TIssue::CdataTerminator);
    }

    let src_tokens = tokens(&u.source);
    let tgt_tokens = tokens(&u.target);
    let missing = multiset_minus(&src_tokens, &tgt_tokens);
    let extra = multiset_minus(&tgt_tokens, &src_tokens);
    if !missing.is_empty() || !extra.is_empty() {
        issues.push(TIssue::TokenMismatch { missing, extra });
    }

    if edge_whitespace(&u.source) != edge_whitespace(&u.target) {
        issues.push(TIssue::EdgeWhitespaceMismatch);
    }

    let src_len = u.source.chars().count();
    let tgt_len = u.target.chars().count();
    if src_len >= 12 {
        let ratio = tgt_len as f32 / src_len as f32;
        if !(0.2..=3.0).contains(&ratio) {
            issues.push(TIssue::LengthRatio { ratio });
        }
    }

    // Identical text is expected for locked units and for strings without any
    // letter ("4096", "---").
    if u.target == u.source && !u.locked && u.source.chars().any(char::is_alphabetic) {
        issues.push(TIssue::IdenticalToSource);
    }

    issues
}

/// Checks on a whole document: every [`validate_unit`] issue, plus
/// [`TIssue::InconsistentDuplicate`] on each unit whose source is translated
/// differently elsewhere (`other_key` is the first unit that disagrees).
pub fn validate_doc(doc: &TranslationDoc) -> Vec<(String, TIssue)> {
    let mut by_source: HashMap<&str, Vec<&TUnit>> = HashMap::new();
    for u in &doc.units {
        if !u.target.is_empty() {
            by_source.entry(u.source.as_str()).or_default().push(u);
        }
    }

    let mut out = Vec::new();
    for u in &doc.units {
        for issue in validate_unit(u) {
            out.push((u.key.clone(), issue));
        }
        if u.target.is_empty() {
            continue;
        }
        if let Some(other) = by_source
            .get(u.source.as_str())
            .and_then(|same| same.iter().find(|o| o.target != u.target))
        {
            out.push((
                u.key.clone(),
                TIssue::InconsistentDuplicate {
                    other_key: other.key.clone(),
                },
            ));
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Propagation and CSV exchange
// ---------------------------------------------------------------------------

/// Columns of the CSV exchange format, in order.
const CSV_HEADER: [&str; 8] = [
    "key", "field", "context", "source", "target", "status", "locked", "note",
];

/// Name of a unit enum (`TField`, `TStatus`) as written in the sidecar.
fn serde_name<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default()
}

/// Append one CSV field, quoted when RFC 4180 requires it (separator, quote
/// or line break inside) or when it has leading/trailing blanks that a
/// spreadsheet could otherwise drop.
pub(crate) fn push_csv_field(out: &mut String, field: &str) {
    let needs_quotes = field.contains([',', ';', '\t', '"', '\n', '\r'])
        || field.starts_with(char::is_whitespace)
        || field.ends_with(char::is_whitespace);
    if needs_quotes {
        out.push('"');
        for c in field.chars() {
            if c == '"' {
                out.push('"');
            }
            out.push(c);
        }
        out.push('"');
    } else {
        out.push_str(field);
    }
}

/// Split CSV text into records (RFC 4180): fields separated by `delimiter`,
/// records by `\n`, `\r\n` or `\r`; a field may be enclosed in double quotes,
/// in which case it may contain separators, line breaks and doubled quotes.
/// Blank lines are skipped. Fails on a quoted field that is never closed.
fn parse_csv(text: &str, delimiter: char) -> Result<Vec<Vec<String>>> {
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut record: Vec<String> = Vec::new();
    let mut field = String::new();
    // `quoted`: inside a quoted field; `was_quoted`: the current field had
    // quotes (so an empty value still counts as content on its line).
    let mut quoted = false;
    let mut was_quoted = false;
    let mut quote_line = 0usize;
    let mut line = 1usize;

    let mut end_record = |record: &mut Vec<String>, field: &mut String, was_quoted: &mut bool| {
        let blank = record.is_empty() && field.is_empty() && !*was_quoted;
        if !blank {
            record.push(std::mem::take(field));
            records.push(std::mem::take(record));
        }
        *was_quoted = false;
    };

    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\n' {
            line += 1;
        }
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    quoted = false;
                }
            } else {
                field.push(c);
            }
            continue;
        }
        match c {
            '"' if field.is_empty() && !was_quoted => {
                quoted = true;
                was_quoted = true;
                quote_line = line;
            }
            c if c == delimiter => {
                record.push(std::mem::take(&mut field));
                was_quoted = false;
            }
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                    line += 1;
                }
                end_record(&mut record, &mut field, &mut was_quoted);
            }
            '\n' => end_record(&mut record, &mut field, &mut was_quoted),
            // Lenient: text after a closing quote is kept as part of the field.
            other => field.push(other),
        }
    }
    if quoted {
        bail!("unterminated quoted field starting on line {quote_line}");
    }
    end_record(&mut record, &mut field, &mut was_quoted);
    Ok(records)
}

/// Separator used by a CSV file, guessed from its header line: spreadsheets
/// write `;` or a tab instead of `,` depending on the locale and the format.
fn detect_csv_delimiter(text: &str) -> char {
    let header = text.lines().next().unwrap_or_default();
    [',', ';', '\t']
        .into_iter()
        .find(|d| {
            header
                .split(*d)
                .any(|f| f.trim().trim_matches('"').eq_ignore_ascii_case("key"))
                && header.contains(*d)
        })
        .unwrap_or(',')
}

impl TranslationDoc {
    /// Copy the target of the unit `key` to every other unit that has the
    /// same source, no target yet and is not locked. The copies are marked
    /// [`TStatus::Auto`] (to be reviewed). Returns the number of units filled.
    pub fn propagate(&mut self, key: &str) -> usize {
        let Some((source, target)) = self
            .units
            .iter()
            .find(|u| u.key == key)
            .filter(|u| !u.target.is_empty())
            .map(|u| (u.source.clone(), u.target.clone()))
        else {
            return 0;
        };
        let mut count = 0;
        for u in &mut self.units {
            if u.key != key && u.source == source && u.target.is_empty() && !u.locked {
                u.target = target.clone();
                u.status = TStatus::Auto;
                count += 1;
            }
        }
        count
    }

    /// "Unique texts" mode: copy the translation and status of the unit `key`
    /// to every other unit with the same source text (locked units excepted),
    /// whether they were translated before or not, so identical texts stay
    /// identical. Returns how many units were updated.
    pub fn sync_identical(&mut self, key: &str) -> usize {
        let Some((source, target, status)) = self
            .units
            .iter()
            .find(|u| u.key == key)
            .map(|u| (u.source.clone(), u.target.clone(), u.status))
        else {
            return 0;
        };
        let mut count = 0;
        for u in &mut self.units {
            if u.key != key && u.source == source && !u.locked && (u.target != target || u.status != status) {
                u.target = target.clone();
                u.status = status;
                count += 1;
            }
        }
        count
    }

    /// Groups of units sharing the same source text: for each representative
    /// (first unit with that text), the number of units with that text.
    pub fn identical_groups(&self) -> std::collections::HashMap<usize, usize> {
        let mut first: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        let mut counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (i, u) in self.units.iter().enumerate() {
            let rep = *first.entry(u.source.as_str()).or_insert(i);
            *counts.entry(rep).or_insert(0) += 1;
        }
        counts
    }

    /// The units as CSV (RFC 4180, `\n` record ends), for translating in a
    /// spreadsheet. Columns: `key,field,context,source,target,status,locked,note`.
    pub fn to_csv(&self) -> String {
        let mut out = CSV_HEADER.join(",");
        out.push('\n');
        for u in &self.units {
            let field = serde_name(&u.field);
            let status = serde_name(&u.status);
            let columns: [&str; 8] = [
                &u.key,
                &field,
                &u.context,
                &u.source,
                &u.target,
                &status,
                if u.locked { "true" } else { "false" },
                &u.note,
            ];
            for (i, column) in columns.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                push_csv_field(&mut out, column);
            }
            out.push('\n');
        }
        out
    }

    /// Read back a CSV written by [`Self::to_csv`] (possibly edited in a
    /// spreadsheet) and update the units it names, by `key`:
    ///
    /// * `target`: a changed target replaces the current one, and the status
    ///   becomes `Translated` (non-empty) or `Untranslated` (emptied);
    /// * `note`: replaced when different;
    /// * `locked`: `true` / `false` (anything else is ignored).
    ///
    /// The first record must be a header naming at least the `key` column;
    /// columns are found by name, so their order and any extra column do not
    /// matter, and a missing column leaves that member untouched. The
    /// separator (`,`, `;` or tab) is detected from the header. Rows with an
    /// unknown key are ignored. Returns the number of units modified.
    pub fn import_csv(&mut self, csv: &str) -> Result<usize> {
        let csv = csv.strip_prefix('\u{FEFF}').unwrap_or(csv);
        let records = parse_csv(csv, detect_csv_delimiter(csv))?;
        let mut rows = records.into_iter();
        let header = rows.next().context("the CSV file is empty")?;
        let column = |name: &str| header.iter().position(|h| h.trim().eq_ignore_ascii_case(name));
        let key_col = column("key").context("the CSV file has no 'key' column")?;
        let (target_col, note_col, locked_col) = (column("target"), column("note"), column("locked"));

        let index: HashMap<String, usize> = self.units.iter().enumerate().map(|(i, u)| (u.key.clone(), i)).collect();
        let mut updated: HashSet<usize> = HashSet::new();
        for row in rows {
            let Some(&i) = row.get(key_col).and_then(|k| index.get(k.trim())) else {
                continue;
            };
            let unit = &mut self.units[i];
            let cell = |col: Option<usize>| col.and_then(|c| row.get(c));
            let mut changed = false;
            if let Some(target) = cell(target_col)
                && *target != unit.target
            {
                unit.target = target.clone();
                unit.status = if target.is_empty() {
                    TStatus::Untranslated
                } else {
                    TStatus::Translated
                };
                changed = true;
            }
            if let Some(note) = cell(note_col)
                && *note != unit.note
            {
                unit.note = note.clone();
                changed = true;
            }
            let locked = cell(locked_col).and_then(|l| match l.trim().to_ascii_lowercase().as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            });
            if let Some(locked) = locked
                && locked != unit.locked
            {
                unit.locked = locked;
                changed = true;
            }
            if changed {
                updated.insert(i);
            }
        }
        Ok(updated.len())
    }
}

// ---------------------------------------------------------------------------
// Translation memory and glossary (shared by every mod, per language pair)
// ---------------------------------------------------------------------------

/// Folder holding the translation memories and glossaries:
/// `<config dir>/translation_memory`. `None` when the configuration directory
/// cannot be resolved on this platform.
pub fn translation_memory_dir() -> Option<PathBuf> {
    crate::config::AppConfig::config_dir().map(|d| d.join("translation_memory"))
}

/// Language tag usable in a file name (`und` when nothing usable is left).
fn lang_file_slug(lang: &str) -> String {
    let slug: String = lang
        .trim()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect::<String>()
        .to_ascii_lowercase();
    if slug.is_empty() { "und".to_string() } else { slug }
}

/// Read a JSON file written by [`save_json`]; `None` when it is missing or
/// unreadable.
fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> Option<T> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(text.strip_prefix('\u{FEFF}').unwrap_or(&text)).ok()
}

/// Write `value` as pretty JSON through a temporary file and a rename.
fn save_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let mut json = serde_json::to_string_pretty(value).context("serializing")?;
    json.push('\n');
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    write_atomic(path, json.as_bytes())
}

/// One remembered translation.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TmEntry {
    pub source: String,
    pub target: String,
    /// How many times the pair was learned or confirmed.
    #[serde(default)]
    pub uses: u32,
    /// When the entry was last learned (RFC 3339).
    #[serde(default)]
    pub last: String,
    /// Name of the mod the translation comes from.
    #[serde(default)]
    pub origin: String,
}

/// Translations accepted in earlier work, reused across mods for one
/// language pair.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TranslationMemory {
    pub source_lang: String,
    pub target_lang: String,
    #[serde(default)]
    pub entries: Vec<TmEntry>,
}

impl TranslationMemory {
    /// File of the memory of a language pair:
    /// `<config dir>/translation_memory/<src>-<tgt>.json`.
    pub fn path(source_lang: &str, target_lang: &str) -> Option<PathBuf> {
        translation_memory_dir().map(|d| {
            d.join(format!(
                "{}-{}.json",
                lang_file_slug(source_lang),
                lang_file_slug(target_lang)
            ))
        })
    }

    /// Memory of a language pair; empty when there is none yet or when the
    /// file cannot be read.
    pub fn load(source_lang: &str, target_lang: &str) -> Self {
        let loaded = Self::path(source_lang, target_lang).and_then(|p| Self::load_from(&p));
        loaded.unwrap_or_else(|| Self {
            source_lang: source_lang.to_string(),
            target_lang: target_lang.to_string(),
            entries: Vec::new(),
        })
    }

    /// Read a memory file (`None` when missing or invalid).
    pub fn load_from(path: &Path) -> Option<Self> {
        load_json(path)
    }

    /// Write the memory to its file ([`Self::path`]), atomically.
    pub fn save(&self) -> Result<()> {
        let path = Self::path(&self.source_lang, &self.target_lang)
            .context("could not determine the configuration directory")?;
        self.save_to(&path)
    }

    /// Write the memory to `path`, atomically.
    pub fn save_to(&self, path: &Path) -> Result<()> {
        save_json(path, self)
    }

    /// Remember the translations of `doc`: every unit with a target and the
    /// status `Translated` (automatic and fuzzy translations are not trusted).
    /// One entry is kept per source text; a later translation replaces an
    /// earlier one, in the document as in the memory.
    ///
    /// Returns the number of entries added or whose translation changed. An
    /// entry that is merely confirmed (same translation) counts for nothing,
    /// but its `uses` counter grows when the confirmation comes from another
    /// mod.
    pub fn learn(&mut self, doc: &TranslationDoc) -> usize {
        if self.source_lang.is_empty() {
            self.source_lang = doc.source_lang.clone();
        }
        if self.target_lang.is_empty() {
            self.target_lang = doc.target_lang.clone();
        }

        // Last translation of each source in the document, in document order.
        let mut order: Vec<&str> = Vec::new();
        let mut latest: HashMap<&str, &str> = HashMap::new();
        for u in &doc.units {
            if u.status == TStatus::Translated
                && !u.target.is_empty()
                && !u.source.is_empty()
                && latest.insert(u.source.as_str(), u.target.as_str()).is_none()
            {
                order.push(u.source.as_str());
            }
        }

        let mut index: HashMap<String, usize> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| (e.source.clone(), i))
            .collect();
        let now = now_rfc3339();
        let mut count = 0;
        for source in order {
            let target = latest[source];
            match index.get(source) {
                Some(&i) => {
                    let entry = &mut self.entries[i];
                    if entry.target != target {
                        entry.target = target.to_string();
                        count += 1;
                    } else if entry.origin == doc.mod_name {
                        // The same mod learned again: nothing new.
                        continue;
                    }
                    entry.uses = entry.uses.saturating_add(1);
                    entry.last = now.clone();
                    entry.origin = doc.mod_name.clone();
                }
                None => {
                    index.insert(source.to_string(), self.entries.len());
                    self.entries.push(TmEntry {
                        source: source.to_string(),
                        target: target.to_string(),
                        uses: 1,
                        last: now.clone(),
                        origin: doc.mod_name.clone(),
                    });
                    count += 1;
                }
            }
        }
        count
    }

    /// Entry to propose for `source`: the same text, else the same text
    /// ignoring surrounding blanks, else ignoring case as well.
    pub fn suggest(&self, source: &str) -> Option<&TmEntry> {
        let wanted = source.trim();
        if wanted.is_empty() {
            return None;
        }
        let usable = || self.entries.iter().filter(|e| !e.target.is_empty());
        let lower = wanted.to_lowercase();
        usable()
            .find(|e| e.source == source)
            .or_else(|| usable().find(|e| e.source.trim() == wanted))
            .or_else(|| usable().find(|e| e.source.trim().to_lowercase() == lower))
    }

    /// Fill the units of `doc` that have no target and are not locked with
    /// the translation remembered for exactly the same source text. They are
    /// marked [`TStatus::Auto`] (to be reviewed). Returns the number filled.
    pub fn apply(&self, doc: &mut TranslationDoc) -> usize {
        let exact: HashMap<&str, &str> = self
            .entries
            .iter()
            .filter(|e| !e.target.is_empty())
            .map(|e| (e.source.as_str(), e.target.as_str()))
            .collect();
        let mut count = 0;
        for u in &mut doc.units {
            if u.target.is_empty()
                && !u.locked
                && let Some(target) = exact.get(u.source.as_str())
            {
                u.target = (*target).to_string();
                u.status = TStatus::Auto;
                count += 1;
            }
        }
        count
    }
}

/// One glossary rule: how a term must be translated.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GlossaryTerm {
    /// Term in the source language.
    pub source: String,
    /// Expected translation (unused when `do_not_translate` is set).
    #[serde(default)]
    pub target: String,
    /// Match the term (and its translation) with their exact case.
    #[serde(default)]
    pub case_sensitive: bool,
    /// The term is a name to keep as it is (mod, character, file…).
    #[serde(default)]
    pub do_not_translate: bool,
}

/// Terminology of a language pair.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Glossary {
    pub source_lang: String,
    pub target_lang: String,
    #[serde(default)]
    pub terms: Vec<GlossaryTerm>,
}

impl Glossary {
    /// File of the glossary of a language pair:
    /// `<config dir>/translation_memory/glossary-<src>-<tgt>.json`.
    pub fn path(source_lang: &str, target_lang: &str) -> Option<PathBuf> {
        translation_memory_dir().map(|d| {
            d.join(format!(
                "glossary-{}-{}.json",
                lang_file_slug(source_lang),
                lang_file_slug(target_lang)
            ))
        })
    }

    /// Glossary of a language pair; empty when there is none yet or when the
    /// file cannot be read.
    pub fn load(source_lang: &str, target_lang: &str) -> Self {
        let loaded = Self::path(source_lang, target_lang).and_then(|p| Self::load_from(&p));
        loaded.unwrap_or_else(|| Self {
            source_lang: source_lang.to_string(),
            target_lang: target_lang.to_string(),
            terms: Vec::new(),
        })
    }

    /// Read a glossary file (`None` when missing or invalid).
    pub fn load_from(path: &Path) -> Option<Self> {
        load_json(path)
    }

    /// Write the glossary to its file ([`Self::path`]), atomically.
    pub fn save(&self) -> Result<()> {
        let path = Self::path(&self.source_lang, &self.target_lang)
            .context("could not determine the configuration directory")?;
        self.save_to(&path)
    }

    /// Write the glossary to `path`, atomically.
    pub fn save_to(&self, path: &Path) -> Result<()> {
        save_json(path, self)
    }
}

/// Does `text` contain `word` delimited by non-alphanumeric characters (or
/// the ends of the text)? Only the ends of `word` that are themselves
/// alphanumeric need a boundary, so a term such as `C++` or `(SE)` matches
/// wherever it is written.
fn contains_whole_word(text: &str, word: &str) -> bool {
    if word.is_empty() {
        return false;
    }
    let first_is_word = word.chars().next().is_some_and(char::is_alphanumeric);
    let last_is_word = word.chars().next_back().is_some_and(char::is_alphanumeric);
    text.match_indices(word).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + word.len()..].chars().next();
        (!first_is_word || !before.is_some_and(char::is_alphanumeric))
            && (!last_is_word || !after.is_some_and(char::is_alphanumeric))
    })
}

/// Terminology checks: a [`TIssue::GlossaryViolation`] for each translated
/// unit whose source contains a glossary term (as a whole word, with the case
/// sensitivity of the term) while its target does not contain the expected
/// translation — or the term itself when it must not be translated. The
/// expected text is searched anywhere in the target, since translations are
/// inflected. Units without a target and terms without an expected
/// translation are skipped. Never blocking.
pub fn glossary_issues(doc: &TranslationDoc, glossary: &Glossary) -> Vec<(String, TIssue)> {
    /// A term prepared for matching.
    struct Rule<'a> {
        term: &'a GlossaryTerm,
        source: String,
        expected: String,
    }
    let fold = |s: &str, case_sensitive: bool| {
        if case_sensitive {
            s.to_string()
        } else {
            s.to_lowercase()
        }
    };
    let rules: Vec<Rule> = glossary
        .terms
        .iter()
        .filter_map(|term| {
            let source = term.source.trim();
            let expected = if term.do_not_translate {
                source
            } else {
                term.target.trim()
            };
            (!source.is_empty() && !expected.is_empty()).then(|| Rule {
                term,
                source: fold(source, term.case_sensitive),
                expected: fold(expected, term.case_sensitive),
            })
        })
        .collect();
    if rules.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for u in doc.units.iter().filter(|u| !u.target.trim().is_empty()) {
        let (source_lower, target_lower) = (u.source.to_lowercase(), u.target.to_lowercase());
        for rule in &rules {
            let (source, target) = if rule.term.case_sensitive {
                (u.source.as_str(), u.target.as_str())
            } else {
                (source_lower.as_str(), target_lower.as_str())
            };
            if contains_whole_word(source, &rule.source) && !target.contains(&rule.expected) {
                out.push((
                    u.key.clone(),
                    TIssue::GlossaryViolation {
                        term: rule.term.source.clone(),
                    },
                ));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Updating a translation after the source FOMOD changed
// ---------------------------------------------------------------------------

/// What [`merge_update`] did.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UpdateReport {
    /// Strings that did not exist before.
    pub new: usize,
    /// Strings whose source text changed (now fuzzy).
    pub changed: usize,
    /// Strings found under another key (reordered FOMOD).
    pub moved: usize,
    /// Strings that no longer exist.
    pub removed: usize,
    /// Strings found unchanged at the same key.
    pub unchanged: usize,
}

/// Carry an existing translation over to a freshly extracted list of units.
///
/// For each new unit, in this order of preference:
///
/// 1. same key and same source → the old unit is kept as it is;
/// 2. same source under another key (the string moved) → the target is kept;
///    the status is preserved when the match is unambiguous (one candidate for
///    one unit), otherwise a translated unit is downgraded to `Auto`;
/// 3. same key but another source → the old target is kept and marked `Fuzzy`;
/// 4. nothing matches → `Untranslated` (pre-filled as `Auto` when the same
///    source text is already translated elsewhere in the document).
///
/// Old units that were not reused go to `obsolete` (when they carry a target),
/// from where rule 2 can bring them back later as `Auto`.
pub fn merge_update(old: &TranslationDoc, new_units: Vec<TUnit>) -> (TranslationDoc, UpdateReport) {
    let mut report = UpdateReport::default();
    let mut used = vec![false; old.units.len()];
    let mut used_obsolete = vec![false; old.obsolete.len()];
    let mut result: Vec<Option<TUnit>> = vec![None; new_units.len()];

    let carry = |n: &TUnit, o: &TUnit, status: TStatus| TUnit {
        key: n.key.clone(),
        field: n.field,
        source: n.source.clone(),
        target: o.target.clone(),
        status: if o.target.is_empty() {
            TStatus::Untranslated
        } else {
            status
        },
        locked: o.locked,
        context: n.context.clone(),
        note: o.note.clone(),
    };

    // 1. Same key, same source.
    let mut by_key: HashMap<&str, usize> = HashMap::new();
    for (i, o) in old.units.iter().enumerate() {
        by_key.entry(o.key.as_str()).or_insert(i);
    }
    for (ni, n) in new_units.iter().enumerate() {
        if let Some(&oi) = by_key.get(n.key.as_str())
            && !used[oi]
            && old.units[oi].source == n.source
        {
            used[oi] = true;
            let o = &old.units[oi];
            let status = if o.status == TStatus::Obsolete {
                TStatus::Auto
            } else {
                o.status
            };
            result[ni] = Some(carry(n, o, status));
            report.unchanged += 1;
        }
    }

    // 2. Same source elsewhere.
    let mut pending_by_source: HashMap<&str, usize> = HashMap::new();
    for (ni, n) in new_units.iter().enumerate() {
        if result[ni].is_none() {
            *pending_by_source.entry(n.source.as_str()).or_default() += 1;
        }
    }
    let mut free_by_source: HashMap<&str, usize> = HashMap::new();
    for (oi, o) in old.units.iter().enumerate() {
        if !used[oi] {
            *free_by_source.entry(o.source.as_str()).or_default() += 1;
        }
    }
    for (ni, n) in new_units.iter().enumerate() {
        if result[ni].is_some() {
            continue;
        }
        let candidates = |same_field: bool| {
            old.units
                .iter()
                .enumerate()
                .position(|(oi, o)| !used[oi] && o.source == n.source && (!same_field || o.field == n.field))
        };
        if let Some(oi) = candidates(true).or_else(|| candidates(false)) {
            used[oi] = true;
            let o = &old.units[oi];
            let unique = pending_by_source.get(n.source.as_str()) == Some(&1)
                && free_by_source.get(n.source.as_str()) == Some(&1);
            let status = match o.status {
                TStatus::Translated if !unique => TStatus::Auto,
                TStatus::Obsolete => TStatus::Auto,
                s => s,
            };
            result[ni] = Some(carry(n, o, status));
            report.moved += 1;
            continue;
        }
        // A translation retired earlier comes back for review.
        let revived = old
            .obsolete
            .iter()
            .enumerate()
            .position(|(oi, o)| !used_obsolete[oi] && o.source == n.source && !o.target.is_empty());
        if let Some(oi) = revived {
            used_obsolete[oi] = true;
            result[ni] = Some(carry(n, &old.obsolete[oi], TStatus::Auto));
            report.moved += 1;
        }
    }

    // 3. Same key, different source; 4. brand new.
    for (ni, n) in new_units.iter().enumerate() {
        if result[ni].is_some() {
            continue;
        }
        if let Some(&oi) = by_key.get(n.key.as_str())
            && !used[oi]
            && old.units[oi].field == n.field
        {
            used[oi] = true;
            result[ni] = Some(carry(n, &old.units[oi], TStatus::Fuzzy));
            report.changed += 1;
            continue;
        }
        // Translation memory: the same text is already translated elsewhere.
        let memory = old
            .units
            .iter()
            .find(|o| o.source == n.source && !o.target.is_empty() && o.status != TStatus::Fuzzy);
        let mut unit = n.clone();
        if let Some(o) = memory {
            unit.target = o.target.clone();
            unit.status = TStatus::Auto;
        } else {
            unit.target.clear();
            unit.status = TStatus::Untranslated;
        }
        result[ni] = Some(unit);
        report.new += 1;
    }

    // Retire what was not reused.
    let mut obsolete: Vec<TUnit> = old
        .obsolete
        .iter()
        .zip(&used_obsolete)
        .filter(|(_, used)| !**used)
        .map(|(o, _)| o.clone())
        .collect();
    for (oi, o) in old.units.iter().enumerate() {
        if used[oi] {
            continue;
        }
        report.removed += 1;
        if o.target.is_empty() {
            continue;
        }
        let duplicate = obsolete.iter().any(|x| x.source == o.source && x.target == o.target);
        if !duplicate {
            let mut retired = o.clone();
            retired.status = TStatus::Obsolete;
            obsolete.push(retired);
        }
    }

    let mut doc = old.clone();
    doc.units = result.into_iter().flatten().collect();
    doc.obsolete = obsolete;
    (doc, report)
}

// ---------------------------------------------------------------------------
// Projection on the project model
// ---------------------------------------------------------------------------

/// Indices found in a `config/step[i]/group[j]/plugin[k]/…` key.
pub(crate) fn key_indices(key: &str) -> Vec<usize> {
    key.split(['[', ']']).filter_map(|p| p.parse().ok()).collect()
}

/// Write `text` into the model field addressed by `key` / `field`. Returns
/// `false` when the key points at a step, group or option that does not
/// exist (nothing is changed then). Shared by the translation preview and
/// the project strings table.
pub(crate) fn apply_field(ximod: &mut crate::models::Ximod, key: &str, field: TField, text: String) -> bool {
    let idx = key_indices(key);
    match field {
        TField::InfoName | TField::ModuleName => ximod.name = text,
        TField::InfoAuthor => ximod.author = text,
        TField::InfoWebsite => ximod.url = text,
        TField::InfoDescription => ximod.description = text,
        TField::StepName => {
            let [s] = idx[..] else { return false };
            let Some(step) = ximod.steps.get_mut(s) else {
                return false;
            };
            step.name = text;
        }
        TField::GroupName => {
            let [s, g] = idx[..] else { return false };
            let Some(group) = ximod.steps.get_mut(s).and_then(|st| st.plugin_groups.get_mut(g)) else {
                return false;
            };
            group.name = text;
        }
        TField::PluginName | TField::PluginDescription => {
            let [s, g, p] = idx[..] else { return false };
            let Some(plugin) = ximod
                .steps
                .get_mut(s)
                .and_then(|st| st.plugin_groups.get_mut(g))
                .and_then(|gr| gr.plugins.get_mut(p))
            else {
                return false;
            };
            if field == TField::PluginName {
                plugin.name = text;
            } else {
                plugin.description = text;
            }
        }
    }
    true
}

/// Copy of `ximod` with its names and descriptions replaced by the effective
/// targets of `doc` (used for previews; exporting goes through the lossless
/// patcher instead). Units whose key does not exist in the model are ignored.
pub fn apply_to_model(ximod: &crate::models::Ximod, doc: &TranslationDoc) -> crate::models::Ximod {
    let mut out = ximod.clone();
    // `moduleName` is what the project loader keeps as the name when both
    // files are present, so it is applied last.
    let mut ordered: Vec<&TUnit> = doc.units.iter().collect();
    ordered.sort_by_key(|u| u.field == TField::ModuleName);

    for u in ordered {
        if u.target.is_empty() {
            continue;
        }
        apply_field(&mut out, &u.key, u.field, u.effective_target().to_string());
    }
    out
}

/// Keys of the units of `doc` that cannot be applied to the FOMOD described by
/// `current` (freshly extracted units): the string is gone or its source text
/// changed since the translation was last updated. Applying such a unit would
/// put a translation on the wrong string.
///
/// A unit whose current text already equals its target is not stale (the
/// translation was applied in place).
pub fn stale_keys(doc: &TranslationDoc, current: &[TUnit]) -> HashSet<String> {
    let now: HashMap<&str, &str> = current.iter().map(|u| (u.key.as_str(), u.source.as_str())).collect();
    doc.units
        .iter()
        .filter(|u| !u.target.is_empty())
        .filter(|u| match now.get(u.key.as_str()) {
            Some(text) => *text != u.source && *text != u.target,
            None => true,
        })
        .map(|u| u.key.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Plugin, PluginGroup, SelectionType, Step, Ximod};

    fn unit(key: &str, field: TField, source: &str, target: &str, status: TStatus) -> TUnit {
        TUnit {
            key: key.to_string(),
            field,
            source: source.to_string(),
            target: target.to_string(),
            status,
            locked: false,
            context: String::new(),
            note: String::new(),
        }
    }

    fn plugin_name(i: usize, source: &str, target: &str, status: TStatus) -> TUnit {
        unit(
            &format!("config/step[0]/group[0]/plugin[{i}]/name"),
            TField::PluginName,
            source,
            target,
            status,
        )
    }

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod_translate_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn field_properties() {
        assert_eq!(TField::InfoDescription.file(), TFile::Info);
        assert_eq!(TField::ModuleName.file(), TFile::Config);
        assert_eq!(TField::PluginDescription.file(), TFile::Config);
        assert!(TField::PluginDescription.is_multiline());
        assert!(TField::InfoDescription.is_multiline());
        assert!(!TField::PluginName.is_multiline());
        assert!(TField::InfoAuthor.locked_by_default());
        assert!(TField::InfoWebsite.locked_by_default());
        assert!(!TField::InfoName.locked_by_default());
    }

    #[test]
    fn tokens_are_extracted() {
        assert_eq!(
            tokens("Install MyMod.esp (v1.5.97) from https://example.org/a?b=1. Use {name} <b>now</b>\\n4K or 2048"),
            vec![
                "MyMod.esp",
                "1.5.97",
                "https://example.org/a?b=1",
                "{name}",
                "<b>",
                "</b>",
                "\\n",
                "2048"
            ]
        );
        assert_eq!(tokens("a\\r\\nb"), vec!["\\r\\n"]);
        assert_eq!(
            tokens("Textures.BA2, armor_01.NIF and x.dds."),
            vec!["Textures.BA2", "armor_01.NIF", "x.dds"]
        );
        // Single digits, comparisons and plain braces are not tokens.
        assert!(tokens("4K, 2 options, a < b, { not this }").is_empty());
        // Real line breaks are not tokens either.
        assert!(tokens("line 1\nline 2").is_empty());
        assert!(tokens("").is_empty());
    }

    #[test]
    fn validate_token_mismatch_and_blocking() {
        let u = unit(
            "info/Description",
            TField::InfoDescription,
            "Needs Foo.esp\\nand 2048 textures",
            "Requiert Foo.esp et des textures 4096",
            TStatus::Translated,
        );
        let issues = validate_unit(&u);
        let mismatch = issues
            .iter()
            .find(|i| matches!(i, TIssue::TokenMismatch { .. }))
            .expect("token mismatch");
        assert_eq!(
            mismatch,
            &TIssue::TokenMismatch {
                missing: vec!["\\n".to_string(), "2048".to_string()],
                extra: vec!["4096".to_string()],
            }
        );
        assert!(mismatch.is_blocking(), "a lost \\n escape is blocking");

        let soft = TIssue::TokenMismatch {
            missing: vec!["2048".into()],
            extra: vec![],
        };
        assert!(!soft.is_blocking());

        // Same tokens in another order are fine.
        let ok = unit(
            "k",
            TField::PluginDescription,
            "Foo.esp then 2048",
            "2048 puis Foo.esp",
            TStatus::Translated,
        );
        assert!(validate_unit(&ok).is_empty(), "{:?}", validate_unit(&ok));
    }

    #[test]
    fn validate_newline_control_chars_and_cdata() {
        let name = plugin_name(0, "High quality", "Haute\nqualité", TStatus::Translated);
        let issues = validate_unit(&name);
        assert!(issues.contains(&TIssue::NewlineInName));
        assert!(issues.iter().any(TIssue::is_blocking));

        // A line break is fine in a description.
        let desc = unit(
            "d",
            TField::PluginDescription,
            "High quality",
            "Haute\nqualité",
            TStatus::Translated,
        );
        assert!(validate_unit(&desc).is_empty());

        let ctrl = plugin_name(0, "High quality", "Haute\u{7}qualité", TStatus::Translated);
        assert!(validate_unit(&ctrl).contains(&TIssue::ControlChars));

        let cdata = unit(
            "d",
            TField::PluginDescription,
            "Some text",
            "Du ]]> texte",
            TStatus::Translated,
        );
        assert!(validate_unit(&cdata).contains(&TIssue::CdataTerminator));
        // `]]>` is harmless in an attribute (it is escaped).
        let attr = plugin_name(0, "Some text", "Du ]]> texte", TStatus::Translated);
        assert!(!validate_unit(&attr).contains(&TIssue::CdataTerminator));
    }

    #[test]
    fn validate_soft_checks() {
        // Untranslated: nothing to say.
        assert!(validate_unit(&plugin_name(0, "Yes", "", TStatus::Untranslated)).is_empty());
        // Claimed translated but empty.
        assert_eq!(
            validate_unit(&plugin_name(0, "Yes", "", TStatus::Translated)),
            vec![TIssue::EmptyTarget]
        );
        // …unless the unit is locked (kept as in the source).
        let mut locked = plugin_name(0, "Yes", "", TStatus::Translated);
        locked.locked = true;
        assert!(validate_unit(&locked).is_empty());

        assert_eq!(
            validate_unit(&plugin_name(0, "Yes", "  ", TStatus::Translated)),
            vec![TIssue::WhitespaceOnly]
        );
        assert_eq!(
            validate_unit(&plugin_name(0, "Yes", "Oui ", TStatus::Translated)),
            vec![TIssue::EdgeWhitespaceMismatch]
        );
        assert_eq!(
            validate_unit(&plugin_name(0, "Yes", "Yes", TStatus::Translated)),
            vec![TIssue::IdenticalToSource]
        );
        // No letters: identical is expected.
        assert!(validate_unit(&plugin_name(0, "4096", "4096", TStatus::Translated)).is_empty());

        let long = plugin_name(0, "A fairly long option name", "Oui", TStatus::Translated);
        assert!(matches!(validate_unit(&long)[..], [TIssue::LengthRatio { ratio }] if ratio < 0.2));
    }

    #[test]
    fn validate_doc_reports_inconsistent_duplicates() {
        let mut doc = TranslationDoc::new("eng", "fra");
        doc.units = vec![
            plugin_name(0, "None", "Aucun", TStatus::Translated),
            plugin_name(1, "None", "Aucune", TStatus::Translated),
            plugin_name(2, "None", "", TStatus::Untranslated),
            plugin_name(3, "Other", "Autre", TStatus::Translated),
        ];
        let issues = validate_doc(&doc);
        assert_eq!(
            issues,
            vec![
                (
                    doc.units[0].key.clone(),
                    TIssue::InconsistentDuplicate {
                        other_key: doc.units[1].key.clone()
                    }
                ),
                (
                    doc.units[1].key.clone(),
                    TIssue::InconsistentDuplicate {
                        other_key: doc.units[0].key.clone()
                    }
                ),
            ]
        );
        assert!(issues.iter().all(|(_, i)| !i.is_blocking()));

        // Consistent duplicates raise nothing.
        doc.units[1].target = "Aucun".into();
        assert!(validate_doc(&doc).is_empty());
    }

    #[test]
    fn issue_serialization_is_tagged() {
        let json = serde_json::to_string(&TIssue::InconsistentDuplicate { other_key: "k".into() }).unwrap();
        assert_eq!(json, r#"{"kind":"inconsistentDuplicate","other_key":"k"}"#);
        assert_eq!(
            serde_json::to_string(&TIssue::EmptyTarget).unwrap(),
            r#"{"kind":"emptyTarget"}"#
        );
        for issue in [
            TIssue::EmptyTarget,
            TIssue::TokenMismatch {
                missing: vec![],
                extra: vec![],
            },
            TIssue::LengthRatio { ratio: 4.0 },
            TIssue::CdataTerminator,
        ] {
            let v: serde_json::Value = serde_json::to_value(&issue).unwrap();
            assert_eq!(v["kind"], issue.kind());
        }
    }

    #[test]
    fn stats_and_target_map() {
        let mut doc = TranslationDoc::new("eng", "fra");
        let mut author = unit("info/Author", TField::InfoAuthor, "Bob", "", TStatus::Untranslated);
        author.locked = true;
        let mut site = unit(
            "info/Website",
            TField::InfoWebsite,
            "https://a",
            "https://a/fr",
            TStatus::Translated,
        );
        site.locked = true;
        doc.units = vec![
            unit("info/Name", TField::InfoName, "Mod", "Le mod", TStatus::Translated),
            author,
            site,
            plugin_name(0, "A", "A'", TStatus::Auto),
            plugin_name(1, "B", "B'", TStatus::Fuzzy),
            plugin_name(2, "C", "", TStatus::Untranslated),
        ];
        let s = doc.stats();
        assert_eq!(
            s,
            TStats {
                total: 6,
                translated: 3,
                auto: 1,
                fuzzy: 1,
                untranslated: 1,
                locked: 2
            }
        );
        assert_eq!(s.percent(), 50);
        assert_eq!(TranslationDoc::new("eng", "fra").stats().percent(), 100);

        let map = doc.target_map();
        assert_eq!(map.len(), 4);
        assert_eq!(map["info/Name"], "Le mod");
        assert_eq!(map["info/Website"], "https://a/fr");
        assert!(!map.contains_key("info/Author"), "locked without target is skipped");
        assert!(!map.contains_key("config/step[0]/group[0]/plugin[2]/name"));

        // Name suffix on export.
        assert_eq!(doc.export_targets(), map);
        doc.export.suffix_name = true;
        doc.units.push(unit(
            "config/moduleName",
            TField::ModuleName,
            "Mod",
            "",
            TStatus::Untranslated,
        ));
        let exported = doc.export_targets();
        assert_eq!(exported["info/Name"], "Le mod [FRA]");
        assert_eq!(exported["config/moduleName"], "Mod [FRA]");
    }

    #[test]
    fn unit_helpers() {
        let mut u = plugin_name(0, "Yes", "", TStatus::Untranslated);
        assert_eq!(u.effective_target(), "Yes");
        assert!(!u.is_done());
        u.locked = true;
        assert!(u.is_done());
        u.target = "Oui".into();
        assert_eq!(u.effective_target(), "Oui");
        assert!(!u.is_done(), "a locked unit with a pending target is not done");
        u.status = TStatus::Translated;
        assert!(u.is_done());
    }

    #[test]
    fn merge_update_moved_fuzzy_obsolete() {
        let mut old = TranslationDoc::new("eng", "fra");
        old.translator = "P.".into();
        old.units = vec![
            unit("info/Name", TField::InfoName, "Mod", "Le mod", TStatus::Translated),
            plugin_name(0, "Low", "Basse", TStatus::Translated),
            plugin_name(1, "Medium", "Moyenne", TStatus::Translated),
            plugin_name(2, "High", "Haute", TStatus::Translated),
            unit(
                "config/step[0]/group[0]/plugin[2]/description",
                TField::PluginDescription,
                "Best quality",
                "Meilleure qualité",
                TStatus::Translated,
            ),
        ];
        old.units[2].note = "check".into();

        // The author removed "Low", so everything shifts up, reworded the
        // description and added "Ultra".
        let mut fresh_medium = plugin_name(0, "Medium", "", TStatus::Untranslated);
        fresh_medium.context = "Step › Group › Medium".into();
        let new_units = vec![
            unit("info/Name", TField::InfoName, "Mod", "", TStatus::Untranslated),
            fresh_medium,
            plugin_name(1, "High", "", TStatus::Untranslated),
            unit(
                "config/step[0]/group[0]/plugin[1]/description",
                TField::PluginDescription,
                "Best quality, big download",
                "",
                TStatus::Untranslated,
            ),
            plugin_name(2, "Ultra", "", TStatus::Untranslated),
        ];

        let (doc, report) = merge_update(&old, new_units);
        assert_eq!(
            report,
            UpdateReport {
                new: 2,
                changed: 0,
                moved: 2,
                removed: 2,
                unchanged: 1
            }
        );
        assert_eq!(doc.translator, "P.");

        assert_eq!(doc.units.len(), 5);
        assert_eq!(
            (doc.units[0].target.as_str(), doc.units[0].status),
            ("Le mod", TStatus::Translated)
        );
        // Moved, unambiguous: translation and status follow the string.
        assert_eq!(doc.units[1].key, "config/step[0]/group[0]/plugin[0]/name");
        assert_eq!(
            (doc.units[1].target.as_str(), doc.units[1].status),
            ("Moyenne", TStatus::Translated)
        );
        assert_eq!(doc.units[1].note, "check");
        assert_eq!(
            doc.units[1].context, "Step › Group › Medium",
            "context comes from the new unit"
        );
        assert_eq!(
            (doc.units[2].target.as_str(), doc.units[2].status),
            ("Haute", TStatus::Translated)
        );
        // The description moved to another key *and* changed: it is new.
        assert_eq!(
            (doc.units[3].target.as_str(), doc.units[3].status),
            ("", TStatus::Untranslated)
        );
        // "Ultra" sits on the key that used to hold "High", already reused.
        assert_eq!(
            (doc.units[4].target.as_str(), doc.units[4].status),
            ("", TStatus::Untranslated)
        );

        // "Low" and the old description are retired with their translations.
        let retired: Vec<(&str, &str, TStatus)> = doc
            .obsolete
            .iter()
            .map(|u| (u.source.as_str(), u.target.as_str(), u.status))
            .collect();
        assert_eq!(
            retired,
            vec![
                ("Low", "Basse", TStatus::Obsolete),
                ("Best quality", "Meilleure qualité", TStatus::Obsolete)
            ]
        );
    }

    #[test]
    fn merge_update_fuzzy_ambiguous_and_revived() {
        let mut old = TranslationDoc::new("eng", "fra");
        old.units = vec![
            plugin_name(0, "Install the patch", "Installer le correctif", TStatus::Translated),
            plugin_name(1, "None", "Aucun", TStatus::Translated),
            plugin_name(2, "None", "Aucune", TStatus::Translated),
            plugin_name(3, "Untouched", "", TStatus::Untranslated),
        ];
        old.obsolete = vec![plugin_name(9, "Legacy", "Ancien", TStatus::Obsolete)];

        let new_units = vec![
            // Same key, reworded source → fuzzy with the old target.
            plugin_name(0, "Install the patch (recommended)", "", TStatus::Untranslated),
            // Two identical strings moved: ambiguous → Auto.
            plugin_name(4, "None", "", TStatus::Untranslated),
            plugin_name(5, "None", "", TStatus::Untranslated),
            // Same key and source, never translated.
            plugin_name(3, "Untouched", "", TStatus::Untranslated),
            // Back from the obsolete list.
            plugin_name(6, "Legacy", "", TStatus::Untranslated),
        ];
        let (doc, report) = merge_update(&old, new_units);
        assert_eq!(
            report,
            UpdateReport {
                new: 0,
                changed: 1,
                moved: 3,
                removed: 0,
                unchanged: 1
            }
        );

        assert_eq!(doc.units[0].source, "Install the patch (recommended)");
        assert_eq!(
            (doc.units[0].target.as_str(), doc.units[0].status),
            ("Installer le correctif", TStatus::Fuzzy)
        );
        assert_eq!(
            (doc.units[1].target.as_str(), doc.units[1].status),
            ("Aucun", TStatus::Auto)
        );
        assert_eq!(
            (doc.units[2].target.as_str(), doc.units[2].status),
            ("Aucune", TStatus::Auto)
        );
        assert_eq!(
            (doc.units[3].target.as_str(), doc.units[3].status),
            ("", TStatus::Untranslated)
        );
        assert_eq!(
            (doc.units[4].target.as_str(), doc.units[4].status),
            ("Ancien", TStatus::Auto)
        );
        assert!(doc.obsolete.is_empty());

        // Merging the result with itself changes nothing.
        let (again, report) = merge_update(&doc, doc.units.clone());
        assert_eq!(again.units, doc.units);
        assert_eq!(
            report,
            UpdateReport {
                unchanged: 5,
                ..UpdateReport::default()
            }
        );
    }

    #[test]
    fn merge_update_prefills_new_duplicates_from_memory() {
        let mut old = TranslationDoc::new("eng", "fra");
        old.units = vec![plugin_name(0, "Yes", "Oui", TStatus::Translated)];
        let (doc, report) = merge_update(
            &old,
            vec![
                plugin_name(0, "Yes", "", TStatus::Untranslated),
                plugin_name(1, "Yes", "", TStatus::Untranslated),
            ],
        );
        assert_eq!(
            report,
            UpdateReport {
                new: 1,
                unchanged: 1,
                ..UpdateReport::default()
            }
        );
        assert_eq!(
            (doc.units[0].target.as_str(), doc.units[0].status),
            ("Oui", TStatus::Translated)
        );
        assert_eq!(
            (doc.units[1].target.as_str(), doc.units[1].status),
            ("Oui", TStatus::Auto)
        );
    }

    #[test]
    fn sidecar_save_load_round_trip() {
        let root = scratch("sidecar");
        let path = TranslationDoc::sidecar_path(&root, "My: Mod/Name?", "fra");
        assert_eq!(
            path,
            root.join("fomod")
                .join("translations")
                .join("My_ Mod_Name_.fra.ximod-translation")
        );
        assert_eq!(
            TranslationDoc::sidecar_path(&root, "  ", "fra").file_name().unwrap(),
            "fomod.fra.ximod-translation"
        );

        let mut doc = TranslationDoc::new("eng", "fra");
        doc.translator = "Patrice".into();
        doc.mod_name = "My: Mod/Name?".into();
        doc.mod_version = "1.2".into();
        doc.source.config = Some(FileFingerprint {
            crc32: 0xDEAD_BEEF,
            bytes: 42,
            encoding: "utf-8-bom".into(),
        });
        doc.export.mode = ExportMode::InPlace;
        doc.updated = "2000-01-01T00:00:00Z".into();
        let mut u = unit(
            "info/Description",
            TField::InfoDescription,
            "Line 1\nLine \"2\"",
            "Ligne 1\nLigne « 2 »",
            TStatus::Fuzzy,
        );
        u.context = "A › B".into();
        u.note = "n".into();
        doc.units.push(u);
        doc.obsolete.push(plugin_name(0, "Old", "Vieux", TStatus::Obsolete));

        assert!(TranslationDoc::find_sidecars(&root).is_empty());
        doc.save(&path).unwrap();
        assert_eq!(TranslationDoc::find_sidecars(&root), vec![path.clone()]);
        // No temporary file is left behind.
        let leftovers: Vec<_> = std::fs::read_dir(path.parent().unwrap())
            .unwrap()
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().ends_with(".tmp"))
            .collect();
        assert!(leftovers.is_empty());

        let loaded = TranslationDoc::load(&path).unwrap();
        assert_ne!(loaded.updated, "2000-01-01T00:00:00Z", "save stamps the update time");
        let mut expected = doc.clone();
        expected.updated = loaded.updated.clone();
        assert_eq!(loaded, expected);

        // The JSON uses camelCase names and is pretty-printed.
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("\n  \"sourceLang\": \"eng\""));
        assert!(text.contains("\"forceExplicitOrder\": true"));
        assert!(text.contains("\"field\": \"infoDescription\""));
        assert!(text.contains("\"status\": \"fuzzy\""));
        assert!(text.contains("\"mode\": \"inPlace\""));

        // Optional members may be missing; a newer format is refused.
        let minimal = r#"{"format":1,"tool":"t","sourceLang":"eng","targetLang":"deu","translator":"",
            "modName":"","modVersion":"","source":{},"created":"","updated":"",
            "units":[{"key":"info/Name","field":"infoName","source":"Mod"}]}"#;
        let p2 = root.join("minimal.ximod-translation");
        std::fs::write(&p2, minimal).unwrap();
        let m = TranslationDoc::load(&p2).unwrap();
        assert_eq!(m.units[0].status, TStatus::Untranslated);
        assert_eq!(m.export, ExportPrefs::default());
        assert!(m.export.force_explicit_order && !m.export.suffix_name);
        assert_eq!(m.export.name_template, "{name}_{LANG}");

        std::fs::write(&p2, minimal.replace("\"format\":1", "\"format\":99")).unwrap();
        assert!(TranslationDoc::load(&p2).is_err());
        std::fs::write(&p2, "not json").unwrap();
        assert!(TranslationDoc::load(&p2).is_err());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn apply_to_model_sets_names_and_descriptions() {
        let mut x = Ximod::new("Mod");
        x.author = "Bob".into();
        x.description = "A mod".into();
        let mut step = Step::new("Options");
        let mut group = PluginGroup::new("Quality", SelectionType::SelectExactlyOne);
        let mut low = Plugin::new("Low");
        low.description = "Small".into();
        group.plugins.push(low);
        group.plugins.push(Plugin::new("High"));
        step.plugin_groups.push(group);
        x.steps.push(step);

        let mut doc = TranslationDoc::new("eng", "fra");
        doc.units = vec![
            unit(
                "config/moduleName",
                TField::ModuleName,
                "Mod",
                "Le mod",
                TStatus::Translated,
            ),
            unit("info/Name", TField::InfoName, "Mod", "Mod (info)", TStatus::Translated),
            unit("info/Author", TField::InfoAuthor, "Bob", "", TStatus::Untranslated),
            unit(
                "info/Description",
                TField::InfoDescription,
                "A mod",
                "Un mod",
                TStatus::Translated,
            ),
            unit(
                "config/step[0]/name",
                TField::StepName,
                "Options",
                "Choix",
                TStatus::Translated,
            ),
            unit(
                "config/step[0]/group[0]/name",
                TField::GroupName,
                "Quality",
                "Qualité",
                TStatus::Translated,
            ),
            plugin_name(0, "Low", "Basse", TStatus::Auto),
            unit(
                "config/step[0]/group[0]/plugin[0]/description",
                TField::PluginDescription,
                "Small",
                "Petit",
                TStatus::Translated,
            ),
            plugin_name(1, "High", "", TStatus::Untranslated),
            // Out of range and malformed keys are ignored.
            plugin_name(7, "Ghost", "Fantôme", TStatus::Translated),
            unit("config/step[x]/name", TField::StepName, "?", "!", TStatus::Translated),
        ];

        let out = apply_to_model(&x, &doc);
        assert_eq!(out.name, "Le mod", "moduleName wins, as in the loader");
        assert_eq!(out.author, "Bob");
        assert_eq!(out.description, "Un mod");
        assert_eq!(out.steps[0].name, "Choix");
        let g = &out.steps[0].plugin_groups[0];
        assert_eq!(g.name, "Qualité");
        assert_eq!(g.plugins[0].name, "Basse");
        assert_eq!(g.plugins[0].description, "Petit");
        assert_eq!(g.plugins[1].name, "High", "untranslated units keep the source");
        // The input model is untouched.
        assert_eq!(x.steps[0].name, "Options");
    }

    #[test]
    fn stale_keys_detects_changed_sources() {
        let mut doc = TranslationDoc::new("eng", "fra");
        doc.units = vec![
            plugin_name(0, "Low", "Basse", TStatus::Translated),
            plugin_name(1, "High", "Haute", TStatus::Translated),
            plugin_name(2, "Gone", "Parti", TStatus::Translated),
            plugin_name(3, "Applied", "Appliqué", TStatus::Translated),
            plugin_name(4, "Changed but untranslated", "", TStatus::Untranslated),
        ];
        let current = vec![
            plugin_name(0, "Low", "", TStatus::Untranslated),
            plugin_name(1, "Medium", "", TStatus::Untranslated),
            plugin_name(3, "Appliqué", "", TStatus::Untranslated),
            plugin_name(4, "Something else", "", TStatus::Untranslated),
        ];
        let mut stale: Vec<String> = stale_keys(&doc, &current).into_iter().collect();
        stale.sort();
        assert_eq!(stale, vec![doc.units[1].key.clone(), doc.units[2].key.clone()]);
    }

    /// Unique-texts mode: the representative's translation is copied to every
    /// identical text, translated or not, except locked ones.
    #[test]
    fn sync_identical_overwrites_every_identical_text() {
        let mut doc = TranslationDoc::new("eng", "fra");
        let mut locked = plugin_name(3, "None", "Verrouillé", TStatus::Translated);
        locked.locked = true;
        doc.units = vec![
            plugin_name(0, "None", "Aucun", TStatus::Translated),
            plugin_name(1, "None", "", TStatus::Untranslated),
            plugin_name(2, "None", "Rien", TStatus::Translated),
            locked,
            plugin_name(4, "Other", "", TStatus::Untranslated),
        ];
        let groups = doc.identical_groups();
        assert_eq!(groups.get(&0), Some(&4), "four units share the text None");
        assert_eq!(groups.get(&4), Some(&1));
        assert!(!groups.contains_key(&1), "only the first unit represents a group");
        let key = doc.units[0].key.clone();
        assert_eq!(doc.sync_identical(&key), 2);
        assert_eq!(doc.units[1].target, "Aucun");
        assert_eq!(doc.units[2].target, "Aucun", "a different translation is aligned too");
        assert_eq!(doc.units[2].status, TStatus::Translated);
        assert_eq!(doc.units[3].target, "Verrouillé", "locked units are left alone");
        assert_eq!(doc.units[4].target, "");
        assert_eq!(doc.sync_identical(&key), 0, "nothing left to update");
    }

    #[test]
    fn propagate_fills_identical_sources() {
        let mut doc = TranslationDoc::new("eng", "fra");
        let mut locked = plugin_name(3, "None", "", TStatus::Untranslated);
        locked.locked = true;
        doc.units = vec![
            plugin_name(0, "None", "Aucun", TStatus::Translated),
            plugin_name(1, "None", "", TStatus::Untranslated),
            plugin_name(2, "None", "Rien", TStatus::Translated),
            locked,
            plugin_name(4, "Other", "", TStatus::Untranslated),
            plugin_name(5, "None", "", TStatus::Fuzzy),
        ];
        let key = doc.units[0].key.clone();
        assert_eq!(doc.propagate(&key), 2);
        assert_eq!(
            (doc.units[1].target.as_str(), doc.units[1].status),
            ("Aucun", TStatus::Auto)
        );
        assert_eq!(
            (doc.units[5].target.as_str(), doc.units[5].status),
            ("Aucun", TStatus::Auto)
        );
        assert_eq!(doc.units[2].target, "Rien", "an existing translation is kept");
        assert_eq!(doc.units[3].target, "", "locked units are left alone");
        assert_eq!(doc.units[4].target, "");
        assert_eq!(doc.units[0].status, TStatus::Translated, "the origin is untouched");

        // Nothing left to fill; unknown keys and empty targets do nothing.
        assert_eq!(doc.propagate(&key), 0);
        assert_eq!(doc.propagate("no/such/key"), 0);
        let empty = doc.units[4].key.clone();
        assert_eq!(doc.propagate(&empty), 0);
    }

    #[test]
    fn csv_round_trip_with_quotes_newlines_and_commas() {
        let mut doc = TranslationDoc::new("eng", "fra");
        let mut desc = unit(
            "info/Description",
            TField::InfoDescription,
            "Line 1, with \"quotes\"\r\nLine 2; done",
            "Ligne 1, avec « guillemets »\nLigne 2",
            TStatus::Fuzzy,
        );
        desc.context = "Mod, info".into();
        desc.note = "say \"hi\"".into();
        let mut author = unit("info/Author", TField::InfoAuthor, "Bob", "", TStatus::Untranslated);
        author.locked = true;
        doc.units = vec![
            desc,
            author,
            plugin_name(0, " padded ", "", TStatus::Untranslated),
            plugin_name(1, "High", "Haute", TStatus::Translated),
        ];

        let csv = doc.to_csv();
        assert!(csv.starts_with("key,field,context,source,target,status,locked,note\n"));
        assert!(!csv.contains("\r\n\""), "records end with a bare \\n");
        assert!(csv.contains(
            "info/Description,infoDescription,\"Mod, info\",\"Line 1, with \"\"quotes\"\"\r\nLine 2; done\","
        ));
        assert!(csv.contains("info/Author,infoAuthor,,Bob,,untranslated,true,\n"));
        assert!(csv.contains(",\" padded \",,untranslated,false,\n"));
        assert!(csv.ends_with(",High,Haute,translated,false,\n"));

        // The parser gives back exactly what was written.
        let records = parse_csv(&csv, ',').unwrap();
        assert_eq!(records.len(), 5);
        assert!(records.iter().all(|r| r.len() == 8));
        assert_eq!(records[1][3], doc.units[0].source);
        assert_eq!(records[1][4], doc.units[0].target);
        assert_eq!(records[1][7], "say \"hi\"");
        assert_eq!(records[3][3], " padded ");

        // Importing an unchanged export changes nothing.
        let mut same = doc.clone();
        assert_eq!(same.import_csv(&csv).unwrap(), 0);
        assert_eq!(same, doc);

        // Edited in a spreadsheet: other column order, CRLF records, a BOM,
        // an unknown key, an emptied target and a new multi-line target.
        let edited = "\u{FEFF}note,locked,target,key,extra\r\n\
            \"new, \"\"note\"\"\",false,\"Ligne A\nLigne B, fin\",info/Description,x\r\n\
            ,FALSE,Robert,info/Author,\r\n\
            ,maybe,,config/step[0]/group[0]/plugin[1]/name,\r\n\
            ,true,Fantôme,no/such/key,\r\n\
            \r\n\
            ,false,Rembourré,config/step[0]/group[0]/plugin[0]/name";
        let mut imported = doc.clone();
        assert_eq!(imported.import_csv(edited).unwrap(), 4);
        let u = &imported.units[0];
        assert_eq!(u.target, "Ligne A\nLigne B, fin");
        assert_eq!((u.status, u.note.as_str()), (TStatus::Translated, "new, \"note\""));
        let u = &imported.units[1];
        assert_eq!(
            (u.target.as_str(), u.status, u.locked),
            ("Robert", TStatus::Translated, false)
        );
        let u = &imported.units[3];
        assert_eq!(
            (u.target.as_str(), u.status, u.locked),
            ("", TStatus::Untranslated, false)
        );
        assert_eq!(imported.units[2].target, "Rembourré", "last record without a line end");
        // Source, key and context never change.
        assert_eq!(imported.units[0].source, doc.units[0].source);
        assert_eq!(imported.units[0].context, "Mod, info");

        // Semicolon-separated files (spreadsheets in many locales) are read too.
        let mut semi = doc.clone();
        let text = "key;target;note\nconfig/step[0]/group[0]/plugin[0]/name;\"a;b\";n,1\n";
        assert_eq!(semi.import_csv(text).unwrap(), 1);
        assert_eq!(
            (semi.units[2].target.as_str(), semi.units[2].note.as_str()),
            ("a;b", "n,1")
        );

        // Malformed input is an error, not a panic.
        assert!(doc.clone().import_csv("").is_err());
        assert!(doc.clone().import_csv("source,target\na,b\n").is_err(), "no key column");
        assert!(
            doc.clone()
                .import_csv("key,target\ninfo/Author,\"never closed\n")
                .is_err()
        );
        // A short row only updates the columns it has.
        let mut short = doc.clone();
        assert_eq!(short.import_csv("key,target,note\ninfo/Author\n").unwrap(), 0);
    }

    #[test]
    fn translation_memory_learn_suggest_apply() {
        let mut doc = TranslationDoc::new("eng", "fra");
        doc.mod_name = "Armory".into();
        doc.units = vec![
            plugin_name(0, "None", "Aucun", TStatus::Translated),
            plugin_name(1, "None", "Aucune", TStatus::Translated),
            plugin_name(2, "High quality", "Haute qualité", TStatus::Translated),
            plugin_name(3, "Guess", "Devine", TStatus::Auto),
            plugin_name(4, "Old", "Vieux", TStatus::Fuzzy),
            plugin_name(5, "Empty", "", TStatus::Translated),
        ];

        let mut tm = TranslationMemory::default();
        assert_eq!(tm.learn(&doc), 2, "only accepted translations, one entry per source");
        assert_eq!((tm.source_lang.as_str(), tm.target_lang.as_str()), ("eng", "fra"));
        assert_eq!(tm.entries.len(), 2);
        let none = &tm.entries[0];
        assert_eq!(
            (none.source.as_str(), none.target.as_str()),
            ("None", "Aucune"),
            "last write wins"
        );
        assert_eq!((none.uses, none.origin.as_str()), (1, "Armory"));
        assert!(none.last.ends_with('Z') && none.last.contains('T'), "{}", none.last);

        // Learning the same document again changes nothing.
        let before = tm.entries.clone();
        assert_eq!(tm.learn(&doc), 0);
        assert_eq!(tm.entries, before);

        // Another mod confirms one pair and changes another.
        let mut other = TranslationDoc::new("eng", "fra");
        other.mod_name = "Aurelia".into();
        other.units = vec![
            plugin_name(0, "None", "Aucune", TStatus::Translated),
            plugin_name(1, "High quality", "Qualité élevée", TStatus::Translated),
            plugin_name(2, "Low", "Basse", TStatus::Translated),
        ];
        assert_eq!(
            tm.learn(&other),
            2,
            "one changed, one added; the confirmation is not counted"
        );
        assert_eq!(tm.entries.len(), 3);
        assert_eq!((tm.entries[0].uses, tm.entries[0].origin.as_str()), (2, "Aurelia"));
        assert_eq!(
            (tm.entries[1].target.as_str(), tm.entries[1].uses),
            ("Qualité élevée", 2)
        );

        // Suggestions: exact, trimmed, then case-insensitive.
        assert_eq!(tm.suggest("None").map(|e| e.target.as_str()), Some("Aucune"));
        assert_eq!(tm.suggest("  None ").map(|e| e.target.as_str()), Some("Aucune"));
        assert_eq!(
            tm.suggest("HIGH QUALITY").map(|e| e.target.as_str()),
            Some("Qualité élevée")
        );
        assert_eq!(tm.suggest("high"), None);
        assert_eq!(tm.suggest("   "), None);
        tm.entries.push(TmEntry {
            source: "none".into(),
            target: "aucun".into(),
            ..TmEntry::default()
        });
        assert_eq!(
            tm.suggest("none").map(|e| e.target.as_str()),
            Some("aucun"),
            "exact case first"
        );
        assert_eq!(tm.suggest("NONE").map(|e| e.target.as_str()), Some("Aucune"));

        // Applying: exact sources only, never over a target or a locked unit.
        let mut fresh = TranslationDoc::new("eng", "fra");
        let mut locked = plugin_name(3, "Low", "", TStatus::Untranslated);
        locked.locked = true;
        fresh.units = vec![
            plugin_name(0, "None", "", TStatus::Untranslated),
            plugin_name(1, "NONE", "", TStatus::Untranslated),
            plugin_name(2, "High quality", "Déjà fait", TStatus::Translated),
            locked,
            plugin_name(4, "Low", "", TStatus::Untranslated),
        ];
        assert_eq!(tm.apply(&mut fresh), 2);
        assert_eq!(
            (fresh.units[0].target.as_str(), fresh.units[0].status),
            ("Aucune", TStatus::Auto)
        );
        assert_eq!(fresh.units[1].target, "", "case differs: suggested, not applied");
        assert_eq!(fresh.units[2].target, "Déjà fait");
        assert_eq!(fresh.units[3].target, "");
        assert_eq!(
            (fresh.units[4].target.as_str(), fresh.units[4].status),
            ("Basse", TStatus::Auto)
        );
        assert_eq!(tm.apply(&mut fresh), 0);
        // What was filled automatically is not learned back.
        assert_eq!(TranslationMemory::default().learn(&fresh), 1);
    }

    #[test]
    fn translation_memory_and_glossary_files() {
        let dir = scratch("tm_files");

        // Default locations: one folder, one file per language pair.
        if let (Some(tm), Some(gl)) = (TranslationMemory::path("ENG", " fr-A "), Glossary::path("eng", "fra")) {
            assert_eq!(tm.file_name().unwrap(), "eng-fra.json");
            assert_eq!(gl.file_name().unwrap(), "glossary-eng-fra.json");
            assert_eq!(tm.parent(), gl.parent());
            assert_eq!(tm.parent().unwrap().file_name().unwrap(), "translation_memory");
        }

        let tm_path = dir.join("sub").join("eng-fra.json");
        assert!(TranslationMemory::load_from(&tm_path).is_none());
        let tm = TranslationMemory {
            source_lang: "eng".into(),
            target_lang: "fra".into(),
            entries: vec![TmEntry {
                source: "Yes".into(),
                target: "Oui".into(),
                uses: 3,
                last: "2026-01-01T00:00:00Z".into(),
                origin: "Mod".into(),
            }],
        };
        tm.save_to(&tm_path).unwrap();
        let text = std::fs::read_to_string(&tm_path).unwrap();
        assert!(text.contains("\"sourceLang\": \"eng\"") && text.contains("\"uses\": 3"));
        let back = TranslationMemory::load_from(&tm_path).unwrap();
        assert_eq!(back.entries, tm.entries);
        assert_eq!(back.target_lang, "fra");

        let gl_path = dir.join("glossary-eng-fra.json");
        let glossary = Glossary {
            source_lang: "eng".into(),
            target_lang: "fra".into(),
            terms: vec![GlossaryTerm {
                source: "Skyrim".into(),
                do_not_translate: true,
                ..GlossaryTerm::default()
            }],
        };
        glossary.save_to(&gl_path).unwrap();
        let text = std::fs::read_to_string(&gl_path).unwrap();
        assert!(text.contains("\"doNotTranslate\": true") && text.contains("\"caseSensitive\": false"));
        assert_eq!(Glossary::load_from(&gl_path).unwrap().terms, glossary.terms);
        // Optional members may be missing; garbage reads as "no file".
        std::fs::write(
            &gl_path,
            r#"{"sourceLang":"eng","targetLang":"fra","terms":[{"source":"armor","target":"armure"}]}"#,
        )
        .unwrap();
        let minimal = Glossary::load_from(&gl_path).unwrap();
        assert!(!minimal.terms[0].case_sensitive && !minimal.terms[0].do_not_translate);
        std::fs::write(&gl_path, "not json").unwrap();
        assert!(Glossary::load_from(&gl_path).is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn glossary_issues_whole_word_and_do_not_translate() {
        let term = |source: &str, target: &str, case_sensitive: bool, do_not_translate: bool| GlossaryTerm {
            source: source.into(),
            target: target.into(),
            case_sensitive,
            do_not_translate,
        };
        let glossary = Glossary {
            source_lang: "eng".into(),
            target_lang: "fra".into(),
            terms: vec![
                term("armor", "armure", false, false),
                term("Skyrim", "", false, true),
                term("ENB", "ENB", true, false),
                term("", "ignored", false, false),
                term("sword", "", false, false),
            ],
        };

        let mut doc = TranslationDoc::new("eng", "fra");
        doc.units = vec![
            // 0: term present, expected translation missing.
            plugin_name(0, "Heavy Armor set", "Ensemble de cuirasse lourde", TStatus::Translated),
            // 1: fine (case ignored, inflected form accepted).
            plugin_name(1, "Heavy armor", "Armures lourdes", TStatus::Translated),
            // 2: "armory" is another word.
            plugin_name(2, "The armory", "L'arsenal", TStatus::Translated),
            // 3: do-not-translate name was translated.
            plugin_name(3, "For Skyrim only", "Pour Bordeciel uniquement", TStatus::Translated),
            // 4: do-not-translate name kept.
            plugin_name(4, "For Skyrim only", "Pour skyrim uniquement", TStatus::Translated),
            // 5: no target yet: nothing to check.
            plugin_name(5, "Armor", "", TStatus::Untranslated),
            // 6: case-sensitive term: "enb" in the target is not "ENB".
            plugin_name(6, "ENB preset", "Préréglage enb", TStatus::Auto),
            // 7: case-sensitive term absent from the source ("enb" ≠ "ENB").
            plugin_name(7, "enb preset", "Préréglage", TStatus::Translated),
            // 8: two terms violated at once; punctuation delimits words.
            plugin_name(8, "Skyrim-armor.", "Cuirasse de Bordeciel", TStatus::Translated),
            // 9: a term without translation ("sword") is not checked.
            plugin_name(9, "Sword", "Épée", TStatus::Translated),
        ];

        let issues = glossary_issues(&doc, &glossary);
        let got: Vec<(usize, &str)> = issues
            .iter()
            .map(|(key, issue)| {
                let i = doc.units.iter().position(|u| &u.key == key).unwrap();
                match issue {
                    TIssue::GlossaryViolation { term } => (i, term.as_str()),
                    other => panic!("unexpected issue {other:?}"),
                }
            })
            .collect();
        assert_eq!(
            got,
            vec![(0, "armor"), (3, "Skyrim"), (6, "ENB"), (8, "armor"), (8, "Skyrim")]
        );
        assert!(
            issues
                .iter()
                .all(|(_, i)| !i.is_blocking() && i.kind() == "glossaryViolation")
        );
        assert_eq!(
            serde_json::to_string(&issues[0].1).unwrap(),
            r#"{"kind":"glossaryViolation","term":"armor"}"#
        );
        assert!(glossary_issues(&doc, &Glossary::default()).is_empty());

        assert!(contains_whole_word("a C++ mod", "C++"));
        assert!(contains_whole_word("armor", "armor"));
        assert!(!contains_whole_word("armored", "armor"));
        assert!(!contains_whole_word("préarmor", "armor"));
        assert!(!contains_whole_word("anything", ""));
    }
}
