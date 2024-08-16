use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn FormProject(onsubmit: EventHandler<String>) -> Element {
    rsx! {
        form {
            onsubmit: move |event| {
                onsubmit(event.values().get("title").unwrap().as_value());
            },
            input {
                r#type: "text",
                name: "title",
                placeholder: "title"
            }
            button {
                r#type: "submit",
                "create"
            }
        }
    }
}
