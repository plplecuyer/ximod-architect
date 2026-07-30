//! In-app translation editor.
//!
//! Lets the user pick a source language (shown as reference) and a target
//! language, then edit every key of `main.ftl` in a three-column table:
//! key (read-only) | source label (read-only) | translation input.
//!
//! Protected tokens (Fluent placeables like `{ $num }` and macros like
//! `$MODNAME$`) are never shown as such in the source column:
//! - when they sit at the start/end of the text they are hidden entirely and
//!   re-attached automatically on save (e.g. `$MODNAME$ – Mod name` shows only
//!   "Mod name");
//! - when they sit in the middle of the text they are shown as compact markers
//!   `[[1]]`, `[[2]]`… that the translator keeps and repositions, and which are
//!   turned back into the real tokens on save.

use crate::ui::main_window::XimodApp;
use eframe::egui;
use std::collections::HashMap;

/// One editable row of the translation table.
pub struct TransEntry {
    pub key: String,
    /// Full source value (used as fallback when the translation is left empty).
    pub source_value: String,
    /// Text shown in the source column (tokens hidden or turned into markers).
    pub display: String,
    /// The translation typed by the user.
    pub input: String,
    /// Affix mode: text before the human part (tokens + separators).
    pub prefix: String,
    /// Affix mode: text after the human part.
    pub suffix: String,
    /// True when tokens are embedded mid-text and shown as `[[n]]` markers.
    pub marker_mode: bool,
    /// Ordered protected tokens (for marker reconstruction).
    pub tokens: Vec<String>,
    pub has_tokens: bool,
}

/// Length in bytes of the UTF-8 character starting with `b`.
fn utf8_len(b: u8) -> usize {
    if b < 0x80 { 1 } else if b < 0xE0 { 2 } else if b < 0xF0 { 3 } else { 4 }
}

/// A separator character that may sit between a token and human text.
fn is_sep(c: char) -> bool {
    c.is_whitespace() || matches!(c, '-' | '\u{2013}' | '\u{2014}' | ':' | '\u{00B7}' | '|' | '/')
}

/// Find protected tokens (byte ranges) in `s`: Fluent placeables `{ … }` and
/// macros `$IDENT$`.
fn find_tokens(s: &str) -> Vec<(usize, usize)> {
    let bytes = s.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let c = bytes[i];
        if c == b'{' {
            if let Some(rel) = s[i..].find('}') {
                let end = i + rel + 1;
                tokens.push((i, end));
                i = end;
                continue;
            }
        } else if c == b'$' {
            let rest = &s[i + 1..];
            let rb = rest.as_bytes();
            let mut j = 0;
            while j < rest.len() && (rb[j].is_ascii_alphanumeric() || rb[j] == b'_') {
                j += 1;
            }
            if j > 0 && j < rest.len() && rb[j] == b'$' {
                let end = i + 1 + j + 1;
                tokens.push((i, end));
                i = end;
                continue;
            }
        }
        i += utf8_len(c);
    }
    tokens
}

struct Analyzed {
    display: String,
    prefix: String,
    suffix: String,
    marker_mode: bool,
    tokens: Vec<String>,
    has_tokens: bool,
}

/// Analyze a value into a display form + reconstruction data.
fn analyze(value: &str) -> Analyzed {
    let toks = find_tokens(value);
    if toks.is_empty() {
        return Analyzed {
            display: value.to_string(),
            prefix: String::new(),
            suffix: String::new(),
            marker_mode: false,
            tokens: Vec::new(),
            has_tokens: false,
        };
    }

    // Human runs = the parts of the string that are not tokens.
    let mut human_runs: Vec<(usize, usize)> = Vec::new();
    let mut cursor = 0;
    for &(s, e) in &toks {
        if s > cursor {
            human_runs.push((cursor, s));
        }
        cursor = e;
    }
    if cursor < value.len() {
        human_runs.push((cursor, value.len()));
    }
    let non_empty: Vec<(usize, usize)> = human_runs
        .iter()
        .cloned()
        .filter(|&(s, e)| !value[s..e].trim().is_empty())
        .collect();

    let token_strings: Vec<String> = toks.iter().map(|&(s, e)| value[s..e].to_string()).collect();

    if non_empty.len() <= 1 {
        // Affix mode: tokens all on one side (or both sides of a single run).
        if let Some(&(hs, he)) = non_empty.first() {
            let run = &value[hs..he];
            // Strip whitespace, and — on the side touching a token — also strip
            // separator punctuation (– - : · | /), so "$MODNAME$ – Mod name"
            // yields "Mod name" and re-attaches "$MODNAME$ – " on save.
            let has_before = toks.iter().any(|&(_, e)| e <= hs);
            let has_after = toks.iter().any(|&(s, _)| s >= he);

            let after_start = if has_before {
                run.trim_start_matches(is_sep)
            } else {
                run.trim_start()
            };
            let start_rel = run.len() - after_start.len();

            let inner = &run[start_rel..];
            let trimmed_str = if has_after {
                inner.trim_end_matches(is_sep)
            } else {
                inner.trim_end()
            };
            let human_start = hs + start_rel;
            let human_end = human_start + trimmed_str.len();

            Analyzed {
                display: value[human_start..human_end].to_string(),
                prefix: value[..human_start].to_string(),
                suffix: value[human_end..].to_string(),
                marker_mode: false,
                tokens: token_strings,
                has_tokens: true,
            }
        } else {
            // Only tokens, no human text.
            Analyzed {
                display: String::new(),
                prefix: value.to_string(),
                suffix: String::new(),
                marker_mode: false,
                tokens: token_strings,
                has_tokens: true,
            }
        }
    } else {
        // Marker mode: tokens embedded between human runs.
        let mut display = String::new();
        let mut cursor = 0;
        for (idx, &(s, e)) in toks.iter().enumerate() {
            display.push_str(&value[cursor..s]);
            display.push_str(&format!("[[{}]]", idx + 1));
            cursor = e;
        }
        display.push_str(&value[cursor..]);
        Analyzed {
            display,
            prefix: String::new(),
            suffix: String::new(),
            marker_mode: true,
            tokens: token_strings,
            has_tokens: true,
        }
    }
}

