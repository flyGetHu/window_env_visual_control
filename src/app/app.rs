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
    show_batch_delete_confirm: bool,
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
    batch_mode: bool,
    selected_variables: std::collections::HashSet<String>,
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
            show_batch_delete_confirm: false,
            delete_confirm_variable: None,
            theme: ModernTheme::new(),
            is_dark_mode: false,
            search_query: String::new(),
            expanded_variables: std::collections::HashSet::new(),
            hovered_variable: None,
            show_variable_details: false,
            selected_detail_variable: None,
            show_export_dialog: false,
            batch_mode: false,
            selected_variables: std::collections::HashSet::new(),
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

    fn toggle_batch_mode(&mut self) {
        self.batch_mode = !self.batch_mode;
        if !self.batch_mode {
            self.selected_variables.clear();
        }
    }

    fn get_selected_counts(&self) -> (usize, usize) {
        let mut user_count = 0;
        let mut system_count = 0;

        for name in &self.selected_variables {
            if let Some(var) = self.variables.iter().find(|v| &v.name == name) {
                match var.scope {
                    EnvScope::User => user_count += 1,
                    EnvScope::System => system_count += 1,
                }
            }
        }

        (user_count, system_count)
    }

    fn batch_delete_variables(&mut self) {
        let (user_count, _system_count) = self.get_selected_counts();
        
        if user_count > 0 {
            self.show_batch_delete_confirm = true;
        } else {
            self.state.set_info_message(Some("没有选择可删除的用户变量".to_string()));
        }
    }

    fn confirm_batch_delete(&mut self) {
        let mut success_count = 0;
        let mut error_count = 0;

        let delete_requests: Vec<String> = self.selected_variables
            .iter()
            .filter(|name| {
                self.variables
                    .iter()
                    .find(|v| &v.name == *name)
                    .map_or(false, |v| v.scope == EnvScope::User)
            })
            .cloned()
            .collect();

        for name in delete_requests {
            match self.state.delete_variable(&name) {
                Ok(_) => success_count += 1,
                Err(_) => error_count += 1,
            }
        }

        self.selected_variables.clear();
        self.refresh_variables();
        
        let message = if error_count > 0 {
            format!("批量删除完成: 成功 {} 个, 失败 {} 个", success_count, error_count)
        } else {
            format!("成功删除 {} 个用户变量", success_count)
        };
        
        self.state.set_info_message(Some(message));
        self.show_batch_delete_confirm = false;
    }

    fn export_selected_variables(&self) {
        let selected_vars: Vec<&EnvVariable> = self.variables
            .iter()
            .filter(|v| self.selected_variables.contains(&v.name))
            .collect();

        if selected_vars.is_empty() {
            self.state.set_info_message(Some("没有选择要导出的变量".to_string()));
            return;
        }

        let export_data: Vec<_> = selected_vars
            .iter()
            .map(|v| format!("{}={}", v.name, v.value))
            .collect();

        let _content = export_data.join("\n");
        
        // 这里可以添加实际的文件导出逻辑
        self.state.set_info_message(Some(format!("已导出 {} 个变量", selected_vars.len())));
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
                self.batch_mode,
                self.is_dark_mode,
                self.compact_mode,
                &self.theme,
            );
            
            let header_actions = header.show(ui);

            // 处理头部操作
            if header_actions.toggle_theme {
                self.toggle_theme();
            }
            if header_actions.toggle_batch_mode {
                self.toggle_batch_mode();
            }
            if header_actions.refresh_variables {
                self.refresh_variables();
            }
            if header_actions.show_add_dialog {
                self.show_add_dialog = true;
            }
            if header_actions.show_export {
                self.export_selected_variables();
            }

            ui.add_space(8.0);

            // 批量操作栏
            if self.batch_mode {
                let batch_actions = BatchActions::new(
                    self.selected_variables.len(),
                    &self.theme,
                );
                let batch_results = batch_actions.show(ui);
                
                if batch_results.delete_selected {
                    self.batch_delete_variables();
                }
                if batch_results.export_selected {
                    self.export_selected_variables();
                }
                
                ui.add_space(8.0);
            }

            // 变量列表
            let mut var_list = VariableList::new(
                &self.variables,
                &self.theme,
                &self.search_query,
                self.batch_mode,
                &mut self.selected_variables,
                &mut self.editing_variable,
                &mut self.hovered_variable,
                &mut self.expanded_variables,
            );

            let delete_requests = var_list.show(ui);
            for name in delete_requests {
                self.delete_variable(&name);
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

            let (user_count, system_count) = self.get_selected_counts();
            let mut batch_delete_dialog = BatchDeleteConfirmDialog::new(
                &mut self.show_batch_delete_confirm,
                user_count,
                system_count,
                &self.theme,
            );
            if batch_delete_dialog.show(ui) {
                self.confirm_batch_delete();
            }
        });
    }
}
