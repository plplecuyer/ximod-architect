//! Preset wizard: generates the "preset + customise" pattern of the user manual
//! (§ 7.5) on top of existing feature pages.
//!
//! The idea is to separate deciding from installing:
//!
//! * a new first step, the *installation mode*, holds a **Preset** group (one
//!   option per preset, which only sets flags) and a **Customise** group (one
//!   option per feature page, which opens that page and resets its flags);
//! * every feature page becomes visible only when its Customise option is
//!   ticked; each of its options sets the feature's flag and is marked
//!   *Recommended* — pre-ticked — when a preset that chose it is active;
//! * the files of those options move to conditional installs keyed on the
//!   final flag values, so one flag per feature (or per option in a
//!   multi-selection group) is the single source of truth and nothing is ever
//!   installed twice.
//!
//! Exclusive groups (*SelectExactlyOne* / *SelectAtMostOne*) get one flag with
//! one value per option (plus `none`); multi-selection groups get one `on/off`
//! flag per option. Flag assignments are evaluated page after page by the mod
//! managers, so the preset (first page) sets the defaults and a feature page
//! shown later overrides them.

use crate::models::{
    ConditionFlag, ConditionalFileSet, Dependency, DependencyGroup, DependencyPattern, LogicalOperator, Plugin,
    PluginGroup, PluginType, SelectionType, Step, Ximod,
};
use std::collections::HashSet;

/// Names and switches of the generated structure. All names are free text
/// (the UI seeds them from the interface language).
#[derive(Debug, Clone)]
pub struct PresetSettings {
    /// Name of the new first step ("Installation mode").
    pub mode_step_name: String,
    /// Name of the group holding the presets ("Preset").
    pub preset_group_name: String,
    /// Name of the group holding the "Customise …" options ("Customise").
    pub customise_group_name: String,
    /// Label of a Customise option; `{name}` is replaced by the page name.
    pub customise_label: String,
    /// Name of the flag that records the chosen preset ("preset").
    pub preset_flag: String,
    /// Name of the "None" option added to exclusive groups.
    pub none_name: String,
    /// Whether to add that "None" option to exclusive groups.
    pub add_none: bool,
}

impl Default for PresetSettings {
    fn default() -> Self {
        Self {
            mode_step_name: "Installation mode".into(),
            preset_group_name: "Preset".into(),
            customise_group_name: "Customise".into(),
            customise_label: "Customise {name}".into(),
            preset_flag: "preset".into(),
            none_name: "None".into(),
            add_none: true,
        }
    }
}

/// What a preset chooses in one group of one feature page.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GroupChoice {
    /// Nothing from this group.
    #[default]
    None,
    /// One option (exclusive groups) — its index in the group.
    One(usize),
    /// Several options (multi-selection groups) — their indices.
    Many(Vec<usize>),
}

/// One preset: a name and, for every feature page and every group of it, a
/// choice. `choices[f][g]` follows the order of [`PresetPlan::feature_steps`]
/// and of the groups of that step.
#[derive(Debug, Clone, Default)]
pub struct PresetDef {
    pub name: String,
    pub choices: Vec<Vec<GroupChoice>>,
}

/// Everything the wizard needs to transform a project.
#[derive(Debug, Clone)]
pub struct PresetPlan {
    pub settings: PresetSettings,
    /// Indices of the steps that are feature pages, in the project's order.
    pub feature_steps: Vec<usize>,
    pub presets: Vec<PresetDef>,
}

/// Why a plan cannot be applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresetError {
    NoFeature,
    NoPreset,
    EmptyName,
    DuplicatePreset(String),
    BadStep(usize),
    /// A chosen option index does not exist in its group.
    BadChoice {
        feature: usize,
        group: usize,
    },
}

/// What the transformation did (for the status message).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PresetSummary {
    pub features: usize,
    pub groups: usize,
    pub options: usize,
    pub conditional_sets: usize,
    pub none_options: usize,
}

/// A feature page to create from scratch (skeleton mode).
#[derive(Debug, Clone)]
pub struct FeatureSpec {
    pub name: String,
    /// `true` → one exclusive group (*SelectExactlyOne*); `false` → one
    /// multi-selection group (*SelectAny*).
    pub exclusive: bool,
    pub variants: Vec<String>,
}

