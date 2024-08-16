use crate::components::form_project::FormProject;
use crate::server::projects::create_project;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        FormProject {
            onsubmit: move |title| {
                spawn(async move {
                    let _ = create_project(title).await;
                });
            }
        }
    }
}
