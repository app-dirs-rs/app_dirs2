#![cfg(target_os = "android")]

use app_dirs2::*;
use log::info;

const NDK_APP_INFO: AppInfo = AppInfo {
    name: "unnecessary",
    author: "The Android NDK Authors",
};

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "full", logger(level = "info", tag = "app_dirs2"))
)]
fn main() {
    let all = [
        AppDataType::UserConfig,
        AppDataType::UserData,
        AppDataType::UserCache,
        AppDataType::SharedConfig,
        AppDataType::SharedData,
    ];
    for t in all {
        info!("{:?}: {:?}", t, app_root(t, &NDK_APP_INFO));
    }
}
