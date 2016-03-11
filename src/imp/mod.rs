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

pub use self::platform::get_app_dir;
// TODO function to create dir (by calling create_dir_all)
// TODO helper functions (optionally using AppInfo) to read/write from actual files