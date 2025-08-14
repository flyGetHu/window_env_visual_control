use std::sync::{Arc, Mutex};

use crate::core::env_manager::EnvironmentManager;
use crate::models::env_variable::{EnvScope, EnvVariable};
use crate::utils::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env_manager: Arc<Mutex<EnvironmentManager>>,
    pub error_message: Arc<Mutex<Option<String>>>,
    pub info_message: Arc<Mutex<Option<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        let config = AppConfig::load();
        let env_manager = EnvironmentManager::new(config.auto_refresh);

        Self {
            env_manager: Arc::new(Mutex::new(env_manager)),
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

    pub fn add_variable(&self, name: String, value: String, scope: EnvScope) -> Result<(), String> {
        let env_manager = self.env_manager.lock().unwrap();
        env_manager
            .add_variable(scope, &name, &value)
            .map_err(|e| e.to_string())
    }

    pub fn update_variable(&self, name: &str, value: String) -> Result<(), String> {
        let mut env_manager = self.env_manager.lock().unwrap();
        env_manager
            .update_variable(name, value)
            .map_err(|e| e.to_string())
    }

    pub fn delete_variable(&self, name: &str, scope: EnvScope) -> Result<(), String> {
        let env_manager = self.env_manager.lock().unwrap();
        env_manager.delete_variable(scope, name).map_err(|e| e.to_string())
    }

    pub fn refresh_environment(&self) -> Result<(), String> {
        let env_manager = self.env_manager.lock().unwrap();
        env_manager.refresh_environment().map_err(|e| e.to_string())
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
}
