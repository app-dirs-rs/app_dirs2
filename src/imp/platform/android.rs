use crate::common::*;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub const USE_AUTHOR: bool = false;

impl From<jni::errors::Error> for AppDirsError {
    fn from(error: jni::errors::Error) -> Self {
        AppDirsError::Io(Error::new(ErrorKind::Other, error))
    }
}

fn get_jni_app_dir(
    context: &jni::objects::JObject<'_>,
    env: &jni::JNIEnv<'_>,
    method: &str,
) -> Result<String, AppDirsError> {
    let dir = env.call_method(*context, method, "()Ljava/io/File;", &Vec::new()[..])?;
    let dir = match dir {
        jni::objects::JValue::Object(o) => o,
        _ => {
            return Err(AppDirsError::Io(Error::new(
                ErrorKind::Other,
                "dir is not `JObject`",
            )))
        },
    };

    let path_string = env.call_method(dir, "getPath", "()Ljava/lang/String;", &Vec::new()[..])?;
    let path_string = match path_string {
        jni::objects::JValue::Object(o) => jni::objects::JString::from(o),
        _ => {
            return Err(AppDirsError::Io(Error::new(
                ErrorKind::Other,
                "path_string is not `JObject`",
            )))
        },
    };

    Ok(env.get_string(path_string)?.into())
}

pub fn get_app_dir(t: AppDataType) -> Result<PathBuf, AppDirsError> {
    let android_context = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) }?;
    let env = vm.attach_current_thread()?;
    let context = jni::objects::JObject::from(android_context.context().cast());

    let path_string = match t {
        AppDataType::UserConfig => get_jni_app_dir(&context, &env, "getDataDir")?,
        AppDataType::UserData => get_jni_app_dir(&context, &env, "getFilesDir")?,
        AppDataType::UserCache => get_jni_app_dir(&context, &env, "getCacheDir")?,
        AppDataType::SharedData => get_jni_app_dir(&context, &env, "getExternalFilesDir")?, // Deprecated in Android 11+
        AppDataType::SharedConfig => get_jni_app_dir(&context, &env, "getExternalFilesDir")?, // Deprecated in Android 11+
    };

    Ok(PathBuf::from(path_string))
}
