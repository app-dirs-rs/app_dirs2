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
    env: &mut jni::JNIEnv<'_>,
    method: &str,
    has_string_argument: bool,
) -> Result<String, AppDirsError> {
    let dir = if has_string_argument {
        env.call_method(
            context,
            method,
            "(Ljava/lang/String;)Ljava/io/File;",
            &[jni::objects::JValue::Object(&jni::objects::JObject::null())],
        )
    } else {
        env.call_method(context, method, "()Ljava/io/File;", &[])
    }?
    .l()?;

    let path_string = env
        .call_method(dir, "getPath", "()Ljava/lang/String;", &[])?
        .l()?;
    let path_string = jni::objects::JString::from(path_string);
    let path_string = env.get_string(&path_string)?;

    Ok(path_string.into())
}

pub fn get_app_dir(t: AppDataType) -> Result<PathBuf, AppDirsError> {
    // Issue #33: Android apps run inside Termux do not have an Android JNI
    // context, so the call to `ndk_context::android_context()` below will
    // panic. However, Termux does provide the usual `$XDG_*` environment
    // variables. So as a workaround, we check those variables first. "Regular"
    // android apps will not have them, but will have a context.

    let maybe_unix = super::unix::get_app_dir(t);
    if maybe_unix.is_ok() {
        return maybe_unix;
    }

    let android_context = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) }?;
    let mut env = vm.attach_current_thread()?;
    let context = unsafe { jni::objects::JObject::from_raw(android_context.context().cast()) };

    let path_string = match t {
        AppDataType::UserConfig => get_jni_app_dir(&context, &mut env, "getDataDir", false)?,
        AppDataType::UserData => get_jni_app_dir(&context, &mut env, "getFilesDir", false)?,
        AppDataType::UserCache => get_jni_app_dir(&context, &mut env, "getCacheDir", false)?,
        AppDataType::SharedData | AppDataType::SharedConfig => {
            get_jni_app_dir(&context, &mut env, "getExternalFilesDir", true)?
        },
        // AppDataType::SharedCache => get_jni_app_dir(&context, &mut env, "getExternalCacheDir", false)?,
    };

    Ok(PathBuf::from(path_string))
}
