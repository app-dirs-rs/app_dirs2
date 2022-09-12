extern crate xdg;
use self::xdg::BaseDirectories as Xdg;
use crate::common::*;
use crate::AppDataType::*;
use std::path::PathBuf;

// On Android we build this module to try XDG environment variables (#33), but
// this constant is unused and triggers a compiler warning.
#[cfg(not(target_os = "android"))]
pub const USE_AUTHOR: bool = false;

pub fn get_app_dir(t: AppDataType) -> Result<PathBuf, AppDirsError> {
    Xdg::new()
        .ok()
        .as_ref()
        .and_then(|x| match t {
            UserConfig => Some(x.get_config_home()),
            UserData => Some(x.get_data_home()),
            UserCache => Some(x.get_cache_home()),
            SharedData => x.get_data_dirs().into_iter().next(),
            SharedConfig => x.get_config_dirs().into_iter().next(),
        })
        .ok_or(AppDirsError::NotSupported)
}