/// Turn free text into a flag-friendly identifier: letters and digits kept,
/// everything else collapsed to one `_`, never empty.
pub fn sanitize(s: &str) -> String {
    let mut out = String::new();
    let mut last_sep = true;
    for c in s.chars() {
        if c.is_alphanumeric() {
            out.push(c);
            last_sep = false;
        } else if !last_sep {
            out.push('_');
            last_sep = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out.is_empty() { "x".to_string() } else { out }
}

/// `base`, or `base_2`, `base_3`… — the first not in `taken`; records it.
fn unique(base: &str, taken: &mut HashSet<String>) -> String {
    let mut name = base.to_string();
    let mut n = 2;
    while taken.contains(&name) {
        name = format!("{base}_{n}");
        n += 1;
    }
    taken.insert(name.clone());
    name
}

fn is_exclusive(t: SelectionType) -> bool {
    matches!(t, SelectionType::SelectExactlyOne | SelectionType::SelectAtMostOne)
}

/// `flag == value` as a one-leaf And group.
fn flag_is(flag: &str, value: &str) -> DependencyGroup {
    DependencyGroup::from_leaves(LogicalOperator::And, vec![Dependency::new_flag(flag, value)])
}

/// `preset == v1 OR preset == v2 …` (a single leaf when there is one value).
fn preset_in(preset_flag: &str, values: &[&str]) -> DependencyGroup {
    let leaves = values.iter().map(|v| Dependency::new_flag(preset_flag, *v)).collect();
    let op = if values.len() > 1 {
        LogicalOperator::Or
    } else {
        LogicalOperator::And
    };
    DependencyGroup::from_leaves(op, leaves)
}

/// Add `dep` to a step's visibility: pushed into an And group, or combined
/// with an existing Or group under a new And root.
fn require_in_visibility(vis: &mut DependencyGroup, dep: Dependency) {
    if vis.is_empty() || vis.operator == LogicalOperator::And {
        vis.operator = LogicalOperator::And;
        vis.push_leaf(dep);
    } else {
        let old = std::mem::take(vis);
        let mut root = DependencyGroup::new(LogicalOperator::And);
        root.push_group(old);
        root.push_leaf(dep);
        *vis = root;
    }
}

/// Check the plan against the project without changing anything.
pub fn validate(ximod: &Ximod, plan: &PresetPlan) -> Result<(), PresetError> {
    if plan.feature_steps.is_empty() {
        return Err(PresetError::NoFeature);
    }
    if plan.presets.is_empty() {
        return Err(PresetError::NoPreset);
    }
    let s = &plan.settings;
    if [
        &s.mode_step_name,
        &s.preset_group_name,
        &s.customise_group_name,
        &s.customise_label,
        &s.preset_flag,
        &s.none_name,
    ]
    .iter()
    .any(|t| t.trim().is_empty())
        || plan.presets.iter().any(|p| p.name.trim().is_empty())
    {
        return Err(PresetError::EmptyName);
    }
    let mut seen_steps = HashSet::new();
    for &si in &plan.feature_steps {
        if si >= ximod.steps.len() || !seen_steps.insert(si) {
            return Err(PresetError::BadStep(si));
        }
    }
    let mut seen_presets = HashSet::new();
    for p in &plan.presets {
        if !seen_presets.insert(sanitize(&p.name)) {
            return Err(PresetError::DuplicatePreset(p.name.clone()));
        }
        for (f, &si) in plan.feature_steps.iter().enumerate() {
            let step = &ximod.steps[si];
            for (g, group) in step.plugin_groups.iter().enumerate() {
                let choice = p.choices.get(f).and_then(|c| c.get(g)).cloned().unwrap_or_default();
                let ok = match &choice {
                    GroupChoice::None => true,
                    GroupChoice::One(pi) => *pi < group.plugins.len(),
                    GroupChoice::Many(v) => v.iter().all(|pi| *pi < group.plugins.len()),
                };
                if !ok {
                    return Err(PresetError::BadChoice { feature: f, group: g });
                }
            }
        }
    }
    Ok(())
}

/// Apply the plan to the project. On success the project holds the new mode
/// step first, the feature pages are conditional and flagged, and their files
/// moved to conditional installs. On error nothing is changed.
pub fn apply(ximod: &mut Ximod, plan: &PresetPlan) -> Result<PresetSummary, PresetError> {
    validate(ximod, plan)?;
    let s = &plan.settings;
    let mut summary = PresetSummary::default();

    // Flag names must not collide with those already used by the project.
    let mut taken: HashSet<String> = ximod.get_all_flags().into_iter().collect();
    let preset_flag = unique(&s.preset_flag, &mut taken);
    let preset_values: Vec<String> = {
        let mut vals = HashSet::new();
        plan.presets
            .iter()
            .map(|p| unique(&sanitize(&p.name), &mut vals))
            .collect()
    };

    // Per feature page: its key, its customise flag, and per group the flag
    // layout (exclusive: one flag + values; multi: one flag per option).
    struct GroupFlags {
        exclusive: bool,
        flag: String,              // exclusive groups
        values: Vec<String>,       // exclusive groups: one value per option
        option_flags: Vec<String>, // multi groups: one flag per option
    }
    struct FeatureFlags {
        cust_flag: String,
        groups: Vec<GroupFlags>,
    }
    let mut keys = HashSet::new();
    let mut features: Vec<FeatureFlags> = Vec::new();
    for &si in &plan.feature_steps {
        let step = &ximod.steps[si];
        let key = unique(&sanitize(&step.name), &mut keys);
        let cust_flag = unique(&format!("cust_{key}"), &mut taken);
        let single = step.plugin_groups.len() == 1;
        let mut groups = Vec::new();
        for group in &step.plugin_groups {
            let gkey = if single {
                key.clone()
            } else {
                format!("{key}_{}", sanitize(&group.name))
            };
            if is_exclusive(group.selection_type) {
                let flag = unique(&format!("feat_{gkey}"), &mut taken);
                let mut vals: HashSet<String> = HashSet::from(["none".to_string()]);
                let values = group
                    .plugins
                    .iter()
                    .map(|p| unique(&sanitize(&p.name), &mut vals))
                    .collect();
                groups.push(GroupFlags {
                    exclusive: true,
                    flag,
                    values,
                    option_flags: Vec::new(),
                });
            } else {
                let option_flags = group
                    .plugins
                    .iter()
                    .map(|p| unique(&format!("opt_{gkey}_{}", sanitize(&p.name)), &mut taken))
                    .collect();
                groups.push(GroupFlags {
                    exclusive: false,
                    flag: String::new(),
                    values: Vec::new(),
                    option_flags,
                });
            }
        }
        features.push(FeatureFlags { cust_flag, groups });
    }

    // The choice of every preset for feature f / group g, normalised.
    let choice = |p: &PresetDef, f: usize, g: usize| -> GroupChoice {
        p.choices.get(f).and_then(|c| c.get(g)).cloned().unwrap_or_default()
    };

    // ---- 1) Feature pages: flags, dynamic types, files → conditional installs
    let mut new_sets: Vec<ConditionalFileSet> = Vec::new();
    for (f, &si) in plan.feature_steps.iter().enumerate() {
        let ff = &features[f];
        let step = &mut ximod.steps[si];
        summary.features += 1;
        require_in_visibility(&mut step.visibility, Dependency::new_flag(&ff.cust_flag, "on"));
        for (g, group) in step.plugin_groups.iter_mut().enumerate() {
            let gf = &ff.groups[g];
            summary.groups += 1;
            for (pi, plugin) in group.plugins.iter_mut().enumerate() {
                summary.options += 1;
                let (flag, value) = if gf.exclusive {
                    (gf.flag.as_str(), gf.values[pi].as_str())
                } else {
                    (gf.option_flags[pi].as_str(), "on")
                };
                plugin.condition_flags.push(ConditionFlag::new(flag, value));
                if !plugin.files.is_empty() {
                    new_sets.push(ConditionalFileSet {
                        condition: flag_is(flag, value),
                        files: std::mem::take(&mut plugin.files),
                    });
                }
                // Recommended under every preset that chose this option.
                let choosers: Vec<&str> = plan
                    .presets
                    .iter()
                    .enumerate()
                    .filter(|(k, p)| match choice(p, f, g) {
                        GroupChoice::One(x) => x == pi,
                        GroupChoice::Many(v) => v.contains(&pi),
                        GroupChoice::None => false,
                    } && *k < preset_values.len())
                    .map(|(k, _)| preset_values[k].as_str())
                    .collect();
                if !choosers.is_empty() {
                    plugin.dependency_patterns.push(DependencyPattern {
                        pattern_type: PluginType::Recommended.as_str().to_string(),
                        condition: preset_in(&preset_flag, &choosers),
                    });
                }
            }
            if gf.exclusive && s.add_none {
                // "None": recommended by default, optional under the presets
                // that pick something in this group.
                let mut none = Plugin::new(s.none_name.clone());
                none.default_type = PluginType::Recommended;
                none.condition_flags.push(ConditionFlag::new(&gf.flag, "none"));
                let pickers: Vec<&str> = plan
                    .presets
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !matches!(choice(p, f, g), GroupChoice::None))
                    .map(|(k, _)| preset_values[k].as_str())
                    .collect();
                if !pickers.is_empty() {
                    none.dependency_patterns.push(DependencyPattern {
                        pattern_type: PluginType::Optional.as_str().to_string(),
                        condition: preset_in(&preset_flag, &pickers),
                    });
                }
                group.plugins.insert(0, none);
                summary.none_options += 1;
            }
        }
    }
    summary.conditional_sets = new_sets.len();
    ximod.conditional_files.extend(new_sets);

    // ---- 2) The installation-mode step
    let mut preset_group = PluginGroup::new(s.preset_group_name.clone(), SelectionType::SelectAtMostOne);
    for (k, p) in plan.presets.iter().enumerate() {
        let mut opt = Plugin::new(p.name.clone());
        opt.condition_flags
            .push(ConditionFlag::new(&preset_flag, &preset_values[k]));
        let mut described: Vec<String> = Vec::new();
        for (f, &si) in plan.feature_steps.iter().enumerate() {
            let step = &ximod.steps[si];
            let ff = &features[f];
            for (g, group) in step.plugin_groups.iter().enumerate() {
                let gf = &ff.groups[g];
                // Option indices refer to the page before "None" was inserted.
                let shift = usize::from(gf.exclusive && s.add_none);
                match choice(p, f, g) {
                    GroupChoice::None => {
                        if gf.exclusive {
                            opt.condition_flags.push(ConditionFlag::new(&gf.flag, "none"));
                        }
                    }
                    GroupChoice::One(pi) => {
                        if gf.exclusive {
                            opt.condition_flags.push(ConditionFlag::new(&gf.flag, &gf.values[pi]));
                        } else {
                            opt.condition_flags.push(ConditionFlag::new(&gf.option_flags[pi], "on"));
                        }
                        described.push(format!("{}: {}", step.name, group.plugins[pi + shift].name));
                    }
                    GroupChoice::Many(v) => {
                        let mut names = Vec::new();
                        for pi in v {
                            if gf.exclusive {
                                opt.condition_flags.push(ConditionFlag::new(&gf.flag, &gf.values[pi]));
                            } else {
                                opt.condition_flags.push(ConditionFlag::new(&gf.option_flags[pi], "on"));
                            }
                            names.push(group.plugins[pi + shift].name.clone());
                        }
                        if !names.is_empty() {
                            described.push(format!("{}: {}", step.name, names.join(", ")));
                        }
                    }
                }
            }
        }
        opt.description = described.join(" · ");
        preset_group.plugins.push(opt);
    }
    let mut customise_group = PluginGroup::new(s.customise_group_name.clone(), SelectionType::SelectAny);
    for (f, &si) in plan.feature_steps.iter().enumerate() {
        let ff = &features[f];
        let step_name = ximod.steps[si].name.clone();
        let mut opt = Plugin::new(s.customise_label.replace("{name}", &step_name));
        opt.condition_flags.push(ConditionFlag::new(&ff.cust_flag, "on"));
        // Reset the page's flags so that what the user ticks there is the
        // whole truth (an unticked multi option must not keep the preset's "on").
        for gf in &ff.groups {
            if gf.exclusive {
                opt.condition_flags.push(ConditionFlag::new(&gf.flag, "none"));
            } else {
                for of in &gf.option_flags {
                    opt.condition_flags.push(ConditionFlag::new(of, "off"));
                }
            }
        }
        customise_group.plugins.push(opt);
    }
    let mut mode = Step::new(s.mode_step_name.clone());
    mode.plugin_groups.push(preset_group);
    mode.plugin_groups.push(customise_group);
    ximod.steps.insert(0, mode);

    Ok(summary)
}

/// `group` is a single flag test `name == value` (what [`flag_is`] builds).
fn single_flag(group: &DependencyGroup) -> Option<(&str, &str)> {
    if group.items.len() != 1 {
        return None;
    }
    let d = group.items[0].as_leaf()?;
    d.is_flag().then_some((d.name.as_str(), d.value.as_str()))
}

/// Remove the leaf `flag == value` from a step's visibility, undoing
/// [`require_in_visibility`] (including the And wrapper it may have added).
fn release_visibility(vis: &mut DependencyGroup, flag: &str, value: &str) {
    vis.items
        .retain(|it| !matches!(it.as_leaf(), Some(d) if d.is_flag() && d.name == flag && d.value == value));
    if vis.operator == LogicalOperator::And
        && vis.items.len() == 1
        && let Some(inner) = vis.items[0].as_group().cloned()
    {
        *vis = inner;
    }
}

/// The installation-mode step built by an earlier run of the wizard, if any:
/// a step whose first group holds options that all set the same flag to
/// distinct values (the presets) and whose second group holds options that
/// each set a flag to "on" which some other step needs to be visible (the
/// Customise options).
fn find_mode_step(ximod: &Ximod) -> Option<usize> {
    ximod.steps.iter().position(|step| {
        if step.plugin_groups.len() != 2 {
            return false;
        }
        let (pg, cg) = (&step.plugin_groups[0], &step.plugin_groups[1]);
        if pg.plugins.is_empty() || cg.plugins.is_empty() {
            return false;
        }
        let pf = match pg.plugins[0].condition_flags.first() {
            Some(f) => f.name.clone(),
            None => return false,
        };
        let mut values = HashSet::new();
        let presets_ok = pg
            .plugins
            .iter()
            .all(|p| matches!(p.condition_flags.first(), Some(f) if f.name == pf && values.insert(f.value.clone())));
        let customise_ok = cg.plugins.iter().all(|o| match o.condition_flags.first() {
            Some(f) if f.value == "on" => ximod.steps.iter().any(|s| {
                s.visibility
                    .items
                    .iter()
                    .filter_map(|it| it.as_leaf())
                    .any(|d| d.is_flag() && d.name == f.name && d.value == "on")
            }),
            _ => false,
        });
        presets_ok && customise_ok
    })
}

/// Whether the project holds a structure built by an earlier run of the
/// wizard (see [`unapply`]).
#[cfg(test)]
pub fn has_structure(ximod: &Ximod) -> bool {
    find_mode_step(ximod).is_some()
}

/// Undo an earlier run of the wizard and return the plan that would rebuild
/// it: the installation-mode step is removed, the feature pages lose their
/// visibility condition, their "None" options and the flags and Recommended
/// patterns the wizard added, and the files the wizard moved to conditional
/// installs go back to their options. `None` when the project holds no such
/// structure (nothing is changed then).
///
/// Running the wizard again therefore never stacks a second structure on the
/// first: the caller unapplies, lets the author add or change presets, and
/// applies the new plan — which regenerates the same flag names, so a mod
/// update keeps working with the flags of the previous release.
pub fn unapply(ximod: &mut Ximod) -> Option<PresetPlan> {
    let mi = find_mode_step(ximod)?;
    let mode = ximod.steps[mi].clone();
    let (pg, cg) = (&mode.plugin_groups[0], &mode.plugin_groups[1]);
    let preset_flag = pg.plugins[0].condition_flags[0].name.clone();

    // Feature pages, in the order of their Customise options.
    struct Feature {
        step: usize,
        cust_flag: String,
        /// Flags the Customise option resets: the wizard's flags of the page.
        reset: HashSet<String>,
    }
    let mut features: Vec<Feature> = Vec::new();
    for o in &cg.plugins {
        let cust = &o.condition_flags[0].name;
        let Some(si) = ximod.steps.iter().position(|s| {
            s.visibility
                .items
                .iter()
                .filter_map(|it| it.as_leaf())
                .any(|d| d.is_flag() && &d.name == cust && d.value == "on")
        }) else {
            continue;
        };
        if si == mi || features.iter().any(|f| f.step == si) {
            continue;
        }
        features.push(Feature {
            step: si,
            cust_flag: cust.clone(),
            reset: o.condition_flags.iter().skip(1).map(|f| f.name.clone()).collect(),
        });
    }

    let mut settings = PresetSettings {
        mode_step_name: mode.name.clone(),
        preset_group_name: pg.name.clone(),
        customise_group_name: cg.name.clone(),
        customise_label: "{name}".into(),
        preset_flag: preset_flag.clone(),
        none_name: PresetSettings::default().none_name,
        add_none: false,
    };
    if let (Some(f), Some(o)) = (features.first(), cg.plugins.first()) {
        let page = &ximod.steps[f.step].name;
        if !page.is_empty() && o.name.contains(page.as_str()) {
            settings.customise_label = o.name.replacen(page.as_str(), "{name}", 1);
        }
    }

    // Per feature / group: the wizard flag and value of every remaining
    // option, to read the presets' choices back.
    #[derive(Clone)]
    struct GroupInfo {
        exclusive: bool,
        flag: Option<String>,
        option_flags: Vec<(String, String)>,
    }
    let mut infos: Vec<Vec<GroupInfo>> = Vec::new();
    let all_preset =
        |g: &DependencyGroup| g.leaf_count() > 0 && g.leaves().all(|d| d.is_flag() && d.name == preset_flag);
    for f in &features {
        let mut groups_info = Vec::new();
        let step = &mut ximod.steps[f.step];
        release_visibility(&mut step.visibility, &f.cust_flag, "on");
        for group in &mut step.plugin_groups {
            let exclusive = is_exclusive(group.selection_type);
            // The "None" option: in an exclusive group, the option whose
            // wizard flag is set to "none".
            if exclusive
                && let Some(ni) = group.plugins.iter().position(|p| {
                    p.condition_flags
                        .iter()
                        .any(|c| f.reset.contains(&c.name) && c.value == "none")
                })
            {
                settings.none_name = group.plugins[ni].name.clone();
                settings.add_none = true;
                group.plugins.remove(ni);
            }
            let mut info = GroupInfo {
                exclusive,
                flag: None,
                option_flags: Vec::new(),
            };
            for plugin in &mut group.plugins {
                let pos = plugin.condition_flags.iter().rposition(|c| f.reset.contains(&c.name));
                let wf = pos.map(|i| plugin.condition_flags.remove(i));
                plugin.dependency_patterns.retain(|pat| !all_preset(&pat.condition));
                match wf {
                    Some(c) => {
                        if exclusive {
                            info.flag = Some(c.name.clone());
                        }
                        info.option_flags.push((c.name, c.value));
                    }
                    None => info.option_flags.push((String::new(), String::new())),
                }
            }
            groups_info.push(info);
        }
        infos.push(groups_info);
    }

    // Files moved to conditional installs go back to their options.
    let mut moved: Vec<bool> = vec![false; ximod.conditional_files.len()];
    for (fi, f) in features.iter().enumerate() {
        for (gi, info) in infos[fi].iter().enumerate() {
            for (pi, (flag, value)) in info.option_flags.iter().enumerate() {
                if flag.is_empty() {
                    continue;
                }
                for (ci, set) in ximod.conditional_files.iter().enumerate() {
                    if moved[ci] || single_flag(&set.condition) != Some((flag.as_str(), value.as_str())) {
                        continue;
                    }
                    moved[ci] = true;
                    let files = set.files.clone();
                    ximod.steps[f.step].plugin_groups[gi].plugins[pi].files.extend(files);
                }
            }
        }
    }
    let mut k = 0;
    ximod.conditional_files.retain(|_| {
        let keep = !moved[k];
        k += 1;
        keep
    });

    // The presets' choices, read from their flags.
    let presets = pg
        .plugins
        .iter()
        .map(|p| {
            let set: Vec<(&str, &str)> = p
                .condition_flags
                .iter()
                .skip(1)
                .map(|c| (c.name.as_str(), c.value.as_str()))
                .collect();
            let choices = infos
                .iter()
                .map(|groups| {
                    groups
                        .iter()
                        .map(|g| {
                            if g.exclusive {
                                let Some(flag) = &g.flag else { return GroupChoice::None };
                                match set.iter().find(|(n, _)| n == flag) {
                                    Some((_, v)) if *v != "none" => g
                                        .option_flags
                                        .iter()
                                        .position(|(n, val)| n == flag && val == v)
                                        .map(GroupChoice::One)
                                        .unwrap_or_default(),
                                    _ => GroupChoice::None,
                                }
                            } else {
                                let on: Vec<usize> = g
                                    .option_flags
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, (n, _))| !n.is_empty() && set.contains(&(n.as_str(), "on")))
                                    .map(|(i, _)| i)
                                    .collect();
                                if on.is_empty() {
                                    GroupChoice::None
                                } else {
                                    GroupChoice::Many(on)
                                }
                            }
                        })
                        .collect()
                })
                .collect();
            PresetDef {
                name: p.name.clone(),
                choices,
            }
        })
        .collect();

    ximod.steps.remove(mi);
    let feature_steps = features
        .iter()
        .map(|f| if f.step > mi { f.step - 1 } else { f.step })
        .collect();
    Some(PresetPlan {
        settings,
        feature_steps,
        presets,
    })
}

