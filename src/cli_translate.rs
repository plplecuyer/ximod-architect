//! Translating an existing FOMOD: file-level operations and the `translate`
//! command-line group.
//!
//! ```text
//! ximod-architect translate extract <root> --lang <iso3> [--source-lang <iso3>] [-o FILE] [--force]
//! ximod-architect translate status  <FILE> [--strict]
//! ximod-architect translate update  <root> <FILE>
//! ximod-architect translate apply   <root> <FILE> [--mode inplace|sibling]
//!                                   [--force-explicit-order] [--force]
//! ximod-architect translate package <root> <FILE> [-o OUT] [--format zip|7z] [--full]
//!                                   [--name-template T]
//! ximod-architect translate export-csv <FILE> <CSV>
//! ximod-architect translate import-csv <FILE> <CSV>
//! ximod-architect translate tm-learn <FILE>
//! ximod-architect translate tm-apply <FILE>
//! ```
//!
//! The functions [`extract_translation`], [`update_translation`],
//! [`apply_translation`] and [`package_translation`] hold the logic (no
//! printing) so that the GUI can call them too; the `cli_*` functions only
//! parse arguments and report.
//!
//! Exit codes: 0 success, 1 error, 2 usage error, 3 `status --strict` failure.

#![allow(dead_code)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::archive::ArchiveFormat;
use crate::models::translate::{
    SourceFingerprint, TFile, TIssue, TStats, TUnit, TranslationDoc, TranslationMemory, UpdateReport, merge_update,
    sanitize_file_stem, stale_keys, validate_doc, write_atomic,
};
use crate::xml::patch::{
    Encoding, FomodFiles, PatchOpts, apply_units, decode_xml_bytes, encode, extract_units, fingerprint,
    locate_fomod_files,
};

/// One source file loaded for translation.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    /// Decoded text (no byte order mark).
    pub xml: String,
    pub encoding: Encoding,
    /// Raw bytes as found on disk.
    pub bytes: Vec<u8>,
}

/// The two XML files of a FOMOD, as found on disk.
#[derive(Debug, Clone, Default)]
pub struct SourceFiles {
    /// The `fomod` directory.
    pub dir: PathBuf,
    pub info: Option<SourceFile>,
    pub config: Option<SourceFile>,
}

impl SourceFiles {
    /// Fingerprints of the files, for the sidecar.
    pub fn fingerprint(&self) -> SourceFingerprint {
        let of = |f: &Option<SourceFile>| f.as_ref().map(|f| fingerprint(&f.bytes, f.encoding));
        SourceFingerprint {
            info: of(&self.info),
            config: of(&self.config),
        }
    }

    /// Translatable units of the files.
    pub fn units(&self) -> Vec<TUnit> {
        extract_units(
            self.info.as_ref().map(|f| f.xml.as_str()),
            self.config.as_ref().map(|f| f.xml.as_str()),
        )
    }
}

fn read_source(path: &Path) -> Result<SourceFile> {
    let bytes = std::fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let (xml, encoding) = decode_xml_bytes(&bytes);
    Ok(SourceFile {
        path: path.to_path_buf(),
        xml,
        encoding,
        bytes,
    })
}

/// Read `info.xml` and `ModuleConfig.xml` of the FOMOD in `root`. Fails when
/// neither exists.
pub fn read_sources(root: &Path) -> Result<SourceFiles> {
    let FomodFiles { dir, info, config } = locate_fomod_files(root);
    if info.is_none() && config.is_none() {
        bail!(
            "no FOMOD found in {} (expected fomod/info.xml or fomod/ModuleConfig.xml)",
            root.display()
        );
    }
    Ok(SourceFiles {
        dir,
        info: info.as_deref().map(read_source).transpose()?,
        config: config.as_deref().map(read_source).transpose()?,
    })
}

/// Build a new translation document for the FOMOD in `root`.
pub fn extract_translation(root: &Path, source_lang: &str, target_lang: &str) -> Result<TranslationDoc> {
    let sources = read_sources(root)?;
    let mut doc = TranslationDoc::new(source_lang, target_lang);
    doc.units = sources.units();
    doc.source = sources.fingerprint();

    let find = |key: &str| doc.units.iter().find(|u| u.key == key).map(|u| u.source.clone());
    doc.mod_name = find("info/Name")
        .or_else(|| find("config/moduleName"))
        .unwrap_or_default();
    if let Some(info) = &sources.info {
        // Best effort: the version is informative only.
        let mut model = crate::models::Ximod::default();
        if crate::xml::parse_info_xml(&info.xml, &mut model).is_ok() {
            doc.mod_version = model.version;
        }
    }
    Ok(doc)
}

/// Re-extract the FOMOD in `root` and merge an existing translation into it.
pub fn update_translation(root: &Path, doc: &TranslationDoc) -> Result<(TranslationDoc, UpdateReport)> {
    let sources = read_sources(root)?;
    let (mut merged, report) = merge_update(doc, sources.units());
    merged.source = sources.fingerprint();
    Ok((merged, report))
}

/// Where [`apply_translation`] writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyMode {
    /// `<root>/fomod_<lang>/`, leaving the original files untouched.
    Sibling,
    /// Over the original files, after a timestamped `.bak` copy.
    InPlace,
}

/// Options of [`apply_translation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyOptions {
    pub mode: ApplyMode,
    pub force_explicit_order: bool,
    /// Proceed despite blocking issues or a changed source. Units whose
    /// source changed are skipped, never applied to another string.
    pub force: bool,
}

/// What [`apply_translation`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApplyOutcome {
    /// Files written.
    pub written: Vec<PathBuf>,
    /// Backups created (in-place mode).
    pub backups: Vec<PathBuf>,
    /// Number of translations handed to the patcher.
    pub applied: usize,
    /// Units skipped because the FOMOD changed since the last update.
    pub stale: usize,
}

/// Lower-cased language tag usable in a folder name.
fn lang_slug(lang: &str) -> String {
    let slug: String = lang
        .trim()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    if slug.is_empty() { "und".to_string() } else { slug }
}

/// Folder receiving the translated files in sibling mode: `<root>/fomod_<lang>`.
pub fn sibling_dir(root: &Path, target_lang: &str) -> PathBuf {
    root.join(format!("fomod_{}", lang_slug(target_lang)))
}

/// Write the translation `doc` into the FOMOD in `root`.
///
/// Refuses (unless `force`) when the document has blocking issues or when the
/// FOMOD no longer matches the sources the translation was made from — run an
/// update first in that case. The files are patched losslessly and written in
/// their original encoding.
pub fn apply_translation(root: &Path, doc: &TranslationDoc, opts: &ApplyOptions) -> Result<ApplyOutcome> {
    let sources = read_sources(root)?;

    let blocking = validate_doc(doc).iter().filter(|(_, i)| i.is_blocking()).count();
    if blocking > 0 && !opts.force {
        bail!("{blocking} blocking issue(s) in the translation; fix them or use --force");
    }

    let stale = stale_keys(doc, &sources.units());
    if !stale.is_empty() && !opts.force {
        bail!(
            "the FOMOD changed since this translation was made ({} string(s) no longer match); \
             run 'translate update' first, or use --force to skip them",
            stale.len()
        );
    }
    let mut targets: HashMap<String, String> = doc.export_targets();
    targets.retain(|key, _| !stale.contains(key));

    let patch_opts = PatchOpts {
        force_explicit_order: opts.force_explicit_order,
    };
    let mut jobs: Vec<(&SourceFile, String)> = Vec::new();
    for (file, kind) in [(&sources.info, TFile::Info), (&sources.config, TFile::Config)] {
        if let Some(f) = file {
            let patched = apply_units(&f.xml, kind, &targets, &patch_opts)
                .with_context(|| format!("patching {}", f.path.display()))?;
            jobs.push((f, patched));
        }
    }

    let mut outcome = ApplyOutcome {
        applied: targets.len(),
        stale: stale.len(),
        ..Default::default()
    };
    match opts.mode {
        ApplyMode::Sibling => {
            let dir = sibling_dir(root, &doc.target_lang);
            std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
            for (f, patched) in &jobs {
                let name = f.path.file_name().context("source file has no name")?;
                let out = dir.join(name);
                write_atomic(&out, &encode(patched, f.encoding))?;
                outcome.written.push(out);
            }
        }
        ApplyMode::InPlace => {
            let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
            for (f, patched) in &jobs {
                // An unchanged file is left alone: no backup, no rewrite.
                if *patched == f.xml {
                    continue;
                }
                let name = f.path.file_name().context("source file has no name")?.to_string_lossy();
                let backup = f.path.with_file_name(format!("{name}.{stamp}.bak"));
                std::fs::write(&backup, &f.bytes)
                    .with_context(|| format!("writing the backup {}", backup.display()))?;
                outcome.backups.push(backup);
                write_atomic(&f.path, &encode(patched, f.encoding))?;
                outcome.written.push(f.path.clone());
            }
        }
    }
    Ok(outcome)
}

