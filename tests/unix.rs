#![cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]

use std::env;
use std::ffi;
use std::fs;
use std::path;
use std::sync;

use app_dirs2::{AppDataType, AppInfo};
use test_case::test_case;

const APP_DATA_TYPES: &[AppDataType] = &[
    AppDataType::UserCache,
    AppDataType::UserConfig,
    AppDataType::UserData,
    AppDataType::SharedConfig,
    AppDataType::SharedData,
];

const APP_INFO: AppInfo = AppInfo {
    name: "app-name",
    author: "app-author",
};

const SUBDIR: &'static str = "testdir";

lazy_static::lazy_static! {
    // For test cases that depend on environment variables
    static ref ENV_MUTEX: sync::Mutex<()> = sync::Mutex::new(());
}

fn set_xdg_env(ty: AppDataType, value: impl AsRef<ffi::OsStr>) {
    env::set_var(get_xdg_env_name(ty), value);
}

fn get_xdg_env_name(ty: AppDataType) -> ffi::OsString {
    match ty {
        AppDataType::UserCache => "XDG_CACHE_HOME",
        AppDataType::UserConfig => "XDG_CONFIG_HOME",
        AppDataType::UserData => "XDG_DATA_HOME",
        AppDataType::SharedConfig => "XDG_CONFIG_DIRS",
        AppDataType::SharedData => "XDG_DATA_DIRS",
    }.into()
}

fn reset_env() {
    env::set_var("HOME", "");
    for ty in APP_DATA_TYPES {
        set_xdg_env(*ty, "");
    }
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

    let app_root = app_dirs2::get_app_root(ty, &APP_INFO).unwrap();
    assert_eq!(data_root.join(APP_INFO.name), app_root);

    let app_dir = app_dirs2::get_app_dir(ty, &APP_INFO, SUBDIR).unwrap();
    assert_eq!(app_root.join(SUBDIR), app_dir);
}

#[test_case(AppDataType::UserCache; "user cache")]
#[test_case(AppDataType::UserConfig; "user config")]
#[test_case(AppDataType::UserData; "user data")]
#[test_case(AppDataType::SharedConfig; "shared config")]
#[test_case(AppDataType::SharedData; "shared data")]
fn test_xdg_dirs(ty: AppDataType) {
    let _env_guard = ENV_MUTEX.lock();

    let dir = tempfile::tempdir().unwrap();
    reset_env();
    set_xdg_env(ty, dir.path());

    let data_root = app_dirs2::get_data_root(ty).unwrap();
    assert_eq!(dir.path(), data_root.as_path());

    let app_root = app_dirs2::get_app_root(ty, &APP_INFO).unwrap();
    assert_eq!(data_root.join(APP_INFO.name), app_root);

    let app_dir = app_dirs2::get_app_dir(ty, &APP_INFO, SUBDIR).unwrap();
    assert_eq!(app_root.join(SUBDIR), app_dir);
}

#[test_case(AppDataType::UserCache; "user cache")]
#[test_case(AppDataType::UserConfig; "user config")]
#[test_case(AppDataType::UserData; "user data")]
#[test_case(AppDataType::SharedConfig; "shared config")]
#[test_case(AppDataType::SharedData; "shared data")]
fn test_home_and_xdg_dirs(ty: AppDataType) {
    let _env_guard = ENV_MUTEX.lock();

    let home_dir = tempfile::tempdir().unwrap();
    let xdg_dir = tempfile::tempdir().unwrap();
    reset_env();
    env::set_var("HOME", home_dir.path());
    set_xdg_env(ty, xdg_dir.path());

    let data_root = app_dirs2::get_data_root(ty).unwrap();
    assert_eq!(xdg_dir.path(), data_root.as_path());

    let app_root = app_dirs2::get_app_root(ty, &APP_INFO).unwrap();
    assert_eq!(data_root.join(APP_INFO.name), app_root);

    let app_dir = app_dirs2::get_app_dir(ty, &APP_INFO, SUBDIR).unwrap();
    assert_eq!(app_root.join(SUBDIR), app_dir);
}

#[test_case(AppDataType::UserCache; "user cache")]
#[test_case(AppDataType::UserConfig; "user config")]
#[test_case(AppDataType::UserData; "user data")]
fn test_multi_dir_user(ty: AppDataType) {
    let _env_guard = ENV_MUTEX.lock();

    let home_dir = tempfile::tempdir().unwrap();
    let xdg_dir = tempfile::tempdir().unwrap();
    reset_env();
    env::set_var("HOME", home_dir.path());
    set_xdg_env(ty, join_os_str(&[xdg_dir.path(), "test".as_ref()], ":"));

    let data_root = app_dirs2::get_data_root(ty).unwrap();
    let data_roots = app_dirs2::get_data_roots(ty).unwrap();
    assert_eq!(vec![data_root], data_roots);

    let app_root = app_dirs2::get_app_root(ty, &APP_INFO).unwrap();
    let app_roots = app_dirs2::get_app_roots(ty, &APP_INFO).unwrap();
    assert_eq!(vec![app_root], app_roots);

    let app_dir = app_dirs2::get_app_dir(ty, &APP_INFO, SUBDIR).unwrap();
    let app_dirs = app_dirs2::get_app_dirs(ty, &APP_INFO, SUBDIR).unwrap();
    assert_eq!(vec![app_dir], app_dirs);
}

