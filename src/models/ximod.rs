//! XIMOD data models
//!
//! Complete data structures representing a mod installer package,
//! migrated from the original C++ implementation

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Logical operator for combining conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LogicalOperator {
    #[default]
    And,
    Or,
}

impl LogicalOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::And => "And",
            Self::Or => "Or",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "or" => Self::Or,
            _ => Self::And,
        }
    }

    pub fn variants() -> &'static [Self] {
        &[Self::And, Self::Or]
    }
}

/// Selection type for plugin groups (5 types from original)
///
/// The variant names deliberately mirror the FOMOD schema's `groupType`
/// values (`SelectExactlyOne`, …) so that `as_str`/`from_str` stay obvious.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectionType {
    SelectExactlyOne,
    SelectAtMostOne,
    #[default]
    SelectAny,
    SelectAll,
    SelectAtLeastOne,
}

impl SelectionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SelectExactlyOne => "SelectExactlyOne",
            Self::SelectAtMostOne => "SelectAtMostOne",
            Self::SelectAny => "SelectAny",
            Self::SelectAll => "SelectAll",
            Self::SelectAtLeastOne => "SelectAtLeastOne",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "SelectExactlyOne" => Self::SelectExactlyOne,
            "SelectAtMostOne" => Self::SelectAtMostOne,
            "SelectAll" => Self::SelectAll,
            "SelectAtLeastOne" => Self::SelectAtLeastOne,
            _ => Self::SelectAny,
        }
    }

    pub fn variants() -> &'static [Self] {
        &[
            Self::SelectExactlyOne,
            Self::SelectAtMostOne,
            Self::SelectAny,
            Self::SelectAll,
            Self::SelectAtLeastOne,
        ]
    }
}

/// Plugin type (from original DefaultType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PluginType {
    #[default]
    Optional,
    Required,
    Recommended,
    NotUsable,
    CouldBeUsable,
}

impl PluginType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Optional => "Optional",
            Self::Required => "Required",
            Self::Recommended => "Recommended",
            Self::NotUsable => "NotUsable",
            Self::CouldBeUsable => "CouldBeUsable",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Required" => Self::Required,
            "Recommended" => Self::Recommended,
            "NotUsable" => Self::NotUsable,
            "CouldBeUsable" => Self::CouldBeUsable,
            _ => Self::Optional,
        }
    }

    pub fn variants() -> &'static [Self] {
        &[
            Self::Optional,
            Self::Required,
            Self::Recommended,
            Self::NotUsable,
            Self::CouldBeUsable,
        ]
    }
}

/// File type (file or folder)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FileType {
    #[default]
    File,
    Folder,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Folder => "folder",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "folder" => Self::Folder,
            _ => Self::File,
        }
    }
}

/// File state for dependencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FileState {
    #[default]
    Active,
    Inactive,
    Missing,
}

impl FileState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Inactive => "Inactive",
            Self::Missing => "Missing",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Inactive" => Self::Inactive,
            "Missing" => Self::Missing,
            _ => Self::Active,
        }
    }

    pub fn variants() -> &'static [Self] {
        &[Self::Active, Self::Inactive, Self::Missing]
    }
}

/// Dependency type enum (CDependency from C++)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    Flag,
    File,
    /// `<gameDependency version="…">`: minimum game version.
    Game,
    /// `<fommDependency version="…">`: minimum mod-manager version.
    Fomm,
}

impl DependencyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Flag => "flag",
            Self::File => "file",
            Self::Game => "game",
            Self::Fomm => "fomm",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "file" => Self::File,
            "game" => Self::Game,
            "fomm" => Self::Fomm,
            _ => Self::Flag,
        }
    }

    pub fn variants() -> &'static [Self] {
        &[Self::Flag, Self::File, Self::Game, Self::Fomm]
    }
}

/// Dependency (CDependency from C++): one leaf condition.
///
/// `dep_type` is `"flag"` (`name` = flag, `value` = expected value),
/// `"file"` (`name` = file, `value` = `Active` / `Inactive` / `Missing`),
/// `"game"` (`value` = minimum game version, `name` empty) or `"fomm"`
/// (`value` = minimum mod-manager version, `name` empty).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    pub dep_type: String, // "flag", "file", "game" or "fomm"
    pub name: String,
    pub value: String,
}

impl Dependency {
    pub fn new_flag(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            dep_type: "flag".to_string(),
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn new_file(file: impl Into<String>, state: impl Into<String>) -> Self {
        Self {
            dep_type: "file".to_string(),
            name: file.into(),
            value: state.into(),
        }
    }

    /// `<gameDependency version="…">`.
    pub fn new_game(version: impl Into<String>) -> Self {
        Self {
            dep_type: "game".to_string(),
            name: String::new(),
            value: version.into(),
        }
    }

    /// `<fommDependency version="…">`.
    pub fn new_fomm(version: impl Into<String>) -> Self {
        Self {
            dep_type: "fomm".to_string(),
            name: String::new(),
            value: version.into(),
        }
    }

    /// The kind of this leaf (unknown strings count as flags, like the
    /// parser does).
    pub fn kind(&self) -> DependencyType {
        DependencyType::from_str(&self.dep_type)
    }

    pub fn is_flag(&self) -> bool {
        self.dep_type == "flag"
    }

    pub fn is_file(&self) -> bool {
        self.dep_type == "file"
    }

    /// A version leaf (`game` / `fomm`) has no name: it is "complete" as
    /// soon as it has a version, where flag / file leaves need a name.
    pub fn is_version(&self) -> bool {
        matches!(self.kind(), DependencyType::Game | DependencyType::Fomm)
    }

    /// Whether the leaf carries enough to be written (a name for flag /
    /// file leaves, a version for game / manager leaves).
    pub fn is_complete(&self) -> bool {
        if self.is_version() {
            !self.value.is_empty()
        } else {
            !self.name.is_empty()
        }
    }

    pub fn display_name(&self) -> String {
        match self.kind() {
            DependencyType::Flag => format!("[Flag] {} = {}", self.name, self.value),
            DependencyType::File => format!("[File] {} ({})", self.name, self.value),
            DependencyType::Game => format!("[Game] >= {}", self.value),
            DependencyType::Fomm => format!("[Mod manager] >= {}", self.value),
        }
    }
}

/// One item of a [`DependencyGroup`]: a leaf condition or a nested group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyItem {
    Leaf(Dependency),
    Group(DependencyGroup),
}

