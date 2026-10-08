//! Visual identity of the application: a semantic palette on top of egui's
//! dark and light visuals, consistent spacing/rounding, and the icon font.
//!
//! Widgets never hard-code colours any more: they ask the [`Palette`] for
//! `success`, `warning`, `danger`, `accent` or `muted`, which are tuned to
//! stay readable on both themes.

use crate::config::Theme;
use eframe::egui::{self, Color32, Rounding, Stroke};

/// Semantic colours of the current theme.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub accent: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub danger: Color32,
    #[allow(dead_code)]
    pub muted: Color32,
    /// Background of a selected row/node (accent, translucent).
    pub selection_bg: Color32,
    /// Background of the update banner and other "information" strips.
    pub info_bg: Color32,
}

impl Palette {
    pub const DARK: Palette = Palette {
        accent: Color32::from_rgb(86, 156, 214),
        success: Color32::from_rgb(110, 200, 120),
        warning: Color32::from_rgb(240, 190, 80),
        danger: Color32::from_rgb(240, 110, 110),
        muted: Color32::from_rgb(150, 150, 150),
        selection_bg: Color32::from_rgba_premultiplied(40, 80, 120, 200),
        info_bg: Color32::from_rgb(40, 70, 105),
    };

    pub const LIGHT: Palette = Palette {
        accent: Color32::from_rgb(30, 100, 190),
        success: Color32::from_rgb(30, 130, 50),
        warning: Color32::from_rgb(170, 110, 0),
        danger: Color32::from_rgb(190, 40, 40),
        muted: Color32::from_rgb(110, 110, 110),
        selection_bg: Color32::from_rgba_premultiplied(190, 215, 240, 255),
        info_bg: Color32::from_rgb(214, 230, 248),
    };

    /// Palette matching the visuals in use.
    pub fn of(visuals: &egui::Visuals) -> Palette {
        if visuals.dark_mode {
            Palette::DARK
        } else {
            Palette::LIGHT
        }
    }

    /// Convenience for widget code.
    pub fn from_ui(ui: &egui::Ui) -> Palette {
        Palette::of(ui.visuals())
    }
}

/// Result of the last out-of-process probe of the OS theme (`reg`,
/// `gsettings`, `defaults`…), with its time. The probe spawns a process, so
/// it is only used when the windowing layer does not report the OS theme,
/// and at most once every [`PROBE_INTERVAL`].
static PROBE: std::sync::Mutex<Option<(std::time::Instant, Theme)>> = std::sync::Mutex::new(None);
const PROBE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(10);

/// The OS theme as a concrete dark/light choice: what the windowing layer
/// reports (kept up to date by eframe on every OS theme change), or, when it
/// reports nothing, the cached command-line probe.
pub fn system_theme(ctx: &egui::Context) -> Theme {
    if let Some(t) = ctx.system_theme() {
        return match t {
            egui::Theme::Light => Theme::Light,
            egui::Theme::Dark => Theme::Dark,
        };
    }
    let mut probe = PROBE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((at, t)) = *probe
        && at.elapsed() < PROBE_INTERVAL
    {
        return t;
    }
    let t = match Theme::detect_system_theme() {
        Theme::Light => Theme::Light,
        _ => Theme::Dark,
    };
    *probe = Some((std::time::Instant::now(), t));
    t
}

/// Resolve a theme setting to the concrete dark/light theme in use.
pub fn concrete(ctx: &egui::Context, theme: Theme) -> Theme {
    match theme {
        Theme::System => system_theme(ctx),
        other => other,
    }
}

/// The application's visuals for one concrete theme.
fn visuals_for(dark: bool) -> egui::Visuals {
    let palette = if dark { Palette::DARK } else { Palette::LIGHT };
    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    // Accent: selection, links, focus ring.
    visuals.selection.bg_fill = palette.selection_bg;
    visuals.selection.stroke = Stroke::new(1.0_f32, palette.accent);
    visuals.hyperlink_color = palette.accent;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, palette.accent);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, palette.accent.gamma_multiply(0.7));
    // Semantic text colours used by egui itself (error labels, warn_fg_color).
    visuals.error_fg_color = palette.danger;
    visuals.warn_fg_color = palette.warning;

    // Softer corners everywhere, a little more generous windows.
    let r = Rounding::same(6.0);
    visuals.widgets.noninteractive.rounding = r;
    visuals.widgets.inactive.rounding = r;
    visuals.widgets.hovered.rounding = r;
    visuals.widgets.active.rounding = r;
    visuals.widgets.open.rounding = r;
    visuals.window_rounding = Rounding::same(8.0);
    visuals.menu_rounding = Rounding::same(6.0);
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur: 12.0,
        spread: 0.0,
        color: Color32::from_black_alpha(if dark { 110 } else { 50 }),
    };
    visuals.striped = true;
    visuals
}

/// Install the visuals and spacing for `theme`.
///
/// egui keeps one style per concrete theme and picks the active one from its
/// theme preference. Both slots receive the application's visuals, and the
/// preference is set to match the setting: a fixed dark/light choice, or
/// "follow the system" — in which case eframe switches the active slot by
/// itself whenever the OS theme changes, with no repaint of ours needed.
/// When the windowing layer cannot report the OS theme, "System" is pinned
/// to the probed value and re-checked by the caller (see `update`).
pub fn apply(ctx: &egui::Context, theme: Theme) {
    ctx.set_visuals_of(egui::Theme::Dark, visuals_for(true));
    ctx.set_visuals_of(egui::Theme::Light, visuals_for(false));

    let preference = match theme {
        Theme::System if ctx.system_theme().is_some() => egui::ThemePreference::System,
        other => match concrete(ctx, other) {
            Theme::Light => egui::ThemePreference::Light,
            _ => egui::ThemePreference::Dark,
        },
    };
    ctx.set_theme(preference);

    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style.spacing.interact_size.y = 26.0;
        style.spacing.window_margin = egui::Margin::same(10.0);
        style.spacing.menu_margin = egui::Margin::same(8.0);
        style.spacing.indent = 18.0;
    });
}

