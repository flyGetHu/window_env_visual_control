use eframe::egui;

use std::sync::Arc;
use crate::app::state::AppState;
use crate::models::env_variable::{EnvScope, EnvVariable};

pub struct EnvManagerApp {
    state: Arc<AppState>,
    variables: Vec<EnvVariable>,
    selected_variable: Option<String>,
    selected_variables: Vec<String>,  // 支持多选
    editing_variable: Option<String>,
    new_variable_name: String,
    new_variable_value: String,
    new_variable_scope: EnvScope,
    show_add_dialog: bool,
    show_settings: bool,
    show_batch_operations: bool,
    batch_operation_mode: bool,
}

impl Default for EnvManagerApp {
    fn default() -> Self {
        Self {
            state: Arc::new(AppState::new()),
            variables: Vec::new(),
            selected_variable: None,
            selected_variables: Vec::new(),
            editing_variable: None,
            new_variable_name: String::new(),
            new_variable_value: String::new(),
            new_variable_scope: EnvScope::User,
            show_add_dialog: false,
            show_settings: false,
            show_batch_operations: false,
            batch_operation_mode: false,
        }
    }
}

impl EnvManagerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        
        // 加载环境变量
        if let Err(e) = app.load_variables() {
            app.state.set_error_message(Some(format!("Failed to load environment variables: {}", e)));
        }
        
        app
    }

    fn load_variables(&mut self,
    ) -> Result<(), String> {
        self.variables = self.state.load_environment_variables()?;
        Ok(())
    }

    fn refresh_variables(&mut self,
    ) {
        if let Err(e) = self.load_variables() {
            self.state.set_error_message(Some(format!("Failed to refresh variables: {}", e)));
        } else {
            self.state.set_info_message(Some("Environment variables refreshed".to_string()));
        }
    }

    fn add_variable(&mut self) {
        if self.new_variable_name.is_empty() || self.new_variable_value.is_empty() {
            self.state.set_error_message(Some("Variable name and value cannot be empty".to_string()));
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
                self.state.set_info_message(Some("Variable added successfully".to_string()));
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
                self.state.set_info_message(Some("Variable updated successfully".to_string()));
            }
            Err(e) => {
                self.state.set_error_message(Some(e));
            }
        }
    }

    fn delete_variable(&mut self, name: &str) {
        match self.state.delete_variable(name) {
            Ok(_) => {
                self.selected_variable = None;
                self.refresh_variables();
                self.state.set_info_message(Some("Variable deleted successfully".to_string()));
            }
            Err(e) => {
                self.state.set_error_message(Some(e));
            }
        }
    }

    fn delete_selected_variables(&mut self) {
        let mut success_count = 0;
        let mut error_count = 0;
        let mut skipped_count = 0;
        let mut errors = Vec::new();
        
        for name in &self.selected_variables {
            // 检查变量是否为用户级变量
            if let Some(var) = self.variables.iter().find(|v| &v.name == name) {
                if var.scope == EnvScope::System {
                    skipped_count += 1;
                    continue; // 跳过系统级变量
                }
            }
            
            match self.state.delete_variable(name) {
                Ok(_) => success_count += 1,
                Err(e) => {
                    error_count += 1;
                    errors.push(format!("{}: {}", name, e));
                }
            }
        }
        
        self.selected_variables.clear();
        self.refresh_variables();
        
        let mut message_parts = Vec::new();
        if success_count > 0 {
            message_parts.push(format!("Successfully deleted {} variables", success_count));
        }
        if skipped_count > 0 {
            message_parts.push(format!("Skipped {} system variables (read-only)", skipped_count));
        }
        
        if error_count == 0 {
            self.state.set_info_message(Some(message_parts.join(", ")));
        } else {
            let error_msg = format!("{}, {} failed: {}", message_parts.join(", "), error_count, errors.join(", "));
            self.state.set_error_message(Some(error_msg));
        }
    }

    fn export_selected_variables(&mut self) {
        // 简单的导出功能，将选中的变量信息记录到日志
        let mut export_data = Vec::new();
        
        for name in &self.selected_variables {
            if let Some(var) = self.variables.iter().find(|v| &v.name == name) {
                export_data.push(format!("{}={}  # Scope: {:?}", var.name, var.value, var.scope));
            }
        }
        
        if !export_data.is_empty() {
            log::info!("Exported variables:\n{}", export_data.join("\n"));
            self.state.set_info_message(Some(format!("Exported {} variables to log", export_data.len())));
        }
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Windows Environment Variables Manager");
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚙ Settings").clicked() {
                    self.show_settings = true;
                }
                
                // 批量操作模式切换
                if ui.button(if self.batch_operation_mode { "📋 Exit Batch" } else { "📋 Batch Mode" }).clicked() {
                    self.batch_operation_mode = !self.batch_operation_mode;
                    if !self.batch_operation_mode {
                        self.selected_variables.clear();
                    }
                }
                
                // 根据模式显示不同的操作按钮
                if self.batch_operation_mode {
                    if ui.button("🗑 Delete Selected").clicked() && !self.selected_variables.is_empty() {
                        self.delete_selected_variables();
                    }
                    
                    if ui.button("📤 Export Selected").clicked() && !self.selected_variables.is_empty() {
                        self.export_selected_variables();
                    }
                    
                    ui.label(format!("Selected: {}", self.selected_variables.len()));
                } else {
                    // 检查选中的变量是否为用户级变量
                    let selected_var_scope = self.selected_variable.as_ref()
                        .and_then(|name| self.variables.iter().find(|v| &v.name == name))
                        .map(|v| &v.scope);
                    
                    let is_user_var = matches!(selected_var_scope, Some(EnvScope::User));
                    
                    // 删除按钮 - 只对用户级变量启用
                    ui.add_enabled_ui(is_user_var, |ui| {
                        if ui.button("🗑 Delete").clicked() {
                            if let Some(name) = self.selected_variable.clone() {
                                self.delete_variable(&name);
                            }
                        }
                    });
                    
                    // 编辑按钮 - 只对用户级变量启用
                    ui.add_enabled_ui(is_user_var, |ui| {
                        if ui.button("✏ Edit").clicked() {
                            if let Some(name) = &self.selected_variable {
                                self.editing_variable = Some(name.clone());
                            }
                        }
                    });
                }
                
                if ui.button("➕ Add").clicked() {
                    self.show_add_dialog = true;
                }
                
                if ui.button("🔄 Refresh").clicked() {
                    self.refresh_variables();
                }
            });
        });
    }

    fn render_search(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Search:");
            let mut search = self.state.search_query.lock().unwrap().clone();
            if ui.text_edit_singleline(&mut search).changed() {
                self.state.set_search_query(search);
            }
            
            ui.label("Filter:");
            let mut selected_scope = self.state.selected_scope.lock().unwrap().clone();
            
            let mut scope_str = match &selected_scope {
                Some(EnvScope::User) => "User",
                Some(EnvScope::System) => "System",
                None => "User", // 默认显示User
            }.to_string();
            
            egui::ComboBox::from_label("")
                .selected_text(scope_str.clone())
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut scope_str, "User".to_string(), "User").clicked() {
                        selected_scope = Some(EnvScope::User);
                    }
                  
                    if ui.selectable_value(&mut scope_str, "System".to_string(), "System").clicked() {
                        selected_scope = Some(EnvScope::System);
                    }
                });
            
            if selected_scope != *self.state.selected_scope.lock().unwrap() {
                self.state.set_selected_scope(selected_scope);
            }
        });
    }

    fn render_variables_list(&mut self, ui: &mut egui::Ui) {
        let filtered_vars = self.state.filter_variables(&self.variables
        );

        egui::ScrollArea::vertical().show(ui, |ui| {
            let num_columns = if self.batch_operation_mode { 4 } else { 3 };
            
            egui::Grid::new("variables_grid")
                .striped(true)
                .num_columns(num_columns)
                .show(ui, |ui| {
                    // 表头
                    if self.batch_operation_mode {
                        ui.label("Select");
                    }
                    ui.label("Name");
                    ui.label("Value");
                    ui.label("Scope");
                    ui.end_row();

                    for var in filtered_vars {
                        // 批量选择模式下的复选框
                        if self.batch_operation_mode {
                            let mut is_batch_selected = self.selected_variables.contains(&var.name);
                            if ui.checkbox(&mut is_batch_selected, "").changed() {
                                if is_batch_selected {
                                    if !self.selected_variables.contains(&var.name) {
                                        self.selected_variables.push(var.name.clone());
                                    }
                                } else {
                                    self.selected_variables.retain(|x| x != &var.name);
                                }
                            }
                        }
                        
                        // 变量名（单选模式下可选择）
                        if self.batch_operation_mode {
                            ui.label(&var.name);
                        } else {
                            let is_selected = self.selected_variable.as_ref() == Some(&var.name);
                            if ui.selectable_label(is_selected, &var.name).clicked() {
                                self.selected_variable = Some(var.name.clone());
                            }
                        }
                        
                        // 变量值（编辑模式 - 系统变量只读）
                        let mut value = var.value.clone();
                        if self.editing_variable.as_ref() == Some(&var.name) && var.scope == EnvScope::User {
                            ui.text_edit_multiline(&mut value);
                            
                            ui.horizontal(|ui| {
                                if ui.button("Save").clicked() {
                                    self.update_variable(&var.name, value);
                                }
                                if ui.button("Cancel").clicked() {
                                    self.editing_variable = None;
                                }
                            });
                        } else {
                            if var.scope == EnvScope::System {
                                ui.colored_label(egui::Color32::GRAY, &var.value); // 系统变量显示为灰色
                            } else {
                                ui.label(&var.value);
                            }
                        }
                        
                        ui.label(match var.scope {
                            EnvScope::User => "User",
                            EnvScope::System => "System",
                        });
                        ui.end_row();
                    }
                });
        });
    }

    fn render_add_dialog(&mut self, ctx: &egui::Context) {
        let has_admin_permission = self.state.check_admin_permission(self.new_variable_scope.clone());
        let mut add_clicked = false;
        let mut cancel_clicked = false;
        
        egui::Window::new("Add New Variable")
            .open(&mut self.show_add_dialog)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.new_variable_name);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Value:");
                        ui.text_edit_multiline(&mut self.new_variable_value);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Scope:");
                        ui.radio_value(&mut self.new_variable_scope, EnvScope::User, "User");
                        ui.radio_value(&mut self.new_variable_scope, EnvScope::System, "System");
                    });
                    
                    if !has_admin_permission {
                        ui.colored_label(egui::Color32::RED, "⚠ Admin privileges required for system variables");
                    }
                    
                    ui.horizontal(|ui| {
                        if ui.button("Add").clicked() {
                            add_clicked = true;
                        }
                        if ui.button("Cancel").clicked() {
                            cancel_clicked = true;
                        }
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

    fn render_settings(&mut self, ctx: &egui::Context) {
        let mut save_clicked = false;
        let mut cancel_clicked = false;
        let mut config_to_save = None;
        let mut refresh_clicked = false;
        
        egui::Window::new("Settings")
            .open(&mut self.show_settings)
            .show(ctx, |ui| {
                let mut config = self.state.get_config();
                
                ui.vertical(|ui| {
                    ui.heading("General Settings");
                    
                    let old_auto_refresh = config.auto_refresh;
                    ui.checkbox(&mut config.auto_refresh, "Auto refresh after changes");
                    
                    // 如果自动刷新设置改变，立即应用
                    if old_auto_refresh != config.auto_refresh {
                        self.state.set_auto_refresh(config.auto_refresh);
                    }
                    
                    ui.checkbox(&mut config.confirm_deletion, "Confirm before deletion");
                    ui.checkbox(&mut config.backup_enabled, "Enable automatic backups");
                    
                    ui.add(egui::Slider::new(&mut config.backup_interval_days, 1..=30
                    ).text("Backup interval (days)"));
                    
                    ui.separator();
                    
                    ui.heading("Environment Control");
                    
                    if ui.button("🔄 Manual Refresh Environment").clicked() {
                        refresh_clicked = true;
                    }
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("Log level:");
                        egui::ComboBox::from_label("")
                            .selected_text(&config.log_level)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config.log_level, "error".to_string(), "Error");
                                ui.selectable_value(&mut config.log_level, "warn".to_string(), "Warn");
                                ui.selectable_value(&mut config.log_level, "info".to_string(), "Info");
                                ui.selectable_value(&mut config.log_level, "debug".to_string(), "Debug");
                                ui.selectable_value(&mut config.log_level, "trace".to_string(), "Trace");
                            });
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Theme:");
                        egui::ComboBox::from_label("")
                            .selected_text(&config.theme)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config.theme, "light".to_string(), "Light");
                                ui.selectable_value(&mut config.theme, "dark".to_string(), "Dark");
                                ui.selectable_value(&mut config.theme, "high-contrast".to_string(), "High Contrast");
                            });
                    });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("Save Settings").clicked() {
                            save_clicked = true;
                            config_to_save = Some(config.clone());
                        }
                        
                        if ui.button("Cancel").clicked() {
                            cancel_clicked = true;
                        }
                    });
                });
            });
            
        if save_clicked {
            if let Some(config) = config_to_save {
                self.state.update_config(config);
                if let Err(e) = self.state.save_config() {
                    self.state.set_error_message(Some(format!("Failed to save settings: {}", e)));
                } else {
                    self.state.set_info_message(Some("Settings saved successfully".to_string()));
                    self.show_settings = false;
                }
            }
        }
        
        if refresh_clicked {
            match self.state.refresh_environment() {
                Ok(_) => {
                    self.state.set_info_message(Some("Environment refreshed successfully".to_string()));
                }
                Err(e) => {
                    self.state.set_error_message(Some(format!("Failed to refresh environment: {}", e)));
                }
            }
        }
        
        if cancel_clicked {
            self.show_settings = false;
        }
    }

    fn render_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let auto_refresh = self.state.get_auto_refresh();
            ui.label(format!("Variables: {}", self.variables.len()));
            ui.label(format!("Auto refresh: {}", if auto_refresh { "ON" } else { "OFF" }));
            
            if let Some(error) = self.state.get_error_message() {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                if ui.button("Clear").clicked() {
                    self.state.set_error_message(None);
                }
            }
            
            if let Some(info) = self.state.get_info_message() {
                ui.colored_label(egui::Color32::GREEN, format!("Info: {}", info));
                if ui.button("Clear").clicked() {
                    self.state.set_info_message(None);
                }
            }
        });
    }
}

impl eframe::App for EnvManagerApp {
    fn update(&mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
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
        self.render_settings(ctx);
    }

    fn save(&mut self,
        _storage: &mut dyn eframe::Storage,
    ) {
        let _ = self.state.save_config();
    }
}