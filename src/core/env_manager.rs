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
    profiles: Arc<Mutex<EnvProfiles>>,
    auto_refresh: bool,
}

impl EnvironmentManager {
    pub fn new(auto_refresh: bool) -> Self {
        Self {
            registry_manager: RegistryManager::new(),
            refresher: EnvironmentRefresher::new(),
            cache: Arc::new(Mutex::new(EnvVariables::new())),
            profiles: Arc::new(Mutex::new(EnvProfiles::new())),
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

    /// 获取所有环境变量
    pub fn get_all_variables(&self,
    ) -> EnvResult<EnvVariables> {
        let cache = self.cache.lock().unwrap();
        if cache.is_empty() {
            drop(cache);
            self.load_all_variables()
        } else {
            Ok(cache.clone())
        }
    }

    /// 按作用域获取环境变量
    pub fn get_variables_by_scope(
        &self,
        scope: EnvScope,
    ) -> EnvResult<Vec<EnvVariable>> {
        let variables = self.get_all_variables()?;
        Ok(variables.filter_by_scope(scope).into_iter().cloned().collect())
    }

    /// 搜索环境变量
    pub fn search_variables(
        &self,
        query: &str,
    ) -> EnvResult<Vec<EnvVariable>> {
        let variables = self.get_all_variables()?;
        Ok(variables.search(query).into_iter().cloned().collect())
    }

    /// 添加环境变量
    pub fn add_variable(
        &mut self,
        name: String,
        value: String,
        scope: EnvScope,
    ) -> EnvResult<()> {
        // 严格验证：只允许添加用户变量
        if scope == EnvScope::System {
            return Err(EnvError::PermissionDenied(
                "Cannot add system variables for safety reasons".to_string()
            ));
        }

        // 设置注册表
        match scope {
            EnvScope::User => self.registry_manager.set_user_env_var(&name, &value)?,
            EnvScope::System => self.registry_manager.set_system_env_var(&name, &value)?,
        }

        // 更新缓存
        let variable = EnvVariable::new(name.clone(), value.clone(), scope.clone());
        {
            let mut cache = self.cache.lock().unwrap();
            cache.add(variable);
        }

        // 刷新环境
        if self.auto_refresh {
            self.refresher.refresh_scope(scope)?;
        }

        log::info!("Added environment variable: {}={}", name, value);
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
                self.refresher.refresh_scope(scope)?;
            }

            log::info!("Updated environment variable: {}={}", name, new_value);
            Ok(())
        } else {
            Err(EnvError::VariableNotFound(name.to_string()))
        }
    }

    /// 删除环境变量
    pub fn delete_variable(
        &mut self,
        name: &str,
    ) -> EnvResult<()> {
        let cache = self.cache.lock().unwrap();
        
        if let Some(variable) = cache.get(name) {
            let scope = variable.scope.clone();
            drop(cache);

            // 严格验证：只允许删除用户变量
            if scope == EnvScope::System {
                return Err(EnvError::PermissionDenied(
                    "Cannot delete system variables for safety reasons".to_string()
                ));
            }

            // 删除注册表
            match scope {
                EnvScope::User => self.registry_manager.delete_user_env_var(name)?,
                EnvScope::System => self.registry_manager.delete_system_env_var(name)?,
            }

            // 更新缓存
            {
                let mut cache = self.cache.lock().unwrap();
                cache.remove(name);
            }

            // 刷新环境
            if self.auto_refresh {
                self.refresher.refresh_scope(scope)?;
            }

            log::info!("Deleted environment variable: {}", name);
            Ok(())
        } else {
            Err(EnvError::VariableNotFound(name.to_string()))
        }
    }

    /// 手动刷新环境变量
    pub fn refresh_environment(
        &self,
    ) -> EnvResult<()> {
        self.refresher.refresh_environment()
    }

    /// 异步刷新环境变量
    pub fn refresh_environment_async(&self,
    ) {
        self.refresher.refresh_environment_async();
    }

    /// 设置自动刷新模式
    pub fn set_auto_refresh(&mut self, auto_refresh: bool) {
        self.auto_refresh = auto_refresh;
        log::info!("Auto refresh mode set to: {}", auto_refresh);
    }

    /// 获取自动刷新模式
    pub fn get_auto_refresh(&self) -> bool {
        self.auto_refresh
    }

    /// 检查管理员权限
    pub fn check_admin_permission(&self,
        scope: EnvScope,
    ) -> bool {
        match scope {
            EnvScope::User => true,
            EnvScope::System => self.refresher.is_user_admin(),
        }
    }

    /// 加载配置文件
    pub fn load_profiles(
        &self,
        path: &str,
    ) -> EnvResult<()> {
        let profiles = EnvProfiles::load_from_file(path)?;
        let profiles_count = profiles.len();
        {
            let mut profiles_cache = self.profiles.lock().unwrap();
            *profiles_cache = profiles;
        }
        log::info!("Loaded {} profiles from {}", profiles_count, path);
        Ok(())
    }

    /// 保存配置文件
    pub fn save_profiles(
        &self,
        path: &str,
    ) -> EnvResult<()> {
        let profiles = self.profiles.lock().unwrap();
        profiles.save_to_file(path)?;
        log::info!("Saved {} profiles to {}", profiles.len(), path);
        Ok(())
    }

    /// 获取所有配置文件
    pub fn get_profiles(&self,
    ) -> EnvResult<EnvProfiles> {
        let profiles = self.profiles.lock().unwrap();
        Ok(profiles.clone())
    }

    /// 应用配置文件
    pub fn apply_profile(
        &mut self,
        profile_name: &str,
    ) -> EnvResult<()> {
        let (scope, variables) = {
            let profiles = self.profiles.lock().unwrap();
            
            if let Some(profile) = profiles.get(profile_name) {
                if !profile.enabled {
                    return Err(EnvError::ConfigurationError(
                        format!("Profile {} is disabled", profile_name)
                    ));
                }

                let scope = profile.scope.clone();
                let variables: Vec<(String, String)> = profile.iter_variables()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                
                (scope, variables)
            } else {
                return Err(EnvError::ProfileNotFound(profile_name.to_string()));
            }
        };

        // 检查权限
        if scope == EnvScope::System && !self.refresher.is_user_admin() {
            return Err(EnvError::PermissionDenied(
                "Admin privileges required for system profiles".to_string()
            ));
        }

        // 应用配置
        for (name, value) in variables {
            match scope {
                EnvScope::User => self.registry_manager.set_user_env_var(&name, &value)?,
                EnvScope::System => self.registry_manager.set_system_env_var(&name, &value)?,
            }
        }

        // 刷新环境
        if self.auto_refresh {
            self.refresher.refresh_scope(scope)?;
        }

        log::info!("Applied profile: {}", profile_name);
        Ok(())
    }

    /// 启用/禁用配置文件
    pub fn toggle_profile(
        &mut self,
        profile_name: &str,
        enabled: bool,
    ) -> EnvResult<()> {
        let mut profiles = self.profiles.lock().unwrap();
        
        if let Some(profile) = profiles.get_mut(profile_name) {
            profile.set_enabled(enabled);
            log::info!("Profile {} {}", profile_name, if enabled { "enabled" } else { "disabled" });
            Ok(())
        } else {
            Err(EnvError::ProfileNotFound(profile_name.to_string()))
        }
    }
}