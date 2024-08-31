use dioxus::prelude::*;
use crate::components::task_list::TaskList;
use crate::models::category::Category;
use crate::route::Route;

#[component]
pub(crate) fn StickyBottom(children: Element) -> Element {
    rsx! {
        div {
            class: "fixed bottom-0 left-0 right-0 flex flex-col",
            {children}
        }
    }
}
