use crate::components::{error_boundary_message::ErrorBoundaryMessage, task_form::TaskForm};
use dioxus::prelude::*;

#[component]
pub(crate) fn TaskFormPage() -> Element {
    rsx! {
        ErrorBoundaryMessage {
            class: "grow py-4 flex flex-col gap-12",
            TaskForm {}
        }
    }
}
