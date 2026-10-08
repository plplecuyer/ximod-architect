//! Human-readable, translated labels for the model's enumerations.
//!
//! The model keeps the FOMOD schema's identifiers (`SelectExactlyOne`,
//! `CouldBeUsable`, `And`…) because that is what gets written to XML. Showing
//! those raw identifiers in combo boxes was opaque for the modders the tool
//! is made for, so the UI maps each variant to a short translated label plus
//! a one-line explanation used as a tooltip.

use crate::config::Theme;
use crate::i18n::I18n;
use crate::models::{LogicalOperator, PluginType, SelectionType};

/// Short label of a group selection type ("Exactly one (required)"…).
pub fn selection_type(i18n: &I18n, st: SelectionType) -> String {
    i18n.t(match st {
        SelectionType::SelectExactlyOne => "seltype-exactly-one",
        SelectionType::SelectAtMostOne => "seltype-at-most-one",
        SelectionType::SelectAny => "seltype-any",
        SelectionType::SelectAll => "seltype-all",
        SelectionType::SelectAtLeastOne => "seltype-at-least-one",
    })
}

/// One-line explanation of a selection type, as the installer shows it.
pub fn selection_type_hint(i18n: &I18n, st: SelectionType) -> String {
    i18n.t(match st {
        SelectionType::SelectExactlyOne => "preview-sel-exactlyone",
        SelectionType::SelectAtMostOne => "preview-sel-atmostone",
        SelectionType::SelectAny => "preview-sel-any",
        SelectionType::SelectAll => "preview-sel-all",
        SelectionType::SelectAtLeastOne => "preview-sel-atleastone",
    })
}

/// Short label of an option type ("Required", "Recommended"…).
pub fn plugin_type(i18n: &I18n, pt: PluginType) -> String {
    i18n.t(plugin_type_key(pt))
}

/// Same, from the schema identifier stored in a dependency pattern.
pub fn plugin_type_name(i18n: &I18n, name: &str) -> String {
    match PluginType::variants().iter().find(|pt| pt.as_str() == name) {
        Some(pt) => plugin_type(i18n, *pt),
        None => name.to_string(),
    }
}

/// One-line explanation of what the installer does with an option type.
pub fn plugin_type_hint(i18n: &I18n, pt: PluginType) -> String {
    i18n.t(match pt {
        PluginType::Required => "plugtype-required-hint",
        PluginType::Optional => "plugtype-optional-hint",
        PluginType::Recommended => "plugtype-recommended-hint",
        PluginType::NotUsable => "plugtype-not-usable-hint",
        PluginType::CouldBeUsable => "plugtype-could-be-usable-hint",
    })
}

fn plugin_type_key(pt: PluginType) -> &'static str {
    match pt {
        PluginType::Required => "plugtype-required",
        PluginType::Optional => "plugtype-optional",
        PluginType::Recommended => "plugtype-recommended",
        PluginType::NotUsable => "plugtype-not-usable",
        PluginType::CouldBeUsable => "plugtype-could-be-usable",
    }
}

/// "All conditions" / "Any condition" instead of And / Or.
pub fn operator(i18n: &I18n, op: LogicalOperator) -> String {
    i18n.t(match op {
        LogicalOperator::And => "op-and",
        LogicalOperator::Or => "op-or",
    })
}

/// Theme name for the settings combo box.
pub fn theme(i18n: &I18n, theme: Theme) -> String {
    i18n.t(match theme {
        Theme::Dark => "theme-dark",
        Theme::Light => "theme-light",
        Theme::System => "theme-system",
    })
}

/// "Step 3" for a step without a name (1-based index).
pub fn step_fallback(i18n: &I18n, index: usize) -> String {
    i18n.t_num("default-step-name", index as i64 + 1)
}
