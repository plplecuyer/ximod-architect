//! Bethesda plugin header reading — .esp / .esm / .esl (V2 roadmap, priority 5).
//!
//! Reading the `TES4` record header of a plugin lets XIMOD auto-suggest file
//! dependencies (required masters) and sanity-check a project against its target
//! game. It also exposes the internal author/description and the ESL ("light")
//! flag.
//!
//! Format (Oblivion TES4 and Skyrim/Fallout/Starfield): a plugin begins with a
//! `TES4` record. The record header is 20 bytes on Oblivion and 24 bytes on the
//! later engines; we auto-detect the size by locating the first `HEDR`
//! subrecord. Subrecords are `Type(4) + Size(u16 LE) + data`. We read `MAST`
//! (one required master each, NUL-terminated), `CNAM` (author) and `SNAM`
//! (description). The ESL flag is bit `0x200` of the record flags. All integers
//! are little-endian. Morrowind (`TES3`) uses a different layout and is not
//! parsed here.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::{Context, Result, bail};

/// The plugin flavor, derived from the file extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PluginKind {
    /// Standard plugin (`.esp`).
    #[default]
    Esp,
    /// Master file (`.esm`).
    Esm,
    /// Light master (`.esl`).
    Esl,
}

impl PluginKind {
    /// Classify by file extension (case-insensitive). `None` if not a plugin.
    pub fn from_path(path: &Path) -> Option<Self> {
        match path
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("esp") => Some(Self::Esp),
            Some("esm") => Some(Self::Esm),
            Some("esl") => Some(Self::Esl),
            _ => None,
        }
    }
}

/// The parsed, high-level plugin header.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PluginHeader {
    /// File flavor from the extension.
    pub kind: Option<PluginKind>,
    /// Master files this plugin depends on, in load order.
    pub masters: Vec<String>,
    /// Author (`CNAM`), if present.
    pub author: Option<String>,
    /// Description (`SNAM`), if present.
    pub description: Option<String>,
    /// Whether the ESL (light) flag (0x200) is set in the TES4 record flags.
    pub light: bool,
}

// `PluginKind` has no sensible Default, but `Option<PluginKind>` does (None).

fn u16_le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32_le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

/// Decode a NUL-terminated Windows-1252-ish byte string (ASCII-safe fallback).
fn zstring(data: &[u8]) -> String {
    let end = data.iter().position(|&c| c == 0).unwrap_or(data.len());
    data[..end].iter().map(|&c| c as char).collect()
}

/// Read and parse the header of a Bethesda plugin file.
pub fn read_plugin_header(path: &Path) -> Result<PluginHeader> {
    let kind = PluginKind::from_path(path);
    if kind.is_none() {
        bail!("not a plugin file (expected .esp/.esm/.esl): {}", path.display());
    }

    let mut f = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    // The TES4 record header is at most 24 bytes; read it first.
    let mut head = [0u8; 24];
    f.read_exact(&mut head)
        .with_context(|| "reading the plugin record header")?;
    if &head[0..4] != b"TES4" {
        bail!("not a TES4 plugin (Morrowind TES3 is unsupported): {}", path.display());
    }
    let data_size = u32_le(&head, 4) as usize;
    let flags = u32_le(&head, 8);
    let light = flags & 0x0000_0200 != 0;

    // Read the record data (bounded) that follows the 24-byte prefix we already
    // consumed, plus a little slack in case the header is only 20 bytes (then 4
    // of our 24 prefix bytes actually belong to the data).
    let mut data = Vec::new();
    f.take(data_size as u64).read_to_end(&mut data)?;
    // Reassemble the bytes from the start of the (possible) subrecord area.
    // Detect header size: subrecords start where "HEDR" appears (offset 20 or 24
    // from the file start).
    let mut full = Vec::with_capacity(24 + data.len());
    full.extend_from_slice(&head);
    full.extend_from_slice(&data);

    let sub_start = if full.len() >= 24 && &full[20..24] == b"HEDR" {
        20
    } else if full.len() >= 28 && &full[24..28] == b"HEDR" {
        24
    } else {
        // Unknown header; still return what we know (kind + light).
        return Ok(PluginHeader {
            kind,
            light,
            ..Default::default()
        });
    };

    let mut hdr = PluginHeader {
        kind,
        light,
        ..Default::default()
    };
    // Subrecords occupy exactly `data_size` bytes from `sub_start`.
    let area_end = (sub_start + data_size).min(full.len());
    let mut o = sub_start;
    while o + 6 <= area_end {
        let ty = &full[o..o + 4];
        let size = u16_le(&full, o + 4) as usize;
        let body_start = o + 6;
        let body_end = (body_start + size).min(area_end);
        let body = &full[body_start..body_end];
        match ty {
            b"MAST" => hdr.masters.push(zstring(body)),
            b"CNAM" => hdr.author = Some(zstring(body)),
            b"SNAM" => hdr.description = Some(zstring(body)),
            _ => {}
        }
        o = body_end; // always advances by at least 6 (the subrecord header)
    }
    Ok(hdr)
}