/// Rebuild the full value from an entry's input, re-attaching protected tokens.
fn reconstruct(entry: &TransEntry) -> String {
    let input = entry.input.trim();
    if input.is_empty() {
        // Untranslated → keep the source value so the file stays complete/valid.
        return entry.source_value.clone();
    }
    if !entry.has_tokens {
        return input.to_string();
    }
    if entry.marker_mode {
        let mut out = input.to_string();
        for (i, tok) in entry.tokens.iter().enumerate() {
            out = out.replace(&format!("[[{}]]", i + 1), tok);
        }
        out
    } else {
        format!("{}{}{}", entry.prefix, input, entry.suffix)
    }
}

/// Keys that must never be translated (proper names / brand). They are hidden
/// from the editor and left untouched on save.
const NON_TRANSLATABLE: &[&str] = &["app-title"];

/// Percent-encode a string for use in a `mailto:` URL.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// Header comment introducing the metadata block of a translated file.
const META_HEADER: &str = "# XIMOD Architect - translation metadata";

/// Metadata keys written at the top of a translated `main.ftl`, as Fluent
/// comments so the file stays perfectly valid:
///
/// ```text
/// # XIMOD Architect - translation metadata
/// # @country = FRA
/// # @language = fra
/// # @font = Noto_Sans/static/NotoSans-Regular.ttf
/// # @endonym = Republique francaise
/// ```
const META_KEYS: [&str; 6] = [
    "country",
    "language",
    "font",
    "endonym",
    "langname",
    "author",
];

/// True when a line belongs to the metadata block (header or `# @key = value`).
///
/// The value may be empty or even absent (`# @endonym =` or `# @endonym`): the
/// key is everything before the first `=`. This matters when the "displayed
/// language" source file carries a metadata block with blank endonym/langname —
/// those lines must still be recognised so they are not copied into the saved
/// translation.
fn is_metadata_line(line: &str) -> bool {
    let t = line.trim();
    if t == META_HEADER {
        return true;
    }
    let Some(rest) = t.strip_prefix("# @") else {
        return false;
    };
    let key = rest.split('=').next().unwrap_or("").trim();
    META_KEYS.contains(&key)
}

/// Extract the metadata block of a translated file.
fn parse_metadata(content: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("# @") {
            if let Some(eq) = rest.find(" = ") {
                let key = &rest[..eq];
                if META_KEYS.contains(&key) {
                    out.insert(key.to_string(), rest[eq + 3..].trim().to_string());
                }
            }
        } else if !t.is_empty() && !t.starts_with('#') {
            break; // metadata only ever sits at the very top
        }
    }
    out
}

/// Read the country endonym recorded in a translation's `countryEndonyms.tsv`
/// for a given (language folder, country). Returns `None` when the side file or
/// the matching row is absent. Format: `<ISO 3166-1 alpha-3>\t<ISO 639-3>\t<endonym>`.
fn read_tsv_country_endonym(
    locales_dir: &std::path::Path,
    lang: &str,
    country: &str,
) -> Option<String> {
    if lang.is_empty() || country.is_empty() {
        return None;
    }
    let path = locales_dir.join(lang).join("countryEndonyms.tsv");
    let text = std::fs::read_to_string(path).ok()?;
    for line in text.lines() {
        let mut it = line.split('\t');
        let c = it.next().unwrap_or("");
        let _l = it.next().unwrap_or("");
        let endo = it.next().unwrap_or("").trim();
        if c.eq_ignore_ascii_case(country) && !endo.is_empty() {
            return Some(endo.to_string());
        }
    }
    None
}

