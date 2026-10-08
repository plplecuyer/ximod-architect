//! Readers for the Bethesda archive containers (V2 roadmap, point 10):
//! **BSA** (Oblivion / Fallout 3 / New Vegas / Skyrim LE, Skyrim SE) and
//! **BA2** (Fallout 4, Starfield; general and texture archives).
//!
//! Only the *directory* of an archive is read — the header, the records and
//! the name table — never the file data, so listing a multi-gigabyte texture
//! archive costs a few seeks. Compressed entries are reported by name and
//! size without being decompressed (no zlib / LZ4 here; the names are what
//! the conflict scan, the simulator and the inspector need).
//!
//! Every read goes through [`Reader`], which turns a short file into
//! [`ArchiveError::Truncated`] instead of a panic, and the counts of a corrupt
//! header never drive an allocation.
//!
//! Language-neutral like the other `models` modules.

use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

/// The container variant of an archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    /// `BSA` version 103 (Oblivion). Morrowind's `.bsa` is a different,
    /// unsupported container.
    Bsa103,
    /// `BSA` version 104 (Fallout 3 / New Vegas, Skyrim LE).
    Bsa104,
    /// `BSA` version 105 (Skyrim Special Edition).
    Bsa105,
    /// `BTDX` general archive (loose files).
    Ba2Gnrl { version: u32 },
    /// `BTDX` texture archive (DDS split into mip chunks).
    Ba2Dx10 { version: u32 },
}

impl ArchiveFormat {
    /// Short technical label (`BSA v105`, `BA2 GNRL v1`…), for reports.
    pub fn label(self) -> String {
        match self {
            ArchiveFormat::Bsa103 => "BSA v103".to_string(),
            ArchiveFormat::Bsa104 => "BSA v104".to_string(),
            ArchiveFormat::Bsa105 => "BSA v105".to_string(),
            ArchiveFormat::Ba2Gnrl { version } => format!("BA2 GNRL v{version}"),
            ArchiveFormat::Ba2Dx10 { version } => format!("BA2 DX10 v{version}"),
        }
    }
}

/// One file inside an archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveEntry {
    /// Path inside the archive, lowercased, with `/` separators.
    pub path: String,
    /// Uncompressed size when the container records it, else the packed size
    /// (BSA stores the original size in front of the compressed data, which
    /// we do not read).
    pub size: u64,
    pub compressed: bool,
}

/// The directory of an archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveListing {
    pub format: ArchiveFormat,
    pub entries: Vec<ArchiveEntry>,
}

impl ArchiveListing {
    /// Sum of the entry sizes.
    pub fn total_size(&self) -> u64 {
        self.entries.iter().map(|e| e.size).sum()
    }
}

/// Why an archive could not be listed.
#[derive(Debug)]
pub enum ArchiveError {
    Io(io::Error),
    /// The file ends before the directory does.
    Truncated,
    /// Not a BSA / BA2 (the magic bytes say so), or a variant without a
    /// readable directory (Morrowind BSA, a BA2 without a name table).
    Unsupported(String),
    /// A known container with a version this reader does not handle.
    UnsupportedVersion {
        container: &'static str,
        version: u32,
    },
}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchiveError::Io(e) => write!(f, "{e}"),
            ArchiveError::Truncated => write!(
                f,
                "the archive is truncated (its directory runs past the end of the file)"
            ),
            ArchiveError::Unsupported(what) => write!(f, "{what}"),
            ArchiveError::UnsupportedVersion { container, version } => {
                write!(f, "unsupported {container} version {version}")
            }
        }
    }
}

impl std::error::Error for ArchiveError {}

impl From<io::Error> for ArchiveError {
    fn from(e: io::Error) -> Self {
        if e.kind() == io::ErrorKind::UnexpectedEof {
            ArchiveError::Truncated
        } else {
            ArchiveError::Io(e)
        }
    }
}

pub type Result<T> = std::result::Result<T, ArchiveError>;

/// Whether `path` names a Bethesda archive, by extension (`.bsa` / `.ba2`,
/// any case).
pub fn is_bethesda_archive(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("bsa") || e.eq_ignore_ascii_case("ba2"))
}

