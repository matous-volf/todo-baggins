use std::env;
use dioxus::prelude::ServerFnError;
use unic_langid_impl::LanguageIdentifier;
use dioxus::prelude::*;
use dotenvy::dotenv;

#[server]
pub(crate) async fn get_language_identifier() -> Result<LanguageIdentifier, ServerFnError> {
    dotenv().expect("Could not load environment variables from the .env file.");
    
    Ok(env::var("LANGUAGE_CODE")
        .expect("The environment variable LANGUAGE_CODE must be set.")
        .parse::<LanguageIdentifier>()?)
}
