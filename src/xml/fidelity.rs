//! Import fidelity: find the constructs of a `ModuleConfig.xml` that the
//! model (`models::ximod`) cannot represent.
//!
//! `parse_module_config_xml` is lenient: it keeps what it understands and
//! silently skips the rest, so a third-party FOMOD that uses an element the
//! model has no place for loads fine but loses it on the next save. This
//! module lists those elements, so the UI and the CLI can warn before
//! anything is overwritten. It is built on the same lightweight DOM as the
//! schema validator (`validate::parse_tree`) and, like the parser, matches
//! element names case-insensitively.
//!
//! Dependency lists are modelled in full since lot N (nested
//! `<dependencies>` groups, `gameDependency`, `fommDependency`): only an
//! unknown child of a list is reported there.
//!
//! Everything here is language-neutral: `Unmodelled` implements `Display`
//! in English for the CLI, and the GUI localises each variant separately.

use super::validate::{Node, parse_tree};

/// Where in the installer an unmodelled construct was found (1-based, as
/// shown to the user).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Loc {
    /// `<moduleDependencies>` (mod-wide requirements).
    Module,
    /// The visibility conditions of a step.
    Step { step: usize, name: String },
    /// A dependency pattern of an option. `plugin` is the 0-based index of
    /// the option in its group (for selection), `plugin_name` its name.
    Plugin {
        step: usize,
        group: usize,
        plugin: usize,
        plugin_name: String,
    },
    /// A conditional-install pattern.
    Conditional { index: usize },
    /// Elsewhere (the element itself is named in the item).
    Other,
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Loc::Module => f.write_str("the mod requirements"),
            Loc::Step { step, name } => write!(f, "step {step} \"{name}\""),
            Loc::Plugin {
                step,
                group,
                plugin_name,
                ..
            } => write!(f, "step {step}, group {group}, option \"{plugin_name}\""),
            Loc::Conditional { index } => write!(f, "conditional set {index}"),
            Loc::Other => f.write_str("the installer"),
        }
    }
}

/// A construct of `ModuleConfig.xml` the model drops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unmodelled {
    /// An element the parser ignores (schema-valid or not).
    UnknownElement { element: String, parent: String, loc: Loc },
}

impl Unmodelled {
    /// Where the construct lives.
    pub fn loc(&self) -> &Loc {
        match self {
            Unmodelled::UnknownElement { loc, .. } => loc,
        }
    }
}

/// English one-line description (used by the CLI; the GUI localises).
impl std::fmt::Display for Unmodelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unmodelled::UnknownElement { element, parent, loc } => {
                write!(f, "element \"{element}\" in \"{parent}\" is not supported ({loc})")
            }
        }
    }
}

/// List every construct of `content` (a `ModuleConfig.xml` document) that
/// `parse_module_config_xml` drops. Empty for a document the model
/// represents faithfully, and for a document that is not well-formed (that
/// is reported elsewhere).
pub fn scan_module_config(content: &str) -> Vec<Unmodelled> {
    let mut out = Vec::new();
    let Some(root) = parse_tree(content) else {
        return out;
    };
    if !is(&root, "config") {
        return out;
    }
    let mut ctx = Scan { out: &mut out };
    ctx.children_only(
        &root,
        &[
            "moduleName",
            "moduleImage",
            "moduleDependencies",
            "requiredInstallFiles",
            "installSteps",
            "conditionalFileInstalls",
        ],
        &Loc::Other,
    );
    if let Some(md) = find(&root, "moduleDependencies") {
        ctx.dependency_list(md, &Loc::Module);
    }
    if let Some(rif) = find(&root, "requiredInstallFiles") {
        ctx.file_list(rif, &Loc::Other);
    }
    if let Some(steps) = find(&root, "installSteps") {
        ctx.children_only(steps, &["installStep"], &Loc::Other);
        for (si, step) in each(steps, "installStep").enumerate() {
            ctx.step(step, si + 1);
        }
    }
    if let Some(cfi) = find(&root, "conditionalFileInstalls") {
        ctx.children_only(cfi, &["patterns"], &Loc::Other);
        if let Some(pats) = find(cfi, "patterns") {
            ctx.children_only(pats, &["pattern"], &Loc::Other);
            for (pi, pat) in each(pats, "pattern").enumerate() {
                let loc = Loc::Conditional { index: pi + 1 };
                ctx.children_only(pat, &["dependencies", "files"], &loc);
                if let Some(deps) = find(pat, "dependencies") {
                    ctx.dependency_list(deps, &loc);
                }
                if let Some(files) = find(pat, "files") {
                    ctx.file_list(files, &loc);
                }
            }
        }
    }
    out
}

