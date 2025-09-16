#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "panic_handle")]
pub mod painc;

pub mod time;
pub use anyhow;
pub use chrono;
pub use serde;
pub use time::*;

pub type AppResult<T = ()> = std::result::Result<T, anyhow::Error>;

use std::time::Duration;
#[cfg(target_os = "linux")]
use tokio::process::Command;

// 程序退出
pub fn after_app_exist(sec: u8) {
    tokio::spawn(async move {
        log::info!("程序退出，{} 秒后重启", sec);
        tokio::time::sleep(Duration::from_secs(sec as u64)).await;
        std::process::exit(1);
    });
}

//重启系统
pub fn after_reboot(sec: u8) {
    #[cfg(target_os = "linux")]
    {
        tokio::spawn(async move {
            log::info!("程序重启，{} 秒后重启", sec);
            tokio::time::sleep(Duration::from_secs(sec as u64)).await;
            let status = Command::new("reboot").status().await.unwrap();
            if status.success() {
                log::info!("程序重启成功");
            } else {
                log::error!("程序重启失败");
            }
        });
    }
    #[cfg(target_os = "windows")]
    log::warn!("windows系统不支持自动重启:{}", sec)
}
