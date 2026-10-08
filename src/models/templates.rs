//! Reusable templates (V2 roadmap, priority 6).
//!
//! Authors repeat the same structures across projects: a 1K/2K/4K resolution
//! selector, a patch-choice group, a set of optional variants. A template saves
//! one such structure (a step, a group, or a single option) to disk so it can be
//! re-injected into any project.
//!
//! Templates are stored as JSON, reusing the model's existing `serde` derives, in
//! the user templates directory (`<config>/templates`). They are purely additive:
//! applying a template appends a cloned structure to the project.
//!
//! Status (Lot B): implemented — load/save and apply are functional.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::{Plugin, PluginGroup, SelectionType, Step, Ximod};

/// What a template carries. Each variant reuses an existing model type, so the
/// template file is just that type serialized under a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum TemplateBody {
    /// A whole install step (page) with its groups and options.
    Step(Step),
    /// A single option group.
    Group(PluginGroup),
    /// A single option (plugin).
    Plugin(Plugin),
}

impl TemplateBody {
    /// A short, language-neutral tag for the kind (for UI badges).
    pub fn kind_tag(&self) -> &'static str {
        match self {
            TemplateBody::Step(_) => "step",
            TemplateBody::Group(_) => "group",
            TemplateBody::Plugin(_) => "plugin",
        }
    }
}

/// A named, savable template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Display name (also the basis for the file name).
    pub name: String,
    /// Optional free-text note shown in the picker.
    #[serde(default)]
    pub description: String,
    /// The reusable structure.
    pub body: TemplateBody,
}

/// Default templates directory (`<config>/templates`). `None` if the config dir
/// cannot be resolved on this platform.
pub fn templates_dir() -> Option<PathBuf> {
    crate::config::AppConfig::config_dir().map(|d| d.join("templates"))
}

/// Slugify a template name into a safe file stem.
fn slugify(name: &str) -> String {
    let mut out: String = name
        .trim()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    while out.contains("__") {
        out = out.replace("__", "_");
    }
    let out = out.trim_matches('_').to_string();
    if out.is_empty() { "template".to_string() } else { out }
}

/// Load every template (`*.json`) found in `dir`. Missing dir → empty list.
/// Malformed files are skipped (not fatal).
pub fn load_templates(dir: &Path) -> Result<Vec<Template>> {
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else { continue };
        match serde_json::from_str::<Template>(&text) {
            Ok(t) => out.push(t),
            Err(e) => tracing::warn!("skipping malformed template {}: {e}", path.display()),
        }
    }
    out.sort_by_key(|a| a.name.to_lowercase());
    Ok(out)
}

/// Write `tpl` into `dir` as pretty JSON, returning the file path written. Avoids
/// clobbering a different template by appending a numeric suffix on collision.
pub fn save_template(dir: &Path, tpl: &Template) -> Result<PathBuf> {
    fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    let stem = slugify(&tpl.name);
    let mut path = dir.join(format!("{stem}.json"));
    let mut n = 2;
    while path.exists() {
        path = dir.join(format!("{stem}_{n}.json"));
        n += 1;
    }
    let json = serde_json::to_string_pretty(tpl).context("serializing template")?;
    fs::write(&path, json).with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
}

