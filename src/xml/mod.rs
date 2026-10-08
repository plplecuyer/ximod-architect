//! XML serialization module
//!
//! Handles reading and writing the installer XML files (info.xml and ModuleConfig.xml)
//! Migrated from the original C++ XML load/save routines

pub mod fidelity;
pub mod patch;
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

/// The installer folder of a mod root: `<root>/fomod`, found regardless of
/// the case the author used (`Fomod`, `FOMOD`…). Windows does not care, but
/// Linux and macOS do, and mod managers accept any case. Falls back to the
/// canonical lower-case name when the folder does not exist yet.
pub fn installer_dir(root_dir: &Path) -> std::path::PathBuf {
    patch::find_entry_ignore_case(root_dir, INSTALLER_DIR, true).unwrap_or_else(|| root_dir.join(INSTALLER_DIR))
}

/// `<dir>/<name>` with the case the file actually has on disk (`Info.xml`
/// is common), or the canonical name when the file does not exist.
pub fn installer_file(dir: &Path, name: &str) -> std::path::PathBuf {
    patch::find_entry_ignore_case(dir, name, false).unwrap_or_else(|| dir.join(name))
}

/// Save the project to XML files
pub fn save_ximod(ximod: &Ximod, root_dir: &Path) -> Result<()> {
    // Ensure the installer output directory exists (keeping the author's
    // casing of an existing folder and of existing files).
    let ximod_dir = installer_dir(root_dir);
    std::fs::create_dir_all(&ximod_dir)?;

    // Save info.xml
    save_info_xml(ximod, &installer_file(&ximod_dir, "info.xml"))?;

    // Save ModuleConfig.xml
    save_module_config_xml(ximod, &installer_file(&ximod_dir, "ModuleConfig.xml"))?;

    Ok(())
}

/// Save info.xml to disk (UTF-8 with BOM).
fn save_info_xml(ximod: &Ximod, path: &Path) -> Result<()> {
    write_atomically(path, |w| write_info_xml(w, ximod, true))
}