/// Add the Phosphor icon font to a font definition set. Icons are glyphs in
/// the Private Use Area, appended to the Proportional family, so they can be
/// mixed with text in any label or button.
pub fn add_icons(fonts: &mut egui::FontDefinitions) {
    egui_phosphor::add_to_fonts(fonts, egui_phosphor::Variant::Regular);
}

/// The icons used across the interface, named by meaning so a swap of icon
/// set touches one place.
#[allow(unused_imports)]
pub mod icon {
    pub use egui_phosphor::regular::{
        ARROW_CLOCKWISE as REDO, ARROW_COUNTER_CLOCKWISE as UNDO, ARROW_DOWN as DOWN, ARROW_LEFT as LEFT,
        ARROW_RIGHT as RIGHT, ARROW_UP as UP, BOOK_OPEN as MANUAL, CARET_DOWN as EXPANDED, CARET_RIGHT as COLLAPSED,
        CHECK_CIRCLE as VALIDATE, CHECK_SQUARE as OPTION, COPY as DUPLICATE, EYE as PREVIEW, FILE, FILE_PLUS as NEW,
        FLAG, FLOPPY_DISK as SAVE, FOLDER, FOLDER_OPEN as OPEN, GEAR as SETTINGS, IMAGE, INFO, LIST_CHECKS as REQUIRED,
        MAGNIFYING_GLASS as SEARCH, PACKAGE as EXPORT, PLUS as ADD, QUESTION as HELP, SQUARES_FOUR as GROUP,
        STACK as STEP, TRANSLATE, TRASH as DELETE, TREE_STRUCTURE as CONDITIONAL, WARNING, X as CLOSE,
        X_CIRCLE as ERROR,
    };
}

/// `"<icon> <text>"`, the usual button/label composition.
pub fn with_icon(icon: &str, text: &str) -> String {
    format!("{icon} {text}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palettes_follow_visuals() {
        assert_eq!(Palette::of(&egui::Visuals::dark()).accent, Palette::DARK.accent);
        assert_eq!(Palette::of(&egui::Visuals::light()).accent, Palette::LIGHT.accent);
    }

    #[test]
    fn apply_sets_accent_and_spacing() {
        let ctx = egui::Context::default();
        apply(&ctx, Theme::Light);
        assert!(!ctx.style().visuals.dark_mode);
        assert_eq!(ctx.style().visuals.hyperlink_color, Palette::LIGHT.accent);
        assert_eq!(ctx.style().spacing.interact_size.y, 26.0);
        apply(&ctx, Theme::Dark);
        assert!(ctx.style().visuals.dark_mode);
    }

    #[test]
    fn both_theme_slots_carry_the_app_visuals() {
        let ctx = egui::Context::default();
        apply(&ctx, Theme::Dark);
        assert_eq!(ctx.theme(), egui::Theme::Dark);
        let light = ctx.style_of(egui::Theme::Light);
        let dark = ctx.style_of(egui::Theme::Dark);
        // A system switch to the other slot keeps the accent and spacing.
        assert_eq!(light.visuals.hyperlink_color, Palette::LIGHT.accent);
        assert_eq!(dark.visuals.hyperlink_color, Palette::DARK.accent);
        assert_eq!(light.spacing.interact_size.y, 26.0);
        assert_eq!(dark.spacing.interact_size.y, 26.0);
    }

    #[test]
    fn system_theme_follows_the_windowing_layer_when_reported() {
        let ctx = egui::Context::default();
        let input = |t: egui::Theme| egui::RawInput {
            system_theme: Some(t),
            ..Default::default()
        };
        // The OS theme reaches egui through the raw input of each frame.
        let _ = ctx.run(input(egui::Theme::Light), |ctx| {
            assert_eq!(system_theme(ctx), Theme::Light);
            assert_eq!(concrete(ctx, Theme::System), Theme::Light);
            assert_eq!(concrete(ctx, Theme::Dark), Theme::Dark);
            apply(ctx, Theme::System);
            assert_eq!(ctx.options(|o| o.theme_preference), egui::ThemePreference::System);
            assert!(!ctx.style().visuals.dark_mode);
        });
        // The OS switches: egui switches the active slot by itself, and the
        // application's visuals are already there.
        let _ = ctx.run(input(egui::Theme::Dark), |ctx| {
            assert!(ctx.style().visuals.dark_mode);
            assert_eq!(ctx.style().visuals.hyperlink_color, Palette::DARK.accent);
        });
    }

    #[test]
    fn system_theme_is_pinned_when_the_os_theme_is_unknown() {
        let ctx = egui::Context::default();
        assert!(ctx.system_theme().is_none());
        let probed = system_theme(&ctx);
        apply(&ctx, Theme::System);
        assert_ne!(ctx.options(|o| o.theme_preference), egui::ThemePreference::System);
        assert_eq!(ctx.style().visuals.dark_mode, probed != Theme::Light);
    }

    #[test]
    fn icons_are_private_use_glyphs() {
        for g in [icon::SAVE, icon::OPEN, icon::DELETE, icon::STEP] {
            let c = g.chars().next().unwrap();
            assert!(('\u{E000}'..='\u{F8FF}').contains(&c), "{c:?}");
        }
    }
}
