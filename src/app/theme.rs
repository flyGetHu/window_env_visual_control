use egui::{Color32, FontId, Rounding, Stroke};

#[derive(Clone)]
pub struct ModernTheme {
    pub primary_color: Color32,
    pub secondary_color: Color32,
    pub accent_color: Color32,
    pub success_color: Color32,
    pub warning_color: Color32,
    pub error_color: Color32,
    pub background_color: Color32,
    pub surface_color: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub border_color: Color32,
    pub card_background: Color32,
    pub card_hover: Color32,
    pub card_selected: Color32,
    pub shadow_color: Color32,
    pub divider_color: Color32,
    pub input_background: Color32,
    pub input_border: Color32,
    pub input_focus: Color32,
    pub button_hover: Color32,
    pub button_active: Color32,
    pub tooltip_background: Color32,
    pub user_variable_accent: Color32,
    pub system_variable_accent: Color32,
}

impl ModernTheme {
    pub fn new() -> Self {
        Self {
            primary_color: Color32::from_rgb(59, 130, 246),
            secondary_color: Color32::from_rgb(99, 102, 241),
            accent_color: Color32::from_rgb(16, 185, 129),
            success_color: Color32::from_rgb(34, 197, 94),
            warning_color: Color32::from_rgb(251, 191, 36),
            error_color: Color32::from_rgb(239, 68, 68),
            background_color: Color32::from_rgb(248, 250, 252),
            surface_color: Color32::WHITE,
            text_primary: Color32::from_rgb(15, 23, 42),
            text_secondary: Color32::from_rgb(100, 116, 139),
            border_color: Color32::from_rgb(226, 232, 240),
            card_background: Color32::from_rgb(255, 255, 255),
            card_hover: Color32::from_rgb(249, 250, 251),
            card_selected: Color32::from_rgb(239, 246, 255),
            shadow_color: Color32::from_rgba_unmultiplied(0, 0, 0, 10),
            divider_color: Color32::from_rgb(241, 245, 249),
            input_background: Color32::from_rgb(255, 255, 255),
            input_border: Color32::from_rgb(209, 213, 219),
            input_focus: Color32::from_rgb(59, 130, 246),
            button_hover: Color32::from_rgb(37, 99, 235),
            button_active: Color32::from_rgb(29, 78, 216),
            tooltip_background: Color32::from_rgb(17, 24, 39),
            user_variable_accent: Color32::from_rgb(34, 197, 94),
            system_variable_accent: Color32::from_rgb(251, 191, 36),
        }
    }

    pub fn dark() -> Self {
        Self {
            primary_color: Color32::from_rgb(96, 165, 250),
            secondary_color: Color32::from_rgb(129, 140, 248),
            accent_color: Color32::from_rgb(52, 211, 153),
            success_color: Color32::from_rgb(74, 222, 128),
            warning_color: Color32::from_rgb(252, 211, 77),
            error_color: Color32::from_rgb(248, 113, 113),
            background_color: Color32::from_rgb(15, 23, 42),
            surface_color: Color32::from_rgb(30, 41, 59),
            text_primary: Color32::from_rgb(248, 250, 252),
            text_secondary: Color32::from_rgb(148, 163, 184),
            border_color: Color32::from_rgb(51, 65, 85),
            card_background: Color32::from_rgb(30, 41, 59),
            card_hover: Color32::from_rgb(51, 65, 85),
            card_selected: Color32::from_rgb(30, 58, 138),
            shadow_color: Color32::from_rgba_unmultiplied(0, 0, 0, 25),
            divider_color: Color32::from_rgb(51, 65, 85),
            input_background: Color32::from_rgb(30, 41, 59),
            input_border: Color32::from_rgb(71, 85, 105),
            input_focus: Color32::from_rgb(96, 165, 250),
            button_hover: Color32::from_rgb(59, 130, 246),
            button_active: Color32::from_rgb(37, 99, 235),
            tooltip_background: Color32::from_rgb(51, 65, 85),
            user_variable_accent: Color32::from_rgb(74, 222, 128),
            system_variable_accent: Color32::from_rgb(252, 211, 77),
        }
    }
}

pub trait ThemeExt {
    fn apply_modern_style(&self, ctx: &egui::Context, is_dark_mode: bool, theme: &ModernTheme);
}

impl ThemeExt for ModernTheme {
    fn apply_modern_style(&self, ctx: &egui::Context, is_dark_mode: bool, theme: &ModernTheme) {
        let mut style = (*ctx.style()).clone();

        style.text_styles.insert(
            egui::TextStyle::Heading,
            FontId::new(24.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            FontId::new(14.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            FontId::new(14.0, egui::FontFamily::Proportional),
        );

        style.spacing.item_spacing = egui::Vec2::new(8.0, 6.0);
        style.spacing.button_padding = egui::Vec2::new(12.0, 8.0);
        style.spacing.menu_margin = egui::Margin::same(8.0);
        style.spacing.indent = 20.0;

        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
        style.visuals.widgets.active.rounding = Rounding::same(8.0);
        style.visuals.widgets.open.rounding = Rounding::same(8.0);

        style.visuals.dark_mode = is_dark_mode;
        style.visuals.override_text_color = Some(theme.text_primary);
        style.visuals.panel_fill = theme.surface_color;
        style.visuals.window_fill = theme.surface_color;

        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, theme.border_color);
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, theme.border_color);

        ctx.set_style(style);
    }
}