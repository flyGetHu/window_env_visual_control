use eframe::egui;
use std::sync::Arc;

use crate::app::state::AppState;
use crate::app::theme::{ModernTheme, ThemeExt};
use crate::app::components::*;
use crate::models::env_variable::{EnvScope, EnvVariable};

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
    expanded_variables: std::collections::HashSet<String>,
    hovered_variable: Option<String>,
    show_variable_details: bool,
    selected_detail_variable: Option<String>,
    show_export_dialog: bool,
    // 响应式UI控制字段
    compact_mode: bool,
    window_width: f32,
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
            expanded_variables: std::collections::HashSet::new(),
            hovered_variable: None,
            show_variable_details: false,
            selected_detail_variable: None,
            show_export_dialog: false,
            compact_mode: false,
            window_width: 1200.0,
        }
    }
}

impl EnvManagerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();

        if let Err(e) = app.load_variables() {
            app.state
                .set_error_message(Some(format!("加载环境变量失败: {}", e)));
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
                .set_error_message(Some(format!("刷新变量失败: {}", e)));
        } else {
            self.state
                .set_info_message(Some("环境变量已刷新".to_string()));
        }
    }

    fn add_variable(&mut self) {
        if self.new_variable_name.is_empty() || self.new_variable_value.is_empty() {
            self.state
                .set_error_message(Some("变量名和值不能为空".to_string()));
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
                    .set_info_message(Some("变量添加成功".to_string()));
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
                    .set_info_message(Some("变量更新成功".to_string()));
            }
            Err(e) => {
                self.state.set_error_message(Some(e));
            }
        }
    }

    fn delete_variable(&mut self, name: &str) {
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
                        .set_info_message(Some("变量删除成功".to_string()));
                }
                Err(e) => {
                    self.state.set_error_message(Some(e));
                }
            }
        }
        self.show_delete_confirm = false;
        self.delete_confirm_variable = None;
    }

    fn render_variable_section(&mut self, ui: &mut egui::Ui, variable: &EnvVariable, is_readonly: bool) {
        let is_expanded = self.expanded_variables.contains(&variable.name);
        let is_hovered = self.hovered_variable.as_ref() == Some(&variable.name);

        let card_color = if is_readonly {
            self.theme.system_variable_accent
        } else {
            self.theme.user_variable_accent
        };

        egui::Frame::none()
            .fill(if is_hovered { self.theme.card_hover } else { self.theme.card_background })
            .inner_margin(egui::Margin::same(8.0))
            .rounding(6.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // 变量名
                    ui.colored_label(card_color, &variable.name);
                    ui.add_space(8.0);

                    // 变量值
                    let value_text = if variable.value.len() > 50 {
                        format!("{}...", &variable.value[..50])
                    } else {
                        variable.value.clone()
                    };
                    
                    ui.label(&value_text);
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if !is_readonly {
                            if ui.small_button("编辑").clicked() {
                                self.editing_variable = Some(variable.name.clone());
                            }
                            
                            if ui.small_button("删除").clicked() {
                                self.delete_variable(&variable.name);
                            }
                        }
                    });
                });

                // 展开/收起按钮
                if ui.small_button(if is_expanded { "收起" } else { "展开" }).clicked() {
                    if is_expanded {
                        self.expanded_variables.remove(&variable.name);
                    } else {
                        self.expanded_variables.insert(variable.name.clone());
                    }
                }

                if is_expanded {
                    ui.add_space(8.0);
                    egui::Frame::none()
                        .fill(self.theme.card_background)
                        .inner_margin(egui::Margin::same(8.0))
                        .rounding(4.0)
                        .show(ui, |ui| {
                            ui.label(format!("完整值: {}", variable.value));
                            ui.label(format!("作用域: {:?}", variable.scope));
                            ui.label(format!("创建时间: {}", variable.created_at.format("%Y-%m-%d %H:%M:%S")));
                            ui.label(format!("更新时间: {}", variable.updated_at.format("%Y-%m-%d %H:%M:%S")));
                        });
                }
            });

        if ui.ctx().input(|i| i.pointer.any_click()) {
            let response = ui.interact(ui.min_rect(), egui::Id::new(&variable.name), egui::Sense::click());
            if response.hovered() {
                self.hovered_variable = Some(variable.name.clone());
            }
        }
    }

    fn export_all_variables(&self) {
        if self.variables.is_empty() {
            self.state.set_info_message(Some("没有变量可导出".to_string()));
            return;
        }

        let content = self.variables
            .iter()
            .map(|v| format!("{}={}", v.name, v.value))
            .collect::<Vec<_>>()
            .join("\n");

        let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
        let filename = format!("env_{}.txt", timestamp);
        
        match std::fs::write(&filename, content) {
            Ok(_) => {
                self.state.set_info_message(Some(format!("已导出 {} 个变量到 {}", self.variables.len(), filename)));
            }
            Err(e) => {
                self.state.set_error_message(Some(format!("导出失败: {}", e)));
            }
        }
    }
}

