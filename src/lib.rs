mod action;
pub use action::Action;

mod extra;
pub use extra::Extra;

mod intent;
pub use intent::{Intent, IntentBuilder};
use jni::{errors::Result, objects::JObject, JNIEnv, JavaVM};

/// Run 'f' with the current [`JNIEnv`] from [`ndk_context`].
pub fn with_current_env(f: impl FnOnce(&mut JNIEnv<'_>)) {
    // XXX: Pass AndroidActivity?
    let cx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(cx.vm().cast()) }.unwrap();
    let mut env = vm.attach_current_thread().unwrap();

    // TODO: Pass current activity?
    f(&mut env);
}

/// Provides the intent from [`Activity#getIntent()`].
///
/// [`Activity#getIntent()`]: https://developer.android.com/reference/android/app/Activity#getIntent()
pub fn with_current_intent<T>(f: impl FnOnce(Intent<'_, '_>) -> T) -> Result<T> {
    // XXX: Pass AndroidActivity?
    // XXX: Support onNewIntent() callback with setIntent()?
    // https://github.com/rust-mobile/ndk/issues/275
    let cx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(cx.vm().cast()) }?;
    let mut env = vm.attach_current_thread().unwrap();
    let activity = unsafe { JObject::from_raw(cx.context() as jni::sys::jobject) };

    let object = env
        .call_method(activity, "getIntent", "()Landroid/content/Intent;", &[])?
        .l()?;

    let intent = Intent::from_object(&mut env, object);

    Ok(f(intent))
}
