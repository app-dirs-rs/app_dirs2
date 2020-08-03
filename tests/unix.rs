#![cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]

use std::env;
use std::ffi;
use std::fs;
use std::path;
use std::sync;

use app_dirs2::AppDataType;
use test_case::test_case;

lazy_static::lazy_static! {
    // For test cases that depend on environment variables
    static ref ENV_MUTEX: sync::Mutex<()> = sync::Mutex::new(());
}

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

#[test_case(AppDataType::UserCache, "XDG_CACHE_HOME"; "user cache")]
#[test_case(AppDataType::UserConfig, "XDG_CONFIG_HOME"; "user config")]
#[test_case(AppDataType::UserData, "XDG_DATA_HOME"; "user data")]
fn test_multi_dir_user(ty: AppDataType, env_var: impl AsRef<ffi::OsStr>) {
    let _env_guard = ENV_MUTEX.lock();

    let home_dir = tempfile::tempdir().unwrap();
    let xdg_dir = tempfile::tempdir().unwrap();
    reset_env();
    env::set_var("HOME", home_dir.path());
    env::set_var(env_var.as_ref(), join_os_str(&[xdg_dir.path(), "test".as_ref()], ":"));

    let data_root = app_dirs2::get_data_root(ty).unwrap();
    let data_roots = app_dirs2::get_data_roots(ty).unwrap();
    assert_eq!(vec![data_root], data_roots);

    let app_info = app_dirs2::AppInfo {
        name: "app-name",
        author: "app-author",
    };

    let app_root = app_dirs2::get_app_root(ty, &app_info).unwrap();
    let app_roots = app_dirs2::get_app_roots(ty, &app_info).unwrap();
    assert_eq!(vec![app_root], app_roots);

    let subdir = "testdir";
    let app_dir = app_dirs2::get_app_dir(ty, &app_info, subdir).unwrap();
    let app_dirs = app_dirs2::get_app_dirs(ty, &app_info, subdir).unwrap();
    assert_eq!(vec![app_dir], app_dirs);
}

#[test_case(AppDataType::SharedConfig, "XDG_CONFIG_DIRS"; "shared config")]
#[test_case(AppDataType::SharedData, "XDG_DATA_DIRS"; "shared data")]
fn test_multi_dir_missing(ty: AppDataType, env_var: impl AsRef<ffi::OsStr>) {
    let _env_guard = ENV_MUTEX.lock();

    let root_dir = tempfile::tempdir().unwrap();
    let dir1 = root_dir.path().join("dir1");
    let dir2 = root_dir.path().join("dir2");
    reset_env();
    env::set_var(env_var.as_ref(), join_os_str(&[&dir1, &dir2], ":"));

    let data_roots = app_dirs2::get_data_roots(ty).unwrap();
    assert_eq!(vec![dir1.clone(), dir2.clone()], data_roots);

    let app_info = app_dirs2::AppInfo {
        name: "app-name",
        author: "app-author",
    };

    let app_roots = app_dirs2::get_app_roots(ty, &app_info).unwrap();
    assert_eq!(data_roots.iter().map(|path| path.join(app_info.name)).collect::<Vec<_>>(), app_roots);

    let subdir = "testdir";
    let app_dirs = app_dirs2::get_app_dirs(ty, &app_info, subdir).unwrap();
    assert_eq!(app_roots.iter().map(|path| path.join(subdir)).collect::<Vec<_>>(), app_dirs);
}

#[test_case(AppDataType::SharedConfig, "XDG_CONFIG_DIRS"; "shared config")]
#[test_case(AppDataType::SharedData, "XDG_DATA_DIRS"; "shared data")]
fn test_multi_dir_existing_app_root(ty: AppDataType, env_var: impl AsRef<ffi::OsStr>) {
    let _env_guard = ENV_MUTEX.lock();

    let root_dir = tempfile::tempdir().unwrap();
    let dir1 = root_dir.path().join("dir1");
    let dir2 = root_dir.path().join("dir2");
    fs::create_dir_all(&dir2.join("app-name")).unwrap();
    reset_env();
    env::set_var(env_var.as_ref(), join_os_str(&[&dir1, &dir2], ":"));

    let data_roots = app_dirs2::get_data_roots(ty).unwrap();
    assert_eq!(vec![dir2.clone()], data_roots);

    let app_info = app_dirs2::AppInfo {
        name: "app-name",
        author: "app-author",
    };

    let app_roots = app_dirs2::get_app_roots(ty, &app_info).unwrap();
    assert_eq!(vec![dir2.join(app_info.name)], app_roots);

    let subdir = "testdir";
    let app_dirs = app_dirs2::get_app_dirs(ty, &app_info, subdir).unwrap();
    assert_eq!(vec![dir2.join(app_info.name).join(subdir)], app_dirs);
}

#[test_case(AppDataType::SharedConfig, "XDG_CONFIG_DIRS"; "shared config")]
#[test_case(AppDataType::SharedData, "XDG_DATA_DIRS"; "shared data")]
fn test_multi_dir_existing_app_dir(ty: AppDataType, env_var: impl AsRef<ffi::OsStr>) {
    let _env_guard = ENV_MUTEX.lock();

    let root_dir = tempfile::tempdir().unwrap();
    let dir1 = root_dir.path().join("dir1");
    let dir2 = root_dir.path().join("dir2");
    fs::create_dir_all(&dir1.join("app-name").join("testdir")).unwrap();
    fs::create_dir_all(&dir2.join("app-name")).unwrap();
    reset_env();
    env::set_var(env_var.as_ref(), join_os_str(&[&dir1, &dir2], ":"));

    let data_roots = app_dirs2::get_data_roots(ty).unwrap();
    assert_eq!(vec![dir1.clone(), dir2.clone()], data_roots);

    let app_info = app_dirs2::AppInfo {
        name: "app-name",
        author: "app-author",
    };

    let app_roots = app_dirs2::get_app_roots(ty, &app_info).unwrap();
    assert_eq!(data_roots.iter().map(|path| path.join(app_info.name)).collect::<Vec<_>>(), app_roots);

    let subdir = "testdir";
    let app_dirs = app_dirs2::get_app_dirs(ty, &app_info, subdir).unwrap();
    assert_eq!(vec![dir1.join(app_info.name).join(subdir)], app_dirs);
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
