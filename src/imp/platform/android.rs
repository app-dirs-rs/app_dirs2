use common::{AppDataType, AppDirsError};
use std::path::PathBuf;
use std::io::{Error, ErrorKind};

pub const USE_AUTHOR: bool = false;

impl From<jni::errors::Error> for AppDirsError {
    fn from(error: jni::errors::Error) -> Self {
        AppDirsError::Io(Error::new(ErrorKind::Other, error))
    }
}

fn get_jni_app_dir(
    activity: &jni::objects::JObject<'_>,
    env: &jni::JNIEnv<'_>,
    method: &str,
) -> Result<String, AppDirsError> {

    let dir = env.call_method(*activity, method, "()Ljava/io/File;", &Vec::new()[..])?;
    let dir = match dir {
        jni::objects::JValue::Object(o) => o,
        _ => return Err(AppDirsError::Io(Error::new(ErrorKind::Other, "dir is not a object"))),
    };

    let path_string = env.call_method(dir, "getPath", "()Ljava/lang/String;", &Vec::new()[..])?;
    let path_string = match path_string {
        jni::objects::JValue::Object(o) => jni::objects::JString::from(o),
        _ => return Err(AppDirsError::Io(Error::new(ErrorKind::Other, "path_string is not a object"))),
    };

    Ok(env.get_string(path_string)?.into())
}

pub fn get_app_dir(t: AppDataType) -> Result<PathBuf, AppDirsError> {
    let native_activity = ndk_glue::native_activity();
    let vm = unsafe { jni::JavaVM::from_raw(native_activity.vm()) }?;
    let env = vm.attach_current_thread()?;
    let activity = jni::objects::JObject::from(native_activity.activity());

    let path_string = match t {
        AppDataType::UserConfig => get_jni_app_dir(&activity, &env, "getDataDir")?,
        AppDataType::UserData => get_jni_app_dir(&activity, &env, "getFilesDir")?,
        AppDataType::UserCache => get_jni_app_dir(&activity, &env, "getCacheDir")?,
        AppDataType::SharedData => get_jni_app_dir(&activity, &env, "getExternalFilesDir")?,
        AppDataType::SharedConfig => get_jni_app_dir(&activity, &env, "getExternalFilesDir")?,
    };

    Ok(PathBuf::from(path_string))
}
