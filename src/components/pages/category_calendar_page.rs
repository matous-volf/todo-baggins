use crate::models::category::Category;
use chrono::{Datelike, Local, Locale};
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_query::prelude::QueryResult;
use crate::components::task_list::TaskList;
use crate::query::QueryValue;
use crate::query::tasks::use_tasks_in_category_query;
use crate::models::task::Task;

const CALENDAR_LENGTH_DAYS: usize = 366 * 3;

#[component]
pub(crate) fn CategoryCalendarPage() -> Element {
    let tasks = use_tasks_in_category_query(Category::Calendar {
        date: Local::now().date_naive(),
        reoccurrence: None,
        time: None,
    });
    let tasks_query_result = tasks.result();

    rsx! {
        match tasks_query_result.value() {
            QueryResult::Ok(QueryValue::Tasks(tasks))
            | QueryResult::Loading(Some(QueryValue::Tasks(tasks))) => {
                let today_date = Local::now().date_naive();
                
                rsx! {
                    div {
                        class: "pt-4 flex flex-col gap-8",
                        for date_current in today_date.iter_days().take(CALENDAR_LENGTH_DAYS) {
                            div {
                                class: "flex flex-col gap-4",
                                div {
                                    class: "px-8 flex flex-row items-center gap-2 font-bold",
                                    div {
                                        class: "pt-1",
                                        {
                                            date_current
                                            .format_localized(
                                                format!(
                                                    "%A %-d. %B{}", 
                                                    if date_current.year() != today_date.year()
                                                    {" %Y"} else {""}
                                                ).as_str(),
                                                Locale::en_US
                                            )
                                            .to_string()
                                        }
                                    }
                                }
                                TaskList {
                                    tasks: tasks.iter().filter(|task| {
                                        if let Category::Calendar { date, .. } = task.category() {
                                            *date == date_current
                                        } else {
                                            panic!("Unexpected category.");
                                        }
                                    }).cloned().collect::<Vec<Task>>()
                                }
                            }
                        }
                    }   
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
}
