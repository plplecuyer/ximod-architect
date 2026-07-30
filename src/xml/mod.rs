//! XML serialization module
//!
//! Handles reading and writing the installer XML files (info.xml and ModuleConfig.xml)
//! Migrated from the original C++ XML load/save routines

pub mod validate;

use crate::models::*;
use anyhow::{Context, Result};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// UTF-8 BOM (Byte Order Mark)
const UTF8_BOM: &[u8] = &[0xEF, 0xBB, 0xBF];

/// Sub-directory and XML root element name required by the mod-installer
/// format specification. Mod managers (Vortex, MO2, NMM) look for this exact
/// name, so it must never be renamed even though the rest of the codebase
/// uses the "ximod" branding.
const INSTALLER_DIR: &str = "fomod";

/// Save the project to XML files
pub fn save_ximod(ximod: &Ximod, root_dir: &Path) -> Result<()> {
    // Ensure the installer output directory exists
    let ximod_dir = root_dir.join(INSTALLER_DIR);
    std::fs::create_dir_all(&ximod_dir)?;

    // Save info.xml
    save_info_xml(ximod, &ximod_dir.join("info.xml"))?;

    // Save ModuleConfig.xml
    save_module_config_xml(ximod, &ximod_dir.join("ModuleConfig.xml"))?;

    Ok(())
}

/// Save info.xml to disk (UTF-8 with BOM).
fn save_info_xml(ximod: &Ximod, path: &Path) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    write_info_xml(&mut writer, ximod, true)
}

/// Serialize info.xml to a String (no BOM), for the in-app XML editor.
pub fn info_xml_to_string(ximod: &Ximod) -> Result<String> {
    let mut buf: Vec<u8> = Vec::new();
    write_info_xml(&mut buf, ximod, false)?;
    Ok(String::from_utf8(buf)?)
}

/// Write the info.xml document to any writer (BOM optional).
fn write_info_xml<W: Write>(mut w: W, ximod: &Ximod, bom: bool) -> Result<()> {
    if bom {
        w.write_all(UTF8_BOM)?;
    }
    let mut xml_writer = Writer::new_with_indent(w, b' ', 4);

    // XML declaration
    xml_writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;
    xml_writer.write_event(Event::Text(BytesText::new("\n")))?;

    // Root element
    xml_writer.write_event(Event::Start(BytesStart::new(INSTALLER_DIR)))?;

    // Name
    write_text_element(&mut xml_writer, "Name", &ximod.name)?;

    // Author
    if !ximod.author.is_empty() {
        write_text_element(&mut xml_writer, "Author", &ximod.author)?;
    }

    // Version
    if !ximod.version.is_empty() {
        write_text_element(&mut xml_writer, "Version", &ximod.version)?;
    }

    // Website
    if !ximod.url.is_empty() {
        write_text_element(&mut xml_writer, "Website", &ximod.url)?;
    }

    // Description
    if !ximod.description.is_empty() {
        write_text_element(&mut xml_writer, "Description", &ximod.description)?;
    }

    // Groups (category)
    write_text_element(&mut xml_writer, "Groups", ximod.category.as_str())?;

    // Game (XIMOD extension, not part of the FOMOD spec). Stores the selected
    // game id so the category list can be restored on reload. Mod managers
    // ignore unknown elements in info.xml, so this is safe.
    if !ximod.game.is_empty() {
        write_text_element(&mut xml_writer, "Game", &ximod.game)?;
    }

    xml_writer.write_event(Event::End(BytesEnd::new(INSTALLER_DIR)))?;

    Ok(())
}

/// Save ModuleConfig.xml to disk (UTF-8 with BOM).
fn save_module_config_xml(ximod: &Ximod, path: &Path) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    write_module_config_xml(&mut writer, ximod, true)
}

/// Serialize ModuleConfig.xml to a String (no BOM), for the in-app XML editor.
pub fn module_config_to_string(ximod: &Ximod) -> Result<String> {
    let mut buf: Vec<u8> = Vec::new();
    write_module_config_xml(&mut buf, ximod, false)?;
    Ok(String::from_utf8(buf)?)
}

