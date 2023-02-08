use jni::{
    objects::{JObject, JString},
    JNIEnv,
};

/// A messaging object you can use to request an action from another android app component.
#[must_use]
pub struct Intent<'env> {
    pub env: JNIEnv<'env>,
    pub object: JObject<'env>,
}

impl<'env> Intent<'env> {
    pub fn from_object(env: JNIEnv<'env>, object: JObject<'env>) -> Self {
        Self { env, object }
    }

    pub fn new(env: JNIEnv<'env>, action: impl AsRef<str>) -> Self {
        let intent_class = env.find_class("android/content/Intent").unwrap();
        let action_view = env
            .get_static_field(intent_class, action.as_ref(), "Ljava/lang/String;")
            .unwrap();

        let intent = env
            .new_object(intent_class, "(Ljava/lang/String;)V", &[action_view.into()])
            .unwrap();

        Self::from_object(env, intent)
    }

    pub fn new_with_uri(env: JNIEnv<'env>, action: impl AsRef<str>, uri: impl AsRef<str>) -> Self {
        let url_string = env.new_string(uri).unwrap();
        let uri_class = env.find_class("android/net/Uri").unwrap();
        let uri = env
            .call_static_method(
                uri_class,
                "parse",
                "(Ljava/lang/String;)Landroid/net/Uri;",
                &[JString::from(url_string).into()],
            )
            .unwrap();

        let intent_class = env.find_class("android/content/Intent").unwrap();
        let action_view = env
            .get_static_field(intent_class, action.as_ref(), "Ljava/lang/String;")
            .unwrap();

        let intent = env
            .new_object(
                intent_class,
                "(Ljava/lang/String;Landroid/net/Uri;)V",
                &[action_view.into(), uri.into()],
            )
            .unwrap();

        Self::from_object(env, intent)
    }

    /// Add extended data to the intent.
    /// ```no_run
    /// use android_intent::{Action, Extra, Intent};
    ///
    /// # android_intent::with_current_env(|env| {
    /// let intent = Intent::new(env, Action::Send);
    /// intent.push_extra(Extra::Text, "Hello World!")
    /// # })
    /// ```
    pub fn push_extra(&self, key: impl AsRef<str>, value: impl AsRef<str>) {
        let key = self.env.new_string(key).unwrap();
        let value = self.env.new_string(value).unwrap();

        self.env
            .call_method(
                self.object,
                "putExtra",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
                &[key.into(), value.into()],
            )
            .unwrap();
    }

    pub fn with_extra(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> &Self {
        self.push_extra(key, value);
        self
    }

    /// Builds a new [`Action::Chooser`](Action) Intent that wraps the given target intent.
    /// ```no_run
    /// use android_intent::{Action, Intent};
    ///
    /// # android_intent::with_current_env(|env| {
    /// let intent = Intent::new(env, Action::Send);
    /// let chooser_intent = intent.create_chooser();
    /// # })
    /// ```
    pub fn create_chooser(&self) -> Self {
        self.create_chooser_with_title(None::<&str>)
    }

    pub fn create_chooser_with_title(&self, title: Option<impl AsRef<str>>) -> Self {
        let title_value = title
            .map(|s| self.env.new_string(s).unwrap().into())
            .unwrap_or_else(|| JObject::null().into());

        let intent_class = self.env.find_class("android/content/Intent").unwrap();
        let intent = self
            .env
            .call_static_method(
                intent_class,
                "createChooser",
                "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;",
                &[self.object.into(), title_value],
            )
            .unwrap();

        Self::from_object(self.env, intent.try_into().unwrap())
    }

    /// Set an explicit MIME data type.
    /// ```no_run
    /// use android_intent::{Action, Intent};
    ///
    /// # android_intent::with_current_env(|env| {
    /// let intent = Intent::new(env, Action::Send);
    /// intent.set_type("text/plain");
    /// # })
    /// ```
    pub fn set_type(&self, type_name: impl AsRef<str>) {
        let jstring = self.env.new_string(type_name).unwrap();

        self.env
            .call_method(
                self.object,
                "setType",
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[jstring.into()],
            )
            .unwrap();
    }

    pub fn with_type(&self, type_name: impl AsRef<str>) -> &Self {
        self.set_type(type_name);
        self
    }

    pub fn start_activity(&self) {
        let cx = ndk_context::android_context();
        let activity = unsafe { JObject::from_raw(cx.context() as jni::sys::jobject) };

        self.env
            .call_method(
                activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[self.object.into()],
            )
            .unwrap();
    }
}
