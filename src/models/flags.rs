//! Condition-flag analysis shared by the visual condition editor and the
//! project validation: for every flag, where it is **set** (option condition
//! flags) and where it is **tested** (step visibility rules, option dependency
//! patterns, conditional installs, mod requirements), plus the rename of a
//! flag across every site.
//!
//! Language-neutral: sites are plain enums the UI maps to labels and tree
//! targets.

use std::collections::{BTreeMap, BTreeSet};

use super::{ConditionSite, DependencyGroup, Ximod};

/// A place of the project that sets or tests a flag (0-based indices).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlagSite {
    /// An option's condition flags (setter).
    Plugin { step: usize, group: usize, plugin: usize },
    /// A step's `<visible>` conditions.
    StepVisibility { step: usize },
    /// One dependency pattern of an option.
    PluginPattern {
        step: usize,
        group: usize,
        plugin: usize,
        pattern: usize,
    },
    /// A conditional-install set.
    CondSet { index: usize },
    /// The mod-wide requirements (`<moduleDependencies>`).
    Module,
}

/// One site where a flag is set or tested, with the value involved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagRef {
    pub site: FlagSite,
    pub value: String,
}

/// Everything known about one flag.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FlagUsage {
    pub name: String,
    /// Options that set the flag, with the value they set.
    pub setters: Vec<FlagRef>,
    /// Conditions that test the flag, with the value they need.
    pub users: Vec<FlagRef>,
}

impl FlagUsage {
    /// Set but never tested, or tested but never set.
    pub fn is_orphan(&self) -> bool {
        self.setters.is_empty() || self.users.is_empty()
    }

    /// Distinct values the flag is set to, sorted.
    pub fn set_values(&self) -> BTreeSet<&str> {
        self.setters.iter().map(|s| s.value.as_str()).collect()
    }

    /// Distinct values the flag is tested against, sorted.
    pub fn tested_values(&self) -> BTreeSet<&str> {
        self.users.iter().map(|u| u.value.as_str()).collect()
    }

    /// Tested values that no option ever sets (usually a typo).
    pub fn values_never_set(&self) -> Vec<&str> {
        let set = self.set_values();
        self.tested_values().into_iter().filter(|v| !set.contains(v)).collect()
    }

    /// Set values that no condition ever tests (dead settings).
    pub fn values_never_tested(&self) -> Vec<&str> {
        let tested = self.tested_values();
        self.set_values().into_iter().filter(|v| !tested.contains(v)).collect()
    }

    /// The users testing a value that is never set, each with that value.
    pub fn users_of_unset_values(&self) -> Vec<&FlagRef> {
        let set = self.set_values();
        self.users.iter().filter(|u| !set.contains(u.value.as_str())).collect()
    }
}

/// A file-state dependency (`<fileDependency>`), listed for the overview.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDep {
    pub site: FlagSite,
    pub name: String,
    pub state: String,
}

/// Result of [`analyze_flags`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FlagReport {
    /// One entry per flag name, sorted by name.
    pub flags: Vec<FlagUsage>,
    /// Every file-state dependency, in project order.
    pub file_deps: Vec<FileDep>,
}

impl FlagReport {
    pub fn flag(&self, name: &str) -> Option<&FlagUsage> {
        self.flags.iter().find(|f| f.name == name)
    }
}

impl From<ConditionSite> for FlagSite {
    fn from(site: ConditionSite) -> Self {
        match site {
            ConditionSite::StepVisibility { step } => FlagSite::StepVisibility { step },
            ConditionSite::PluginPattern {
                step,
                group,
                plugin,
                pattern,
            } => FlagSite::PluginPattern {
                step,
                group,
                plugin,
                pattern,
            },
            ConditionSite::CondSet { index } => FlagSite::CondSet { index },
            ConditionSite::Module => FlagSite::Module,
        }
    }
}

