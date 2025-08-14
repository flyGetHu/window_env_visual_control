use eframe::egui;
use egui::{Color32, FontId, Rounding, Stroke, Vec2, Widget};

use crate::app::state::AppState;
use crate::models::env_variable::{EnvScope, EnvVariable};
use std::sync::Arc;

// 现代化UI主题配置
#[derive(Clone)]
struct ModernTheme {
    primary_color: Color32,
    secondary_color: Color32,
    accent_color: Color32,
    success_color: Color32,
    warning_color: Color32,
    error_color: Color32,
    background_color: Color32,
    surface_color: Color32,
    text_primary: Color32,
    text_secondary: Color32,
    border_color: Color32,
    // 新增视觉层次颜色
    card_background: Color32,
    card_hover: Color32,
    card_selected: Color32,
    shadow_color: Color32,
    divider_color: Color32,
    input_background: Color32,
    input_border: Color32,
    input_focus: Color32,
    button_hover: Color32,
    button_active: Color32,
    tooltip_background: Color32,
    user_variable_accent: Color32,
    system_variable_accent: Color32,
}

impl ModernTheme {
    fn new() -> Self {
        Self {
            primary_color: Color32::from_rgb(59, 130, 246), // 蓝色主色调
            secondary_color: Color32::from_rgb(99, 102, 241), // 紫色辅助色
            accent_color: Color32::from_rgb(16, 185, 129),  // 绿色强调色
            success_color: Color32::from_rgb(34, 197, 94),  // 成功绿色
            warning_color: Color32::from_rgb(251, 191, 36), // 警告黄色
            error_color: Color32::from_rgb(239, 68, 68),    // 错误红色
            background_color: Color32::from_rgb(248, 250, 252), // 浅灰背景
            surface_color: Color32::WHITE,                  // 白色表面
            text_primary: Color32::from_rgb(15, 23, 42),    // 深色主文本
            text_secondary: Color32::from_rgb(100, 116, 139), // 灰色辅助文本
            border_color: Color32::from_rgb(226, 232, 240), // 边框颜色
            // 新增视觉层次颜色
            card_background: Color32::from_rgb(255, 255, 255), // 卡片背景
            card_hover: Color32::from_rgb(249, 250, 251),      // 卡片悬停
            card_selected: Color32::from_rgb(239, 246, 255),   // 卡片选中
            shadow_color: Color32::from_rgba_unmultiplied(0, 0, 0, 10), // 阴影
            divider_color: Color32::from_rgb(241, 245, 249),   // 分割线
            input_background: Color32::from_rgb(255, 255, 255), // 输入框背景
            input_border: Color32::from_rgb(209, 213, 219),    // 输入框边框
            input_focus: Color32::from_rgb(59, 130, 246),      // 输入框聚焦
            button_hover: Color32::from_rgb(37, 99, 235),      // 按钮悬停
            button_active: Color32::from_rgb(29, 78, 216),     // 按钮激活
            tooltip_background: Color32::from_rgb(17, 24, 39), // 工具提示背景
            user_variable_accent: Color32::from_rgb(34, 197, 94), // 用户变量强调色
            system_variable_accent: Color32::from_rgb(251, 191, 36), // 系统变量强调色
        }
    }

    fn dark() -> Self {
        Self {
            primary_color: Color32::from_rgb(96, 165, 250), // 亮蓝色
            secondary_color: Color32::from_rgb(129, 140, 248), // 亮紫色
            accent_color: Color32::from_rgb(52, 211, 153),  // 亮绿色
            success_color: Color32::from_rgb(74, 222, 128), // 亮绿色
            warning_color: Color32::from_rgb(252, 211, 77), // 亮黄色
            error_color: Color32::from_rgb(248, 113, 113),  // 亮红色
            background_color: Color32::from_rgb(15, 23, 42), // 深色背景
            surface_color: Color32::from_rgb(30, 41, 59),   // 深色表面
            text_primary: Color32::from_rgb(248, 250, 252), // 浅色主文本
            text_secondary: Color32::from_rgb(148, 163, 184), // 灰色辅助文本
            border_color: Color32::from_rgb(51, 65, 85),    // 深色边框
            // 新增视觉层次颜色（暗色主题）
            card_background: Color32::from_rgb(30, 41, 59), // 卡片背景
            card_hover: Color32::from_rgb(51, 65, 85),      // 卡片悬停
            card_selected: Color32::from_rgb(30, 58, 138),  // 卡片选中
            shadow_color: Color32::from_rgba_unmultiplied(0, 0, 0, 25), // 阴影
            divider_color: Color32::from_rgb(51, 65, 85),   // 分割线
            input_background: Color32::from_rgb(30, 41, 59), // 输入框背景
            input_border: Color32::from_rgb(71, 85, 105),   // 输入框边框
            input_focus: Color32::from_rgb(96, 165, 250),   // 输入框聚焦
            button_hover: Color32::from_rgb(59, 130, 246),  // 按钮悬停
            button_active: Color32::from_rgb(37, 99, 235),  // 按钮激活
            tooltip_background: Color32::from_rgb(51, 65, 85), // 工具提示背景
            user_variable_accent: Color32::from_rgb(74, 222, 128), // 用户变量强调色
            system_variable_accent: Color32::from_rgb(252, 211, 77), // 系统变量强调色
        }
    }
}

pub struct EnvManagerApp {
    state: Arc<AppState>,
    variables: Vec<EnvVariable>,
    selected_variable: Option<String>,
    editing_variable: Option<String>,
    new_variable_name: String,
    new_variable_value: String,
    new_variable_scope: EnvScope,
    show_add_dialog: bool,
    show_delete_confirm: bool,
    delete_confirm_variable: Option<String>,
    theme: ModernTheme,
    is_dark_mode: bool,
    search_query: String,
    // UI状态字段
    expanded_variables: std::collections::HashSet<String>, // 展开的变量详情
    hovered_variable: Option<String>,                      // 当前悬停的变量
    show_variable_details: bool,                           // 显示变量详情面板
    selected_detail_variable: Option<String>,              // 详情面板中选中的变量
    show_export_dialog: bool,                              // 显示导出对话框
    animation_time: f32,                                   // 动画时间
    search_focused: bool,                                  // 搜索框是否聚焦
    window_width: f32,                                     // 窗口宽度，用于响应式设计
    window_height: f32,                                    // 窗口高度
    // 响应式UI控制字段
    show_secondary_buttons: bool, // 控制次要按钮的显示
    compact_mode: bool,           // 紧凑模式
    header_collapsed: bool,       // 头部折叠状态
}

