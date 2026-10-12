//! Colours of the project tree, one per kind of node, so that steps, groups,
//! options and the two file sections can be told apart at a glance. Two sets
//! exist, one per theme, because a colour legible on a dark background is not
//! legible on a light one. Both are stored in `Config.ini` and edited from
//! Tools → Tree colours…

/// A kind of row of the project tree that carries its own colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreeKind {
    /// The "Mod information" row.
    Info,
    /// The steps header and every step.
    Step,
    /// Every group.
    Group,
    /// Every option (with or without an image).
    Option,
    /// The "Required files" row.
    Required,
    /// The conditional installs header and every conditional set.
    Conditional,
}

impl TreeKind {
    /// Drawing order of the tree, which is also the order of the editor.
    pub const ALL: [TreeKind; 6] = [
        TreeKind::Info,
        TreeKind::Step,
        TreeKind::Group,
        TreeKind::Option,
        TreeKind::Required,
        TreeKind::Conditional,
    ];

    /// Stable identifier used in `Config.ini` keys (`TreeDark_<key>`).
    pub fn key(self) -> &'static str {
        match self {
            TreeKind::Info => "Info",
            TreeKind::Step => "Step",
            TreeKind::Group => "Group",
            TreeKind::Option => "Option",
            TreeKind::Required => "Required",
            TreeKind::Conditional => "Conditional",
        }
    }

    /// i18n key of the row label shown in the colours editor.
    pub fn label_key(self) -> &'static str {
        match self {
            TreeKind::Info => "tree-kind-info",
            TreeKind::Step => "tree-kind-step",
            TreeKind::Group => "tree-kind-group",
            TreeKind::Option => "tree-kind-option",
            TreeKind::Required => "tree-kind-required",
            TreeKind::Conditional => "tree-kind-conditional",
        }
    }

    fn index(self) -> usize {
        match self {
            TreeKind::Info => 0,
            TreeKind::Step => 1,
            TreeKind::Group => 2,
            TreeKind::Option => 3,
            TreeKind::Required => 4,
            TreeKind::Conditional => 5,
        }
    }

    /// The kind whose `key()` is `key`, if any (used when reading the INI).
    pub fn from_key(key: &str) -> Option<TreeKind> {
        TreeKind::ALL.iter().copied().find(|k| k.key() == key)
    }
}

/// One colour per [`TreeKind`], as `(r, g, b)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeColors {
    colors: [(u8, u8, u8); 6],
}

impl TreeColors {
    /// Defaults for the dark theme: a syntax-highlighting-like palette that
    /// stays readable on the dark panels.
    pub const DARK: TreeColors = TreeColors {
        colors: [
            (200, 200, 200), // Info — neutral light grey
            (86, 156, 214),  // Step — blue (the accent)
            (197, 134, 192), // Group — mauve
            (181, 206, 168), // Option — soft green
            (220, 170, 100), // Required — amber
            (229, 128, 128), // Conditional — coral
        ],
    };

    /// Defaults for the light theme: the same hues, darkened for white panels.
    pub const LIGHT: TreeColors = TreeColors {
        colors: [
            (70, 70, 70),   // Info
            (30, 100, 190), // Step
            (128, 56, 150), // Group
            (46, 125, 50),  // Option
            (178, 106, 0),  // Required
            (198, 40, 40),  // Conditional
        ],
    };

    /// The defaults of a theme.
    pub fn defaults(dark: bool) -> TreeColors {
        if dark { TreeColors::DARK } else { TreeColors::LIGHT }
    }

    /// The colour of a kind.
    pub fn get(&self, kind: TreeKind) -> (u8, u8, u8) {
        self.colors[kind.index()]
    }

    /// Change the colour of a kind.
    pub fn set(&mut self, kind: TreeKind, rgb: (u8, u8, u8)) {
        self.colors[kind.index()] = rgb;
    }

    /// The colour of a kind as an egui colour.
    pub fn color32(&self, kind: TreeKind) -> eframe::egui::Color32 {
        let (r, g, b) = self.get(kind);
        eframe::egui::Color32::from_rgb(r, g, b)
    }
}

/// Parse an `r,g,b` triple as written in `Config.ini`.
pub fn parse_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let mut it = s.split(',').map(|x| x.trim().parse::<u8>());
    let r = it.next()?.ok()?;
    let g = it.next()?.ok()?;
    let b = it.next()?.ok()?;
    if it.next().is_some() {
        return None;
    }
    Some((r, g, b))
}

/// Format an `(r, g, b)` triple for `Config.ini`.
pub fn format_rgb((r, g, b): (u8, u8, u8)) -> String {
    format!("{r},{g},{b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_has_a_distinct_default_in_both_themes() {
        for set in [TreeColors::DARK, TreeColors::LIGHT] {
            let mut seen = std::collections::HashSet::new();
            for k in TreeKind::ALL {
                assert!(seen.insert(set.get(k)), "duplicate default colour for {k:?}");
            }
        }
    }

    #[test]
    fn keys_round_trip() {
        for k in TreeKind::ALL {
            assert_eq!(TreeKind::from_key(k.key()), Some(k));
        }
        assert_eq!(TreeKind::from_key("Nope"), None);
    }

    #[test]
    fn rgb_text_round_trips_and_rejects_junk() {
        assert_eq!(parse_rgb("12, 200,255"), Some((12, 200, 255)));
        assert_eq!(parse_rgb(&format_rgb((1, 2, 3))), Some((1, 2, 3)));
        assert_eq!(parse_rgb("1,2"), None);
        assert_eq!(parse_rgb("1,2,3,4"), None);
        assert_eq!(parse_rgb("1,2,300"), None);
        assert_eq!(parse_rgb("red"), None);
    }

    #[test]
    fn set_changes_only_that_kind() {
        let mut c = TreeColors::DARK;
        c.set(TreeKind::Group, (1, 2, 3));
        assert_eq!(c.get(TreeKind::Group), (1, 2, 3));
        assert_eq!(c.get(TreeKind::Step), TreeColors::DARK.get(TreeKind::Step));
    }
}