/// Call `f` on every condition group of the project with its site, in the
/// order the overview lists them: per step the option patterns then the
/// step visibility, then the conditional sets, then the mod requirements.
fn for_each_dep_group<'a>(ximod: &'a Ximod, mut f: impl FnMut(FlagSite, &'a DependencyGroup)) {
    for (si, step) in ximod.steps.iter().enumerate() {
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            for (pi, plugin) in group.plugins.iter().enumerate() {
                for (pat_i, pat) in plugin.dependency_patterns.iter().enumerate() {
                    f(
                        FlagSite::PluginPattern {
                            step: si,
                            group: gi,
                            plugin: pi,
                            pattern: pat_i,
                        },
                        &pat.condition,
                    );
                }
            }
        }
        f(FlagSite::StepVisibility { step: si }, &step.visibility);
    }
    for (ci, set) in ximod.conditional_files.iter().enumerate() {
        f(FlagSite::CondSet { index: ci }, &set.condition);
    }
    if let Some(m) = &ximod.module_dependencies {
        f(FlagSite::Module, m);
    }
}

/// Same as [`for_each_dep_group`], mutably (order irrelevant).
fn for_each_dep_group_mut(ximod: &mut Ximod, mut f: impl FnMut(&mut DependencyGroup)) {
    ximod.for_each_condition_mut(|_, g| f(g));
}

/// Collect who sets and who tests every flag, and the file dependencies.
pub fn analyze_flags(ximod: &Ximod) -> FlagReport {
    let mut flags: BTreeMap<String, FlagUsage> = BTreeMap::new();
    let mut file_deps = Vec::new();

    for (si, step) in ximod.steps.iter().enumerate() {
        for (gi, group) in step.plugin_groups.iter().enumerate() {
            for (pi, plugin) in group.plugins.iter().enumerate() {
                for cf in &plugin.condition_flags {
                    flags
                        .entry(cf.name.clone())
                        .or_insert_with(|| FlagUsage {
                            name: cf.name.clone(),
                            ..Default::default()
                        })
                        .setters
                        .push(FlagRef {
                            site: FlagSite::Plugin {
                                step: si,
                                group: gi,
                                plugin: pi,
                            },
                            value: cf.value.clone(),
                        });
                }
            }
        }
    }

    for_each_dep_group(ximod, |site, group| {
        // Leaves at every depth: a flag tested inside a nested group is a
        // user of that flag like any other.
        for dep in group.leaves() {
            if dep.is_flag() {
                flags
                    .entry(dep.name.clone())
                    .or_insert_with(|| FlagUsage {
                        name: dep.name.clone(),
                        ..Default::default()
                    })
                    .users
                    .push(FlagRef {
                        site,
                        value: dep.value.clone(),
                    });
            } else if dep.is_file() {
                file_deps.push(FileDep {
                    site,
                    name: dep.name.clone(),
                    state: dep.value.clone(),
                });
            }
        }
    });

    FlagReport {
        flags: flags.into_values().collect(),
        file_deps,
    }
}

/// Rename the flag `from` to `to` everywhere: every option condition flag
/// and every flag dependency (step visibility, option patterns, conditional
/// sets, mod requirements). Returns the number of occurrences changed; a
/// no-op when `to` is empty or equal to `from`.
pub fn rename_flag(ximod: &mut Ximod, from: &str, to: &str) -> usize {
    if to.is_empty() || from == to {
        return 0;
    }
    let mut n = 0;
    for step in &mut ximod.steps {
        for group in &mut step.plugin_groups {
            for plugin in &mut group.plugins {
                for cf in &mut plugin.condition_flags {
                    if cf.name == from {
                        cf.name = to.to_string();
                        n += 1;
                    }
                }
            }
        }
    }
    for_each_dep_group_mut(ximod, |group| {
        for dep in group.leaves_mut() {
            if dep.is_flag() && dep.name == from {
                dep.name = to.to_string();
                n += 1;
            }
        }
    });
    n
}

