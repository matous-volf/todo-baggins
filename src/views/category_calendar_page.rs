use crate::components::category_calendar_task_list::CategoryCalendarTaskList;
use crate::components::error_boundary_message::ErrorBoundaryMessage;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn CategoryCalendarPage() -> Element {
    rsx! {
        ErrorBoundaryMessage {
            CategoryCalendarTaskList {}
        }
    }
}