// 现代化UI辅助函数
impl EnvManagerApp {
    fn apply_modern_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // 设置现代化的字体大小和间距
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

        // 设置现代化的间距
        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        style.spacing.button_padding = Vec2::new(12.0, 8.0);
        style.spacing.menu_margin = egui::Margin::same(8.0);
        style.spacing.indent = 20.0;

        // 设置现代化的圆角
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
        style.visuals.widgets.active.rounding = Rounding::same(8.0);
        style.visuals.widgets.open.rounding = Rounding::same(8.0);

        // 设置现代化的颜色
        if self.is_dark_mode {
            style.visuals.dark_mode = true;
            style.visuals.override_text_color = Some(self.theme.text_primary);
            style.visuals.panel_fill = self.theme.surface_color;
            style.visuals.window_fill = self.theme.surface_color;
        } else {
            style.visuals.dark_mode = false;
            style.visuals.override_text_color = Some(self.theme.text_primary);
            style.visuals.panel_fill = self.theme.surface_color;
            style.visuals.window_fill = self.theme.surface_color;
        }

        // 设置现代化的边框
        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.theme.border_color);
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, self.theme.border_color);

        ctx.set_style(style);
    }

    fn modern_button(&self, ui: &mut egui::Ui, text: &str, color: Color32) -> egui::Response {
        let button = egui::Button::new(text)
            .fill(color)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(0.0, Color32::TRANSPARENT));
        ui.add_sized([120.0, 36.0], button)
    }

    fn modern_small_button(&self, ui: &mut egui::Ui, text: &str, color: Color32) -> egui::Response {
        let button = egui::Button::new(text)
            .fill(color)
            .rounding(Rounding::same(6.0))
            .stroke(Stroke::new(0.0, Color32::TRANSPARENT));
        ui.add_sized([80.0, 28.0], button)
    }

    fn modern_card(&self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
        egui::Frame::none()
            .fill(self.theme.card_background)
            .rounding(Rounding::same(12.0))
            .stroke(Stroke::new(1.0, self.theme.border_color))
            .inner_margin(egui::Margin::same(16.0))
            .shadow(egui::epaint::Shadow {
                offset: Vec2::new(0.0, 2.0),
                blur: 8.0,
                spread: 0.0,
                color: self.theme.shadow_color,
            })
            .show(ui, add_contents);
    }

    fn interactive_card(
        &self,
        ui: &mut egui::Ui,
        is_hovered: bool,
        is_selected: bool,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> egui::Response {
        let fill_color = if is_selected {
            self.theme.card_selected
        } else if is_hovered {
            self.theme.card_hover
        } else {
            self.theme.card_background
        };

        let shadow_blur = if is_hovered { 12.0 } else { 6.0 };
        let shadow_offset = if is_hovered {
            Vec2::new(0.0, 4.0)
        } else {
            Vec2::new(0.0, 2.0)
        };

        egui::Frame::none()
            .fill(fill_color)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(
                1.0,
                if is_selected {
                    self.theme.primary_color
                } else {
                    self.theme.border_color
                },
            ))
            .inner_margin(egui::Margin::same(12.0))
            .shadow(egui::epaint::Shadow {
                offset: shadow_offset,
                blur: shadow_blur,
                spread: 0.0,
                color: self.theme.shadow_color,
            })
            .show(ui, add_contents)
            .response
    }

    fn enhanced_button(
        &self,
        ui: &mut egui::Ui,
        text: &str,
        color: Color32,
        icon: Option<&str>,
    ) -> egui::Response {
        let button_height = 36.0;
        let button_padding = egui::Margin::symmetric(16.0, 8.0);

        ui.allocate_ui_with_layout(
            egui::Vec2::new(ui.available_width(), button_height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                let response = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), button_height),
                    egui::Sense::click(),
                );
                let rect = response.rect;

                let fill_color = if response.clicked() {
                    self.theme.button_active
                } else if response.hovered() {
                    self.theme.button_hover
                } else {
                    color
                };

                ui.painter()
                    .rect_filled(rect, Rounding::same(8.0), fill_color);

                let text_color = Color32::WHITE;
                let text_pos = rect.center() - egui::Vec2::new(0.0, 0.0);

                if let Some(icon_text) = icon {
                    let icon_pos = egui::Pos2::new(rect.left() + 12.0, rect.center().y);
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        icon_text,
                        FontId::proportional(16.0),
                        text_color,
                    );

                    let text_pos = egui::Pos2::new(rect.left() + 32.0, rect.center().y);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_CENTER,
                        text,
                        FontId::proportional(14.0),
                        text_color,
                    );
                } else {
                    ui.painter().text(
                        text_pos,
                        egui::Align2::CENTER_CENTER,
                        text,
                        FontId::proportional(14.0),
                        text_color,
                    );
                }

                response
            },
        )
        .inner
    }

    fn show_tooltip(&self, ui: &mut egui::Ui, text: &str, response: &egui::Response) {
        if response.hovered() {
            egui::show_tooltip_at_pointer(
                ui.ctx(),
                egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("tooltip")),
                egui::Id::new("tooltip"),
                |ui: &mut egui::Ui| {
                    egui::Frame::popup(ui.style())
                        .fill(self.theme.tooltip_background)
                        .stroke(Stroke::new(1.0, self.theme.border_color))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(egui::Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.colored_label(Color32::WHITE, text);
                        });
                },
            );
        }
    }
}

impl Default for EnvManagerApp {
    fn default() -> Self {
        Self {
            state: Arc::new(AppState::new()),
            variables: Vec::new(),
            selected_variable: None,
            editing_variable: None,
            new_variable_name: String::new(),
            new_variable_value: String::new(),
            new_variable_scope: EnvScope::User,
            show_add_dialog: false,
            show_delete_confirm: false,
            delete_confirm_variable: None,
            theme: ModernTheme::new(),
            is_dark_mode: false,
            search_query: String::new(),
            // UI状态字段的默认值
            expanded_variables: std::collections::HashSet::new(),
            hovered_variable: None,
            show_variable_details: false,
            selected_detail_variable: None,
            show_export_dialog: false,
            animation_time: 0.0,
            search_focused: false,
            window_width: 1200.0,
            window_height: 800.0,
            // 响应式UI控制字段的默认值
            show_secondary_buttons: true,
            compact_mode: false,
            header_collapsed: false,
        }
    }
}

