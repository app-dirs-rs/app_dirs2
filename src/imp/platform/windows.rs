use common::*;
use std::path::PathBuf;
use std::env;
use AppDataType::*;

pub const USE_AUTHOR: bool = true;

pub fn get_app_dir(t: AppDataType) -> Result<PathBuf, AppDirsError> {
    match t {
            UserConfig | UserData | SharedConfig | SharedData => env::var("APPDATA"),
            UserCache => env::var("LOCALAPPDATA"),
        }
        .and_then(|x| Ok(PathBuf::from(x)))
        .or_else(|_| Err(AppDirsError::NotSupported))
}
