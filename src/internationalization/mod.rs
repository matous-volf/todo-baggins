use std::ops::Deref;
use std::str::FromStr;
use chrono::Locale;
use dioxus_sdk::i18n::Language;
use unic_langid_impl::LanguageIdentifier;

const EN_US: &str = include_str!("en_us.json");
const CS_CZ: &str = include_str!("cs_cz.json");

pub(crate) fn get_languages() -> Vec<Language> {
    Vec::from([EN_US, CS_CZ]).into_iter().map(|texts| Language::from_str(texts).unwrap()).collect()
}

pub(crate) struct LocaleFromLanguageIdentifier<'a>(&'a LanguageIdentifier);

impl<'a> Deref for LocaleFromLanguageIdentifier<'a> {
    type Target = LanguageIdentifier;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<LocaleFromLanguageIdentifier<'a>> for Locale {
    fn from(language_identifier: LocaleFromLanguageIdentifier) -> Self {
        language_identifier.to_string().replace("-", "_").parse().unwrap()
    }
}

impl<'a> From<&'a LanguageIdentifier> for LocaleFromLanguageIdentifier<'a> {
    fn from(language_identifier: &'a LanguageIdentifier) -> Self {
        LocaleFromLanguageIdentifier(language_identifier)
    }
}
