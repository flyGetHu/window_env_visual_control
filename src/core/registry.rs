use std::collections::HashMap;
use std::ptr;

use winapi::shared::minwindef::{DWORD, HKEY, LPBYTE};
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winnt::{KEY_READ, KEY_WRITE, REG_EXPAND_SZ, REG_SZ};
use winapi::um::winreg::{
    RegCloseKey, RegDeleteValueW, RegEnumValueW, RegOpenKeyExW, RegSetValueExW, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE,
};

use crate::models::error::{EnvError, EnvResult};

#[derive(Debug)]
pub struct RegistryManager;

impl RegistryManager {
    pub fn new() -> Self {
        Self
    }

    /// 获取用户级环境变量
    pub fn get_user_env_vars(&self) -> EnvResult<HashMap<String, String>> {
        self.get_env_vars_from_registry(HKEY_CURRENT_USER)
    }

    /// 获取系统级环境变量
    pub fn get_system_env_vars(&self) -> EnvResult<HashMap<String, String>> {
        self.get_env_vars_from_registry(HKEY_LOCAL_MACHINE)
    }

    /// 设置用户级环境变量
    pub fn set_user_env_var(&self, name: &str, value: &str) -> EnvResult<()> {
        self.set_env_var_in_registry(HKEY_CURRENT_USER, name, value)
    }

    /// 设置系统级环境变量
    pub fn set_system_env_var(&self, name: &str, value: &str) -> EnvResult<()> {
        self.set_env_var_in_registry(HKEY_LOCAL_MACHINE, name, value)
    }

    /// 删除用户级环境变量
    pub fn delete_user_env_var(&self, name: &str) -> EnvResult<()> {
        self.delete_env_var_from_registry(HKEY_CURRENT_USER, name)
    }

    /// 删除系统级环境变量
    pub fn delete_system_env_var(&self, name: &str) -> EnvResult<()> {
        self.delete_env_var_from_registry(HKEY_LOCAL_MACHINE, name)
    }

    /// 从注册表获取环境变量
    fn get_env_vars_from_registry(&self,
        root_key: HKEY,
    ) -> EnvResult<HashMap<String, String>> {
        let mut env_vars = HashMap::new();
        let mut hkey: HKEY = ptr::null_mut();
        
        let env_path = if root_key == HKEY_CURRENT_USER {
            "Environment"
        } else {
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment"
        };

        let env_path_wide: Vec<u16> = env_path.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            let result = RegOpenKeyExW(
                root_key,
                env_path_wide.as_ptr(),
                0,
                KEY_READ,
                &mut hkey,
            );

            if result != ERROR_SUCCESS as i32 {
                return Err(EnvError::RegistryError(format!("Failed to open registry key: {}", result)));
            }

            let mut index = 0;
            let mut value_name = vec![0u16; 255];
            let mut value_name_len: DWORD;
            let mut value_data = vec![0u8; 8192];
            let mut value_data_len: DWORD;
            let mut value_type: DWORD = 0;

            loop {
                value_name_len = 255;
                value_data_len = 8192;

                let result = RegEnumValueW(
                    hkey,
                    index,
                    value_name.as_mut_ptr(),
                    &mut value_name_len,
                    ptr::null_mut(),
                    &mut value_type,
                    value_data.as_mut_ptr(),
                    &mut value_data_len,
                );

                if result == ERROR_SUCCESS as i32 {
                    let name = String::from_utf16(&value_name[..value_name_len as usize])?;
                    
                    match value_type {
                        REG_SZ | REG_EXPAND_SZ => {
                            let value_slice = std::slice::from_raw_parts(
                                value_data.as_ptr() as *const u16,
                                value_data_len as usize / 2
                            );
                            let value = String::from_utf16(value_slice)?;
                            env_vars.insert(name, value);
                        },
                        _ => {
                            log::warn!("Skipping non-string registry value: {}", name);
                        }
                    }

                    index += 1;
                } else {
                    break;
                }
            }

            RegCloseKey(hkey);
        }

        Ok(env_vars)
    }

    /// 设置注册表环境变量
    fn set_env_var_in_registry(
        &self,
        root_key: HKEY,
        name: &str,
        value: &str,
    ) -> EnvResult<()> {
        let mut hkey: HKEY = ptr::null_mut();
        
        let env_path = if root_key == HKEY_CURRENT_USER {
            "Environment"
        } else {
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment"
        };

        let env_path_wide: Vec<u16> = env_path.encode_utf16().chain(std::iter::once(0)).collect();
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let value_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            let result = RegOpenKeyExW(
                root_key,
                env_path_wide.as_ptr(),
                0,
                KEY_WRITE,
                &mut hkey,
            );

            if result != ERROR_SUCCESS as i32 {
                return Err(EnvError::RegistryError(format!("Failed to open registry key: {}", result)));
            }

            let result = RegSetValueExW(
                hkey,
                name_wide.as_ptr(),
                0,
                REG_SZ,
                value_wide.as_ptr() as LPBYTE,
                (value_wide.len() * 2) as DWORD,
            );

            RegCloseKey(hkey);

            if result != ERROR_SUCCESS as i32 {
                return Err(EnvError::RegistryError(format!("Failed to set registry value: {}", result)));
            }
        }

        Ok(())
    }

    /// 从注册表删除环境变量
    fn delete_env_var_from_registry(
        &self,
        root_key: HKEY,
        name: &str,
    ) -> EnvResult<()> {
        let mut hkey: HKEY = ptr::null_mut();
        
        let env_path = if root_key == HKEY_CURRENT_USER {
            "Environment"
        } else {
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment"
        };

        let env_path_wide: Vec<u16> = env_path.encode_utf16().chain(std::iter::once(0)).collect();
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            let result = RegOpenKeyExW(
                root_key,
                env_path_wide.as_ptr(),
                0,
                KEY_WRITE,
                &mut hkey,
            );

            if result != ERROR_SUCCESS as i32 {
                return Err(EnvError::RegistryError(format!("Failed to open registry key: {}", result)));
            }

            let result = RegDeleteValueW(hkey, name_wide.as_ptr());

            RegCloseKey(hkey);

            if result != ERROR_SUCCESS as i32 {
                return Err(EnvError::RegistryError(format!("Failed to delete registry value: {}", result)));
            }
        }

        Ok(())
    }
}