/// Append the template's structure into `target` at a sensible end location,
/// returning a short language-neutral description of what was added.
///
/// - `Step` → appended to the project's steps.
/// - `Group` → appended to the last step (a step is created if none exists).
/// - `Plugin` → appended to the last group of the last step (group/step created
///   if needed).
pub fn apply_template(target: &mut Ximod, tpl: &Template) -> String {
    match &tpl.body {
        TemplateBody::Step(step) => {
            target.steps.push(step.clone());
            format!("step:{}", target.steps.len())
        }
        TemplateBody::Group(group) => {
            if target.steps.is_empty() {
                target.steps.push(Step::new("Step 1"));
            }
            let last = target.steps.last_mut().unwrap();
            last.plugin_groups.push(group.clone());
            format!("group:{}", last.plugin_groups.len())
        }
        TemplateBody::Plugin(plugin) => {
            if target.steps.is_empty() {
                target.steps.push(Step::new("Step 1"));
            }
            let step = target.steps.last_mut().unwrap();
            if step.plugin_groups.is_empty() {
                step.plugin_groups
                    .push(PluginGroup::new("Group 1", SelectionType::SelectAny));
            }
            let group = step.plugin_groups.last_mut().unwrap();
            group.plugins.push(plugin.clone());
            format!("plugin:{}", group.plugins.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Plugin, PluginGroup, SelectionType, Step};

    #[test]
    fn roundtrips_through_disk() {
        let dir = std::env::temp_dir().join(format!("ximod_tpl_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);

        let mut step = Step::new("Textures");
        let mut g = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        g.plugins.push(Plugin::new("2K"));
        g.plugins.push(Plugin::new("4K"));
        step.plugin_groups.push(g);

        let tpl = Template {
            name: "Resolution selector".into(),
            description: "1K/2K/4K".into(),
            body: TemplateBody::Step(step),
        };
        let p = save_template(&dir, &tpl).unwrap();
        assert!(p.is_file());

        let loaded = load_templates(&dir).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].name, "Resolution selector");
        assert_eq!(loaded[0].body.kind_tag(), "step");

        let _ = fs::remove_dir_all(&dir);
    }

    /// Lot N: a template saved before nested groups existed (flat
    /// `visibility_operator` + `visibility_dependencies` on the step,
    /// `operator` + `dependencies` on a pattern) still loads, and a template
    /// with a nested group round-trips.
    #[test]
    fn old_shape_template_still_loads() {
        use crate::models::{Dependency, DependencyGroup, DependencyItem, LogicalOperator};
        let json = r#"{
  "name": "Old shape",
  "description": "saved by XIMOD 2.0 before lot N",
  "body": {
    "kind": "step",
    "payload": {
      "name": "Textures",
      "visibility_operator": "Or",
      "visibility_dependencies": [
        { "dep_type": "flag", "name": "res", "value": "4K" },
        { "dep_type": "file", "name": "Skyrim.esm", "value": "Active" }
      ],
      "plugin_groups": [
        {
          "name": "G",
          "selection_type": "SelectAny",
          "plugins": [
            {
              "name": "P",
              "description": "",
              "image_path": null,
              "default_type": "Optional",
              "condition_flags": [],
              "files": [],
              "dependency_patterns": [
                {
                  "operator": "And",
                  "pattern_type": "Required",
                  "dependencies": [ { "dep_type": "flag", "name": "x", "value": "1" } ]
                }
              ]
            }
          ]
        }
      ]
    }
  }
}"#;
        let tpl: Template = serde_json::from_str(json).expect("old shape loads");
        let TemplateBody::Step(step) = &tpl.body else {
            panic!("step expected");
        };
        assert_eq!(
            step.visibility,
            DependencyGroup::from_leaves(
                LogicalOperator::Or,
                vec![
                    Dependency::new_flag("res", "4K"),
                    Dependency::new_file("Skyrim.esm", "Active"),
                ]
            )
        );
        let pat = &step.plugin_groups[0].plugins[0].dependency_patterns[0];
        assert_eq!(pat.pattern_type, "Required");
        assert_eq!(
            pat.condition,
            DependencyGroup::from_leaves(LogicalOperator::And, vec![Dependency::new_flag("x", "1")])
        );
        // A step without any visibility field at all (older still).
        let minimal = r#"{"name":"m","body":{"kind":"step","payload":{"name":"S","plugin_groups":[]}}}"#;
        let tpl: Template = serde_json::from_str(minimal).unwrap();
        let TemplateBody::Step(step) = &tpl.body else {
            panic!("step expected");
        };
        assert!(step.visibility.is_empty());

        // New shape with a nested group: JSON round trip.
        let mut step = Step::new("Nested");
        let mut or = DependencyGroup::new(LogicalOperator::Or);
        or.push_leaf(Dependency::new_flag("a", "1"));
        or.push_leaf(Dependency::new_game("1.6"));
        step.visibility.push_group(or);
        step.visibility.push_leaf(Dependency::new_flag("b", "2"));
        let tpl = Template {
            name: "N".into(),
            description: String::new(),
            body: TemplateBody::Step(step.clone()),
        };
        let text = serde_json::to_string_pretty(&tpl).unwrap();
        assert!(text.contains("\"visibility\""), "{text}");
        assert!(!text.contains("visibility_operator"), "{text}");
        let back: Template = serde_json::from_str(&text).unwrap();
        let TemplateBody::Step(back) = back.body else {
            panic!("step expected");
        };
        assert_eq!(back.visibility, step.visibility);
        assert!(matches!(back.visibility.items[0], DependencyItem::Group(_)));
    }

    #[test]
    fn apply_step_appends() {
        let mut x = Ximod::new("M");
        let tpl = Template {
            name: "S".into(),
            description: String::new(),
            body: TemplateBody::Step(Step::new("New")),
        };
        let desc = apply_template(&mut x, &tpl);
        assert_eq!(x.steps.len(), 1);
        assert_eq!(desc, "step:1");
    }

    #[test]
    fn apply_plugin_creates_step_and_group_if_needed() {
        let mut x = Ximod::new("M");
        let tpl = Template {
            name: "P".into(),
            description: String::new(),
            body: TemplateBody::Plugin(Plugin::new("Opt")),
        };
        apply_template(&mut x, &tpl);
        assert_eq!(x.steps.len(), 1);
        assert_eq!(x.steps[0].plugin_groups.len(), 1);
        assert_eq!(x.steps[0].plugin_groups[0].plugins.len(), 1);
    }
}