impl DependencyItem {
    pub fn as_leaf(&self) -> Option<&Dependency> {
        match self {
            Self::Leaf(d) => Some(d),
            Self::Group(_) => None,
        }
    }

    pub fn as_group(&self) -> Option<&DependencyGroup> {
        match self {
            Self::Group(g) => Some(g),
            Self::Leaf(_) => None,
        }
    }
}

/// A `compositeDependency` of the FOMOD schema: an operator and a list of
/// items, each a leaf condition or another group, nested to any depth.
///
/// Used for a step's `<visible>`, an option's dependency patterns, the
/// conditional-install sets and the mod-wide `<moduleDependencies>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyGroup {
    pub operator: LogicalOperator,
    pub items: Vec<DependencyItem>,
}

impl DependencyGroup {
    pub fn new(operator: LogicalOperator) -> Self {
        Self {
            operator,
            items: Vec::new(),
        }
    }

    /// A flat group (one level of leaves) under `operator`.
    pub fn from_leaves(operator: LogicalOperator, leaves: Vec<Dependency>) -> Self {
        Self {
            operator,
            items: leaves.into_iter().map(DependencyItem::Leaf).collect(),
        }
    }

    /// No item at all (an empty group always holds, and is omitted on save
    /// where the schema allows).
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Whether any item is a nested group.
    pub fn has_groups(&self) -> bool {
        self.items.iter().any(|i| matches!(i, DependencyItem::Group(_)))
    }

    /// The operator and the leaves when the group has no nested group
    /// (the shape the model used to be limited to), `None` otherwise.
    pub fn flat(&self) -> Option<(LogicalOperator, Vec<&Dependency>)> {
        if self.has_groups() {
            return None;
        }
        Some((
            self.operator,
            self.items.iter().filter_map(DependencyItem::as_leaf).collect(),
        ))
    }

    /// Every leaf, depth-first, in document order.
    pub fn leaves(&self) -> Box<dyn Iterator<Item = &Dependency> + '_> {
        Box::new(self.items.iter().flat_map(|item| match item {
            DependencyItem::Leaf(d) => Box::new(std::iter::once(d)) as Box<dyn Iterator<Item = &Dependency>>,
            DependencyItem::Group(g) => g.leaves(),
        }))
    }

    /// Every leaf, mutably, depth-first.
    pub fn leaves_mut(&mut self) -> Box<dyn Iterator<Item = &mut Dependency> + '_> {
        Box::new(self.items.iter_mut().flat_map(|item| match item {
            DependencyItem::Leaf(d) => Box::new(std::iter::once(d)) as Box<dyn Iterator<Item = &mut Dependency>>,
            DependencyItem::Group(g) => g.leaves_mut(),
        }))
    }

    /// Number of leaves at every depth.
    pub fn leaf_count(&self) -> usize {
        self.leaves().count()
    }

    /// Call `f` on every leaf, depth-first.
    pub fn visit(&self, mut f: impl FnMut(&Dependency)) {
        fn walk(g: &DependencyGroup, f: &mut impl FnMut(&Dependency)) {
            for item in &g.items {
                match item {
                    DependencyItem::Leaf(d) => f(d),
                    DependencyItem::Group(sub) => walk(sub, f),
                }
            }
        }
        walk(self, &mut f);
    }

    /// Keep only the leaves `pred` accepts, at every depth (nested groups
    /// are kept even when they become empty). Returns how many were removed.
    pub fn retain_leaves(&mut self, mut pred: impl FnMut(&Dependency) -> bool) -> usize {
        fn walk(g: &mut DependencyGroup, pred: &mut impl FnMut(&Dependency) -> bool) -> usize {
            let before = g.items.len();
            g.items.retain(|item| match item {
                DependencyItem::Leaf(d) => pred(d),
                DependencyItem::Group(_) => true,
            });
            let mut n = before - g.items.len();
            for item in &mut g.items {
                if let DependencyItem::Group(sub) = item {
                    n += walk(sub, pred);
                }
            }
            n
        }
        walk(self, &mut pred)
    }

    pub fn push_leaf(&mut self, dep: Dependency) {
        self.items.push(DependencyItem::Leaf(dep));
    }

    pub fn push_group(&mut self, group: DependencyGroup) {
        self.items.push(DependencyItem::Group(group));
    }

    /// Nesting depth: 0 for a group without nested groups, 1 when it holds
    /// groups that are themselves flat, and so on.
    pub fn depth(&self) -> usize {
        self.items
            .iter()
            .filter_map(DependencyItem::as_group)
            .map(|g| g.depth() + 1)
            .max()
            .unwrap_or(0)
    }

    /// The item at `path` (an index per level, from this group down).
    /// `None` for an empty path or an index out of range.
    pub fn get_path(&self, path: &[usize]) -> Option<&DependencyItem> {
        let (&first, rest) = path.split_first()?;
        let item = self.items.get(first)?;
        if rest.is_empty() {
            return Some(item);
        }
        item.as_group()?.get_path(rest)
    }

    /// Mutable [`get_path`](Self::get_path).
    pub fn get_path_mut(&mut self, path: &[usize]) -> Option<&mut DependencyItem> {
        let (&first, rest) = path.split_first()?;
        let item = self.items.get_mut(first)?;
        if rest.is_empty() {
            return Some(item);
        }
        match item {
            DependencyItem::Group(g) => g.get_path_mut(rest),
            DependencyItem::Leaf(_) => None,
        }
    }

    /// The group at `path`: this group for an empty path, otherwise the
    /// nested group the path designates (`None` when it names a leaf).
    pub fn group_at(&self, path: &[usize]) -> Option<&DependencyGroup> {
        if path.is_empty() {
            return Some(self);
        }
        self.get_path(path)?.as_group()
    }

    /// Mutable [`group_at`](Self::group_at).
    pub fn group_at_mut(&mut self, path: &[usize]) -> Option<&mut DependencyGroup> {
        if path.is_empty() {
            return Some(self);
        }
        match self.get_path_mut(path)? {
            DependencyItem::Group(g) => Some(g),
            DependencyItem::Leaf(_) => None,
        }
    }

    /// Remove and return the item at `path`.
    pub fn remove_path(&mut self, path: &[usize]) -> Option<DependencyItem> {
        let (&last, parent) = path.split_last()?;
        let group = self.group_at_mut(parent)?;
        if last < group.items.len() {
            Some(group.items.remove(last))
        } else {
            None
        }
    }
}