impl EnvManagerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();

        // 加载环境变量
        if let Err(e) = app.load_variables() {
            app.state
                .set_error_message(Some(format!("Failed to load environment variables: {}", e)));
        }

        app
    }

    fn toggle_theme(&mut self) {
        self.is_dark_mode = !self.is_dark_mode;
        self.theme = if self.is_dark_mode {
            ModernTheme::dark()
        } else {
            ModernTheme::new()
        };
    }

    fn load_variables(&mut self) -> Result<(), String> {
        self.variables = self.state.load_environment_variables()?;
        Ok(())
    }

    fn refresh_variables(&mut self) {
        if let Err(e) = self.load_variables() {
            self.state
                .set_error_message(Some(format!("Failed to refresh variables: {}", e)));
        } else {
            self.state
                .set_info_message(Some("Environment variables refreshed".to_string()));
        }
    }

    fn add_variable(&mut self) {
        if self.new_variable_name.is_empty() || self.new_variable_value.is_empty() {
            self.state
                .set_error_message(Some("Variable name and value cannot be empty".to_string()));
            return;
        }

        match self.state.add_variable(
            self.new_variable_name.clone(),
            self.new_variable_value.clone(),
            self.new_variable_scope.clone(),
        ) {
            Ok(_) => {
                self.new_variable_name.clear();
                self.new_variable_value.clear();
                self.show_add_dialog = false;
                self.refresh_variables();
                self.state
                    .set_info_message(Some("Variable added successfully".to_string()));
            }
            Err(e) => {
                self.state.set_error_message(Some(e));
            }
        }
    }

    fn update_variable(&mut self, name: &str, value: String) {
        match self.state.update_variable(name, value) {
            Ok(_) => {
                self.editing_variable = None;
                self.refresh_variables();
                self.state
                    .set_info_message(Some("Variable updated successfully".to_string()));
            }
            Err(e) => {
                self.state.set_error_message(Some(e));
            }
        }
    }

    fn delete_variable(&mut self, name: &str) {
        // 显示删除确认对话框
        self.delete_confirm_variable = Some(name.to_string());
        self.show_delete_confirm = true;
    }

    fn confirm_delete_variable(&mut self) {
        if let Some(name) = &self.delete_confirm_variable {
            match self.state.delete_variable(name) {
                Ok(_) => {
                    self.selected_variable = None;
                    self.refresh_variables();
                    self.state
                        .set_info_message(Some("Variable deleted successfully".to_string()));
                }
                Err(e) => {
                    self.state.set_error_message(Some(e));
                }
            }
        }
        self.show_delete_confirm = false;
        self.delete_confirm_variable = None;
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        // 更新窗口尺寸以实现响应式设计
        self.window_width = ui.available_width();
        self.window_height = ui.available_height();

        // 根据窗口宽度自动调整UI模式
        let is_narrow = self.window_width < 800.0;

        // 自动调整紧凑模式
        self.compact_mode = is_narrow;

        let mut toggle_theme = false;
        let mut show_settings = false;
        let mut show_add_dialog = false;
        let mut refresh_vars = false;
        let mut show_export_dialog = false;

        let theme = &self.theme;
        let is_dark_mode = self.is_dark_mode;

        egui::Frame::none()
            .fill(theme.surface_color)
            .stroke(Stroke::new(1.0, theme.border_color))
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::same(if self.compact_mode {
                8.0
            } else {
                12.0
            }))
            .show(ui, |ui| {
                if self.compact_mode {
                    // 紧凑模式：垂直布局
                    ui.vertical(|ui| {
                        // 标题行
                        ui.horizontal(|ui| {
                            ui.colored_label(
                                theme.text_primary,
                                egui::RichText::new("🌐 环境变量管理器").size(16.0).strong(),
                            );

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    // 主题切换按钮
                                    let theme_icon = if is_dark_mode { "☀" } else { "🌙" };
                                    if ui
                                        .add(
                                            egui::Button::new(theme_icon)
                                                .fill(theme.secondary_color.gamma_multiply(0.1))
                                                .stroke(Stroke::new(1.0, theme.secondary_color))
                                                .rounding(Rounding::same(4.0)),
                                        )
                                        .clicked()
                                    {
                                        toggle_theme = true;
                                    }
                                },
                            );
                        });

                        ui.add_space(8.0);

                        // 操作按钮
                        ui.horizontal_wrapped(|ui| {
                            // 添加按钮
                            if ui
                                .add(
                                    egui::Button::new("➕ 添加变量")
                                        .fill(theme.success_color.gamma_multiply(0.1))
                                        .stroke(Stroke::new(1.0, theme.success_color))
                                        .rounding(Rounding::same(4.0)),
                                )
                                .clicked()
                            {
                                show_add_dialog = true;
                            }

                            // 刷新按钮
                            if ui
                                .add(
                                    egui::Button::new("🔄 刷新")
                                        .fill(theme.primary_color.gamma_multiply(0.1))
                                        .stroke(Stroke::new(1.0, theme.primary_color))
                                        .rounding(Rounding::same(4.0)),
                                )
                                .clicked()
                            {
                                refresh_vars = true;
                            }

                            // 导出按钮
                            if ui
                                .add(
                                    egui::Button::new("📤 导出")
                                        .fill(theme.accent_color.gamma_multiply(0.1))
                                        .stroke(Stroke::new(1.0, theme.accent_color))
                                        .rounding(Rounding::same(4.0)),
                                )
                                .clicked()
                            {
                                show_export_dialog = true;
                            }
                        });
                    });
                } else {
                    // 标准模式：水平布局
                    ui.horizontal(|ui| {
                        // 左侧标题区域
                        ui.vertical(|ui| {
                            ui.add_space(4.0);
                            ui.colored_label(
                                theme.text_primary,
                                egui::RichText::new("🌐 环境变量管理器").size(20.0).strong(),
                            );
                            ui.colored_label(theme.text_secondary, "Windows环境变量管理工具");
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // 主题切换按钮
                            let theme_icon = if is_dark_mode { "☀" } else { "🌙" };
                            if ui
                                .add(
                                    egui::Button::new(theme_icon)
                                        .fill(theme.secondary_color.gamma_multiply(0.1))
                                        .stroke(Stroke::new(1.0, theme.secondary_color))
                                        .rounding(Rounding::same(6.0)),
                                )
                                .clicked()
                            {
                                toggle_theme = true;
                            }

                            ui.add_space(8.0);

                            // 导出按钮
                            if ui
                                .add(
                                    egui::Button::new("📤 导出")
                                        .fill(theme.accent_color.gamma_multiply(0.1))
                                        .stroke(Stroke::new(1.0, theme.accent_color))
                                        .rounding(Rounding::same(6.0)),
                                )
                                .clicked()
                            {
                                show_export_dialog = true;
                            }

                            ui.add_space(4.0);

                            // 刷新按钮
                            if ui
                                .add(
                                    egui::Button::new("🔄 刷新")
                                        .fill(theme.primary_color.gamma_multiply(0.1))
                                        .stroke(Stroke::new(1.0, theme.primary_color))
                                        .rounding(Rounding::same(6.0)),
                                )
                                .clicked()
                            {
                                refresh_vars = true;
                            }

                            ui.add_space(4.0);

                            // 添加按钮
                            if ui
                                .add(
                                    egui::Button::new("➕ 添加变量")
                                        .fill(theme.success_color.gamma_multiply(0.1))
                                        .stroke(Stroke::new(1.0, theme.success_color))
                                        .rounding(Rounding::same(6.0)),
                                )
                                .clicked()
                            {
                                show_add_dialog = true;
                            }
                        });
                    });
                }
            });

        // 在闭包外部处理状态变更
        if toggle_theme {
            self.toggle_theme();
        }
        if show_add_dialog {
            self.show_add_dialog = true;
        }
        if refresh_vars {
            self.refresh_variables();
        }
        if show_export_dialog {
            self.show_export_dialog = true;
        }
    }

    fn render_search(&mut self, ui: &mut egui::Ui) {
        self.modern_card(ui, |ui| {
            ui.horizontal(|ui| {
                // 搜索图标和标签
                ui.colored_label(self.theme.text_primary, "🔍");
                ui.colored_label(self.theme.text_primary, "搜索:");

                // 现代化搜索框
                let mut search = self.state.search_query.lock().unwrap().clone();
                let search_is_empty = search.is_empty();
                let search_response = ui.add_sized(
                    [300.0, 32.0],
                    egui::TextEdit::singleline(&mut search)
                        .hint_text("输入变量名称进行搜索...")
                        .desired_width(300.0),
                );

                if search_response.changed() {
                    self.state.set_search_query(search.clone());
                }

                ui.add_space(16.0);

                // 过滤器图标和标签
                ui.colored_label(self.theme.text_primary, "🎯");
                ui.colored_label(self.theme.text_primary, "过滤器:");

                let mut selected_scope = self.state.selected_scope.lock().unwrap().clone();

                let mut scope_str = match &selected_scope {
                    Some(EnvScope::User) => "用户变量",
                    Some(EnvScope::System) => "系统变量",
                    None => "用户变量", // 默认显示User
                }
                .to_string();

                // 现代化下拉框
                egui::ComboBox::from_label("")
                    .selected_text(&scope_str)
                    .width(120.0)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(&mut scope_str, "用户变量".to_string(), "👤 用户变量")
                            .clicked()
                        {
                            selected_scope = Some(EnvScope::User);
                        }

                        if ui
                            .selectable_value(&mut scope_str, "系统变量".to_string(), "🖥 系统变量")
                            .clicked()
                        {
                            selected_scope = Some(EnvScope::System);
                        }
                    });

                if selected_scope != *self.state.selected_scope.lock().unwrap() {
                    self.state.set_selected_scope(selected_scope);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 清除搜索按钮
                    if !search_is_empty {
                        if self
                            .modern_small_button(ui, "✖ 清除", self.theme.text_secondary)
                            .clicked()
                        {
                            self.state.set_search_query(String::new());
                        }
                    }
                });
            });
        });
    }

    fn render_variables_list(&mut self, ui: &mut egui::Ui) {
        let filtered_vars = self.state.filter_variables(&self.variables);
        let available_width = ui.available_width();

        // 计算响应式列宽
        let scope_width = 80.0;
        let actions_width = 120.0; // 操作按钮列宽度
        let remaining_width = available_width - scope_width - actions_width - 40.0; // 40.0 for margins
        let name_width = (remaining_width * 0.35).max(150.0); // 35% 最小150px
        let value_width = remaining_width - name_width;

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if filtered_vars.is_empty() {
                    // 空状态显示
                    ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.colored_label(
                            self.theme.text_secondary,
                            egui::RichText::new("📭").size(48.0),
                        );
                        ui.add_space(16.0);
                        ui.colored_label(self.theme.text_secondary, "没有找到匹配的环境变量");
                        ui.colored_label(self.theme.text_secondary, "尝试调整搜索条件或添加新变量");
                    });
                    return;
                }

                // 响应式表头
                self.modern_card(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(
                            Vec2::new(name_width, 24.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.colored_label(
                                    self.theme.text_secondary,
                                    egui::RichText::new("变量名").strong(),
                                );
                            },
                        );

                        ui.allocate_ui_with_layout(
                            Vec2::new(value_width, 24.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.colored_label(
                                    self.theme.text_secondary,
                                    egui::RichText::new("变量值").strong(),
                                );
                            },
                        );

                        ui.allocate_ui_with_layout(
                            Vec2::new(scope_width, 24.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.colored_label(
                                    self.theme.text_secondary,
                                    egui::RichText::new("作用域").strong(),
                                );
                            },
                        );

                        // 操作列标题
                        ui.allocate_ui_with_layout(
                            Vec2::new(actions_width, 24.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.colored_label(
                                    self.theme.text_secondary,
                                    egui::RichText::new("操作").strong(),
                                );
                            },
                        );
                    });
                });

                ui.add_space(8.0);

                // 响应式变量列表
                for (index, var) in filtered_vars.iter().enumerate() {
                    let is_selected = self.selected_variable.as_ref() == Some(&var.name);
                    let is_system = var.scope == EnvScope::System;
                    let is_editing = self.editing_variable.as_ref() == Some(&var.name);
                    let is_hovered = self.hovered_variable.as_ref() == Some(&var.name);
                    let is_expanded = self.expanded_variables.contains(&var.name);

                    // 变量行背景色
                    let bg_color = if is_selected {
                        self.theme.card_selected
                    } else if is_hovered {
                        self.theme.card_hover
                    } else if index % 2 == 0 {
                        self.theme.surface_color
                    } else {
                        self.theme.background_color
                    };

                    // 克隆需要的数据以避免借用冲突
                    let theme = self.theme.clone();
                    let var_name = var.name.clone();
                    let var_value = var.value.clone();
                    let var_scope = var.scope.clone();

                    // 用于在闭包外处理状态更新的变量
                    let mut name_clicked = false;
                    let mut save_clicked = false;
                    let mut cancel_clicked = false;
                    let mut edit_clicked = false;
                    let mut delete_clicked = false;
                    let mut new_value = var.value.clone();

                    let card_response = self.interactive_card(ui, is_hovered, is_selected, |ui| {
                        ui.horizontal(|ui| {
                            // 变量名 - 响应式
                            let name_color = if is_system {
                                theme.text_secondary
                            } else {
                                theme.text_primary
                            };
                            let name_icon = if is_system { "🖥" } else { "👤" };

                            ui.allocate_ui_with_layout(
                                Vec2::new(name_width, 32.0),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    let name_response = ui.selectable_label(
                                        is_selected,
                                        egui::RichText::new(format!("{} {}", name_icon, var_name))
                                            .color(name_color),
                                    );
                                    if name_response.clicked() {
                                        name_clicked = true;
                                    }
                                },
                            );

                            // 变量值 - 响应式
                            ui.allocate_ui_with_layout(
                                Vec2::new(
                                    value_width,
                                    if is_editing && !is_system { 80.0 } else { 32.0 },
                                ),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    if is_editing && !is_system {
                                        ui.vertical(|ui| {
                                            let _edit_response = ui.add_sized(
                                                [value_width - 10.0, 50.0],
                                                egui::TextEdit::multiline(&mut new_value)
                                                    .desired_width(value_width - 10.0),
                                            );

                                            ui.horizontal(|ui| {
                                                if ui.button("💾 保存").clicked() {
                                                    save_clicked = true;
                                                }
                                                if ui.button("❌ 取消").clicked() {
                                                    cancel_clicked = true;
                                                }
                                            });
                                        });
                                    } else {
                                        // 智能截断长文本
                                        let max_chars = ((value_width / 8.0) as usize).max(20);
                                        let value_text = if var_value.len() > max_chars {
                                            format!("{}...", &var_value[..max_chars])
                                        } else {
                                            var_value.clone()
                                        };

                                        let value_color = if is_system {
                                            theme.text_secondary
                                        } else {
                                            theme.text_primary
                                        };
                                        ui.label(
                                            egui::RichText::new(value_text)
                                                .color(value_color)
                                                .monospace(),
                                        );
                                    }
                                },
                            );

                            // 作用域标签 - 响应式
                            ui.allocate_ui_with_layout(
                                Vec2::new(scope_width, 32.0),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    let (scope_text, scope_color) = match var_scope {
                                        EnvScope::User => ("用户", theme.success_color),
                                        EnvScope::System => ("系统", theme.warning_color),
                                    };

                                    egui::Frame::none()
                                        .fill(scope_color.gamma_multiply(0.1))
                                        .rounding(Rounding::same(12.0))
                                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                        .show(ui, |ui| {
                                            ui.colored_label(scope_color, scope_text);
                                        });
                                },
                            );

                            // 操作按钮 - 响应式
                            ui.allocate_ui_with_layout(
                                Vec2::new(actions_width, 32.0),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.spacing_mut().item_spacing.x = 4.0;

                                    // 编辑按钮 - 只对用户变量显示
                                    if !is_system {
                                        if ui.small_button("✏️").on_hover_text("编辑变量").clicked()
                                        {
                                            edit_clicked = true;
                                        }
                                    }

                                    // 删除按钮 - 只对用户变量显示
                                    if !is_system {
                                        let delete_button =
                                            ui.small_button("🗑️").on_hover_text("删除变量");
                                        if delete_button.clicked() {
                                            delete_clicked = true;
                                        }
                                    }

                                    // 复制按钮 - 对所有变量显示
                                    if ui.small_button("📋").on_hover_text("复制变量值").clicked()
                                    {
                                        ui.output_mut(|o| o.copied_text = var_value.clone());
                                    }
                                },
                            );
                        });
                    });

                    // 在闭包外处理状态更新
                    if name_clicked {
                        self.selected_variable = Some(var.name.clone());
                    }

                    if save_clicked {
                        self.update_variable(&var.name, new_value);
                    }

                    if cancel_clicked {
                        self.editing_variable = None;
                    }

                    if edit_clicked {
                        self.editing_variable = Some(var.name.clone());
                    }

                    if delete_clicked {
                        self.delete_confirm_variable = Some(var.name.clone());
                        self.show_delete_confirm = true;
                    }

                    // 处理悬停和点击事件
                    if card_response.hovered() {
                        self.hovered_variable = Some(var.name.clone());
                        // 显示完整值的工具提示
                        if var.value.len() > 50 {
                            self.show_tooltip(ui, &var.value, &card_response);
                        }
                    } else if self.hovered_variable.as_ref() == Some(&var.name) {
                        self.hovered_variable = None;
                    }

                    // 双击展开详情
                    if card_response.double_clicked() {
                        if self.expanded_variables.contains(&var.name) {
                            self.expanded_variables.remove(&var.name);
                        } else {
                            self.expanded_variables.insert(var.name.clone());
                        }
                    }

                    // 右键显示详情面板
                    if card_response.secondary_clicked() {
                        self.selected_detail_variable = Some(var.name.clone());
                        self.show_variable_details = true;
                    }

                    // 展开的详情区域
                    if is_expanded {
                        ui.add_space(4.0);
                        self.modern_card(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(self.theme.text_secondary, "完整值:");
                                    ui.add_space(8.0);
                                    if ui.small_button("📋 复制").clicked() {
                                        ui.output_mut(|o| o.copied_text = var.value.clone());
                                    }
                                });

                                ui.add_space(4.0);

                                egui::Frame::none()
                                    .fill(self.theme.input_background)
                                    .rounding(Rounding::same(4.0))
                                    .inner_margin(egui::Margin::same(8.0))
                                    .show(ui, |ui| {
                                        ui.add(
                                            egui::TextEdit::multiline(&mut var.value.clone())
                                                .desired_rows(3)
                                                .interactive(false)
                                                .font(egui::TextStyle::Monospace),
                                        );
                                    });

                                ui.add_space(8.0);

                                ui.horizontal(|ui| {
                                    ui.colored_label(
                                        self.theme.text_secondary,
                                        format!("字符长度: {}", var.value.len()),
                                    );
                                    ui.add_space(16.0);
                                    ui.colored_label(
                                        self.theme.text_secondary,
                                        format!(
                                            "作用域: {}",
                                            match var.scope {
                                                EnvScope::User => "用户变量",
                                                EnvScope::System => "系统变量",
                                            }
                                        ),
                                    );
                                });
                            });
                        });
                    }

                    ui.add_space(2.0);
                }
            });
    }

    fn render_add_dialog(&mut self, ctx: &egui::Context) {
        let has_admin_permission = self
            .state
            .check_admin_permission(self.new_variable_scope.clone());
        let mut add_clicked = false;
        let mut cancel_clicked = false;

        egui::Window::new("")
            .open(&mut self.show_add_dialog)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([480.0, 360.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(self.theme.surface_color)
                    .stroke(Stroke::new(2.0, self.theme.primary_color))
                    .rounding(Rounding::same(16.0))
                    .shadow(egui::epaint::Shadow {
                        offset: Vec2::new(0.0, 4.0),
                        blur: 20.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(50),
                    }),
            )
            .show(ctx, |ui| {
                // 标题区域
                ui.vertical_centered(|ui| {
                    ui.add_space(16.0);
                    ui.colored_label(
                        self.theme.primary_color,
                        egui::RichText::new("➕ 添加新环境变量").size(20.0).strong(),
                    );
                    ui.colored_label(self.theme.text_secondary, "创建一个新的用户级环境变量");
                    ui.add_space(24.0);
                });

                // 表单区域
                ui.vertical(|ui| {
                    // 变量名输入
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            [80.0, 24.0],
                            egui::Label::new(
                                egui::RichText::new("🏷 变量名:")
                                    .color(self.theme.text_primary)
                                    .strong(),
                            ),
                        );

                        let name_edit = egui::TextEdit::singleline(&mut self.new_variable_name)
                            .desired_width(320.0)
                            .hint_text("例如: MY_CUSTOM_PATH")
                            .font(egui::TextStyle::Monospace);

                        ui.add(name_edit);
                    });

                    ui.add_space(16.0);

                    // 变量值输入
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            [80.0, 24.0],
                            egui::Label::new(
                                egui::RichText::new("📝 变量值:")
                                    .color(self.theme.text_primary)
                                    .strong(),
                            ),
                        );

                        let value_edit = egui::TextEdit::multiline(&mut self.new_variable_value)
                            .desired_width(320.0)
                            .desired_rows(4)
                            .hint_text("输入变量的值...")
                            .font(egui::TextStyle::Monospace);

                        ui.add(value_edit);
                    });

                    ui.add_space(16.0);

                    // 作用域选择
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            [80.0, 24.0],
                            egui::Label::new(
                                egui::RichText::new("🎯 作用域:")
                                    .color(self.theme.text_primary)
                                    .strong(),
                            ),
                        );

                        // 用户作用域（启用）
                        egui::Frame::none()
                            .fill(self.theme.success_color.gamma_multiply(0.1))
                            .stroke(Stroke::new(2.0, self.theme.success_color))
                            .rounding(Rounding::same(8.0))
                            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(self.theme.success_color, "👤");
                                    ui.colored_label(
                                        self.theme.success_color,
                                        egui::RichText::new("用户").strong(),
                                    );
                                    ui.colored_label(self.theme.text_secondary, "(推荐)");
                                });
                            });

                        ui.add_space(16.0);

                        // 系统作用域（禁用）
                        egui::Frame::none()
                            .fill(self.theme.text_secondary.gamma_multiply(0.05))
                            .stroke(Stroke::new(
                                1.0,
                                self.theme.text_secondary.gamma_multiply(0.3),
                            ))
                            .rounding(Rounding::same(8.0))
                            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(self.theme.text_secondary, "🖥");
                                    ui.colored_label(
                                        self.theme.text_secondary,
                                        egui::RichText::new("系统").strikethrough(),
                                    );
                                    ui.colored_label(self.theme.text_secondary, "(已禁用)");
                                });
                            });
                    });

                    ui.add_space(24.0);

                    // 按钮区域
                    ui.horizontal(|ui| {
                        ui.add_space(80.0); // 对齐到标签位置

                        if egui::Button::new("✅ 添加变量").ui(ui).clicked() {
                            add_clicked = true;
                        }

                        ui.add_space(12.0);

                        if egui::Button::new("❌ 取消").ui(ui).clicked() {
                            cancel_clicked = true;
                        }
                    });

                    ui.add_space(16.0);

                    // 提示信息
                    egui::Frame::none()
                        .fill(self.theme.primary_color.gamma_multiply(0.05))
                        .stroke(Stroke::new(
                            1.0,
                            self.theme.primary_color.gamma_multiply(0.3),
                        ))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(12.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.colored_label(self.theme.primary_color, "💡");
                                ui.vertical(|ui| {
                                    ui.colored_label(
                                        self.theme.text_secondary,
                                        "提示: 为了系统安全，只能创建用户级环境变量。",
                                    );
                                    ui.colored_label(
                                        self.theme.text_secondary,
                                        "变量名建议使用大写字母和下划线。",
                                    );
                                });
                            });
                        });
                });
            });

        if add_clicked {
            self.add_variable();
        }
        if cancel_clicked {
            self.show_add_dialog = false;
        }
    }

    fn render_status_bar(&mut self, ui: &mut egui::Ui) {
        self.modern_card(ui, |ui| {
            ui.horizontal(|ui| {
                // 变量统计
                ui.horizontal(|ui| {
                    ui.colored_label(self.theme.primary_color, "📊");
                    ui.colored_label(
                        self.theme.text_primary,
                        format!("变量总数: {}", self.variables.len()),
                    );
                });

                ui.add_space(16.0);

                // 自动刷新状态
                let auto_refresh = self.state.get_auto_refresh();
                ui.horizontal(|ui| {
                    let (icon, color) = if auto_refresh {
                        ("🔄", self.theme.success_color)
                    } else {
                        ("⏸️", self.theme.text_secondary)
                    };
                    ui.colored_label(color, icon);
                    ui.colored_label(
                        self.theme.text_primary,
                        format!("自动刷新: {}", if auto_refresh { "开启" } else { "关闭" }),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 错误消息
                    if let Some(error) = self.state.get_error_message() {
                        if self
                            .modern_small_button(ui, "✖ 清除", self.theme.error_color)
                            .clicked()
                        {
                            self.state.set_error_message(None);
                        }

                        ui.add_space(8.0);

                        egui::Frame::none()
                            .fill(self.theme.error_color.gamma_multiply(0.1))
                            .stroke(Stroke::new(1.0, self.theme.error_color))
                            .rounding(Rounding::same(6.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(self.theme.error_color, "❌");
                                    ui.colored_label(
                                        self.theme.error_color,
                                        egui::RichText::new(format!("错误: {}", error)).size(12.0),
                                    );
                                });
                            });
                    }

                    // 信息消息
                    if let Some(info) = self.state.get_info_message() {
                        if self
                            .modern_small_button(ui, "✖ 清除", self.theme.success_color)
                            .clicked()
                        {
                            self.state.set_info_message(None);
                        }

                        ui.add_space(8.0);

                        egui::Frame::none()
                            .fill(self.theme.success_color.gamma_multiply(0.1))
                            .stroke(Stroke::new(1.0, self.theme.success_color))
                            .rounding(Rounding::same(6.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(self.theme.success_color, "✅");
                                    ui.colored_label(
                                        self.theme.success_color,
                                        egui::RichText::new(format!("信息: {}", info)).size(12.0),
                                    );
                                });
                            });
                    }
                });
            });
        });
    }

    fn render_delete_confirm(&mut self, ctx: &egui::Context) {
        let mut confirm_clicked = false;
        let mut cancel_clicked = false;

        egui::Window::new("")
            .open(&mut self.show_delete_confirm)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([400.0, 280.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(self.theme.surface_color)
                    .stroke(Stroke::new(2.0, self.theme.warning_color))
                    .rounding(Rounding::same(16.0))
                    .shadow(egui::epaint::Shadow {
                        offset: Vec2::new(0.0, 4.0),
                        blur: 20.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(50),
                    }),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    // 警告图标
                    ui.colored_label(
                        self.theme.warning_color,
                        egui::RichText::new("⚠️").size(48.0),
                    );

                    ui.add_space(16.0);

                    // 标题
                    ui.colored_label(
                        self.theme.text_primary,
                        egui::RichText::new("确认删除").size(18.0).strong(),
                    );

                    ui.add_space(12.0);

                    if let Some(var_name) = &self.delete_confirm_variable {
                        // 描述文本
                        ui.colored_label(self.theme.text_secondary, "您确定要删除这个环境变量吗？");

                        ui.add_space(8.0);

                        // 变量名显示
                        egui::Frame::none()
                            .fill(self.theme.surface_color)
                            .stroke(Stroke::new(1.0, self.theme.border_color))
                            .rounding(Rounding::same(8.0))
                            .inner_margin(12.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(self.theme.error_color, "🗑️");
                                    ui.colored_label(
                                        self.theme.text_primary,
                                        egui::RichText::new("变量名:").strong(),
                                    );
                                    ui.colored_label(
                                        self.theme.error_color,
                                        egui::RichText::new(var_name).monospace().strong(),
                                    );
                                });
                            });

                        ui.add_space(12.0);

                        // 警告信息
                        ui.colored_label(self.theme.error_color, "⚠ 此操作无法撤销");

                        ui.add_space(24.0);

                        // 按钮区域
                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() / 2.0 - 100.0);

                            if egui::Button::new("🗑️ 删除").ui(ui).clicked() {
                                confirm_clicked = true;
                            }

                            ui.add_space(12.0);

                            if egui::Button::new("❌ 取消").ui(ui).clicked() {
                                cancel_clicked = true;
                            }
                        });
                    }

                    ui.add_space(20.0);
                });
            });

        if confirm_clicked {
            self.confirm_delete_variable();
        }
        if cancel_clicked {
            self.show_delete_confirm = false;
            self.delete_confirm_variable = None;
        }
    }

    fn render_export_dialog(&mut self, ctx: &egui::Context) {
        let theme = self.theme.clone();

        let mut export_file_clicked = false;
        let mut export_clipboard_clicked = false;
        let mut cancel_clicked = false;

        egui::Window::new("导出环境变量")
            .collapsible(false)
            .resizable(true)
            .default_size([500.0, 400.0])
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(theme.card_background)
                    .rounding(egui::Rounding::same(8.0))
                    .stroke(egui::Stroke::new(1.0, theme.border_color))
                    .inner_margin(egui::Margin::same(16.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.add_space(10.0);

                            ui.label(
                                egui::RichText::new("📤 导出环境变量")
                                    .size(18.0)
                                    .color(theme.primary_color),
                            );

                            ui.add_space(15.0);

                            ui.horizontal(|ui| {
                                if ui.button("💾 导出到文件").clicked() {
                                    export_file_clicked = true;
                                }

                                ui.add_space(10.0);

                                if ui.button("📋 复制到剪贴板").clicked() {
                                    export_clipboard_clicked = true;
                                }
                            });

                            ui.add_space(15.0);

                            ui.label(
                                egui::RichText::new("导出格式：")
                                    .size(14.0)
                                    .color(theme.text_secondary),
                            );

                            ui.horizontal(|ui| {
                                ui.radio_value(&mut true, true, ".env 格式");
                                ui.radio_value(&mut false, true, "JSON 格式");
                                ui.radio_value(&mut false, true, "PowerShell 格式");
                            });

                            ui.add_space(10.0);

                            ui.checkbox(&mut true, "仅导出用户变量");
                            ui.checkbox(&mut false, "包含系统变量");

                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                if ui.button("取消").clicked() {
                                    cancel_clicked = true;
                                }
                            });

                            ui.add_space(10.0);
                        });
                    });
            });

        // 在闭包外处理状态更新
        if export_file_clicked {
            // TODO: 实现文件导出功能
        }

        if export_clipboard_clicked {
            // TODO: 实现剪贴板导出功能
        }

        if cancel_clicked {
            self.show_export_dialog = false;
        }
    }

    fn render_variable_details(&mut self, ctx: &egui::Context) {
        if let Some(var_name) = &self.selected_detail_variable.clone() {
            // 克隆变量数据以避免借用冲突
            if let Some(variable) = self.variables.iter().find(|v| &v.name == var_name).cloned() {
                let theme = self.theme.clone(); // 克隆主题以避免借用冲突
                let var_name_clone = var_name.clone();

                let window_response = egui::Window::new(format!("变量详情 - {}", var_name))
                    .collapsible(false)
                    .resizable(true)
                    .default_size([600.0, 500.0])
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ctx, |ui| {
                        // 使用简单的 Frame 替代 modern_card 以避免借用冲突
                        egui::Frame::none()
                            .fill(theme.card_background)
                            .rounding(12.0)
                            .inner_margin(20.0)
                            .stroke(egui::Stroke::new(1.0, theme.border_color))
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.add_space(10.0);

                                    // 标题和作用域标识
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(&variable.name)
                                                .size(20.0)
                                                .color(theme.text_primary)
                                                .strong(),
                                        );

                                        ui.add_space(10.0);

                                        let (scope_text, scope_color) = match variable.scope {
                                            EnvScope::User => ("用户", theme.user_variable_accent),
                                            EnvScope::System => {
                                                ("系统", theme.system_variable_accent)
                                            }
                                        };

                                        ui.label(
                                            egui::RichText::new(scope_text)
                                                .size(12.0)
                                                .color(scope_color)
                                                .background_color(scope_color.gamma_multiply(0.1)),
                                        );
                                    });

                                    ui.add_space(15.0);

                                    // 变量值
                                    ui.label(
                                        egui::RichText::new("值：")
                                            .size(14.0)
                                            .color(theme.text_secondary)
                                            .strong(),
                                    );

                                    ui.add_space(5.0);

                                    egui::ScrollArea::vertical()
                                        .max_height(150.0)
                                        .show(ui, |ui| {
                                            ui.add(
                                                egui::TextEdit::multiline(
                                                    &mut variable.value.clone(),
                                                )
                                                .desired_width(f32::INFINITY)
                                                .desired_rows(5)
                                                .interactive(false),
                                            );
                                        });

                                    ui.add_space(15.0);

                                    // 元数据信息
                                    ui.label(
                                        egui::RichText::new("元数据：")
                                            .size(14.0)
                                            .color(theme.text_secondary)
                                            .strong(),
                                    );

                                    ui.add_space(5.0);

                                    ui.horizontal(|ui| {
                                        ui.label("创建时间：");
                                        ui.label(
                                            egui::RichText::new("2024-01-15 10:30:00")
                                                .color(theme.text_primary),
                                        );
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("最后修改：");
                                        ui.label(
                                            egui::RichText::new("2024-01-20 14:25:30")
                                                .color(theme.text_primary),
                                        );
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("字符长度：");
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{} 字符",
                                                variable.value.len()
                                            ))
                                            .color(theme.text_primary),
                                        );
                                    });

                                    ui.add_space(20.0);

                                    // 操作按钮
                                    let mut edit_clicked = false;
                                    let mut delete_clicked = false;
                                    let mut close_clicked = false;

                                    ui.horizontal(|ui| {
                                        if ui.button("📋 复制值").clicked() {
                                            ui.output_mut(|o| {
                                                o.copied_text = variable.value.clone()
                                            });
                                        }

                                        ui.add_space(10.0);

                                        if ui.button("✏️ 编辑").clicked() {
                                            edit_clicked = true;
                                        }

                                        ui.add_space(10.0);

                                        if ui.button("🗑️ 删除").clicked() {
                                            delete_clicked = true;
                                        }
                                    });

                                    ui.add_space(15.0);

                                    ui.horizontal(|ui| {
                                        if ui.button("关闭").clicked() {
                                            close_clicked = true;
                                        }
                                    });

                                    ui.add_space(10.0);

                                    // 返回按钮点击状态
                                    (edit_clicked, delete_clicked, close_clicked)
                                })
                            })
                    });

                // 在窗口外处理状态更新
                if let Some(window_inner) = window_response {
                    if let Some(frame_inner) = window_inner.inner {
                        let vertical_inner = frame_inner.inner;
                        let (edit_clicked, delete_clicked, close_clicked) = vertical_inner.inner;

                        if edit_clicked {
                            self.editing_variable = Some(var_name_clone.clone());
                            self.show_variable_details = false;
                        }

                        if delete_clicked {
                            self.delete_confirm_variable = Some(var_name_clone.clone());
                            self.show_delete_confirm = true;
                            self.show_variable_details = false;
                        }

                        if close_clicked {
                            self.show_variable_details = false;
                            self.selected_detail_variable = None;
                        }
                    }
                }
            }
        }
    }
}

impl eframe::App for EnvManagerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 更新窗口尺寸以支持响应式设计
        let screen_rect = ctx.screen_rect();
        self.window_width = screen_rect.width();
        self.window_height = screen_rect.height();

        // 更新动画时间
        self.animation_time += ctx.input(|i| i.unstable_dt);

        // 应用现代样式
        self.apply_modern_style(ctx);

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(4.0);
            self.render_header(ui);
            ui.add_space(4.0);
        });

        egui::TopBottomPanel::top("search").show(ctx, |ui| {
            ui.add_space(4.0);
            self.render_search(ui);
            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_variables_list(ui);
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.add_space(4.0);
            self.render_status_bar(ui);
            ui.add_space(4.0);
        });

        self.render_add_dialog(ctx);

        if self.show_delete_confirm {
            self.render_delete_confirm(ctx);
        }

        if self.show_export_dialog {
            self.render_export_dialog(ctx);
        }

        if self.show_variable_details {
            self.render_variable_details(ctx);
        }

        // 请求重绘以支持动画
        ctx.request_repaint();
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = self.state.save_config();
    }
}