// ---------------------------------------------------------------------------
// Light plugin (ESL) eligibility
// ---------------------------------------------------------------------------

/// Classic light-plugin limit: 2048 new records (Skyrim SE before 1.6.1130,
/// Fallout 4), whose object index must lie in `0x800..=0xFFF`.
pub const ESL_LIMIT_CLASSIC: u32 = 0x800;

/// Extended light-plugin limit: 4096 new records with the object index in
/// `0x000..=0xFFF` (Starfield, Skyrim SE 1.6.1130 and later).
pub const ESL_LIMIT_EXTENDED: u32 = 0x1000;

/// Highest object index (low 24 bits of a FormID) a light plugin may use.
const ESL_MAX_INDEX: u32 = 0xFFF;

/// The light-plugin rules of a game: how many records a plugin may add and
/// where their object indices must lie.
///
/// The limit is 2048 for the classic engines and 4096 for Starfield and the
/// post-1.6.1130 Skyrim SE runtime. XIMOD cannot know which runtime the
/// players use, so it keeps the conservative classic value unless the game
/// entry of `Categories.json` says otherwise (`"eslLimit": 4096`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EslLimits {
    /// Maximum number of new records.
    pub limit: u32,
}

impl Default for EslLimits {
    fn default() -> Self {
        Self::new(ESL_LIMIT_CLASSIC)
    }
}

impl EslLimits {
    /// Rules with the given record limit (`2048` or `4096`; anything else is
    /// clamped to the nearest known rule).
    pub fn new(limit: u32) -> Self {
        let limit = if limit > ESL_LIMIT_CLASSIC {
            ESL_LIMIT_EXTENDED
        } else {
            ESL_LIMIT_CLASSIC
        };
        Self { limit }
    }

    /// The rules of a game id, as described by `Categories.json`
    /// (`"eslLimit"`, default 2048).
    pub fn for_game(games: &crate::games::GamesData, game_id: &str) -> Self {
        Self::new(games.esl_limit_for(game_id))
    }

    /// Lowest allowed object index: `0x800` under the classic rule (the lower
    /// indices are reserved), `0` under the extended one.
    pub fn min_index(&self) -> u32 {
        if self.limit <= ESL_LIMIT_CLASSIC {
            ESL_LIMIT_CLASSIC
        } else {
            0
        }
    }

    /// Whether an object index may be used by a light plugin.
    pub fn index_allowed(&self, object_index: u32) -> bool {
        (self.min_index()..=ESL_MAX_INDEX).contains(&object_index)
    }
}

/// What [`count_new_records`] found out about a plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EslReport {
    /// Records whose FormID belongs to this plugin (not overrides of a master).
    pub new_forms: u32,
    /// Of those, records whose object index lies outside the light range.
    pub out_of_range: u32,
    /// File flavor from the extension.
    pub kind: PluginKind,
    /// Whether the ESL flag is set in the TES4 header.
    pub light_flag: bool,
    /// `new_forms <= limit` and every new FormID is in the light range.
    pub eligible: bool,
    /// The record limit the verdict was computed against.
    pub limit: u32,
}

/// Flag of a compressed record (its data is zlib-compressed on disk; the
/// header's size is still the on-disk size, so it is skipped like any other).
#[allow(dead_code)]
const RECORD_FLAG_COMPRESSED: u32 = 0x0004_0000;