// ---------------------------------------------------------------------------
// Packaging a translation
// ---------------------------------------------------------------------------

/// What a translation package contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PackageKind {
    /// Only the translated `fomod` XML files and a README: a patch to install
    /// over the original mod.
    #[default]
    PatchOnly,
    /// The whole mod, with the translated XML files in place of the originals.
    Full,
}

/// Options of [`package_translation`].
#[derive(Debug, Clone)]
pub struct PackageOptions {
    pub kind: PackageKind,
    pub format: ArchiveFormat,
    /// Archive to write.
    pub out: PathBuf,
    pub force_explicit_order: bool,
    /// README text, already localised by the caller and stored as it is
    /// (UTF-8). `None`: the English default ([`default_readme`]).
    pub readme: Option<String>,
}

/// What [`package_translation`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageOutcome {
    /// The archive written.
    pub path: PathBuf,
    /// Number of files in the archive.
    pub files: usize,
    /// Number of translations handed to the patcher.
    pub applied: usize,
    /// Units skipped because the FOMOD changed since the last update.
    pub stale: usize,
}

/// Default README of a patch-only translation package, in English. `{name}`
/// stands for the mod name and `{langname}` for the target language;
/// [`default_readme`] fills them in.
pub const DEFAULT_README_EN: &str = "\
{name} - {langname} translation of the installer

This archive contains the translated installer files of {name}:
fomod/info.xml and fomod/ModuleConfig.xml. It does not contain the mod itself.

How to install: install the original mod first, then install this archive over
it and let it overwrite the two files (or let your mod manager merge it with
the original mod). The installer of the mod is then shown in {langname}.

Produced with XIMOD Architect.
";

/// Default README of a full translation package (the whole mod with its
/// installer translated), in English. Same placeholders as
/// [`DEFAULT_README_EN`].
pub const DEFAULT_README_FULL_EN: &str = "\
{name} - {langname} translation of the installer

This archive contains {name} with its installer files (fomod/info.xml and
fomod/ModuleConfig.xml) translated. The other files are those of the original
mod.

How to install: install this archive with your mod manager instead of the
original mod. The installer of the mod is then shown in {langname}.

Produced with XIMOD Architect.
";

/// Default template of a package name.
pub const DEFAULT_NAME_TEMPLATE: &str = "{name}_{LANG}";

/// ISO 639-1 code of a language tag, when the built-in table knows it
/// (`fra` → `fr`). `crate::i18n::iso639_3_to_1` answers `en` for anything it
/// does not know, hence the check.
fn iso1_of(lang: &str) -> Option<String> {
    let lang = lang_slug(lang);
    let iso1 = crate::i18n::iso639_3_to_1(&lang);
    (iso1 != "en" || lang == "eng" || lang == "en").then_some(iso1)
}

/// Upper-case language code used in file names: the two-letter code when the
/// language has one (`FR`), else the tag itself (`FRA`).
fn lang_code_upper(target_lang: &str) -> String {
    iso1_of(target_lang)
        .unwrap_or_else(|| lang_slug(target_lang))
        .to_uppercase()
}

/// English [`DEFAULT_README_EN`] filled in for `doc`.
pub fn default_readme(doc: &TranslationDoc) -> String {
    default_readme_for(doc, PackageKind::PatchOnly)
}

/// English default README of a package of the given kind, filled in for `doc`.
pub fn default_readme_for(doc: &TranslationDoc, kind: PackageKind) -> String {
    let name = if doc.mod_name.trim().is_empty() {
        "this mod"
    } else {
        doc.mod_name.trim()
    };
    let lang = crate::i18n::locale_display_name(&lang_slug(&doc.target_lang));
    let lang = if iso1_of(&doc.target_lang).is_some() && !lang.is_empty() {
        lang.to_string()
    } else {
        lang_code_upper(&doc.target_lang)
    };
    let template = match kind {
        PackageKind::PatchOnly => DEFAULT_README_EN,
        PackageKind::Full => DEFAULT_README_FULL_EN,
    };
    template.replace("{name}", name).replace("{langname}", &lang)
}

/// Name of the README stored in a translation package: `README_<LANG>.txt`.
pub fn readme_file_name(doc: &TranslationDoc) -> String {
    format!("README_{}.txt", lang_code_upper(&doc.target_lang))
}

/// File name of a translation package, from a template.
///
/// Tokens: `{name}` (mod name, `fomod` when empty), `{version}` (mod version),
/// `{LANG}` (upper-case two-letter code of the target language when it has
/// one, else the upper-case three-letter tag), `{lang}` (the same in lower
/// case), `{lang3}` (the target language tag of the document) and
/// `{langname}` (`lang_display`, the name of the language as the caller wants
/// it shown). Unknown tokens are left as they are. The result is made safe
/// for a file name and gets `.ext` appended (nothing when `ext` is empty). An
/// empty template means [`DEFAULT_NAME_TEMPLATE`].
pub fn package_name(doc: &TranslationDoc, template: &str, lang_display: &str, ext: &str) -> String {
    let template = if template.trim().is_empty() {
        DEFAULT_NAME_TEMPLATE
    } else {
        template.trim()
    };

    let mut name = sanitize_file_stem(&doc.mod_name);
    if name.is_empty() {
        name = "fomod".to_string();
    }
    let version = sanitize_file_stem(&doc.mod_version);
    let lang3 = lang_slug(&doc.target_lang);
    let lang_lower = iso1_of(&doc.target_lang).unwrap_or_else(|| lang3.clone());
    let lang_upper = lang_lower.to_uppercase();
    let lang_name = sanitize_file_stem(lang_display);

    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert("name", &name);
    values.insert("version", &version);
    values.insert("LANG", &lang_upper);
    values.insert("lang", &lang_lower);
    values.insert("lang3", &lang3);
    values.insert("langname", &lang_name);

    // Single pass, so that a value containing braces is never expanded again.
    let mut stem = String::with_capacity(template.len() + name.len());
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        stem.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        match after.find('}').map(|close| (&after[..close], close)) {
            Some((token, close)) if values.contains_key(token) => {
                stem.push_str(values[token]);
                rest = &after[close + 1..];
            }
            _ => {
                stem.push('{');
                rest = after;
            }
        }
    }
    stem.push_str(rest);

    let mut stem = sanitize_file_stem(&stem);
    if stem.is_empty() {
        stem = "fomod".to_string();
    }
    let ext = ext.trim().trim_start_matches('.');
    if ext.is_empty() { stem } else { format!("{stem}.{ext}") }
}

/// Default location of a translation package: next to the mod folder (in the
/// current directory when `root` has no parent), named after `template`.
pub fn default_package_path(root: &Path, doc: &TranslationDoc, template: &str, format: ArchiveFormat) -> PathBuf {
    // `canonicalize` so that a root given as "." still has a parent.
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let dir = root
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    dir.join(package_name(
        doc,
        template,
        &lang_slug(&doc.target_lang),
        format.extension(),
    ))
}

/// Path of `file` inside an archive of `root`: its real relative path (case
/// preserved), with `/` separators.
fn archive_rel_path(root: &Path, files_dir: &Path, file: &Path) -> Result<String> {
    let rel = match file.strip_prefix(root) {
        Ok(rel) => rel.to_path_buf(),
        // Cannot happen with `locate_fomod_files`; rebuild `<fomod>/<file>`.
        Err(_) => {
            let dir = files_dir
                .file_name()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("fomod"));
            dir.join(file.file_name().context("source file has no name")?)
        }
    };
    Ok(rel.to_string_lossy().replace('\\', "/"))
}

