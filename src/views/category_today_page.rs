use crate::components::category_today_task_list::CategoryTodayTaskList;
use crate::components::error_boundary_message::ErrorBoundaryMessage;
use dioxus::prelude::*;

#[component]
pub(crate) fn CategoryTodayPage() -> Element {
    rsx! {
        ErrorBoundaryMessage {
            CategoryTodayTaskList {}
        }
    }
}
