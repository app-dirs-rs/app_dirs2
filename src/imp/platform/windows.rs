//! Windows provides three different ways to get the paths to roaming and local
//! app data: environment variables, KNOWNFOLDERID, and CSIDL. From the CSIDL
//! documentation:
//!
//! *"These values supersede the use of environment variables for this purpose.
//! They are in turn superseded in Windows Vista and later by the KNOWNFOLDERID
//! values."*
//! - https://msdn.microsoft.com/en-us/library/windows/desktop/bb762494.aspx
//!
//! -_-

// The function get_folder_path was adapted from:
// https://github.com/AndyBarron/preferences-rs/blob/f03c7/src/lib.rs#L211-L296
//
// Credit for the above code goes to Connorcpu (https://github.com/Connorcpu).

use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::UI::Shell::*;
use windows::core::{ PWSTR, GUID };
use crate::common::*;
use crate::AppDataType::*;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

pub const USE_AUTHOR: bool = true;

pub fn get_app_dir(t: AppDataType) -> Result<PathBuf, AppDirsError> {
    let folder_id = match t {
        UserConfig => &FOLDERID_RoamingAppData,
        SharedConfig | SharedData => &FOLDERID_ProgramData,
        UserCache | UserData => &FOLDERID_LocalAppData,
    };
    get_folder_path(folder_id).map(|os_str| os_str.into())
}

/// Wrapper around `windows::core::PWSTR` to automatically free the string pointer.
/// This ensures the memory is freed when `get_folder_path` scope is left,
/// regardless of whether the call succeeded or failed/panicked.
struct SafePwstr(PWSTR);
impl Drop for SafePwstr {
    fn drop(&mut self) {
        unsafe { CoTaskMemFree(Some(self.0.as_ptr() as *mut _)) }
    }
}

fn get_folder_path(folder_id: &GUID) -> Result<OsString, AppDirsError> {
    unsafe {
        // SHGetKnownFolderPath arguments:
        // 1. reference to KNOWNFOLDERID
        // 2. no flags
        // 3. `None` handle -> current user
        //
        // Returns a PWSTR, which contains the path to requested folder.
        match SHGetKnownFolderPath(folder_id, KNOWN_FOLDER_FLAG(0), None) {
            Ok(raw_path) => {
                // Ensures that the PWSTR is free when we leave this scope through
                // normal execution or a thread panic.
                let _cleanup = SafePwstr(raw_path);
                Ok(OsStringExt::from_wide(raw_path.as_wide()))
                // _cleanup is deallocated, so raw_path is freed
            },
            Err(_) => {
                Err(AppDirsError::NotSupported)
            }
        }
    }
}