/// Write the ModuleConfig.xml document to any writer (BOM optional).
fn write_module_config_xml<W: Write>(mut w: W, ximod: &Ximod, bom: bool) -> Result<()> {
    if bom {
        w.write_all(UTF8_BOM)?;
    }
    let mut xml_writer = Writer::new_with_indent(w, b'\t', 1);

    // XML declaration
    xml_writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;
    xml_writer.write_event(Event::Text(BytesText::new("\n")))?;

    // Root element with schema
    let mut config = BytesStart::new("config");
    config.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    config.push_attribute(("xsi:noNamespaceSchemaLocation", "http://qconsulting.ca/fo3/ModConfig5.0.xsd"));
    xml_writer.write_event(Event::Start(config))?;

    // Module name
    write_text_element(&mut xml_writer, "moduleName", &ximod.name)?;

    // Module image
    if let Some(ref img) = ximod.header_image {
        let mut elem = BytesStart::new("moduleImage");
        elem.push_attribute(("path", img.as_str()));
        xml_writer.write_event(Event::Empty(elem))?;
    }

    // Required install files
    if !ximod.required_files.is_empty() {
        xml_writer.write_event(Event::Start(BytesStart::new("requiredInstallFiles")))?;
        for file in &ximod.required_files {
            write_install_file(&mut xml_writer, file)?;
        }
        xml_writer.write_event(Event::End(BytesEnd::new("requiredInstallFiles")))?;
    }

    // Install steps
    if !ximod.steps.is_empty() {
        let mut steps_elem = BytesStart::new("installSteps");
        steps_elem.push_attribute(("order", "Explicit"));
        xml_writer.write_event(Event::Start(steps_elem))?;

        for step in &ximod.steps {
            write_step(&mut xml_writer, step)?;
        }

        xml_writer.write_event(Event::End(BytesEnd::new("installSteps")))?;
    }

    // Conditional file installs
    if !ximod.conditional_files.is_empty() {
        xml_writer.write_event(Event::Start(BytesStart::new("conditionalFileInstalls")))?;
        xml_writer.write_event(Event::Start(BytesStart::new("patterns")))?;

        for cond in &ximod.conditional_files {
            write_conditional_pattern(&mut xml_writer, cond)?;
        }

        xml_writer.write_event(Event::End(BytesEnd::new("patterns")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("conditionalFileInstalls")))?;
    }

    xml_writer.write_event(Event::End(BytesEnd::new("config")))?;

    Ok(())
}

/// Write a text element
fn write_text_element<W: Write>(writer: &mut Writer<W>, name: &str, content: &str) -> Result<()> {
    writer.write_event(Event::Start(BytesStart::new(name)))?;
    writer.write_event(Event::Text(BytesText::new(content)))?;
    writer.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

/// Write an install file element
fn write_install_file<W: Write>(writer: &mut Writer<W>, file: &InstallFile) -> Result<()> {
    let tag_name = file.file_type.as_str();
    let mut elem = BytesStart::new(tag_name);
    elem.push_attribute(("source", file.source.as_str()));
    elem.push_attribute(("destination", file.destination.as_str()));
    elem.push_attribute(("priority", file.priority.to_string().as_str()));
    writer.write_event(Event::Empty(elem))?;
    Ok(())
}

/// Write an install step
fn write_step<W: Write>(writer: &mut Writer<W>, step: &Step) -> Result<()> {
    let mut step_elem = BytesStart::new("installStep");
    step_elem.push_attribute(("name", step.name.as_str()));
    writer.write_event(Event::Start(step_elem))?;

    // Visibility conditions
    if !step.visibility_dependencies.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("visible")))?;
        write_dependencies(writer, &step.visibility_dependencies, step.visibility_operator)?;
        writer.write_event(Event::End(BytesEnd::new("visible")))?;
    }

    // Optional file groups
    writer.write_event(Event::Start(BytesStart::new("optionalFileGroups")))?;

    for group in &step.plugin_groups {
        write_plugin_group(writer, group)?;
    }

    writer.write_event(Event::End(BytesEnd::new("optionalFileGroups")))?;
    writer.write_event(Event::End(BytesEnd::new("installStep")))?;

    Ok(())
}

/// Write a plugin group
fn write_plugin_group<W: Write>(writer: &mut Writer<W>, group: &PluginGroup) -> Result<()> {
    let mut group_elem = BytesStart::new("group");
    group_elem.push_attribute(("name", group.name.as_str()));
    group_elem.push_attribute(("type", group.selection_type.as_str()));
    writer.write_event(Event::Start(group_elem))?;

    let mut plugins_elem = BytesStart::new("plugins");
    plugins_elem.push_attribute(("order", "Explicit"));
    writer.write_event(Event::Start(plugins_elem))?;

    for plugin in &group.plugins {
        write_plugin(writer, plugin)?;
    }

    writer.write_event(Event::End(BytesEnd::new("plugins")))?;
    writer.write_event(Event::End(BytesEnd::new("group")))?;

    Ok(())
}

