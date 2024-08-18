use crate::components::project_form::ProjectForm;
use crate::server::projects::create_project;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        ProjectForm {
            onsubmit: move |title| {
                spawn(async move {
                    create_project(title).await;
                });
            }
        }
    }
}
