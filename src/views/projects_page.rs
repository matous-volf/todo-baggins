use crate::components::{error_boundary_message::ErrorBoundaryMessage, project_list::ProjectList};
use dioxus::prelude::*;

#[component]
pub(crate) fn ProjectsPage() -> Element {
    rsx! {
        ErrorBoundaryMessage {
            ProjectList {}
        }
    }
}
