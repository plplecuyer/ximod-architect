//! Locate the user manual (PDF) shipped with the application.
//!
//! The manuals are plain PDF files named `XIMOD_Architect_<word>_<CC>.pdf`,
//! where `<CC>` is the ISO 3166-1 alpha-3 code of the country whose language
//! the manual is written in (`GBR` for English, `FRA` for French, …). They
//! live in a `Manuals/` folder: next to the executable for a release, at the
//! repository root for a source checkout.

use std::path::{Path, PathBuf};

/// Country code suffix of the manual written in the given interface locale
/// (ISO 639-3). Locales without a translated manual fall back to English.
fn manual_country(locale: &str) -> &'static str {
    match locale {
        "bul" => "BGR",
        "ces" => "CZE",
        "dan" => "DNK",
        "deu" => "DEU",
        "ell" => "ELL",
        "est" => "EST",
        "fin" => "FIN",
        "fra" => "FRA",
        "gle" => "IRL",
        "hrv" => "HRV",
        "hun" => "HUN",
        "ita" => "ITA",
        "jpn" => "JPN",
        "kor" => "KOR",
        "lav" => "LVA",
        "lit" => "LTU",
        "mlt" => "MLT",
        "nld" => "NLD",
        "nob" | "nor" => "NOR",
        "pol" => "POL",
        "por" => "PRT",
        "roh" => "CHE",
        "ron" => "ROU",
        "rus" => "RUS",
        "slk" => "SVK",
        "slv" => "SVN",
        "spa" => "ESP",
        "swe" => "SWE",
        "tur" => "TUR",
        "zho" => "CHN",
        _ => "GBR",
    }
}

/// Candidate `Manuals/` folders, most specific first.
fn candidate_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(exe) = std::env::current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        dirs.push(exe_dir.join("Manuals"));
        dirs.push(exe_dir.to_path_buf());
        // `cargo run`: target/<profile>/ximod-architect → repository root.
        for up in [1, 2, 3] {
            let mut d = exe_dir.to_path_buf();
            for _ in 0..up {
                d.pop();
            }
            dirs.push(d.join("Manuals"));
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        dirs.push(cwd.join("Manuals"));
    }
    dirs
}

/// The manual PDF whose name ends with `_<cc>.pdf` inside `dir`, if any.
fn manual_in(dir: &Path, cc: &str) -> Option<PathBuf> {
    let suffix = format!("_{cc}.pdf");
    let entries = std::fs::read_dir(dir).ok()?;
    let mut found: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("XIMOD") && n.ends_with(&suffix))
        })
        .collect();
    found.sort();
    found.into_iter().next()
}

/// Find the manual for `locale`, falling back to English, then to any manual.
pub fn find_manual(locale: &str) -> Option<PathBuf> {
    let cc = manual_country(locale);
    let dirs = candidate_dirs();
    for want in [cc, "GBR"] {
        for dir in &dirs {
            if let Some(p) = manual_in(dir, want) {
                return Some(p);
            }
        }
    }
    for dir in &dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut pdfs: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.extension()
                        .and_then(|e| e.to_str())
                        .is_some_and(|e| e.eq_ignore_ascii_case("pdf"))
                        && p.file_name()
                            .and_then(|n| n.to_str())
                            .is_some_and(|n| n.starts_with("XIMOD"))
                })
                .collect();
            pdfs.sort();
            if let Some(p) = pdfs.into_iter().next() {
                return Some(p);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_to_country_falls_back_to_english() {
        assert_eq!(manual_country("fra"), "FRA");
        assert_eq!(manual_country("nob"), "NOR");
        assert_eq!(manual_country("ara"), "GBR");
        assert_eq!(manual_country("eng"), "GBR");
    }

    #[test]
    fn picks_the_matching_manual() {
        let dir = std::env::temp_dir().join(format!("ximod_manual_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("XIMOD_Architect_Manual_GBR.pdf"), b"%PDF").unwrap();
        std::fs::write(dir.join("XIMOD_Architect_Manuel_FRA.pdf"), b"%PDF").unwrap();
        assert!(
            manual_in(&dir, "FRA")
                .unwrap()
                .ends_with("XIMOD_Architect_Manuel_FRA.pdf")
        );
        assert!(manual_in(&dir, "DEU").is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
