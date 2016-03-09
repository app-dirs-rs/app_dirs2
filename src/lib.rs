mod common;
pub use common::*;
mod imp;
pub use imp::*;

#[cfg(test)]
mod tests {
    use super::*;
    use AppDirType::*;
    #[test]
    fn it_works() {
        // let info = AppInfo::new("Fancy Dev", "Cool App");
        let types = [
            UserConfig,
            UserData,
            UserCache,
            SharedData,
            SharedConfig,
        ];
        for &t in types.iter() {
            println!("{:?} = {:?}", t, get_app_dir(t))
        }
    }
}
