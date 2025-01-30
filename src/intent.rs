use jni::{
    errors::Result,
    objects::{JObject, JString, JValue},
    JNIEnv,
};

/// A messaging object you can use to request an action from another android app component.
#[must_use]
pub struct Intent<'vm, 'env> {
    env: &'vm mut JNIEnv<'env>,
    object: JObject<'env>,
}

impl<'vm, 'env> Intent<'vm, 'env> {
    pub fn from_object(env: &'vm mut JNIEnv<'env>, object: JObject<'env>) -> Self {
        Self { env, object }
    }

    // TODO: Could also return a borrowed JavaStr so that the caller decides if they just want to read or also allocate
    pub fn action(&mut self) -> Result<String> {
        let action = self
            .env
            .call_method(&self.object, "getAction", "()Ljava/lang/String;", &[])?
            .l()?
            .into();

        let action = self.env.get_string(&action)?;
        Ok(action.into())
    }

    pub fn data_string(&mut self) -> Result<String> {
        let data_string = self
            .env
            .call_method(&self.object, "getDataString", "()Ljava/lang/String;", &[])?
            .l()?
            .into();
        let data_string = self.env.get_string(&data_string)?;
        Ok(data_string.into())
    }

    /// <https://developer.android.com/reference/android/content/Intent#getStringExtra(java.lang.String)>
    pub fn string_extra(&mut self, name: &str) -> Result<String> {
        let name = self.env.new_string(name)?;

        let extra = self
            .env
            .call_method(
                &self.object,
                "getStringExtra",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[JValue::Object(&name.into())],
            )?
            .l()?
            .into();
        let extra = self.env.get_string(&extra)?;
        Ok(extra.into())
    }
}

/// A messaging object you can use to request an action from another Android app component.
#[must_use]
pub struct IntentBuilder<'vm, 'env> {
    inner: Result<Intent<'vm, 'env>>,
}

impl<'vm, 'env> IntentBuilder<'vm, 'env> {
    pub fn from_object(env: &'vm mut JNIEnv<'env>, object: JObject<'env>) -> Self {
        Self {
            inner: Ok(Intent::from_object(env, object)),
        }
    }

    fn from_fn(f: impl FnOnce() -> Result<Intent<'vm, 'env>>) -> Self {
        let inner = f();
        Self { inner }
    }

    pub fn new(env: &'vm mut JNIEnv<'env>, action: impl AsRef<str>) -> Self {
        Self::from_fn(|| {
            let intent_class = env.find_class("android/content/Intent")?;
            let action_view =
                env.get_static_field(&intent_class, action.as_ref(), "Ljava/lang/String;")?;

            let intent = env.new_object(
                &intent_class,
                "(Ljava/lang/String;)V",
                &[action_view.borrow()],
            )?;

            Ok(Intent::from_object(env, intent))
        })
    }

    pub fn new_with_uri(
        env: &'env mut JNIEnv<'env>,
        action: impl AsRef<str>,
        uri: impl AsRef<str>,
    ) -> Self {
        Self::from_fn(|| {
            let url_string = env.new_string(uri)?;
            let uri_class = env.find_class("android/net/Uri")?;
            let uri = env.call_static_method(
                uri_class,
                "parse",
                "(Ljava/lang/String;)Landroid/net/Uri;",
                &[JValue::Object(&url_string)],
            )?;

            let intent_class = env.find_class("android/content/Intent")?;
            let action_view = env
                .get_static_field(&intent_class, action.as_ref(), "Ljava/lang/String;")?
                .l()?;

            let intent = env.new_object(
                &intent_class,
                "(Ljava/lang/String;Landroid/net/Uri;)V",
                &[JValue::Object(&action_view), uri.borrow()],
            )?;

            Ok(Intent {
                env,
                object: intent,
            })
        })
    }

    /// Set the class name for the intent target.
    /// ```no_run
    /// use android_intent::{Action, Extra, IntentBuilder};
    ///
    /// # android_intent::with_current_env(|env| {
    /// let intent = IntentBuilder::new(env, Action::Send)
    ///     .set_class_name("com.excample", "IntentTarget");
    /// # })
    /// ```
    pub fn set_class_name(
        self,
        package_name: impl AsRef<str>,
        class_name: impl AsRef<str>,
    ) -> Self {
        self.and_then(|inner| {
            let package_name = inner.env.new_string(package_name)?;
            let class_name = inner.env.new_string(class_name)?;

            inner.env.call_method(
                &inner.object,
                "setClassName",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
                &[JValue::Object(&package_name), JValue::Object(&class_name)],
            )?;

            Ok(inner)
        })
    }

    /// Add extended data to the intent.
    /// ```no_run
    /// use android_intent::{Action, Extra, IntentBuilder};
    ///
    /// # android_intent::with_current_env(|env| {
    /// let intent = IntentBuilder::new(env, Action::Send)
    ///     .with_extra(Extra::Text, "Hello World!");
    /// # })
    /// ```
    pub fn with_extra(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.and_then(|inner| {
            let key = inner.env.new_string(key)?;
            let value = inner.env.new_string(value)?;

            inner.env.call_method(
                &inner.object,
                "putExtra",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
                &[JValue::Object(&key), JValue::Object(&value)],
            )?;

            Ok(inner)
        })
    }

    /// Builds a new [`super::Action::Chooser`] Intent that wraps the given target intent.
    /// ```no_run
    /// use android_intent::{Action, IntentBuilder};
    ///
    /// # android_intent::with_current_env(|env| {
    /// let intent = IntentBuilder::new(env, Action::Send)
    ///     .into_chooser();
    /// # })
    /// ```
    pub fn into_chooser(self) -> Self {
        self.into_chooser_with_title(None::<&str>)
    }

    pub fn into_chooser_with_title(self, title: Option<impl AsRef<str>>) -> Self {
        self.and_then(|mut inner| {
            let title_value = if let Some(title) = title {
                inner.env.new_string(title)?
            } else {
                JString::default()
            };

            let intent_class = inner.env.find_class("android/content/Intent")?;
            let intent = inner.env.call_static_method(
                intent_class,
                "createChooser",
                "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;",
                &[JValue::Object(&inner.object), JValue::Object(&title_value)],
            )?;

            inner.object = intent.try_into()?;
            Ok(inner)
        })
    }

    /// Set an explicit MIME data type.
    /// ```no_run
    /// use android_intent::{Action, IntentBuilder};
    ///
    /// # android_intent::with_current_env(|env| {
    /// let intent = IntentBuilder::new(env, Action::Send)
    ///     .with_type("text/plain");
    /// # })
    /// ```
    pub fn with_type(self, type_name: impl AsRef<str>) -> Self {
        self.and_then(|inner| {
            let jstring = inner.env.new_string(type_name)?;

            inner.env.call_method(
                &inner.object,
                "setType",
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[JValue::Object(&jstring)],
            )?;

            Ok(inner)
        })
    }

    pub fn start_activity(self) -> Result<()> {
        let cx = ndk_context::android_context();
        let activity = unsafe { JObject::from_raw(cx.context() as jni::sys::jobject) };

        self.inner.and_then(|inner| {
            inner.env.call_method(
                activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[JValue::Object(&inner.object)],
            )?;

            Ok(())
        })
    }

    fn and_then(mut self, f: impl FnOnce(Intent<'vm, 'env>) -> Result<Intent<'vm, 'env>>) -> Self {
        self.inner = self.inner.and_then(f);
        self
    }
}
