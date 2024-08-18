use crate::models::project::NewProject;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn ProjectForm(onsubmit: EventHandler<NewProject>) -> Element {
    rsx! {
        form {
            onsubmit: move |event| {
                onsubmit(NewProject::new(event.values().get("title").unwrap().as_value()));
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
