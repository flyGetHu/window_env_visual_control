use eframe::egui::{Ui, Window};
use crate::app::theme::ModernTheme;
use crate::models::env_variable::EnvScope;

pub struct AddVariableDialog<'a> {
    pub show: &'a mut bool,
    pub name: &'a mut String,
    pub value: &'a mut String,
    pub scope: &'a mut EnvScope,
    pub theme: &'a ModernTheme,
}

impl<'a> AddVariableDialog<'a> {
    pub fn new(
        show: &'a mut bool,
        name: &'a mut String,
        value: &'a mut String,
        scope: &'a mut EnvScope,
        theme: &'a ModernTheme,
    ) -> Self {
        Self {
            show,
            name,
            value,
            scope,
            theme,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> bool {
        if !*self.show {
            return false;
        }

        let mut open = true;
        let mut clicked_add = false;
        Window::new("添加环境变量")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("变量名:");
                        ui.text_edit_singleline(self.name);
                    });
                    
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("变量值:");
                        ui.text_edit_singleline(self.value);
                    });
                    
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("作用域:");
                        ui.radio_value(self.scope, EnvScope::User, "用户");
                        ui.radio_value(self.scope, EnvScope::System, "系统");
                    });
                    
                    ui.add_space(16.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("取消").clicked() {
                            return;
                        }
                        
                        if ui.button("添加").clicked() {
                            clicked_add = true;
                        }
                    });
                });
            });
        
        *self.show = open;
        clicked_add
    }
}

pub struct DeleteConfirmDialog<'a> {
    pub show: &'a mut bool,
    pub variable_name: &'a str,
    pub theme: &'a ModernTheme,
}

impl<'a> DeleteConfirmDialog<'a> {
    pub fn new(
        show: &'a mut bool,
        variable_name: &'a str,
        theme: &'a ModernTheme,
    ) -> Self {
        Self {
            show,
            variable_name,
            theme,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> bool {
        if !*self.show {
            return false;
        }

        let mut open = true;
        let mut clicked_confirm = false;
        Window::new("确认删除")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    ui.add_space(8.0);
                    
                    ui.label(format!("确定要删除变量 '{}' 吗？", self.variable_name));
                    
                    ui.add_space(16.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("取消").clicked() {
                            return;
                        }
                        
                        if ui.button("删除").clicked() {
                            clicked_confirm = true;
                        }
                    });
                });
            });
        
        *self.show = open;
        clicked_confirm
    }
}