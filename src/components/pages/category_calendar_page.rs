use crate::components::task_list::TaskList;
use crate::internationalization::LocaleFromLanguageIdentifier;
use crate::models::category::Category;
use crate::models::task::TaskWithSubtasks;
use crate::query::QueryValue;
use crate::query::tasks::use_tasks_with_subtasks_in_category_query;
use chrono::{Datelike, Local};
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::t;
use dioxus_query::prelude::QueryResult;

const CALENDAR_LENGTH_DAYS: usize = 366 * 3;

#[component]
pub(crate) fn CategoryCalendarPage() -> Element {
    let tasks = use_tasks_with_subtasks_in_category_query(Category::Calendar {
        date: Local::now().date_naive(),
        reoccurrence: None,
        time: None,
    });
    let tasks_query_result = tasks.result();

    rsx! {
        match tasks_query_result.value() {
            QueryResult::Ok(QueryValue::TasksWithSubtasks(tasks))
            | QueryResult::Loading(Some(QueryValue::TasksWithSubtasks(tasks))) => {
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
                                            date_current.format_localized(t!(
                                                    if date_current.year() == Local::now().year() {
                                                        "date-weekday-format"
                                                    } else {
                                                        "date-weekday-year-format"
                                                    }
                                                ).as_str(),
                                                LocaleFromLanguageIdentifier::from(
                                                    &i18n().language()
                                                ).into()
                                            )
                                            .to_string()
                                        }
                                    }
                                }
                                TaskList {
                                    tasks: tasks.iter().filter(|task| {
                                        if let Category::Calendar { date, .. }
                                            = task.task().category() {
                                            *date == date_current
                                        } else {
                                            panic!("Unexpected category.");
                                        }
                                    }).cloned().collect::<Vec<TaskWithSubtasks>>()
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
