//! Structural validation of a FOMOD `ModuleConfig.xml` against the ModConfig 5.0
//! schema, implemented in pure Rust (no external XSD engine).
//!
//! This goes beyond the well-formedness check (`check_well_formed`): it verifies
//! the element hierarchy, required/optional children and their cardinality,
//! required attributes and enumerated attribute values, faithfully following the
//! FOMOD 5.0 (`ModuleConfig.xsd`) rules. Child *order* is intentionally not
//! enforced (mod managers accept the documented elements regardless of order),
//! which avoids false positives while still catching real schema violations.
//!
//! Issues are returned as structured `SchemaIssue` values (with line/column) and
//! localised by the UI layer, keeping this module free of any i18n dependency.

use quick_xml::events::Event;
use quick_xml::Reader;

// Enumerations, mirroring the schema (and the model's own enums).
const ORDERS: &[&str] = &["Ascending", "Descending", "Explicit"];
const GROUP_TYPES: &[&str] = &[
    "SelectAtLeastOne",
    "SelectAtMostOne",
    "SelectExactlyOne",
    "SelectAll",
    "SelectAny",
];
const PLUGIN_TYPES: &[&str] = &[
    "Required",
    "Optional",
    "Recommended",
    "NotUsable",
    "CouldBeUsable",
];
const STATES: &[&str] = &["Missing", "Inactive", "Active"];
const OPERATORS: &[&str] = &["And", "Or"];
const DEP_CHILDREN: &[&str] = &[
    "fileDependency",
    "flagDependency",
    "gameDependency",
    "fommDependency",
    "dependencies",
];

/// A schema violation, independent of any language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaIssueKind {
    WrongRoot { found: String, expected: String },
    UnknownElement { element: String, parent: String },
    MissingChild { parent: String, child: String },
    NeedsOne { parent: String, child: String },
    TooMany { parent: String, child: String },
    MissingAttr { element: String, attr: String },
    BadEnum {
        element: String,
        attr: String,
        value: String,
        allowed: String,
    },
    ChooseOne { parent: String, options: String },
}

#[derive(Debug, Clone)]
pub struct SchemaIssue {
    pub line: usize,
    pub column: usize,
    pub kind: SchemaIssueKind,
}

/// English one-line description (used by the CLI; the GUI localises separately).
impl std::fmt::Display for SchemaIssueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaIssueKind::WrongRoot { found, expected } => {
                write!(f, "unexpected root \"{found}\" (expected \"{expected}\")")
            }
            SchemaIssueKind::UnknownElement { element, parent } => {
                write!(f, "unexpected element \"{element}\" in \"{parent}\"")
            }
            SchemaIssueKind::MissingChild { parent, child } => {
                write!(f, "\"{parent}\" must contain \"{child}\"")
            }
            SchemaIssueKind::NeedsOne { parent, child } => {
                write!(f, "\"{parent}\" must contain at least one \"{child}\"")
            }
            SchemaIssueKind::TooMany { parent, child } => {
                write!(f, "\"{child}\" may appear only once in \"{parent}\"")
            }
            SchemaIssueKind::MissingAttr { element, attr } => {
                write!(f, "attribute \"{attr}\" is required on \"{element}\"")
            }
            SchemaIssueKind::BadEnum { element, attr, value, allowed } => {
                write!(f, "invalid value \"{value}\" for {element}/@{attr} (expected: {allowed})")
            }
            SchemaIssueKind::ChooseOne { parent, options } => {
                write!(f, "\"{parent}\" must contain exactly one of: {options}")
            }
        }
    }
}

// --------------------------------------------------------------------------- //
// Lightweight DOM
// --------------------------------------------------------------------------- //
struct Node {
    name: String,
    attrs: Vec<(String, String)>,
    text: String,
    children: Vec<Node>,
    line: usize,
    column: usize,
}

