//! A colour picker modeled on AutoCAD's "Select a colour" dialog, with two
//! tabs only: **Index colour** (the ACI palette grid) and **True colours** (a
//! hue/saturation area + luminance slider + HSL/RGB fields). Ported from
//! XIMOD Translator so both applications pick colours the same way.

use eframe::egui::{self, Color32, Rect, Rounding, Sense, Stroke, pos2, vec2};

use crate::color;
use crate::i18n::I18n;
use crate::tree_colors::TreeKind;

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Index,
    True,
}

/// What the picker is currently editing. The commit result carries this back so
/// the host knows where to store the chosen colour.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PickerTarget {
    /// The colour of one kind of row of the project tree, for one theme
    /// (`dark` tells which set to write).
    TreeColor { kind: TreeKind, dark: bool },
}

/// Stateful colour picker. Open it with [`ColorPicker::open_for`]; call
/// [`ColorPicker::ui`] every frame — it returns `Some((target, rgb))` once,
/// on the frame OK is pressed.
pub struct ColorPicker {
    open: bool,
    /// True for one frame after opening, so the host viewport centers itself.
    pub needs_center: bool,
    target: Option<PickerTarget>,
    tab: Tab,
    r: u8,
    g: u8,
    b: u8,
    index: u8,
    h: f64,
    s: f64,
    l: f64,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            open: false,
            needs_center: false,
            target: None,
            tab: Tab::Index,
            r: 255,
            g: 255,
            b: 255,
            index: 7,
            h: 0.0,
            s: 0.0,
            l: 100.0,
        }
    }
}

impl ColorPicker {
    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    /// Open the picker to edit `target`'s colour, seeded with `rgb`.
    pub fn open_for(&mut self, target: PickerTarget, rgb: (u8, u8, u8)) {
        self.open = true;
        self.needs_center = true;
        self.target = Some(target);
        self.tab = Tab::Index;
        self.set_rgb(rgb);
        self.index = color::nearest_aci(rgb);
    }

    fn set_rgb(&mut self, rgb: (u8, u8, u8)) {
        self.r = rgb.0;
        self.g = rgb.1;
        self.b = rgb.2;
        let (h, s, l) = color::rgb_to_hsl(rgb);
        self.h = h;
        self.s = s;
        self.l = l;
    }

    fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    fn recompute_from_hsl(&mut self) {
        let rgb = color::hsl_to_rgb(self.h, self.s, self.l);
        self.r = rgb.0;
        self.g = rgb.1;
        self.b = rgb.2;
        self.index = color::nearest_aci(rgb);
    }