/// Where a condition group lives in the project (0-based indices).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionSite {
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

/// Condition flag (CCondition from C++)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionFlag {
    pub name: String,
    pub value: String,
}

impl ConditionFlag {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// Install file (CFile from C++)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstallFile {
    pub file_type: FileType,
    pub source: String,
    pub destination: String,
    pub priority: u32,
    /// `alwaysInstall` attribute: install even when the option is not selected.
    #[serde(default)]
    pub always_install: bool,
    /// `installIfUsable` attribute: install whenever the option is usable.
    #[serde(default)]
    pub install_if_usable: bool,
}

impl InstallFile {
    pub fn new_file(source: impl Into<String>) -> Self {
        // Destination is left empty by default: an empty destination installs
        // the item at the root of the game's Data folder, which is the correct
        // and predictable default. It used to be auto-derived from the source
        // (see `get_proper_destination_path`), but that produced wrong paths
        // for packages whose folders are not recognizable game directories
        // (e.g. "00 Data/1. Ultra Version"). The user sets the destination
        // explicitly in the Files table when a different target is needed.
        Self {
            file_type: FileType::File,
            source: source.into(),
            destination: String::new(),
            priority: 0,
            always_install: false,
            install_if_usable: false,
        }
    }

    pub fn new_folder(source: impl Into<String>) -> Self {
        Self {
            file_type: FileType::Folder,
            source: source.into(),
            destination: String::new(),
            priority: 0,
            always_install: false,
            install_if_usable: false,
        }
    }
}

/// Get proper destination path (from getProperDestinationPath in C++)
/// Strips leading directories until finding a known game folder
pub fn get_proper_destination_path(path: &str) -> String {
    let path_lower = path.to_lowercase();

    // Check for plugin files (.esp, .esm, .esl, .ba2)
    if path_lower.ends_with(".esp")
        || path_lower.ends_with(".esm")
        || path_lower.ends_with(".esl")
        || path_lower.ends_with(".ba2")
    {
        // Return just the filename
        if let Some(pos) = path.rfind('\\') {
            return path[pos + 1..].to_string();
        }
        if let Some(pos) = path.rfind('/') {
            return path[pos + 1..].to_string();
        }
        return path.to_string();
    }

    // Known Bethesda game folders
    let known_folders = [
        "strings",
        "textures",
        "music",
        "sound",
        "interface",
        "meshes",
        "programs",
        "materials",
        "lodsettings",
        "vis",
        "misc",
        "scripts",
        "shadersfx",
        "mcm",
        "seq",
        "grass",
        "terrain",
        "lod",
        "geometries",
        "animations",
        "actors",
        "video",
        "voices",
        "facegen",
        "landscape",
    ];

    // Split path by backslash or forward slash
    let parts: Vec<&str> = path.split(['\\', '/']).collect();

    for (i, part) in parts.iter().enumerate() {
        let part_lower = part.to_lowercase();
        if known_folders.contains(&part_lower.as_str()) {
            // Return from this folder onwards
            return parts[i..].join("\\");
        }
    }

    path.to_string()
}

/// Dependency pattern (CDependencyPattern from C++): the option takes
/// `pattern_type` when `condition` holds.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "DependencyPatternCompat")]
pub struct DependencyPattern {
    pub pattern_type: String, // Plugin type name for this pattern
    pub condition: DependencyGroup,
}

