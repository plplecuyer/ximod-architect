//! Lossless FOMOD patcher — the core of "translate an existing FOMOD".
//!
//! Translating a third-party installer must not rewrite it: the regular
//! reader/writer in [`crate::xml`] goes through the [`crate::models::Ximod`]
//! model and therefore drops everything the model does not know (comments,
//! `moduleDependencies`, unknown attributes, the author's formatting, …).
//!
//! This module works directly on the source text instead:
//!
//! 1. [`scan`] walks the document once with `quick-xml` and records the byte
//!    range of every translatable string (an attribute value or the text
//!    content of an element) together with its stable key.
//! 2. [`extract_units`] turns those sites into [`TUnit`]s.
//! 3. [`apply_units`] splices the translated strings into the recorded ranges
//!    and copies every other byte of the input verbatim.
//!
//! Because untouched bytes are never re-serialised, applying an empty set of
//! translations returns the input byte-for-byte, whatever it contains.
//!
//! Keys (0-based indices, the group index restarts in each step and the plugin
//! index in each group):
//!
//! | File             | Key                                              |
//! |------------------|--------------------------------------------------|
//! | `info.xml`       | `info/Name`, `info/Author`, `info/Website`, `info/Description` |
//! | `ModuleConfig`   | `config/moduleName`                              |
//! |                  | `config/step[i]/name`                            |
//! |                  | `config/step[i]/group[j]/name`                   |
//! |                  | `config/step[i]/group[j]/plugin[k]/name`         |
//! |                  | `config/step[i]/group[j]/plugin[k]/description`  |
//!
//! Text content is extracted *trimmed* (like the project loader does) and the
//! whitespace surrounding it in the file is kept when a translation is
//! applied, so a pretty-printed `<description>` stays pretty-printed.

#![allow(dead_code)]

use std::collections::HashMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};

use crate::models::translate::{FileFingerprint, TField, TFile, TStatus, TUnit};

/// Separator used between the levels of a unit's breadcrumb.
pub const CONTEXT_SEPARATOR: &str = " \u{203A} ";

// ---------------------------------------------------------------------------
// Encodings
// ---------------------------------------------------------------------------

/// Byte encoding of an XML file on disk, as detected by
/// [`read_xml_with_encoding`]. Written back unchanged by [`encode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// UTF-8 without a byte order mark (also used for legacy single-byte
    /// files, which are transcoded to UTF-8 when written back).
    Utf8,
    /// UTF-8 with the `EF BB BF` byte order mark.
    Utf8Bom,
    /// UTF-16 little endian (written with its `FF FE` byte order mark).
    Utf16Le,
    /// UTF-16 big endian (written with its `FE FF` byte order mark).
    Utf16Be,
}

impl Encoding {
    /// Stable, language-neutral label stored in the sidecar fingerprint.
    pub fn label(self) -> &'static str {
        match self {
            Self::Utf8 => "utf-8",
            Self::Utf8Bom => "utf-8-bom",
            Self::Utf16Le => "utf-16le",
            Self::Utf16Be => "utf-16be",
        }
    }
}

/// Windows-1252 mapping of the bytes `0x80..=0x9F` (the rest of the upper half
/// is identical to Latin-1). The five unassigned bytes map to the matching C1
/// control code, as browsers do.
const CP1252_HIGH: [char; 32] = [
    '\u{20AC}', '\u{0081}', '\u{201A}', '\u{0192}', '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}', '\u{02C6}',
    '\u{2030}', '\u{0160}', '\u{2039}', '\u{0152}', '\u{008D}', '\u{017D}', '\u{008F}', '\u{0090}', '\u{2018}',
    '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}', '\u{02DC}', '\u{2122}', '\u{0161}',
    '\u{203A}', '\u{0153}', '\u{009D}', '\u{017E}', '\u{0178}',
];

/// Decode one Windows-1252 byte.
fn cp1252_char(b: u8) -> char {
    match b {
        0x80..=0x9F => CP1252_HIGH[(b - 0x80) as usize],
        _ => b as char,
    }
}

/// Decode bytes as UTF-8, mapping every byte that is not part of a valid
/// UTF-8 sequence through Windows-1252. Never fails.
fn decode_utf8_or_cp1252(mut bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    loop {
        match std::str::from_utf8(bytes) {
            Ok(s) => {
                out.push_str(s);
                return out;
            }
            Err(e) => {
                let (valid, rest) = bytes.split_at(e.valid_up_to());
                out.push_str(&String::from_utf8_lossy(valid));
                // `error_len` is `None` for a sequence truncated by the end of
                // the input: every remaining byte is then invalid.
                let bad = e.error_len().unwrap_or(rest.len()).max(1).min(rest.len());
                out.extend(rest[..bad].iter().map(|&b| cp1252_char(b)));
                bytes = &rest[bad..];
            }
        }
    }
}

/// Decode UTF-16 code units (unpaired surrogates become U+FFFD, a trailing odd
/// byte is ignored).
fn decode_utf16(bytes: &[u8], little_endian: bool) -> String {
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| {
            if little_endian {
                u16::from_le_bytes([c[0], c[1]])
            } else {
                u16::from_be_bytes([c[0], c[1]])
            }
        })
        .collect();
    String::from_utf16_lossy(&units)
}

/// Decode the raw bytes of an XML file and report how they were encoded.
///
/// Detection order: byte order mark, then a BOM-less UTF-16 heuristic (every
/// other byte of the first 64 is zero), then UTF-8 with a Windows-1252
/// fallback for invalid bytes. Never fails: the worst case is a lossy decode.
pub fn decode_xml_bytes(bytes: &[u8]) -> (String, Encoding) {
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return (decode_utf16(&bytes[2..], true), Encoding::Utf16Le);
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return (decode_utf16(&bytes[2..], false), Encoding::Utf16Be);
    }
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return (decode_utf8_or_cp1252(&bytes[3..]), Encoding::Utf8Bom);
    }

    // UTF-16 without a BOM: an XML file starts with ASCII (`<?xml`, `<config`),
    // so one byte of each pair is zero.
    let head = &bytes[..bytes.len().min(64) & !1];
    if head.len() >= 4 {
        let odd_zero = head.chunks_exact(2).all(|c| c[0] != 0 && c[1] == 0);
        let even_zero = head.chunks_exact(2).all(|c| c[0] == 0 && c[1] != 0);
        if odd_zero {
            return (decode_utf16(bytes, true), Encoding::Utf16Le);
        }
        if even_zero {
            return (decode_utf16(bytes, false), Encoding::Utf16Be);
        }
    }

    (decode_utf8_or_cp1252(bytes), Encoding::Utf8)
}

/// Read an XML file as text together with its on-disk encoding.
///
/// Only an I/O error can fail; see [`decode_xml_bytes`] for the detection.
pub fn read_xml_with_encoding(path: &Path) -> Result<(String, Encoding)> {
    let bytes = std::fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;
    Ok(decode_xml_bytes(&bytes))
}