/// List the directory of the archive at `path`.
pub fn list_archive(path: &Path) -> Result<ArchiveListing> {
    let file = File::open(path)?;
    parse_archive(BufReader::new(file))
}

/// List the directory of an archive from any seekable stream (the file, or
/// an in-memory buffer in the tests).
pub fn parse_archive<R: Read + Seek>(stream: R) -> Result<ArchiveListing> {
    let mut r = Reader::new(stream);
    let magic = r.bytes4()?;
    match &magic {
        b"BSA\0" => parse_bsa(&mut r),
        b"BTDX" => parse_ba2(&mut r),
        [0x00, 0x01, 0x00, 0x00] => Err(ArchiveError::Unsupported(
            "Morrowind BSA archives (TES3 format) are not supported".to_string(),
        )),
        _ => Err(ArchiveError::Unsupported(
            "not a Bethesda archive (expected a BSA or BTDX header)".to_string(),
        )),
    }
}

// --------------------------------------------------------------------------- //
// Bounds-checked reader
// --------------------------------------------------------------------------- //

/// Little-endian reader over a seekable stream; every short read is a
/// [`ArchiveError::Truncated`].
struct Reader<R> {
    inner: R,
    /// Length of the stream, for sanity checks on offsets.
    len: u64,
}

/// Longest string the readers accept (a BSA bzstring is one byte long, a BA2
/// name is `u16`-prefixed: 65 535 is the natural bound).
const MAX_NAME: usize = u16::MAX as usize;

impl<R: Read + Seek> Reader<R> {
    fn new(mut inner: R) -> Self {
        let len = inner.seek(SeekFrom::End(0)).unwrap_or(0);
        let _ = inner.seek(SeekFrom::Start(0));
        Self { inner, len }
    }

    fn seek(&mut self, pos: u64) -> Result<()> {
        if pos > self.len {
            return Err(ArchiveError::Truncated);
        }
        self.inner.seek(SeekFrom::Start(pos))?;
        Ok(())
    }

    fn position(&mut self) -> Result<u64> {
        Ok(self.inner.stream_position()?)
    }

    fn bytes4(&mut self) -> Result<[u8; 4]> {
        let mut b = [0u8; 4];
        self.inner.read_exact(&mut b)?;
        Ok(b)
    }

    fn u8(&mut self) -> Result<u8> {
        let mut b = [0u8; 1];
        self.inner.read_exact(&mut b)?;
        Ok(b[0])
    }

    fn u16(&mut self) -> Result<u16> {
        let mut b = [0u8; 2];
        self.inner.read_exact(&mut b)?;
        Ok(u16::from_le_bytes(b))
    }

    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.bytes4()?))
    }

    fn u64(&mut self) -> Result<u64> {
        let mut b = [0u8; 8];
        self.inner.read_exact(&mut b)?;
        Ok(u64::from_le_bytes(b))
    }

    fn skip(&mut self, n: u64) -> Result<()> {
        let pos = self.position()?;
        let target = pos.checked_add(n).ok_or(ArchiveError::Truncated)?;
        self.seek(target)
    }

    /// `n` raw bytes (`n` is bounded by the caller).
    fn bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut v = vec![0u8; n];
        self.inner.read_exact(&mut v)?;
        Ok(v)
    }

    /// A BSA *bzstring*: one length byte (null included) then the bytes.
    fn bzstring(&mut self) -> Result<String> {
        let n = self.u8()? as usize;
        let raw = self.bytes(n)?;
        Ok(lossy_name(&raw))
    }

    /// A BSA zero-terminated string.
    fn zstring(&mut self) -> Result<String> {
        let mut raw = Vec::new();
        loop {
            let b = self.u8()?;
            if b == 0 {
                break;
            }
            if raw.len() >= MAX_NAME {
                return Err(ArchiveError::Unsupported(
                    "a file name is longer than 65535 bytes".to_string(),
                ));
            }
            raw.push(b);
        }
        Ok(lossy_name(&raw))
    }

    /// A BA2 name-table string: `u16` length then the bytes.
    fn u16_string(&mut self) -> Result<String> {
        let n = self.u16()? as usize;
        let raw = self.bytes(n)?;
        Ok(lossy_name(&raw))
    }
}