/// Keyboard navigation for the translation table. Consumes the keys so widgets
/// don't also react. Returns:
///   * whether the row cursor moved (arrows / Page / Home / End),
///   * the scroll alignment for that move (`None` = minimal, `TOP`/`BOTTOM` for
///     Page/Home/End),
///   * the row whose input field should be focused this frame (Enter, or
///     Tab / Shift+Tab), if any.
///
/// Row navigation is active only when no field is being edited; Tab / Shift+Tab
/// (next / previous field) work in both modes and focus the target input.
fn handle_trans_keys(
    ctx: &egui::Context,
    cursor: &mut usize,
    n: usize,
    page: usize,
    editing: bool,
) -> (bool, Option<egui::Align>, Option<usize>) {
    if n == 0 {
        *cursor = 0;
        return (false, None, None);
    }
    if *cursor >= n {
        *cursor = n - 1;
    }
    let start = *cursor;
    let mut align: Option<egui::Align> = None;
    let mut focus: Option<usize> = None;
    let page = page.max(1);
    ctx.input_mut(|i| {
        use egui::{Align, Key, Modifiers};
        let none = Modifiers::NONE;
        // Tab / Shift+Tab: next / previous field (always active); focus follows.
        if i.consume_key(Modifiers::SHIFT, Key::Tab) {
            *cursor = cursor.saturating_sub(1);
            focus = Some(*cursor);
        }
        if i.consume_key(none, Key::Tab) {
            if *cursor + 1 < n {
                *cursor += 1;
            }
            focus = Some(*cursor);
        }
        // Row navigation only when not typing in a field.
        if !editing {
            if i.consume_key(none, Key::ArrowDown) && *cursor + 1 < n {
                *cursor += 1;
            }
            if i.consume_key(none, Key::ArrowUp) {
                *cursor = cursor.saturating_sub(1);
            }
            if i.consume_key(none, Key::PageDown) {
                *cursor = (*cursor + page).min(n - 1);
                align = Some(Align::TOP);
            }
            if i.consume_key(none, Key::PageUp) {
                *cursor = cursor.saturating_sub(page);
                align = Some(Align::TOP);
            }
            if i.consume_key(none, Key::Home) {
                *cursor = 0;
                align = Some(Align::TOP);
            }
            if i.consume_key(none, Key::End) {
                *cursor = n - 1;
                align = Some(Align::BOTTOM);
            }
            if i.consume_key(none, Key::Enter) {
                focus = Some(*cursor);
            }
        }
    });
    (*cursor != start, align, focus)
}

/// Parse a `.ftl` file into ordered (key, value) pairs (comments/blanks skipped).
fn parse_ftl(content: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim_start();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if let Some(eq) = line.find(" = ") {
            let key = line[..eq].trim().to_string();
            let val = line[eq + 3..].to_string();
            if !key.is_empty()
                && key
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                out.push((key, val));
            }
        }
    }
    out
}

impl XimodApp {
    /// Load the source and target `main.ftl` files and build the editor rows.
    pub fn load_translation_entries(&mut self) {
        let dir = self.i18n.locales_dir().to_path_buf();
        let src_path = dir.join(&self.trans_source_lang).join("main.ftl");
        let tgt_path = dir.join(&self.trans_target_lang).join("main.ftl");

        let src_content = std::fs::read_to_string(&src_path).unwrap_or_default();
        let tgt_content = std::fs::read_to_string(&tgt_path).unwrap_or_default();

        let tgt_map: HashMap<String, String> = parse_ftl(&tgt_content).into_iter().collect();

        // Restore the font and author of an existing translation so they can be
        // corrected. The country stays the one chosen with the flag (never
        // overridden by the file), and the endonyms are handled below.
        let meta = parse_metadata(&tgt_content);
        // Reset the font from the target file's header; if it has none,
        // refresh_translation_meta below fills it from Languages.json for the new
        // language (so switching languages never keeps the previous font).
        self.trans_font = meta
            .get("font")
            .cloned()
            .filter(|s| !s.is_empty())
            .unwrap_or_default();
        if let Some(v) = meta.get("author") {
            self.trans_author = v.clone();
        }
        // The country/language endonyms are set by this call: from the file's
        // own header when it exists, otherwise seeded from the reference JSON
        // (see refresh_translation_meta).
        self.refresh_translation_meta();

        let mut entries = Vec::new();
        for (key, sval) in parse_ftl(&src_content) {
            if NON_TRANSLATABLE.contains(&key.as_str()) {
                continue;
            }
            let a = analyze(&sval);
            // Pre-fill from the target file's existing translation, if any.
            let input = tgt_map
                .get(&key)
                .map(|tval| analyze(tval).display)
                .unwrap_or_default();
            entries.push(TransEntry {
                key,
                source_value: sval,
                display: a.display,
                input,
                prefix: a.prefix,
                suffix: a.suffix,
                marker_mode: a.marker_mode,
                tokens: a.tokens,
                has_tokens: a.has_tokens,
            });
        }
        self.trans_entries = entries;
    }

    /// React to a fresh (country, target language) selection by setting the two
    /// endonym fields.
    ///
    ///   * Country endonym — always resolved from `Countries.json` for the
    ///     country currently chosen with the flag. A language file serves several
    ///     countries, so this value is never taken from the `main.ftl` header; it
    ///     is keyed on (country, language) in Countries.json and therefore
    ///     follows the selected flag (e.g. German → "Deutschland" for Germany,
    ///     "Schweiz" for Switzerland).
    ///   * Language endonym — taken from the file's own header (`# @langname`)
    ///     when it has one, so a saved correction is shown; older files without a
    ///     header, or files that do not exist yet, fall back to `Languages.json`.
    ///
    /// Both fields stay editable so the user can correct them; a save then
    /// propagates the corrections to the JSON reference files.
    pub fn refresh_translation_meta(&mut self) {
        if self.trans_font.is_empty() {
            if let Some(f) = self.i18n.font_for(&self.trans_target_lang) {
                self.trans_font = f.to_string();
            }
        }

        // Country endonym: prefer the value the translator recorded in the
        // translation's own countryEndonyms.tsv (authoritative and travels with
        // the translation), then the exact Countries.json entry, and only then a
        // display fallback (the French/English country name). Reading the .tsv is
        // what lets a received translation refresh Countries.json on save, and
        // makes the field update consistently for every flag.
        let dir = self.i18n.locales_dir().to_path_buf();
        let tsv = read_tsv_country_endonym(&dir, &self.trans_target_lang, &self.trans_country);
        let exact = self
            .countries
            .endonym_exact(&self.trans_country, &self.trans_target_lang)
            .map(|s| s.to_string());
        match tsv.or(exact) {
            Some(v) => {
                self.trans_endonym = v;
                self.trans_endonym_authoritative = true;
            }
            None => {
                self.trans_endonym = self
                    .countries
                    .endonym_for(&self.trans_country, &self.trans_target_lang)
                    .unwrap_or("")
                    .to_string();
                self.trans_endonym_authoritative = false;
            }
        }

        // Language endonym: from the file's header when present, otherwise from
        // Languages.json (older files without a header, or no file yet).
        let ftl_path = self
            .i18n
            .locales_dir()
            .join(&self.trans_target_lang)
            .join("main.ftl");
        let meta = if !self.trans_target_lang.is_empty() && ftl_path.is_file() {
            parse_metadata(&std::fs::read_to_string(&ftl_path).unwrap_or_default())
        } else {
            HashMap::new()
        };
        self.trans_lang_endonym = match meta.get("langname") {
            Some(v) if !v.is_empty() => v.clone(),
            _ => self.i18n.display_name(&self.trans_target_lang),
        };

        // Remember the just-loaded values as the baseline. A save propagates the
        // font / endonyms to the reference JSON only when the user has since
        // changed one of these fields — never merely because a file header
        // happened to differ from the JSON (which would silently overwrite the
        // canonical data and wrongly report "reference data updated").
        self.trans_font_loaded = self.trans_font.clone();
        self.trans_endonym_loaded = self.trans_endonym.clone();
        self.trans_lang_endonym_loaded = self.trans_lang_endonym.clone();
    }

