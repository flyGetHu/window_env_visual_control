use winapi::shared::minwindef::{DWORD, LPARAM, WPARAM};
use winapi::um::winuser::{HWND_BROADCAST, WM_SETTINGCHANGE, SMTO_ABORTIFHUNG, SendMessageTimeoutW};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use crate::models::error::{EnvError, EnvResult};

#[derive(Debug)]
pub struct EnvironmentRefresher;

impl EnvironmentRefresher {
    pub fn new() -> Self {
        Self
    }

    /// 广播环境变量更改消息
    pub fn refresh_environment(&self,
    ) -> EnvResult<()> {
        self.broadcast_setting_change("Environment")
    }

    /// 广播特定设置更改
    pub fn broadcast_setting_change(
        &self,
        section: &str,
    ) -> EnvResult<()> {
        let section_wide: Vec<u16> = OsStr::new(section)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let result = SendMessageTimeoutW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                0 as WPARAM,
                section_wide.as_ptr() as LPARAM,
                SMTO_ABORTIFHUNG,
                5000,
                std::ptr::null_mut(),
            );

            if result == 0 {
                return Err(EnvError::RefreshFailed(
                    "Failed to broadcast WM_SETTINGCHANGE message".to_string()
                ));
            }
        }

        log::info!("Successfully broadcast environment change notification");
        Ok(())
    }

    /// 异步刷新环境变量（避免阻塞UI）
    pub fn refresh_environment_async(&self,
    ) {
        let section = "Environment".to_string();
        std::thread::spawn(move || {
            if let Err(e) = EnvironmentRefresher::new().broadcast_setting_change(&section) {
                log::error!("Failed to refresh environment: {}", e);
            }
        });
    }

    /// 刷新特定作用域的环境变量
    pub fn refresh_scope(
        &self,
        scope: crate::models::env_variable::EnvScope,
    ) -> EnvResult<()> {
        log::info!("Refreshing environment variables for scope: {:?}", scope);
        self.refresh_environment()
    }

    /// 检查是否需要管理员权限
    pub fn check_admin_permission(&self,
        scope: crate::models::env_variable::EnvScope,
    ) -> bool {
        match scope {
            crate::models::env_variable::EnvScope::User => false,
            crate::models::env_variable::EnvScope::System => true,
        }
    }

    /// 获取当前用户是否是管理员
    pub fn is_user_admin(&self,
    ) -> bool {
        use winapi::um::securitybaseapi::GetTokenInformation;
        use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
        use winapi::um::winnt::{TOKEN_ELEVATION, TokenElevation};
        use winapi::um::handleapi::CloseHandle;

        unsafe {
            let mut token = std::ptr::null_mut();
            if OpenProcessToken(GetCurrentProcess(), 0x0008 /* TOKEN_QUERY */, &mut token) == 0 {
                return false;
            }

            let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
            let mut size = 0;
            let result = GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                std::mem::size_of::<TOKEN_ELEVATION>() as DWORD,
                &mut size,
            );

            CloseHandle(token);

            if result == 0 {
                return false;
            }

            elevation.TokenIsElevated != 0
        }
    }
}