/// Write a plugin
fn write_plugin<W: Write>(writer: &mut Writer<W>, plugin: &Plugin) -> Result<()> {
    let mut plugin_elem = BytesStart::new("plugin");
    plugin_elem.push_attribute(("name", plugin.name.as_str()));
    writer.write_event(Event::Start(plugin_elem))?;

    // Description
    write_text_element(writer, "description", &plugin.description)?;

    // Image
    if let Some(ref img) = plugin.image_path {
        let mut img_elem = BytesStart::new("image");
        img_elem.push_attribute(("path", img.as_str()));
        writer.write_event(Event::Empty(img_elem))?;
    }

    // Condition flags
    if !plugin.condition_flags.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("conditionFlags")))?;
        for flag in &plugin.condition_flags {
            let mut flag_elem = BytesStart::new("flag");
            flag_elem.push_attribute(("name", flag.name.as_str()));
            writer.write_event(Event::Start(flag_elem))?;
            writer.write_event(Event::Text(BytesText::new(&flag.value)))?;
            writer.write_event(Event::End(BytesEnd::new("flag")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("conditionFlags")))?;
    }

    // Files
    if !plugin.files.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("files")))?;
        for file in &plugin.files {
            write_install_file(writer, file)?;
        }
        writer.write_event(Event::End(BytesEnd::new("files")))?;
    }

    // Type descriptor
    writer.write_event(Event::Start(BytesStart::new("typeDescriptor")))?;

    if plugin.dependency_patterns.is_empty() {
        // Simple type
        let mut type_elem = BytesStart::new("type");
        type_elem.push_attribute(("name", plugin.default_type.as_str()));
        writer.write_event(Event::Empty(type_elem))?;
    } else {
        // Dependency type
        writer.write_event(Event::Start(BytesStart::new("dependencyType")))?;

        // Default type
        let mut default_elem = BytesStart::new("defaultType");
        default_elem.push_attribute(("name", plugin.default_type.as_str()));
        writer.write_event(Event::Empty(default_elem))?;

        // Patterns
        writer.write_event(Event::Start(BytesStart::new("patterns")))?;
        for pattern in &plugin.dependency_patterns {
            writer.write_event(Event::Start(BytesStart::new("pattern")))?;

            // Dependencies
            write_dependencies(writer, &pattern.dependencies, pattern.operator)?;

            // Type
            let mut type_elem = BytesStart::new("type");
            type_elem.push_attribute(("name", pattern.pattern_type.as_str()));
            writer.write_event(Event::Empty(type_elem))?;

            writer.write_event(Event::End(BytesEnd::new("pattern")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("patterns")))?;

        writer.write_event(Event::End(BytesEnd::new("dependencyType")))?;
    }

    writer.write_event(Event::End(BytesEnd::new("typeDescriptor")))?;
    writer.write_event(Event::End(BytesEnd::new("plugin")))?;

    Ok(())
}

/// Write dependencies
fn write_dependencies<W: Write>(
    writer: &mut Writer<W>,
    deps: &[Dependency],
    operator: LogicalOperator,
) -> Result<()> {
    let mut deps_elem = BytesStart::new("dependencies");
    deps_elem.push_attribute(("operator", operator.as_str()));
    writer.write_event(Event::Start(deps_elem))?;

    for dep in deps {
        if dep.dep_type == "file" {
            let mut elem = BytesStart::new("fileDependency");
            elem.push_attribute(("file", dep.name.as_str()));
            elem.push_attribute(("state", dep.value.as_str()));
            writer.write_event(Event::Empty(elem))?;
        } else {
            let mut elem = BytesStart::new("flagDependency");
            elem.push_attribute(("flag", dep.name.as_str()));
            elem.push_attribute(("value", dep.value.as_str()));
            writer.write_event(Event::Empty(elem))?;
        }
    }

    writer.write_event(Event::End(BytesEnd::new("dependencies")))?;
    Ok(())
}

