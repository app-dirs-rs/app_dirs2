extern crate xdg;
use self::xdg::BaseDirectories as Xdg;
use common::*;
use std::path::PathBuf;
use AppDataType::*;

pub fn get_app_dir(t: AppDataType) -> AppDirsResult<PathBuf> {
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
        .ok_or_else(|| AppDirsError::NotSupported)
}
