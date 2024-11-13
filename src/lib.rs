mod action;
pub use action::Action;

mod extra;
pub use extra::Extra;

mod intent;
pub use intent::Intent;
use jni::{JNIEnv, JavaVM};
use jni::errors::Error;

/// Run 'f' with the current [`JNIEnv`] from [`ndk_context`].
pub fn with_current_env<F, T>(f: F) -> Result<T, Error>
where
    F: FnOnce(&mut JNIEnv) -> Result<T, Error>,
{
    let cx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(cx.vm().cast()) }.unwrap();
    let mut env = vm.attach_current_thread().unwrap();

    env.with_local_frame(16, |env| f(env))
}
