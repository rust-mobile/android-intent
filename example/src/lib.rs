use android_activity::AndroidApp;
use android_intent::{current_vm, Action, Intent};

#[no_mangle]
fn android_main(android_app: AndroidApp) {
    let vm = current_vm();
    Intent::new(&vm, Action::View)
        .with_type("text/plain")
        .with_extra("EXTRA_TEXT", "Hello World!")
        .start_activity();

    loop {}
}
