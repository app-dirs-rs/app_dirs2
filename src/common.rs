use std;

/// Data structure that holds information about your app.
///
/// This is used to pinpoint a specific location for your app's data on the
/// file system, relative to where app data must be stored. Therefore, the
/// attributes `name` and `author` MUST be valid directory names! It's HIGHLY
/// recommended that you only use letters, numbers, spaces, hyphens, and
/// underscores.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AppInfo {
    /// Filename-safe name of your app (e.g. "Hearthstone").
    pub name: String,
    /// Filename-safe author of your app (e.g. "Blizzard").
    pub author: String,
}

impl AppInfo {
    /// Convenience constructor to automatically convert e.g. static `&str`
    /// into `String`.
    pub fn new<A, B>(name: A, author: B) -> Self
        where A: Into<String>,
              B: Into<String>
    {
        AppInfo {
            name: name.into(),
            author: author.into(),
        }
    }
}

/// Enum specifying the type of app data you want to store.
///
/// Note that different platforms are not guaranteed or required
/// to provide different locations for different variants. For example,
/// Windows does not supported shared application data and does not
/// distinguish between config and data. Therefore, on Windows, all variants
/// except `UserCache` return the same path! Keep this in mind when choosing
/// data file names and paths.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppDataType {
    /// User-specific app configuration data.
    UserConfig,
    /// User-specific arbitrary app data.
    UserData,
    /// User-specific app cache data.
    UserCache,
    /// System-wide arbitrary app data.
    SharedData,
    /// System-wide app configuration data.
    SharedConfig,
}

impl AppDataType {
    /// Returns `true` for non-user-specific data types.
    pub fn is_shared(&self) -> bool {
        use AppDataType::*;
        match *self {
            SharedData | SharedConfig => true,
            _ => false,
        }
    }
}

/// Error type for any `app_dirs` operation.
#[derive(Debug)]
pub enum AppDirsError {
    /// An I/O error occurred during the operation.
    Io(std::io::Error),
    /// App-specific directories are not properly supported by the system
    /// (e.g. required environment variables don't exist).
    NotSupported,
    /// App info given to this library was invalid (e.g. app name or author
    /// were empty).
    InvalidAppData,
}

impl From<std::io::Error> for AppDirsError {
    fn from(e: std::io::Error) -> Self {
        AppDirsError::Io(e)
    }
}

/// Result type for any `app_dirs` operation.
pub type AppDirsResult<T> = Result<T, AppDirsError>;
