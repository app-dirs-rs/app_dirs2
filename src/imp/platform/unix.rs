extern crate xdg;
use crate::AppDataType::*;
use crate::common::*;
use std::path::PathBuf;

pub const USE_AUTHOR: bool = false;

pub fn get_app_dirs(t: AppDataType) -> Result<Vec<PathBuf>, AppDirsError> {
    let base_directories = xdg::BaseDirectories::new()?;
    let paths = match t {
        UserConfig => vec![base_directories.get_config_home()],
        UserData => vec![base_directories.get_data_home()],
        UserCache => vec![base_directories.get_cache_home()],
        SharedData => base_directories.get_data_dirs(),
        SharedConfig => base_directories.get_config_dirs(),
    };
    if paths.is_empty() {
        Err(AppDirsError::NotSupported)
    } else {
        Ok(paths)
    }
}

impl From<xdg::BaseDirectoriesError> for AppDirsError {
    fn from(_: xdg::BaseDirectoriesError) -> AppDirsError {
        AppDirsError::NotSupported
    }
}