/// Write a conditional file pattern
fn write_conditional_pattern<W: Write>(
    writer: &mut Writer<W>,
    cond: &ConditionalFileSet,
) -> Result<()> {
    writer.write_event(Event::Start(BytesStart::new("pattern")))?;

    // Dependencies
    if !cond.dependencies.is_empty() {
        write_dependencies(writer, &cond.dependencies, cond.operator)?;
    }

    // Files
    if !cond.files.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("files")))?;
        for file in &cond.files {
            write_install_file(writer, file)?;
        }
        writer.write_event(Event::End(BytesEnd::new("files")))?;
    }

    writer.write_event(Event::End(BytesEnd::new("pattern")))?;
    Ok(())
}

// ============================================================================
// Loading functions
// ============================================================================

/// Load the project from XML files
/// Read an XML file into a UTF-8 `String`, auto-detecting the encoding from the
/// byte-order mark (BOM).
///
/// FOMOD files in the wild use different encodings: XIMOD writes UTF-8 (with
/// BOM), but the original C++ tool and several XML editors save as UTF-16 LE.
/// quick-xml only decodes UTF-8, so we transcode here before parsing. Supports
/// UTF-8 (with or without BOM) and UTF-16 LE/BE; the BOM is stripped.
fn read_xml_to_string(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    // UTF-16 LE BOM: FF FE
    if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
        let units: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        return String::from_utf16(&units).context("Invalid UTF-16 LE content");
    }

    // UTF-16 BE BOM: FE FF
    if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        let units: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        return String::from_utf16(&units).context("Invalid UTF-16 BE content");
    }

    // UTF-8 with BOM (EF BB BF): skip it; otherwise assume plain UTF-8.
    let start = if bytes.starts_with(UTF8_BOM) { 3 } else { 0 };
    String::from_utf8(bytes[start..].to_vec()).context("Invalid UTF-8 content")
}

pub fn load_ximod(root_dir: &Path) -> Result<Ximod> {
    let mut ximod = Ximod::default();

    let ximod_dir = root_dir.join(INSTALLER_DIR);
    let info_path = ximod_dir.join("info.xml");
    let config_path = ximod_dir.join("ModuleConfig.xml");

    // Load info.xml if exists
    if info_path.exists() {
        load_info_xml(&mut ximod, &info_path)?;
    }

    // Load ModuleConfig.xml if exists
    if config_path.exists() {
        load_module_config_xml(&mut ximod, &config_path)?;
    }

    Ok(ximod)
}

/// Load info.xml from disk into the model.
fn load_info_xml(ximod: &mut Ximod, path: &Path) -> Result<()> {
    let content = read_xml_to_string(path).context("Failed to read info.xml")?;
    parse_info_xml(&content, ximod)
}

