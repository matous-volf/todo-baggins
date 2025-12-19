use crate::components::task_list::TaskList;
use crate::hooks::use_tasks_with_subtasks_in_category;
use crate::models::category::Category;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn CategoryPage(category: Category) -> Element {
    let tasks = use_tasks_with_subtasks_in_category(category)?;

    rsx! {
        TaskList {
            tasks: tasks.clone(),
            class: "pb-36"
        }
    }
}
