use crate::components::task_list::TaskList;
use crate::models::category::Category;
use crate::query::tasks::use_tasks_with_subtasks_in_category_query;
use crate::query::QueryValue;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_query::prelude::QueryResult;

#[component]
pub(crate) fn CategoryPage(category: Category) -> Element {
    let tasks_query = use_tasks_with_subtasks_in_category_query(category);
    let tasks_query_result = tasks_query.result();

    match tasks_query_result.value() {
        QueryResult::Ok(QueryValue::TasksWithSubtasks(tasks))
        | QueryResult::Loading(Some(QueryValue::TasksWithSubtasks(tasks))) => rsx! {
            TaskList {
                tasks: tasks.clone(),
                class: "pb-36"
            }
        },
        QueryResult::Loading(None) => rsx! {
            // TODO: Add a loading indicator.
        },
        QueryResult::Err(errors) => rsx! {
            div {
                "Errors occurred: {errors:?}"
            }
        },
        value => panic!("Unexpected query result: {value:?}")
    }
}
