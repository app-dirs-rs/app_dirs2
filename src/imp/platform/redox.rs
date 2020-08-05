use crate::AppDataType::*;
use crate::common::*;
use std::env::home_dir;
use std::path::{Component, PathBuf};

pub const USE_AUTHOR: bool = false;

pub fn get_app_dirs(t: AppDataType) -> Result<Vec<PathBuf>, AppDirsError> {
    let dir_base: Result<PathBuf, AppDirsError> = if t.is_shared() {
        Ok(Component::RootDir.as_ref().into())
    } else {
        home_dir().ok_or_else(|| AppDirsError::NotSupported)
    };
    dir_base.map(|mut path| {
        match t {
            UserConfig => {
                path.push(".config");
            },
            UserCache => {
                path.push(".cache");
            },
            UserData => {
                path.push(".share");
            },
            SharedConfig => {
                path.push("etc");
            },
            SharedData => {
                path.push("share");
            }
        };
        vec![path]
    })
}
