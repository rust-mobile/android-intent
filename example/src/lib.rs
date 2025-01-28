use android_activity::AndroidApp;
use android_intent::{with_current_env, Action, Extra, IntentBuilder};

#[no_mangle]
fn android_main(_android_app: AndroidApp) {
    with_current_env(|env| {
        IntentBuilder::new(env, Action::Send)
            .with_type("text/plain")
            .with_extra(Extra::Text, "Hello World!")
            .into_chooser()
            .start_activity()
            .unwrap()
    });
}
