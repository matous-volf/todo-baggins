use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
