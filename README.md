# android-intent

[![crate](https://img.shields.io/crates/v/android-intent.svg)](https://crates.io/crates/android-intent)
[![documentation](https://docs.rs/android-intent/badge.svg)](https://docs.rs/android-intent)

```rust
use android_activity::AndroidApp;
use android_intent::{with_current_env, Action, Extra, Intent};

#[no_mangle]
fn android_main(_android_app: AndroidApp) {
    with_current_env(|env| {
        Intent::new(env, Action::Send)
            .with_type("text/plain")
            .with_extra(Extra::Text, "Hello World!")
            .into_chooser()
            .start_activity()
            .unwrap()
    });
}
```