/// Build an archive holding the translation `doc` of the FOMOD in `root`,
/// without modifying anything in `root`.
///
/// * [`PackageKind::PatchOnly`]: the translated `info.xml` and/or
///   `ModuleConfig.xml`, at their real relative path (`fomod/info.xml`,
///   `Fomod/Info.xml`…), plus `README_<LANG>.txt`.
/// * [`PackageKind::Full`]: every distribution file of the mod, with the two
///   XML files replaced by their translation, plus the README.
///
/// The XML files are patched losslessly and stored in their original
/// encoding. Blocking issues are refused. Units whose source changed since
/// the translation was made are skipped (never written over another string)
/// and counted in [`PackageOutcome::stale`].
pub fn package_translation(
    root: &Path,
    doc: &TranslationDoc,
    opts: &PackageOptions,
    progress: crate::export::Progress<'_>,
) -> Result<PackageOutcome> {
    package_translation_excluding(root, doc, opts, &[], progress)
}

/// [`package_translation`], leaving the files `exclude` out of a full
/// package (archives built earlier inside `root`, for instance).
pub(crate) fn package_translation_excluding(
    root: &Path,
    doc: &TranslationDoc,
    opts: &PackageOptions,
    exclude: &[PathBuf],
    progress: crate::export::Progress<'_>,
) -> Result<PackageOutcome> {
    let sources = read_sources(root)?;

    let blocking = validate_doc(doc).iter().filter(|(_, i)| i.is_blocking()).count();
    if blocking > 0 {
        bail!("{blocking} blocking issue(s) in the translation; fix them before packaging");
    }

    let stale = stale_keys(doc, &sources.units());
    let mut targets: HashMap<String, String> = doc.export_targets();
    targets.retain(|key, _| !stale.contains(key));

    let patch_opts = PatchOpts {
        force_explicit_order: opts.force_explicit_order,
    };
    let mut overrides: HashMap<String, Vec<u8>> = HashMap::new();
    for (file, kind) in [(&sources.info, TFile::Info), (&sources.config, TFile::Config)] {
        if let Some(f) = file {
            let patched = apply_units(&f.xml, kind, &targets, &patch_opts)
                .with_context(|| format!("patching {}", f.path.display()))?;
            let rel = archive_rel_path(root, &sources.dir, &f.path)?;
            overrides.insert(rel, encode(&patched, f.encoding));
        }
    }
    let readme = opts
        .readme
        .clone()
        .unwrap_or_else(|| default_readme_for(doc, opts.kind));
    overrides.insert(readme_file_name(doc), readme.into_bytes());

    if let Some(parent) = opts.out.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }

    let only: Option<Vec<String>> = match opts.kind {
        PackageKind::PatchOnly => Some(overrides.keys().cloned().collect()),
        PackageKind::Full if exclude.is_empty() => None,
        PackageKind::Full => {
            let skip: Vec<PathBuf> = exclude
                .iter()
                .map(|p| p.canonicalize().unwrap_or_else(|_| p.clone()))
                .collect();
            let files = crate::export::collect_distribution_files(root, &opts.out)?;
            Some(
                files
                    .into_iter()
                    .filter(|rel| {
                        let abs = root.join(rel);
                        !skip.contains(&abs.canonicalize().unwrap_or(abs))
                    })
                    .map(|rel| rel.to_string_lossy().replace('\\', "/"))
                    .collect(),
            )
        }
    };
    let files =
        crate::archive::export_with_overrides(root, &opts.out, opts.format, &overrides, only.as_deref(), progress)?;

    Ok(PackageOutcome {
        path: opts.out.clone(),
        files,
        applied: targets.len(),
        stale: stale.len(),
    })
}

// ---------------------------------------------------------------------------
// Command line
// ---------------------------------------------------------------------------

/// Usage of the `translate` command group.
pub fn usage() -> &'static str {
    "Usage:\n\
\x20 ximod-architect translate extract <root> --lang <iso3> [--source-lang <iso3>] [-o FILE] [--force]\n\
\x20 ximod-architect translate status  <FILE> [--strict]\n\
\x20 ximod-architect translate update  <root> <FILE>\n\
\x20 ximod-architect translate apply   <root> <FILE> [--mode inplace|sibling] [--force-explicit-order] [--force]\n\
\x20 ximod-architect translate package <root> <FILE> [-o OUT] [--format zip|7z] [--full] [--name-template T]\n\
\x20 ximod-architect translate export-csv <FILE> <CSV>\n\
\x20 ximod-architect translate import-csv <FILE> <CSV>\n\
\x20 ximod-architect translate tm-learn <FILE>\n\
\x20 ximod-architect translate tm-apply <FILE>"
}

fn usage_error(msg: &str) -> i32 {
    eprintln!("translate: {msg}\n\n{}", usage());
    2
}

/// Arguments split into positionals, flags and options with a value.
struct Parsed {
    positional: Vec<String>,
    flags: Vec<String>,
    values: HashMap<String, String>,
}

/// Split `args`, accepting only the given flags and valued options.
fn parse_args(args: &[String], flags: &[&str], valued: &[&str]) -> Result<Parsed, String> {
    let mut parsed = Parsed {
        positional: Vec::new(),
        flags: Vec::new(),
        values: HashMap::new(),
    };
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if valued.contains(&a) {
            i += 1;
            match args.get(i) {
                Some(v) => {
                    parsed.values.insert(a.to_string(), v.clone());
                }
                None => return Err(format!("{a} requires a value")),
            }
        } else if flags.contains(&a) {
            parsed.flags.push(a.to_string());
        } else if a.len() > 1 && a.starts_with('-') {
            return Err(format!("unknown option '{a}'"));
        } else {
            parsed.positional.push(a.to_string());
        }
        i += 1;
    }
    Ok(parsed)
}

impl Parsed {
    fn flag(&self, name: &str) -> bool {
        self.flags.iter().any(|f| f == name)
    }

    fn value(&self, names: &[&str]) -> Option<&str> {
        names.iter().find_map(|n| self.values.get(*n)).map(String::as_str)
    }
}

/// Dispatch `translate <sub-command> …`. Returns the process exit code.
pub fn run(args: &[String]) -> i32 {
    let Some(sub) = args.first() else {
        return usage_error("missing sub-command");
    };
    let rest = &args[1..];
    match sub.as_str() {
        "extract" => cli_extract(rest),
        "status" => cli_status(rest),
        "update" => cli_update(rest),
        "apply" => cli_apply(rest),
        "package" => cli_package(rest),
        "export-csv" => cli_export_csv(rest),
        "import-csv" => cli_import_csv(rest),
        "tm-learn" => cli_tm_learn(rest),
        "tm-apply" => cli_tm_apply(rest),
        "-h" | "--help" | "help" => {
            println!("{}", usage());
            0
        }
        other => usage_error(&format!("unknown sub-command '{other}'")),
    }
}

fn print_stats(s: &TStats) {
    println!(
        "  {} string(s): {} done ({}%), {} auto, {} fuzzy, {} untranslated, {} locked",
        s.total,
        s.translated,
        s.percent(),
        s.auto,
        s.fuzzy,
        s.untranslated,
        s.locked
    );
}

fn cli_extract(args: &[String]) -> i32 {
    let p = match parse_args(args, &["--force"], &["--lang", "--source-lang", "-o", "--output"]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [root] = p.positional.as_slice() else {
        return usage_error("extract expects exactly one <root>");
    };
    let Some(lang) = p.value(&["--lang"]).filter(|l| !l.trim().is_empty()) else {
        return usage_error("extract requires --lang <iso3>");
    };
    let source_lang = p.value(&["--source-lang"]).unwrap_or("eng");
    let root = Path::new(root);

    let doc = match extract_translation(root, source_lang, lang) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: {e:#}");
            return 1;
        }
    };
    let out = match p.value(&["-o", "--output"]) {
        Some(o) => PathBuf::from(o),
        None => TranslationDoc::sidecar_path(root, &doc.mod_name, lang),
    };
    // Never overwrite somebody's work with an empty translation.
    if out.exists() && !p.flag("--force") {
        eprintln!(
            "Error: {} already exists. Use 'translate update' to refresh it, or --force to start over.",
            out.display()
        );
        return 1;
    }
    if let Err(e) = doc.save(&out) {
        eprintln!("Error: {e:#}");
        return 1;
    }
    println!("Extracted {} string(s) into {}", doc.units.len(), out.display());
    print_stats(&doc.stats());
    0
}