/// Remove every setting and every test of the flag `name`. Returns the
/// number of occurrences removed.
pub fn delete_flag_uses(ximod: &mut Ximod, name: &str) -> usize {
    let mut n = 0;
    for step in &mut ximod.steps {
        for group in &mut step.plugin_groups {
            for plugin in &mut group.plugins {
                let before = plugin.condition_flags.len();
                plugin.condition_flags.retain(|cf| cf.name != name);
                n += before - plugin.condition_flags.len();
            }
        }
    }
    for_each_dep_group_mut(ximod, |group| {
        n += group.retain_leaves(|d| !(d.is_flag() && d.name == name));
    });
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        ConditionFlag, ConditionalFileSet, Dependency, DependencyPattern, LogicalOperator, Plugin, PluginGroup,
        SelectionType, Step,
    };

    /// Flag `res` set to 2K / 4K by two options; tested by a step, a pattern,
    /// a conditional set and the mod requirements. `hd` is tested only.
    fn sample() -> Ximod {
        let mut m = Ximod::new("Sample");
        let mut p2k = Plugin::new("2K");
        p2k.condition_flags.push(ConditionFlag::new("res", "2K"));
        p2k.condition_flags.push(ConditionFlag::new("dead", "1"));
        let mut p4k = Plugin::new("4K");
        p4k.condition_flags.push(ConditionFlag::new("res", "4K"));
        let mut pat = DependencyPattern::new();
        pat.condition.push_leaf(Dependency::new_flag("res", "2K"));
        pat.condition.push_leaf(Dependency::new_file("Skyrim.esm", "Active"));
        p4k.dependency_patterns.push(pat);
        let mut g = PluginGroup::new("Resolution", SelectionType::SelectExactlyOne);
        g.plugins.push(p2k);
        g.plugins.push(p4k);
        let mut s1 = Step::new("Resolution");
        s1.plugin_groups.push(g);
        let mut s2 = Step::new("Extras");
        s2.visibility.push_leaf(Dependency::new_flag("res", "4K"));
        s2.visibility.push_leaf(Dependency::new_flag("hd", "yes"));
        m.steps.push(s1);
        m.steps.push(s2);
        let mut cs = ConditionalFileSet::new();
        cs.condition.push_leaf(Dependency::new_flag("res", "8K"));
        m.conditional_files.push(cs);
        m.module_dependencies = Some(DependencyGroup::from_leaves(
            Default::default(),
            vec![Dependency::new_flag("res", "2K")],
        ));
        m
    }

    /// `sample()` plus, in the conditional set, a nested
    /// `Or(res = 2K, Or(res = 4K, file))` group.
    fn nested_sample() -> Ximod {
        let mut m = sample();
        let mut inner = DependencyGroup::new(LogicalOperator::Or);
        inner.push_leaf(Dependency::new_flag("res", "4K"));
        inner.push_leaf(Dependency::new_file("Deep.esm", "Missing"));
        let mut outer = DependencyGroup::new(LogicalOperator::Or);
        outer.push_leaf(Dependency::new_flag("res", "2K"));
        outer.push_group(inner);
        m.conditional_files[0].condition.push_group(outer);
        m
    }

    #[test]
    fn analysis_lists_setters_and_users_with_their_sites() {
        let r = analyze_flags(&sample());
        let names: Vec<&str> = r.flags.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["dead", "hd", "res"]);
        let res = r.flag("res").unwrap();
        assert_eq!(res.setters.len(), 2);
        assert_eq!(
            res.setters[1].site,
            FlagSite::Plugin {
                step: 0,
                group: 0,
                plugin: 1
            }
        );
        let sites: Vec<FlagSite> = res.users.iter().map(|u| u.site).collect();
        assert_eq!(
            sites,
            vec![
                FlagSite::PluginPattern {
                    step: 0,
                    group: 0,
                    plugin: 1,
                    pattern: 0
                },
                FlagSite::StepVisibility { step: 1 },
                FlagSite::CondSet { index: 0 },
                FlagSite::Module,
            ]
        );
        assert!(!res.is_orphan());
        assert!(r.flag("hd").unwrap().is_orphan());
        assert!(r.flag("dead").unwrap().is_orphan());
        assert_eq!(r.file_deps.len(), 1);
        assert_eq!(r.file_deps[0].name, "Skyrim.esm");
    }

    #[test]
    fn value_never_set_and_never_tested_are_detected() {
        let r = analyze_flags(&sample());
        let res = r.flag("res").unwrap();
        assert_eq!(res.values_never_set(), vec!["8K"]);
        assert!(res.values_never_tested().is_empty());
        let unset: Vec<FlagSite> = res.users_of_unset_values().iter().map(|u| u.site).collect();
        assert_eq!(unset, vec![FlagSite::CondSet { index: 0 }]);
        let dead = r.flag("dead").unwrap();
        assert_eq!(dead.values_never_tested(), vec!["1"]);
        assert!(dead.values_never_set().is_empty());
        let hd = r.flag("hd").unwrap();
        assert_eq!(hd.values_never_set(), vec!["yes"]);
    }

    #[test]
    fn rename_touches_every_site() {
        let mut m = sample();
        assert_eq!(rename_flag(&mut m, "res", "resolution"), 6);
        let r = analyze_flags(&m);
        assert!(r.flag("res").is_none());
        let f = r.flag("resolution").unwrap();
        assert_eq!(f.setters.len(), 2);
        assert_eq!(f.users.len(), 4);
        // The file dependency is untouched.
        assert_eq!(
            m.steps[0].plugin_groups[0].plugins[1].dependency_patterns[0]
                .condition
                .leaves()
                .nth(1)
                .unwrap()
                .name,
            "Skyrim.esm"
        );
        // No-ops.
        assert_eq!(rename_flag(&mut m, "resolution", ""), 0);
        assert_eq!(rename_flag(&mut m, "resolution", "resolution"), 0);
        assert_eq!(rename_flag(&mut m, "nope", "x"), 0);
    }

    #[test]
    fn delete_uses_removes_settings_and_tests() {
        let mut m = sample();
        assert_eq!(delete_flag_uses(&mut m, "res"), 6);
        let r = analyze_flags(&m);
        assert!(r.flag("res").is_none());
        assert_eq!(m.steps[1].visibility.leaf_count(), 1);
        assert_eq!(m.module_dependencies.as_ref().unwrap().leaf_count(), 0);
        assert_eq!(r.file_deps.len(), 1);
    }

    /// Lot N: the analysis, the rename and the delete descend into nested
    /// groups at every depth.
    #[test]
    fn analysis_rename_and_delete_descend_into_nested_groups() {
        let m = nested_sample();
        let r = analyze_flags(&m);
        let res = r.flag("res").unwrap();
        // 4 flat users + 2 nested ones, all attributed to the set.
        assert_eq!(res.users.len(), 6);
        let nested: Vec<&FlagRef> = res
            .users
            .iter()
            .filter(|u| u.site == FlagSite::CondSet { index: 0 })
            .collect();
        assert_eq!(nested.len(), 3);
        assert_eq!(nested[1].value, "2K");
        assert_eq!(nested[2].value, "4K");
        assert_eq!(r.file_deps.len(), 2);
        assert_eq!(r.file_deps[1].name, "Deep.esm");
        assert_eq!(r.file_deps[1].site, FlagSite::CondSet { index: 0 });

        let mut m = nested_sample();
        assert_eq!(rename_flag(&mut m, "res", "resolution"), 8);
        assert!(analyze_flags(&m).flag("res").is_none());
        let set = &m.conditional_files[0].condition;
        assert_eq!(set.depth(), 2);
        assert!(set.leaves().filter(|d| d.is_flag()).all(|d| d.name == "resolution"));

        let mut m = nested_sample();
        assert_eq!(delete_flag_uses(&mut m, "res"), 8);
        let set = &m.conditional_files[0].condition;
        // The groups stay, only the flag leaves went.
        assert_eq!(set.depth(), 2);
        assert_eq!(set.leaf_count(), 1);
        assert_eq!(set.leaves().next().unwrap().name, "Deep.esm");
    }
}
