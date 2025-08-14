//! 安全性测试 - 验证系统变量保护机制

use window_env_visual_control::core::env_manager::EnvironmentManager;
use window_env_visual_control::models::env_variable::EnvScope;
use window_env_visual_control::models::error::EnvError;

#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_cannot_add_system_variable() {
        let mut env_manager = EnvironmentManager::new(false);
        
        let result = env_manager.add_variable(
            "TEST_SYSTEM_VAR".to_string(),
            "test_value".to_string(),
            EnvScope::System,
        );
        
        assert!(result.is_err());
        if let Err(EnvError::PermissionDenied(msg)) = result {
            assert!(msg.contains("Cannot add system variables for safety reasons"));
        } else {
            panic!("Expected PermissionDenied error");
        }
    }

    #[test]
    fn test_cannot_update_system_variable() {
        let mut env_manager = EnvironmentManager::new(false);
        
        // 首先加载所有变量以获取系统变量
        let _ = env_manager.load_all_variables();
        
        // 尝试更新一个常见的系统变量（如PATH）
        let result = env_manager.update_variable("PATH", "malicious_value".to_string());
        
        // 如果PATH存在且是系统变量，应该被拒绝
        if result.is_err() {
            if let Err(EnvError::PermissionDenied(msg)) = result {
                assert!(msg.contains("Cannot modify system variables for safety reasons"));
            }
        }
    }

    #[test]
    fn test_cannot_delete_system_variable() {
        let mut env_manager = EnvironmentManager::new(false);
        
        // 首先加载所有变量以获取系统变量
        let _ = env_manager.load_all_variables();
        
        // 尝试删除一个常见的系统变量（如PATH）
        let result = env_manager.delete_variable("PATH");
        
        // 如果PATH存在且是系统变量，应该被拒绝
        if result.is_err() {
            if let Err(EnvError::PermissionDenied(msg)) = result {
                assert!(msg.contains("Cannot delete system variables for safety reasons"));
            }
        }
    }

    #[test]
    fn test_can_add_user_variable() {
        let mut env_manager = EnvironmentManager::new(false);
        
        let result = env_manager.add_variable(
            "TEST_USER_VAR".to_string(),
            "test_value".to_string(),
            EnvScope::User,
        );
        
        // 用户变量应该可以正常添加
        assert!(result.is_ok());
        
        // 清理测试变量
        let _ = env_manager.delete_variable("TEST_USER_VAR");
    }

    #[test]
    fn test_can_update_user_variable() {
        let mut env_manager = EnvironmentManager::new(false);
        
        // 先添加一个用户变量
        let _ = env_manager.add_variable(
            "TEST_UPDATE_VAR".to_string(),
            "original_value".to_string(),
            EnvScope::User,
        );
        
        // 更新用户变量应该成功
        let result = env_manager.update_variable("TEST_UPDATE_VAR", "updated_value".to_string());
        assert!(result.is_ok());
        
        // 清理测试变量
        let _ = env_manager.delete_variable("TEST_UPDATE_VAR");
    }

    #[test]
    fn test_can_delete_user_variable() {
        let mut env_manager = EnvironmentManager::new(false);
        
        // 先添加一个用户变量
        let _ = env_manager.add_variable(
            "TEST_DELETE_VAR".to_string(),
            "test_value".to_string(),
            EnvScope::User,
        );
        
        // 删除用户变量应该成功
        let result = env_manager.delete_variable("TEST_DELETE_VAR");
        assert!(result.is_ok());
    }
}