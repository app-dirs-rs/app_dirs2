#![cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]

use std::env;
use std::ffi;
use std::path;
use std::sync;

use app_dirs2::AppDataType;
use once_cell::sync::Lazy;
use test_case::test_case;

// For test cases that depend on environment variables
static ENV_MUTEX: Lazy<sync::Mutex<()>> = Lazy::new(|| sync::Mutex::new(()));

fn reset_env() {
    env::set_var("HOME", "");
    env::set_var("XDG_CACHE_HOME", "");
    env::set_var("XDG_CONFIG_HOME", "");
    env::set_var("XDG_DATA_HOME", "");
    env::set_var("XDG_DATA_DIRS", "");
    env::set_var("XDG_CONFIG_DIRS", "");
}

#[test_case(AppDataType::UserCache, ".cache"; "user cache")]
#[test_case(AppDataType::UserConfig, ".config"; "user config")]
#[test_case(AppDataType::UserData, ".local/share"; "user data")]
#[test_case(AppDataType::SharedConfig, "/etc/xdg"; "shared config")]
#[test_case(AppDataType::SharedData, "/usr/local/share"; "shared data")]
fn test_home(ty: AppDataType, path: impl AsRef<path::Path>) {
    let _env_guard = ENV_MUTEX.lock();

    let dir = tempfile::tempdir().unwrap();
    reset_env();
    env::set_var("HOME", dir.path());

    let data_root = app_dirs2::get_data_root(ty).unwrap();
    if ty.is_shared() {
        assert_eq!(path.as_ref(), data_root.as_path());
    } else {
        assert_eq!(dir.path().join(path.as_ref()), data_root);
    }

    let app_info = app_dirs2::AppInfo {
        name: "app-name",
        author: "app-author",
    };

    let app_root = app_dirs2::get_app_root(ty, &app_info).unwrap();
    assert_eq!(data_root.join(app_info.name), app_root);

    let subdir = "testdir";
    let app_dir = app_dirs2::get_app_dir(ty, &app_info, subdir).unwrap();
    assert_eq!(app_root.join(subdir), app_dir);
}

#[test_case(AppDataType::UserCache, "XDG_CACHE_HOME"; "user cache")]
#[test_case(AppDataType::UserConfig, "XDG_CONFIG_HOME"; "user config")]
#[test_case(AppDataType::UserData, "XDG_DATA_HOME"; "user data")]
#[test_case(AppDataType::SharedConfig, "XDG_CONFIG_DIRS"; "shared config")]
#[test_case(AppDataType::SharedData, "XDG_DATA_DIRS"; "shared data")]
fn test_xdg_dirs(ty: AppDataType, env_var: impl AsRef<ffi::OsStr>) {
    let _env_guard = ENV_MUTEX.lock();

    let dir = tempfile::tempdir().unwrap();
    reset_env();
    env::set_var(env_var.as_ref(), dir.path());

    let data_root = app_dirs2::get_data_root(ty).unwrap();
    assert_eq!(dir.path(), data_root.as_path());

    let app_info = app_dirs2::AppInfo {
        name: "app-name",
        author: "app-author",
    };

    let app_root = app_dirs2::get_app_root(ty, &app_info).unwrap();
    assert_eq!(data_root.join(app_info.name), app_root);

    let subdir = "testdir";
    let app_dir = app_dirs2::get_app_dir(ty, &app_info, subdir).unwrap();
    assert_eq!(app_root.join(subdir), app_dir);
}

#[test_case(AppDataType::UserCache, "XDG_CACHE_HOME"; "user cache")]
#[test_case(AppDataType::UserConfig, "XDG_CONFIG_HOME"; "user config")]
#[test_case(AppDataType::UserData, "XDG_DATA_HOME"; "user data")]
#[test_case(AppDataType::SharedConfig, "XDG_CONFIG_DIRS"; "shared config")]
#[test_case(AppDataType::SharedData, "XDG_DATA_DIRS"; "shared data")]
fn test_home_and_xdg_dirs(ty: AppDataType, env_var: impl AsRef<ffi::OsStr>) {
    let _env_guard = ENV_MUTEX.lock();

    let home_dir = tempfile::tempdir().unwrap();
    let xdg_dir = tempfile::tempdir().unwrap();
    reset_env();
    env::set_var("HOME", home_dir.path());
    env::set_var(env_var.as_ref(), xdg_dir.path());

    let data_root = app_dirs2::get_data_root(ty).unwrap();
    assert_eq!(xdg_dir.path(), data_root.as_path());

    let app_info = app_dirs2::AppInfo {
        name: "app-name",
        author: "app-author",
    };

    let app_root = app_dirs2::get_app_root(ty, &app_info).unwrap();
    assert_eq!(data_root.join(app_info.name), app_root);

    let subdir = "testdir";
    let app_dir = app_dirs2::get_app_dir(ty, &app_info, subdir).unwrap();
    assert_eq!(app_root.join(subdir), app_dir);
}
