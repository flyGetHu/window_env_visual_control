use egui::Ui;
use crate::app::theme::ModernTheme;
use crate::app::components::card::{InteractiveCard, ModernCard};
use crate::models::env_variable::{EnvScope, EnvVariable};
use std::collections::HashSet;

pub struct VariableList<'a> {
    variables: &'a [EnvVariable],
    theme: &'a ModernTheme,
    search_query: &'a str,
    batch_mode: bool,
    selected_variables: &'a mut HashSet<String>,
    editing_variable: &'a mut Option<String>,
    hovered_variable: &'a mut Option<String>,
    expanded_variables: &'a mut HashSet<String>,
}

impl<'a> VariableList<'a> {
    pub fn new(
        variables: &'a [EnvVariable],
        theme: &'a ModernTheme,
        search_query: &'a str,
        batch_mode: bool,
        selected_variables: &'a mut HashSet<String>,
        editing_variable: &'a mut Option<String>,
        hovered_variable: &'a mut Option<String>,
        expanded_variables: &'a mut HashSet<String>,
    ) -> Self {
        Self {
            variables,
            theme,
            search_query,
            batch_mode,
            selected_variables,
            editing_variable,
            hovered_variable,
            expanded_variables,
        }
    }

    fn filter_variables(&self) -> Vec<EnvVariable> {
        self.variables
            .iter()
            .filter(|var| {
                self.search_query.is_empty()
                    || var.name.to_lowercase().contains(&self.search_query.to_lowercase())
                    || var.value.to_lowercase().contains(&self.search_query.to_lowercase())
            })
            .cloned()
            .collect()
    }

    fn render_variable_item(
        &mut self,
        ui: &mut Ui,
        variable: &EnvVariable,
    ) -> Option<String> {
        let is_system = matches!(variable.scope, EnvScope::System);
        let is_selected = self.selected_variables.contains(&variable.name);
        let is_hovered = self.hovered_variable.as_ref() == Some(&variable.name);
        let is_expanded = self.expanded_variables.contains(&variable.name);

        let card = InteractiveCard::new(self.theme, is_hovered, is_selected);
        
        let mut delete_name = None;
        let response = card.show(ui, |ui| {
            ui.horizontal(|ui| {
                if self.batch_mode && !is_system {
                    let mut selected = is_selected;
                    let checkbox_response = ui.checkbox(&mut selected, "");
                    if checkbox_response.clicked() {
                        if is_selected {
                            self.selected_variables.remove(&variable.name);
                        } else {
                            self.selected_variables.insert(variable.name.clone());
                        }
                    }
                }

                // 变量名
                let name_color = if is_system {
                    self.theme.system_variable_accent
                } else {
                    self.theme.user_variable_accent
                };
                
                ui.colored_label(name_color, &variable.name);
                ui.add_space(8.0);

                // 变量值
                let value_text = if variable.value.len() > 50 {
                    format!("{}...", &variable.value[..50])
                } else {
                    variable.value.clone()
                };
                
                ui.label(&value_text);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !is_system {
                        if ui.small_button("编辑").clicked() {
                            *self.editing_variable = Some(variable.name.clone());
                        }
                        
                        if ui.small_button("删除").clicked() {
                            delete_name = Some(variable.name.clone());
                        }
                    }
                });
            });

            // 展开详情
            if ui.small_button(if is_expanded { "收起" } else { "展开" }).clicked() {
                if is_expanded {
                    self.expanded_variables.remove(&variable.name);
                } else {
                    self.expanded_variables.insert(variable.name.clone());
                }
            }

            if is_expanded {
                ui.add_space(8.0);
                ModernCard::new(self.theme)
                    .with_margin(8.0)
                    .show(ui, |ui| {
                        ui.label(format!("完整值: {}", variable.value));
                        ui.label(format!("作用域: {:?}", variable.scope));
                        ui.label(format!("创建时间: {:?}", variable.created_at));
                        ui.label(format!("更新时间: {:?}", variable.updated_at));
                    });
            }
        });

        if response.response.hovered() {
            *self.hovered_variable = Some(variable.name.clone());
        }

        delete_name
    }

    pub fn show(&mut self, ui: &mut Ui) -> Vec<String> {
        let filtered_vars = self.filter_variables();
        let mut delete_requests = Vec::new();

        if filtered_vars.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("没有找到匹配的环境变量");
            });
            return delete_requests;
        }

        for variable in filtered_vars {
            if let Some(name) = self.render_variable_item(ui, &variable) {
                delete_requests.push(name);
            }
            ui.add_space(4.0);
        }

        delete_requests
    }
}

pub struct VariableEditor<'a> {
    variable: &'a EnvVariable,
    new_value: &'a mut String,
    theme: &'a ModernTheme,
}

impl<'a> VariableEditor<'a> {
    pub fn new(variable: &'a EnvVariable, new_value: &'a mut String, theme: &'a ModernTheme) -> Self {
        Self {
            variable,
            new_value,
            theme,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> bool {
        let mut saved = false;
        
        ModernCard::new(self.theme).show(ui, |ui| {
            ui.heading(format!("编辑: {}", self.variable.name));
            ui.add_space(8.0);
            
            ui.horizontal(|ui| {
                ui.label("变量值:");
                ui.text_edit_singleline(self.new_value);
            });
            
            ui.add_space(16.0);
            
            ui.horizontal(|ui| {
                if ui.button("保存").clicked() {
                    saved = true;
                }
                
                if ui.button("取消").clicked() {
                    saved = false;
                }
            });
        });
        
        saved
    }
}