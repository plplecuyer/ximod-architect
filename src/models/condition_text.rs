//! Natural-language rendering of a condition group (lot N), shared by the
//! visual condition editor's sentence and any other text output (Nexus
//! description, CLI): `flag A = x AND (flag B = y OR file C is Active)`.
//!
//! Language-neutral: the caller passes the translated templates. Templates
//! use plain `{name}` / `{value}` placeholders (see [`fill`]), not Fluent
//! arguments, so they stay cheap to apply per leaf.

use super::{DependencyGroup, DependencyItem, DependencyType, LogicalOperator};

/// Translated fragments of a sentence.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConditionLabels {
    /// Joiner of an `And` group (`AND`).
    pub and: String,
    /// Joiner of an `Or` group (`OR`).
    pub or: String,
    /// `flag {name} = {value}`.
    pub flag: String,
    /// `file {name} is {value}`.
    pub file: String,
    /// `game version >= {value}`.
    pub game: String,
    /// `mod manager version >= {value}`.
    pub fomm: String,
    /// Shown when the group has no complete leaf (`(no condition)`).
    pub empty: String,
}

impl ConditionLabels {
    /// Plain English, for the CLI and the tests.
    pub fn english() -> Self {
        Self {
            and: "AND".into(),
            or: "OR".into(),
            flag: "flag {name} = {value}".into(),
            file: "file {name} is {value}".into(),
            game: "game version >= {value}".into(),
            fomm: "mod manager version >= {value}".into(),
            empty: "(no condition: always true)".into(),
        }
    }
}

/// Fill a `{placeholder}` template.
pub fn fill(template: &str, pairs: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (k, v) in pairs {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

/// The sentence of `group`: leaves joined by the group's operator, nested
/// groups in parentheses (recursively). Incomplete leaves (no name, or no
/// version) and nested groups without any complete leaf are skipped;
/// `labels.empty` when nothing is left.
pub fn describe(group: &DependencyGroup, labels: &ConditionLabels) -> String {
    match describe_inner(group, labels) {
        Some(s) => s,
        None => labels.empty.clone(),
    }
}

/// `None` when the group has no complete leaf at any depth.
fn describe_inner(group: &DependencyGroup, labels: &ConditionLabels) -> Option<String> {
    let parts: Vec<String> = group
        .items
        .iter()
        .filter_map(|item| match item {
            DependencyItem::Leaf(d) if d.is_complete() => Some(match d.kind() {
                DependencyType::Flag => fill(&labels.flag, &[("name", &d.name), ("value", &d.value)]),
                DependencyType::File => fill(&labels.file, &[("name", &d.name), ("value", &d.value)]),
                DependencyType::Game => fill(&labels.game, &[("value", &d.value)]),
                DependencyType::Fomm => fill(&labels.fomm, &[("value", &d.value)]),
            }),
            DependencyItem::Leaf(_) => None,
            DependencyItem::Group(g) => describe_inner(g, labels).map(|s| format!("({s})")),
        })
        .collect();
    if parts.is_empty() {
        return None;
    }
    let joiner = match group.operator {
        LogicalOperator::And => &labels.and,
        LogicalOperator::Or => &labels.or,
    };
    Some(parts.join(&format!(" {joiner} ")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Dependency;

    #[test]
    fn nested_groups_get_parentheses() {
        let l = ConditionLabels::english();
        let mut inner = DependencyGroup::new(LogicalOperator::Or);
        inner.push_leaf(Dependency::new_flag("A", "x"));
        inner.push_leaf(Dependency::new_flag("A", "y"));
        let mut g = DependencyGroup::new(LogicalOperator::And);
        g.push_group(inner);
        g.push_leaf(Dependency::new_flag("B", "z"));
        assert_eq!(describe(&g, &l), "(flag A = x OR flag A = y) AND flag B = z");

        // Deeper nesting, every leaf kind, and incomplete leaves skipped.
        let mut deep = DependencyGroup::new(LogicalOperator::And);
        deep.push_leaf(Dependency::new_file("Skyrim.esm", "Active"));
        deep.push_leaf(Dependency::new_game("1.6"));
        deep.push_leaf(Dependency::new_fomm(""));
        deep.push_leaf(Dependency::new_flag("", "ignored"));
        let mut mid = DependencyGroup::new(LogicalOperator::Or);
        mid.push_group(deep);
        mid.push_leaf(Dependency::new_fomm("0.13"));
        let mut top = DependencyGroup::new(LogicalOperator::And);
        top.push_leaf(Dependency::new_flag("f", "1"));
        top.push_group(mid);
        top.push_group(DependencyGroup::new(LogicalOperator::Or));
        assert_eq!(
            describe(&top, &l),
            "flag f = 1 AND ((file Skyrim.esm is Active AND game version >= 1.6) OR mod manager version >= 0.13)"
        );
        // Nothing complete at all.
        let mut empty = DependencyGroup::new(LogicalOperator::Or);
        empty.push_leaf(Dependency::new_flag("", ""));
        let mut nested_empty = DependencyGroup::new(LogicalOperator::And);
        nested_empty.push_group(DependencyGroup::default());
        empty.push_group(nested_empty);
        assert_eq!(describe(&empty, &l), l.empty);
        assert_eq!(describe(&DependencyGroup::default(), &l), l.empty);
    }
}
