//! Project strings: every name and description of the active project as a
//! flat table of [`TUnit`]s, addressed by the same stable keys as the
//! translation extractor (`xml::patch::extract_units`) would produce for the
//! saved XML — `info/Name`, `config/step[0]/group[1]/plugin[2]/description`…
//!
//! Backs the "Project strings" window (search, duplicates, find / replace,
//! inline edit) and the `strings` command of the CLI.

use std::collections::HashMap;

use super::Ximod;
use super::translate::{TField, TStatus, TUnit, apply_field, push_csv_field};
use crate::xml::patch::CONTEXT_SEPARATOR;

/// Build one unit; empty strings produce none (like the extractor).
fn unit(key: String, field: TField, source: &str, context: &[&str]) -> Option<TUnit> {
    if source.trim().is_empty() {
        return None;
    }
    let context = context
        .iter()
        .filter(|l| !l.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(CONTEXT_SEPARATOR);
    Some(TUnit {
        key,
        field,
        source: source.to_string(),
        target: String::new(),
        status: TStatus::Untranslated,
        locked: field.locked_by_default(),
        context,
        note: String::new(),
    })
}

/// The strings of the model, `info.xml` first then `ModuleConfig.xml`, in
/// document order, with the keys, fields and breadcrumbs of the extractor.
/// `info/Name` and `config/moduleName` both mirror the mod name.
pub fn model_units(ximod: &Ximod) -> Vec<TUnit> {
    let mut out = Vec::new();
    out.extend(unit("info/Name".into(), TField::InfoName, &ximod.name, &[]));
    out.extend(unit("info/Author".into(), TField::InfoAuthor, &ximod.author, &[]));
    out.extend(unit("info/Website".into(), TField::InfoWebsite, &ximod.url, &[]));
    out.extend(unit(
        "info/Description".into(),
        TField::InfoDescription,
        &ximod.description,
        &[],
    ));
    out.extend(unit("config/moduleName".into(), TField::ModuleName, &ximod.name, &[]));
    for (si, step) in ximod.steps.iter().enumerate() {
        out.extend(unit(
            format!("config/step[{si}]/name"),
            TField::StepName,
            &step.name,
            &[&step.name],
        ));
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            out.extend(unit(
                format!("config/step[{si}]/group[{gi}]/name"),
                TField::GroupName,
                &group.name,
                &[&step.name, &group.name],
            ));
            for (pi, plugin) in group.plugins.iter().enumerate() {
                let ctx = [step.name.as_str(), group.name.as_str(), plugin.name.as_str()];
                out.extend(unit(
                    format!("config/step[{si}]/group[{gi}]/plugin[{pi}]/name"),
                    TField::PluginName,
                    &plugin.name,
                    &ctx,
                ));
                out.extend(unit(
                    format!("config/step[{si}]/group[{gi}]/plugin[{pi}]/description"),
                    TField::PluginDescription,
                    &plugin.description,
                    &ctx,
                ));
            }
        }
    }
    out
}

/// Write `text` into the model string addressed by `unit_key` / `field`.
/// Returns `false` when the key points at a node that no longer exists.
pub fn set_unit(ximod: &mut Ximod, unit_key: &str, field: TField, text: &str) -> bool {
    apply_field(ximod, unit_key, field, text.to_string())
}

/// Case- and whitespace-insensitive form of a string, the identity used to
/// find duplicates.
pub fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// Groups of units with the same text (ignoring case and whitespace):
/// normalised text → indices into `units`. Only groups of two or more are
/// kept, empty strings are ignored, and a group made only of the mod name's
/// two mirrors (`info/Name`, `config/moduleName`) does not count.
pub fn duplicates(units: &[TUnit]) -> HashMap<String, Vec<usize>> {
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, u) in units.iter().enumerate() {
        let n = normalize(&u.source);
        if n.is_empty() {
            continue;
        }
        groups.entry(n).or_default().push(i);
    }
    groups.retain(|_, idx| {
        idx.len() >= 2
            && !idx
                .iter()
                .all(|&i| matches!(units[i].field, TField::InfoName | TField::ModuleName))
    });
    groups
}

