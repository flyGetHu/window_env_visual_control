use egui::{Color32, Rounding, Stroke, Widget};
use crate::app::theme::ModernTheme;

pub struct ModernButton<'a> {
    text: &'a str,
    color: Color32,
    width: f32,
    height: f32,
}

impl<'a> ModernButton<'a> {
    pub fn new(text: &'a str, color: Color32) -> Self {
        Self {
            text,
            color,
            width: 120.0,
            height: 36.0,
        }
    }

    pub fn small(text: &'a str, color: Color32) -> Self {
        Self {
            text,
            color,
            width: 80.0,
            height: 28.0,
        }
    }

    pub fn sized(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl<'a> Widget for ModernButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = egui::Button::new(self.text)
            .fill(self.color)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(0.0, Color32::TRANSPARENT));
        ui.add_sized([self.width, self.height], button)
    }
}

pub struct EnhancedButton<'a> {
    text: &'a str,
    color: Color32,
    icon: Option<&'a str>,
}

impl<'a> EnhancedButton<'a> {
    pub fn new(text: &'a str, color: Color32) -> Self {
        Self {
            text,
            color,
            icon: None,
        }
    }

    pub fn with_icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl<'a> Widget for EnhancedButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button_height = 36.0;

        ui.allocate_ui_with_layout(
            egui::Vec2::new(ui.available_width(), button_height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                let response = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), button_height),
                    egui::Sense::click(),
                );
                let rect = response.rect;

                let _theme = &ui.style().visuals.clone();
                let fill_color = if response.clicked() {
                    Color32::from_rgb(29, 78, 216)
                } else if response.hovered() {
                    Color32::from_rgb(37, 99, 235)
                } else {
                    self.color
                };

                ui.painter()
                    .rect_filled(rect, Rounding::same(8.0), fill_color);

                let text_color = Color32::WHITE;

                if let Some(icon_text) = self.icon {
                    let icon_pos = egui::Pos2::new(rect.left() + 12.0, rect.center().y);
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        icon_text,
                        egui::FontId::proportional(16.0),
                        text_color,
                    );

                    let text_pos = egui::Pos2::new(rect.left() + 32.0, rect.center().y);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_CENTER,
                        self.text,
                        egui::FontId::proportional(14.0),
                        text_color,
                    );
                } else {
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        self.text,
                        egui::FontId::proportional(14.0),
                        text_color,
                    );
                }

                response
            },
        )
        .inner
    }
}

pub fn show_tooltip(ui: &mut egui::Ui, text: &str, response: &egui::Response, theme: &ModernTheme) {
    if response.hovered() {
        egui::show_tooltip_at_pointer(
            ui.ctx(),
            egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("tooltip")),
            egui::Id::new("tooltip"),
            |ui: &mut egui::Ui| {
                egui::Frame::popup(ui.style())
                    .fill(theme.tooltip_background)
                    .stroke(egui::Stroke::new(1.0, theme.border_color))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(8.0))
                    .show(ui, |ui| {
                        ui.colored_label(Color32::WHITE, text);
                    });
            },
        );
    }
}