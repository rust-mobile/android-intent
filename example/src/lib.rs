use android_activity::AndroidApp;
use android_intent::{with_current_env, Action, Extra, Intent};
use jni::JavaVM;

#[no_mangle]
fn android_main(_android_app: AndroidApp) {
    with_current_env(|env| {
        Intent::new(env.clone(), Action::Send)
            .with_type("text/plain")
            .with_extra(Extra::Text, "Hello World!")
            .create_chooser()
            .start_activity()
    });

    loop {}
}
