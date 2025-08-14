use egui::{Rounding, Stroke, Vec2};
use crate::app::theme::ModernTheme;

pub struct ModernCard<'a> {
    theme: &'a ModernTheme,
    inner_margin: f32,
}

impl<'a> ModernCard<'a> {
    pub fn new(theme: &'a ModernTheme) -> Self {
        Self {
            theme,
            inner_margin: 16.0,
        }
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.inner_margin = margin;
        self
    }

    pub fn show(
        &self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> egui::InnerResponse<()> {
        egui::Frame::none()
            .fill(self.theme.card_background)
            .rounding(Rounding::same(12.0))
            .stroke(Stroke::new(1.0, self.theme.border_color))
            .inner_margin(egui::Margin::same(self.inner_margin))
            .shadow(egui::epaint::Shadow {
                offset: Vec2::new(0.0, 2.0),
                blur: 8.0,
                spread: 0.0,
                color: self.theme.shadow_color,
            })
            .show(ui, add_contents)
    }
}

pub struct InteractiveCard<'a> {
    theme: &'a ModernTheme,
    is_hovered: bool,
    is_selected: bool,
    inner_margin: f32,
}

impl<'a> InteractiveCard<'a> {
    pub fn new(theme: &'a ModernTheme, is_hovered: bool, is_selected: bool) -> Self {
        Self {
            theme,
            is_hovered,
            is_selected,
            inner_margin: 12.0,
        }
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.inner_margin = margin;
        self
    }

    pub fn show(
        &self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> egui::InnerResponse<()> {
        let fill_color = if self.is_selected {
            self.theme.card_selected
        } else if self.is_hovered {
            self.theme.card_hover
        } else {
            self.theme.card_background
        };

        let shadow_blur = if self.is_hovered { 12.0 } else { 6.0 };
        let shadow_offset = if self.is_hovered {
            Vec2::new(0.0, 4.0)
        } else {
            Vec2::new(0.0, 2.0)
        };

        egui::Frame::none()
            .fill(fill_color)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(
                1.0,
                if self.is_selected {
                    self.theme.primary_color
                } else {
                    self.theme.border_color
                },
            ))
            .inner_margin(egui::Margin::same(self.inner_margin))
            .shadow(egui::epaint::Shadow {
                offset: shadow_offset,
                blur: shadow_blur,
                spread: 0.0,
                color: self.theme.shadow_color,
            })
            .show(ui, add_contents)
    }
}

pub struct SurfaceCard<'a> {
    theme: &'a ModernTheme,
    inner_margin: f32,
}

impl<'a> SurfaceCard<'a> {
    pub fn new(theme: &'a ModernTheme) -> Self {
        Self {
            theme,
            inner_margin: 12.0,
        }
    }

    pub fn show(
        &self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> egui::InnerResponse<()> {
        egui::Frame::none()
            .fill(self.theme.surface_color)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(1.0, self.theme.border_color))
            .inner_margin(egui::Margin::same(self.inner_margin))
            .show(ui, add_contents)
    }
}