fn is(node: &Node, name: &str) -> bool {
    node.name.eq_ignore_ascii_case(name)
}

fn find<'a>(node: &'a Node, name: &str) -> Option<&'a Node> {
    node.children.iter().find(|c| is(c, name))
}

fn each<'a>(node: &'a Node, name: &'a str) -> impl Iterator<Item = &'a Node> {
    node.children.iter().filter(move |c| is(c, name))
}

struct Scan<'a> {
    out: &'a mut Vec<Unmodelled>,
}

impl Scan<'_> {
    /// Report every child of `node` whose name is not in `allowed`.
    fn children_only(&mut self, node: &Node, allowed: &[&str], loc: &Loc) {
        for c in &node.children {
            if !allowed.iter().any(|a| is(c, a)) {
                self.out.push(Unmodelled::UnknownElement {
                    element: c.name.clone(),
                    parent: node.name.clone(),
                    loc: loc.clone(),
                });
            }
        }
    }

    fn file_list(&mut self, node: &Node, loc: &Loc) {
        self.children_only(node, &["file", "folder"], loc);
    }

    /// A `<dependencies>` / `<visible>` / `<moduleDependencies>` list: the
    /// four leaf kinds and nested groups (any depth) are modelled, anything
    /// else is reported.
    fn dependency_list(&mut self, node: &Node, loc: &Loc) {
        for c in &node.children {
            if is(c, "dependencies") {
                self.dependency_list(c, loc);
            } else if !(is(c, "fileDependency")
                || is(c, "flagDependency")
                || is(c, "gameDependency")
                || is(c, "fommDependency"))
            {
                self.out.push(Unmodelled::UnknownElement {
                    element: c.name.clone(),
                    parent: node.name.clone(),
                    loc: loc.clone(),
                });
            }
        }
    }

    fn step(&mut self, node: &Node, step: usize) {
        let loc = Loc::Step {
            step,
            name: node.attr("name").unwrap_or("").to_string(),
        };
        self.children_only(node, &["visible", "optionalFileGroups"], &loc);
        if let Some(v) = find(node, "visible") {
            self.dependency_list(v, &loc);
        }
        if let Some(groups) = find(node, "optionalFileGroups") {
            self.children_only(groups, &["group"], &loc);
            for (gi, group) in each(groups, "group").enumerate() {
                self.children_only(group, &["plugins"], &loc);
                if let Some(plugins) = find(group, "plugins") {
                    self.children_only(plugins, &["plugin"], &loc);
                    for (pi, plugin) in each(plugins, "plugin").enumerate() {
                        let ploc = Loc::Plugin {
                            step,
                            group: gi + 1,
                            plugin: pi,
                            plugin_name: plugin.attr("name").unwrap_or("").to_string(),
                        };
                        self.plugin(plugin, &ploc);
                    }
                }
            }
        }
    }

    fn plugin(&mut self, node: &Node, loc: &Loc) {
        self.children_only(
            node,
            &["description", "image", "conditionFlags", "files", "typeDescriptor"],
            loc,
        );
        if let Some(cf) = find(node, "conditionFlags") {
            self.children_only(cf, &["flag"], loc);
        }
        if let Some(files) = find(node, "files") {
            self.file_list(files, loc);
        }
        if let Some(td) = find(node, "typeDescriptor") {
            self.children_only(td, &["type", "dependencyType"], loc);
            if let Some(dt) = find(td, "dependencyType") {
                self.children_only(dt, &["defaultType", "patterns"], loc);
                if let Some(pats) = find(dt, "patterns") {
                    self.children_only(pats, &["pattern"], loc);
                    for pat in each(pats, "pattern") {
                        self.children_only(pat, &["dependencies", "type"], loc);
                        if let Some(deps) = find(pat, "dependencies") {
                            self.dependency_list(deps, loc);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOSSY: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<config>
    <moduleName>Demo</moduleName>
    <moduleDependencies operator="And">
        <fileDependency file="Skyrim.esm" state="Active"/>
        <gameDependency version="1.6.1170"/>
        <dependencies operator="Or">
            <flagDependency flag="a" value="1"/>
            <fommDependency version="0.13"/>
        </dependencies>
    </moduleDependencies>
    <installSteps order="Explicit">
        <installStep name="Step 1">
            <visible>
                <dependencies operator="And">
                    <flagDependency flag="x" value="1"/>
                </dependencies>
            </visible>
            <optionalFileGroups order="Explicit">
                <group name="G" type="SelectExactlyOne">
                    <plugins order="Explicit">
                        <plugin name="P">
                            <description>d</description>
                            <bogus/>
                            <typeDescriptor>
                                <dependencyType>
                                    <defaultType name="Optional"/>
                                    <patterns>
                                        <pattern>
                                            <dependencies operator="And">
                                                <gameDependency version="1.5.97"/>
                                            </dependencies>
                                            <type name="Required"/>
                                        </pattern>
                                    </patterns>
                                </dependencyType>
                            </typeDescriptor>
                        </plugin>
                    </plugins>
                </group>
            </optionalFileGroups>
        </installStep>
        <installStep name="Step 2">
            <visible operator="Or">
                <fileDependency file="a.esp" state="Active"/>
                <dependencies operator="And">
                    <dependencies operator="Or">
                        <flagDependency flag="y" value="1"/>
                    </dependencies>
                </dependencies>
            </visible>
            <optionalFileGroups order="Explicit">
                <group name="G2" type="SelectAny">
                    <plugins order="Explicit">
                        <plugin name="Q">
                            <description>d</description>
                            <typeDescriptor><type name="Optional"/></typeDescriptor>
                        </plugin>
                    </plugins>
                </group>
            </optionalFileGroups>
        </installStep>
    </installSteps>
    <conditionalFileInstalls>
        <patterns>
            <pattern>
                <dependencies operator="And">
                    <dependencies operator="Or">
                        <flagDependency flag="res" value="4K"/>
                    </dependencies>
                </dependencies>
                <files>
                    <file source="p.esp" destination="p.esp" priority="0"/>
                </files>
            </pattern>
        </patterns>
    </conditionalFileInstalls>
</config>"#;

    /// Nested groups and game / manager requirements are modelled since
    /// lot N: only the genuinely unknown element is reported.
    #[test]
    fn scan_reports_unknown_elements_only() {
        let found = scan_module_config(LOSSY);
        let ploc = Loc::Plugin {
            step: 1,
            group: 1,
            plugin: 0,
            plugin_name: "P".into(),
        };
        assert_eq!(
            found,
            vec![Unmodelled::UnknownElement {
                element: "bogus".into(),
                parent: "plugin".into(),
                loc: ploc
            }],
            "{found:?}"
        );
        // English rendering for the CLI.
        let text = found[0].to_string();
        assert!(text.contains("bogus") && text.contains("option \"P\""), "{text}");
        // An unknown child of a (nested) dependency list is still reported.
        let xml = r#"<config><moduleName>x</moduleName><moduleDependencies operator="And">
<dependencies operator="Or"><weird/></dependencies></moduleDependencies></config>"#;
        assert_eq!(
            scan_module_config(xml),
            vec![Unmodelled::UnknownElement {
                element: "weird".into(),
                parent: "dependencies".into(),
                loc: Loc::Module
            }]
        );
    }

    #[test]
    fn faithful_document_and_own_output_have_no_report() {
        use crate::models::*;
        let mut m = Ximod::new("Demo");
        m.module_dependencies = Some(DependencyGroup::from_leaves(
            LogicalOperator::Or,
            vec![
                Dependency::new_file("Skyrim.esm", "Active"),
                Dependency::new_game("1.6"),
            ],
        ));
        let mut s = Step::new("S");
        s.visibility.push_leaf(Dependency::new_flag("a", "1"));
        let mut nested = DependencyGroup::new(LogicalOperator::Or);
        nested.push_leaf(Dependency::new_flag("b", "1"));
        nested.push_leaf(Dependency::new_fomm("0.13"));
        s.visibility.push_group(nested);
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        let mut p = Plugin::new("P");
        let mut pat = DependencyPattern::new();
        pat.condition.push_leaf(Dependency::new_flag("a", "1"));
        p.dependency_patterns.push(pat);
        g.plugins.push(p);
        s.plugin_groups.push(g);
        m.steps.push(s);
        let mut c = ConditionalFileSet::new();
        c.condition.push_leaf(Dependency::new_flag("a", "1"));
        c.files.push(InstallFile::new_file("x.esp"));
        m.conditional_files.push(c);
        let xml = crate::xml::module_config_to_string(&m).unwrap();
        assert!(scan_module_config(&xml).is_empty());
        assert!(scan_module_config("<config><moduleName>x</moduleName></config>").is_empty());
        // Not well-formed: nothing to report here.
        assert!(scan_module_config("<config><moduleName>").is_empty());
    }
}
