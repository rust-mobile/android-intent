mod action;
pub use action::Action;

mod extra;
pub use extra::Extra;

mod intent;
pub use intent::Intent;
use jni::{JNIEnv, JavaVM};

/// Run 'f' with the current `[JNIEnv]` from `ndk_context`.
pub fn with_current_env(f: impl FnOnce(JNIEnv)) {
    let cx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(cx.vm().cast()) }.unwrap();
    let env = vm.attach_current_thread().unwrap();

    f(env.clone());

    drop(env);
}
