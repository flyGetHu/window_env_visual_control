use std::sync::{Arc, Mutex};

use crate::core::registry::RegistryManager;
use crate::core::refresh::EnvironmentRefresher;
use crate::models::env_variable::{EnvVariable, EnvScope, EnvVariables};
use crate::models::error::{EnvError, EnvResult};
use crate::models::profile::EnvProfiles;

#[derive(Debug)]
pub struct EnvironmentManager {
    registry_manager: RegistryManager,
    refresher: EnvironmentRefresher,
    cache: Arc<Mutex<EnvVariables>>,
    auto_refresh: bool,
}

impl EnvironmentManager {
    pub fn new(auto_refresh: bool) -> Self {
        Self {
            registry_manager: RegistryManager::new(),
            refresher: EnvironmentRefresher::new(),
            cache: Arc::new(Mutex::new(EnvVariables::new())),
            auto_refresh,
        }
    }

    /// 加载所有环境变量
    pub fn load_all_variables(&self,
    ) -> EnvResult<EnvVariables> {
        let mut variables = EnvVariables::new();

        // 加载用户级变量
        let user_vars = self.registry_manager.get_user_env_vars()?;
        for (name, value) in user_vars {
            let variable = EnvVariable::new(name, value, EnvScope::User);
            variables.add(variable);
        }

        // 加载系统级变量
        let system_vars = self.registry_manager.get_system_env_vars()?;
        for (name, value) in system_vars {
            let variable = EnvVariable::new(name, value, EnvScope::System);
            variables.add(variable);
        }

        // 更新缓存
        {
            let mut cache = self.cache.lock().unwrap();
            *cache = variables.clone();
        }

        Ok(variables)
    }

    /// 添加环境变量
    pub fn add_variable(&self, scope: EnvScope, name: &str, value: &str) -> EnvResult<()> {
        match scope {
            EnvScope::User => self.registry_manager.set_user_env_var(name, value)?,
            EnvScope::System => self.registry_manager.set_system_env_var(name, value)?,
        }
        self.refresher.refresh_environment()?;
        Ok(())
    }

    /// 更新环境变量
    pub fn update_variable(
        &mut self,
        name: &str,
        new_value: String,
    ) -> EnvResult<()> {
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(variable) = cache.get_mut(name) {
            let scope = variable.scope.clone();
            drop(cache);

            // 严格验证：只允许修改用户变量
            if scope == EnvScope::System {
                return Err(EnvError::PermissionDenied(
                    "Cannot modify system variables for safety reasons".to_string()
                ));
            }

            // 设置注册表
            match scope {
                EnvScope::User => self.registry_manager.set_user_env_var(name, &new_value)?,
                EnvScope::System => self.registry_manager.set_system_env_var(name, &new_value)?,
            }

            // 更新缓存
            {
                let mut cache = self.cache.lock().unwrap();
                if let Some(variable) = cache.get_mut(name) {
                    variable.update_value(new_value.clone());
                }
            }

            // 刷新环境
            if self.auto_refresh {
                self.refresher.refresh_environment()?;
            }

            log::info!("Updated environment variable: {}={}", name, new_value);
            Ok(())
        } else {
            Err(EnvError::VariableNotFound(name.to_string()))
        }
    }

    /// 删除环境变量
    pub fn delete_variable(&self, scope: EnvScope, name: &str) -> EnvResult<()> {
        match scope {
            EnvScope::User => self.registry_manager.delete_user_env_var(name)?,
            EnvScope::System => self.registry_manager.delete_system_env_var(name)?,
        }
        self.refresher.refresh_environment()?;
        Ok(())
    }

    /// 手动刷新环境变量
    pub fn refresh_environment(&self,
    ) -> EnvResult<()> {
        self.refresher.refresh_environment()
    }
}