impl DependencyPattern {
    pub fn new() -> Self {
        Self {
            pattern_type: "Optional".to_string(),
            condition: DependencyGroup::default(),
        }
    }
}

/// Old JSON shape of a pattern (`operator` + flat `dependencies`), still
/// accepted when loading templates saved before nested groups existed.
#[derive(Deserialize)]
struct DependencyPatternCompat {
    #[serde(default)]
    pattern_type: String,
    #[serde(default)]
    condition: Option<DependencyGroup>,
    #[serde(default)]
    operator: Option<LogicalOperator>,
    #[serde(default)]
    dependencies: Option<Vec<Dependency>>,
}

/// Merge the new `condition` field with the old `operator` / flat list.
fn compat_group(
    condition: Option<DependencyGroup>,
    operator: Option<LogicalOperator>,
    dependencies: Option<Vec<Dependency>>,
) -> DependencyGroup {
    match condition {
        Some(g) => g,
        None => DependencyGroup::from_leaves(operator.unwrap_or_default(), dependencies.unwrap_or_default()),
    }
}

impl From<DependencyPatternCompat> for DependencyPattern {
    fn from(c: DependencyPatternCompat) -> Self {
        Self {
            pattern_type: c.pattern_type,
            condition: compat_group(c.condition, c.operator, c.dependencies),
        }
    }
}

/// Plugin (CPlugin from C++)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub description: String,
    pub image_path: Option<String>,
    pub default_type: PluginType,
    pub condition_flags: Vec<ConditionFlag>,
    pub files: Vec<InstallFile>,
    pub dependency_patterns: Vec<DependencyPattern>,
}

impl Plugin {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            image_path: None,
            default_type: PluginType::Optional,
            condition_flags: Vec::new(),
            files: Vec::new(),
            dependency_patterns: Vec::new(),
        }
    }
}

/// Plugin group (CPluginGroup from C++)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginGroup {
    pub name: String,
    pub selection_type: SelectionType,
    pub plugins: Vec<Plugin>,
    /// `<plugins order="…">` as authored when it is not `Explicit`
    /// (`Ascending` / `Descending`); written back verbatim. XIMOD itself
    /// keeps the authored order of the options on save.
    #[serde(default)]
    pub plugins_order: Option<String>,
}

impl PluginGroup {
    pub fn new(name: impl Into<String>, selection_type: SelectionType) -> Self {
        Self {
            name: name.into(),
            selection_type,
            plugins: Vec::new(),
            plugins_order: None,
        }
    }

    /// Assign `dest` as the install destination of every file of every plugin in
    /// this group, in a single action. Returns how many files were updated.
    ///
    /// Backs the "same destination for a whole group" feature: instead of
    /// editing the destination on each option's file list, the author sets one
    /// destination for the entire group at once.
    pub fn set_all_destinations(&mut self, dest: &str) -> usize {
        let mut count = 0;
        for plugin in &mut self.plugins {
            for file in &mut plugin.files {
                file.destination = dest.to_string();
                count += 1;
            }
        }
        count
    }
}

/// Installation step (CStep from C++)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "StepCompat")]
pub struct Step {
    pub name: String,
    /// `<visible>`: the step is shown only when this group holds (an empty
    /// group means always).
    pub visibility: DependencyGroup,
    pub plugin_groups: Vec<PluginGroup>,
    /// `<optionalFileGroups order="…">` as authored when it is not
    /// `Explicit`; written back verbatim (XIMOD keeps the authored order).
    #[serde(default)]
    pub groups_order: Option<String>,
}

/// Old JSON shape of a step (`visibility_operator` + flat
/// `visibility_dependencies`), still accepted when loading older templates.
#[derive(Deserialize)]
struct StepCompat {
    #[serde(default)]
    name: String,
    #[serde(default)]
    visibility: Option<DependencyGroup>,
    #[serde(default)]
    visibility_operator: Option<LogicalOperator>,
    #[serde(default)]
    visibility_dependencies: Option<Vec<Dependency>>,
    #[serde(default)]
    plugin_groups: Vec<PluginGroup>,
    #[serde(default)]
    groups_order: Option<String>,
}

impl From<StepCompat> for Step {
    fn from(c: StepCompat) -> Self {
        Self {
            name: c.name,
            visibility: compat_group(c.visibility, c.visibility_operator, c.visibility_dependencies),
            plugin_groups: c.plugin_groups,
            groups_order: c.groups_order,
        }
    }
}

impl Step {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: DependencyGroup::default(),
            plugin_groups: Vec::new(),
            groups_order: None,
        }
    }

    /// Assign `dest` as the install destination of every file of every plugin on
    /// this step/page (across all of its groups), in a single action. Returns how
    /// many files were updated.
    pub fn set_all_destinations(&mut self, dest: &str) -> usize {
        let mut count = 0;
        for group in &mut self.plugin_groups {
            count += group.set_all_destinations(dest);
        }
        count
    }
}

/// Conditional file set (CConditionalFile from C++): `files` are installed
/// when `condition` holds.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "ConditionalFileSetCompat")]
pub struct ConditionalFileSet {
    pub condition: DependencyGroup,
    pub files: Vec<InstallFile>,
}

impl ConditionalFileSet {
    pub fn new() -> Self {
        Self {
            condition: DependencyGroup::default(),
            files: Vec::new(),
        }
    }
}

