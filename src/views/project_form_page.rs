use crate::components::{error_boundary_message::ErrorBoundaryMessage, project_form::ProjectForm};
use dioxus::prelude::*;

#[component]
pub(crate) fn ProjectFormPage() -> Element {
    rsx! {
        ErrorBoundaryMessage {
            class: "grow py-8 max-w-xl w-full mx-auto flex flex-col gap-12",
            ProjectForm {}
        }
    }
}
