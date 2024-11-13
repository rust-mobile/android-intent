use jni::{
    objects::{GlobalRef, JObject},
    JNIEnv,
};
use crate::{Error, with_current_env};

struct Inner {
    object: GlobalRef,
}

/// A messaging object you can use to request an action from another android app component.
#[must_use]
pub struct Intent {
    inner: Result<Inner, Error>,
}

impl Intent {
    pub fn from_object(env: &mut JNIEnv, object: JObject) -> Result<Self, Error> {
        Ok(Self {
            inner: Ok(Inner {
                object: env.new_global_ref(object)?,
            }),
        })
    }

    fn from_fn(f: impl FnOnce() -> Result<Inner, Error>) -> Self {
        let inner = f();
        Self { inner }
    }

    pub fn new(env: &mut JNIEnv, action: impl AsRef<str>) -> Self {
        Self::from_fn(|| {
            let intent_class = env.find_class("android/content/Intent")?;
            let action_view =
                env.get_static_field(&intent_class, action.as_ref(), "Ljava/lang/String;")?;

            let intent =
                env.new_object(intent_class, "(Ljava/lang/String;)V", &[(&action_view).into()])?;

            Ok(Inner {
                object: env.new_global_ref(intent)?,
            })
        })
    }

    pub fn new_with_uri(env: &mut JNIEnv, action: impl AsRef<str>, uri: impl AsRef<str>) -> Self {
        Self::from_fn(|| {
            let url_string = env.new_string(uri)?;
            let uri_class = env.find_class("android/net/Uri")?;
            let uri = env.call_static_method(
                uri_class,
                "parse",
                "(Ljava/lang/String;)Landroid/net/Uri;",
                &[(&url_string).into()],
            )?;

            let intent_class = env.find_class("android/content/Intent")?;
            let action_view =
                env.get_static_field(&intent_class, action.as_ref(), "Ljava/lang/String;")?;

            let intent = env.new_object(
                intent_class,
                "(Ljava/lang/String;Landroid/net/Uri;)V",
                &[(&action_view).into(), (&uri).into()],
            )?;

            Ok(Inner {
                object: env.new_global_ref(intent)?,
            })
        })
    }

    /// Set the class name for the intent target.
    /// ```no_run
    /// use android_intent::{Action, Extra, Intent};
    ///
    /// # android_intent::with_current_env(|env| {
    ///     let intent = Intent::new(env, Action::Send);
    ///     Ok(intent.set_class_name("com.excample", "IntentTarget"))
    /// # });
    /// ```
    pub fn set_class_name(self, package_name: impl AsRef<str>, class_name: impl AsRef<str>) -> Self {
        self.and_then(|inner| { with_current_env(|env| {
            let package_name = env.new_string(package_name)?;
            let class_name = env.new_string(class_name)?;

            env.call_method(
                &inner.object,
                "setClassName",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
                &[(&package_name).into(), (&class_name).into()],
            )?;

            Ok(inner)
        })})
    }

    /// Add extended data to the intent.
    /// ```no_run
    /// use android_intent::{Action, Extra, Intent};
    ///
    /// # android_intent::with_current_env(|env| {
    ///     let intent = Intent::new(env, Action::Send);
    ///     Ok(intent.with_extra(Extra::Text, "Hello World!"))
    /// # });
    /// ```
    pub fn with_extra(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.and_then(|inner| { with_current_env(|env| {
            let key = env.new_string(key)?;
            let value = env.new_string(value)?;

            env.call_method(
                &inner.object,
                "putExtra",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
                &[(&key).into(), (&value).into()],
            )?;

            Ok(inner)
        })})
    }

    /// Builds a new [`Action::Chooser`] Intent that wraps the given target intent.
    /// ```no_run
    /// use android_intent::{Action, Intent};
    ///
    /// # android_intent::with_current_env(|env| {
    ///     Ok(Intent::new(env, Action::Send).into_chooser())
    /// # });
    /// ```
    pub fn into_chooser(self) -> Self {
        self.into_chooser_with_title(None::<&str>)
    }

    pub fn into_chooser_with_title(self, title: Option<impl AsRef<str>>) -> Self {
        self.and_then(|mut inner| { with_current_env(|env| {
            let title_value = if let Some(title) = title {
                env.new_string(title)?
            } else {
                JObject::null().into()
            };

            let intent_class = env.find_class("android/content/Intent")?;
            let intent = env.call_static_method(
                intent_class,
                "createChooser",
                "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;",
                &[(&inner.object).into(), (&title_value).into()],
            )?;

            inner.object = env.new_global_ref(intent.l()?)?;
            Ok(inner)
        })})
    }

    /// Set an explicit MIME data type.
    /// ```no_run
    /// use android_intent::{Action, Intent};
    ///
    /// # android_intent::with_current_env(|env| {
    ///     let intent = Intent::new(env, Action::Send);
    ///     Ok(intent.with_type("text/plain"))
    /// # });
    /// ```
    pub fn with_type(self, type_name: impl AsRef<str>) -> Self {
        self.and_then(|inner| { with_current_env(|env| {
            let jstring = env.new_string(type_name)?;

            env.call_method(
                &inner.object,
                "setType",
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[(&jstring).into()],
            )?;

            Ok(inner)
        })})
    }

    pub fn start_activity(self) -> Result<(), Error> {
        let cx = ndk_context::android_context();
        let activity = unsafe { JObject::from_raw(cx.context() as jni::sys::jobject) };

        self.inner.and_then(|inner| { with_current_env(|env| {
            env.call_method(
                activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[(&inner.object).into()],
            )?;

            Ok(())
        })})
    }

    fn and_then(mut self, f: impl FnOnce(Inner) -> Result<Inner, Error>) -> Self {
        self.inner = self.inner.and_then(f);
        self
    }
}