fn cli_status(args: &[String]) -> i32 {
    let p = match parse_args(args, &["--strict"], &[]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [file] = p.positional.as_slice() else {
        return usage_error("status expects exactly one <FILE>");
    };
    let doc = match TranslationDoc::load(Path::new(file)) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: {e:#}");
            return 1;
        }
    };

    let stats = doc.stats();
    println!(
        "{} [{} -> {}]",
        if doc.mod_name.is_empty() {
            file.as_str()
        } else {
            doc.mod_name.as_str()
        },
        doc.source_lang,
        doc.target_lang
    );
    print_stats(&stats);
    if !doc.obsolete.is_empty() {
        println!("  {} obsolete translation(s) kept for reuse", doc.obsolete.len());
    }

    let issues = validate_doc(&doc);
    let blocking = issues.iter().filter(|(_, i)| i.is_blocking()).count();
    println!(
        "  {} issue(s): {} blocking, {} warning(s)",
        issues.len(),
        blocking,
        issues.len() - blocking
    );
    let mut by_kind: Vec<(&'static str, usize, usize)> = Vec::new();
    for (_, issue) in &issues {
        let block = usize::from(issue.is_blocking());
        match by_kind.iter_mut().find(|(k, _, _)| *k == issue.kind()) {
            Some(entry) => {
                entry.1 += 1;
                entry.2 += block;
            }
            None => by_kind.push((issue.kind(), 1, block)),
        }
    }
    for (kind, count, block) in &by_kind {
        if *block > 0 {
            println!("    {kind}: {count} ({block} blocking)");
        } else {
            println!("    {kind}: {count}");
        }
    }
    for (key, issue) in issues.iter().filter(|(_, i)| i.is_blocking()) {
        println!("    blocking: {key}: {}", describe_issue(issue));
    }

    if p.flag("--strict") && (stats.untranslated > 0 || stats.fuzzy > 0 || blocking > 0) {
        eprintln!("status: translation is not complete (--strict)");
        return 3;
    }
    0
}

/// Short English description of an issue, for the console.
fn describe_issue(issue: &TIssue) -> String {
    match issue {
        TIssue::TokenMismatch { missing, extra } => {
            format!("token mismatch (missing: {missing:?}, extra: {extra:?})")
        }
        TIssue::LengthRatio { ratio } => format!("length ratio {ratio:.2}"),
        TIssue::InconsistentDuplicate { other_key } => {
            format!("translated differently from {other_key}")
        }
        other => other.kind().to_string(),
    }
}

fn cli_update(args: &[String]) -> i32 {
    let p = match parse_args(args, &[], &[]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [root, file] = p.positional.as_slice() else {
        return usage_error("update expects <root> <FILE>");
    };
    let file = Path::new(file);
    let result = TranslationDoc::load(file).and_then(|doc| {
        let (merged, report) = update_translation(Path::new(root), &doc)?;
        merged.save(file)?;
        Ok((merged, report))
    });
    match result {
        Ok((merged, r)) => {
            println!(
                "Updated {}: {} unchanged, {} moved, {} changed (fuzzy), {} new, {} removed",
                file.display(),
                r.unchanged,
                r.moved,
                r.changed,
                r.new,
                r.removed
            );
            print_stats(&merged.stats());
            0
        }
        Err(e) => {
            eprintln!("Error: {e:#}");
            1
        }
    }
}

fn cli_apply(args: &[String]) -> i32 {
    let p = match parse_args(args, &["--force-explicit-order", "--force"], &["--mode"]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [root, file] = p.positional.as_slice() else {
        return usage_error("apply expects <root> <FILE>");
    };
    let mode = match p.value(&["--mode"]).map(str::to_ascii_lowercase).as_deref() {
        None | Some("sibling") => ApplyMode::Sibling,
        Some("inplace") | Some("in-place") => ApplyMode::InPlace,
        Some(other) => return usage_error(&format!("unknown mode '{other}' (inplace or sibling)")),
    };
    let opts = ApplyOptions {
        mode,
        force_explicit_order: p.flag("--force-explicit-order"),
        force: p.flag("--force"),
    };

    let result = TranslationDoc::load(Path::new(file)).and_then(|doc| apply_translation(Path::new(root), &doc, &opts));
    match result {
        Ok(outcome) => {
            println!("Applied {} translation(s)", outcome.applied);
            if outcome.stale > 0 {
                println!("  skipped {} string(s) that no longer match the FOMOD", outcome.stale);
            }
            for b in &outcome.backups {
                println!("  backup: {}", b.display());
            }
            for w in &outcome.written {
                println!("  wrote:  {}", w.display());
            }
            if outcome.written.is_empty() {
                println!("  nothing to write (the files are already up to date)");
            }
            0
        }
        Err(e) => {
            eprintln!("Error: {e:#}");
            1
        }
    }
}

/// Report the outcome of a command: print on success, `Error: …` otherwise.
fn report(result: Result<()>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Error: {e:#}");
            1
        }
    }
}

/// Archive format named on the command line (`zip`, `7z`).
pub fn parse_archive_format(name: &str) -> Option<ArchiveFormat> {
    match name.trim().to_ascii_lowercase().as_str() {
        "zip" => Some(ArchiveFormat::Zip),
        "7z" | "7zip" | "sevenzip" => Some(ArchiveFormat::SevenZip),
        _ => None,
    }
}

fn cli_package(args: &[String]) -> i32 {
    let p = match parse_args(args, &["--full"], &["-o", "--output", "--format", "--name-template"]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [root, file] = p.positional.as_slice() else {
        return usage_error("package expects <root> <FILE>");
    };
    let format = match p.value(&["--format"]) {
        None => ArchiveFormat::Zip,
        Some(name) => match parse_archive_format(name) {
            Some(f) => f,
            None => return usage_error(&format!("unknown format '{name}' (zip or 7z)")),
        },
    };
    let root = Path::new(root);

    let result = TranslationDoc::load(Path::new(file)).and_then(|doc| {
        // The template remembered with the translation is the default.
        let template = p.value(&["--name-template"]).unwrap_or(&doc.export.name_template);
        let out = match p.value(&["-o", "--output"]) {
            Some(o) => PathBuf::from(o),
            None => default_package_path(root, &doc, template, format),
        };
        let opts = PackageOptions {
            kind: if p.flag("--full") {
                PackageKind::Full
            } else {
                PackageKind::PatchOnly
            },
            format,
            out,
            force_explicit_order: doc.export.force_explicit_order,
            readme: None,
        };
        package_translation(root, &doc, &opts, &mut |_, _, _| true)
    });
    report(result.map(|outcome| {
        println!(
            "Packaged {} translation(s) into {} ({} file(s))",
            outcome.applied,
            outcome.path.display(),
            outcome.files
        );
        if outcome.stale > 0 {
            println!("  skipped {} string(s) that no longer match the FOMOD", outcome.stale);
        }
    }))
}

fn cli_export_csv(args: &[String]) -> i32 {
    let p = match parse_args(args, &[], &[]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [file, csv] = p.positional.as_slice() else {
        return usage_error("export-csv expects <FILE> <CSV>");
    };
    let result = TranslationDoc::load(Path::new(file)).and_then(|doc| {
        let csv = Path::new(csv);
        if let Some(parent) = csv.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        // A byte order mark makes spreadsheets read the file as UTF-8.
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(doc.to_csv().as_bytes());
        write_atomic(csv, &bytes)?;
        println!("Exported {} string(s) to {}", doc.units.len(), csv.display());
        Ok(())
    });
    report(result)
}

fn cli_import_csv(args: &[String]) -> i32 {
    let p = match parse_args(args, &[], &[]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [file, csv] = p.positional.as_slice() else {
        return usage_error("import-csv expects <FILE> <CSV>");
    };
    let file = Path::new(file);
    let result = TranslationDoc::load(file).and_then(|mut doc| {
        let bytes = std::fs::read(csv).with_context(|| format!("Failed to read {csv}"))?;
        // Spreadsheets save CSV in various encodings; reuse the XML decoder
        // (UTF-8 with or without BOM, UTF-16, Windows-1252 fallback).
        let (text, _) = decode_xml_bytes(&bytes);
        let updated = doc.import_csv(&text).with_context(|| format!("reading {csv}"))?;
        doc.save(file)?;
        println!("Imported {csv}: {updated} string(s) updated");
        print_stats(&doc.stats());
        Ok(())
    });
    report(result)
}

fn cli_tm_learn(args: &[String]) -> i32 {
    let p = match parse_args(args, &[], &[]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [file] = p.positional.as_slice() else {
        return usage_error("tm-learn expects exactly one <FILE>");
    };
    let result = TranslationDoc::load(Path::new(file)).and_then(|doc| {
        let mut tm = TranslationMemory::load(&doc.source_lang, &doc.target_lang);
        let learned = tm.learn(&doc);
        tm.save()?;
        println!(
            "Learned {learned} translation(s); the {} -> {} memory now holds {}",
            doc.source_lang,
            doc.target_lang,
            tm.entries.len()
        );
        Ok(())
    });
    report(result)
}

fn cli_tm_apply(args: &[String]) -> i32 {
    let p = match parse_args(args, &[], &[]) {
        Ok(p) => p,
        Err(e) => return usage_error(&e),
    };
    let [file] = p.positional.as_slice() else {
        return usage_error("tm-apply expects exactly one <FILE>");
    };
    let file = Path::new(file);
    let result = TranslationDoc::load(file).and_then(|mut doc| {
        let tm = TranslationMemory::load(&doc.source_lang, &doc.target_lang);
        let filled = tm.apply(&mut doc);
        if filled > 0 {
            doc.save(file)?;
        }
        println!("Filled {filled} string(s) from the translation memory (to review)");
        print_stats(&doc.stats());
        Ok(())
    });
    report(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::translate::TStatus;

    const INFO: &str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\r\n<fomod>\r\n\t<Name>Armory</Name>\r\n\t<Author>Bob</Author>\r\n\t<Version>1.2</Version>\r\n\t<Website>https://example.org</Website>\r\n</fomod>\r\n";
    const CONFIG: &str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\r\n<!-- hand written -->\r\n<config>\r\n\t<moduleName>Armory</moduleName>\r\n\t<moduleDependencies><fileDependency file=\"Skyrim.esm\" state=\"Active\"/></moduleDependencies>\r\n\t<installSteps order=\"Ascending\">\r\n\t\t<installStep name=\"Options\">\r\n\t\t\t<optionalFileGroups>\r\n\t\t\t\t<group name=\"Quality\" type=\"SelectExactlyOne\">\r\n\t\t\t\t\t<plugins order=\"Explicit\">\r\n\t\t\t\t\t\t<plugin name=\"Low\"><description><![CDATA[Small & fast]]></description><files><file source=\"a.esp\" destination=\"a.esp\"></file></files></plugin>\r\n\t\t\t\t\t\t<plugin name=\"High\"><description>Big</description></plugin>\r\n\t\t\t\t\t</plugins>\r\n\t\t\t\t</group>\r\n\t\t\t</optionalFileGroups>\r\n\t\t</installStep>\r\n\t</installSteps>\r\n</config>\r\n";

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    /// Temp mod root with a UTF-8-BOM info.xml and a plain UTF-8 ModuleConfig.
    fn sample_root(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("ximod_cli_tr_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("fomod")).unwrap();
        let mut info = vec![0xEF, 0xBB, 0xBF];
        info.extend_from_slice(INFO.as_bytes());
        std::fs::write(root.join("fomod/info.xml"), info).unwrap();
        std::fs::write(root.join("fomod/ModuleConfig.xml"), CONFIG).unwrap();
        root
    }

    fn translate(doc: &mut TranslationDoc, key: &str, target: &str) {
        let u = doc.units.iter_mut().find(|u| u.key == key).expect(key);
        u.target = target.to_string();
        u.status = TStatus::Translated;
    }

    #[test]
    fn extract_then_apply_sibling() {
        let root = sample_root("sibling");
        let root_s = root.to_string_lossy().to_string();

        assert_eq!(run(&s(&["extract", &root_s, "--lang", "fra"])), 0);
        let sidecar = TranslationDoc::sidecar_path(&root, "Armory", "fra");
        assert!(sidecar.is_file(), "{}", sidecar.display());
        // A second extract must not wipe the sidecar.
        assert_eq!(run(&s(&["extract", &root_s, "--lang", "fra"])), 1);

        let mut doc = TranslationDoc::load(&sidecar).unwrap();
        assert_eq!((doc.source_lang.as_str(), doc.target_lang.as_str()), ("eng", "fra"));
        assert_eq!((doc.mod_name.as_str(), doc.mod_version.as_str()), ("Armory", "1.2"));
        assert_eq!(doc.units.len(), 10);
        assert_eq!(doc.source.info.as_ref().unwrap().encoding, "utf-8-bom");
        assert_eq!(doc.source.config.as_ref().unwrap().encoding, "utf-8");
        assert_eq!(doc.source.config.as_ref().unwrap().bytes, CONFIG.len() as u64);

        // Incomplete: fine normally, exit 3 in strict mode.
        let sidecar_s = sidecar.to_string_lossy().to_string();
        assert_eq!(run(&s(&["status", &sidecar_s])), 0);
        assert_eq!(run(&s(&["status", &sidecar_s, "--strict"])), 3);

        translate(&mut doc, "info/Name", "Armurerie");
        translate(&mut doc, "config/moduleName", "Armurerie");
        translate(&mut doc, "config/step[0]/name", "Choix");
        translate(&mut doc, "config/step[0]/group[0]/name", "Qualité");
        translate(&mut doc, "config/step[0]/group[0]/plugin[0]/name", "Basse");
        translate(
            &mut doc,
            "config/step[0]/group[0]/plugin[0]/description",
            "Petit & rapide",
        );
        translate(&mut doc, "config/step[0]/group[0]/plugin[1]/name", "Haute");
        translate(&mut doc, "config/step[0]/group[0]/plugin[1]/description", "Gros");
        doc.save(&sidecar).unwrap();
        assert_eq!(
            run(&s(&["status", &sidecar_s, "--strict"])),
            0,
            "author/website are locked"
        );

        assert_eq!(run(&s(&["apply", &root_s, &sidecar_s])), 0);

        // Originals untouched.
        assert_eq!(
            std::fs::read(root.join("fomod/ModuleConfig.xml")).unwrap(),
            CONFIG.as_bytes()
        );
        // Translated copies in fomod_fra, same encodings.
        let info = std::fs::read(root.join("fomod_fra/info.xml")).unwrap();
        assert!(info.starts_with(&[0xEF, 0xBB, 0xBF]));
        let info = String::from_utf8(info[3..].to_vec()).unwrap();
        assert_eq!(info, INFO.replace("<Name>Armory</Name>", "<Name>Armurerie</Name>"));

        let config = std::fs::read_to_string(root.join("fomod_fra/ModuleConfig.xml")).unwrap();
        let expected = CONFIG
            .replace("<moduleName>Armory</moduleName>", "<moduleName>Armurerie</moduleName>")
            .replace("name=\"Options\"", "name=\"Choix\"")
            .replace("name=\"Quality\"", "name=\"Qualité\"")
            .replace("name=\"Low\"", "name=\"Basse\"")
            .replace("Small & fast", "Petit & rapide")
            .replace("name=\"High\"", "name=\"Haute\"")
            .replace("<description>Big</description>", "<description>Gros</description>");
        assert_eq!(config, expected, "only the translated strings differ");

        // The translated copy is still a loadable FOMOD.
        let mut model = crate::models::Ximod::default();
        crate::xml::parse_module_config_xml(&config, &mut model).unwrap();
        assert_eq!(model.steps[0].plugin_groups[0].plugins[1].name, "Haute");

        // Neither the sidecar nor the translated copy ends up in a package.
        let files = crate::export::collect_distribution_files(&root, &root.join("out.zip")).unwrap();
        let names: Vec<String> = files.iter().map(|p| p.to_string_lossy().replace('\\', "/")).collect();
        assert_eq!(names, vec!["fomod/ModuleConfig.xml", "fomod/info.xml"]);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn apply_in_place_backs_up_and_keeps_encoding() {
        let root = sample_root("inplace");
        let mut doc = extract_translation(&root, "eng", "deu").unwrap();
        translate(&mut doc, "info/Name", "Rüstkammer");
        let sidecar = root.join("t.ximod-translation");
        doc.save(&sidecar).unwrap();

        let code = run(&s(&[
            "apply",
            &root.to_string_lossy(),
            &sidecar.to_string_lossy(),
            "--mode",
            "inplace",
            "--force-explicit-order",
        ]));
        assert_eq!(code, 0);

        let info = std::fs::read(root.join("fomod/info.xml")).unwrap();
        assert!(info.starts_with(&[0xEF, 0xBB, 0xBF]), "BOM kept");
        assert!(String::from_utf8_lossy(&info).contains("<Name>Rüstkammer</Name>"));
        let config = std::fs::read_to_string(root.join("fomod/ModuleConfig.xml")).unwrap();
        assert_eq!(config, CONFIG.replace("order=\"Ascending\"", "order=\"Explicit\""));

        // One backup per modified file, holding the original bytes.
        let mut backups: Vec<PathBuf> = std::fs::read_dir(root.join("fomod"))
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.to_string_lossy().ends_with(".bak"))
            .collect();
        backups.sort();
        assert_eq!(backups.len(), 2);
        assert!(
            backups[0]
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("ModuleConfig.xml.")
        );
        assert_eq!(std::fs::read(&backups[0]).unwrap(), CONFIG.as_bytes());

        // Applying again is a no-op (already applied, not stale).
        let again = apply_translation(
            &root,
            &doc,
            &ApplyOptions {
                mode: ApplyMode::InPlace,
                force_explicit_order: true,
                force: false,
            },
        )
        .unwrap();
        assert!(again.written.is_empty() && again.backups.is_empty());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn apply_refuses_blocking_issues_and_stale_sources() {
        let root = sample_root("refuse");
        let opts = ApplyOptions {
            mode: ApplyMode::Sibling,
            force_explicit_order: false,
            force: false,
        };
        let forced = ApplyOptions { force: true, ..opts };

        // Blocking issue: a line break in a name.
        let mut doc = extract_translation(&root, "eng", "fra").unwrap();
        translate(&mut doc, "config/step[0]/name", "Cho\nix");
        let err = apply_translation(&root, &doc, &opts).unwrap_err();
        assert!(err.to_string().contains("blocking"), "{err}");
        assert!(!sibling_dir(&root, "fra").exists());
        assert!(apply_translation(&root, &doc, &forced).is_ok());

        // Same refusal through the command line (exit code 1).
        let sidecar = root.join("t.ximod-translation");
        doc.save(&sidecar).unwrap();
        let args = s(&["apply", &root.to_string_lossy(), &sidecar.to_string_lossy()]);
        assert_eq!(run(&args), 1);

        // Stale: the author renamed an option after the translation was made.
        let mut doc = extract_translation(&root, "eng", "fra").unwrap();
        translate(&mut doc, "config/step[0]/group[0]/plugin[0]/name", "Basse");
        translate(&mut doc, "config/step[0]/group[0]/plugin[1]/name", "Haute");
        std::fs::write(
            root.join("fomod/ModuleConfig.xml"),
            CONFIG.replace("name=\"High\"", "name=\"Ultra\""),
        )
        .unwrap();
        let err = apply_translation(&root, &doc, &opts).unwrap_err();
        assert!(err.to_string().contains("translate update"), "{err}");
        // Forced: the stale unit is skipped, not written over "Ultra".
        let outcome = apply_translation(&root, &doc, &forced).unwrap();
        assert_eq!((outcome.applied, outcome.stale), (1, 1));
        let config = std::fs::read_to_string(root.join("fomod_fra/ModuleConfig.xml")).unwrap();
        assert!(config.contains("name=\"Basse\"") && config.contains("name=\"Ultra\""));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn update_merges_and_refreshes_the_fingerprint() {
        let root = sample_root("update");
        let mut doc = extract_translation(&root, "eng", "fra").unwrap();
        translate(&mut doc, "config/step[0]/group[0]/plugin[1]/name", "Haute");
        translate(&mut doc, "config/step[0]/group[0]/plugin[1]/description", "Gros");
        let sidecar = TranslationDoc::sidecar_path(&root, &doc.mod_name, "fra");
        doc.save(&sidecar).unwrap();
        let before = doc.source.config.clone();

        // The author drops "Low" (so "High" moves to index 0) and rewords "Big".
        let low = "\t\t\t\t\t\t<plugin name=\"Low\"><description><![CDATA[Small & fast]]></description><files><file source=\"a.esp\" destination=\"a.esp\"></file></files></plugin>\r\n";
        assert!(CONFIG.contains(low));
        let changed = CONFIG.replace(low, "").replace(">Big<", ">Bigger<");
        std::fs::write(root.join("fomod/ModuleConfig.xml"), changed).unwrap();

        let args = s(&["update", &root.to_string_lossy(), &sidecar.to_string_lossy()]);
        assert_eq!(run(&args), 0);

        let updated = TranslationDoc::load(&sidecar).unwrap();
        assert_ne!(updated.source.config, before);
        assert_eq!(updated.source.info, doc.source.info);
        let name = updated
            .units
            .iter()
            .find(|u| u.key == "config/step[0]/group[0]/plugin[0]/name")
            .unwrap();
        assert_eq!(
            (name.source.as_str(), name.target.as_str(), name.status),
            ("High", "Haute", TStatus::Translated)
        );
        let desc = updated
            .units
            .iter()
            .find(|u| u.key == "config/step[0]/group[0]/plugin[0]/description")
            .unwrap();
        assert_eq!((desc.source.as_str(), desc.target.as_str()), ("Bigger", ""));
        assert_eq!(updated.obsolete.len(), 1);
        assert_eq!(updated.obsolete[0].target, "Gros");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn usage_errors_return_2() {
        assert_eq!(run(&[]), 2);
        assert_eq!(run(&s(&["frobnicate"])), 2);
        assert_eq!(run(&s(&["extract", "some/root"])), 2, "--lang is mandatory");
        assert_eq!(run(&s(&["extract", "--lang"])), 2);
        assert_eq!(run(&s(&["status"])), 2);
        assert_eq!(run(&s(&["update", "root-only"])), 2);
        assert_eq!(run(&s(&["apply", "root", "file", "--mode", "elsewhere"])), 2);
        assert_eq!(run(&s(&["apply", "root", "file", "--bogus"])), 2);
        assert_eq!(run(&s(&["help"])), 0);
        // Runtime errors are 1.
        assert_eq!(run(&s(&["status", "does/not/exist.ximod-translation"])), 1);
        let empty = std::env::temp_dir().join(format!("ximod_cli_tr_empty_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&empty);
        assert_eq!(run(&s(&["extract", &empty.to_string_lossy(), "--lang", "fra"])), 1);
        let _ = std::fs::remove_dir_all(&empty);
    }

    /// Names of the files of a ZIP archive, sorted.
    fn zip_names(path: &Path) -> Vec<String> {
        let zip = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
        let mut names: Vec<String> = zip.file_names().map(str::to_string).collect();
        names.sort();
        names
    }

    fn zip_read(path: &Path, name: &str) -> Vec<u8> {
        use std::io::Read;
        let mut zip = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
        let mut out = Vec::new();
        zip.by_name(name).unwrap().read_to_end(&mut out).unwrap();
        out
    }

    fn package_opts(kind: PackageKind, out: PathBuf) -> PackageOptions {
        PackageOptions {
            kind,
            format: ArchiveFormat::Zip,
            out,
            force_explicit_order: false,
            readme: None,
        }
    }

    #[test]
    fn package_patch_only_keeps_encoding_and_originals() {
        let root = sample_root("pkg_patch");
        // The author saved ModuleConfig.xml as UTF-16 and shipped loose files.
        let config16 = CONFIG.replace("encoding=\"utf-8\"", "encoding=\"utf-16\"");
        let config_bytes = encode(&config16, Encoding::Utf16Le);
        assert!(config_bytes.starts_with(&[0xFF, 0xFE]));
        std::fs::write(root.join("fomod/ModuleConfig.xml"), &config_bytes).unwrap();
        std::fs::write(root.join("a.esp"), b"plugin").unwrap();
        let info_bytes = std::fs::read(root.join("fomod/info.xml")).unwrap();

        let mut doc = extract_translation(&root, "eng", "fra").unwrap();
        assert_eq!(doc.source.config.as_ref().unwrap().encoding, "utf-16le");
        translate(&mut doc, "info/Name", "Armurerie");
        translate(&mut doc, "config/step[0]/group[0]/name", "Qualité");
        translate(
            &mut doc,
            "config/step[0]/group[0]/plugin[0]/description",
            "Petit & rapide",
        );

        let out = root.join("dist").join("patch.zip");
        let mut calls = 0;
        let outcome = package_translation(
            &root,
            &doc,
            &package_opts(PackageKind::PatchOnly, out.clone()),
            &mut |_, _, _| {
                calls += 1;
                true
            },
        )
        .unwrap();
        assert_eq!(
            outcome,
            PackageOutcome {
                path: out.clone(),
                files: 3,
                applied: 3,
                stale: 0
            }
        );
        assert_eq!(calls, 4, "one call per file and a final one");

        // Exactly the two XML files and the README.
        assert_eq!(
            zip_names(&out),
            vec!["README_FR.txt", "fomod/ModuleConfig.xml", "fomod/info.xml"]
        );

        // info.xml: UTF-8 with its BOM, only the name changed.
        let info = zip_read(&out, "fomod/info.xml");
        assert!(info.starts_with(&[0xEF, 0xBB, 0xBF]));
        assert_eq!(
            String::from_utf8(info[3..].to_vec()).unwrap(),
            INFO.replace("<Name>Armory</Name>", "<Name>Armurerie</Name>")
        );
        // ModuleConfig.xml: still UTF-16 LE, patched text present.
        let config = zip_read(&out, "fomod/ModuleConfig.xml");
        let expected = config16
            .replace("name=\"Quality\"", "name=\"Qualité\"")
            .replace("Small & fast", "Petit & rapide");
        assert_eq!(config, encode(&expected, Encoding::Utf16Le));
        let (text, encoding) = decode_xml_bytes(&config);
        assert_eq!(encoding, Encoding::Utf16Le);
        assert!(text.contains("name=\"Qualité\"") && text.contains("Petit & rapide"));

        let readme = String::from_utf8(zip_read(&out, "README_FR.txt")).unwrap();
        assert_eq!(readme, default_readme(&doc));
        assert!(readme.contains("Armory") && readme.contains("XIMOD Architect"));
        assert!(!readme.contains('{'), "placeholders are filled in: {readme}");

        // Nothing changed on disk.
        assert_eq!(
            std::fs::read(root.join("fomod/ModuleConfig.xml")).unwrap(),
            config_bytes
        );
        assert_eq!(std::fs::read(root.join("fomod/info.xml")).unwrap(), info_bytes);
        assert!(!sibling_dir(&root, "fra").exists());
        assert!(!root.join("README_FR.txt").exists());

        // A caller-supplied README is stored as it is; 7z works the same way.
        let out7 = root.join("dist").join("patch.7z");
        let opts = PackageOptions {
            format: ArchiveFormat::SevenZip,
            readme: Some("Lisez-moi : écrasez les fichiers.".to_string()),
            force_explicit_order: true,
            ..package_opts(PackageKind::PatchOnly, out7.clone())
        };
        let outcome = package_translation(&root, &doc, &opts, &mut |_, _, _| true).unwrap();
        assert_eq!(outcome.files, 3);
        let back = root.join("dist").join("unpacked");
        sevenz_rust::decompress_file(&out7, &back).expect("7z should be readable");
        assert_eq!(
            std::fs::read_to_string(back.join("README_FR.txt")).unwrap(),
            "Lisez-moi : écrasez les fichiers."
        );
        let (text, encoding) = decode_xml_bytes(&std::fs::read(back.join("fomod/ModuleConfig.xml")).unwrap());
        assert_eq!(encoding, Encoding::Utf16Le);
        assert!(text.contains("<installSteps order=\"Explicit\">") && text.contains("name=\"Qualité\""));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn package_full_overrides_the_xml_and_preserves_case() {
        // A third-party layout: `Fomod/Info.xml`, `Fomod/moduleconfig.xml`.
        let root = std::env::temp_dir().join(format!("ximod_cli_tr_pkg_full_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("Fomod/translations")).unwrap();
        std::fs::create_dir_all(root.join("textures")).unwrap();
        std::fs::write(root.join("Fomod/Info.xml"), INFO).unwrap();
        std::fs::write(root.join("Fomod/moduleconfig.xml"), CONFIG).unwrap();
        std::fs::write(root.join("a.esp"), b"plugin").unwrap();
        std::fs::write(root.join("textures/a.dds"), b"texture").unwrap();

        let mut doc = extract_translation(&root, "eng", "deu").unwrap();
        translate(&mut doc, "info/Name", "Rüstkammer");
        translate(&mut doc, "config/step[0]/group[0]/plugin[1]/name", "Hoch");
        // The sidecar lives in the mod folder but never ships.
        doc.save(&TranslationDoc::sidecar_path(&root, &doc.mod_name, "deu"))
            .unwrap();

        let out = root.join("full.zip");
        let outcome = package_translation(
            &root,
            &doc,
            &package_opts(PackageKind::Full, out.clone()),
            &mut |_, _, _| true,
        )
        .unwrap();
        assert_eq!((outcome.files, outcome.applied, outcome.stale), (5, 2, 0));
        assert_eq!(
            zip_names(&out),
            vec![
                "Fomod/Info.xml",
                "Fomod/moduleconfig.xml",
                "README_DE.txt",
                "a.esp",
                "textures/a.dds"
            ]
        );
        assert_eq!(
            String::from_utf8(zip_read(&out, "Fomod/Info.xml")).unwrap(),
            INFO.replace("<Name>Armory</Name>", "<Name>Rüstkammer</Name>")
        );
        assert_eq!(
            String::from_utf8(zip_read(&out, "Fomod/moduleconfig.xml")).unwrap(),
            CONFIG.replace("name=\"High\"", "name=\"Hoch\"")
        );
        assert_eq!(zip_read(&out, "a.esp"), b"plugin");
        assert_eq!(std::fs::read_to_string(root.join("Fomod/Info.xml")).unwrap(), INFO);
        assert_eq!(
            String::from_utf8(zip_read(&out, "README_DE.txt")).unwrap(),
            default_readme_for(&doc, PackageKind::Full)
        );
        assert_ne!(default_readme_for(&doc, PackageKind::Full), default_readme(&doc));

        // Patch-only uses the same spelling; an existing archive in the
        // folder can be left out of a later full package.
        let patch = root.join("patch.zip");
        package_translation(
            &root,
            &doc,
            &package_opts(PackageKind::PatchOnly, patch.clone()),
            &mut |_, _, _| true,
        )
        .unwrap();
        assert_eq!(
            zip_names(&patch),
            vec!["Fomod/Info.xml", "Fomod/moduleconfig.xml", "README_DE.txt"]
        );
        let again = root.join("again.zip");
        let outcome = package_translation_excluding(
            &root,
            &doc,
            &package_opts(PackageKind::Full, again.clone()),
            &[out.clone(), patch.clone()],
            &mut |_, _, _| true,
        )
        .unwrap();
        assert_eq!(outcome.files, 5);
        assert!(!zip_names(&again).iter().any(|n| n.ends_with(".zip")));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn package_refuses_blocking_issues_and_skips_stale_units() {
        let root = sample_root("pkg_refuse");
        let out = root.join("out.zip");

        let mut doc = extract_translation(&root, "eng", "fra").unwrap();
        translate(&mut doc, "config/step[0]/name", "Cho\nix");
        let err = package_translation(
            &root,
            &doc,
            &package_opts(PackageKind::PatchOnly, out.clone()),
            &mut |_, _, _| true,
        )
        .unwrap_err();
        assert!(err.to_string().contains("blocking"), "{err}");
        assert!(!out.exists());

        // The author renamed an option: that unit is skipped, the rest ships.
        let mut doc = extract_translation(&root, "eng", "fra").unwrap();
        translate(&mut doc, "config/step[0]/group[0]/plugin[0]/name", "Basse");
        translate(&mut doc, "config/step[0]/group[0]/plugin[1]/name", "Haute");
        std::fs::write(
            root.join("fomod/ModuleConfig.xml"),
            CONFIG.replace("name=\"High\"", "name=\"Ultra\""),
        )
        .unwrap();
        let outcome = package_translation(
            &root,
            &doc,
            &package_opts(PackageKind::PatchOnly, out.clone()),
            &mut |_, _, _| true,
        )
        .unwrap();
        assert_eq!((outcome.applied, outcome.stale), (1, 1));
        let config = String::from_utf8(zip_read(&out, "fomod/ModuleConfig.xml")).unwrap();
        assert!(config.contains("name=\"Basse\"") && config.contains("name=\"Ultra\""));

        // Cancelling leaves no archive behind.
        let cancelled = root.join("cancelled.zip");
        let err = package_translation(
            &root,
            &doc,
            &package_opts(PackageKind::PatchOnly, cancelled.clone()),
            &mut |_, _, _| false,
        )
        .unwrap_err();
        assert!(err.downcast_ref::<crate::export::Cancelled>().is_some(), "{err}");
        assert!(!cancelled.exists());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn package_name_tokens() {
        let mut doc = TranslationDoc::new("eng", "fra");
        doc.mod_name = "My: Mod/Name".into();
        doc.mod_version = "1.2".into();

        assert_eq!(
            package_name(&doc, "{name}_{LANG}", "Français", "zip"),
            "My_ Mod_Name_FR.zip"
        );
        assert_eq!(
            package_name(&doc, "{name}-{version} [{lang}] ({lang3}) {langname}", "Français", "7z"),
            "My_ Mod_Name-1.2 [fr] (fra) Français.7z"
        );
        // Empty template: the default; empty extension: none; a leading dot is accepted.
        assert_eq!(package_name(&doc, "  ", "French", ""), "My_ Mod_Name_FR");
        assert_eq!(package_name(&doc, "{name}", "French", ".zip"), "My_ Mod_Name.zip");
        // Unknown tokens and stray braces are kept; values are not expanded again.
        assert_eq!(
            package_name(&doc, "{name}_{foo}_{", "French", "zip"),
            "My_ Mod_Name_{foo}_{.zip"
        );
        doc.mod_name = "{LANG}".into();
        assert_eq!(package_name(&doc, "{name}_{LANG}", "French", "zip"), "{LANG}_FR.zip");
        // The template itself cannot smuggle a path.
        assert_eq!(package_name(&doc, "../{lang}/x", "French", "zip"), "_fr_x.zip");

        // Fallbacks: no mod name, a language without a two-letter code.
        let mut doc = TranslationDoc::new("eng", "TLH");
        assert_eq!(
            package_name(&doc, "{name}_{LANG}_{lang}_{lang3}", "Klingon", "zip"),
            "fomod_TLH_tlh_tlh.zip"
        );
        assert_eq!(readme_file_name(&doc), "README_TLH.txt");
        doc.target_lang = "eng".into();
        assert_eq!(package_name(&doc, "{name}_{LANG}", "English", "zip"), "fomod_EN.zip");
        doc.target_lang = "de".into();
        assert_eq!(package_name(&doc, "{LANG}{version}", "Deutsch", "zip"), "DE.zip");

        // Default location: next to the mod folder.
        let root = std::env::temp_dir().join("ximod_no_such_parent").join("MyMod");
        let path = default_package_path(&root, &doc, "{name}_{LANG}", ArchiveFormat::SevenZip);
        assert_eq!(path, root.parent().unwrap().join("fomod_DE.7z"));
    }

    #[test]
    fn package_and_csv_commands() {
        let root = sample_root("pkg_cli");
        let root_s = root.to_string_lossy().to_string();
        assert_eq!(run(&s(&["extract", &root_s, "--lang", "fra"])), 0);
        let sidecar = TranslationDoc::sidecar_path(&root, "Armory", "fra");
        let sidecar_s = sidecar.to_string_lossy().to_string();

        // CSV out, edited, and back in.
        let csv = root.join("work").join("armory.csv");
        let csv_s = csv.to_string_lossy().to_string();
        assert_eq!(run(&s(&["export-csv", &sidecar_s, &csv_s])), 0);
        let bytes = std::fs::read(&csv).unwrap();
        assert!(bytes.starts_with(&[0xEF, 0xBB, 0xBF]), "BOM for spreadsheets");
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains("key,field,context,source,target,status,locked,note\n"));
        let edited = text
            .replace(
                ",Armory,,untranslated,",
                ",Armory,\"Armurerie, la vraie\",untranslated,",
            )
            .replace(",Big,,untranslated,", ",Big,Gros,untranslated,");
        assert_ne!(edited, text);
        std::fs::write(&csv, edited).unwrap();
        assert_eq!(run(&s(&["import-csv", &sidecar_s, &csv_s])), 0);
        let doc = TranslationDoc::load(&sidecar).unwrap();
        let name = doc.units.iter().find(|u| u.key == "info/Name").unwrap();
        assert_eq!(
            (name.target.as_str(), name.status),
            ("Armurerie, la vraie", TStatus::Translated)
        );
        assert_eq!(
            doc.stats().translated,
            3 + 2,
            "name, module name, description + the two locked units"
        );

        // Package: explicit output, then the default one next to the mod folder.
        let out = root.join("dist").join("p.7z");
        let out_s = out.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["package", &root_s, &sidecar_s, "-o", &out_s, "--format", "7z"])),
            0
        );
        assert!(out.is_file());
        let default_out = default_package_path(&root, &doc, "{name}-{lang3}", ArchiveFormat::Zip);
        assert_eq!(default_out.file_name().unwrap(), "Armory-fra.zip");
        let _ = std::fs::remove_file(&default_out);
        std::fs::remove_dir_all(root.join("dist")).unwrap();
        std::fs::remove_dir_all(root.join("work")).unwrap();
        std::fs::write(root.join("a.esp"), b"plugin").unwrap();
        assert_eq!(
            run(&s(&[
                "package",
                &root_s,
                &sidecar_s,
                "--full",
                "--name-template",
                "{name}-{lang3}"
            ])),
            0
        );
        let names = zip_names(&default_out);
        assert_eq!(
            names,
            vec!["README_FR.txt", "a.esp", "fomod/ModuleConfig.xml", "fomod/info.xml"]
        );
        assert!(String::from_utf8_lossy(&zip_read(&default_out, "fomod/info.xml")).contains("Armurerie, la vraie"));
        let _ = std::fs::remove_file(&default_out);

        // Usage and runtime errors.
        assert_eq!(run(&s(&["package", &root_s])), 2);
        assert_eq!(run(&s(&["package", &root_s, &sidecar_s, "--format", "rar"])), 2);
        assert_eq!(run(&s(&["package", &root_s, "missing.ximod-translation"])), 1);
        assert_eq!(run(&s(&["export-csv", &sidecar_s])), 2);
        assert_eq!(run(&s(&["import-csv", &sidecar_s])), 2);
        assert_eq!(run(&s(&["import-csv", &sidecar_s, "missing.csv"])), 1);
        assert_eq!(run(&s(&["tm-learn"])), 2);
        assert_eq!(run(&s(&["tm-apply", "a", "b"])), 2);
        assert_eq!(run(&s(&["tm-apply", "missing.ximod-translation"])), 1);

        let _ = std::fs::remove_dir_all(&root);
    }
}