    /// Ask for a font file, which must live inside `assets/fonts`.
    ///
    /// A font stored anywhere else is refused: the program loads fonts by a path
    /// relative to that folder, so an outside file would not be found at the next
    /// start. The user is told to install it there first (the Google Fonts button
    /// opens the site where they can be downloaded).
    pub fn pick_translation_font(&mut self) {
        let Some(root) = crate::fonts::fonts_dir() else {
            self.trans_message = self.i18n.t("trans-font-dir-missing");
            return;
        };
        let picked = rfd::FileDialog::new()
            .add_filter("Fonts", &["ttf", "otf", "ttc"])
            .set_directory(&root)
            .pick_file();
        let Some(path) = picked else { return };

        // Keep only the part relative to assets/fonts.
        let rel = path
            .canonicalize()
            .ok()
            .and_then(|abs| {
                root.canonicalize()
                    .ok()
                    .and_then(|r| abs.strip_prefix(r).ok().map(|p| p.to_path_buf()))
            });
        match rel {
            Some(r) => {
                self.trans_font = r.to_string_lossy().replace('\\', "/");
                self.trans_message.clear();
            }
            None => self.trans_message = self.i18n.t("trans-font-outside"),
        }
    }

    /// Write back into `Languages.json` and `Countries.json` what the translator
    /// entered, so the endonyms and font become part of the reference data.
    ///
    /// Both files are rewritten only when something actually changed, and the
    /// JSON is edited through `serde_json::Value` so unknown fields and the
    /// overall structure are preserved.
    fn update_reference_data(&mut self) -> Vec<std::path::PathBuf> {
        let mut written = Vec::new();
        let lang = self.trans_target_lang.clone();
        if lang.is_empty() {
            return written;
        }

        // ---- Languages.json: endonym (name) and font of the language ----
        if let Some(path) = crate::data::find_data_file("Languages.json") {
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(mut doc) = serde_json::from_str::<serde_json::Value>(&text) {
                    let mut changed = false;
                    if let Some(arr) = doc.get_mut("languages").and_then(|v| v.as_array_mut()) {
                        for entry in arr.iter_mut() {
                            if entry.get("iso639_3").and_then(|v| v.as_str()) != Some(&lang) {
                                continue;
                            }
                            // Only write when the user actually edited the field
                            // (current value differs from the one loaded), and it
                            // differs from what the JSON already holds.
                            if self.trans_lang_endonym != self.trans_lang_endonym_loaded
                                && !self.trans_lang_endonym.is_empty()
                                && entry.get("name").and_then(|v| v.as_str())
                                    != Some(self.trans_lang_endonym.as_str())
                            {
                                entry["name"] =
                                    serde_json::Value::String(self.trans_lang_endonym.clone());
                                changed = true;
                            }
                            if self.trans_font != self.trans_font_loaded
                                && !self.trans_font.is_empty()
                                && entry.get("font").and_then(|v| v.as_str())
                                    != Some(self.trans_font.as_str())
                            {
                                entry["font"] =
                                    serde_json::Value::String(self.trans_font.clone());
                                changed = true;
                            }
                            break;
                        }
                    }
                    if changed {
                        if let Ok(s) = serde_json::to_string_pretty(&doc) {
                            if std::fs::write(&path, s + "\n").is_ok() {
                                written.push(path);
                            }
                        }
                    }
                }
            }
        }

