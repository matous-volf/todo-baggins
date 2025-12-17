use crate::dotenv;
use chrono::Locale;
use feruca::Collator;
use std::ops::Deref;
use std::sync::{LazyLock, Mutex};
use unic_langid_impl::LanguageIdentifier;

pub(crate) fn get_language_identifier() -> LanguageIdentifier {
    dotenv::LANGUAGE_CODE
        .parse::<LanguageIdentifier>()
        .expect("The LANGUAGE_CODE environment variable is not a valid language code.")
}

pub(crate) static COLLATOR: LazyLock<Mutex<Collator>> =
    LazyLock::new(|| Mutex::new(Collator::default()));

pub(crate) struct LocaleFromLanguageIdentifier<'a>(&'a LanguageIdentifier);

impl Deref for LocaleFromLanguageIdentifier<'_> {
    type Target = LanguageIdentifier;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl From<LocaleFromLanguageIdentifier<'_>> for Locale {
    fn from(language_identifier: LocaleFromLanguageIdentifier) -> Self {
        language_identifier
            .to_string()
            .replace("-", "_")
            .parse()
            .unwrap()
    }
}

impl<'a> From<&'a LanguageIdentifier> for LocaleFromLanguageIdentifier<'a> {
    fn from(language_identifier: &'a LanguageIdentifier) -> Self {
        LocaleFromLanguageIdentifier(language_identifier)
    }
}
