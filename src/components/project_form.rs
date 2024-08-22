use crate::models::project::NewProject;
use crate::server::projects::create_project;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn ProjectForm() -> Element {
    rsx! {
        form {
            onsubmit: move |event| {
                async move {
                    let new_project = NewProject::new(
                        event.values().get("title").unwrap().as_value()
                    );
                    let _ = create_project(new_project).await;
                }
            },
            input {
                r#type: "text",
                name: "title",
                required: true,
                placeholder: "title"
            }
            button {
                r#type: "submit",
                "create"
            }
        }
    }
}
