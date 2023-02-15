#![cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]

use std::env;
use std::io;
use std::path;
use std::sync;

use app_dirs2::AppDataType;
use once_cell::sync::Lazy;
use test_case::test_case;

// This test suite checks the effects of the app_dirs2 crate on the file system.
//
// The functions with the prefix get_ should not touch the file system.  The functions without the
// prefix should create the returned directory if it doesn’t exist.
//
// As only the unix/XDG implementation supports changing the root configuration directory, we can
// only run this test suite on this platform.  As we use environment variables to set the
// configuration root, we have to make sure that the tests are run in sequence and don’t overlap,
// see the `ENV_MUTEX` mutex.

// For test cases that depend on environment variables
static ENV_MUTEX: Lazy<sync::Mutex<()>> = Lazy::new(|| sync::Mutex::new(()));

fn set_root_dir(path: &path::Path) -> path::PathBuf {
    let root = path.join("root");
    env::set_var("HOME", root.join("home"));
    env::set_var("XDG_CACHE_HOME", "");
    env::set_var("XDG_CONFIG_HOME", "");
    env::set_var("XDG_DATA_HOME", "");
    env::set_var("XDG_DATA_DIRS", root.join("data"));
    env::set_var("XDG_CONFIG_DIRS", root.join("config"));
    root
}

#[test_case(AppDataType::UserCache; "user cache")]
#[test_case(AppDataType::UserConfig; "user config")]
#[test_case(AppDataType::UserData; "user data")]
#[test_case(AppDataType::SharedConfig; "shared config")]
#[test_case(AppDataType::SharedData; "shared data")]
fn test_no_create(ty: AppDataType) -> io::Result<()> {
    let _env_guard = ENV_MUTEX.lock();

    let dir = tempfile::tempdir()?;
    let root_dir = set_root_dir(dir.path());

    let info = app_dirs2::AppInfo {
        name: "test-app",
        author: "test-author",
    };

    let data_root = app_dirs2::get_data_root(ty).unwrap();
    assert!(
        data_root.starts_with(&root_dir),
        "Data root does not start with root dir: data root = {}, root dir = {}",
        data_root.display(),
        root_dir.display()
    );
    assert!(!root_dir.exists());

    let app_root = app_dirs2::get_app_root(ty, &info).unwrap();
    assert!(
        app_root.starts_with(&data_root),
        "App root does not start with data root: app root = {}, data root = {}",
        app_root.display(),
        data_root.display()
    );
    assert!(!root_dir.exists());

    let app_dir = app_dirs2::get_app_dir(ty, &info, "testdir").unwrap();
    assert!(
        app_dir.starts_with(&app_root),
        "App dir does not start with app root: app dir = {}, app root = {}",
        app_dir.display(),
        app_root.display()
    );
    assert!(!root_dir.exists());

    dir.close()
}

#[test_case(AppDataType::UserCache; "user cache")]
#[test_case(AppDataType::UserConfig; "user config")]
#[test_case(AppDataType::UserData; "user data")]
#[test_case(AppDataType::SharedConfig; "shared config")]
#[test_case(AppDataType::SharedData; "shared data")]
fn test_create(ty: AppDataType) -> io::Result<()> {
    let _env_guard = ENV_MUTEX.lock();

    let dir = tempfile::tempdir()?;
    let root_dir = set_root_dir(dir.path());

    let info = app_dirs2::AppInfo {
        name: "test-app",
        author: "test-author",
    };

    let data_root = app_dirs2::data_root(ty).unwrap();
    assert!(
        data_root.starts_with(&root_dir),
        "Data root does not start with root dir: data root = {}, root dir = {}",
        data_root.display(),
        root_dir.display()
    );
    assert!(data_root.is_dir());

    let app_root = app_dirs2::app_root(ty, &info).unwrap();
    assert!(
        app_root.starts_with(&data_root),
        "App root does not start with data root: app root = {}, data root = {}",
        app_root.display(),
        data_root.display()
    );
    assert!(app_root.is_dir());

    let app_dir = app_dirs2::app_dir(ty, &info, "testdir").unwrap();
    assert!(
        app_dir.starts_with(&app_root),
        "App dir does not start with app root: app dir = {}, app root = {}",
        app_dir.display(),
        app_root.display()
    );
    assert!(app_dir.is_dir());

    dir.close()
}