/// Old JSON shape of a conditional set (`operator` + flat `dependencies`).
#[derive(Deserialize)]
struct ConditionalFileSetCompat {
    #[serde(default)]
    condition: Option<DependencyGroup>,
    #[serde(default)]
    operator: Option<LogicalOperator>,
    #[serde(default)]
    dependencies: Option<Vec<Dependency>>,
    #[serde(default)]
    files: Vec<InstallFile>,
}

impl From<ConditionalFileSetCompat> for ConditionalFileSet {
    fn from(c: ConditionalFileSetCompat) -> Self {
        Self {
            condition: compat_group(c.condition, c.operator, c.dependencies),
            files: c.files,
        }
    }
}

/// Mod category
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ModCategory {
    Animation,
    Armour,
    Audio,
    Body,
    Clothing,
    Creatures,
    Gameplay,
    Hair,
    Items,
    Locations,
    #[default]
    Miscellaneous,
    ModdersResources,
    Npc,
    Quests,
    Textures,
    Utilities,
    Weapons,
    Custom(String),
}

impl ModCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Animation => "Animation",
            Self::Armour => "Armour",
            Self::Audio => "Audio",
            Self::Body => "Body",
            Self::Clothing => "Clothing",
            Self::Creatures => "Creatures",
            Self::Gameplay => "Gameplay",
            Self::Hair => "Hair",
            Self::Items => "Items",
            Self::Locations => "Locations",
            Self::Miscellaneous => "Miscellaneous",
            Self::ModdersResources => "Modders Resources",
            Self::Npc => "NPC",
            Self::Quests => "Quests",
            Self::Textures => "Textures",
            Self::Utilities => "Utilities",
            Self::Weapons => "Weapons",
            Self::Custom(s) => s,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Animation" => Self::Animation,
            "Armour" => Self::Armour,
            "Audio" => Self::Audio,
            "Body" => Self::Body,
            "Clothing" => Self::Clothing,
            "Creatures" => Self::Creatures,
            "Gameplay" => Self::Gameplay,
            "Hair" => Self::Hair,
            "Items" => Self::Items,
            "Locations" => Self::Locations,
            "Miscellaneous" => Self::Miscellaneous,
            "Modders Resources" => Self::ModdersResources,
            "NPC" => Self::Npc,
            "Quests" => Self::Quests,
            "Textures" => Self::Textures,
            "Utilities" => Self::Utilities,
            "Weapons" => Self::Weapons,
            other => Self::Custom(other.to_string()),
        }
    }

    pub fn predefined() -> &'static [Self] {
        &[
            Self::Animation,
            Self::Armour,
            Self::Audio,
            Self::Body,
            Self::Clothing,
            Self::Creatures,
            Self::Gameplay,
            Self::Hair,
            Self::Items,
            Self::Locations,
            Self::Miscellaneous,
            Self::ModdersResources,
            Self::Npc,
            Self::Quests,
            Self::Textures,
            Self::Utilities,
            Self::Weapons,
        ]
    }
}

/// Main XIMOD structure (root of a mod installer project)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ximod {
    pub name: String,
    pub author: String,
    pub version: String,
    /// Selected game id (matches a key in Categories.json, e.g. "skyrimSpecialEdition").
    /// Drives which category list is shown. Not part of the FOMOD spec, kept as
    /// project state.
    pub game: String,
    pub category: ModCategory,
    pub url: String,
    pub header_image: Option<String>,
    pub description: String,
    pub steps: Vec<Step>,
    pub required_files: Vec<InstallFile>,
    pub conditional_files: Vec<ConditionalFileSet>,
    /// `<moduleDependencies>` (mod-wide requirements). `None` or an empty
    /// group means the element is omitted on save.
    #[serde(default)]
    pub module_dependencies: Option<DependencyGroup>,
    /// `moduleName/@position` (`Left`, `Right`, `RightOfImage`), kept verbatim.
    #[serde(default)]
    pub title_position: Option<String>,
    /// `moduleName/@colour` (hex `RRGGBB`), kept verbatim.
    #[serde(default)]
    pub title_colour: Option<String>,
    /// `moduleImage/@showImage`.
    #[serde(default)]
    pub image_show_image: Option<bool>,
    /// `moduleImage/@showFade`.
    #[serde(default)]
    pub image_show_fade: Option<bool>,
    /// `moduleImage/@height`.
    #[serde(default)]
    pub image_height: Option<i32>,
    /// `<installSteps order="…">` as authored when it is not `Explicit`;
    /// written back verbatim. XIMOD keeps the authored order of the steps on
    /// save (it never re-sorts them).
    #[serde(default)]
    pub steps_order: Option<String>,
}

