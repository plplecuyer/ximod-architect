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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    Flag,
    File,
}

impl DependencyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Flag => "flag",
            Self::File => "file",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "file" => Self::File,
            _ => Self::Flag,
        }
    }
}

/// Dependency (CDependency from C++)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    pub dep_type: String,  // "flag" or "file"
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

    pub fn display_name(&self) -> String {
        if self.dep_type == "flag" {
            format!("[Flag] {} = {}", self.name, self.value)
        } else {
            format!("[File] {} ({})", self.name, self.value)
        }
    }
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
}

impl InstallFile {
    pub fn new_file(source: impl Into<String>) -> Self {
        let src = source.into();
        let dst = get_proper_destination_path(&src);
        Self {
            file_type: FileType::File,
            source: src,
            destination: dst,
            priority: 0,
        }
    }

    pub fn new_folder(source: impl Into<String>) -> Self {
        let src = source.into();
        let dst = get_proper_destination_path(&src);
        Self {
            file_type: FileType::Folder,
            source: src,
            destination: dst,
            priority: 0,
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
        "strings", "textures", "music", "sound", "interface",
        "meshes", "programs", "materials", "lodsettings", "vis",
        "misc", "scripts", "shadersfx", "mcm", "seq", "grass",
        "terrain", "lod", "geometries", "animations", "actors",
        "video", "voices", "facegen", "landscape",
    ];

    // Split path by backslash or forward slash
    let parts: Vec<&str> = path.split(|c| c == '\\' || c == '/').collect();
    
    for (i, part) in parts.iter().enumerate() {
        let part_lower = part.to_lowercase();
        if known_folders.contains(&part_lower.as_str()) {
            // Return from this folder onwards
            return parts[i..].join("\\");
        }
    }

    path.to_string()
}

/// Dependency pattern (CDependencyPattern from C++)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyPattern {
    pub operator: LogicalOperator,
    pub pattern_type: String,  // Plugin type name for this pattern
    pub dependencies: Vec<Dependency>,
}

impl DependencyPattern {
    pub fn new() -> Self {
        Self {
            operator: LogicalOperator::And,
            pattern_type: "Optional".to_string(),
            dependencies: Vec::new(),
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
}

impl PluginGroup {
    pub fn new(name: impl Into<String>, selection_type: SelectionType) -> Self {
        Self {
            name: name.into(),
            selection_type,
            plugins: Vec::new(),
        }
    }
}

/// Installation step (CStep from C++)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub visibility_operator: LogicalOperator,
    pub visibility_dependencies: Vec<Dependency>,
    pub plugin_groups: Vec<PluginGroup>,
}

impl Step {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility_operator: LogicalOperator::And,
            visibility_dependencies: Vec::new(),
            plugin_groups: Vec::new(),
        }
    }
}

/// Conditional file set (CConditionalFile from C++)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConditionalFileSet {
    pub operator: LogicalOperator,
    pub dependencies: Vec<Dependency>,
    pub files: Vec<InstallFile>,
}

impl ConditionalFileSet {
    pub fn new() -> Self {
        Self {
            operator: LogicalOperator::And,
            dependencies: Vec::new(),
            files: Vec::new(),
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
        }
    }

    /// Get all condition flags used in this project
    pub fn get_all_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        
        for step in &self.steps {
            for group in &step.plugin_groups {
                for plugin in &group.plugins {
                    for flag in &plugin.condition_flags {
                        if !flags.contains(&flag.name) {
                            flags.push(flag.name.clone());
                        }
                    }
                }
            }
        }
        
        flags
    }

    /// Get all flag values used in this project
    pub fn get_all_flag_values(&self) -> Vec<String> {
        let mut values = Vec::new();
        
        for step in &self.steps {
            for group in &step.plugin_groups {
                for plugin in &group.plugins {
                    for flag in &plugin.condition_flags {
                        if !values.contains(&flag.value) {
                            values.push(flag.value.clone());
                        }
                    }
                }
            }
        }
        
        values
    }

    /// Get all dependency names used in this project
    pub fn get_all_dependency_names(&self) -> Vec<String> {
        let mut deps = Vec::new();
        
        for step in &self.steps {
            // Visibility dependencies
            for dep in &step.visibility_dependencies {
                if !deps.contains(&dep.name) {
                    deps.push(dep.name.clone());
                }
            }
            
            // Plugin dependency patterns
            for group in &step.plugin_groups {
                for plugin in &group.plugins {
                    for pattern in &plugin.dependency_patterns {
                        for dep in &pattern.dependencies {
                            if !deps.contains(&dep.name) {
                                deps.push(dep.name.clone());
                            }
                        }
                    }
                }
            }
        }
        
        // Conditional file dependencies
        for cond in &self.conditional_files {
            for dep in &cond.dependencies {
                if !deps.contains(&dep.name) {
                    deps.push(dep.name.clone());
                }
            }
        }
        
        deps
    }

    /// Count total plugins
    pub fn plugin_count(&self) -> usize {
        self.steps.iter()
            .flat_map(|s| &s.plugin_groups)
            .map(|g| g.plugins.len())
            .sum()
    }

    /// Count total files
    pub fn file_count(&self) -> usize {
        let plugin_files: usize = self.steps.iter()
            .flat_map(|s| &s.plugin_groups)
            .flat_map(|g| &g.plugins)
            .map(|p| p.files.len())
            .sum();
        
        let required = self.required_files.len();
        let conditional: usize = self.conditional_files.iter()
            .map(|c| c.files.len())
            .sum();
        
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
        assert_eq!(get_proper_destination_path("MyMod\\textures\\test.dds"), "textures\\test.dds");
        assert_eq!(get_proper_destination_path("MyMod.esp"), "MyMod.esp");
        assert_eq!(get_proper_destination_path("Data\\MyMod.esp"), "MyMod.esp");
    }

    #[test]
    fn test_plugin_type_conversion() {
        assert_eq!(PluginType::from_str("Required"), PluginType::Required);
        assert_eq!(PluginType::from_str("Unknown"), PluginType::Optional);
    }
}
