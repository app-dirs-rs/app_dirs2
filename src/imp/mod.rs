use common::{AppDirsError, AppDirsResult, AppDataType, AppInfo};
use std::fs;
use std::path::PathBuf;

#[cfg(target_os="macos")]
mod platform {
    mod macos;
    pub use self::macos::*;
}
#[cfg(all(unix, not(target_os="macos")))]
mod platform {
    mod unix;
    pub use self::unix::*;
}
#[cfg(windows)]
mod platform {
    mod windows;
    pub use self::windows::*;
}

/// Creates (if necessary) directory hierarchy for the data type and app info
/// provided.
///
/// A result of `Ok` guarantees that the directory located at the returned
/// `PathBuf` was created, including its full parent hierarchy if required.
///
/// Different `AppDataType` variants are NOT GUARANTEED to return different
/// directories (e.g. on Windows, everything except `UserCache` goes in
/// `%APPDATA%`).
pub fn create_app_dir(t: AppDataType, app: &AppInfo) -> AppDirsResult<PathBuf> {
    get_app_dir_path(t, app).and_then(|path| {
        match fs::create_dir_all(&path) {
            Ok(..) => Ok(path),
            Err(e) => Err(e.into()),
        }
    })
}

/// Gets directory path for the data type and app info provided.
///
/// A result of `Ok` DOES NOT necessarily mean that the directory actually
/// exists -- just that we were able to determine what the path SHOULD be.
///
/// Different `AppDataType` variants are NOT GUARANTEED to return different
/// directories (e.g. on Windows, everything except `UserCache` goes in
/// `%APPDATA%`).
pub fn get_app_dir_path(t: AppDataType, app: &AppInfo) -> AppDirsResult<PathBuf> {
    if app.author.len() == 0 || app.name.len() == 0 {
        return Err(AppDirsError::InvalidAppData);
    }
    get_app_data_root(t).map(|mut path| {
        path.push(app.author.clone());
        path.push(app.name.clone());
        path
    })
}

/// Gets path to root app data directory for the data type and app info
/// provided.
///
/// "Root" in this case means that this function will return the top-level
/// directory for all app data of type `t`. Generally, you should prefer to
/// call `get_app_dir_path` with an instance of `AppInfo`.
///
/// A result of `Ok` DOES NOT necessarily mean that the directory actually
/// exists -- just that we were able to determine what the path SHOULD be.
///
/// Different `AppDataType` variants are NOT GUARANTEED to return different
/// directories (e.g. on Windows, everything except `UserCache` goes in
/// `%APPDATA%`).
pub fn get_app_data_root(t: AppDataType) -> AppDirsResult<PathBuf> {
    platform::get_app_dir(t)
}