/// Walk the records of a plugin and count the ones it introduces (FormID
/// load-order index equal to the number of masters, i.e. its own index), to
/// decide whether it could be, or rightly is, a light plugin.
///
/// Layout (Skyrim SE, Fallout 4, Starfield; Oblivion's 20-byte headers are
/// detected from the TES4 record): a record is `Type(4) DataSize(u32)
/// Flags(u32) FormID(u32) …` followed by `DataSize` bytes; a group is
/// `"GRUP"(4) GroupSize(u32, header included) Label(4) GroupType(i32) …`
/// followed by its records and sub-groups. Groups nest, but their content is
/// contiguous, so a sequential walk that skips group headers and record
/// bodies visits every record of the file without decompressing anything.
/// Only headers are read; record bodies are seeked over.
pub fn count_new_records(path: &Path, limits: EslLimits) -> Result<EslReport> {
    let Some(kind) = PluginKind::from_path(path) else {
        bail!("not a plugin file (expected .esp/.esm/.esl): {}", path.display());
    };
    let mut f = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let len = f.metadata()?.len();

    // TES4 record: header size (20 or 24), flags, masters.
    let mut head = [0u8; 24];
    f.read_exact(&mut head).context("reading the plugin record header")?;
    if &head[0..4] != b"TES4" {
        bail!("not a TES4 plugin (Morrowind TES3 is unsupported): {}", path.display());
    }
    let tes4_size = u32_le(&head, 4) as u64;
    let light_flag = u32_le(&head, 8) & 0x0000_0200 != 0;
    let header_size: u64 = if &head[20..24] == b"HEDR" { 20 } else { 24 };
    let masters = read_plugin_header(path)?.masters.len() as u32;
    let own_index = masters & 0xFF;

    let mut pos = header_size + tes4_size;
    let mut new_forms: u32 = 0;
    let mut out_of_range: u32 = 0;
    let mut hdr = vec![0u8; header_size as usize];
    while pos + header_size <= len {
        f.seek(SeekFrom::Start(pos))?;
        f.read_exact(&mut hdr)?;
        let size = u32_le(&hdr, 4) as u64;
        if &hdr[0..4] == b"GRUP" {
            // Skip the header only: the content is walked sequentially.
            pos += header_size;
            if size < header_size {
                bail!("corrupt group header at offset {pos}");
            }
            continue;
        }
        let form_id = u32_le(&hdr, 12);
        if form_id >> 24 == own_index {
            new_forms = new_forms.saturating_add(1);
            if !limits.index_allowed(form_id & 0x00FF_FFFF) {
                out_of_range = out_of_range.saturating_add(1);
            }
        }
        pos += header_size + size;
    }

    Ok(EslReport {
        new_forms,
        out_of_range,
        kind,
        light_flag,
        eligible: new_forms <= limits.limit && out_of_range == 0,
        limit: limits.limit,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::io::Write;
    use std::path::Path;

    /// Append a subrecord.
    fn sub(out: &mut Vec<u8>, ty: &[u8; 4], body: &[u8]) {
        out.extend_from_slice(ty);
        out.extend_from_slice(&(body.len() as u16).to_le_bytes());
        out.extend_from_slice(body);
    }

    /// A 24-byte record header followed by `data`.
    fn record(out: &mut Vec<u8>, ty: &[u8; 4], flags: u32, form_id: u32, data: &[u8]) {
        out.extend_from_slice(ty);
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&form_id.to_le_bytes());
        out.extend_from_slice(&[0u8; 8]);
        out.extend_from_slice(data);
    }

    /// A GRUP (24-byte header, size includes the header) holding `content`.
    fn group(out: &mut Vec<u8>, label: &[u8; 4], content: &[u8]) {
        out.extend_from_slice(b"GRUP");
        out.extend_from_slice(&((content.len() + 24) as u32).to_le_bytes());
        out.extend_from_slice(label);
        out.extend_from_slice(&0i32.to_le_bytes());
        out.extend_from_slice(&[0u8; 8]);
        out.extend_from_slice(content);
    }

    /// Build a minimal modern plugin: TES4 (flags, masters) + the given
    /// top-level groups/records.
    pub(crate) fn build_plugin(flags: u32, masters: &[&str], body: &[u8]) -> Vec<u8> {
        let mut data = Vec::new();
        sub(&mut data, b"HEDR", &[0; 12]);
        sub(&mut data, b"CNAM", b"Tester\0");
        for m in masters {
            let mut z = m.as_bytes().to_vec();
            z.push(0);
            sub(&mut data, b"MAST", &z);
            sub(&mut data, b"DATA", &[0; 8]);
        }
        let mut out = Vec::new();
        record(&mut out, b"TES4", flags, 0, &data);
        out.extend_from_slice(body);
        out
    }

    /// Write bytes to a scratch plugin file and return its path.
    pub(crate) fn write_plugin(name: &str, bytes: &[u8]) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod_esl_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join(name);
        std::fs::File::create(&p).unwrap().write_all(bytes).unwrap();
        p
    }

    #[test]
    fn classifies_by_extension() {
        assert_eq!(PluginKind::from_path(Path::new("a.ESP")), Some(PluginKind::Esp));
        assert_eq!(PluginKind::from_path(Path::new("a.esm")), Some(PluginKind::Esm));
        assert_eq!(PluginKind::from_path(Path::new("a.esl")), Some(PluginKind::Esl));
        assert_eq!(PluginKind::from_path(Path::new("a.txt")), None);
    }

    #[test]
    fn non_plugin_errors() {
        assert!(read_plugin_header(Path::new("readme.txt")).is_err());
        assert!(count_new_records(Path::new("readme.txt"), EslLimits::default()).is_err());
    }

    #[test]
    fn esl_limits_rules() {
        let classic = EslLimits::default();
        assert_eq!(classic.limit, 2048);
        assert_eq!(classic.min_index(), 0x800);
        assert!(classic.index_allowed(0x800) && classic.index_allowed(0xFFF));
        assert!(!classic.index_allowed(0x7FF) && !classic.index_allowed(0x1000));
        let extended = EslLimits::new(4096);
        assert_eq!(extended.min_index(), 0);
        assert!(extended.index_allowed(0) && extended.index_allowed(0xFFF));
        assert!(!extended.index_allowed(0x1000));
        assert_eq!(EslLimits::new(3000).limit, 4096, "unknown values snap to a known rule");
        assert_eq!(EslLimits::new(100).limit, 2048);
    }

    /// New records are the ones whose load-order index is the plugin's own
    /// (two masters → index 2); overrides of a master are not counted, nested
    /// groups are walked, and compressed records are skipped by size.
    #[test]
    fn counts_new_records_through_nested_groups() {
        let mut inner = Vec::new();
        record(&mut inner, b"WEAP", 0, 0x02000800, &[1, 2, 3]);
        record(&mut inner, b"WEAP", RECORD_FLAG_COMPRESSED, 0x02000801, &[9; 40]);
        record(&mut inner, b"WEAP", 0, 0x00012345, &[]); // override of Skyrim.esm
        let mut top = Vec::new();
        record(&mut top, b"ARMO", 0, 0x02000FFF, &[7; 5]);
        group(&mut top, b"WEAP", &inner);
        let mut body = Vec::new();
        group(&mut body, b"ARMO", &top);
        record(&mut body, b"KYWD", 0, 0x01000010, &[]); // override of Update.esm
        let bytes = build_plugin(0, &["Skyrim.esm", "Update.esm"], &body);
        let p = write_plugin("Nested.esp", &bytes);

        let r = count_new_records(&p, EslLimits::default()).unwrap();
        assert_eq!(r.new_forms, 3);
        assert_eq!(r.out_of_range, 0);
        assert_eq!(r.kind, PluginKind::Esp);
        assert!(!r.light_flag);
        assert!(r.eligible);
        assert_eq!(r.limit, 2048);
    }

    #[test]
    fn eligibility_follows_the_index_range_and_the_limit() {
        // An object index below 0x800 breaks the classic rule, not the extended one.
        let mut body = Vec::new();
        record(&mut body, b"MISC", 0, 0x00000123, &[]);
        let bytes = build_plugin(0x200, &[], &body);
        let p = write_plugin("Low.esp", &bytes);
        let classic = count_new_records(&p, EslLimits::default()).unwrap();
        assert_eq!((classic.new_forms, classic.out_of_range), (1, 1));
        assert!(!classic.eligible && classic.light_flag);
        let extended = count_new_records(&p, EslLimits::new(4096)).unwrap();
        assert!(extended.eligible);

        // More records than the limit: not eligible even when all are in range.
        let mut body = Vec::new();
        for i in 0..2049u32 {
            record(&mut body, b"MISC", 0, 0x00000800 + (i % 0x800), &[]);
        }
        let bytes = build_plugin(0, &[], &body);
        let p = write_plugin("Big.esp", &bytes);
        let r = count_new_records(&p, EslLimits::default()).unwrap();
        assert_eq!(r.new_forms, 2049);
        assert!(!r.eligible);
        assert!(count_new_records(&p, EslLimits::new(4096)).unwrap().eligible);
    }

    /// Build a minimal modern (24-byte header) TES4 record with HEDR, CNAM and
    /// two MAST subrecords, and parse it back.
    #[test]
    fn parses_masters_author_and_esl() {
        let mut data = Vec::new();
        sub(&mut data, b"HEDR", &[0; 12]);
        sub(&mut data, b"CNAM", b"Wenderer\0");
        sub(&mut data, b"MAST", b"Skyrim.esm\0");
        sub(&mut data, b"MAST", b"Update.esm\0");

        let mut rec = Vec::new();
        rec.extend_from_slice(b"TES4");
        rec.extend_from_slice(&(data.len() as u32).to_le_bytes()); // data size
        rec.extend_from_slice(&0x0000_0200u32.to_le_bytes()); // flags: ESL set
        rec.extend_from_slice(&[0u8; 12]); // formid(4)+timestamp/vc/version/unk(8) = modern 24-byte header
        rec.extend_from_slice(&data);

        let dir = std::env::temp_dir().join(format!("ximod_esp_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join("Test.esp");
        std::fs::File::create(&p).unwrap().write_all(&rec).unwrap();

        let h = read_plugin_header(&p).unwrap();
        assert_eq!(h.kind, Some(PluginKind::Esp));
        assert!(h.light);
        assert_eq!(h.author.as_deref(), Some("Wenderer"));
        assert_eq!(h.masters, vec!["Skyrim.esm".to_string(), "Update.esm".to_string()]);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