impl Ximod {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            author: String::new(),
            version: "1.0.0".to_string(),
            game: String::new(),
            category: ModCategory::Miscellaneous,
            url: String::new(),
            header_image: None,
            description: String::new(),
            steps: Vec::new(),
            required_files: Vec::new(),
            conditional_files: Vec::new(),
            module_dependencies: None,
            title_position: None,
            title_colour: None,
            image_show_image: None,
            image_show_fade: None,
            image_height: None,
            steps_order: None,
        }
    }

    /// The mod-wide requirements, if any were authored (an empty list counts
    /// as none).
    pub fn has_module_dependencies(&self) -> bool {
        self.module_dependencies.as_ref().is_some_and(|m| !m.is_empty())
    }

    /// Every condition group of the project with its kind, in project
    /// order: mod requirements, then per step its visibility and its
    /// options' patterns, then the conditional sets.
    pub fn for_each_condition<'a>(&'a self, mut f: impl FnMut(ConditionSite, &'a DependencyGroup)) {
        if let Some(m) = &self.module_dependencies {
            f(ConditionSite::Module, m);
        }
        for (si, step) in self.steps.iter().enumerate() {
            f(ConditionSite::StepVisibility { step: si }, &step.visibility);
            for (gi, group) in step.plugin_groups.iter().enumerate() {
                for (pi, plugin) in group.plugins.iter().enumerate() {
                    for (pat_i, pat) in plugin.dependency_patterns.iter().enumerate() {
                        f(
                            ConditionSite::PluginPattern {
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
        }
        for (ci, set) in self.conditional_files.iter().enumerate() {
            f(ConditionSite::CondSet { index: ci }, &set.condition);
        }
    }

    /// Mutable [`for_each_condition`](Self::for_each_condition).
    pub fn for_each_condition_mut(&mut self, mut f: impl FnMut(ConditionSite, &mut DependencyGroup)) {
        if let Some(m) = &mut self.module_dependencies {
            f(ConditionSite::Module, m);
        }
        for (si, step) in self.steps.iter_mut().enumerate() {
            f(ConditionSite::StepVisibility { step: si }, &mut step.visibility);
            for (gi, group) in step.plugin_groups.iter_mut().enumerate() {
                for (pi, plugin) in group.plugins.iter_mut().enumerate() {
                    for (pat_i, pat) in plugin.dependency_patterns.iter_mut().enumerate() {
                        f(
                            ConditionSite::PluginPattern {
                                step: si,
                                group: gi,
                                plugin: pi,
                                pattern: pat_i,
                            },
                            &mut pat.condition,
                        );
                    }
                }
            }
        }
        for (ci, set) in self.conditional_files.iter_mut().enumerate() {
            f(ConditionSite::CondSet { index: ci }, &mut set.condition);
        }
    }

    /// Every leaf condition of the project, at every depth.
    pub fn all_leaves(&self) -> Vec<&Dependency> {
        let mut out = Vec::new();
        self.for_each_condition(|_, g| out.extend(g.leaves()));
        out
    }

    /// Visit every user-facing text of the project (names, descriptions,
    /// author, flag names and values), e.g. to find the scripts it uses.
    pub fn for_each_text(&self, mut f: impl FnMut(&str)) {
        f(&self.name);
        f(&self.author);
        f(&self.description);
        for step in &self.steps {
            f(&step.name);
            for group in &step.plugin_groups {
                f(&group.name);
                for plugin in &group.plugins {
                    f(&plugin.name);
                    f(&plugin.description);
                    for flag in &plugin.condition_flags {
                        f(&flag.name);
                        f(&flag.value);
                    }
                }
            }
        }
    }

    /// Get all condition flags used in this project
    pub fn get_all_flags(&self) -> Vec<String> {
        let mut flags: indexmap::IndexSet<&str> = indexmap::IndexSet::new();
        for step in &self.steps {
            for group in &step.plugin_groups {
                for plugin in &group.plugins {
                    for flag in &plugin.condition_flags {
                        flags.insert(flag.name.as_str());
                    }
                }
            }
        }
        flags.into_iter().map(str::to_string).collect()
    }

    /// Get all flag values used in this project
    pub fn get_all_flag_values(&self) -> Vec<String> {
        let mut values: indexmap::IndexSet<&str> = indexmap::IndexSet::new();
        for step in &self.steps {
            for group in &step.plugin_groups {
                for plugin in &group.plugins {
                    for flag in &plugin.condition_flags {
                        values.insert(flag.value.as_str());
                    }
                }
            }
        }
        values.into_iter().map(str::to_string).collect()
    }

    /// Get all dependency names used in this project (flag and file
    /// leaves at every depth; step visibility, option patterns and
    /// conditional sets, in that order).
    pub fn get_all_dependency_names(&self) -> Vec<String> {
        fn take<'a>(deps: &mut indexmap::IndexSet<&'a str>, g: &'a DependencyGroup) {
            for dep in g.leaves() {
                if !dep.is_version() {
                    deps.insert(dep.name.as_str());
                }
            }
        }
        let mut deps: indexmap::IndexSet<&str> = indexmap::IndexSet::new();
        for step in &self.steps {
            take(&mut deps, &step.visibility);
            for group in &step.plugin_groups {
                for plugin in &group.plugins {
                    for pattern in &plugin.dependency_patterns {
                        take(&mut deps, &pattern.condition);
                    }
                }
            }
        }
        for cond in &self.conditional_files {
            take(&mut deps, &cond.condition);
        }
        deps.into_iter().map(str::to_string).collect()
    }

    /// Names tested by file dependencies (`<fileDependency file="…">`),
    /// in project order: step visibility, option patterns, conditional sets
    /// and mod requirements.
    pub fn get_all_file_names(&self) -> Vec<String> {
        fn take<'a>(names: &mut indexmap::IndexSet<&'a str>, g: &'a DependencyGroup) {
            for dep in g.leaves() {
                if dep.is_file() {
                    names.insert(dep.name.as_str());
                }
            }
        }
        let mut names: indexmap::IndexSet<&str> = indexmap::IndexSet::new();
        for step in &self.steps {
            take(&mut names, &step.visibility);
            for group in &step.plugin_groups {
                for plugin in &group.plugins {
                    for pattern in &plugin.dependency_patterns {
                        take(&mut names, &pattern.condition);
                    }
                }
            }
        }
        for cond in &self.conditional_files {
            take(&mut names, &cond.condition);
        }
        if let Some(m) = &self.module_dependencies {
            take(&mut names, m);
        }
        names.into_iter().map(str::to_string).collect()
    }

    /// Count total plugins
    pub fn plugin_count(&self) -> usize {
        self.steps
            .iter()
            .flat_map(|s| &s.plugin_groups)
            .map(|g| g.plugins.len())
            .sum()
    }

    /// Count total files
    pub fn file_count(&self) -> usize {
        let plugin_files: usize = self
            .steps
            .iter()
            .flat_map(|s| &s.plugin_groups)
            .flat_map(|g| &g.plugins)
            .map(|p| p.files.len())
            .sum();

        let required = self.required_files.len();
        let conditional: usize = self.conditional_files.iter().map(|c| c.files.len()).sum();

        plugin_files + required + conditional
    }

    /// Validate the project structure
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(ValidationError::NoName);
        }

        if self.steps.is_empty() && self.required_files.is_empty() {
            errors.push(ValidationError::NoSteps);
        }

        for (i, step) in self.steps.iter().enumerate() {
            if step.name.is_empty() {
                errors.push(ValidationError::EmptyStep { step: i + 1 });
            }

            for (j, group) in step.plugin_groups.iter().enumerate() {
                if group.name.is_empty() {
                    errors.push(ValidationError::EmptyGroup {
                        step: i + 1,
                        group: j + 1,
                    });
                }

                if group.plugins.is_empty() {
                    errors.push(ValidationError::NoPlugins {
                        step: i + 1,
                        group: group.name.clone(),
                    });
                }
            }
        }

        errors
    }
}

