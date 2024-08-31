use crate::components::bottom_panel::BottomPanel;
use crate::components::create_task_button::CreateTaskButton;
use crate::components::navigation::Navigation;
use crate::components::sticky_bottom::StickyBottom;
use crate::components::task_form::TaskForm;
use crate::components::task_list::TaskList;
use crate::models::category::Category;
use crate::models::task::Task;
use crate::route::Route;
use crate::schema::tasks::category;
use crate::server::tasks::get_tasks_in_category;
use chrono::{Local, NaiveDate};
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn CategoryTodayPage() -> Element {
    let tasks = use_server_future(
        move || get_tasks_in_category(Category::Calendar {
            date: NaiveDate::default(),
            reoccurrence: None,
            time: None,
        })
    )?.unwrap().unwrap().iter().filter(|task| {
        if let Category::Calendar { date, .. } = task.category() {
            *date == Local::now().date_naive()
        } else {
            panic!("Unexpected category.");
        }
    }).cloned().collect::<Vec<Task>>();

    rsx! {
        TaskList {
            tasks: tasks,
            class: "pb-36"
        }
    }
}
