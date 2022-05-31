use crate::common::*;
use crate::AppDataType::*;
use std::path::{Component, Path, PathBuf};

pub const USE_AUTHOR: bool = false;

#[allow(deprecated)] // it's fine on macOS
pub fn get_app_dir(t: AppDataType) -> Result<PathBuf, AppDirsError> {
    let dir_base: Result<PathBuf, AppDirsError> = if t.is_shared() {
        Ok(Path::new(&Component::RootDir).into())
    } else {
        std::env::home_dir().ok_or_else(|| AppDirsError::NotSupported)
    };
    dir_base.map(|mut path| {
        match t {
            UserConfig | UserData | SharedConfig | SharedData => {
                path.push("Library");
                path.push("Application Support");
            },
            UserCache => {
                path.push("Library");
                path.push("Caches");
            },
        };
        path
    })
}