impl eframe::App for EnvManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.theme.apply_modern_style(ctx, self.is_dark_mode, &self.theme);

        // 显示状态消息
        if let Some(error) = &self.state.get_error_message() {
            egui::Window::new("错误")
                .open(&mut true)
                .show(ctx, |ui| {
                    ui.label(error);
                    if ui.button("确定").clicked() {
                        self.state.set_error_message(None);
                    }
                });
        }

        if let Some(info) = &self.state.get_info_message() {
            egui::Window::new("提示")
                .open(&mut true)
                .show(ctx, |ui| {
                    ui.label(info);
                    if ui.button("确定").clicked() {
                        self.state.set_info_message(None);
                    }
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.window_width = ui.available_width();
            self.compact_mode = self.window_width < 800.0;

            // 头部区域
            let mut header = AppHeader::new(
                &mut self.search_query,
                self.is_dark_mode,
                self.compact_mode,
                &self.theme,
            );
            
            let header_actions = header.show(ui);

            // 处理头部操作
            if header_actions.toggle_theme {
                self.toggle_theme();
            }
            if header_actions.refresh_variables {
                self.refresh_variables();
            }
            if header_actions.show_add_dialog {
                self.show_add_dialog = true;
            }
            if header_actions.show_export {
                self.export_all_variables();
            }

            ui.add_space(8.0);

            // 分类显示变量
            let system_vars: Vec<EnvVariable> = self.variables
                .iter()
                .filter(|var| {
                    matches!(var.scope, EnvScope::System) &&
                    (self.search_query.is_empty()
                        || var.name.to_lowercase().contains(&self.search_query.to_lowercase())
                        || var.value.to_lowercase().contains(&self.search_query.to_lowercase()))
                })
                .cloned()
                .collect();

            let user_vars: Vec<EnvVariable> = self.variables
                .iter()
                .filter(|var| {
                    !matches!(var.scope, EnvScope::System) &&
                    (self.search_query.is_empty()
                        || var.name.to_lowercase().contains(&self.search_query.to_lowercase())
                        || var.value.to_lowercase().contains(&self.search_query.to_lowercase()))
                })
                .cloned()
                .collect();

            // 系统变量区域
            if !system_vars.is_empty() {
                ui.heading("系统变量 (只读)");
                ui.add_space(4.0);
                
                for variable in &system_vars {
                    self.render_variable_section(ui, variable, true);
                }
                
                ui.add_space(16.0);
            }

            // 用户变量区域
            if !user_vars.is_empty() {
                ui.heading("用户变量");
                ui.add_space(4.0);
                
                for variable in &user_vars {
                    self.render_variable_section(ui, variable, false);
                }
            }

            if system_vars.is_empty() && user_vars.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("没有找到匹配的环境变量");
                });
            }

            // 处理编辑状态
            let editing_name = self.editing_variable.clone();
            if let Some(name) = editing_name {
                if let Some(variable) = self.variables.iter().find(|v| v.name == name) {
                    let mut new_value = variable.value.clone();
                    
                    egui::Window::new(format!("编辑: {}", name))
                        .open(&mut true)
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("变量值:");
                                ui.text_edit_singleline(&mut new_value);
                            });
                            
                            ui.horizontal(|ui| {
                                if ui.button("保存").clicked() {
                                    self.update_variable(&name, new_value);
                                    self.editing_variable = None;
                                }
                                
                                if ui.button("取消").clicked() {
                                    self.editing_variable = None;
                                }
                            });
                        });
                }
            }

            // 对话框
            let mut add_dialog = AddVariableDialog::new(
                &mut self.show_add_dialog,
                &mut self.new_variable_name,
                &mut self.new_variable_value,
                &mut self.new_variable_scope,
                &self.theme,
            );
            if add_dialog.show(ui) {
                self.add_variable();
            }

            let mut delete_dialog = DeleteConfirmDialog::new(
                &mut self.show_delete_confirm,
                self.delete_confirm_variable.as_deref().unwrap_or(""),
                &self.theme,
            );
            if delete_dialog.show(ui) {
                self.confirm_delete_variable();
            }
        });
    }
}
