/// Action to invoke with an intent
pub enum Action {
    Main,
    Send,
    Edit,
    Chooser,
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        match self {
            Self::Main => "ACTION_MAIN",
            Self::Send => "ACTION_SEND",
            Self::Edit => "ACTION_EDIT",
            Self::Chooser => "ACTION_CHOOSER",
        }
    }
}