    /// Render the picker's contents into `ui` (the host viewport owns the
    /// window). Returns the chosen `(target, rgb)` once, on OK.
    pub fn ui(&mut self, ui: &mut egui::Ui, i18n: &I18n) -> Option<(PickerTarget, (u8, u8, u8))> {
        let mut committed = None;
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, Tab::Index, i18n.t("picker-index-tab"));
            ui.selectable_value(&mut self.tab, Tab::True, i18n.t("picker-true-tab"));
        });
        ui.separator();
        match self.tab {
            Tab::Index => self.index_tab(ui, i18n),
            Tab::True => self.true_tab(ui, i18n),
        }
        ui.separator();
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(vec2(64.0, 22.0), Sense::hover());
            ui.painter().rect_filled(rect, Rounding::same(3.0), self.color32());
            ui.painter()
                .rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0_f32, Color32::GRAY));
            ui.label(format!("{},{},{}", self.r, self.g, self.b));
            ui.add_space(12.0);
            if ui.button(i18n.t("btn-ok")).clicked() {
                if let Some(t) = self.target {
                    committed = Some((t, self.rgb()));
                }
                self.open = false;
            }
            if ui.button(i18n.t("btn-cancel")).clicked() {
                self.open = false;
            }
        });
        committed
    }

    fn color32(&self) -> Color32 {
        Color32::from_rgb(self.r, self.g, self.b)
    }

    // ---- Tab 1: ACI index palette -----------------------------------------

    fn index_tab(&mut self, ui: &mut egui::Ui, i18n: &I18n) {
        ui.label(i18n.t("picker-aci-label"));
        let cols = color::ACI_HUES;
        let rows = color::ACI_SHADES;
        let cell = vec2(20.0, 16.0);
        let (resp, painter) = ui.allocate_painter(vec2(cols as f32 * cell.x, rows as f32 * cell.y), Sense::click());
        let origin = resp.rect.min;
        for c in 0..cols {
            for r in 0..rows {
                let idx = 10 + (c * rows + r) as u8;
                let rgb = color::aci_rgb(idx);
                let rect = Rect::from_min_size(origin + vec2(c as f32 * cell.x, r as f32 * cell.y), cell);
                painter.rect_filled(rect, Rounding::ZERO, Color32::from_rgb(rgb.0, rgb.1, rgb.2));
                if self.index == idx {
                    painter.rect_stroke(rect, Rounding::ZERO, Stroke::new(2.0_f32, Color32::BLACK));
                }
            }
        }
        if resp.clicked()
            && let Some(p) = resp.interact_pointer_pos()
        {
            let rel = p - origin;
            let c = (rel.x / cell.x) as usize;
            let r = (rel.y / cell.y) as usize;
            if c < cols && r < rows {
                self.pick_index(10 + (c * rows + r) as u8);
            }
        }

        ui.add_space(6.0);
        // Standard colors 1–9 and the grayscale ramp 250–255.
        self.swatch_row(ui, 1..=9);
        self.swatch_row(ui, 250..=255);

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(i18n.t("picker-color-field"));
            let mut v = self.index as i32;
            if ui.add(egui::DragValue::new(&mut v).range(1..=255).speed(1.0)).changed() {
                self.pick_index(v.clamp(1, 255) as u8);
            }
        });
    }

    fn pick_index(&mut self, idx: u8) {
        self.set_rgb(color::aci_rgb(idx));
        self.index = idx;
    }

    fn swatch_row(&mut self, ui: &mut egui::Ui, range: std::ops::RangeInclusive<u8>) {
        let cell = vec2(20.0, 16.0);
        let count = (*range.end() - *range.start() + 1) as usize;
        let (resp, painter) = ui.allocate_painter(vec2(count as f32 * cell.x, cell.y), Sense::click());
        let origin = resp.rect.min;
        for (i, idx) in range.clone().enumerate() {
            let rgb = color::aci_rgb(idx);
            let rect = Rect::from_min_size(origin + vec2(i as f32 * cell.x, 0.0), cell);
            painter.rect_filled(rect, Rounding::ZERO, Color32::from_rgb(rgb.0, rgb.1, rgb.2));
            painter.rect_stroke(rect, Rounding::ZERO, Stroke::new(0.5_f32, Color32::GRAY));
            if self.index == idx {
                painter.rect_stroke(rect, Rounding::ZERO, Stroke::new(2.0_f32, Color32::BLACK));
            }
        }
        if resp.clicked()
            && let Some(p) = resp.interact_pointer_pos()
        {
            let i = ((p.x - origin.x) / cell.x) as usize;
            if i < count {
                self.pick_index(*range.start() + i as u8);
            }
        }
    }

    // ---- Tab 2: true colors (HSL area + luminance + fields) ----------------

    fn true_tab(&mut self, ui: &mut egui::Ui, i18n: &I18n) {
        ui.horizontal(|ui| {
            self.hsl_field(ui, i18n.t("picker-hue"), 0.0..=360.0, Component::H);
            self.hsl_field(ui, i18n.t("picker-sat"), 0.0..=100.0, Component::S);
            self.hsl_field(ui, i18n.t("picker-lum"), 0.0..=100.0, Component::L);
        });
        ui.add_space(4.0);
        ui.horizontal_top(|ui| {
            self.hue_sat_area(ui);
            self.luminance_slider(ui);
            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.label(i18n.t("picker-stored-rgb"));
                ui.add_space(6.0);
                // Two aligned columns: right-aligned label (so the colons line up)
                // and the value (all values start at the same x).
                egui::Grid::new("picker_rgb_values")
                    .num_columns(2)
                    .spacing([8.0, 2.0])
                    .show(ui, |ui| {
                        for (name, val) in [
                            (i18n.t("picker-red"), self.r),
                            (i18n.t("picker-green"), self.g),
                            (i18n.t("picker-blue"), self.b),
                        ] {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("{name} :"));
                            });
                            ui.label(val.to_string());
                            ui.end_row();
                        }
                    });
            });
        });
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(i18n.t("picker-rgb-input"));
            let mut text = format!("{},{},{}", self.r, self.g, self.b);
            if ui
                .add(egui::TextEdit::singleline(&mut text).desired_width(120.0))
                .changed()
                && let Some(rgb) = parse_rgb(&text)
            {
                self.set_rgb(rgb);
                self.index = color::nearest_aci(rgb);
            }
        });
    }

    fn hsl_field(&mut self, ui: &mut egui::Ui, label: String, range: std::ops::RangeInclusive<f64>, comp: Component) {
        ui.vertical(|ui| {
            ui.label(label);
            let mut v = match comp {
                Component::H => self.h,
                Component::S => self.s,
                Component::L => self.l,
            };
            if ui
                .add(egui::DragValue::new(&mut v).range(range).speed(1.0).max_decimals(0))
                .changed()
            {
                match comp {
                    Component::H => self.h = v,
                    Component::S => self.s = v,
                    Component::L => self.l = v,
                }
                self.recompute_from_hsl();
            }
        });
    }

    fn hue_sat_area(&mut self, ui: &mut egui::Ui) {
        let size = vec2(240.0, 170.0);
        let (resp, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        let origin = resp.rect.min;
        let (cols, rows) = (48usize, 34usize);
        let cw = size.x / cols as f32;
        let ch = size.y / rows as f32;
        for i in 0..cols {
            for j in 0..rows {
                let h = i as f64 / (cols - 1) as f64 * 360.0;
                let s = 100.0 - j as f64 / (rows - 1) as f64 * 100.0;
                let rgb = color::hsl_to_rgb(h, s, self.l);
                let rect = Rect::from_min_size(origin + vec2(i as f32 * cw, j as f32 * ch), vec2(cw + 1.0, ch + 1.0));
                painter.rect_filled(rect, Rounding::ZERO, Color32::from_rgb(rgb.0, rgb.1, rgb.2));
            }
        }
        // Cursor at current (h, s).
        let cx = origin.x + (self.h / 360.0) as f32 * size.x;
        let cy = origin.y + (1.0 - self.s / 100.0) as f32 * size.y;
        painter.circle_stroke(pos2(cx, cy), 5.0, Stroke::new(2.0_f32, Color32::BLACK));
        painter.circle_stroke(pos2(cx, cy), 6.0, Stroke::new(1.0_f32, Color32::WHITE));
        if (resp.dragged() || resp.clicked())
            && let Some(p) = resp.interact_pointer_pos()
        {
            let rel = p - origin;
            self.h = ((rel.x / size.x).clamp(0.0, 1.0) * 360.0) as f64;
            self.s = ((1.0 - rel.y / size.y).clamp(0.0, 1.0) * 100.0) as f64;
            self.recompute_from_hsl();
        }
    }

    fn luminance_slider(&mut self, ui: &mut egui::Ui) {
        let size = vec2(22.0, 170.0);
        let (resp, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        let origin = resp.rect.min;
        let steps = 34usize;
        let sh = size.y / steps as f32;
        for j in 0..steps {
            let l = 100.0 - j as f64 / (steps - 1) as f64 * 100.0;
            let rgb = color::hsl_to_rgb(self.h, self.s, l);
            let rect = Rect::from_min_size(origin + vec2(0.0, j as f32 * sh), vec2(size.x, sh + 1.0));
            painter.rect_filled(rect, Rounding::ZERO, Color32::from_rgb(rgb.0, rgb.1, rgb.2));
        }
        let y = origin.y + (1.0 - self.l / 100.0) as f32 * size.y;
        painter.rect_stroke(
            Rect::from_min_size(pos2(origin.x - 2.0, y - 2.0), vec2(size.x + 4.0, 4.0)),
            Rounding::ZERO,
            Stroke::new(1.5_f32, Color32::BLACK),
        );
        if (resp.dragged() || resp.clicked())
            && let Some(p) = resp.interact_pointer_pos()
        {
            self.l = ((1.0 - (p.y - origin.y) / size.y).clamp(0.0, 1.0) * 100.0) as f64;
            self.recompute_from_hsl();
        }
    }
}

#[derive(Clone, Copy)]
enum Component {
    H,
    S,
    L,
}

fn parse_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let p: Vec<&str> = s.split(',').map(|x| x.trim()).collect();
    if p.len() != 3 {
        return None;
    }
    Some((p[0].parse().ok()?, p[1].parse().ok()?, p[2].parse().ok()?))
}
