use winapi::shared::minwindef::{LPARAM, WPARAM};
use winapi::um::winuser::{HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE, SendMessageTimeoutW};
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
}