use egui::Ui;
use crate::app::theme::ModernTheme;


pub struct AppHeader<'a> {
    pub search_query: &'a mut String,
    pub batch_mode: bool,
    pub is_dark_mode: bool,
    pub compact_mode: bool,
    pub theme: &'a ModernTheme,
}

impl<'a> AppHeader<'a> {
    pub fn new(
        search_query: &'a mut String,
        is_dark_mode: bool,
        compact_mode: bool,
        theme: &'a ModernTheme,
    ) -> Self {
        Self {
            search_query,
            batch_mode: false, // Always false since batch mode is removed
            is_dark_mode,
            compact_mode,
            theme,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> HeaderActions {
        let mut actions = HeaderActions::default();

        if self.compact_mode {
            self.show_compact(ui, &mut actions);
        } else {
            self.show_normal(ui, &mut actions);
        }

        actions
    }

    fn show_normal(&mut self, ui: &mut Ui, actions: &mut HeaderActions) {
        ui.horizontal(|ui| {
            ui.colored_label(
                self.theme.text_primary,
                egui::RichText::new("🌐 环境变量管理器")
                    .size(20.0)
                    .strong(),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 主题切换
                let theme_icon = if self.is_dark_mode { "☀" } else { "🌙" };
                if ui.button(theme_icon).clicked() {
                    actions.toggle_theme = true;
                }

                ui.add_space(8.0);



                // 设置按钮
                if ui.button("⚙ 设置").clicked() {
                    actions.show_settings = true;
                }

                ui.add_space(8.0);

                // 导出按钮
                if ui.button("📤 导出").clicked() {
                    actions.show_export = true;
                }

                ui.add_space(8.0);

                // 刷新按钮
                if ui.button("🔄 刷新").clicked() {
                    actions.refresh_variables = true;
                }

                ui.add_space(8.0);

                // 添加按钮
                if ui.button("➕ 添加").clicked() {
                    actions.show_add_dialog = true;
                }

                ui.add_space(16.0);

                // 搜索框
                let search_width = (ui.available_width() * 0.3).min(300.0).max(200.0);
                ui.add_sized(
                    [search_width, 32.0],
                    egui::TextEdit::singleline(self.search_query)
                        .hint_text("搜索环境变量..."),
                );
            });
        });
    }

    fn show_compact(&mut self, ui: &mut Ui, actions: &mut HeaderActions) {
        ui.vertical(|ui| {
            // 标题行
            ui.horizontal(|ui| {
                ui.colored_label(
                    self.theme.text_primary,
                    egui::RichText::new("🌐 环境变量管理器")
                        .size(16.0)
                        .strong(),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let theme_icon = if self.is_dark_mode { "☀" } else { "🌙" };
                    if ui.small_button(theme_icon).clicked() {
                        actions.toggle_theme = true;
                    }
                });
            });

            ui.add_space(8.0);

            // 搜索框
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.add(
                    egui::TextEdit::singleline(self.search_query)
                        .hint_text("搜索...")
                );
            });

            ui.add_space(8.0);

            // 操作按钮
            ui.horizontal(|ui| {
                if ui.small_button("➕").clicked() {
                    actions.show_add_dialog = true;
                }
                
                if ui.small_button("🔄").clicked() {
                    actions.refresh_variables = true;
                }
                
                if ui.small_button("📤").clicked() {
                    actions.show_export = true;
                }
                
                if ui.small_button("⚙").clicked() {
                    actions.show_settings = true;
                }
            });
        });
    }
}

#[derive(Default)]
pub struct HeaderActions {
    pub toggle_theme: bool,
    pub show_settings: bool,
    pub show_export: bool,
    pub refresh_variables: bool,
    pub show_add_dialog: bool,
}