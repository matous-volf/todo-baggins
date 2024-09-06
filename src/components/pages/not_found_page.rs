use dioxus::prelude::*;

#[component]
pub(crate) fn NotFoundPage(route: Vec<String>) -> Element {
    rsx! {
        {"404"}
    }
}