        // ---- Countries.json: endonym of the country in that language ----
        // Write when the value is authoritative (from the translation's .tsv or
        // an exact entry) or the user edited it — never a mere French/English
        // fallback. The inner comparison still avoids redundant writes when the
        // stored value already matches.
        if (self.trans_endonym_authoritative || self.trans_endonym != self.trans_endonym_loaded)
            && !self.trans_country.is_empty()
            && !self.trans_endonym.is_empty()
        {
            if let Some(path) = crate::data::find_data_file("Countries.json") {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    if let Ok(mut doc) = serde_json::from_str::<serde_json::Value>(&text) {
                        let mut changed = false;
                        if let Some(arr) =
                            doc.get_mut("countries").and_then(|v| v.as_array_mut())
                        {
                            for country in arr.iter_mut() {
                                if country.get("a3").and_then(|v| v.as_str())
                                    != Some(self.trans_country.as_str())
                                {
                                    continue;
                                }
                                let langs = country
                                    .get_mut("languages")
                                    .and_then(|v| v.as_array_mut());
                                if let Some(langs) = langs {
                                    let existing = langs.iter_mut().find(|l| {
                                        l.get("iso639_3").and_then(|v| v.as_str())
                                            == Some(lang.as_str())
                                    });
                                    match existing {
                                        Some(l) => {
                                            if l.get("countryEndonym").and_then(|v| v.as_str())
                                                != Some(self.trans_endonym.as_str())
                                            {
                                                l["countryEndonym"] = serde_json::Value::String(
                                                    self.trans_endonym.clone(),
                                                );
                                                changed = true;
                                            }
                                        }
                                        None => {
                                            langs.push(serde_json::json!({
                                                "iso639_3": lang,
                                                "countryEndonym": self.trans_endonym,
                                            }));
                                            changed = true;
                                        }
                                    }
                                }
                                break;
                            }
                        }
                        if changed {
                            if let Ok(s) = serde_json::to_string_pretty(&doc) {
                                if std::fs::write(&path, s + "\n").is_ok() {
                                    written.push(path);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Reload so the interface immediately reflects the new data.
        if !written.is_empty() {
            self.countries = crate::data::CountriesData::load();
        }
        written
    }

    /// Record the country endonym in a small side file next to the translated
    /// `main.ftl`, so a maintainer who receives only the language folder can
    /// still update `Countries.json` (the endonym is no longer stored in the
    /// `.ftl` header, since a language is shared by several countries).
    ///
    /// Format: one tab-separated line per country,
    /// `<ISO 3166-1 alpha-3>\t<ISO 639-3>\t<country endonym>`. The ISO 639-3
    /// code is the language folder's, so the file accumulates one row per
    /// country (keyed on the country code: the current entry is inserted or
    /// updated). Only the codes and names actually used are written — the ISO
    /// code sets themselves are never redistributed.
    fn write_country_endonym_record(&self) {
        if self.trans_country.is_empty()
            || self.trans_endonym.is_empty()
            || self.trans_target_lang.is_empty()
        {
            return;
        }
        let path = self
            .i18n
            .locales_dir()
            .join(&self.trans_target_lang)
            .join("countryEndonyms.tsv");

        // Existing rows (country a3 -> endonym), then upsert the current one.
        let mut rows: std::collections::BTreeMap<String, String> =
            std::collections::BTreeMap::new();
        if let Ok(text) = std::fs::read_to_string(&path) {
            for line in text.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                let mut it = line.split('\t');
                if let (Some(a3), Some(_iso), Some(endo)) = (it.next(), it.next(), it.next()) {
                    rows.insert(a3.trim().to_string(), endo.to_string());
                }
            }
        }
        // Guard against a stray tab/newline in the typed endonym.
        let endonym = self.trans_endonym.replace(['\t', '\r', '\n'], " ");
        rows.insert(self.trans_country.clone(), endonym);

        let mut out = String::new();
        for (a3, endo) in &rows {
            out.push_str(&format!("{}\t{}\t{}\n", a3, self.trans_target_lang, endo));
        }
        let _ = std::fs::write(&path, out);
    }

    /// Package the translation for submission: a zip holding the translated
    /// `main.ftl` and, when present, the `countryEndonyms.tsv` side file (both
    /// inside the language folder).
    ///
    /// The archive deliberately does NOT bundle `Languages.json` (nor
    /// `Countries.json`). `Languages.json` is, in essence, the ISO 639-3 code
    /// set, whose terms of use forbid providing a means to redistribute it.
    /// The language-level data (language code, font, language endonym, author)
    /// travels in the `main.ftl` header; the country endonym — which is not in
    /// the header, as a language serves several countries — travels in
    /// `countryEndonyms.tsv`, so the maintainer can refresh Countries.json
    /// without shipping the ISO code sets.
    ///
    /// Returns the path of the archive.
    fn build_translation_package(&self) -> std::io::Result<std::path::PathBuf> {
        use std::io::Write;

        let lang = if self.trans_target_lang.is_empty() {
            "translation"
        } else {
            &self.trans_target_lang
        };
        let dir = std::env::temp_dir();
        let zip_path = dir.join(format!("ximod-translation-{}.zip", lang));

        let file = std::fs::File::create(&zip_path)?;
        let mut zip = zip::ZipWriter::new(file);
        let opts: zip::write::FileOptions<()> = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // Only the translated file, keeping its folder so the layout is obvious.
        // Its metadata header (country, language, font, endonyms, author) lets
        // the maintainer refresh the reference data without shipping the
        // ISO 639-3 code set.
        let ftl = self
            .i18n
            .locales_dir()
            .join(&self.trans_target_lang)
            .join("main.ftl");
        if ftl.is_file() {
            zip.start_file(format!("locales/{}/main.ftl", lang), opts)?;
            zip.write_all(&std::fs::read(&ftl)?)?;
        }
        // The country endonym travels here, not in the header.
        let record = self
            .i18n
            .locales_dir()
            .join(&self.trans_target_lang)
            .join("countryEndonyms.tsv");
        if record.is_file() {
            zip.start_file(format!("locales/{}/countryEndonyms.tsv", lang), opts)?;
            zip.write_all(&std::fs::read(&record)?)?;
        }
        zip.finish()?;
        Ok(zip_path)
    }

    /// Prepare the submission: build the archive, reveal it in the file manager
    /// and open a pre-filled e-mail. The user attaches the file and sends it —
    /// nothing leaves the machine without an explicit action.
    pub fn submit_translation(&mut self) {
        match self.build_translation_package() {
            Ok(zip_path) => {
                if let Some(parent) = zip_path.parent() {
                    crate::fonts::open_path(parent);
                }
                let subject = format!(
                    "XIMOD Architect - translation {}",
                    self.trans_target_lang
                );
                let body = format!(
                    "Language: {}\nCountry: {}\nAuthor: {}\n\nPlease attach: {}",
                    self.trans_target_lang,
                    self.trans_country,
                    self.trans_author,
                    zip_path.display()
                );
                crate::fonts::open_url(&format!(
                    "mailto:plplecuyer@gmail.com?subject={}&body={}",
                    urlencode(&subject),
                    urlencode(&body)
                ));
                self.trans_message = format!("{} {}", self.i18n.t("trans-package-ready"), zip_path.display());
            }
            Err(e) => {
                self.trans_message = format!("{} {}", self.i18n.t("trans-package-error"), e);
            }
        }
    }

    /// Write the target `main.ftl`, preserving the source file's structure.
    pub fn save_translation(&mut self) {
        if self.trans_target_lang.is_empty() {
            return;
        }
        // Translating requires a language identified by an ISO 639-3 code: the
        // locale folder, the font lookup and the reference data are all keyed on
        // it. A language absent from Languages.json cannot be used.
        if !self.i18n.has_language(&self.trans_target_lang) {
            self.trans_message = self.i18n.t("trans-lang-not-iso");
            return;
        }
        let dir = self.i18n.locales_dir().to_path_buf();
        let tgt_dir = dir.join(&self.trans_target_lang);
        let _ = std::fs::create_dir_all(&tgt_dir);
        let tgt_path = tgt_dir.join("main.ftl");

        let src_path = dir.join(&self.trans_source_lang).join("main.ftl");
        let src_content = std::fs::read_to_string(&src_path).unwrap_or_default();

        let map: HashMap<String, String> = self
            .trans_entries
            .iter()
            .map(|e| (e.key.clone(), reconstruct(e)))
            .collect();

        let mut out = String::new();
        // Metadata block first (Fluent comments: the file stays valid).
        //
        // A `main.ftl` describes a LANGUAGE, which may be spoken in several
        // countries, so it carries no country information: neither `@country`
        // (the country code) nor `@endonym` (the country's name, which is
        // country-specific). The country endonym lives in Countries.json,
        // keyed on (country, language), and is resolved dynamically from the
        // flag currently selected.
        out.push_str(META_HEADER);
        out.push('\n');
        out.push_str(&format!("# @language = {}\n", self.trans_target_lang));
        out.push_str(&format!("# @font = {}\n", self.trans_font));
        out.push_str(&format!("# @langname = {}\n", self.trans_lang_endonym));
        out.push_str(&format!("# @author = {}\n", self.trans_author));
        out.push('\n');

        let mut body_started = false;
        for line in src_content.lines() {
            // Never copy the source file's own metadata.
            if is_metadata_line(line) {
                continue;
            }
            // Collapse the blank line(s) separating the metadata block from the
            // body: exactly one blank was already emitted above. Otherwise a file
            // that is its own "displayed language" gains a blank line at every
            // save (two, three, … LFs after `# @author`).
            if !body_started {
                if line.trim().is_empty() {
                    continue;
                }
                body_started = true;
            }
            if let Some(eq) = line.find(" = ") {
                let key = line[..eq].trim();
                if let Some(val) = map.get(key) {
                    out.push_str(&format!("{} = {}\n", key, val));
                    continue;
                }
            }
            out.push_str(line);
            out.push('\n');
        }

        match std::fs::write(&tgt_path, out) {
            Ok(_) => {
                // Propagate the endonyms and font to the reference data.
                let updated = self.update_reference_data();
                // Export the country endonym next to the .ftl so a maintainer
                // who only receives the language folder can update Countries.json.
                self.write_country_endonym_record();
                self.status_message = self.i18n.t("trans-saved");
                self.trans_message = if updated.is_empty() {
                    String::new()
                } else {
                    self.i18n.t("trans-data-updated")
                };
            }
            Err(_) => self.status_message = self.i18n.t("trans-save-error"),
        }
    }

    /// Render the translation editor window.
    pub fn render_translation_window(&mut self, ctx: &egui::Context) {
        if !self.show_translation {
            self.free_window_closed("ximod_translation");
            return;
        }

        let title = self.i18n.t("trans-title");
        let lbl_source = self.i18n.t("trans-source-lang");
        let lbl_target = self.i18n.t("trans-target-lang");
        let col_key = self.i18n.t("trans-col-key");
        let col_source = self.i18n.t("trans-col-source");
        let col_target = self.i18n.t("trans-col-target");
        let btn_save = self.i18n.t("btn-save");
        let lbl_endonym = self.i18n.t("trans-endonym");
        let lbl_font = self.i18n.t("trans-font");
        let lbl_no_font = self.i18n.t("trans-no-font");
        let lbl_browse = self.i18n.t("trans-browse");
        let lbl_google = self.i18n.t("trans-google-fonts");
        let lbl_pick_country = self.i18n.t("trans-pick-country");
        let lbl_lang_endonym = self.i18n.t("trans-lang-endonym");
        let lbl_author = self.i18n.t("trans-author");
        let lbl_submit = self.i18n.t("trans-submit");
        let lbl_submit_hint = self.i18n.t("trans-submit-hint");
        let lbl_not_iso = self.i18n.t("trans-lang-not-iso");

        let country_langs: Vec<String> = self
            .country_languages
            .languages_for(&self.trans_country)
            .to_vec();
        // A translation can only be produced for a language carrying an
        // ISO 639-3 code (i.e. listed in Languages.json).
        let can_translate = !self.trans_target_lang.is_empty()
            && self.i18n.has_language(&self.trans_target_lang);

        let locales: Vec<String> = self.i18n.available_locales().to_vec();

        let mut do_close = false;
        let mut do_save = false;
        let mut reload = false;
        let mut meta_refresh = false;
        let mut pick_font = false;
        let mut do_submit = false;

        // Independent OS-level window (viewport): freely movable and resizable,
        // including onto a second screen, like the Preview / Properties / XML
        // editor windows.
        let vb = self.free_viewport_builder(ctx, "ximod_translation", title, [980.0, 640.0]);
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("ximod_translation"),
            vb,
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
ui.horizontal_top(|ui| {
                    // --- Flag: picks the country whose languages can be translated
                    let flag_size = egui::vec2(120.0, 80.0);
                    let flag_path = self
                        .countries
                        .flag_for(&self.trans_country)
                        .and_then(|f| crate::data::flags_dir().map(|d| d.join(f)))
                        .filter(|p| p.is_file());
                    let flag_resp = match &flag_path {
                        Some(p) => ui.add_sized(
                            flag_size,
                            egui::ImageButton::new(
                                egui::Image::from_uri(format!("file://{}", p.display()))
                                    .fit_to_exact_size(flag_size),
                            ),
                        ),
                        None => ui.add_sized(
                            flag_size,
                            egui::Button::new(egui::RichText::new("\u{1F3F3}").size(26.0)),
                        ),
                    };
                    if flag_resp.on_hover_text(&lbl_pick_country).clicked() {
                        self.flag_target = crate::ui::flag_picker::FlagTarget::Translation;
                        self.flag_filter.clear();
                        self.flag_cursor = 0;
                        self.flag_scroll_offset = 0.0;
                        self.show_flag_picker = true;
                    }

                    ui.add_space(10.0);

                    // --- Country endonym, rendered with the chosen font
                    ui.vertical(|ui| {
                        ui.label(&lbl_endonym);
                        let font_id = egui::FontId::new(
                            16.0,
                            egui::FontFamily::Name(crate::fonts::PREVIEW_FAMILY.into()),
                        );
                        ui.add(
                            egui::TextEdit::multiline(&mut self.trans_endonym)
                                .desired_width(230.0)
                                .desired_rows(2)
                                .font(font_id.clone()),
                        );

                        ui.add_space(4.0);
                        ui.label(&lbl_lang_endonym);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.trans_lang_endonym)
                                .desired_width(230.0)
                                .font(font_id),
                        );

                        ui.add_space(4.0);
                        ui.label(&lbl_author);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.trans_author)
                                .desired_width(230.0),
                        );
                    });

                    ui.add_space(12.0);

                    // --- Languages
                    ui.vertical(|ui| {
                        // Source: any language that already has a main.ftl.
                        ui.horizontal(|ui| {
                            ui.label(&lbl_source);
                            egui::ComboBox::from_id_salt("trans_src")
                                .selected_text(self.i18n.display_name(&self.trans_source_lang))
                                .height(260.0)
                                .show_ui(ui, |ui| {
                                    egui::ScrollArea::both().show(ui, |ui| {
                                        for loc in &locales {
                                            if ui
                                                .selectable_label(
                                                    self.trans_source_lang == *loc,
                                                    self.i18n.display_name(loc),
                                                )
                                                .clicked()
                                            {
                                                self.trans_source_lang = loc.clone();
                                                reload = true;
                                            }
                                        }
                                    });
                                });
                        });

                        ui.add_space(4.0);

                        // Target: every language of the selected country.
                        ui.horizontal(|ui| {
                            ui.label(&lbl_target);
                            ui.add_enabled_ui(!country_langs.is_empty(), |ui| {
                                egui::ComboBox::from_id_salt("trans_tgt")
                                    .selected_text(
                                        self.i18n.display_name(&self.trans_target_lang),
                                    )
                                    .height(260.0)
                                    .show_ui(ui, |ui| {
                                        egui::ScrollArea::both().show(ui, |ui| {
                                            for loc in &country_langs {
                                                if ui
                                                    .selectable_label(
                                                        self.trans_target_lang == *loc,
                                                        self.i18n.display_name(loc),
                                                    )
                                                    .clicked()
                                                {
                                                    self.trans_target_lang = loc.clone();
                                                    reload = true;
                                                    meta_refresh = true;
                                                }
                                            }
                                        });
                                    });
                            });
                        });

                        ui.add_space(4.0);

                        // Font of the target language.
                        ui.horizontal(|ui| {
                            ui.label(&lbl_font);
                            let shown = if self.trans_font.is_empty() {
                                lbl_no_font.clone()
                            } else {
                                self.trans_font.clone()
                            };
                            ui.add(
                                egui::Label::new(egui::RichText::new(shown).monospace())
                                    .wrap_mode(egui::TextWrapMode::Truncate),
                            );
                        });
                        ui.horizontal(|ui| {
                            if ui.button(&lbl_browse).clicked() {
                                pick_font = true;
                            }
                            if ui.button(&lbl_google).clicked() {
                                crate::fonts::open_url("https://fonts.google.com/");
                            }
                            ui.add_space(12.0);
                            ui.add_enabled_ui(can_translate, |ui| {
                                if ui.button(&btn_save).clicked() {
                                    do_save = true;
                                }
                                if ui
                                    .button(&lbl_submit)
                                    .on_hover_text(&lbl_submit_hint)
                                    .clicked()
                                {
                                    do_submit = true;
                                }
                            });
                        });
                    });
                });

                if !can_translate && !self.trans_target_lang.is_empty() {
                    ui.colored_label(
                        egui::Color32::from_rgb(220, 80, 80),
                        &lbl_not_iso,
                    );
                }
                if !self.trans_message.is_empty() {
                    ui.colored_label(
                        egui::Color32::from_rgb(220, 120, 60),
                        &self.trans_message,
                    );
                }

                ui.separator();

                // Column layout: the key column has a fixed width; the source
                // (column 2) and the editable translation (column 3) share the
                // remaining width *equally* and grow together when the window is
                // widened — helpful for long strings and CJK scripts that would
                // otherwise be cramped.
                let key_w = 150.0_f32;
                let gap = ui.spacing().item_spacing.x;
                // Reserve for the scrollbar/margins so the header (above the
                // scroll area) and the rows (inside it) stay aligned.
                let col_w = ((ui.available_width() - key_w - gap * 2.0 - 18.0) / 2.0).max(160.0);
                let row_h = ui.spacing().interact_size.y;

                egui::Grid::new("trans_header")
                    .num_columns(3)
                    .show(ui, |ui| {
                        ui.add_sized(
                            [key_w, row_h],
                            egui::Label::new(egui::RichText::new(&col_key).strong()),
                        );
                        ui.add_sized(
                            [col_w, row_h],
                            egui::Label::new(egui::RichText::new(&col_source).strong()),
                        );
                        ui.add_sized(
                            [col_w, row_h],
                            egui::Label::new(egui::RichText::new(&col_target).strong()),
                        );
                        ui.end_row();
                    });

                // Keyboard navigation for the table. Row heights vary (the source
                // column wraps), so the page size uses the average row height
                // measured on the previous frame — recomputed each frame, so it
                // follows window resizing (a font-based estimate seeds frame 1).
                let n = self.trans_entries.len();
                let editing = ctx.wants_keyboard_input();
                let est = ((ui.available_height() / row_h.max(1.0)).floor() as usize).max(1);
                let page = if self.trans_visible > 0 {
                    self.trans_visible
                } else {
                    est
                };
                let (moved, nav_align, focus_row) =
                    handle_trans_keys(ctx, &mut self.trans_cursor, n, page, editing);

                let out = egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("trans_grid")
                        .num_columns(3)
                        .striped(true)
                        .show(ui, |ui| {
                            for idx in 0..n {
                                let is_cursor = idx == self.trans_cursor;
                                let key = self.trans_entries[idx].key.clone();
                                let display = self.trans_entries[idx].display.clone();
                                // col 1: key — highlighted on the cursor row.
                                let key_resp = ui
                                    .scope(|ui| {
                                        ui.set_min_width(key_w);
                                        ui.set_max_width(key_w);
                                        ui.selectable_label(
                                            is_cursor,
                                            egui::RichText::new(&key).monospace(),
                                        )
                                        .on_hover_text(&key)
                                    })
                                    .inner;
                                if key_resp.clicked() {
                                    self.trans_cursor = idx;
                                }
                                if moved && is_cursor && focus_row.is_none() {
                                    key_resp.scroll_to_me(nav_align);
                                }
                                // col 2: source label — same width as col 3, wraps
                                ui.scope(|ui| {
                                    ui.set_min_width(col_w);
                                    ui.set_max_width(col_w);
                                    ui.add(
                                        egui::Label::new(&display)
                                            .wrap_mode(egui::TextWrapMode::Wrap),
                                    );
                                });
                                // col 3: editable translation — same width as col 2
                                let resp = ui.add(
                                    egui::TextEdit::singleline(
                                        &mut self.trans_entries[idx].input,
                                    )
                                    .desired_width(col_w),
                                );
                                if focus_row == Some(idx) {
                                    resp.request_focus();
                                    resp.scroll_to_me(Some(egui::Align::Center));
                                }
                                if resp.has_focus() {
                                    self.trans_cursor = idx;
                                }
                                ui.end_row();
                            }
                        });
                });
                // Average row height → number of rows that fit, for Page Up/Down.
                if n > 0 {
                    let avg = (out.content_size.y / n as f32).max(1.0);
                    self.trans_visible =
                        ((out.inner_rect.height() / avg).floor() as usize).max(1);
                }
                });

                crate::ui::main_window::record_win_geom(&mut self.config, ctx, "ximod_translation");
                if ctx.input(|i| i.viewport().close_requested()) {
                    do_close = true;
                }
            },
        );

        if pick_font {
            self.pick_translation_font();
        }
        if meta_refresh {
            self.refresh_translation_meta();
        }
        if reload {
            self.load_translation_entries();
        }
        if do_save {
            self.save_translation();
        }
        if do_submit {
            self.submit_translation();
        }
        if do_close {
            self.show_translation = false;
            self.free_window_closed("ximod_translation");
        }
    }
}