impl Node {
    fn attr(&self, key: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
    fn count(&self, name: &str) -> usize {
        self.children.iter().filter(|c| c.name == name).count()
    }
    fn find(&self, name: &str) -> Option<&Node> {
        self.children.iter().find(|c| c.name == name)
    }
    fn each<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Node> {
        self.children.iter().filter(move |c| c.name == name)
    }
}

fn byte_to_line_col(content: &str, byte: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;
    for (i, ch) in content.char_indices() {
        if i >= byte {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn make_node(e: &quick_xml::events::BytesStart, content: &str, pos: usize) -> Node {
    let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
    let mut attrs = Vec::new();
    for a in e.attributes().flatten() {
        let k = String::from_utf8_lossy(a.key.as_ref()).into_owned();
        let v = a
            .unescape_value()
            .map(|c| c.into_owned())
            .unwrap_or_default();
        attrs.push((k, v));
    }
    let (line, column) = byte_to_line_col(content, pos.min(content.len()));
    Node {
        name,
        attrs,
        text: String::new(),
        children: Vec::new(),
        line,
        column,
    }
}

/// Parse the document into a single root node, or `None` if it is not
/// well-formed (well-formedness is reported separately by `check_well_formed`).
fn parse_tree(content: &str) -> Option<Node> {
    let mut reader = Reader::from_reader(content.as_bytes());
    reader.config_mut().trim_text(true);
    let mut stack: Vec<Node> = Vec::new();
    let mut root: Option<Node> = None;
    let mut buf = Vec::new();

    fn attach(stack: &mut Vec<Node>, root: &mut Option<Node>, node: Node) {
        match stack.last_mut() {
            Some(parent) => parent.children.push(node),
            None => *root = Some(node),
        }
    }

    loop {
        let pos = reader.buffer_position() as usize;
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => stack.push(make_node(&e, content, pos)),
            Ok(Event::Empty(e)) => {
                let node = make_node(&e, content, pos);
                attach(&mut stack, &mut root, node);
            }
            Ok(Event::End(_)) => {
                if let Some(node) = stack.pop() {
                    attach(&mut stack, &mut root, node);
                }
            }
            Ok(Event::Text(t)) => {
                if let Some(top) = stack.last_mut() {
                    if let Ok(txt) = t.unescape() {
                        top.text.push_str(txt.trim());
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(_) => return None,
        }
        buf.clear();
    }
    root
}

// --------------------------------------------------------------------------- //
// Checks
// --------------------------------------------------------------------------- //
struct Ctx {
    issues: Vec<SchemaIssue>,
}

impl Ctx {
    fn push(&mut self, node: &Node, kind: SchemaIssueKind) {
        self.issues.push(SchemaIssue {
            line: node.line,
            column: node.column,
            kind,
        });
    }

    /// Flag any child element not in `allowed`.
    fn only(&mut self, node: &Node, allowed: &[&str]) {
        for c in &node.children {
            if !allowed.contains(&c.name.as_str()) {
                self.push(
                    c,
                    SchemaIssueKind::UnknownElement {
                        element: c.name.clone(),
                        parent: node.name.clone(),
                    },
                );
            }
        }
    }

    /// `child` must appear exactly once.
    fn exactly_one(&mut self, node: &Node, child: &str) {
        match node.count(child) {
            0 => self.push(
                node,
                SchemaIssueKind::MissingChild {
                    parent: node.name.clone(),
                    child: child.to_string(),
                },
            ),
            1 => {}
            _ => self.push(
                node,
                SchemaIssueKind::TooMany {
                    parent: node.name.clone(),
                    child: child.to_string(),
                },
            ),
        }
    }

    /// `child` may appear zero or one time.
    fn at_most_one(&mut self, node: &Node, child: &str) {
        if node.count(child) > 1 {
            self.push(
                node,
                SchemaIssueKind::TooMany {
                    parent: node.name.clone(),
                    child: child.to_string(),
                },
            );
        }
    }

    /// `child` must appear at least once.
    fn at_least_one(&mut self, node: &Node, child: &str) {
        if node.count(child) == 0 {
            self.push(
                node,
                SchemaIssueKind::NeedsOne {
                    parent: node.name.clone(),
                    child: child.to_string(),
                },
            );
        }
    }

    fn require_attr(&mut self, node: &Node, attr: &str) {
        if node.attr(attr).is_none() {
            self.push(
                node,
                SchemaIssueKind::MissingAttr {
                    element: node.name.clone(),
                    attr: attr.to_string(),
                },
            );
        }
    }

    /// Check an enumerated attribute; when `required`, its absence is an error.
    fn enum_attr(&mut self, node: &Node, attr: &str, allowed: &[&str], required: bool) {
        match node.attr(attr) {
            Some(v) => {
                if !allowed.contains(&v) {
                    self.push(
                        node,
                        SchemaIssueKind::BadEnum {
                            element: node.name.clone(),
                            attr: attr.to_string(),
                            value: v.to_string(),
                            allowed: allowed.join(", "),
                        },
                    );
                }
            }
            None => {
                if required {
                    self.push(
                        node,
                        SchemaIssueKind::MissingAttr {
                            element: node.name.clone(),
                            attr: attr.to_string(),
                        },
                    );
                }
            }
        }
    }
}

/// Validate a `ModuleConfig.xml` string. Returns all schema issues found (empty
/// when the document conforms). Not-well-formed input yields no issue here — use
/// `check_well_formed` for that.
pub fn validate_module_config(content: &str) -> Vec<SchemaIssue> {
    let mut ctx = Ctx { issues: Vec::new() };
    let Some(root) = parse_tree(content) else {
        return ctx.issues;
    };
    if root.name != "config" {
        ctx.push(
            &root,
            SchemaIssueKind::WrongRoot {
                found: root.name.clone(),
                expected: "config".to_string(),
            },
        );
        return ctx.issues;
    }
    validate_config(&mut ctx, &root);
    ctx.issues
}

/// Validate an `info.xml` string (loose schema): only the root is enforced, as
/// managers ignore unknown elements and XIMOD adds its own `Game` extension.
pub fn validate_info(content: &str) -> Vec<SchemaIssue> {
    let mut ctx = Ctx { issues: Vec::new() };
    if let Some(root) = parse_tree(content) {
        if root.name != "fomod" {
            ctx.push(
                &root,
                SchemaIssueKind::WrongRoot {
                    found: root.name.clone(),
                    expected: "fomod".to_string(),
                },
            );
        }
    }
    ctx.issues
}

fn validate_config(ctx: &mut Ctx, node: &Node) {
    ctx.only(
        node,
        &[
            "moduleName",
            "moduleImage",
            "moduleDependencies",
            "requiredInstallFiles",
            "installSteps",
            "conditionalFileInstalls",
        ],
    );
    ctx.exactly_one(node, "moduleName");
    for e in ["moduleImage", "moduleDependencies", "requiredInstallFiles", "installSteps", "conditionalFileInstalls"] {
        ctx.at_most_one(node, e);
    }
    if let Some(mi) = node.find("moduleImage") {
        ctx.require_attr(mi, "path");
    }
    if let Some(md) = node.find("moduleDependencies") {
        validate_dependencies(ctx, md);
    }
    if let Some(rif) = node.find("requiredInstallFiles") {
        validate_filelist(ctx, rif);
    }
    if let Some(steps) = node.find("installSteps") {
        validate_install_steps(ctx, steps);
    }
    if let Some(cfi) = node.find("conditionalFileInstalls") {
        validate_conditional_installs(ctx, cfi);
    }
}

fn validate_filelist(ctx: &mut Ctx, node: &Node) {
    ctx.only(node, &["file", "folder"]);
    for f in node.children.iter().filter(|c| c.name == "file" || c.name == "folder") {
        ctx.require_attr(f, "source");
        if let Some(p) = f.attr("priority") {
            if p.parse::<i64>().is_err() {
                ctx.push(
                    f,
                    SchemaIssueKind::BadEnum {
                        element: f.name.clone(),
                        attr: "priority".to_string(),
                        value: p.to_string(),
                        allowed: "integer".to_string(),
                    },
                );
            }
        }
    }
}

fn validate_dependencies(ctx: &mut Ctx, node: &Node) {
    ctx.enum_attr(node, "operator", OPERATORS, false);
    ctx.only(node, DEP_CHILDREN);
    for c in &node.children {
        match c.name.as_str() {
            "fileDependency" => {
                ctx.require_attr(c, "file");
                ctx.enum_attr(c, "state", STATES, true);
            }
            "flagDependency" => {
                ctx.require_attr(c, "flag");
                ctx.require_attr(c, "value");
            }
            "dependencies" => validate_dependencies(ctx, c),
            _ => {}
        }
    }
}

fn validate_install_steps(ctx: &mut Ctx, node: &Node) {
    ctx.enum_attr(node, "order", ORDERS, false);
    ctx.only(node, &["installStep"]);
    ctx.at_least_one(node, "installStep");
    for s in node.each("installStep") {
        validate_install_step(ctx, s);
    }
}

fn validate_install_step(ctx: &mut Ctx, node: &Node) {
    ctx.require_attr(node, "name");
    ctx.only(node, &["visible", "optionalFileGroups"]);
    ctx.at_most_one(node, "visible");
    if let Some(v) = node.find("visible") {
        validate_dependencies(ctx, v);
    }
    ctx.exactly_one(node, "optionalFileGroups");
    if let Some(g) = node.find("optionalFileGroups") {
        validate_optional_file_groups(ctx, g);
    }
}

fn validate_optional_file_groups(ctx: &mut Ctx, node: &Node) {
    ctx.enum_attr(node, "order", ORDERS, false);
    ctx.only(node, &["group"]);
    ctx.at_least_one(node, "group");
    for g in node.each("group") {
        validate_group(ctx, g);
    }
}

fn validate_group(ctx: &mut Ctx, node: &Node) {
    ctx.require_attr(node, "name");
    ctx.enum_attr(node, "type", GROUP_TYPES, true);
    ctx.only(node, &["plugins"]);
    ctx.exactly_one(node, "plugins");
    if let Some(p) = node.find("plugins") {
        validate_plugins(ctx, p);
    }
}

fn validate_plugins(ctx: &mut Ctx, node: &Node) {
    ctx.enum_attr(node, "order", ORDERS, false);
    ctx.only(node, &["plugin"]);
    ctx.at_least_one(node, "plugin");
    for p in node.each("plugin") {
        validate_plugin(ctx, p);
    }
}

fn validate_plugin(ctx: &mut Ctx, node: &Node) {
    ctx.require_attr(node, "name");
    ctx.only(
        node,
        &["description", "image", "conditionFlags", "files", "typeDescriptor"],
    );
    ctx.exactly_one(node, "description");
    ctx.at_most_one(node, "image");
    if let Some(img) = node.find("image") {
        ctx.require_attr(img, "path");
    }
    ctx.at_most_one(node, "conditionFlags");
    if let Some(cf) = node.find("conditionFlags") {
        ctx.only(cf, &["flag"]);
        ctx.at_least_one(cf, "flag");
        for f in cf.each("flag") {
            ctx.require_attr(f, "name");
        }
    }
    ctx.at_most_one(node, "files");
    if let Some(files) = node.find("files") {
        validate_filelist(ctx, files);
    }
    ctx.exactly_one(node, "typeDescriptor");
    if let Some(td) = node.find("typeDescriptor") {
        validate_type_descriptor(ctx, td);
    }
}

fn validate_type_descriptor(ctx: &mut Ctx, node: &Node) {
    ctx.only(node, &["type", "dependencyType"]);
    let has_type = node.count("type");
    let has_dep = node.count("dependencyType");
    if has_type + has_dep != 1 {
        ctx.push(
            node,
            SchemaIssueKind::ChooseOne {
                parent: node.name.clone(),
                options: "type | dependencyType".to_string(),
            },
        );
    }
    if let Some(t) = node.find("type") {
        validate_type(ctx, t);
    }
    if let Some(dt) = node.find("dependencyType") {
        validate_dependency_type(ctx, dt);
    }
}

fn validate_type(ctx: &mut Ctx, node: &Node) {
    ctx.enum_attr(node, "name", PLUGIN_TYPES, true);
}

fn validate_dependency_type(ctx: &mut Ctx, node: &Node) {
    ctx.only(node, &["defaultType", "patterns"]);
    ctx.exactly_one(node, "defaultType");
    if let Some(dt) = node.find("defaultType") {
        ctx.enum_attr(dt, "name", PLUGIN_TYPES, true);
    }
    ctx.exactly_one(node, "patterns");
    if let Some(pats) = node.find("patterns") {
        ctx.only(pats, &["pattern"]);
        ctx.at_least_one(pats, "pattern");
        for p in pats.each("pattern") {
            ctx.only(p, &["dependencies", "type"]);
            ctx.exactly_one(p, "dependencies");
            if let Some(d) = p.find("dependencies") {
                validate_dependencies(ctx, d);
            }
            ctx.exactly_one(p, "type");
            if let Some(t) = p.find("type") {
                validate_type(ctx, t);
            }
        }
    }
}

fn validate_conditional_installs(ctx: &mut Ctx, node: &Node) {
    ctx.only(node, &["patterns"]);
    ctx.exactly_one(node, "patterns");
    if let Some(pats) = node.find("patterns") {
        ctx.only(pats, &["pattern"]);
        for p in pats.each("pattern") {
            ctx.only(p, &["dependencies", "files"]);
            ctx.exactly_one(p, "dependencies");
            if let Some(d) = p.find("dependencies") {
                validate_dependencies(ctx, d);
            }
            ctx.exactly_one(p, "files");
            if let Some(f) = p.find("files") {
                validate_filelist(ctx, f);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<config xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo3/ModConfig5.0.xsd">
    <moduleName>Demo</moduleName>
    <requiredInstallFiles>
        <file source="a.esp" destination="a.esp" priority="0"/>
    </requiredInstallFiles>
    <installSteps order="Explicit">
        <installStep name="Step 1">
            <optionalFileGroups order="Explicit">
                <group name="G" type="SelectExactlyOne">
                    <plugins order="Explicit">
                        <plugin name="P">
                            <description>d</description>
                            <conditionFlags>
                                <flag name="res">2K</flag>
                            </conditionFlags>
                            <files>
                                <folder source="tex" destination="tex" priority="0"/>
                            </files>
                            <typeDescriptor>
                                <type name="Optional"/>
                            </typeDescriptor>
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
                    <flagDependency flag="res" value="4K"/>
                </dependencies>
                <files>
                    <file source="p.esp" destination="p.esp" priority="0"/>
                </files>
            </pattern>
        </patterns>
    </conditionalFileInstalls>
</config>"#;

    fn kinds(content: &str) -> Vec<SchemaIssueKind> {
        validate_module_config(content).into_iter().map(|i| i.kind).collect()
    }

    #[test]
    fn valid_document_has_no_issue() {
        assert_eq!(validate_module_config(VALID).len(), 0, "{:?}", kinds(VALID));
    }

    /// The validator must accept XIMOD's own serialized output (no false
    /// positives), including dynamic typeDescriptor patterns and conditionals.
    #[test]
    fn accepts_serialized_output_from_model() {
        use crate::models::*;
        let mut m = Ximod::new("Demo");
        m.required_files.push(InstallFile::new_file("a.esp"));

        let mut g = PluginGroup::new("G", SelectionType::SelectExactlyOne);
        let mut p = Plugin::new("P");
        p.description = "desc".into();
        p.condition_flags.push(ConditionFlag::new("res", "2K"));
        p.files.push(InstallFile::new_folder("tex"));
        g.plugins.push(p);

        let mut p2 = Plugin::new("P2");
        p2.description = "d2".into();
        let mut pat = DependencyPattern::new();
        pat.pattern_type = "Recommended".into();
        pat.dependencies.push(Dependency::new_flag("res", "2K"));
        p2.dependency_patterns.push(pat);
        g.plugins.push(p2);

        let mut s = Step::new("S");
        s.visibility_dependencies
            .push(Dependency::new_file("Skyrim.esm", "Active"));
        s.plugin_groups.push(g);
        m.steps.push(s);

        let mut cfs = ConditionalFileSet::new();
        cfs.dependencies.push(Dependency::new_flag("res", "4K"));
        cfs.files.push(InstallFile::new_file("p.esp"));
        m.conditional_files.push(cfs);

        let xml = super::super::module_config_to_string(&m).unwrap();
        let issues = validate_module_config(&xml);
        assert!(
            issues.is_empty(),
            "serialized output should be schema-valid: {:?}",
            issues.iter().map(|i| &i.kind).collect::<Vec<_>>()
        );
    }

    #[test]
    fn detects_bad_group_type() {
        let bad = VALID.replace("SelectExactlyOne", "SelectExactlyTwo");
        let ks = kinds(&bad);
        assert!(ks.iter().any(|k| matches!(k, SchemaIssueKind::BadEnum { attr, .. } if attr == "type")), "{ks:?}");
    }

    #[test]
    fn detects_missing_type_descriptor() {
        let bad = VALID.replace(
            "<typeDescriptor>\n                                <type name=\"Optional\"/>\n                            </typeDescriptor>",
            "",
        );
        let ks = kinds(&bad);
        assert!(ks.iter().any(|k| matches!(k, SchemaIssueKind::MissingChild { child, .. } if child == "typeDescriptor")), "{ks:?}");
    }

    #[test]
    fn detects_unknown_element_and_missing_required() {
        let bad = VALID.replace("<moduleName>Demo</moduleName>", "<oops/>");
        let ks = kinds(&bad);
        assert!(ks.iter().any(|k| matches!(k, SchemaIssueKind::UnknownElement { element, .. } if element == "oops")), "{ks:?}");
        assert!(ks.iter().any(|k| matches!(k, SchemaIssueKind::MissingChild { child, .. } if child == "moduleName")), "{ks:?}");
    }

    #[test]
    fn detects_missing_required_attribute() {
        let bad = VALID.replace("<flagDependency flag=\"res\" value=\"4K\"/>", "<flagDependency value=\"4K\"/>");
        let ks = kinds(&bad);
        assert!(ks.iter().any(|k| matches!(k, SchemaIssueKind::MissingAttr { .. })), "{ks:?}");
    }
}