/// A validation error, independent of any language.
/// The UI layer maps each variant to a translation key + arguments,
/// so the model layer stays free of i18n dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The mod name is missing.
    NoName,
    /// No installation step and no required file.
    NoSteps,
    /// A step (1-based index) has no name.
    EmptyStep { step: usize },
    /// A group (1-based indices) has no name.
    EmptyGroup { step: usize, group: usize },
    /// A group has no plugins (carries the group name for context).
    NoPlugins { step: usize, group: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ximod_creation() {
        let ximod = Ximod::new("Test Mod");
        assert_eq!(ximod.name, "Test Mod");
        assert_eq!(ximod.version, "1.0.0");
    }

    #[test]
    fn test_proper_destination_path() {
        assert_eq!(get_proper_destination_path("textures\\test.dds"), "textures\\test.dds");
        assert_eq!(
            get_proper_destination_path("MyMod\\textures\\test.dds"),
            "textures\\test.dds"
        );
        assert_eq!(get_proper_destination_path("MyMod.esp"), "MyMod.esp");
        assert_eq!(get_proper_destination_path("Data\\MyMod.esp"), "MyMod.esp");
    }

    #[test]
    fn test_plugin_type_conversion() {
        assert_eq!(PluginType::from_str("Required"), PluginType::Required);
        assert_eq!(PluginType::from_str("Unknown"), PluginType::Optional);
    }

    #[test]
    fn group_set_all_destinations() {
        let mut g = PluginGroup::new("G", SelectionType::SelectAny);
        let mut a = Plugin::new("A");
        a.files.push(InstallFile::new_file("a1.esp"));
        a.files.push(InstallFile::new_file("a2.esp"));
        let mut b = Plugin::new("B");
        b.files.push(InstallFile::new_folder("b_tex"));
        g.plugins.push(a);
        g.plugins.push(b);

        let n = g.set_all_destinations("meshes\\mymod");
        assert_eq!(n, 3);
        for p in &g.plugins {
            for f in &p.files {
                assert_eq!(f.destination, "meshes\\mymod");
            }
        }
    }

    #[test]
    fn step_set_all_destinations_covers_every_group() {
        let mut s = Step::new("Page");
        let mut g1 = PluginGroup::new("G1", SelectionType::SelectAny);
        let mut p1 = Plugin::new("P1");
        p1.files.push(InstallFile::new_file("x.esp"));
        g1.plugins.push(p1);
        let mut g2 = PluginGroup::new("G2", SelectionType::SelectAll);
        let mut p2 = Plugin::new("P2");
        p2.files.push(InstallFile::new_file("y.esp"));
        p2.files.push(InstallFile::new_file("z.esp"));
        g2.plugins.push(p2);
        s.plugin_groups.push(g1);
        s.plugin_groups.push(g2);

        let n = s.set_all_destinations("textures");
        assert_eq!(n, 3);
        assert!(
            s.plugin_groups
                .iter()
                .flat_map(|g| &g.plugins)
                .flat_map(|p| &p.files)
                .all(|f| f.destination == "textures")
        );
    }

    /// Lot N: the group helpers (paths, leaves, depth, removal, flat view).
    #[test]
    fn dependency_group_helpers() {
        let mut g = DependencyGroup::new(LogicalOperator::And);
        assert!(g.is_empty());
        assert_eq!(g.depth(), 0);
        assert_eq!(g.flat(), Some((LogicalOperator::And, Vec::new())));
        g.push_leaf(Dependency::new_flag("a", "1"));
        let mut or = DependencyGroup::new(LogicalOperator::Or);
        or.push_leaf(Dependency::new_flag("b", "2"));
        let mut deep = DependencyGroup::new(LogicalOperator::And);
        deep.push_leaf(Dependency::new_file("X.esm", "Active"));
        deep.push_leaf(Dependency::new_game("1.6"));
        or.push_group(deep);
        g.push_group(or);
        g.push_leaf(Dependency::new_fomm("0.13"));

        assert!(!g.is_empty());
        assert!(g.has_groups());
        assert_eq!(g.flat(), None);
        assert_eq!(g.depth(), 2);
        assert_eq!(g.leaf_count(), 5);
        let names: Vec<String> = g.leaves().map(|d| d.display_name()).collect();
        assert_eq!(
            names,
            vec![
                "[Flag] a = 1",
                "[Flag] b = 2",
                "[File] X.esm (Active)",
                "[Game] >= 1.6",
                "[Mod manager] >= 0.13",
            ]
        );
        let mut visited = 0;
        g.visit(|_| visited += 1);
        assert_eq!(visited, 5);

        // Paths.
        assert_eq!(g.get_path(&[]), None);
        assert_eq!(
            g.get_path(&[0]),
            Some(&DependencyItem::Leaf(Dependency::new_flag("a", "1")))
        );
        assert_eq!(
            g.get_path(&[1, 1, 1]),
            Some(&DependencyItem::Leaf(Dependency::new_game("1.6")))
        );
        assert_eq!(g.get_path(&[1, 1, 9]), None);
        assert_eq!(g.get_path(&[0, 0]), None, "a leaf has no children");
        assert_eq!(g.group_at(&[]).map(|x| x.operator), Some(LogicalOperator::And));
        assert_eq!(g.group_at(&[1]).map(|x| x.operator), Some(LogicalOperator::Or));
        assert_eq!(g.group_at(&[0]), None);
        g.group_at_mut(&[1, 1]).unwrap().operator = LogicalOperator::Or;
        assert_eq!(g.group_at(&[1, 1]).unwrap().operator, LogicalOperator::Or);
        if let Some(DependencyItem::Leaf(d)) = g.get_path_mut(&[1, 0]) {
            d.value = "changed".into();
        }
        assert_eq!(g.leaves().nth(1).unwrap().value, "changed");
        for d in g.leaves_mut() {
            if d.is_file() {
                d.value = "Missing".into();
            }
        }
        assert_eq!(g.leaves().nth(2).unwrap().value, "Missing");

        // Removal.
        assert_eq!(g.remove_path(&[]), None);
        assert_eq!(g.remove_path(&[7]), None);
        assert_eq!(
            g.remove_path(&[1, 1, 1]),
            Some(DependencyItem::Leaf(Dependency::new_game("1.6")))
        );
        assert_eq!(g.leaf_count(), 4);
        assert!(g.remove_path(&[1]).is_some_and(|i| i.as_group().is_some()));
        assert_eq!(g.depth(), 0);
        assert_eq!(g.leaf_count(), 2);
        assert_eq!(g.retain_leaves(|d| !d.is_version()), 1);
        assert_eq!(g.leaf_count(), 1);

        // Leaf kinds.
        assert!(Dependency::new_game("1").is_version());
        assert!(!Dependency::new_game("").is_complete());
        assert!(!Dependency::new_flag("", "x").is_complete());
        assert!(Dependency::new_file("a", "").is_complete());
        assert_eq!(DependencyType::from_str("GAME"), DependencyType::Game);
        assert_eq!(DependencyType::from_str("nope"), DependencyType::Flag);
    }

    /// Lot N: the project-wide walkers see leaves at every depth.
    #[test]
    fn project_walkers_descend_into_groups() {
        let mut m = Ximod::new("W");
        let mut s = Step::new("S");
        let mut or = DependencyGroup::new(LogicalOperator::Or);
        or.push_leaf(Dependency::new_file("Deep.esm", "Active"));
        or.push_leaf(Dependency::new_flag("deep", "1"));
        s.visibility.push_group(or);
        s.visibility.push_leaf(Dependency::new_flag("top", "1"));
        m.steps.push(s);
        m.module_dependencies = Some(DependencyGroup::from_leaves(
            LogicalOperator::And,
            vec![Dependency::new_file("Mod.esm", "Active"), Dependency::new_game("1")],
        ));
        assert_eq!(m.get_all_dependency_names(), vec!["Deep.esm", "deep", "top"]);
        assert_eq!(m.get_all_file_names(), vec!["Deep.esm", "Mod.esm"]);
        assert_eq!(m.all_leaves().len(), 5);
        let mut sites = Vec::new();
        m.for_each_condition(|site, _| sites.push(site));
        assert_eq!(
            sites,
            vec![ConditionSite::Module, ConditionSite::StepVisibility { step: 0 }]
        );
        m.for_each_condition_mut(|_, g| g.operator = LogicalOperator::Or);
        assert_eq!(m.steps[0].visibility.operator, LogicalOperator::Or);
        assert!(m.has_module_dependencies());
        m.module_dependencies = Some(DependencyGroup::default());
        assert!(!m.has_module_dependencies());
    }

    #[test]
    fn set_all_destinations_on_empty_group_updates_nothing() {
        let mut g = PluginGroup::new("Empty", SelectionType::SelectAny);
        g.plugins.push(Plugin::new("NoFiles"));
        assert_eq!(g.set_all_destinations("data"), 0);
    }
}
