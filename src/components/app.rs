use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_query::prelude::{use_init_query_client};
use crate::query::{QueryErrors, QueryKey, QueryValue};

#[component]
pub(crate) fn App() -> Element {
    use_init_query_client::<QueryValue, QueryErrors, QueryKey>();
    
    rsx! {
        div {
            class: "min-h-screen text-zinc-200 bg-zinc-800 pt-4 pb-36",
            Router::<Route> {}
        }
    }
}
