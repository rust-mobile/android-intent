use android_activity::AndroidApp;
use android_intent::{with_current_env, Action, Extra, Intent};

#[no_mangle]
fn android_main(_android_app: AndroidApp) {
    let _ = with_current_env(|env| {
        Intent::new(env, Action::Send)
            .with_type("text/plain")
            .with_extra(Extra::Text, "Hello World!")
            .into_chooser()
            .start_activity()
    });

    loop {}
}
