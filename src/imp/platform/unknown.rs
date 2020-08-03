use crate::AppDataType::*;
use crate::common::*;
use std::path::PathBuf;

pub const USE_AUTHOR: bool = false;

pub fn get_app_dirs(_t: AppDataType) -> Result<Vec<PathBuf>, AppDirsError> {
    Err(AppDirsError::NotSupported)
}
