use android_activity::AndroidApp;
use android_intent::{current_vm, Action, Intent};

#[no_mangle]
fn android_main(android_app: AndroidApp) {
    let vm = current_vm();
    Intent::new(&vm, Action::View, "https://google.com").start_activity();

    loop {}
}
