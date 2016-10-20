use common::*;
use AppDataType::*;
use std::env::home_dir;
use std::path::{Component, PathBuf};

pub fn get_app_dir(t: AppDataType) -> AppDirsResult<PathBuf> {
    let dir_base: AppDirsResult<PathBuf> = if t.is_shared() {
        Ok(Component::RootDir.as_ref().into())
    } else {
        home_dir().ok_or_else(|| AppDirsError::NotSupported)
    };
    dir_base.map(|mut path| {
        match t {
            UserConfig | UserData | SharedConfig | SharedData => {
                path.push("Library");
                path.push("Application Support");
            }
            UserCache => {
                path.push("Library");
                path.push("Caches");
            }
        };
        path
    })
}