/// Byte range of the value of the `encoding` pseudo-attribute of the XML
/// declaration (without its quotes), when the document has one.
fn declared_encoding_range(xml: &str) -> Option<Range<usize>> {
    if !xml.starts_with("<?xml") {
        return None;
    }
    let decl_end = xml.find("?>")?;
    let decl = &xml[..decl_end];
    let at = decl.find("encoding")?;
    let after = &decl[at + "encoding".len()..];
    let eq = after.find('=')?;
    if !after[..eq].trim().is_empty() {
        return None;
    }
    let rest = &after[eq + 1..];
    let quote_at = rest.find(['"', '\''])?;
    if !rest[..quote_at].trim().is_empty() {
        return None;
    }
    let quote = rest[quote_at..].chars().next()?;
    let value = &rest[quote_at + 1..];
    let close = value.find(quote)?;
    let start = decl.len() - value.len();
    Some(start..start + close)
}

/// Make the XML declaration agree with the bytes that are about to be written.
///
/// A file declared `encoding="windows-1252"` (or `iso-8859-1`, …) is always
/// written back as UTF-8 by [`encode`]. As long as it is pure ASCII the bytes
/// are identical in both charsets and nothing is touched; as soon as it holds
/// a non-ASCII character (a translation usually adds some) the declaration is
/// rewritten to `utf-8`, otherwise mod managers would decode the file with the
/// wrong charset. UTF-16 documents are left alone.
pub fn fix_declared_encoding(xml: &str, enc: Encoding) -> std::borrow::Cow<'_, str> {
    use std::borrow::Cow;
    if !matches!(enc, Encoding::Utf8 | Encoding::Utf8Bom) || xml.is_ascii() {
        return Cow::Borrowed(xml);
    }
    let Some(range) = declared_encoding_range(xml) else {
        return Cow::Borrowed(xml);
    };
    let declared = xml[range.clone()].to_ascii_lowercase();
    if declared == "utf-8" || declared == "utf8" {
        return Cow::Borrowed(xml);
    }
    let mut out = String::with_capacity(xml.len());
    out.push_str(&xml[..range.start]);
    out.push_str("utf-8");
    out.push_str(&xml[range.end..]);
    Cow::Owned(out)
}

/// Encode a document for writing, in the encoding it was read with (the byte
/// order mark is restored). See [`fix_declared_encoding`] for the single case
/// where the text itself is adjusted.
pub fn encode(xml: &str, enc: Encoding) -> Vec<u8> {
    let xml = fix_declared_encoding(xml, enc);
    match enc {
        Encoding::Utf8 => xml.as_bytes().to_vec(),
        Encoding::Utf8Bom => {
            let mut out = Vec::with_capacity(xml.len() + 3);
            out.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
            out.extend_from_slice(xml.as_bytes());
            out
        }
        Encoding::Utf16Le => {
            let mut out = Vec::with_capacity(xml.len() * 2 + 2);
            out.extend_from_slice(&[0xFF, 0xFE]);
            for u in xml.encode_utf16() {
                out.extend_from_slice(&u.to_le_bytes());
            }
            out
        }
        Encoding::Utf16Be => {
            let mut out = Vec::with_capacity(xml.len() * 2 + 2);
            out.extend_from_slice(&[0xFE, 0xFF]);
            for u in xml.encode_utf16() {
                out.extend_from_slice(&u.to_be_bytes());
            }
            out
        }
    }
}

/// Fingerprint of a source file (raw bytes as found on disk), stored in the
/// sidecar to detect that the FOMOD changed since the translation was made.
pub fn fingerprint(bytes: &[u8], enc: Encoding) -> FileFingerprint {
    FileFingerprint {
        crc32: crate::archive::crc32(bytes),
        bytes: bytes.len() as u64,
        encoding: enc.label().to_string(),
    }
}

// ---------------------------------------------------------------------------
// Locating the files of a FOMOD
// ---------------------------------------------------------------------------

/// The installer files found under a mod root.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FomodFiles {
    /// The `fomod` directory (existing spelling, or `<root>/fomod`).
    pub dir: PathBuf,
    /// `info.xml`, when present.
    pub info: Option<PathBuf>,
    /// `ModuleConfig.xml`, when present.
    pub config: Option<PathBuf>,
}

/// Find an entry of `dir` by name, ignoring ASCII case (third-party archives
/// ship `Fomod/`, `Info.xml`, `moduleconfig.xml`, … which only work as-is on
/// case-insensitive file systems). An exact match wins.
pub(crate) fn find_entry_ignore_case(dir: &Path, name: &str, want_dir: bool) -> Option<PathBuf> {
    let exact = dir.join(name);
    let matches_kind = |p: &Path| if want_dir { p.is_dir() } else { p.is_file() };
    if matches_kind(&exact) {
        return Some(exact);
    }
    let mut found: Vec<PathBuf> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(name))
        .map(|e| e.path())
        .filter(|p| matches_kind(p))
        .collect();
    found.sort();
    found.into_iter().next()
}

/// Locate the `fomod` directory of `root` and its two XML files, ignoring the
/// case of their names.
pub fn locate_fomod_files(root: &Path) -> FomodFiles {
    let dir = find_entry_ignore_case(root, "fomod", true).unwrap_or_else(|| root.join("fomod"));
    let info = find_entry_ignore_case(&dir, "info.xml", false);
    let config = find_entry_ignore_case(&dir, "ModuleConfig.xml", false);
    FomodFiles { dir, info, config }
}

// ---------------------------------------------------------------------------
// Scanning
// ---------------------------------------------------------------------------

/// One piece of the direct text content of an element.
#[derive(Debug, Clone)]
struct TextPart {
    /// Byte range of the content: the whole event for plain text, the inside
    /// of the `<![CDATA[…]]>` markers for CDATA. `None` if it could not be
    /// located in the input.
    range: Option<Range<usize>>,
    cdata: bool,
    /// Decoded content (entities resolved for plain text).
    text: String,
}

/// Where a translatable string lives in the document.
#[derive(Debug, Clone)]
enum Place {
    /// An attribute value (range excludes the quotes).
    Attr(Option<Range<usize>>),
    /// The text content of an element.
    Text(Vec<TextPart>),
}

/// A translatable string found in a document.
#[derive(Debug, Clone)]
struct Site {
    key: String,
    field: TField,
    source: String,
    context: String,
    place: Place,
}

/// Result of one pass over a document.
#[derive(Debug, Default)]
struct Scan {
    sites: Vec<Site>,
    /// Value ranges of `order` attributes that are not `Explicit`.
    order_sites: Vec<Range<usize>>,
    /// Parse error that stopped the scan, if any (sites found before it are
    /// still reported).
    error: Option<String>,
}

