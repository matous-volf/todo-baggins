use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn App() -> Element {
    rsx! {
        div {
            class: "min-h-screen text-white bg-neutral-800",
            Router::<Route> {}
        }
    }
}