/// Decode a stored name: trailing NULs dropped, invalid UTF-8 replaced.
fn lossy_name(raw: &[u8]) -> String {
    let end = raw.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(0);
    String::from_utf8_lossy(&raw[..end]).into_owned()
}

/// Normalize an archive path: lowercase, `/` separators, no leading `/`.
fn norm_entry_path(raw: &str) -> String {
    raw.replace('\\', "/").trim_start_matches('/').to_lowercase()
}

/// `Vec::with_capacity` that trusts an untrusted count only up to a point.
fn capacity_for(count: u32) -> usize {
    (count as usize).min(1 << 16)
}

// --------------------------------------------------------------------------- //
// BSA (versions 103 / 104 / 105)
// --------------------------------------------------------------------------- //

/// Archive flag: folder records are followed by the folder names.
const BSA_FLAG_DIR_NAMES: u32 = 0x1;
/// Archive flag: the file name block is present.
const BSA_FLAG_FILE_NAMES: u32 = 0x2;
/// Archive flag: files are compressed unless their size toggles it.
const BSA_FLAG_COMPRESSED: u32 = 0x4;
/// Size-field bit toggling the archive-level compression flag for one file.
const BSA_SIZE_COMPRESSION_TOGGLE: u32 = 1 << 30;
/// Size-field bits holding the actual size.
const BSA_SIZE_MASK: u32 = (1 << 30) - 1;

fn parse_bsa<R: Read + Seek>(r: &mut Reader<R>) -> Result<ArchiveListing> {
    let version = r.u32()?;
    let format = match version {
        103 => ArchiveFormat::Bsa103,
        104 => ArchiveFormat::Bsa104,
        105 => ArchiveFormat::Bsa105,
        other => {
            return Err(ArchiveError::UnsupportedVersion {
                container: "BSA",
                version: other,
            });
        }
    };
    let folder_offset = r.u32()? as u64;
    let flags = r.u32()?;
    let folder_count = r.u32()?;
    let file_count = r.u32()?;
    let _total_folder_name_len = r.u32()?;
    let _total_file_name_len = r.u32()?;
    let _file_flags = r.u32()?;

    let has_dir_names = flags & BSA_FLAG_DIR_NAMES != 0;
    let has_file_names = flags & BSA_FLAG_FILE_NAMES != 0;
    let default_compressed = flags & BSA_FLAG_COMPRESSED != 0;

    // Folder records: 16 bytes (hash, count, offset) up to v104; v105 widens
    // the offset to 64 bits with 4 bytes of padding before it.
    r.seek(folder_offset)?;
    let mut folders: Vec<(u64, u32)> = Vec::with_capacity(capacity_for(folder_count));
    for _ in 0..folder_count {
        let hash = r.u64()?;
        let count = r.u32()?;
        if version == 105 {
            let _padding = r.u32()?;
            let _offset = r.u64()?;
        } else {
            let _offset = r.u32()?;
        }
        folders.push((hash, count));
    }

    // File record blocks, one per folder, in folder order: the folder name
    // (when stored) then `count` records of (hash, size, offset).
    struct Raw {
        folder: usize,
        hash: u64,
        size: u32,
    }
    let mut raws: Vec<Raw> = Vec::with_capacity(capacity_for(file_count));
    let mut folder_names: Vec<String> = Vec::with_capacity(folders.len());
    for (fi, (_, count)) in folders.iter().enumerate() {
        let name = if has_dir_names { r.bzstring()? } else { String::new() };
        folder_names.push(name);
        for _ in 0..*count {
            let hash = r.u64()?;
            let size = r.u32()?;
            let _offset = r.u32()?;
            raws.push(Raw { folder: fi, hash, size });
        }
    }
    // The folder records are what the game follows; the header's file count
    // only sizes the name block, so a disagreement is tolerated.

    // File name block: one zero-terminated name per file, in record order.
    let mut entries = Vec::with_capacity(raws.len());
    for raw in &raws {
        let file_name = if has_file_names {
            r.zstring()?
        } else {
            format!("{:016x}", raw.hash)
        };
        let folder = &folder_names[raw.folder];
        let full = if folder.is_empty() {
            file_name
        } else {
            format!("{folder}\\{file_name}")
        };
        let compressed = default_compressed ^ (raw.size & BSA_SIZE_COMPRESSION_TOGGLE != 0);
        entries.push(ArchiveEntry {
            path: norm_entry_path(&full),
            size: (raw.size & BSA_SIZE_MASK) as u64,
            compressed,
        });
    }
    Ok(ArchiveListing { format, entries })
}