/// Byte range of `part` inside `base`, when `part` is a sub-slice of it.
///
/// `Reader::from_str` + `read_event` hands out events borrowed from the input,
/// so comparing addresses gives exact offsets without depending on how the
/// reader accounts for its position.
fn offset_in(base: &str, part: &[u8]) -> Option<Range<usize>> {
    let b = base.as_ptr() as usize;
    let p = part.as_ptr() as usize;
    if p >= b && p + part.len() <= b + base.len() {
        Some(p - b..p - b + part.len())
    } else {
        None
    }
}

/// Lower-cased local name of an element.
fn lower_name(e: &BytesStart<'_>) -> String {
    String::from_utf8_lossy(e.local_name().as_ref()).to_lowercase()
}

/// Unescaped value and byte range of the attribute `key` of `e`.
fn find_attr(xml: &str, e: &BytesStart<'_>, key: &[u8]) -> Option<(String, Option<Range<usize>>)> {
    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == key {
            let value = attr
                .unescape_value()
                .map(|c| c.into_owned())
                .unwrap_or_else(|_| String::from_utf8_lossy(&attr.value).into_owned());
            return Some((value, offset_in(xml, &attr.value)));
        }
    }
    None
}

/// XML parsers hand out `\n` for every line break; do the same so a unit's
/// source does not depend on the line endings of the file.
fn normalize_newlines(s: &str) -> String {
    if s.contains('\r') {
        s.replace("\r\n", "\n").replace('\r', "\n")
    } else {
        s.to_string()
    }
}

/// Join the non-empty levels of a breadcrumb.
fn breadcrumb(levels: &[&str]) -> String {
    levels
        .iter()
        .filter(|l| !l.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(CONTEXT_SEPARATOR)
}

/// Text element being captured.
struct Capture {
    key: String,
    field: TField,
    context: String,
    /// Stack depth while directly inside the element.
    depth: usize,
    parts: Vec<TextPart>,
}

/// Mutable state of a scan.
#[derive(Default)]
struct Walker {
    stack: Vec<String>,
    capture: Option<Capture>,
    seen: std::collections::HashSet<&'static str>,
    steps: usize,
    groups: usize,
    plugins: usize,
    step: usize,
    group: usize,
    plugin: usize,
    step_name: String,
    group_name: String,
    plugin_name: String,
}

/// Canonical spelling and field of the translatable children of `<fomod>`.
fn info_field(lower: &str) -> Option<(&'static str, TField)> {
    match lower {
        "name" => Some(("Name", TField::InfoName)),
        "author" => Some(("Author", TField::InfoAuthor)),
        "website" => Some(("Website", TField::InfoWebsite)),
        "description" => Some(("Description", TField::InfoDescription)),
        _ => None,
    }
}

impl Walker {
    /// Handle a start tag: record attribute sites, advance the counters and
    /// return the text capture to start for this element, if any.
    fn open(&mut self, xml: &str, file: TFile, name: &str, e: &BytesStart<'_>, scan: &mut Scan) -> Option<Capture> {
        match file {
            TFile::Info => {
                // Direct children of the root element only, first occurrence.
                if self.stack.len() != 1 {
                    return None;
                }
                let (canonical, field) = info_field(name)?;
                if !self.seen.insert(canonical) {
                    return None;
                }
                Some(Capture {
                    key: format!("info/{canonical}"),
                    field,
                    context: String::new(),
                    depth: self.stack.len() + 1,
                    parts: Vec::new(),
                })
            }
            TFile::Config => self.open_config(xml, name, e, scan),
        }
    }

    fn open_config(&mut self, xml: &str, name: &str, e: &BytesStart<'_>, scan: &mut Scan) -> Option<Capture> {
        let in_element = |stack: &[String], n: &str| stack.iter().any(|s| s == n);
        match name {
            "modulename" if self.stack.len() == 1 && self.seen.insert("moduleName") => {
                return Some(Capture {
                    key: "config/moduleName".to_string(),
                    field: TField::ModuleName,
                    context: String::new(),
                    depth: self.stack.len() + 1,
                    parts: Vec::new(),
                });
            }
            "installsteps" | "optionalfilegroups" | "plugins" => {
                if let Some((value, Some(range))) = find_attr(xml, e, b"order")
                    && value != "Explicit"
                {
                    scan.order_sites.push(range);
                }
            }
            "installstep" => {
                self.step = self.steps;
                self.steps += 1;
                self.groups = 0;
                self.plugins = 0;
                self.group_name.clear();
                self.plugin_name.clear();
                let attr = find_attr(xml, e, b"name");
                self.step_name = attr.as_ref().map(|a| a.0.clone()).unwrap_or_default();
                if let Some((value, range)) = attr
                    && !value.is_empty()
                {
                    scan.sites.push(Site {
                        key: format!("config/step[{}]/name", self.step),
                        field: TField::StepName,
                        context: breadcrumb(&[&value]),
                        source: value,
                        place: Place::Attr(range),
                    });
                }
            }
            "group" if in_element(&self.stack, "installstep") => {
                self.group = self.groups;
                self.groups += 1;
                self.plugins = 0;
                self.plugin_name.clear();
                let attr = find_attr(xml, e, b"name");
                self.group_name = attr.as_ref().map(|a| a.0.clone()).unwrap_or_default();
                if let Some((value, range)) = attr
                    && !value.is_empty()
                {
                    scan.sites.push(Site {
                        key: format!("config/step[{}]/group[{}]/name", self.step, self.group),
                        field: TField::GroupName,
                        context: breadcrumb(&[&self.step_name, &value]),
                        source: value,
                        place: Place::Attr(range),
                    });
                }
            }
            "plugin" if in_element(&self.stack, "group") => {
                self.plugin = self.plugins;
                self.plugins += 1;
                let attr = find_attr(xml, e, b"name");
                self.plugin_name = attr.as_ref().map(|a| a.0.clone()).unwrap_or_default();
                if let Some((value, range)) = attr
                    && !value.is_empty()
                {
                    scan.sites.push(Site {
                        key: format!(
                            "config/step[{}]/group[{}]/plugin[{}]/name",
                            self.step, self.group, self.plugin
                        ),
                        field: TField::PluginName,
                        context: breadcrumb(&[&self.step_name, &self.group_name, &value]),
                        source: value,
                        place: Place::Attr(range),
                    });
                }
            }
            "description"
                if self.stack.last().map(String::as_str) == Some("plugin") && in_element(&self.stack, "group") =>
            {
                return Some(Capture {
                    key: format!(
                        "config/step[{}]/group[{}]/plugin[{}]/description",
                        self.step, self.group, self.plugin
                    ),
                    field: TField::PluginDescription,
                    context: breadcrumb(&[&self.step_name, &self.group_name, &self.plugin_name]),
                    depth: self.stack.len() + 1,
                    parts: Vec::new(),
                });
            }
            _ => {}
        }
        None
    }

    /// Close the capture of a text element and record its site.
    fn finish_capture(&mut self, scan: &mut Scan) {
        let Some(cap) = self.capture.take() else { return };
        let full: String = cap.parts.iter().map(|p| p.text.as_str()).collect();
        let source = normalize_newlines(full.trim());
        if source.is_empty() {
            return;
        }
        scan.sites.push(Site {
            key: cap.key,
            field: cap.field,
            source,
            context: cap.context,
            place: Place::Text(cap.parts),
        });
    }
}

/// Walk `xml` once and collect every translatable string of `file`.
fn scan(xml: &str, file: TFile) -> Scan {
    let mut reader = Reader::from_str(xml);
    {
        let config = reader.config_mut();
        config.trim_text(false);
        config.expand_empty_elements = false;
    }

    let mut scan = Scan::default();
    let mut w = Walker::default();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = lower_name(&e);
                // Nothing is translatable inside an element being captured.
                let capture = if w.capture.is_none() {
                    w.open(xml, file, &name, &e, &mut scan)
                } else {
                    None
                };
                w.stack.push(name);
                if capture.is_some() {
                    w.capture = capture;
                }
            }
            Ok(Event::Empty(e)) => {
                // Only the `order` attribute matters on an empty element: the
                // project loader does not count empty steps/groups/plugins, so
                // counting them here would shift every following index.
                if file == TFile::Config
                    && matches!(
                        lower_name(&e).as_str(),
                        "installsteps" | "optionalfilegroups" | "plugins"
                    )
                    && let Some((value, Some(range))) = find_attr(xml, &e, b"order")
                    && value != "Explicit"
                {
                    scan.order_sites.push(range);
                }
            }
            Ok(Event::End(_)) => {
                if w.capture.as_ref().is_some_and(|c| c.depth == w.stack.len()) {
                    w.finish_capture(&mut scan);
                }
                w.stack.pop();
            }
            Ok(Event::Text(t)) => {
                if let Some(cap) = w.capture.as_mut()
                    && cap.depth == w.stack.len()
                {
                    let text = t
                        .unescape()
                        .map(|s| s.into_owned())
                        .unwrap_or_else(|_| String::from_utf8_lossy(&t).into_owned());
                    cap.parts.push(TextPart {
                        range: offset_in(xml, &t),
                        cdata: false,
                        text,
                    });
                }
            }
            Ok(Event::CData(c)) => {
                if let Some(cap) = w.capture.as_mut()
                    && cap.depth == w.stack.len()
                {
                    cap.parts.push(TextPart {
                        range: offset_in(xml, &c),
                        cdata: true,
                        text: String::from_utf8_lossy(&c).into_owned(),
                    });
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => {
                scan.error = Some(format!("{e} (at byte {})", reader.error_position()));
                break;
            }
        }
    }
    scan
}

