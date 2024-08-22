use crate::components::project_form::ProjectForm;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::task_form::TaskForm;

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        ProjectForm {}
        TaskForm {}
    }
}