// --------------------------------------------------------------------------- //
// BA2 (BTDX)
// --------------------------------------------------------------------------- //

/// Size of the fixed part of a BTDX header (magic, version, type, count,
/// name-table offset).
const BA2_BASE_HEADER: u64 = 24;

/// Header lengths to try for a BTDX version, most likely first. Version 1
/// (Fallout 4, Skyrim SE tools) and 7 (Fallout 4 next-gen) keep the 24-byte
/// header; Starfield's version 2 adds a 64-bit field, version 3 a further
/// 32-bit one; Fallout 4 next-gen version 8 adds a 32-bit field. The exact
/// layouts of the newer variants vary between tools, so the candidates are
/// tried in order and validated against the name table (see
/// [`parse_ba2`]).
fn ba2_header_candidates(version: u32) -> Option<&'static [u64]> {
    Some(match version {
        1 => &[24],
        7 => &[24, 28],
        2 => &[32, 24, 28, 36],
        3 => &[36, 32, 28, 24],
        8 => &[28, 24, 32, 36],
        _ => return None,
    })
}

fn parse_ba2<R: Read + Seek>(r: &mut Reader<R>) -> Result<ArchiveListing> {
    let version = r.u32()?;
    let kind = r.bytes4()?;
    let file_count = r.u32()?;
    let name_table_offset = r.u64()?;

    let is_dx10 = match &kind {
        b"GNRL" => false,
        b"DX10" => true,
        other => {
            return Err(ArchiveError::Unsupported(format!(
                "unsupported BA2 archive type '{}'",
                String::from_utf8_lossy(other).trim_end_matches('\0')
            )));
        }
    };
    let Some(candidates) = ba2_header_candidates(version) else {
        return Err(ArchiveError::UnsupportedVersion {
            container: "BA2",
            version,
        });
    };
    if name_table_offset == 0 || name_table_offset >= r.len {
        return Err(ArchiveError::Unsupported(
            "this BA2 has no readable name table (its file names are not stored)".to_string(),
        ));
    }

    // Try each header length: the records must end before the name table,
    // and the name table must hold exactly `file_count` names.
    let mut last_err = ArchiveError::Truncated;
    for &header_len in candidates {
        if header_len < BA2_BASE_HEADER || header_len > name_table_offset {
            continue;
        }
        r.seek(header_len)?;
        let records = if is_dx10 {
            read_dx10_records(r, file_count, name_table_offset)
        } else {
            read_gnrl_records(r, file_count, name_table_offset)
        };
        let records = match records {
            Ok(v) => v,
            Err(e) => {
                last_err = e;
                continue;
            }
        };
        r.seek(name_table_offset)?;
        let mut names = Vec::with_capacity(capacity_for(file_count));
        let mut ok = true;
        for _ in 0..file_count {
            match r.u16_string() {
                Ok(n) => names.push(n),
                Err(e) => {
                    last_err = e;
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let entries = names
            .into_iter()
            .zip(records)
            .map(|(name, (size, compressed))| ArchiveEntry {
                path: norm_entry_path(&name),
                size,
                compressed,
            })
            .collect();
        let format = if is_dx10 {
            ArchiveFormat::Ba2Dx10 { version }
        } else {
            ArchiveFormat::Ba2Gnrl { version }
        };
        return Ok(ArchiveListing { format, entries });
    }
    Err(last_err)
}

/// GNRL records (36 bytes each): `(unpacked size, compressed)` per file.
fn read_gnrl_records<R: Read + Seek>(r: &mut Reader<R>, count: u32, limit: u64) -> Result<Vec<(u64, bool)>> {
    let start = r.position()?;
    let needed = 36u64.checked_mul(count as u64).ok_or(ArchiveError::Truncated)?;
    if start.checked_add(needed).ok_or(ArchiveError::Truncated)? > limit {
        return Err(ArchiveError::Truncated);
    }
    let mut out = Vec::with_capacity(capacity_for(count));
    for _ in 0..count {
        let _name_hash = r.u32()?;
        let _ext = r.bytes4()?;
        let _dir_hash = r.u32()?;
        let _flags = r.u32()?;
        let _offset = r.u64()?;
        let packed = r.u32()?;
        let unpacked = r.u32()?;
        let _align = r.u32()?;
        out.push((unpacked as u64, packed != 0));
    }
    Ok(out)
}

/// DX10 records (24-byte header + 24 bytes per chunk): `(sum of the unpacked
/// chunk sizes, any chunk compressed)` per texture.
fn read_dx10_records<R: Read + Seek>(r: &mut Reader<R>, count: u32, limit: u64) -> Result<Vec<(u64, bool)>> {
    let mut out = Vec::with_capacity(capacity_for(count));
    for _ in 0..count {
        let _name_hash = r.u32()?;
        let _ext = r.bytes4()?;
        let _dir_hash = r.u32()?;
        let _unknown = r.u8()?;
        let chunk_count = r.u8()?;
        let chunk_header_size = r.u16()?;
        let _height = r.u16()?;
        let _width = r.u16()?;
        let _mips = r.u8()?;
        let _format = r.u8()?;
        let _flags = r.u16()?;
        let mut size = 0u64;
        let mut compressed = false;
        for _ in 0..chunk_count {
            let _offset = r.u64()?;
            let packed = r.u32()?;
            let unpacked = r.u32()?;
            let _mip_start = r.u16()?;
            let _mip_end = r.u16()?;
            let _align = r.u32()?;
            size += unpacked as u64;
            compressed |= packed != 0;
            // A chunk header wider than the 24 bytes we know is skipped.
            if chunk_header_size > 24 {
                r.skip(u64::from(chunk_header_size) - 24)?;
            }
        }
        if r.position()? > limit {
            return Err(ArchiveError::Truncated);
        }
        out.push((size, compressed));
    }
    Ok(out)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::io::Cursor;

    fn le32(v: &mut Vec<u8>, x: u32) {
        v.extend_from_slice(&x.to_le_bytes());
    }
    fn le64(v: &mut Vec<u8>, x: u64) {
        v.extend_from_slice(&x.to_le_bytes());
    }
    fn le16(v: &mut Vec<u8>, x: u16) {
        v.extend_from_slice(&x.to_le_bytes());
    }

    /// A BSA with the given folders (`(folder, [(file, size field)])`) and
    /// archive flags; file data is omitted (the directory is all we read).
    pub(crate) fn build_bsa(version: u32, flags: u32, folders: &[(&str, &[(&str, u32)])]) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(b"BSA\0");
        le32(&mut v, version);
        le32(&mut v, 36);
        le32(&mut v, flags);
        le32(&mut v, folders.len() as u32);
        let file_count: u32 = folders.iter().map(|(_, f)| f.len() as u32).sum();
        le32(&mut v, file_count);
        let total_folder_names: u32 = folders.iter().map(|(d, _)| d.len() as u32 + 1).sum();
        le32(&mut v, total_folder_names);
        let total_file_names: u32 = folders
            .iter()
            .flat_map(|(_, f)| f.iter().map(|(n, _)| n.len() as u32 + 1))
            .sum();
        le32(&mut v, total_file_names);
        le32(&mut v, 0);
        for (i, (_, files)) in folders.iter().enumerate() {
            le64(&mut v, 0x1000 + i as u64);
            le32(&mut v, files.len() as u32);
            if version == 105 {
                le32(&mut v, 0);
                le64(&mut v, 0);
            } else {
                le32(&mut v, 0);
            }
        }
        for (dir, files) in folders {
            if flags & BSA_FLAG_DIR_NAMES != 0 {
                v.push(dir.len() as u8 + 1);
                v.extend_from_slice(dir.as_bytes());
                v.push(0);
            }
            for (j, (_, size)) in files.iter().enumerate() {
                le64(&mut v, 0x2000 + j as u64);
                le32(&mut v, *size);
                le32(&mut v, 0);
            }
        }
        if flags & BSA_FLAG_FILE_NAMES != 0 {
            for (_, files) in folders {
                for (name, _) in files.iter() {
                    v.extend_from_slice(name.as_bytes());
                    v.push(0);
                }
            }
        }
        v
    }

    /// A GNRL BA2 (`(name, packed, unpacked)` per file) with the given
    /// version and header length; the data region is zero-filled.
    pub(crate) fn build_ba2_gnrl(version: u32, header_len: usize, files: &[(&str, u32, u32)]) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(b"BTDX");
        le32(&mut v, version);
        v.extend_from_slice(b"GNRL");
        le32(&mut v, files.len() as u32);
        let data_start = header_len as u64 + 36 * files.len() as u64;
        let data_len: u64 = files
            .iter()
            .map(|(_, p, u)| u64::from(if *p != 0 { *p } else { *u }))
            .sum();
        le64(&mut v, data_start + data_len);
        while v.len() < header_len {
            v.push(0);
        }
        let mut offset = data_start;
        for (name, packed, unpacked) in files {
            le32(&mut v, crate::archive::crc32(name.as_bytes()));
            v.extend_from_slice(b"dds\0");
            le32(&mut v, 0);
            le32(&mut v, 0);
            le64(&mut v, offset);
            le32(&mut v, *packed);
            le32(&mut v, *unpacked);
            le32(&mut v, 0xBAAD_F00D);
            offset += u64::from(if *packed != 0 { *packed } else { *unpacked });
        }
        v.resize(v.len() + data_len as usize, 0);
        for (name, _, _) in files {
            le16(&mut v, name.len() as u16);
            v.extend_from_slice(name.as_bytes());
        }
        v
    }

    /// A DX10 BA2: `(name, [(packed, unpacked)])` per texture.
    pub(crate) fn build_ba2_dx10(files: &[(&str, &[(u32, u32)])]) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(b"BTDX");
        le32(&mut v, 1);
        v.extend_from_slice(b"DX10");
        le32(&mut v, files.len() as u32);
        let records_len: u64 = files.iter().map(|(_, c)| 24 + 24 * c.len() as u64).sum();
        let data_start = 24 + records_len;
        let data_len: u64 = files
            .iter()
            .flat_map(|(_, c)| c.iter().map(|(p, u)| u64::from(if *p != 0 { *p } else { *u })))
            .sum();
        le64(&mut v, data_start + data_len);
        let mut offset = data_start;
        for (name, chunks) in files {
            le32(&mut v, crate::archive::crc32(name.as_bytes()));
            v.extend_from_slice(b"dds\0");
            le32(&mut v, 0);
            v.push(0);
            v.push(chunks.len() as u8);
            le16(&mut v, 24);
            le16(&mut v, 512);
            le16(&mut v, 512);
            v.push(chunks.len() as u8);
            v.push(0x62);
            le16(&mut v, 0x0800);
            for (i, (packed, unpacked)) in chunks.iter().enumerate() {
                le64(&mut v, offset);
                le32(&mut v, *packed);
                le32(&mut v, *unpacked);
                le16(&mut v, i as u16);
                le16(&mut v, i as u16);
                le32(&mut v, 0xBAAD_F00D);
                offset += u64::from(if *packed != 0 { *packed } else { *unpacked });
            }
        }
        v.resize(v.len() + data_len as usize, 0);
        for (name, _) in files {
            le16(&mut v, name.len() as u16);
            v.extend_from_slice(name.as_bytes());
        }
        v
    }

    fn parse(bytes: &[u8]) -> Result<ArchiveListing> {
        parse_archive(Cursor::new(bytes.to_vec()))
    }

    #[test]
    fn extension_check_is_case_insensitive() {
        assert!(is_bethesda_archive("Mod - Textures.BSA"));
        assert!(is_bethesda_archive(Path::new("x/Mod - Main.ba2")));
        assert!(!is_bethesda_archive("Mod.esp"));
        assert!(!is_bethesda_archive("archive.bsa.txt"));
        assert!(!is_bethesda_archive("bsa"));
    }

    #[test]
    fn bsa_v104_lists_folders_files_and_compression() {
        let bytes = build_bsa(
            104,
            BSA_FLAG_DIR_NAMES | BSA_FLAG_FILE_NAMES,
            &[
                (
                    "Textures\\Armor",
                    &[("Cuirass.dds", 100), ("Helmet.dds", 50 | BSA_SIZE_COMPRESSION_TOGGLE)],
                ),
                ("meshes", &[("Sword.nif", 7)]),
            ],
        );
        let l = parse(&bytes).unwrap();
        assert_eq!(l.format, ArchiveFormat::Bsa104);
        assert_eq!(
            l.entries,
            vec![
                ArchiveEntry {
                    path: "textures/armor/cuirass.dds".into(),
                    size: 100,
                    compressed: false
                },
                ArchiveEntry {
                    path: "textures/armor/helmet.dds".into(),
                    size: 50,
                    compressed: true
                },
                ArchiveEntry {
                    path: "meshes/sword.nif".into(),
                    size: 7,
                    compressed: false
                },
            ]
        );
        assert_eq!(l.total_size(), 157);

        // Archive-level compression flag: the toggle bit now means "stored".
        let bytes = build_bsa(
            103,
            BSA_FLAG_DIR_NAMES | BSA_FLAG_FILE_NAMES | BSA_FLAG_COMPRESSED,
            &[("a", &[("x.dds", 10), ("y.dds", 20 | BSA_SIZE_COMPRESSION_TOGGLE)])],
        );
        let l = parse(&bytes).unwrap();
        assert_eq!(l.format, ArchiveFormat::Bsa103);
        assert!(l.entries[0].compressed && !l.entries[1].compressed);
        assert_eq!((l.entries[0].size, l.entries[1].size), (10, 20));
    }

    #[test]
    fn bsa_v105_uses_24_byte_folder_records() {
        let bytes = build_bsa(
            105,
            BSA_FLAG_DIR_NAMES | BSA_FLAG_FILE_NAMES,
            &[
                ("Scripts", &[("A.pex", 1), ("B.pex", 2)]),
                ("Interface", &[("c.swf", 3)]),
            ],
        );
        let l = parse(&bytes).unwrap();
        assert_eq!(l.format, ArchiveFormat::Bsa105);
        let paths: Vec<&str> = l.entries.iter().map(|e| e.path.as_str()).collect();
        assert_eq!(paths, vec!["scripts/a.pex", "scripts/b.pex", "interface/c.swf"]);
        assert_eq!(l.entries[2].size, 3);

        // Without the name blocks the entries are named by hash.
        let bytes = build_bsa(105, 0, &[("Scripts", &[("A.pex", 1)])]);
        let l = parse(&bytes).unwrap();
        assert_eq!(l.entries.len(), 1);
        assert_eq!(l.entries[0].path, format!("{:016x}", 0x2000u64));
    }

    #[test]
    fn bsa_unknown_version_and_morrowind_are_rejected() {
        let bytes = build_bsa(106, 3, &[("a", &[("b", 1)])]);
        assert!(matches!(
            parse(&bytes),
            Err(ArchiveError::UnsupportedVersion {
                container: "BSA",
                version: 106
            })
        ));
        let mut tes3 = vec![0x00, 0x01, 0x00, 0x00];
        tes3.extend_from_slice(&[0u8; 32]);
        let err = parse(&tes3).unwrap_err();
        assert!(matches!(err, ArchiveError::Unsupported(_)));
        assert!(err.to_string().contains("Morrowind"), "{err}");
        assert!(matches!(parse(b"PK\x03\x04junk"), Err(ArchiveError::Unsupported(_))));
    }

    #[test]
    fn ba2_gnrl_v1_lists_sizes_and_compression() {
        let bytes = build_ba2_gnrl(1, 24, &[("Meshes\\Sword.nif", 0, 300), ("scripts\\a.pex", 40, 90)]);
        let l = parse(&bytes).unwrap();
        assert_eq!(l.format, ArchiveFormat::Ba2Gnrl { version: 1 });
        assert_eq!(
            l.entries,
            vec![
                ArchiveEntry {
                    path: "meshes/sword.nif".into(),
                    size: 300,
                    compressed: false
                },
                ArchiveEntry {
                    path: "scripts/a.pex".into(),
                    size: 90,
                    compressed: true
                },
            ]
        );
    }

    #[test]
    fn ba2_newer_versions_find_their_header_length() {
        // Starfield v2 (32-byte header), v3 (36), Fallout 4 next-gen v7 / v8.
        for (version, header_len) in [(2, 32), (3, 36), (7, 24), (8, 28)] {
            let bytes = build_ba2_gnrl(version, header_len, &[("a\\b.nif", 0, 5), ("c.nif", 0, 6)]);
            let l = parse(&bytes).unwrap_or_else(|e| panic!("v{version}: {e}"));
            assert_eq!(l.format, ArchiveFormat::Ba2Gnrl { version });
            assert_eq!(l.entries[0].path, "a/b.nif");
            assert_eq!(l.entries[1].size, 6);
        }
        let bytes = build_ba2_gnrl(42, 24, &[("a.nif", 0, 5)]);
        assert!(matches!(
            parse(&bytes),
            Err(ArchiveError::UnsupportedVersion {
                container: "BA2",
                version: 42
            })
        ));
    }

    #[test]
    fn ba2_dx10_sums_chunks() {
        let bytes = build_ba2_dx10(&[
            ("Textures\\A.dds", &[(0, 1000), (0, 250), (0, 64)]),
            ("textures\\b.dds", &[(500, 4096)]),
        ]);
        let l = parse(&bytes).unwrap();
        assert_eq!(l.format, ArchiveFormat::Ba2Dx10 { version: 1 });
        assert_eq!(
            l.entries,
            vec![
                ArchiveEntry {
                    path: "textures/a.dds".into(),
                    size: 1314,
                    compressed: false
                },
                ArchiveEntry {
                    path: "textures/b.dds".into(),
                    size: 4096,
                    compressed: true
                },
            ]
        );
    }

    #[test]
    fn truncated_inputs_fail_without_panicking() {
        let full = build_ba2_gnrl(1, 24, &[("a.nif", 0, 5), ("b\\c.nif", 0, 6)]);
        for cut in 0..full.len() {
            let err = parse(&full[..cut]).unwrap_err();
            assert!(
                matches!(err, ArchiveError::Truncated | ArchiveError::Unsupported(_)),
                "cut at {cut}: {err}"
            );
        }
        let full = build_bsa(105, 3, &[("Scripts", &[("A.pex", 1), ("B.pex", 2)])]);
        for cut in 0..full.len() {
            assert!(parse(&full[..cut]).is_err(), "cut at {cut}");
        }
        let full = build_ba2_dx10(&[("t.dds", &[(0, 10), (0, 20)])]);
        for cut in 0..full.len() {
            assert!(parse(&full[..cut]).is_err(), "cut at {cut}");
        }
        // A file count the stream cannot hold is a truncation, not an OOM.
        let mut huge = build_ba2_gnrl(1, 24, &[("a.nif", 0, 5)]);
        huge[12..16].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(parse(&huge), Err(ArchiveError::Truncated)));
        let mut huge = build_bsa(104, 3, &[("a", &[("b", 1)])]);
        huge[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(parse(&huge), Err(ArchiveError::Truncated)));
    }

    #[test]
    fn list_archive_reads_from_disk() {
        let dir = std::env::temp_dir().join(format!("ximod_bsa_read_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("Test.bsa");
        std::fs::write(&path, build_bsa(104, 3, &[("meshes", &[("x.nif", 9)])])).unwrap();
        let l = list_archive(&path).unwrap();
        assert_eq!(l.entries[0].path, "meshes/x.nif");
        assert!(matches!(
            list_archive(&dir.join("missing.bsa")),
            Err(ArchiveError::Io(_))
        ));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