/// Extract the translatable units of a FOMOD, `info.xml` first.
///
/// Strings that are empty in the source produce no unit. A document that is
/// not well-formed yields the units found before the error.
pub fn extract_units(info_xml: Option<&str>, config_xml: Option<&str>) -> Vec<TUnit> {
    let mut units = Vec::new();
    for (xml, file) in [(info_xml, TFile::Info), (config_xml, TFile::Config)] {
        let Some(xml) = xml else { continue };
        for site in scan(xml, file).sites {
            units.push(TUnit {
                key: site.key,
                field: site.field,
                source: site.source,
                target: String::new(),
                status: TStatus::Untranslated,
                locked: site.field.locked_by_default(),
                context: site.context,
                note: String::new(),
            });
        }
    }
    units
}

// ---------------------------------------------------------------------------
// Patching
// ---------------------------------------------------------------------------

/// Options of [`apply_units`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PatchOpts {
    /// Rewrite every existing `order` attribute of `installSteps`,
    /// `optionalFileGroups` and `plugins` to `Explicit`, so that translated
    /// names are not re-sorted alphabetically by the mod manager.
    pub force_explicit_order: bool,
}

/// `true` for characters that cannot appear in an XML 1.0 document.
fn is_xml_illegal(c: char) -> bool {
    matches!(c, '\u{0}'..='\u{8}' | '\u{B}' | '\u{C}' | '\u{E}'..='\u{1F}' | '\u{FFFE}' | '\u{FFFF}')
}

/// Escape a translated attribute value. Only the quote that delimits the
/// attribute is escaped, so an apostrophe stays readable in `name="l'option"`.
/// Line breaks and tabs become character references: a literal one would be
/// turned into a space by attribute-value normalisation.
fn escape_attr(value: &str, quote: char) -> String {
    let mut out = String::with_capacity(value.len() + 8);
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' if quote == '"' => out.push_str("&quot;"),
            '\'' if quote == '\'' => out.push_str("&apos;"),
            '\n' => out.push_str("&#10;"),
            '\r' => out.push_str("&#13;"),
            '\t' => out.push_str("&#9;"),
            c => out.push(c),
        }
    }
    out
}

/// Escape translated text content.
fn escape_text(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 8);
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
    out
}

/// Number of bytes of leading and trailing literal whitespace of `raw`.
fn edge_whitespace(raw: &str) -> (usize, usize) {
    let lead = raw.len() - raw.trim_start().len();
    let trail = raw.len() - raw.trim_end().len();
    if lead == raw.len() { (lead, 0) } else { (lead, trail) }
}

/// Compute the edits replacing the text content described by `parts`.
fn text_edits(
    xml: &str,
    key: &str,
    parts: &[TextPart],
    target: &str,
    crlf: bool,
    edits: &mut Vec<(Range<usize>, String)>,
) -> Result<()> {
    let solid: Vec<&TextPart> = parts.iter().filter(|p| !p.text.trim().is_empty()).collect();
    let (Some(first), Some(last)) = (solid.first(), solid.last()) else {
        return Ok(());
    };
    let mut ranges = Vec::with_capacity(solid.len());
    for part in &solid {
        match &part.range {
            Some(r) => ranges.push(r.clone()),
            None => bail!("cannot locate the text of '{key}' in the document"),
        }
    }

    let mut text = normalize_newlines(target);
    if first.cdata {
        if text.contains("]]>") {
            bail!("the translation of '{key}' contains ']]>' and cannot be written in a CDATA section");
        }
    } else {
        text = escape_text(&text);
    }
    if crlf {
        text = text.replace('\n', "\r\n");
    }

    let first_range = &ranges[0];
    let last_range = &ranges[ranges.len() - 1];
    let (lead, _) = edge_whitespace(&xml[first_range.clone()]);
    let (_, trail) = edge_whitespace(&xml[last_range.clone()]);
    let single = std::ptr::eq(*first, *last);

    // The first non-blank piece receives the whole translation; the following
    // ones are emptied. Whitespace around the content is kept.
    let first_end = if single {
        first_range.end - trail
    } else {
        first_range.end
    };
    edits.push((first_range.start + lead..first_end, text));
    for (i, r) in ranges.iter().enumerate().skip(1) {
        let end = if i == ranges.len() - 1 { r.end - trail } else { r.end };
        edits.push((r.start..end, String::new()));
    }
    Ok(())
}

