use std::sync::{Arc, Mutex};

use crate::core::env_manager::EnvironmentManager;
use crate::models::env_variable::{EnvScope, EnvVariable};
use crate::utils::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env_manager: Arc<Mutex<EnvironmentManager>>,
    pub config: Arc<Mutex<AppConfig>>,
    pub search_query: Arc<Mutex<String>>,
    pub selected_scope: Arc<Mutex<Option<EnvScope>>>,
    pub selected_variables: Arc<Mutex<Vec<String>>>,
    pub is_loading: Arc<Mutex<bool>>,
    pub error_message: Arc<Mutex<Option<String>>>,
    pub info_message: Arc<Mutex<Option<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        let config = AppConfig::load();
        let env_manager = EnvironmentManager::new(config.auto_refresh);

        Self {
            env_manager: Arc::new(Mutex::new(env_manager)),
            config: Arc::new(Mutex::new(config)),
            search_query: Arc::new(Mutex::new(String::new())),
            selected_scope: Arc::new(Mutex::new(Some(EnvScope::User))), // 默认选中User环境变量
            selected_variables: Arc::new(Mutex::new(Vec::new())),
            is_loading: Arc::new(Mutex::new(false)),
            error_message: Arc::new(Mutex::new(None)),
            info_message: Arc::new(Mutex::new(None)),
        }
    }

    pub fn load_environment_variables(&self) -> Result<Vec<EnvVariable>, String> {
        let env_manager = self.env_manager.lock().unwrap();
        env_manager
            .load_all_variables()
            .map(|vars| vars.iter().cloned().collect())
            .map_err(|e| e.to_string())
    }

    pub fn filter_variables(&self, variables: &[EnvVariable]) -> Vec<EnvVariable> {
        let search_query = self.search_query.lock().unwrap().clone();
        let selected_scope = self.selected_scope.lock().unwrap().clone();

        variables
            .iter()
            .filter(|var| {
                let matches_scope = selected_scope
                    .as_ref()
                    .map_or(true, |scope| var.scope == *scope);
                let matches_search = search_query.is_empty() || {
                    let query = search_query.to_lowercase();
                    var.name.to_lowercase().contains(&query)
                        || var.value.to_lowercase().contains(&query)
                        || var
                            .description
                            .as_ref()
                            .map_or(false, |desc| desc.to_lowercase().contains(&query))
                };
                matches_scope && matches_search
            })
            .cloned()
            .collect()
    }

    pub fn set_search_query(&self, query: String) {
        *self.search_query.lock().unwrap() = query;
    }

    pub fn set_selected_scope(&self, scope: Option<EnvScope>) {
        *self.selected_scope.lock().unwrap() = scope;
    }

    pub fn add_variable(&self, name: String, value: String, scope: EnvScope) -> Result<(), String> {
        let mut env_manager = self.env_manager.lock().unwrap();
        env_manager
            .add_variable(name, value, scope)
            .map_err(|e| e.to_string())
    }

    pub fn update_variable(&self, name: &str, value: String) -> Result<(), String> {
        let mut env_manager = self.env_manager.lock().unwrap();
        env_manager
            .update_variable(name, value)
            .map_err(|e| e.to_string())
    }

    pub fn delete_variable(&self, name: &str) -> Result<(), String> {
        let mut env_manager = self.env_manager.lock().unwrap();
        env_manager.delete_variable(name).map_err(|e| e.to_string())
    }

    pub fn refresh_environment(&self) -> Result<(), String> {
        let env_manager = self.env_manager.lock().unwrap();
        env_manager.refresh_environment().map_err(|e| e.to_string())
    }

    pub fn set_auto_refresh(&self, auto_refresh: bool) {
        let mut env_manager = self.env_manager.lock().unwrap();
        env_manager.set_auto_refresh(auto_refresh);

        let mut config = self.config.lock().unwrap();
        config.auto_refresh = auto_refresh;
    }

    pub fn get_auto_refresh(&self) -> bool {
        let env_manager = self.env_manager.lock().unwrap();
        env_manager.get_auto_refresh()
    }

    pub fn check_admin_permission(&self, scope: EnvScope) -> bool {
        let env_manager = self.env_manager.lock().unwrap();
        env_manager.check_admin_permission(scope)
    }

    pub fn set_error_message(&self, message: Option<String>) {
        *self.error_message.lock().unwrap() = message;
    }

    pub fn get_error_message(&self) -> Option<String> {
        self.error_message.lock().unwrap().clone()
    }

    pub fn set_info_message(&self, message: Option<String>) {
        *self.info_message.lock().unwrap() = message;
    }

    pub fn get_info_message(&self) -> Option<String> {
        self.info_message.lock().unwrap().clone()
    }

    pub fn save_config(&self) -> Result<(), String> {
        let config = self.config.lock().unwrap().clone();
        config.save().map_err(|e| e.to_string())
    }

    pub fn get_config(&self) -> AppConfig {
        self.config.lock().unwrap().clone()
    }

    pub fn update_config(&self, new_config: AppConfig) {
        *self.config.lock().unwrap() = new_config;
    }
}