/// Parse an info.xml string into the model (used by the in-app XML editor).
pub fn parse_info_xml(content: &str, ximod: &mut Ximod) -> Result<()> {
    let mut reader = Reader::from_reader(content.as_bytes());
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut current_element = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape()?.to_string();
                match current_element.as_str() {
                    "Name" => ximod.name = text,
                    "Author" => ximod.author = text,
                    "Version" => ximod.version = text,
                    "Groups" => ximod.category = ModCategory::from_str(&text),
                    "Website" => ximod.url = text,
                    "Description" => ximod.description = text,
                    // XIMOD extension: restore the selected game id.
                    "Game" => ximod.game = text,
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("Error parsing info.xml: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

/// Load ModuleConfig.xml from disk into the model.
fn load_module_config_xml(ximod: &mut Ximod, path: &Path) -> Result<()> {
    let content = read_xml_to_string(path).context("Failed to read ModuleConfig.xml")?;
    parse_module_config_xml(&content, ximod)
}

/// Load a donor `ModuleConfig.xml` file into a fresh model, for the
/// "Merge FOMOD" feature.
///
/// Only the installation data (steps, required files, conditional files) is
/// meaningful to the caller. Any metadata parsed from the donor
/// (module name, header image) stays confined to the returned value, so the
/// merge can append the installation data without disturbing the recipient's
/// own identity. This is the deliberate, clean counterpart of the original
/// C++ tool, whose merge silently overwrote the recipient's header image.
pub fn load_module_config_file(path: &Path) -> Result<Ximod> {
    let mut donor = Ximod::default();
    load_module_config_xml(&mut donor, path)?;
    Ok(donor)
}

/// Parse a ModuleConfig.xml string into the model (used by the in-app XML editor).
pub fn parse_module_config_xml(content: &str, ximod: &mut Ximod) -> Result<()> {
    let mut reader = Reader::from_reader(content.as_bytes());
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    // Parser state
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Section {
        None,
        RequiredInstallFiles,
        InstallSteps,
        ConditionalFileInstalls,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum DependencyContext {
        None,
        Visibility,
        Pattern,
        Conditional,
    }

    let mut section = Section::None;
    let mut dep_context = DependencyContext::None;
    let mut current_operator = LogicalOperator::And;

    let mut current_step: Option<Step> = None;
    let mut current_group: Option<PluginGroup> = None;
    let mut current_plugin: Option<Plugin> = None;
    let mut current_pattern: Option<DependencyPattern> = None;
    let mut current_cond: Option<ConditionalFileSet> = None;
    let mut in_type_descriptor = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();
                stack.push(name.clone());

                match name.as_str() {
                    "requiredinstallfiles" => section = Section::RequiredInstallFiles,
                    "installsteps" => section = Section::InstallSteps,
                    "conditionalfileinstalls" => section = Section::ConditionalFileInstalls,

                    "installstep" => {
                        let mut step = Step::new("");
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                step.name = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                        current_step = Some(step);
                    }

                    "visible" => {
                        dep_context = DependencyContext::Visibility;
                    }

                    "group" => {
                        let mut group = PluginGroup::new("", SelectionType::SelectAny);
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"name" => group.name = String::from_utf8_lossy(&attr.value).to_string(),
                                b"type" => group.selection_type = SelectionType::from_str(&String::from_utf8_lossy(&attr.value)),
                                _ => {}
                            }
                        }
                        current_group = Some(group);
                    }

                    "plugin" => {
                        let mut plugin = Plugin::new("");
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                plugin.name = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                        current_plugin = Some(plugin);
                    }

                    "typedescriptor" => {
                        in_type_descriptor = true;
                    }

                    "pattern" => {
                        if section == Section::ConditionalFileInstalls {
                            current_cond = Some(ConditionalFileSet::new());
                            dep_context = DependencyContext::Conditional;
                        } else if in_type_descriptor {
                            current_pattern = Some(DependencyPattern::new());
                            dep_context = DependencyContext::Pattern;
                        }
                    }

                    "dependencies" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"operator" {
                                current_operator = LogicalOperator::from_str(&String::from_utf8_lossy(&attr.value));
                            }
                        }
                        
                        // Apply operator to current context
                        match dep_context {
                            DependencyContext::Visibility => {
                                if let Some(ref mut step) = current_step {
                                    step.visibility_operator = current_operator;
                                }
                            }
                            DependencyContext::Pattern => {
                                if let Some(ref mut pattern) = current_pattern {
                                    pattern.operator = current_operator;
                                }
                            }
                            DependencyContext::Conditional => {
                                if let Some(ref mut cond) = current_cond {
                                    cond.operator = current_operator;
                                }
                            }
                            _ => {}
                        }
                    }

                    "moduleimage" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"path" {
                                ximod.header_image = Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }

                    "flag" => {
                        // Condition flag in plugin
                        if let Some(ref mut plugin) = current_plugin {
                            let mut flag = ConditionFlag::default();
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"name" {
                                    flag.name = String::from_utf8_lossy(&attr.value).to_string();
                                }
                            }
                            plugin.condition_flags.push(flag);
                        }
                    }

                    _ => {}
                }
            }

            Ok(Event::Empty(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();

                match name.as_str() {
                    "file" | "folder" => {
                        let mut file = InstallFile::default();
                        file.file_type = if name == "folder" { FileType::Folder } else { FileType::File };
                        
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"source" => file.source = String::from_utf8_lossy(&attr.value).to_string(),
                                b"destination" => file.destination = String::from_utf8_lossy(&attr.value).to_string(),
                                b"priority" => file.priority = String::from_utf8_lossy(&attr.value).parse().unwrap_or(0),
                                _ => {}
                            }
                        }

                        match section {
                            Section::RequiredInstallFiles => ximod.required_files.push(file),
                            Section::InstallSteps => {
                                if let Some(ref mut plugin) = current_plugin {
                                    plugin.files.push(file);
                                }
                            }
                            Section::ConditionalFileInstalls => {
                                if let Some(ref mut cond) = current_cond {
                                    cond.files.push(file);
                                }
                            }
                            _ => {}
                        }
                    }

                    "flagdependency" => {
                        let mut dep = Dependency::default();
                        dep.dep_type = "flag".to_string();
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"flag" => dep.name = String::from_utf8_lossy(&attr.value).to_string(),
                                b"value" => dep.value = String::from_utf8_lossy(&attr.value).to_string(),
                                _ => {}
                            }
                        }

                        match dep_context {
                            DependencyContext::Visibility => {
                                if let Some(ref mut step) = current_step {
                                    step.visibility_dependencies.push(dep);
                                }
                            }
                            DependencyContext::Pattern => {
                                if let Some(ref mut pattern) = current_pattern {
                                    pattern.dependencies.push(dep);
                                }
                            }
                            DependencyContext::Conditional => {
                                if let Some(ref mut cond) = current_cond {
                                    cond.dependencies.push(dep);
                                }
                            }
                            _ => {}
                        }
                    }

                    "filedependency" => {
                        let mut dep = Dependency::default();
                        dep.dep_type = "file".to_string();
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"file" => dep.name = String::from_utf8_lossy(&attr.value).to_string(),
                                b"state" => dep.value = String::from_utf8_lossy(&attr.value).to_string(),
                                _ => {}
                            }
                        }

                        match dep_context {
                            DependencyContext::Visibility => {
                                if let Some(ref mut step) = current_step {
                                    step.visibility_dependencies.push(dep);
                                }
                            }
                            DependencyContext::Pattern => {
                                if let Some(ref mut pattern) = current_pattern {
                                    pattern.dependencies.push(dep);
                                }
                            }
                            DependencyContext::Conditional => {
                                if let Some(ref mut cond) = current_cond {
                                    cond.dependencies.push(dep);
                                }
                            }
                            _ => {}
                        }
                    }

                    "type" | "defaulttype" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                let type_name = String::from_utf8_lossy(&attr.value).to_string();
                                
                                if name == "defaulttype" || current_pattern.is_none() {
                                    if let Some(ref mut plugin) = current_plugin {
                                        plugin.default_type = PluginType::from_str(&type_name);
                                    }
                                } else if let Some(ref mut pattern) = current_pattern {
                                    pattern.pattern_type = type_name;
                                }
                            }
                        }
                    }

                    "image" => {
                        if let Some(ref mut plugin) = current_plugin {
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"path" {
                                    plugin.image_path = Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                    }

                    "moduleimage" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"path" {
                                ximod.header_image = Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }

                    _ => {}
                }
            }

            Ok(Event::Text(ref e)) => {
                let text = e.unescape().map(|s| s.to_string()).unwrap_or_default();
                
                if let Some(current) = stack.last() {
                    match current.as_str() {
                        "modulename" => ximod.name = text,
                        "description" => {
                            if let Some(ref mut plugin) = current_plugin {
                                plugin.description.push_str(&text);
                            }
                        }
                        "flag" => {
                            if let Some(ref mut plugin) = current_plugin {
                                if let Some(flag) = plugin.condition_flags.last_mut() {
                                    flag.value = text;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();
                stack.pop();

                match name.as_str() {
                    "installstep" => {
                        if let Some(step) = current_step.take() {
                            ximod.steps.push(step);
                        }
                    }

                    "group" => {
                        if let (Some(step), Some(group)) = (&mut current_step, current_group.take()) {
                            step.plugin_groups.push(group);
                        }
                    }

                    "plugin" => {
                        if let (Some(group), Some(plugin)) = (&mut current_group, current_plugin.take()) {
                            group.plugins.push(plugin);
                        }
                    }

                    "pattern" => {
                        if section == Section::ConditionalFileInstalls {
                            if let Some(cond) = current_cond.take() {
                                ximod.conditional_files.push(cond);
                            }
                            dep_context = DependencyContext::None;
                        } else if in_type_descriptor {
                            if let (Some(plugin), Some(pattern)) = (&mut current_plugin, current_pattern.take()) {
                                plugin.dependency_patterns.push(pattern);
                            }
                            dep_context = DependencyContext::None;
                        }
                    }

                    "visible" => {
                        dep_context = DependencyContext::None;
                    }

                    "typedescriptor" => {
                        in_type_descriptor = false;
                    }

                    "requiredinstallfiles" => section = Section::None,
                    "installsteps" => section = Section::None,
                    "conditionalfileinstalls" => section = Section::None,

                    _ => {}
                }
            }

            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("Error parsing ModuleConfig.xml: {}", e)),
            _ => {}
        }

        buf.clear();
    }

    Ok(())
}

/// A well-formedness error located in the source text.
#[derive(Debug, Clone)]
pub struct XmlError {
    /// Byte offset in the source where the error was detected.
    pub byte: usize,
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number.
    pub column: usize,
    /// Human-readable message (from quick-xml, English/technical).
    pub message: String,
}

/// Convert a byte offset to a 1-based (line, column).
fn byte_to_line_col(content: &str, byte: usize) -> (usize, usize) {
    let byte = byte.min(content.len());
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

/// Check that `content` is well-formed XML.
///
/// Returns the first error found (with position), or `None` if well-formed.
/// Used by the in-app editor for live validation while typing, so it must be
/// fast and must never panic.
pub fn check_well_formed(content: &str) -> Option<XmlError> {
    let mut reader = Reader::from_reader(content.as_bytes());
    reader.config_mut().trim_text(false);

    let mut buf = Vec::new();
    let mut depth: i32 = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => depth += 1,
            Ok(Event::End(_)) => depth -= 1,
            Ok(Event::Eof) => {
                if depth > 0 {
                    let byte = content.len();
                    let (line, column) = byte_to_line_col(content, byte);
                    return Some(XmlError {
                        byte,
                        line,
                        column,
                        message: "unexpected end of document: unclosed element".to_string(),
                    });
                }
                return None;
            }
            Ok(_) => {}
            Err(e) => {
                let byte = reader.buffer_position() as usize;
                let (line, column) = byte_to_line_col(content, byte);
                return Some(XmlError {
                    byte,
                    line,
                    column,
                    message: format!("{}", e),
                });
            }
        }
        buf.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load() {
        let temp_dir = std::env::temp_dir().join("ximod_test");
        let _ = std::fs::create_dir_all(&temp_dir);

        let mut ximod = Ximod::new("Test Mod");
        ximod.author = "Test Author".to_string();
        ximod.version = "1.0.0".to_string();
        ximod.game = "skyrimSpecialEdition".to_string();

        let mut step = Step::new("Test Step");
        let mut group = PluginGroup::new("Test Group", SelectionType::SelectAny);
        let plugin = Plugin::new("Test Plugin");
        group.plugins.push(plugin);
        step.plugin_groups.push(group);
        ximod.steps.push(step);

        assert!(save_ximod(&ximod, &temp_dir).is_ok());

        let loaded = load_ximod(&temp_dir).unwrap();
        assert_eq!(loaded.name, "Test Mod");
        assert_eq!(loaded.author, "Test Author");
        assert_eq!(loaded.game, "skyrimSpecialEdition");
        assert_eq!(loaded.steps.len(), 1);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_read_xml_utf16_le() {
        // A ModuleConfig.xml saved as UTF-16 LE with BOM (as produced by the
        // original C++ tool and some editors) must load correctly.
        let temp_dir = std::env::temp_dir().join("ximod_test_utf16");
        let fomod_dir = temp_dir.join(INSTALLER_DIR);
        let _ = std::fs::create_dir_all(&fomod_dir);

        let xml = "<?xml version=\"1.0\" encoding=\"utf-16\"?>\n\
                   <config><moduleName>Utf16 Mod</moduleName></config>";

        // Encode as UTF-16 LE with BOM.
        let mut bytes: Vec<u8> = vec![0xFF, 0xFE];
        for unit in xml.encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        std::fs::write(fomod_dir.join("ModuleConfig.xml"), &bytes).unwrap();

        let loaded = load_ximod(&temp_dir).unwrap();
        assert_eq!(loaded.name, "Utf16 Mod");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_read_xml_utf8_no_bom() {
        // Plain UTF-8 without BOM must still work.
        let temp_dir = std::env::temp_dir().join("ximod_test_utf8nobom");
        let fomod_dir = temp_dir.join(INSTALLER_DIR);
        let _ = std::fs::create_dir_all(&fomod_dir);

        let xml = "<config><moduleName>Plain Mod</moduleName></config>";
        std::fs::write(fomod_dir.join("ModuleConfig.xml"), xml.as_bytes()).unwrap();

        let loaded = load_ximod(&temp_dir).unwrap();
        assert_eq!(loaded.name, "Plain Mod");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