/// Apply translations to one FOMOD file without rewriting it.
///
/// `targets` maps unit keys to translated strings; keys that do not exist in
/// this file are ignored, so the same map can be passed for both files. A
/// target equal to the current content leaves the original bytes untouched.
/// Everything that is not replaced — declaration, comments, processing
/// instructions, whitespace, unknown elements and attributes — is copied
/// byte-for-byte; with no target and `force_explicit_order` off the result is
/// identical to the input.
///
/// Fails when the document is not well-formed, when a target contains a
/// character that XML cannot represent, or when a target containing `]]>` has
/// to be written inside a CDATA section.
pub fn apply_units(xml: &str, file: TFile, targets: &HashMap<String, String>, opts: &PatchOpts) -> Result<String> {
    let scan = scan(xml, file);
    if let Some(err) = &scan.error {
        bail!("XML is not well-formed: {err}");
    }

    let crlf = xml.contains("\r\n");
    let mut edits: Vec<(Range<usize>, String)> = Vec::new();

    for site in &scan.sites {
        let Some(target) = targets.get(&site.key) else { continue };
        if *target == site.source {
            continue;
        }
        if let Some(bad) = target.chars().find(|c| is_xml_illegal(*c)) {
            bail!(
                "the translation of '{}' contains the control character U+{:04X}, which XML cannot represent",
                site.key,
                bad as u32
            );
        }
        match &site.place {
            Place::Attr(Some(range)) => {
                let quote = match xml.as_bytes().get(range.start.wrapping_sub(1)) {
                    Some(b'\'') => '\'',
                    _ => '"',
                };
                edits.push((range.clone(), escape_attr(target, quote)));
            }
            Place::Attr(None) => {
                bail!("cannot locate the attribute of '{}' in the document", site.key)
            }
            Place::Text(parts) => text_edits(xml, &site.key, parts, target, crlf, &mut edits)?,
        }
    }

    if opts.force_explicit_order {
        for range in &scan.order_sites {
            edits.push((range.clone(), "Explicit".to_string()));
        }
    }

    if edits.is_empty() {
        return Ok(xml.to_string());
    }

    edits.sort_by_key(|(r, _)| (r.start, r.end));
    let mut out = String::with_capacity(xml.len() + edits.iter().map(|(_, s)| s.len()).sum::<usize>());
    let mut pos = 0usize;
    for (range, text) in &edits {
        if range.start < pos || range.end < range.start || range.end > xml.len() {
            bail!("internal error: overlapping edits while patching the document");
        }
        out.push_str(&xml[pos..range.start]);
        out.push_str(text);
        pos = range.end;
    }
    out.push_str(&xml[pos..]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A deliberately awkward third-party ModuleConfig: comments, CDATA,
    /// entities in attributes, `moduleDependencies`, nested dependencies,
    /// unknown attributes, a non-empty `<file></file>`, tabs and CRLF.
    const COMPLEX: &str = concat!(
        "<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>\r\n",
        "<!-- Created by hand &amp; proud of it -->\r\n",
        "<config xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://qconsulting.ca/fo3/ModConfig5.0.xsd\">\r\n",
        "\t<moduleName position=\"Left\" colour=\"000000\">Tom &amp; Jerry's Armory</moduleName>\r\n",
        "\t<moduleImage path=\"fomod\\images\\header.png\" showFade='true' height=\"-1\"/>\r\n",
        "\t<moduleDependencies operator=\"And\">\r\n",
        "\t\t<fileDependency file=\"Skyrim.esm\" state=\"Active\"/>\r\n",
        "\t\t<dependencies operator=\"Or\">\r\n",
        "\t\t\t<gameDependency version=\"1.5.97\" />\r\n",
        "\t\t\t<flagDependency flag=\"a&amp;b\" value=\"On\"/>\r\n",
        "\t\t</dependencies>\r\n",
        "\t</moduleDependencies>\r\n",
        "\t<requiredInstallFiles>\r\n",
        "\t\t<file source=\"Core\\Armory.esp\" destination=\"Armory.esp\" alwaysInstall=\"true\" installIfUsable=\"false\"></file>\r\n",
        "\t\t<folder source=\"Core\\meshes\" destination=\"meshes\" priority=\"0\" />\r\n",
        "\t</requiredInstallFiles>\r\n",
        "\t<installSteps order=\"Ascending\">\r\n",
        "\t\t<installStep name=\"Textures &amp; Meshes\">\r\n",
        "\t\t\t<visible operator=\"And\"><flagDependency flag=\"x\" value=\"\"/></visible>\r\n",
        "\t\t\t<optionalFileGroups order='Explicit'>\r\n",
        "\t\t\t\t<group name=\"Resolution\" type=\"SelectExactlyOne\">\r\n",
        "\t\t\t\t\t<plugins order=\"Descending\">\r\n",
        "\t\t\t\t\t\t<plugin name='2K &quot;Lite&quot;'>\r\n",
        "\t\t\t\t\t\t\t<description><![CDATA[Uses <b>2048</b> textures & saves VRAM]]></description>\r\n",
        "\t\t\t\t\t\t\t<image path=\"fomod\\2k.png\"/>\r\n",
        "\t\t\t\t\t\t\t<conditionFlags><flag name=\"res\">2k</flag></conditionFlags>\r\n",
        "\t\t\t\t\t\t\t<typeDescriptor><type name=\"Recommended\"/></typeDescriptor>\r\n",
        "\t\t\t\t\t\t</plugin>\r\n",
        "\t\t\t\t\t\t<plugin name=\"4K\">\r\n",
        "\t\t\t\t\t\t\t<description>\r\n",
        "\t\t\t\t\t\t\t\tSharper &amp; heavier.\r\n",
        "\t\t\t\t\t\t\t\tNeeds Bob&apos;s patch.\r\n",
        "\t\t\t\t\t\t\t</description>\r\n",
        "\t\t\t\t\t\t\t<files><file source=\"4K\\a.dds\" destination=\"textures\\a.dds\"></file></files>\r\n",
        "\t\t\t\t\t\t\t<typeDescriptor>\r\n",
        "\t\t\t\t\t\t\t\t<dependencyType>\r\n",
        "\t\t\t\t\t\t\t\t\t<defaultType name=\"Optional\"/>\r\n",
        "\t\t\t\t\t\t\t\t\t<patterns><pattern><dependencies operator=\"And\"><dependencies operator=\"Or\"><fileDependency file=\"HD.esp\" state=\"Missing\"/></dependencies></dependencies><type name=\"NotUsable\"/></pattern></patterns>\r\n",
        "\t\t\t\t\t\t\t\t</dependencyType>\r\n",
        "\t\t\t\t\t\t\t</typeDescriptor>\r\n",
        "\t\t\t\t\t\t</plugin>\r\n",
        "\t\t\t\t\t\t<plugin name=\"\"><description></description></plugin>\r\n",
        "\t\t\t\t\t</plugins>\r\n",
        "\t\t\t\t</group>\r\n",
        "\t\t\t\t<group name=\"Extras\" type=\"SelectAny\">\r\n",
        "\t\t\t\t\t<plugins>\r\n",
        "\t\t\t\t\t\t<plugin name=\"Cloak\"><description>A cloak</description></plugin>\r\n",
        "\t\t\t\t\t</plugins>\r\n",
        "\t\t\t\t</group>\r\n",
        "\t\t\t</optionalFileGroups>\r\n",
        "\t\t</installStep>\r\n",
        "\t\t<installStep name=\"Patches\">\r\n",
        "\t\t\t<optionalFileGroups>\r\n",
        "\t\t\t\t<group name=\"Compatibility\" type=\"SelectAny\">\r\n",
        "\t\t\t\t\t<plugins order=\"Explicit\">\r\n",
        "\t\t\t\t\t\t<plugin name=\"USSEP\"><description>Patch for USSEP.esp</description></plugin>\r\n",
        "\t\t\t\t\t</plugins>\r\n",
        "\t\t\t\t</group>\r\n",
        "\t\t\t</optionalFileGroups>\r\n",
        "\t\t</installStep>\r\n",
        "\t</installSteps>\r\n",
        "\t<?custom keep=\"me\"?>\r\n",
        "\t<conditionalFileInstalls><patterns><pattern><dependencies><flagDependency flag=\"res\" value=\"2k\"/></dependencies><files><folder source=\"2K\" destination=\"\"/></files></pattern></patterns></conditionalFileInstalls>\r\n",
        "</config>\r\n",
        "<!-- trailing comment -->\r\n",
    );

    const INFO: &str = concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-16\"?>\n",
        "<fomod>\n",
        "  <Name>Tom &amp; Jerry's Armory</Name>\n",
        "  <author>Tom</author>\n",
        "  <Version MachineVersion=\"1.2\">1.2</Version>\n",
        "  <Website>https://example.org/mods/42</Website>\n",
        "  <Description><![CDATA[An armory.]]></Description>\n",
        "  <Groups><element>Armour</element></Groups>\n",
        "</fomod>\n",
    );

    fn no_opts() -> PatchOpts {
        PatchOpts {
            force_explicit_order: false,
        }
    }

    fn targets(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn noop_round_trip_is_byte_identical() {
        let out = apply_units(COMPLEX, TFile::Config, &HashMap::new(), &no_opts()).unwrap();
        assert_eq!(out.as_bytes(), COMPLEX.as_bytes());
        let out = apply_units(INFO, TFile::Info, &HashMap::new(), &no_opts()).unwrap();
        assert_eq!(out.as_bytes(), INFO.as_bytes());
    }

    #[test]
    fn identical_targets_leave_the_document_untouched() {
        // Applying every source as its own translation must not re-escape
        // anything (`&apos;`, `&quot;`, CDATA…).
        let units = extract_units(Some(INFO), Some(COMPLEX));
        let map: HashMap<String, String> = units.iter().map(|u| (u.key.clone(), u.source.clone())).collect();
        assert_eq!(apply_units(COMPLEX, TFile::Config, &map, &no_opts()).unwrap(), COMPLEX);
        assert_eq!(apply_units(INFO, TFile::Info, &map, &no_opts()).unwrap(), INFO);
    }

    #[test]
    fn extraction_yields_keys_sources_and_contexts() {
        let units = extract_units(Some(INFO), Some(COMPLEX));
        let got: Vec<(&str, &str, &str)> = units
            .iter()
            .map(|u| (u.key.as_str(), u.source.as_str(), u.context.as_str()))
            .collect();
        let step = "Textures & Meshes";
        let expected: Vec<(&str, &str, String)> = vec![
            ("info/Name", "Tom & Jerry's Armory", String::new()),
            ("info/Author", "Tom", String::new()),
            ("info/Website", "https://example.org/mods/42", String::new()),
            ("info/Description", "An armory.", String::new()),
            ("config/moduleName", "Tom & Jerry's Armory", String::new()),
            ("config/step[0]/name", step, step.to_string()),
            (
                "config/step[0]/group[0]/name",
                "Resolution",
                format!("{step} › Resolution"),
            ),
            (
                "config/step[0]/group[0]/plugin[0]/name",
                "2K \"Lite\"",
                format!("{step} › Resolution › 2K \"Lite\""),
            ),
            (
                "config/step[0]/group[0]/plugin[0]/description",
                "Uses <b>2048</b> textures & saves VRAM",
                format!("{step} › Resolution › 2K \"Lite\""),
            ),
            (
                "config/step[0]/group[0]/plugin[1]/name",
                "4K",
                format!("{step} › Resolution › 4K"),
            ),
            (
                "config/step[0]/group[0]/plugin[1]/description",
                "Sharper & heavier.\n\t\t\t\t\t\t\t\tNeeds Bob's patch.",
                format!("{step} › Resolution › 4K"),
            ),
            ("config/step[0]/group[1]/name", "Extras", format!("{step} › Extras")),
            (
                "config/step[0]/group[1]/plugin[0]/name",
                "Cloak",
                format!("{step} › Extras › Cloak"),
            ),
            (
                "config/step[0]/group[1]/plugin[0]/description",
                "A cloak",
                format!("{step} › Extras › Cloak"),
            ),
            ("config/step[1]/name", "Patches", "Patches".to_string()),
            (
                "config/step[1]/group[0]/name",
                "Compatibility",
                "Patches › Compatibility".to_string(),
            ),
            (
                "config/step[1]/group[0]/plugin[0]/name",
                "USSEP",
                "Patches › Compatibility › USSEP".to_string(),
            ),
            (
                "config/step[1]/group[0]/plugin[0]/description",
                "Patch for USSEP.esp",
                "Patches › Compatibility › USSEP".to_string(),
            ),
        ];
        let expected: Vec<(&str, &str, &str)> = expected.iter().map(|(k, s, c)| (*k, *s, c.as_str())).collect();
        assert_eq!(got, expected);

        // Fields, default locks and statuses.
        let by_key = |k: &str| units.iter().find(|u| u.key == k).unwrap();
        assert_eq!(by_key("info/Author").field, TField::InfoAuthor);
        assert!(by_key("info/Author").locked);
        assert!(by_key("info/Website").locked);
        assert!(!by_key("info/Name").locked);
        assert_eq!(by_key("config/step[0]/group[1]/name").field, TField::GroupName);
        assert!(
            units
                .iter()
                .all(|u| u.status == TStatus::Untranslated && u.target.is_empty())
        );
    }

    #[test]
    fn indices_match_the_project_loader() {
        let mut model = crate::models::Ximod::default();
        crate::xml::parse_module_config_xml(COMPLEX, &mut model).unwrap();
        for u in extract_units(None, Some(COMPLEX)) {
            let Some(rest) = u.key.strip_prefix("config/step[") else {
                continue;
            };
            let nums: Vec<usize> = rest.split(['[', ']']).filter_map(|p| p.parse().ok()).collect();
            let step = &model.steps[nums[0]];
            match u.field {
                TField::StepName => assert_eq!(step.name, u.source),
                TField::GroupName => assert_eq!(step.plugin_groups[nums[1]].name, u.source),
                TField::PluginName => {
                    assert_eq!(step.plugin_groups[nums[1]].plugins[nums[2]].name, u.source)
                }
                TField::PluginDescription => {
                    let d = &step.plugin_groups[nums[1]].plugins[nums[2]].description;
                    assert_eq!(normalize_newlines(d), u.source);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn apply_replaces_names_and_descriptions_with_escaping() {
        let map = targets(&[
            ("config/moduleName", "L'armurerie de Tom & Jerry"),
            ("config/step[0]/name", "Textures <et> \"maillages\""),
            ("config/step[0]/group[0]/name", "Résolution"),
            ("config/step[0]/group[0]/plugin[0]/name", "2K 'léger'"),
            (
                "config/step[0]/group[0]/plugin[1]/description",
                "Plus net & plus lourd.\nCorrectif <requis>.",
            ),
            (
                "config/step[1]/group[0]/plugin[0]/description",
                "Correctif pour USSEP.esp",
            ),
            ("info/Name", "ignored: belongs to the other file"),
        ]);
        let out = apply_units(COMPLEX, TFile::Config, &map, &no_opts()).unwrap();

        assert!(
            out.contains("<moduleName position=\"Left\" colour=\"000000\">L'armurerie de Tom &amp; Jerry</moduleName>")
        );
        assert!(out.contains("<installStep name=\"Textures &lt;et&gt; &quot;maillages&quot;\">"));
        assert!(out.contains("<group name=\"Résolution\" type=\"SelectExactlyOne\">"));
        // Single-quoted attribute: only the delimiter is escaped.
        assert!(out.contains("<plugin name='2K &apos;léger&apos;'>"));
        // Surrounding whitespace and the file's CRLF line endings are kept.
        assert!(out.contains(
            "<description>\r\n\t\t\t\t\t\t\t\tPlus net &amp; plus lourd.\r\nCorrectif &lt;requis&gt;.\r\n\t\t\t\t\t\t\t</description>"
        ));
        assert!(out.contains("<description>Correctif pour USSEP.esp</description>"));
        // Untouched parts are still there verbatim.
        assert!(out.contains("<!-- Created by hand &amp; proud of it -->"));
        assert!(out.contains("alwaysInstall=\"true\" installIfUsable=\"false\"></file>"));
        assert!(out.contains("<installSteps order=\"Ascending\">"));

        // The patched document reads back as the translations.
        let units = extract_units(None, Some(&out));
        for (key, value) in &map {
            if key.starts_with("config/") {
                let u = units.iter().find(|u| &u.key == key).unwrap();
                assert_eq!(&u.source, value, "{key}");
            }
        }
        // And nothing else changed: translating back restores the original.
        let original = extract_units(None, Some(COMPLEX));
        let back: HashMap<String, String> = original
            .iter()
            .filter(|u| map.contains_key(&u.key))
            .map(|u| (u.key.clone(), u.source.clone()))
            .collect();
        let restored = apply_units(&out, TFile::Config, &back, &no_opts()).unwrap();
        assert_eq!(
            extract_units(None, Some(&restored)),
            original,
            "round trip through a translation must restore every source"
        );
    }

    #[test]
    fn apply_patches_info_xml_case_insensitively() {
        let map = targets(&[("info/Name", "Armurerie"), ("info/Author", "Thomas <T>")]);
        let out = apply_units(INFO, TFile::Info, &map, &no_opts()).unwrap();
        assert!(out.contains("<Name>Armurerie</Name>"));
        assert!(out.contains("<author>Thomas &lt;T&gt;</author>"));
        assert!(out.contains("<Version MachineVersion=\"1.2\">1.2</Version>"));
        assert!(out.contains("<Groups><element>Armour</element></Groups>"));
    }

    #[test]
    fn cdata_is_preserved_and_terminator_refused() {
        let key = "config/step[0]/group[0]/plugin[0]/description";
        let out = apply_units(
            COMPLEX,
            TFile::Config,
            &targets(&[(key, "Textures <b>2048</b> & moins de VRAM")]),
            &no_opts(),
        )
        .unwrap();
        assert!(out.contains("<description><![CDATA[Textures <b>2048</b> & moins de VRAM]]></description>"));

        let err = apply_units(COMPLEX, TFile::Config, &targets(&[(key, "a ]]> b")]), &no_opts()).unwrap_err();
        assert!(err.to_string().contains("]]>"), "{err}");

        // In plain text content the same string is simply escaped.
        let plain = "config/step[1]/group[0]/plugin[0]/description";
        let out = apply_units(COMPLEX, TFile::Config, &targets(&[(plain, "a ]]> b")]), &no_opts()).unwrap();
        assert!(out.contains("<description>a ]]&gt; b</description>"));
    }

    #[test]
    fn whitespace_around_cdata_is_kept() {
        let xml = "<config><installSteps><installStep name=\"S\"><optionalFileGroups><group name=\"G\"><plugins>\
                   <plugin name=\"P\"><description>\n  <![CDATA[ Hello ]]>\n</description></plugin>\
                   </plugins></group></optionalFileGroups></installStep></installSteps></config>";
        let key = "config/step[0]/group[0]/plugin[0]/description";
        let units = extract_units(None, Some(xml));
        assert_eq!(units.iter().find(|u| u.key == key).unwrap().source, "Hello");
        let out = apply_units(xml, TFile::Config, &targets(&[(key, "Bonjour")]), &no_opts()).unwrap();
        assert!(out.contains("<description>\n  <![CDATA[ Bonjour ]]>\n</description>"));
    }

    #[test]
    fn control_characters_are_refused() {
        let err = apply_units(
            COMPLEX,
            TFile::Config,
            &targets(&[("config/step[0]/name", "bad\u{1}name")]),
            &no_opts(),
        )
        .unwrap_err();
        assert!(err.to_string().contains("U+0001"), "{err}");
    }

    #[test]
    fn newline_in_attribute_survives_a_round_trip() {
        let out = apply_units(
            COMPLEX,
            TFile::Config,
            &targets(&[("config/step[0]/name", "Line 1\nLine 2")]),
            &no_opts(),
        )
        .unwrap();
        assert!(out.contains("<installStep name=\"Line 1&#10;Line 2\">"));
        let units = extract_units(None, Some(&out));
        assert_eq!(
            units.iter().find(|u| u.key == "config/step[0]/name").unwrap().source,
            "Line 1\nLine 2"
        );
    }

    #[test]
    fn force_explicit_order_rewrites_only_existing_non_explicit_attributes() {
        let out = apply_units(
            COMPLEX,
            TFile::Config,
            &HashMap::new(),
            &PatchOpts {
                force_explicit_order: true,
            },
        )
        .unwrap();
        assert!(out.contains("<installSteps order=\"Explicit\">"));
        assert!(out.contains("<plugins order=\"Explicit\">"));
        assert!(!out.contains("Ascending"));
        assert!(!out.contains("Descending"));
        // The quoting style of an already explicit attribute is untouched and
        // no attribute is added where there was none.
        assert!(out.contains("<optionalFileGroups order='Explicit'>"));
        assert!(out.contains("\t\t\t\t\t<plugins>\r\n"));
        // Exactly the two attribute values changed.
        let expected = COMPLEX
            .replace("order=\"Ascending\"", "order=\"Explicit\"")
            .replace("order=\"Descending\"", "order=\"Explicit\"");
        assert_eq!(out, expected);
    }

    #[test]
    fn malformed_xml_is_an_error_for_apply_but_not_for_extract() {
        let xml = "<config><moduleName>Mod</moduleName><installSteps><installStep name=\"A\"></oops></config>";
        assert!(apply_units(xml, TFile::Config, &HashMap::new(), &no_opts()).is_err());
        let units = extract_units(None, Some(xml));
        assert_eq!(units.len(), 2);
    }

    #[test]
    fn utf16_le_round_trip() {
        let dir = std::env::temp_dir().join(format!("ximod_patch_utf16_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("info.xml");

        let mut bytes = vec![0xFF, 0xFE];
        for u in INFO.encode_utf16() {
            bytes.extend_from_slice(&u.to_le_bytes());
        }
        std::fs::write(&path, &bytes).unwrap();

        let (xml, enc) = read_xml_with_encoding(&path).unwrap();
        assert_eq!(enc, Encoding::Utf16Le);
        assert_eq!(xml, INFO);
        assert_eq!(fingerprint(&bytes, enc).encoding, "utf-16le");
        assert_eq!(fingerprint(&bytes, enc).bytes, bytes.len() as u64);

        // No-op: the exact original bytes come back.
        let same = apply_units(&xml, TFile::Info, &HashMap::new(), &no_opts()).unwrap();
        assert_eq!(encode(&same, enc), bytes);

        // Translated: still UTF-16 LE with its BOM, and decodes to the patch.
        let patched = apply_units(
            &xml,
            TFile::Info,
            &targets(&[("info/Name", "Armurerie éèà")]),
            &no_opts(),
        )
        .unwrap();
        let out = encode(&patched, enc);
        assert_eq!(&out[..2], &[0xFF, 0xFE]);
        let (decoded, enc2) = decode_xml_bytes(&out);
        assert_eq!(enc2, Encoding::Utf16Le);
        assert_eq!(decoded, patched);
        assert!(decoded.contains("<Name>Armurerie éèà</Name>"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn encoding_detection_and_fallbacks() {
        // UTF-8 BOM.
        let (s, e) = decode_xml_bytes(b"\xEF\xBB\xBF<a/>");
        assert_eq!((s.as_str(), e), ("<a/>", Encoding::Utf8Bom));
        assert_eq!(encode(&s, e), b"\xEF\xBB\xBF<a/>");

        // UTF-16 BE with BOM.
        let mut be = vec![0xFE, 0xFF];
        for u in "<a>é</a>".encode_utf16() {
            be.extend_from_slice(&u.to_be_bytes());
        }
        let (s, e) = decode_xml_bytes(&be);
        assert_eq!((s.as_str(), e), ("<a>é</a>", Encoding::Utf16Be));
        assert_eq!(encode(&s, e), be);

        // UTF-16 LE without BOM (heuristic).
        let le: Vec<u8> = "<config/>".encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
        let (s, e) = decode_xml_bytes(&le);
        assert_eq!((s.as_str(), e), ("<config/>", Encoding::Utf16Le));

        // Windows-1252 bytes in an otherwise ASCII file: never an error.
        let (s, e) = decode_xml_bytes(b"<a>caf\xE9 \x93x\x94 \x80</a>");
        assert_eq!(e, Encoding::Utf8);
        assert_eq!(s, "<a>café \u{201C}x\u{201D} \u{20AC}</a>");

        // Valid UTF-8 mixed with a stray legacy byte keeps the valid part.
        let (s, _) = decode_xml_bytes(
            "<a>é\u{4E2D}"
                .as_bytes()
                .iter()
                .copied()
                .chain([0xE8])
                .collect::<Vec<_>>()
                .as_slice(),
        );
        assert_eq!(s, "<a>é\u{4E2D}è");

        // Empty and tiny inputs.
        assert_eq!(decode_xml_bytes(b"").0, "");
        assert_eq!(decode_xml_bytes(b"<").0, "<");
    }

    #[test]
    fn legacy_declaration_is_fixed_only_when_needed() {
        let ascii = "<?xml version=\"1.0\" encoding=\"windows-1252\"?><fomod><Name>Mod</Name></fomod>";
        // Pure ASCII: identical bytes in both charsets, nothing to fix.
        assert_eq!(encode(ascii, Encoding::Utf8), ascii.as_bytes());

        let patched = apply_units(ascii, TFile::Info, &targets(&[("info/Name", "Modé")]), &no_opts()).unwrap();
        let out = String::from_utf8(encode(&patched, Encoding::Utf8)).unwrap();
        assert_eq!(
            out,
            "<?xml version=\"1.0\" encoding=\"utf-8\"?><fomod><Name>Modé</Name></fomod>"
        );

        // A UTF-8 declaration (any case) and UTF-16 output are left alone.
        let utf8 = "<?xml version='1.0' encoding='UTF-8'?><a>é</a>";
        assert_eq!(fix_declared_encoding(utf8, Encoding::Utf8Bom), utf8);
        let utf16 = "<?xml version=\"1.0\" encoding=\"utf-16\"?><a>é</a>";
        assert_eq!(fix_declared_encoding(utf16, Encoding::Utf16Le), utf16);
        // No declaration at all.
        assert_eq!(fix_declared_encoding("<a>é</a>", Encoding::Utf8), "<a>é</a>");
    }

    #[test]
    fn locate_fomod_files_ignores_case() {
        let dir = std::env::temp_dir().join(format!("ximod_patch_locate_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Fomod")).unwrap();
        std::fs::write(dir.join("Fomod/Info.xml"), "<fomod/>").unwrap();
        std::fs::write(dir.join("Fomod/moduleconfig.XML"), "<config/>").unwrap();
        let found = locate_fomod_files(&dir);
        assert!(found.info.as_deref().is_some_and(|p| p.is_file()));
        assert!(found.config.as_deref().is_some_and(|p| p.is_file()));
        let none = locate_fomod_files(&dir.join("missing"));
        assert!(none.info.is_none() && none.config.is_none());
        assert!(none.dir.ends_with("fomod"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