/// `true` when `c` is part of a word (letters, digits, underscore).
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Replace every occurrence of `needle` in `text`, optionally ignoring case
/// and matching whole words only. `None` when nothing matched.
pub fn replace_in(
    text: &str,
    needle: &str,
    replacement: &str,
    case_sensitive: bool,
    whole_word: bool,
) -> Option<String> {
    if needle.is_empty() {
        return None;
    }
    let needle_chars: Vec<char> = needle.chars().collect();
    let chars: Vec<char> = text.chars().collect();
    let eq = |a: char, b: char| {
        if case_sensitive {
            a == b
        } else {
            a.to_lowercase().eq(b.to_lowercase())
        }
    };
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let mut replaced = false;
    while i < chars.len() {
        let end = i + needle_chars.len();
        let matches = end <= chars.len()
            && chars[i..end].iter().zip(needle_chars.iter()).all(|(a, b)| eq(*a, *b))
            && (!whole_word
                || ((i == 0 || !is_word_char(chars[i - 1])) && (end == chars.len() || !is_word_char(chars[end]))));
        if matches {
            out.push_str(replacement);
            i = end;
            replaced = true;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    replaced.then_some(out)
}

/// Find / replace over the units: the index and the new text of every unit
/// whose source contains `needle` (restricted to `only_fields` when given).
/// Nothing is written: the caller applies the result with [`set_unit`].
pub fn replace_all(
    units: &[TUnit],
    needle: &str,
    replacement: &str,
    case_sensitive: bool,
    whole_word: bool,
    only_fields: Option<&[TField]>,
) -> Vec<(usize, String)> {
    units
        .iter()
        .enumerate()
        .filter(|(_, u)| only_fields.is_none_or(|f| f.contains(&u.field)))
        .filter_map(|(i, u)| replace_in(&u.source, needle, replacement, case_sensitive, whole_word).map(|t| (i, t)))
        .collect()
}

/// Name of a field as written in sidecars and CSV (`stepName`, …).
pub fn field_name(field: TField) -> String {
    serde_json::to_value(field)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default()
}

/// The units as an RFC 4180 CSV (`key,field,context,text`), for the CLI.
pub fn to_csv(units: &[TUnit]) -> String {
    let mut out = String::from("key,field,context,text\n");
    for u in units {
        let field = field_name(u.field);
        for (i, column) in [u.key.as_str(), field.as_str(), u.context.as_str(), u.source.as_str()]
            .iter()
            .enumerate()
        {
            if i > 0 {
                out.push(',');
            }
            push_csv_field(&mut out, column);
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Plugin, PluginGroup, SelectionType, Step};

    fn sample() -> Ximod {
        let mut m = Ximod::new("Aurelia");
        m.author = "Axl".into();
        m.url = "https://example.org".into();
        m.description = "Big mod\nwith lines".into();
        let mut step = Step::new("Textures");
        let mut group = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        let mut p = Plugin::new("4K");
        p.description = "High resolution.".into();
        group.plugins.push(p);
        group.plugins.push(Plugin::new("2K"));
        step.plugin_groups.push(group);
        let mut g2 = PluginGroup::new("Extras", SelectionType::SelectAny);
        let mut cloak = Plugin::new("Cloak");
        cloak.description = "high resolution.".into();
        g2.plugins.push(cloak);
        step.plugin_groups.push(g2);
        m.steps.push(step);
        m.steps.push(Step::new("Textures"));
        m
    }

    /// The model units carry exactly the keys, fields, sources and contexts
    /// that the extractor finds in the saved XML.
    #[test]
    fn model_units_round_trip_through_the_xml_writers() {
        let m = sample();
        let info = crate::xml::info_xml_to_string(&m).unwrap();
        let config = crate::xml::module_config_to_string(&m).unwrap();
        let extracted = crate::xml::patch::extract_units(Some(&info), Some(&config));
        let ours = model_units(&m);
        let view = |v: &[TUnit]| -> Vec<(String, TField, String, String)> {
            v.iter()
                .map(|u| (u.key.clone(), u.field, u.source.clone(), u.context.clone()))
                .collect()
        };
        assert_eq!(view(&ours), view(&extracted));
        assert!(
            ours.iter()
                .any(|u| u.key == "config/step[0]/group[1]/plugin[0]/description")
        );
        assert_eq!(ours.iter().filter(|u| u.key == "config/step[1]/name").count(), 1);
        // Empty strings produce no unit.
        let mut m2 = m.clone();
        m2.author.clear();
        assert!(model_units(&m2).iter().all(|u| u.key != "info/Author"));
    }

    #[test]
    fn set_unit_writes_the_model_and_rejects_missing_nodes() {
        let mut m = sample();
        assert!(set_unit(
            &mut m,
            "config/step[0]/group[0]/plugin[1]/name",
            TField::PluginName,
            "2K Lite"
        ));
        assert_eq!(m.steps[0].plugin_groups[0].plugins[1].name, "2K Lite");
        assert!(set_unit(&mut m, "info/Description", TField::InfoDescription, "New"));
        assert_eq!(m.description, "New");
        assert!(set_unit(&mut m, "config/moduleName", TField::ModuleName, "Aurelia 2"));
        assert_eq!(m.name, "Aurelia 2");
        assert!(!set_unit(&mut m, "config/step[7]/name", TField::StepName, "x"));
        assert!(!set_unit(&mut m, "config/step[0]/name", TField::GroupName, "x"));
    }

    #[test]
    fn duplicates_ignore_case_whitespace_and_the_name_mirrors() {
        let units = model_units(&sample());
        let d = duplicates(&units);
        // "Textures" (two steps, plus the breadcrumb-free names) and
        // "High resolution." / "high resolution." are duplicated; the mod name
        // mirrors alone are not a group.
        assert_eq!(d.len(), 2, "{d:?}");
        let textures = &d["textures"];
        assert_eq!(textures.len(), 2);
        assert!(textures.iter().all(|&i| units[i].field == TField::StepName));
        assert_eq!(d["high resolution."].len(), 2);
        assert_eq!(normalize("  Big\n  Mod "), "big mod");
    }

    #[test]
    fn replace_handles_case_and_whole_words() {
        assert_eq!(
            replace_in("Cat cat concat", "cat", "dog", true, false),
            Some("Cat dog condog".into())
        );
        assert_eq!(
            replace_in("Cat cat concat", "cat", "dog", false, false),
            Some("dog dog condog".into())
        );
        assert_eq!(
            replace_in("Cat cat concat", "cat", "dog", false, true),
            Some("dog dog concat".into())
        );
        assert_eq!(replace_in("nothing here", "cat", "dog", false, false), None);
        assert_eq!(replace_in("x", "", "dog", false, false), None);
        assert_eq!(
            replace_in("Élan élan", "élan", "elk", false, true),
            Some("elk elk".into())
        );
        let units = model_units(&sample());
        let all = replace_all(&units, "resolution", "res", false, false, None);
        assert_eq!(all.len(), 3); // group name + two descriptions
        let only = replace_all(&units, "resolution", "res", false, false, Some(&[TField::GroupName]));
        assert_eq!(only.len(), 1);
        assert_eq!(only[0].1, "res");
    }

    #[test]
    fn csv_quotes_what_needs_it() {
        let csv = to_csv(&model_units(&sample()));
        let first = csv.lines().next().unwrap();
        assert_eq!(first, "key,field,context,text");
        assert!(csv.contains("info/Name,infoName,,Aurelia\n"));
        assert!(csv.contains("\"Big mod\nwith lines\""));
        assert!(csv.contains("config/step[0]/group[0]/plugin[0]/description,pluginDescription,Textures › Resolution › 4K,High resolution.\n"));
    }
}
