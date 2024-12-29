use chrono::Locale;
use dioxus::fullstack::once_cell::sync::Lazy;
use feruca::Collator;
use std::ops::Deref;
use std::sync::Mutex;
use unic_langid_impl::LanguageIdentifier;

pub(crate) static COLLATOR: Lazy<Mutex<Collator>> = Lazy::new(|| Mutex::new(Collator::default()));

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
