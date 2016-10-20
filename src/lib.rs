#![warn(missing_docs)]
//! Access platform-dependent canonical locations for app-specific data.
mod common;
pub use common::*;
mod imp;
pub use imp::*;

#[cfg(test)]
mod tests {
    use super::*;
    use AppDataType::*;
    #[test]
    fn it_works() {
        let info = AppInfo::new("Awesome App", "Dedicated Dev");
        let types = [UserConfig, UserData, UserCache, SharedData, SharedConfig];
        for &t in types.iter() {
            println!("{:?} = {:?}", t, get_app_dir_path(t, &info))
        }
    }
}