#[test_case(AppDataType::SharedConfig; "shared config")]
#[test_case(AppDataType::SharedData; "shared data")]
fn test_multi_dir_missing(ty: AppDataType) {
    let _env_guard = ENV_MUTEX.lock();

    let root_dir = tempfile::tempdir().unwrap();
    let dir1 = root_dir.path().join("dir1");
    let dir2 = root_dir.path().join("dir2");
    reset_env();
    set_xdg_env(ty, join_os_str(&[&dir1, &dir2], ":"));

    let data_roots = app_dirs2::get_data_roots(ty).unwrap();
    assert_eq!(vec![dir1.clone(), dir2.clone()], data_roots);

    let app_roots = app_dirs2::get_app_roots(ty, &APP_INFO).unwrap();
    assert_eq!(data_roots.iter().map(|path| path.join(APP_INFO.name)).collect::<Vec<_>>(), app_roots);

    let app_dirs = app_dirs2::get_app_dirs(ty, &APP_INFO, SUBDIR).unwrap();
    assert_eq!(app_roots.iter().map(|path| path.join(SUBDIR)).collect::<Vec<_>>(), app_dirs);
}

#[test_case(AppDataType::SharedConfig; "shared config")]
#[test_case(AppDataType::SharedData; "shared data")]
fn test_multi_dir_existing_app_root(ty: AppDataType) {
    let _env_guard = ENV_MUTEX.lock();

    let root_dir = tempfile::tempdir().unwrap();
    let dir1 = root_dir.path().join("dir1");
    let dir2 = root_dir.path().join("dir2");
    fs::create_dir_all(&dir2.join(APP_INFO.name)).unwrap();
    reset_env();
    set_xdg_env(ty, join_os_str(&[&dir1, &dir2], ":"));

    let data_roots = app_dirs2::get_data_roots(ty).unwrap();
    assert_eq!(vec![dir2.clone()], data_roots);

    let app_roots = app_dirs2::get_app_roots(ty, &APP_INFO).unwrap();
    assert_eq!(vec![dir2.join(APP_INFO.name)], app_roots);

    let app_dirs = app_dirs2::get_app_dirs(ty, &APP_INFO, SUBDIR).unwrap();
    assert_eq!(vec![dir2.join(APP_INFO.name).join(SUBDIR)], app_dirs);
}

#[test_case(AppDataType::SharedConfig; "shared config")]
#[test_case(AppDataType::SharedData; "shared data")]
fn test_multi_dir_existing_app_dir(ty: AppDataType) {
    let _env_guard = ENV_MUTEX.lock();

    let root_dir = tempfile::tempdir().unwrap();
    let dir1 = root_dir.path().join("dir1");
    let dir2 = root_dir.path().join("dir2");
    fs::create_dir_all(&dir1.join(APP_INFO.name).join(SUBDIR)).unwrap();
    fs::create_dir_all(&dir2.join(APP_INFO.name)).unwrap();
    reset_env();
    set_xdg_env(ty, join_os_str(&[&dir1, &dir2], ":"));

    let data_roots = app_dirs2::get_data_roots(ty).unwrap();
    assert_eq!(vec![dir1.clone(), dir2.clone()], data_roots);

    let app_roots = app_dirs2::get_app_roots(ty, &APP_INFO).unwrap();
    assert_eq!(data_roots.iter().map(|path| path.join(APP_INFO.name)).collect::<Vec<_>>(), app_roots);

    let app_dirs = app_dirs2::get_app_dirs(ty, &APP_INFO, SUBDIR).unwrap();
    assert_eq!(vec![dir1.join(APP_INFO.name).join(SUBDIR)], app_dirs);
}

fn join_os_str(
    parts: &[impl AsRef<ffi::OsStr>],
    separator: impl AsRef<ffi::OsStr>,
) -> ffi::OsString {
    let mut s = ffi::OsString::new();
    if let Some(last) = parts.last() {
        for part in &parts[..parts.len() - 1] {
            s.push(part.as_ref());
            s.push(separator.as_ref());
        }
        s.push(last.as_ref());
    }
    s
}