/// Skeleton mode: append one step per feature, holding one group with one
/// (file-less) option per variant. Returns the indices of the new steps, so
/// the caller can feed them to [`PresetPlan::feature_steps`].
pub fn create_feature_pages(ximod: &mut Ximod, features: &[FeatureSpec]) -> Vec<usize> {
    let mut created = Vec::new();
    for f in features {
        let name = f.name.trim();
        if name.is_empty() {
            continue;
        }
        let mut step = Step::new(name);
        let kind = if f.exclusive {
            SelectionType::SelectExactlyOne
        } else {
            SelectionType::SelectAny
        };
        let mut group = PluginGroup::new(name, kind);
        for v in &f.variants {
            let v = v.trim();
            if !v.is_empty() {
                group.plugins.push(Plugin::new(v));
            }
        }
        step.plugin_groups.push(group);
        created.push(ximod.steps.len());
        ximod.steps.push(step);
    }
    created
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::InstallFile;

    fn leaves(g: &DependencyGroup) -> Vec<&Dependency> {
        g.leaves().collect()
    }

    fn sample() -> Ximod {
        let mut x = Ximod::new("Sample");
        // Feature A: exclusive, two variants with files.
        let mut a = Step::new("Feature A");
        let mut ga = PluginGroup::new("Feature A", SelectionType::SelectExactlyOne);
        for v in ["Version 1", "Version 2"] {
            let mut p = Plugin::new(v);
            p.files.push(InstallFile::new_folder(format!("FeatureA/{v}")));
            ga.plugins.push(p);
        }
        a.plugin_groups.push(ga);
        // Feature B: multi, two add-ons (one without files).
        let mut b = Step::new("Feature B");
        let mut gb = PluginGroup::new("Add-ons", SelectionType::SelectAny);
        let mut x1 = Plugin::new("Extra meshes");
        x1.files.push(InstallFile::new_folder("FeatureB/meshes"));
        gb.plugins.push(x1);
        gb.plugins.push(Plugin::new("Extra sounds"));
        b.plugin_groups.push(gb);
        x.steps.push(a);
        x.steps.push(b);
        x
    }

    fn plan() -> PresetPlan {
        PresetPlan {
            settings: PresetSettings::default(),
            feature_steps: vec![0, 1],
            presets: vec![
                PresetDef {
                    name: "Full".into(),
                    choices: vec![vec![GroupChoice::One(1)], vec![GroupChoice::Many(vec![0, 1])]],
                },
                PresetDef {
                    name: "Light".into(),
                    choices: vec![vec![GroupChoice::One(0)], vec![GroupChoice::None]],
                },
            ],
        }
    }

    #[test]
    fn sanitize_keeps_identifiers_readable() {
        assert_eq!(sanitize("Feature A"), "Feature_A");
        assert_eq!(sanitize("  Hi-Res (4K) textures! "), "Hi_Res_4K_textures");
        assert_eq!(sanitize("Éclairage été"), "Éclairage_été");
        assert_eq!(sanitize("***"), "x");
        let mut taken = HashSet::new();
        assert_eq!(unique("a", &mut taken), "a");
        assert_eq!(unique("a", &mut taken), "a_2");
        assert_eq!(unique("a", &mut taken), "a_3");
    }

    #[test]
    fn validation_catches_bad_plans() {
        let x = sample();
        let mut p = plan();
        p.feature_steps.clear();
        assert_eq!(validate(&x, &p), Err(PresetError::NoFeature));
        let mut p = plan();
        p.presets.clear();
        assert_eq!(validate(&x, &p), Err(PresetError::NoPreset));
        let mut p = plan();
        p.presets[1].name = "Full".into();
        assert_eq!(validate(&x, &p), Err(PresetError::DuplicatePreset("Full".into())));
        let mut p = plan();
        p.feature_steps = vec![0, 7];
        assert_eq!(validate(&x, &p), Err(PresetError::BadStep(7)));
        let mut p = plan();
        p.presets[0].choices[0][0] = GroupChoice::One(5);
        assert_eq!(validate(&x, &p), Err(PresetError::BadChoice { feature: 0, group: 0 }));
        let mut p = plan();
        p.settings.preset_flag = "  ".into();
        assert_eq!(validate(&x, &p), Err(PresetError::EmptyName));
        assert_eq!(validate(&x, &plan()), Ok(()));
    }

    #[test]
    fn apply_builds_the_pattern() {
        let mut x = sample();
        let summary = apply(&mut x, &plan()).unwrap();
        assert_eq!(
            summary,
            PresetSummary {
                features: 2,
                groups: 2,
                options: 4,
                conditional_sets: 3,
                none_options: 1
            }
        );

        // The mode step comes first with the two groups.
        assert_eq!(x.steps.len(), 3);
        let mode = &x.steps[0];
        assert_eq!(mode.name, "Installation mode");
        assert_eq!(mode.plugin_groups[0].name, "Preset");
        assert_eq!(mode.plugin_groups[0].selection_type, SelectionType::SelectAtMostOne);
        assert_eq!(mode.plugin_groups[1].name, "Customise");
        let full = &mode.plugin_groups[0].plugins[0];
        assert_eq!(full.name, "Full");
        assert!(full.files.is_empty());
        let flags: Vec<(&str, &str)> = full
            .condition_flags
            .iter()
            .map(|f| (f.name.as_str(), f.value.as_str()))
            .collect();
        assert_eq!(
            flags,
            vec![
                ("preset", "Full"),
                ("feat_Feature_A", "Version_2"),
                ("opt_Feature_B_Extra_meshes", "on"),
                ("opt_Feature_B_Extra_sounds", "on"),
            ]
        );
        assert_eq!(
            full.description,
            "Feature A: Version 2 · Feature B: Extra meshes, Extra sounds"
        );
        let light = &mode.plugin_groups[0].plugins[1];
        assert!(
            light
                .condition_flags
                .iter()
                .any(|f| f.name == "feat_Feature_A" && f.value == "Version_1")
        );
        assert!(light.condition_flags.iter().all(|f| !f.name.starts_with("opt_")));

        // Customise options open their page and reset its flags.
        let cust_a = &mode.plugin_groups[1].plugins[0];
        assert_eq!(cust_a.name, "Customise Feature A");
        assert!(
            cust_a
                .condition_flags
                .contains(&ConditionFlag::new("cust_Feature_A", "on"))
        );
        assert!(
            cust_a
                .condition_flags
                .contains(&ConditionFlag::new("feat_Feature_A", "none"))
        );
        let cust_b = &mode.plugin_groups[1].plugins[1];
        assert!(
            cust_b
                .condition_flags
                .contains(&ConditionFlag::new("opt_Feature_B_Extra_meshes", "off"))
        );

        // Feature A: visible on cust flag, None first (recommended, optional
        // under both presets since both pick something), variants flagged,
        // Version 2 recommended under Full only, files moved.
        let a = &x.steps[1];
        assert_eq!(leaves(&a.visibility).len(), 1);
        assert_eq!(leaves(&a.visibility)[0].name, "cust_Feature_A");
        let ga = &a.plugin_groups[0];
        assert_eq!(ga.plugins[0].name, "None");
        assert_eq!(ga.plugins[0].default_type, PluginType::Recommended);
        assert_eq!(ga.plugins[0].dependency_patterns[0].pattern_type, "Optional");
        assert_eq!(leaves(&ga.plugins[0].dependency_patterns[0].condition).len(), 2);
        let v2 = &ga.plugins[2];
        assert_eq!(v2.name, "Version 2");
        assert!(v2.files.is_empty());
        assert_eq!(
            v2.condition_flags,
            vec![ConditionFlag::new("feat_Feature_A", "Version_2")]
        );
        assert_eq!(v2.dependency_patterns.len(), 1);
        assert_eq!(v2.dependency_patterns[0].pattern_type, "Recommended");
        let lv = leaves(&v2.dependency_patterns[0].condition);
        assert_eq!((lv[0].name.as_str(), lv[0].value.as_str()), ("preset", "Full"));

        // Feature B: per-option flags; the file-less option got no cond set.
        let gb = &x.steps[2].plugin_groups[0];
        assert_eq!(gb.plugins.len(), 2, "no None in a multi group");
        assert_eq!(
            gb.plugins[1].condition_flags,
            vec![ConditionFlag::new("opt_Feature_B_Extra_sounds", "on")]
        );
        assert_eq!(x.conditional_files.len(), 3);
        let set = &x.conditional_files[2];
        assert_eq!(leaves(&set.condition)[0].name, "opt_Feature_B_Extra_meshes");
        assert_eq!(set.files[0].source, "FeatureB/meshes");
    }

    #[test]
    fn apply_leaves_the_project_untouched_on_error() {
        let mut x = sample();
        let before = crate::xml::module_config_to_string(&x).unwrap();
        let mut p = plan();
        p.presets.clear();
        assert!(apply(&mut x, &p).is_err());
        assert_eq!(crate::xml::module_config_to_string(&x).unwrap(), before);
    }

    #[test]
    fn generated_structure_survives_xml_round_trip() {
        let mut x = sample();
        apply(&mut x, &plan()).unwrap();
        let xml = crate::xml::module_config_to_string(&x).unwrap();
        let mut back = Ximod::new("Back");
        crate::xml::parse_module_config_xml(&xml, &mut back).unwrap();
        assert_eq!(back.steps.len(), 3);
        assert_eq!(back.steps[0].plugin_groups[0].plugins.len(), 2);
        assert_eq!(back.steps[1].plugin_groups[0].plugins.len(), 3);
        assert_eq!(back.conditional_files.len(), 3);
        assert_eq!(back.steps[1].plugin_groups[0].plugins[2].dependency_patterns.len(), 1);
        assert_eq!(leaves(&back.steps[1].visibility).len(), 1);
    }

    #[test]
    fn existing_visibility_and_flag_names_are_respected() {
        let mut x = sample();
        // Feature A already visible on an Or group, and "preset" already used.
        x.steps[0].visibility = DependencyGroup::from_leaves(
            LogicalOperator::Or,
            vec![Dependency::new_flag("a", "1"), Dependency::new_flag("b", "1")],
        );
        x.steps[0].plugin_groups[0].plugins[0]
            .condition_flags
            .push(ConditionFlag::new("preset", "legacy"));
        apply(&mut x, &plan()).unwrap();
        let vis = &x.steps[1].visibility;
        assert_eq!(vis.operator, LogicalOperator::And);
        assert_eq!(vis.items.len(), 2, "old Or group + the cust leaf");
        let mode_flags: Vec<&str> = x.steps[0].plugin_groups[0].plugins[0]
            .condition_flags
            .iter()
            .map(|f| f.name.as_str())
            .collect();
        assert!(mode_flags.contains(&"preset_2"), "{mode_flags:?}");
    }

    #[test]
    fn skeleton_pages_are_created_and_usable() {
        let mut x = Ximod::new("New");
        let idx = create_feature_pages(
            &mut x,
            &[
                FeatureSpec {
                    name: "Lighting".into(),
                    exclusive: true,
                    variants: vec!["Warm".into(), "Cold".into(), "".into()],
                },
                FeatureSpec {
                    name: "  ".into(),
                    exclusive: false,
                    variants: vec!["skipped".into()],
                },
                FeatureSpec {
                    name: "Extras".into(),
                    exclusive: false,
                    variants: vec!["Sounds".into()],
                },
            ],
        );
        assert_eq!(idx, vec![0, 1]);
        assert_eq!(
            x.steps[0].plugin_groups[0].selection_type,
            SelectionType::SelectExactlyOne
        );
        assert_eq!(x.steps[0].plugin_groups[0].plugins.len(), 2);
        assert_eq!(x.steps[1].plugin_groups[0].selection_type, SelectionType::SelectAny);
        let plan = PresetPlan {
            settings: PresetSettings::default(),
            feature_steps: idx,
            presets: vec![PresetDef {
                name: "Default".into(),
                choices: vec![vec![GroupChoice::One(0)], vec![GroupChoice::Many(vec![0])]],
            }],
        };
        let summary = apply(&mut x, &plan).unwrap();
        assert_eq!(summary.conditional_sets, 0, "skeleton options carry no files yet");
        assert_eq!(x.steps.len(), 3);
    }

    fn xml(x: &Ximod) -> String {
        crate::xml::module_config_to_string(x).unwrap()
    }

    #[test]
    fn unapply_restores_the_project_and_returns_the_plan() {
        let original = sample();
        let mut x = original.clone();
        apply(&mut x, &plan()).unwrap();
        assert!(has_structure(&x));
        let back = unapply(&mut x).expect("structure detected");
        assert!(!has_structure(&x));
        assert_eq!(
            xml(&x),
            xml(&original),
            "files, flags, patterns and None options undone"
        );
        assert_eq!(back.feature_steps, vec![0, 1]);
        assert_eq!(back.settings.mode_step_name, "Installation mode");
        assert_eq!(back.settings.customise_label, "Customise {name}");
        assert!(back.settings.add_none);
        assert_eq!(back.presets.len(), 2);
        assert_eq!(back.presets[0].name, "Full");
        assert_eq!(back.presets[0].choices, plan().presets[0].choices);
        assert_eq!(back.presets[1].choices, plan().presets[1].choices);
        // A project without the structure is left alone.
        assert!(unapply(&mut x).is_none());
    }

    #[test]
    fn running_the_wizard_again_rebuilds_instead_of_stacking() {
        let mut x = sample();
        apply(&mut x, &plan()).unwrap();
        let first = x.clone();
        // Second run: one more preset.
        let mut again = unapply(&mut x).unwrap();
        again.presets.push(PresetDef {
            name: "Minimal".into(),
            choices: vec![vec![GroupChoice::None], vec![GroupChoice::None]],
        });
        apply(&mut x, &again).unwrap();
        // Still one installation-mode step, the same page and flag names.
        assert_eq!(x.steps.len(), first.steps.len());
        assert_eq!(x.steps[0].plugin_groups[0].plugins.len(), 3);
        let mut a = first.get_all_flags();
        let mut b = x.get_all_flags();
        a.sort();
        b.sort();
        assert_eq!(a, b, "flag names are kept, so an update of the mod keeps working");
        assert_eq!(x.conditional_files.len(), first.conditional_files.len());
        // The pages got exactly one "None" each.
        let nones = x.steps[1].plugin_groups[0]
            .plugins
            .iter()
            .filter(|p| p.name == "None")
            .count();
        assert_eq!(nones, 1);
    }
}
