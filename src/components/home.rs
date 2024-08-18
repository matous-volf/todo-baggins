use crate::components::project_form::ProjectForm;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        ProjectForm {}
    }
}
