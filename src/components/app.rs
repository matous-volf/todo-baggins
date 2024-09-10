use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_query::prelude::{use_init_query_client};
use crate::query::{QueryErrors, QueryKey, QueryValue};
use dioxus_sdk::i18n::{use_init_i18n};
use crate::internationalization::get_languages;
use crate::server::internationalization::get_language_identifier;

#[component]
pub(crate) fn App() -> Element {
    use_init_query_client::<QueryValue, QueryErrors, QueryKey>();
    
    let language_identifier = use_server_future(get_language_identifier)?.unwrap().unwrap();
    use_init_i18n(language_identifier.clone(), language_identifier, get_languages);
    
    rsx! {
        div {
            class: "min-h-screen text-zinc-200 bg-zinc-800 pt-4 pb-36",
            Router::<Route> {}
        }
    }
}