/// Write a file through a temporary sibling and rename it into place.
///
/// `File::create` truncates the target before anything is written, so a crash
/// or a disk-full error in the middle of a save used to leave an empty
/// `ModuleConfig.xml`. The buffered writer is also flushed and synced
/// explicitly: a `BufWriter` dropped without `flush()` swallows the final
/// write error, and the UI would report a successful save.
fn write_atomically(path: &Path, write: impl FnOnce(&mut BufWriter<File>) -> Result<()>) -> Result<()> {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("ximod");
    let tmp = path.with_file_name(format!("{file_name}.{}.tmp", std::process::id()));
    let result = (|| -> Result<()> {
        let mut writer =
            BufWriter::new(File::create(&tmp).with_context(|| format!("Failed to create {}", tmp.display()))?);
        write(&mut writer)?;
        writer.flush()?;
        writer.get_ref().sync_all()?;
        drop(writer);
        std::fs::rename(&tmp, path).with_context(|| format!("Failed to replace {}", path.display()))?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
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

    // Tool signature (must come after the declaration, never before it)
    write_tool_signature(&mut xml_writer)?;

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
    write_atomically(path, |w| write_module_config_xml(w, ximod, true))
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

    // Tool signature (must come after the declaration, never before it)
    write_tool_signature(&mut xml_writer)?;

    // Root element with schema
    let mut config = BytesStart::new("config");
    config.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    config.push_attribute((
        "xsi:noNamespaceSchemaLocation",
        "http://qconsulting.ca/fo3/ModConfig5.0.xsd",
    ));
    xml_writer.write_event(Event::Start(config))?;

    // Module name (with the optional title position / colour attributes,
    // kept verbatim from the imported file).
    {
        let mut elem = BytesStart::new("moduleName");
        if let Some(pos) = ximod.title_position.as_deref().filter(|s| !s.trim().is_empty()) {
            let pos = clean_attr(pos);
            elem.push_attribute(("position", pos.as_str()));
        }
        if let Some(colour) = ximod.title_colour.as_deref().filter(|s| !s.trim().is_empty()) {
            let colour = clean_attr(colour);
            elem.push_attribute(("colour", colour.as_str()));
        }
        let content = clean_text(&ximod.name);
        xml_writer.write_event(Event::Start(elem))?;
        xml_writer.write_event(Event::Text(BytesText::new(&content)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("moduleName")))?;
    }

    // Module image
    if let Some(ref img) = ximod.header_image {
        let mut elem = BytesStart::new("moduleImage");
        let img_path = clean_attr(img);
        elem.push_attribute(("path", img_path.as_str()));
        if let Some(show) = ximod.image_show_image {
            elem.push_attribute(("showImage", if show { "true" } else { "false" }));
        }
        if let Some(fade) = ximod.image_show_fade {
            elem.push_attribute(("showFade", if fade { "true" } else { "false" }));
        }
        if let Some(h) = ximod.image_height {
            elem.push_attribute(("height", h.to_string().as_str()));
        }
        xml_writer.write_event(Event::Empty(elem))?;
    }

    // Module dependencies (mod-wide requirements): omitted when empty.
    if let Some(md) = ximod.module_dependencies.as_ref().filter(|m| !m.is_empty()) {
        write_dependency_group(&mut xml_writer, "moduleDependencies", md)?;
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
        // The authored `order` (Ascending / Descending) is written back as
        // read; XIMOD itself never re-sorts the steps and keeps the authored
        // order on save. Projects created here use Explicit.
        let mut steps_elem = BytesStart::new("installSteps");
        steps_elem.push_attribute(("order", ximod.steps_order.as_deref().unwrap_or("Explicit")));
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

/// Write the "Created with…" tool signature as an XML comment.
///
/// Placed immediately after the XML declaration (a comment must never precede
/// the `<?xml ?>` declaration, which has to be the very first thing in the
/// document). The URL is included only when [`crate::APP_URL`] is set.
fn write_tool_signature<W: Write>(writer: &mut Writer<W>) -> Result<()> {
    let comment = if crate::APP_URL.is_empty() {
        format!(" Created with {} {} ", crate::APP_NAME, crate::APP_VERSION)
    } else {
        format!(
            " Created with {} {} [{}] ",
            crate::APP_NAME,
            crate::APP_VERSION,
            crate::APP_URL
        )
    };
    writer.write_event(Event::Comment(BytesText::from_escaped(comment)))?;
    writer.write_event(Event::Text(BytesText::new("\n")))?;
    Ok(())
}

/// Strip characters that must never appear in an XML attribute value: C0/C1
/// control characters (including a stray CR/LF/TAB left by a paste), the
/// Unicode replacement character U+FFFD, zero-width characters and a BOM.
/// XML 1.0 forbids raw control characters, so cleaning them here both removes
/// the "invisible garbage" users occasionally saw after a name and guarantees
/// well-formed output.
fn clean_attr(s: &str) -> String {
    s.chars().filter(|&c| !(c.is_control() || is_invisible(c))).collect()
}

/// Same as [`clean_attr`] for element text, but keeps the line breaks and
/// tabs that are legitimate inside a description.
fn clean_text(s: &str) -> String {
    s.chars()
        .filter(|&c| matches!(c, '\n' | '\r' | '\t') || !(c.is_control() || is_invisible(c)))
        .collect()
}

/// Zero-width and bidi-isolation characters that must never reach the XML.
/// U+2066..U+2069 are the Unicode isolation marks that Fluent inserts around
/// placeables; they are invisible in egui but show up as garbage in other
/// tools and in mod managers. Stripping them here also repairs files that
/// were written before the i18n layer stopped emitting them.
fn is_invisible(c: char) -> bool {
    matches!(
        c,
        '\u{FFFD}' | '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' | '\u{2066}'..='\u{2069}'
    )
}

/// Decode an attribute value, resolving XML entities (`&amp;`, `&lt;`, `&quot;`,
/// `&#10;`…). Reading the raw bytes instead would keep the escaped form, and
/// since the writer escapes again on save, every open/save cycle would turn
/// `&` into `&amp;`, then `&amp;amp;`, and so on. Falls back to the raw text if
/// an entity is unknown so a stray `&nbsp;` never aborts loading.
fn attr_str(attr: &quick_xml::events::attributes::Attribute<'_>) -> String {
    attr.unescape_value()
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| String::from_utf8_lossy(&attr.value).into_owned())
}

/// Text content of a `Text` or `CData` event. Entities are resolved for plain
/// text (falling back to the raw text on an unknown entity rather than
/// dropping the content); CDATA is taken verbatim, as the XML spec requires.
/// Third-party FOMODs frequently wrap descriptions in `<![CDATA[…]]>`, which
/// the old parser silently discarded.
fn event_text(ev: &Event<'_>) -> String {
    match ev {
        Event::Text(e) => e
            .unescape()
            .map(|s| s.into_owned())
            .unwrap_or_else(|_| String::from_utf8_lossy(e).into_owned()),
        Event::CData(e) => String::from_utf8_lossy(e).into_owned(),
        _ => String::new(),
    }
}

/// Write a text element
fn write_text_element<W: Write>(writer: &mut Writer<W>, name: &str, content: &str) -> Result<()> {
    let content = clean_text(content);
    writer.write_event(Event::Start(BytesStart::new(name)))?;
    writer.write_event(Event::Text(BytesText::new(&content)))?;
    writer.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

/// Write an install file element
fn write_install_file<W: Write>(writer: &mut Writer<W>, file: &InstallFile) -> Result<()> {
    let tag_name = file.file_type.as_str();
    let mut elem = BytesStart::new(tag_name);
    let source = clean_attr(&file.source);
    let destination = clean_attr(&file.destination);
    elem.push_attribute(("source", source.as_str()));
    elem.push_attribute(("destination", destination.as_str()));
    elem.push_attribute(("priority", file.priority.to_string().as_str()));
    // Both default to false in the schema: only written when set.
    if file.always_install {
        elem.push_attribute(("alwaysInstall", "true"));
    }
    if file.install_if_usable {
        elem.push_attribute(("installIfUsable", "true"));
    }
    writer.write_event(Event::Empty(elem))?;
    Ok(())
}

/// Write an install step
fn write_step<W: Write>(writer: &mut Writer<W>, step: &Step) -> Result<()> {
    let mut step_elem = BytesStart::new("installStep");
    let step_name = clean_attr(&step.name);
    step_elem.push_attribute(("name", step_name.as_str()));
    writer.write_event(Event::Start(step_elem))?;

    // Visibility conditions: `<visible>` wraps one `<dependencies>` group
    // (the parser folds that single wrapper back into the step's group).
    if !step.visibility.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("visible")))?;
        write_dependencies(writer, &step.visibility)?;
        writer.write_event(Event::End(BytesEnd::new("visible")))?;
    }

    // Optional file groups (authored `order` kept, see `Step::groups_order`).
    let mut groups_elem = BytesStart::new("optionalFileGroups");
    if let Some(order) = step.groups_order.as_deref() {
        groups_elem.push_attribute(("order", order));
    }
    writer.write_event(Event::Start(groups_elem))?;

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
    let group_name = clean_attr(&group.name);
    group_elem.push_attribute(("name", group_name.as_str()));
    group_elem.push_attribute(("type", group.selection_type.as_str()));
    writer.write_event(Event::Start(group_elem))?;

    // Authored `order` kept (see `PluginGroup::plugins_order`).
    let mut plugins_elem = BytesStart::new("plugins");
    plugins_elem.push_attribute(("order", group.plugins_order.as_deref().unwrap_or("Explicit")));
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
    let plugin_name = clean_attr(&plugin.name);
    plugin_elem.push_attribute(("name", plugin_name.as_str()));
    writer.write_event(Event::Start(plugin_elem))?;

    // Description
    write_text_element(writer, "description", &plugin.description)?;

    // Image
    if let Some(ref img) = plugin.image_path {
        let mut img_elem = BytesStart::new("image");
        let img_path = clean_attr(img);
        img_elem.push_attribute(("path", img_path.as_str()));
        writer.write_event(Event::Empty(img_elem))?;
    }

    // Condition flags
    if !plugin.condition_flags.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("conditionFlags")))?;
        for flag in &plugin.condition_flags {
            let mut flag_elem = BytesStart::new("flag");
            let flag_name = clean_attr(&flag.name);
            flag_elem.push_attribute(("name", flag_name.as_str()));
            writer.write_event(Event::Start(flag_elem))?;
            let flag_value = clean_attr(&flag.value);
            writer.write_event(Event::Text(BytesText::new(&flag_value)))?;
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
            write_dependencies(writer, &pattern.condition)?;

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

/// Write a `<dependencies>` group.
fn write_dependencies<W: Write>(writer: &mut Writer<W>, group: &DependencyGroup) -> Result<()> {
    write_dependency_group(writer, "dependencies", group)
}

/// Write a `<{tag} operator="…">` element holding the leaves and nested
/// groups of `group` (`dependencies`, or `moduleDependencies` at the root).
/// A nested group is written as a `<dependencies>` child, recursively.
fn write_dependency_group<W: Write>(writer: &mut Writer<W>, tag: &str, group: &DependencyGroup) -> Result<()> {
    let mut deps_elem = BytesStart::new(tag);
    deps_elem.push_attribute(("operator", group.operator.as_str()));
    writer.write_event(Event::Start(deps_elem))?;

    for item in &group.items {
        match item {
            DependencyItem::Leaf(dep) => write_dependency_leaf(writer, dep)?,
            DependencyItem::Group(sub) => write_dependency_group(writer, "dependencies", sub)?,
        }
    }

    writer.write_event(Event::End(BytesEnd::new(tag)))?;
    Ok(())
}

/// One leaf: `fileDependency`, `flagDependency`, `gameDependency` or
/// `fommDependency`.
fn write_dependency_leaf<W: Write>(writer: &mut Writer<W>, dep: &Dependency) -> Result<()> {
    let dep_name = clean_attr(&dep.name);
    let dep_value = clean_attr(&dep.value);
    let elem = match dep.kind() {
        DependencyType::File => {
            let mut elem = BytesStart::new("fileDependency");
            elem.push_attribute(("file", dep_name.as_str()));
            elem.push_attribute(("state", dep_value.as_str()));
            elem
        }
        DependencyType::Flag => {
            let mut elem = BytesStart::new("flagDependency");
            elem.push_attribute(("flag", dep_name.as_str()));
            elem.push_attribute(("value", dep_value.as_str()));
            elem
        }
        DependencyType::Game => {
            let mut elem = BytesStart::new("gameDependency");
            elem.push_attribute(("version", dep_value.as_str()));
            elem
        }
        DependencyType::Fomm => {
            let mut elem = BytesStart::new("fommDependency");
            elem.push_attribute(("version", dep_value.as_str()));
            elem
        }
    };
    writer.write_event(Event::Empty(elem))?;
    Ok(())
}

/// Write a conditional file pattern
fn write_conditional_pattern<W: Write>(writer: &mut Writer<W>, cond: &ConditionalFileSet) -> Result<()> {
    writer.write_event(Event::Start(BytesStart::new("pattern")))?;

    // Dependencies
    if !cond.condition.is_empty() {
        write_dependencies(writer, &cond.condition)?;
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
    let bytes = std::fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;

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
    load_ximod_with_report(root_dir).map(|(ximod, _)| ximod)
}

/// [`load_ximod`] plus the list of constructs of `ModuleConfig.xml` the
/// model cannot represent (see [`fidelity::scan_module_config`]): they are
/// lost on the next save, so the caller should warn about them.
pub fn load_ximod_with_report(root_dir: &Path) -> Result<(Ximod, Vec<fidelity::Unmodelled>)> {
    load_from_installer_dir(&installer_dir(root_dir))
}

/// Load a project from a folder that holds `info.xml` and `ModuleConfig.xml`
/// directly (a `fomod/` folder, or one of the backup folders under
/// `fomod/backups/`), with the fidelity report of `ModuleConfig.xml`.
pub fn load_from_installer_dir(ximod_dir: &Path) -> Result<(Ximod, Vec<fidelity::Unmodelled>)> {
    let mut ximod = Ximod::default();
    let mut report = Vec::new();

    let info_path = installer_file(ximod_dir, "info.xml");
    let config_path = installer_file(ximod_dir, "ModuleConfig.xml");

    // Load info.xml if exists
    if info_path.exists() {
        load_info_xml(&mut ximod, &info_path)?;
    }

    // Load ModuleConfig.xml if exists
    if config_path.exists() {
        let content = read_xml_to_string(&config_path).context("Failed to read ModuleConfig.xml")?;
        parse_module_config_xml(&content, &mut ximod)?;
        report = fidelity::scan_module_config(&content);
    }

    Ok((ximod, report))
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
            Ok(ev @ (Event::Text(_) | Event::CData(_))) => {
                let text = event_text(&ev);
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
        /// Inside `<moduleDependencies>` (mod-wide requirements).
        Module,
    }

    /// `order` attribute of a list element: `None` for the default
    /// (`Explicit`), the authored value otherwise.
    fn order_attr(e: &BytesStart) -> Option<String> {
        e.attributes()
            .flatten()
            .find(|a| a.key.as_ref() == b"order")
            .map(|a| attr_str(&a))
            .filter(|v| !v.eq_ignore_ascii_case("Explicit") && !v.trim().is_empty())
    }

    /// `operator` attribute of a dependency list (`And` by default).
    fn operator_attr(e: &BytesStart) -> LogicalOperator {
        e.attributes()
            .flatten()
            .find(|a| a.key.as_ref() == b"operator")
            .map(|a| LogicalOperator::from_str(&attr_str(&a)))
            .unwrap_or_default()
    }

    /// Fold `stack` (outermost first) into its root group, closing every
    /// unclosed nested level, and hand the root to `assign`. A `<visible>`
    /// whose only child is one `<dependencies>` group (the form XIMOD
    /// writes) folds to that inner group, which is equivalent and keeps
    /// the round trip stable.
    fn finish_group(stack: &mut Vec<DependencyGroup>, fold_single: bool, assign: impl FnOnce(DependencyGroup)) {
        while stack.len() > 1 {
            let inner = stack.pop().expect("len > 1");
            stack.last_mut().expect("len >= 1").push_group(inner);
        }
        let Some(mut root) = stack.pop() else {
            return;
        };
        if fold_single
            && root.items.len() == 1
            && let Some(DependencyItem::Group(_)) = root.items.first()
            && let Some(DependencyItem::Group(inner)) = root.items.pop()
        {
            root = inner;
        }
        assign(root);
    }

    let mut section = Section::None;
    let mut dep_context = DependencyContext::None;
    // Dependency groups being read, outermost first (the root of the current
    // `visible` / `moduleDependencies` / pattern `dependencies` at index 0).
    let mut group_stack: Vec<DependencyGroup> = Vec::new();

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
                    "installsteps" => {
                        section = Section::InstallSteps;
                        ximod.steps_order = order_attr(e);
                    }
                    "conditionalfileinstalls" => section = Section::ConditionalFileInstalls,

                    "modulename" => {
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"position" => ximod.title_position = Some(attr_str(&attr)),
                                b"colour" => ximod.title_colour = Some(attr_str(&attr)),
                                _ => {}
                            }
                        }
                    }

                    "moduledependencies" => {
                        // The element is itself the root group.
                        group_stack = vec![DependencyGroup::new(operator_attr(e))];
                        dep_context = DependencyContext::Module;
                    }

                    "installstep" => {
                        let mut step = Step::new("");
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                step.name = attr_str(&attr);
                            }
                        }
                        current_step = Some(step);
                    }

                    "optionalfilegroups" => {
                        if let Some(ref mut step) = current_step {
                            step.groups_order = order_attr(e);
                        }
                    }

                    "plugins" => {
                        if let Some(ref mut group) = current_group {
                            group.plugins_order = order_attr(e);
                        }
                    }

                    "visible" => {
                        // `<visible operator="…">` is itself a dependency
                        // group in the schema: its conditions may be direct
                        // children, or wrapped in one `<dependencies>` (the
                        // form XIMOD writes, folded back on `</visible>`).
                        dep_context = DependencyContext::Visibility;
                        group_stack = vec![DependencyGroup::new(operator_attr(e))];
                    }

                    "group" => {
                        let mut group = PluginGroup::new("", SelectionType::SelectAny);
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"name" => group.name = attr_str(&attr),
                                b"type" => group.selection_type = SelectionType::from_str(&attr_str(&attr)),
                                _ => {}
                            }
                        }
                        current_group = Some(group);
                    }

                    "plugin" => {
                        let mut plugin = Plugin::new("");
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                plugin.name = attr_str(&attr);
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
                            group_stack.clear();
                        } else if in_type_descriptor {
                            current_pattern = Some(DependencyPattern::new());
                            dep_context = DependencyContext::Pattern;
                            group_stack.clear();
                        }
                    }

                    // The root group of a pattern, or a group nested in the
                    // current one (any depth).
                    "dependencies" if dep_context != DependencyContext::None => {
                        group_stack.push(DependencyGroup::new(operator_attr(e)));
                    }

                    "moduleimage" => {
                        parse_module_image(e, ximod);
                    }

                    "flag" => {
                        // Condition flag in plugin
                        if let Some(ref mut plugin) = current_plugin {
                            let mut flag = ConditionFlag::default();
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"name" {
                                    flag.name = attr_str(&attr);
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
                        let mut file = InstallFile {
                            file_type: if name == "folder" {
                                FileType::Folder
                            } else {
                                FileType::File
                            },
                            ..InstallFile::default()
                        };

                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"source" => file.source = attr_str(&attr),
                                b"destination" => file.destination = attr_str(&attr),
                                b"priority" => file.priority = attr_str(&attr).parse().unwrap_or(0),
                                b"alwaysInstall" => file.always_install = xml_bool(&attr_str(&attr)),
                                b"installIfUsable" => file.install_if_usable = xml_bool(&attr_str(&attr)),
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

                    "flagdependency" | "filedependency" | "gamedependency" | "fommdependency"
                        if dep_context != DependencyContext::None =>
                    {
                        let mut dep = match name.as_str() {
                            "filedependency" => Dependency::new_file("", ""),
                            "gamedependency" => Dependency::new_game(""),
                            "fommdependency" => Dependency::new_fomm(""),
                            _ => Dependency::new_flag("", ""),
                        };
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"flag" | b"file" => dep.name = attr_str(&attr),
                                b"value" | b"state" | b"version" => dep.value = attr_str(&attr),
                                _ => {}
                            }
                        }
                        // A leaf outside any `<dependencies>` (lenient: not
                        // schema-valid under a pattern) opens the root group.
                        if group_stack.is_empty() {
                            group_stack.push(DependencyGroup::default());
                        }
                        if let Some(top) = group_stack.last_mut() {
                            top.push_leaf(dep);
                        }
                    }

                    // An empty, self-closing group.
                    "dependencies" if dep_context != DependencyContext::None => {
                        let g = DependencyGroup::new(operator_attr(e));
                        match group_stack.last_mut() {
                            Some(top) => top.push_group(g),
                            None => group_stack.push(g),
                        }
                    }

                    "type" | "defaulttype" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                let type_name = attr_str(&attr);

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
                                    plugin.image_path = Some(attr_str(&attr));
                                }
                            }
                        }
                    }

                    "moduleimage" => {
                        parse_module_image(e, ximod);
                    }

                    _ => {}
                }
            }

            Ok(ev @ (Event::Text(_) | Event::CData(_))) => {
                let text = event_text(&ev);

                if let Some(current) = stack.last() {
                    match current.as_str() {
                        "modulename" => ximod.name = text,
                        "description" => {
                            if let Some(ref mut plugin) = current_plugin {
                                plugin.description.push_str(&text);
                            }
                        }
                        "flag" => {
                            if let Some(ref mut plugin) = current_plugin
                                && let Some(flag) = plugin.condition_flags.last_mut()
                            {
                                flag.value = text;
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

                    "dependencies" => {
                        // A closed nested group joins its parent; the root
                        // of a pattern waits for `</pattern>` (so that a
                        // stray leaf after it still lands somewhere sane).
                        if dep_context != DependencyContext::None
                            && group_stack.len() > 1
                            && let Some(inner) = group_stack.pop()
                            && let Some(parent) = group_stack.last_mut()
                        {
                            parent.push_group(inner);
                        }
                    }

                    "pattern" => {
                        if section == Section::ConditionalFileInstalls {
                            if let Some(mut cond) = current_cond.take() {
                                finish_group(&mut group_stack, false, |g| cond.condition = g);
                                ximod.conditional_files.push(cond);
                            }
                            group_stack.clear();
                            dep_context = DependencyContext::None;
                        } else if in_type_descriptor {
                            if let (Some(plugin), Some(mut pattern)) = (&mut current_plugin, current_pattern.take()) {
                                finish_group(&mut group_stack, false, |g| pattern.condition = g);
                                plugin.dependency_patterns.push(pattern);
                            }
                            group_stack.clear();
                            dep_context = DependencyContext::None;
                        }
                    }

                    "visible" => {
                        if let Some(ref mut step) = current_step {
                            finish_group(&mut group_stack, true, |g| step.visibility = g);
                        }
                        group_stack.clear();
                        dep_context = DependencyContext::None;
                    }

                    "moduledependencies" => {
                        finish_group(&mut group_stack, false, |g| ximod.module_dependencies = Some(g));
                        group_stack.clear();
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

/// `moduleImage` attributes: `path` plus the optional `showImage`,
/// `showFade` and `height`, kept verbatim for a faithful round-trip.
fn parse_module_image(e: &BytesStart, ximod: &mut Ximod) {
    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"path" => ximod.header_image = Some(attr_str(&attr)),
            b"showImage" => ximod.image_show_image = Some(xml_bool(&attr_str(&attr))),
            b"showFade" => ximod.image_show_fade = Some(xml_bool(&attr_str(&attr))),
            b"height" => ximod.image_height = attr_str(&attr).trim().parse().ok(),
            _ => {}
        }
    }
}

/// An XML Schema boolean (`true` / `1`, anything else is false).
fn xml_bool(s: &str) -> bool {
    let s = s.trim();
    s.eq_ignore_ascii_case("true") || s == "1"
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

    /// Unique scratch directory per test so parallel runs never collide.
    fn scratch(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("ximod_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(INSTALLER_DIR)).unwrap();
        dir
    }

    /// Regression test: attributes used to be read raw (still escaped), so
    /// every open/save cycle turned `&` into `&amp;`, then `&amp;amp;`…
    #[test]
    fn test_roundtrip_special_characters_twice() {
        let dir = scratch("roundtrip");
        let special = "Guns & Roses <\"Live\"> 'n' more";

        let mut ximod = Ximod::new(special);
        ximod.description = "Line 1 & 2\n<b>bold</b> \"quoted\"".to_string();
        let mut step = Step::new(special);
        let mut dep = Dependency::new_flag(special, special);
        dep.dep_type = "flag".to_string();
        step.visibility.push_leaf(dep);
        let mut group = PluginGroup::new(special, SelectionType::SelectExactlyOne);
        let mut plugin = Plugin::new(special);
        plugin.description = "Textures & Meshes <4K>".to_string();
        plugin.image_path = Some("fomod/images/a&b.png".to_string());
        plugin.condition_flags.push(ConditionFlag::new(special, special));
        let mut file = InstallFile::new_file("src & dir/file<1>.esp");
        file.destination = "dst \"quoted\".esp".to_string();
        plugin.files.push(file);
        group.plugins.push(plugin);
        step.plugin_groups.push(group);
        ximod.steps.push(step);
        ximod.header_image = Some("fomod/h&i.png".to_string());

        // Two full cycles: the second one is where double escaping showed up.
        save_ximod(&ximod, &dir).unwrap();
        let once = load_ximod(&dir).unwrap();
        save_ximod(&once, &dir).unwrap();
        let twice = load_ximod(&dir).unwrap();

        for loaded in [&once, &twice] {
            assert_eq!(loaded.name, special);
            assert_eq!(loaded.description, ximod.description);
            assert_eq!(loaded.header_image.as_deref(), Some("fomod/h&i.png"));
            let step = &loaded.steps[0];
            assert_eq!(step.name, special);
            let vdep = step.visibility.leaves().next().unwrap();
            assert_eq!(vdep.name, special);
            assert_eq!(vdep.value, special);
            let group = &step.plugin_groups[0];
            assert_eq!(group.name, special);
            let plugin = &group.plugins[0];
            assert_eq!(plugin.name, special);
            assert_eq!(plugin.description, "Textures & Meshes <4K>");
            assert_eq!(plugin.image_path.as_deref(), Some("fomod/images/a&b.png"));
            assert_eq!(plugin.condition_flags[0].name, special);
            assert_eq!(plugin.condition_flags[0].value, special);
            assert_eq!(plugin.files[0].source, "src & dir/file<1>.esp");
            assert_eq!(plugin.files[0].destination, "dst \"quoted\".esp");
        }

        // The file on disk must contain a single level of escaping.
        let raw = std::fs::read_to_string(dir.join(INSTALLER_DIR).join("ModuleConfig.xml")).unwrap();
        assert!(raw.contains("Guns &amp; Roses"), "{raw}");
        assert!(!raw.contains("&amp;amp;"), "double escaping: {raw}");
        assert!(!raw.contains("&amp;lt;"), "double escaping: {raw}");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_parse_cdata_and_unknown_entities() {
        let xml = r#"<?xml version="1.0"?>
<config>
  <moduleName><![CDATA[Mod <with> & CDATA]]></moduleName>
  <installSteps order="Explicit">
    <installStep name="Step &amp; one">
      <optionalFileGroups order="Explicit">
        <group name="G" type="SelectAny">
          <plugins order="Explicit">
            <plugin name="P">
              <description><![CDATA[Keep <b>this</b> & that]]></description>
              <typeDescriptor><type name="Optional"/></typeDescriptor>
            </plugin>
            <plugin name="Q">
              <description>Non-breaking&nbsp;space stays</description>
              <typeDescriptor><type name="Optional"/></typeDescriptor>
            </plugin>
          </plugins>
        </group>
      </optionalFileGroups>
    </installStep>
  </installSteps>
</config>"#;
        let mut ximod = Ximod::default();
        parse_module_config_xml(xml, &mut ximod).unwrap();
        assert_eq!(ximod.name, "Mod <with> & CDATA");
        assert_eq!(ximod.steps[0].name, "Step & one");
        let plugins = &ximod.steps[0].plugin_groups[0].plugins;
        assert_eq!(plugins[0].description, "Keep <b>this</b> & that");
        // An unknown entity must not wipe the description out.
        assert!(
            plugins[1].description.contains("space stays"),
            "{:?}",
            plugins[1].description
        );

        let info =
            r#"<?xml version="1.0"?><fomod><Name><![CDATA[A & B]]></Name><Description>x&nbsp;y</Description></fomod>"#;
        let mut ximod = Ximod::default();
        parse_info_xml(info, &mut ximod).unwrap();
        assert_eq!(ximod.name, "A & B");
        assert!(ximod.description.contains('y'));
    }

    /// Lot F1: the "cheap" constructs round-trip through parse → write →
    /// parse (moduleDependencies, alwaysInstall / installIfUsable, the
    /// moduleName / moduleImage attributes and the authored `order`s).
    #[test]
    fn test_roundtrip_fidelity_constructs() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<config xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo3/ModConfig5.0.xsd">
    <moduleName position="RightOfImage" colour="FFAA00">Faithful</moduleName>
    <moduleImage path="fomod\header.png" showImage="true" showFade="false" height="240"/>
    <moduleDependencies operator="Or">
        <fileDependency file="Skyrim.esm" state="Active"/>
        <flagDependency flag="ready" value="1"/>
    </moduleDependencies>
    <requiredInstallFiles>
        <file source="core.esp" destination="core.esp" priority="0" alwaysInstall="true"/>
        <folder source="meshes" destination="meshes" priority="2" installIfUsable="1"/>
    </requiredInstallFiles>
    <installSteps order="Ascending">
        <installStep name="S">
            <optionalFileGroups order="Descending">
                <group name="G" type="SelectAny">
                    <plugins order="Ascending">
                        <plugin name="P">
                            <description>d</description>
                            <files>
                                <file source="p.esp" destination="p.esp" priority="0" alwaysInstall="false"/>
                            </files>
                            <typeDescriptor><type name="Optional"/></typeDescriptor>
                        </plugin>
                    </plugins>
                </group>
            </optionalFileGroups>
        </installStep>
    </installSteps>
</config>"#;
        let mut once = Ximod::default();
        parse_module_config_xml(xml, &mut once).unwrap();
        let written = module_config_to_string(&once).unwrap();
        let mut twice = Ximod::default();
        parse_module_config_xml(&written, &mut twice).unwrap();

        for m in [&once, &twice] {
            assert_eq!(m.title_position.as_deref(), Some("RightOfImage"));
            assert_eq!(m.title_colour.as_deref(), Some("FFAA00"));
            assert_eq!(m.header_image.as_deref(), Some("fomod\\header.png"));
            assert_eq!(m.image_show_image, Some(true));
            assert_eq!(m.image_show_fade, Some(false));
            assert_eq!(m.image_height, Some(240));
            let md = m.module_dependencies.as_ref().expect("moduleDependencies");
            assert_eq!(md.operator, LogicalOperator::Or);
            let (_, leaves) = md.flat().expect("flat");
            assert_eq!(leaves.len(), 2);
            assert_eq!(leaves[0], &Dependency::new_file("Skyrim.esm", "Active"));
            assert_eq!(leaves[1], &Dependency::new_flag("ready", "1"));
            assert!(m.required_files[0].always_install);
            assert!(!m.required_files[0].install_if_usable);
            assert!(m.required_files[1].install_if_usable);
            assert_eq!(m.required_files[1].priority, 2);
            assert_eq!(m.steps_order.as_deref(), Some("Ascending"));
            assert_eq!(m.steps[0].groups_order.as_deref(), Some("Descending"));
            assert_eq!(m.steps[0].plugin_groups[0].plugins_order.as_deref(), Some("Ascending"));
            let pf = &m.steps[0].plugin_groups[0].plugins[0].files[0];
            assert!(!pf.always_install && !pf.install_if_usable);
        }
        // The written form keeps every attribute, and omits the false flags.
        assert!(
            written.contains(r#"<moduleName position="RightOfImage" colour="FFAA00">"#),
            "{written}"
        );
        assert!(
            written.contains(r#"showImage="true" showFade="false" height="240""#),
            "{written}"
        );
        assert!(written.contains(r#"<moduleDependencies operator="Or">"#), "{written}");
        assert!(written.contains(r#"alwaysInstall="true""#), "{written}");
        assert!(written.contains(r#"installIfUsable="true""#), "{written}");
        assert!(!written.contains(r#"alwaysInstall="false""#), "{written}");
        assert!(written.contains(r#"<installSteps order="Ascending">"#), "{written}");
        assert!(
            written.contains(r#"<optionalFileGroups order="Descending">"#),
            "{written}"
        );
        assert!(written.contains(r#"<plugins order="Ascending">"#), "{written}");
        // Schema-valid, and nothing left unmodelled.
        assert!(validate::validate_module_config(&written).is_empty());
        assert!(fidelity::scan_module_config(&written).is_empty());
        // Element order: moduleDependencies sits between moduleImage and
        // requiredInstallFiles.
        let (a, b, c) = (
            written.find("<moduleImage").unwrap(),
            written.find("<moduleDependencies").unwrap(),
            written.find("<requiredInstallFiles").unwrap(),
        );
        assert!(a < b && b < c);

        // Defaults stay out of the output: a plain project is unchanged.
        let plain = module_config_to_string(&Ximod::new("Plain")).unwrap();
        assert!(plain.contains("<moduleName>Plain</moduleName>"), "{plain}");
        assert!(!plain.contains("moduleDependencies"), "{plain}");
        // An empty moduleDependencies list is omitted, and Explicit order
        // is the default (not recorded).
        let mut empty = Ximod::new("E");
        empty.module_dependencies = Some(DependencyGroup::default());
        empty.steps.push(Step::new("S"));
        let out = module_config_to_string(&empty).unwrap();
        assert!(!out.contains("moduleDependencies"), "{out}");
        assert!(out.contains(r#"<installSteps order="Explicit">"#), "{out}");
        let mut back = Ximod::default();
        parse_module_config_xml(&out, &mut back).unwrap();
        assert_eq!(back.steps_order, None);
        assert_eq!(back.steps[0].groups_order, None);
    }

    /// `<visible operator="Or">` with bare conditions (no `<dependencies>`
    /// wrapper) keeps its operator.
    #[test]
    fn test_visible_operator_without_wrapper() {
        let xml = r#"<config><moduleName>V</moduleName><installSteps order="Explicit">
<installStep name="S"><visible operator="Or"><fileDependency file="a.esp" state="Active"/><flagDependency flag="f" value="1"/></visible>
<optionalFileGroups order="Explicit"><group name="G" type="SelectAny"><plugins order="Explicit">
<plugin name="P"><description>d</description><typeDescriptor><type name="Optional"/></typeDescriptor></plugin>
</plugins></group></optionalFileGroups></installStep></installSteps></config>"#;
        let mut m = Ximod::default();
        parse_module_config_xml(xml, &mut m).unwrap();
        assert_eq!(m.steps[0].visibility.operator, LogicalOperator::Or);
        assert_eq!(m.steps[0].visibility.leaf_count(), 2);
        assert!(!m.steps[0].visibility.has_groups());
    }

    /// Lot N: the baseline writer's output for a flat project, captured
    /// from the previous version (byte for byte, after the signature line).
    const FLAT_BASELINE: &str = concat!(
        "<config xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://qconsulting.ca/fo3/ModConfig5.0.xsd\">\n",
        "\t<moduleName>Flat &amp; Co</moduleName>\n",
        "\t<moduleImage path=\"fomod\\header.png\"/>\n",
        "\t<moduleDependencies operator=\"Or\">\n",
        "\t\t<fileDependency file=\"Skyrim.esm\" state=\"Active\"/>\n",
        "\t\t<flagDependency flag=\"ready\" value=\"1\"/>\n",
        "\t</moduleDependencies>\n",
        "\t<requiredInstallFiles>\n",
        "\t\t<file source=\"core.esp\" destination=\"\" priority=\"0\"/>\n",
        "\t</requiredInstallFiles>\n",
        "\t<installSteps order=\"Explicit\">\n",
        "\t\t<installStep name=\"Step &lt;1&gt;\">\n",
        "\t\t\t<visible>\n",
        "\t\t\t\t<dependencies operator=\"Or\">\n",
        "\t\t\t\t\t<flagDependency flag=\"res\" value=\"4K\"/>\n",
        "\t\t\t\t\t<fileDependency file=\"A.esp\" state=\"Missing\"/>\n",
        "\t\t\t\t</dependencies>\n",
        "\t\t\t</visible>\n",
        "\t\t\t<optionalFileGroups>\n",
        "\t\t\t\t<group name=\"G\" type=\"SelectExactlyOne\">\n",
        "\t\t\t\t\t<plugins order=\"Explicit\">\n",
        "\t\t\t\t\t\t<plugin name=\"P &quot;quoted&quot;\">\n",
        "\t\t\t\t\t\t\t<description>d &amp; e</description>\n",
        "\t\t\t\t\t\t\t<conditionFlags>\n",
        "\t\t\t\t\t\t\t\t<flag name=\"res\">4K</flag>\n",
        "\t\t\t\t\t\t\t</conditionFlags>\n",
        "\t\t\t\t\t\t\t<files>\n",
        "\t\t\t\t\t\t\t\t<folder source=\"tex\" destination=\"\" priority=\"0\"/>\n",
        "\t\t\t\t\t\t\t</files>\n",
        "\t\t\t\t\t\t\t<typeDescriptor>\n",
        "\t\t\t\t\t\t\t\t<dependencyType>\n",
        "\t\t\t\t\t\t\t\t\t<defaultType name=\"Optional\"/>\n",
        "\t\t\t\t\t\t\t\t\t<patterns>\n",
        "\t\t\t\t\t\t\t\t\t\t<pattern>\n",
        "\t\t\t\t\t\t\t\t\t\t\t<dependencies operator=\"And\">\n",
        "\t\t\t\t\t\t\t\t\t\t\t\t<fileDependency file=\"B.esp\" state=\"Active\"/>\n",
        "\t\t\t\t\t\t\t\t\t\t\t\t<flagDependency flag=\"x\" value=\"\"/>\n",
        "\t\t\t\t\t\t\t\t\t\t\t</dependencies>\n",
        "\t\t\t\t\t\t\t\t\t\t\t<type name=\"Required\"/>\n",
        "\t\t\t\t\t\t\t\t\t\t</pattern>\n",
        "\t\t\t\t\t\t\t\t\t\t<pattern>\n",
        "\t\t\t\t\t\t\t\t\t\t\t<dependencies operator=\"Or\">\n",
        "\t\t\t\t\t\t\t\t\t\t\t</dependencies>\n",
        "\t\t\t\t\t\t\t\t\t\t\t<type name=\"Optional\"/>\n",
        "\t\t\t\t\t\t\t\t\t\t</pattern>\n",
        "\t\t\t\t\t\t\t\t\t</patterns>\n",
        "\t\t\t\t\t\t\t\t</dependencyType>\n",
        "\t\t\t\t\t\t\t</typeDescriptor>\n",
        "\t\t\t\t\t\t</plugin>\n",
        "\t\t\t\t\t\t<plugin name=\"Q\">\n",
        "\t\t\t\t\t\t\t<description></description>\n",
        "\t\t\t\t\t\t\t<typeDescriptor>\n",
        "\t\t\t\t\t\t\t\t<type name=\"Optional\"/>\n",
        "\t\t\t\t\t\t\t</typeDescriptor>\n",
        "\t\t\t\t\t\t</plugin>\n",
        "\t\t\t\t\t</plugins>\n",
        "\t\t\t\t</group>\n",
        "\t\t\t</optionalFileGroups>\n",
        "\t\t</installStep>\n",
        "\t\t<installStep name=\"Empty\">\n",
        "\t\t\t<optionalFileGroups>\n",
        "\t\t\t</optionalFileGroups>\n",
        "\t\t</installStep>\n",
        "\t</installSteps>\n",
        "\t<conditionalFileInstalls>\n",
        "\t\t<patterns>\n",
        "\t\t\t<pattern>\n",
        "\t\t\t\t<dependencies operator=\"And\">\n",
        "\t\t\t\t\t<flagDependency flag=\"res\" value=\"4K\"/>\n",
        "\t\t\t\t</dependencies>\n",
        "\t\t\t\t<files>\n",
        "\t\t\t\t\t<file source=\"patch.esp\" destination=\"\" priority=\"0\"/>\n",
        "\t\t\t\t</files>\n",
        "\t\t\t</pattern>\n",
        "\t\t\t<pattern>\n",
        "\t\t\t</pattern>\n",
        "\t\t</patterns>\n",
        "\t</conditionalFileInstalls>\n",
        "</config>",
    );

    /// Lot N: a project with only flat conditions is written exactly as
    /// before nested groups existed.
    #[test]
    fn test_flat_output_is_byte_identical_to_baseline() {
        let mut m = Ximod::new("Flat & Co");
        m.header_image = Some("fomod\\header.png".to_string());
        m.module_dependencies = Some(DependencyGroup::from_leaves(
            LogicalOperator::Or,
            vec![
                Dependency::new_file("Skyrim.esm", "Active"),
                Dependency::new_flag("ready", "1"),
            ],
        ));
        m.required_files.push(InstallFile::new_file("core.esp"));
        let mut s = Step::new("Step <1>");
        s.visibility.operator = LogicalOperator::Or;
        s.visibility.push_leaf(Dependency::new_flag("res", "4K"));
        s.visibility.push_leaf(Dependency::new_file("A.esp", "Missing"));
        let mut g = PluginGroup::new("G", SelectionType::SelectExactlyOne);
        let mut p = Plugin::new("P \"quoted\"");
        p.description = "d & e".into();
        p.condition_flags.push(ConditionFlag::new("res", "4K"));
        p.files.push(InstallFile::new_folder("tex"));
        let mut pat = DependencyPattern::new();
        pat.pattern_type = "Required".into();
        pat.condition.push_leaf(Dependency::new_file("B.esp", "Active"));
        pat.condition.push_leaf(Dependency::new_flag("x", ""));
        p.dependency_patterns.push(pat);
        let mut pat2 = DependencyPattern::new();
        pat2.condition.operator = LogicalOperator::Or;
        p.dependency_patterns.push(pat2);
        g.plugins.push(p);
        g.plugins.push(Plugin::new("Q"));
        s.plugin_groups.push(g);
        m.steps.push(s);
        m.steps.push(Step::new("Empty"));
        let mut c = ConditionalFileSet::new();
        c.condition.push_leaf(Dependency::new_flag("res", "4K"));
        c.files.push(InstallFile::new_file("patch.esp"));
        m.conditional_files.push(c);
        m.conditional_files.push(ConditionalFileSet::new());
        let out = module_config_to_string(&m).unwrap();
        // Skip the declaration and the "Created with" signature line.
        let body = out.splitn(3, '\n').nth(2).expect("body");
        assert_eq!(body, FLAT_BASELINE);
        // And it reads back to the same model.
        let mut back = Ximod::default();
        parse_module_config_xml(&out, &mut back).unwrap();
        assert_eq!(back.steps[0].visibility, m.steps[0].visibility);
        assert_eq!(back.module_dependencies, m.module_dependencies);
        assert_eq!(
            back.steps[0].plugin_groups[0].plugins[0].dependency_patterns[0].condition,
            m.steps[0].plugin_groups[0].plugins[0].dependency_patterns[0].condition
        );
        assert_eq!(back.conditional_files[0].condition, m.conditional_files[0].condition);
    }

    /// Lot N: Aurelia-style nesting (`And(Or(a, b), c)`) in every kind of
    /// condition round-trips parse → write → parse, with the nested
    /// `<dependencies>` in the written text.
    #[test]
    fn test_nested_groups_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<config xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo3/ModConfig5.0.xsd">
    <moduleName>Aurelia UI</moduleName>
    <moduleDependencies operator="And">
        <fileDependency file="Skyrim.esm" state="Active"/>
        <dependencies operator="Or">
            <gameDependency version="1.6.1170"/>
            <fommDependency version="0.13"/>
        </dependencies>
    </moduleDependencies>
    <installSteps order="Explicit">
        <installStep name="Presets">
            <visible>
                <dependencies operator="And">
                    <dependencies operator="Or">
                        <flagDependency flag="DataVersion" value="Standard"/>
                        <flagDependency flag="DataVersion" value="Undelayed"/>
                    </dependencies>
                    <flagDependency flag="DataPreset" value="Aurelia"/>
                </dependencies>
            </visible>
            <optionalFileGroups order="Explicit">
                <group name="G" type="SelectAny">
                    <plugins order="Explicit">
                        <plugin name="P">
                            <description>d</description>
                            <typeDescriptor>
                                <dependencyType>
                                    <defaultType name="Optional"/>
                                    <patterns>
                                        <pattern>
                                            <dependencies operator="Or">
                                                <dependencies operator="And">
                                                    <flagDependency flag="a" value="1"/>
                                                    <dependencies operator="Or">
                                                        <flagDependency flag="b" value="2"/>
                                                        <fileDependency file="X.esp" state="Missing"/>
                                                    </dependencies>
                                                </dependencies>
                                                <flagDependency flag="c" value="3"/>
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
        <installStep name="Bare">
            <visible operator="Or">
                <fileDependency file="a.esp" state="Active"/>
                <dependencies operator="And">
                    <flagDependency flag="y" value="1"/>
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
                        <flagDependency flag="res" value="8K"/>
                    </dependencies>
                    <dependencies operator="And"/>
                </dependencies>
                <files>
                    <file source="p.esp" destination="p.esp" priority="0"/>
                </files>
            </pattern>
        </patterns>
    </conditionalFileInstalls>
</config>"#;
        let mut once = Ximod::default();
        parse_module_config_xml(xml, &mut once).unwrap();
        let written = module_config_to_string(&once).unwrap();
        let mut twice = Ximod::default();
        parse_module_config_xml(&written, &mut twice).unwrap();

        // The Aurelia visibility: And(Or(Standard, Undelayed), Aurelia).
        let mut inner = DependencyGroup::new(LogicalOperator::Or);
        inner.push_leaf(Dependency::new_flag("DataVersion", "Standard"));
        inner.push_leaf(Dependency::new_flag("DataVersion", "Undelayed"));
        let mut aurelia = DependencyGroup::new(LogicalOperator::And);
        aurelia.push_group(inner);
        aurelia.push_leaf(Dependency::new_flag("DataPreset", "Aurelia"));
        assert_eq!(once.steps[0].visibility, aurelia);
        assert_eq!(once.steps[0].visibility.depth(), 1);

        // Mod requirements with version leaves inside a nested Or.
        let md = once.module_dependencies.as_ref().unwrap();
        assert_eq!(md.operator, LogicalOperator::And);
        assert_eq!(md.items.len(), 2);
        let sub = md.items[1].as_group().expect("nested group");
        assert_eq!(sub.operator, LogicalOperator::Or);
        assert_eq!(
            sub.items,
            vec![
                DependencyItem::Leaf(Dependency::new_game("1.6.1170")),
                DependencyItem::Leaf(Dependency::new_fomm("0.13")),
            ]
        );

        // Three levels in the pattern.
        let pat = &once.steps[0].plugin_groups[0].plugins[0].dependency_patterns[0];
        assert_eq!(pat.pattern_type, "Required");
        assert_eq!(pat.condition.operator, LogicalOperator::Or);
        assert_eq!(pat.condition.depth(), 2);
        assert_eq!(pat.condition.leaf_count(), 4);
        assert_eq!(
            pat.condition.get_path(&[0, 1, 1]),
            Some(&DependencyItem::Leaf(Dependency::new_file("X.esp", "Missing")))
        );

        // Bare `<visible operator="Or">` with a nested group: the element
        // itself is the root group.
        let bare = &once.steps[1].visibility;
        assert_eq!(bare.operator, LogicalOperator::Or);
        assert_eq!(bare.items.len(), 2);
        assert!(bare.items[1].as_group().is_some());

        // The conditional set keeps its empty nested group.
        let cond = &once.conditional_files[0].condition;
        assert_eq!(cond.items.len(), 2);
        assert!(cond.items[1].as_group().is_some_and(|g| g.is_empty()));

        // Round trip: identical model, nested groups in the text.
        assert_eq!(twice.steps[0].visibility, once.steps[0].visibility);
        assert_eq!(twice.steps[1].visibility, once.steps[1].visibility);
        assert_eq!(twice.module_dependencies, once.module_dependencies);
        assert_eq!(
            twice.steps[0].plugin_groups[0].plugins[0].dependency_patterns[0].condition,
            pat.condition
        );
        assert_eq!(twice.conditional_files[0].condition, *cond);
        assert!(
            written.contains(
                "<dependencies operator=\"And\">\n\t\t\t\t\t<dependencies operator=\"Or\">\n\t\t\t\t\t\t<flagDependency flag=\"DataVersion\" value=\"Standard\"/>"
            ),
            "{written}"
        );
        assert!(written.contains(r#"<gameDependency version="1.6.1170"/>"#), "{written}");
        assert!(written.contains(r#"<fommDependency version="0.13"/>"#), "{written}");
        // Nothing is lost any more, and the output is schema-valid.
        assert!(fidelity::scan_module_config(xml).is_empty());
        assert!(fidelity::scan_module_config(&written).is_empty());
        assert!(
            validate::validate_module_config(&written).is_empty(),
            "{:?}",
            validate::validate_module_config(&written)
        );
    }

    /// Lot N: `gameDependency` / `fommDependency` leaves at the top level
    /// of every list survive save → load.
    #[test]
    fn test_version_leaves_round_trip() {
        let dir = scratch("versions");
        let mut m = Ximod::new("V");
        m.module_dependencies = Some(DependencyGroup::from_leaves(
            LogicalOperator::And,
            vec![Dependency::new_game("1.6.1170"), Dependency::new_fomm("0.13.21")],
        ));
        let mut s = Step::new("S");
        s.visibility.push_leaf(Dependency::new_game("1.5.97"));
        m.steps.push(s);
        let mut c = ConditionalFileSet::new();
        c.condition.push_leaf(Dependency::new_fomm("0.12"));
        c.files.push(InstallFile::new_file("x.esp"));
        m.conditional_files.push(c);
        save_ximod(&m, &dir).unwrap();
        let (back, report) = load_ximod_with_report(&dir).unwrap();
        assert!(report.is_empty(), "{report:?}");
        assert_eq!(back.module_dependencies, m.module_dependencies);
        assert_eq!(back.steps[0].visibility, m.steps[0].visibility);
        assert_eq!(back.conditional_files[0].condition, m.conditional_files[0].condition);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `load_ximod_with_report` lists the dropped constructs of the file
    /// (a game requirement is modelled since lot N: only `<odd>` is lost).
    #[test]
    fn test_load_with_report_flags_lossy_files() {
        let dir = scratch("report");
        let xml = r#"<config><moduleName>R</moduleName>
<moduleDependencies operator="And"><gameDependency version="1.6"/><odd/></moduleDependencies></config>"#;
        std::fs::write(dir.join(INSTALLER_DIR).join("ModuleConfig.xml"), xml).unwrap();
        let (m, report) = load_ximod_with_report(&dir).unwrap();
        assert_eq!(m.name, "R");
        assert_eq!(
            m.module_dependencies.as_ref().unwrap().leaves().next(),
            Some(&Dependency::new_game("1.6"))
        );
        assert_eq!(report.len(), 1);
        assert!(matches!(&report[0], fidelity::Unmodelled::UnknownElement { element, .. } if element == "odd"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_clean_strips_isolation_marks_but_keeps_newlines() {
        assert_eq!(clean_attr("Step \u{2068}1\u{2069}"), "Step 1");
        assert_eq!(clean_attr("a\u{2066}b\u{2067}c\u{200B}"), "abc");
        assert_eq!(clean_text("l1\nl2\tx\u{2068}\u{0007}"), "l1\nl2\tx");
    }

    #[test]
    fn test_save_is_atomic_and_leaves_no_temp_file() {
        let dir = scratch("atomic");
        let ximod = Ximod::new("Atomic");
        save_ximod(&ximod, &dir).unwrap();
        let entries: Vec<String> = std::fs::read_dir(dir.join(INSTALLER_DIR))
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert!(entries.iter().any(|n| n == "ModuleConfig.xml"), "{entries:?}");
        assert!(entries.iter().any(|n| n == "info.xml"), "{entries:?}");
        assert!(!entries.iter().any(|n| n.ends_with(".tmp")), "{entries:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }

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
