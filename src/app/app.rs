use eframe::egui;
use std::sync::Arc;

use crate::app::state::AppState;
use crate::models::env_variable::{EnvScope, EnvVariable};

struct AppStyle {
    sidebar_width: f32,
    spacing: f32,
}

impl Default for AppStyle {
    fn default() -> Self {
        Self {
            sidebar_width: 200.0,
            spacing: 8.0,
        }
    }
}

pub struct EnvManagerApp {
    state: Arc<AppState>,
    variables: Vec<EnvVariable>,
    selected_variable_name: Option<String>,
    editing_variable_name: Option<String>,
    new_variable_name: String,
    new_variable_value: String,
    show_add_dialog: bool,
    show_delete_confirm: bool,
    search_query: String,
    selected_scope: EnvScope,
    style: AppStyle,
}

impl Default for EnvManagerApp {
    fn default() -> Self {
        Self {
            state: Arc::new(AppState::new()),
            variables: Vec::new(),
            selected_variable_name: None,
            editing_variable_name: None,
            new_variable_name: String::new(),
            new_variable_value: String::new(),
            show_add_dialog: false,
            show_delete_confirm: false,
            search_query: String::new(),
            selected_scope: EnvScope::User,
            style: AppStyle::default(),
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
            self.selected_scope.clone(),
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
                self.editing_variable_name = None;
                self.refresh_variables();
                self.state
                    .set_info_message(Some("变量更新成功".to_string()));
            }
            Err(e) => {
                self.state.set_error_message(Some(e));
            }
        }
    }

    fn delete_variable(&mut self) {
        if let Some(name) = self.selected_variable_name.clone() {
            // 从变量列表中找到对应的变量及其作用域
            if let Some(variable) = self.variables.iter().find(|v| v.name == name) {
                match self.state.delete_variable(&name, variable.scope.clone()) {
                    Ok(_) => {
                        self.selected_variable_name = None;
                        self.refresh_variables();
                        self.state
                            .set_info_message(Some("变量删除成功".to_string()));
                    }
                    Err(e) => {
                        self.state.set_error_message(Some(e));
                    }
                }
            }
        }
        self.show_delete_confirm = false;
    }

    fn apply_changes(&mut self) {
        if let Err(e) = self.state.refresh_environment() {
            self.state.set_error_message(Some(format!("应用更改失败: {}", e)));
        } else {
            self.state.set_info_message(Some("更改已应用，可能需要重启应用生效".to_string()));
        }
    }
}

impl eframe::App for EnvManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_main_panel(ctx);
        self.handle_dialogs(ctx);
    }
}

impl EnvManagerApp {
    fn render_main_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar")
            .width_range(self.style.sidebar_width..=self.style.sidebar_width + 100.0)
            .show(ctx, |ui| {
                self.render_left_panel(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_right_panel(ui);
        });
    }

    fn render_left_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("环境变量管理器");
        });
        ui.add_space(self.style.spacing * 2.0);

        ui.group(|ui| {
            ui.label("变量类型");
            ui.selectable_value(&mut self.selected_scope, EnvScope::User, "用户变量");
            ui.selectable_value(&mut self.selected_scope, EnvScope::System, "系统变量");
        });
        ui.add_space(self.style.spacing);

        ui.label("操作");
        if ui.button("➕ 添加变量").clicked() {
            self.show_add_dialog = true;
        }

        let edit_button_enabled = self.selected_variable_name.is_some();
        ui.add_enabled(edit_button_enabled, egui::Button::new("✏️ 编辑变量"))
            .on_hover_text("选择一个变量后启用")
            .clicked()
            .then(|| {
                if let Some(name) = self.selected_variable_name.clone() {
                    self.editing_variable_name = Some(name);
                }
            });

        let delete_button_enabled = self.selected_variable_name.is_some();
        ui.add_enabled(delete_button_enabled, egui::Button::new("🗑️ 删除变量"))
            .on_hover_text("选择一个变量后启用")
            .clicked()
            .then(|| {
                self.show_delete_confirm = true;
            });
        
        ui.add_space(self.style.spacing * 2.0);
        
        if ui.button("🔄 应用更改").clicked() {
            self.apply_changes();
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.label("状态信息");
            if let Some(info) = self.state.get_info_message() {
                ui.label(info);
            }
             if let Some(error) = self.state.get_error_message() {
                ui.colored_label(egui::Color32::RED, error);
            }
        });
    }

    fn render_right_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("搜索:");
            ui.text_edit_singleline(&mut self.search_query);
        });
        ui.separator();

        let filtered_vars: Vec<EnvVariable> = self.variables
            .iter()
            .filter(|var| {
                let scope_match = var.scope == self.selected_scope;
                let search_match = self.search_query.is_empty()
                    || var.name.to_lowercase().contains(&self.search_query.to_lowercase())
                    || var.value.to_lowercase().contains(&self.search_query.to_lowercase());
                scope_match && search_match
            })
            .cloned()
            .collect();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for var in filtered_vars {
                let is_selected = self.selected_variable_name.as_ref() == Some(&var.name);
                let response = ui.selectable_label(is_selected, format!("{}: {}", var.name, var.value));
                if response.clicked() {
                    self.selected_variable_name = Some(var.name.clone());
                }
            }
        });
    }

    fn handle_dialogs(&mut self, ctx: &egui::Context) {
        let mut wants_to_add = false;
        if self.show_add_dialog {
            egui::Window::new("添加新变量")
                .open(&mut self.show_add_dialog)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("名称:");
                        ui.text_edit_singleline(&mut self.new_variable_name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("值:");
                        ui.text_edit_singleline(&mut self.new_variable_value);
                    });
                    if ui.button("确认添加").clicked() {
                        wants_to_add = true;
                    }
                });
        }
        if wants_to_add {
            self.add_variable();
        }

        let mut wants_to_update = false;
        let mut updated_value = String::new();
        if let Some(editing_name) = self.editing_variable_name.clone() {
             if let Some(variable) = self.variables.iter_mut().find(|v| v.name == editing_name) {
                let mut open = true;
                egui::Window::new(format!("编辑: {}", editing_name))
                    .open(&mut open)
                    .show(ctx, |ui| {
                        ui.text_edit_singleline(&mut variable.value);
                        if ui.button("保存").clicked() {
                            wants_to_update = true;
                            updated_value = variable.value.clone();
                        }
                    });
                if !open {
                    self.editing_variable_name = None;
                }
             }
        }
        if wants_to_update {
            if let Some(name) = self.editing_variable_name.clone() {
                self.update_variable(&name, updated_value);
            }
        }

        let mut wants_to_delete = false;
        if self.show_delete_confirm {
            let variable_to_delete = self.selected_variable_name.clone().unwrap_or_default();
            let mut open = true;
            let mut cancel = false;
            egui::Window::new("确认删除")
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.label(format!("确定要删除 '{}' 吗?", variable_to_delete));
                    ui.horizontal(|ui| {
                        if ui.button("确认").clicked() {
                            wants_to_delete = true;
                        }
                        if ui.button("取消").clicked() {
                            cancel = true;
                        }
                    });
                });

            if !open || wants_to_delete || cancel {
                self.show_delete_confirm = false;
            }

            if wants_to_delete {
                self.delete_variable();
            }
        }
    }
}
