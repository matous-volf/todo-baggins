use dioxus::prelude::*;
use crate::components::task_list::TaskList;
use crate::models::category::Category;
use crate::route::Route;

#[component]
pub(crate) fn NotFoundPage(route: Vec<String>) -> Element {
    rsx! {
        {"404"}
    }
}
