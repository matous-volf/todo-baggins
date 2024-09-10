use crate::components::task_list::TaskList;
use crate::components::task_list_item::TaskListItem;
use crate::internationalization::LocaleFromLanguageIdentifier;
use crate::models::category::Category;
use crate::models::task::TaskWithSubtasks;
use crate::query::tasks::use_tasks_with_subtasks_in_category_query;
use crate::query::QueryValue;
use chrono::Local;
use dioxus::prelude::*;
use dioxus_query::prelude::QueryResult;
use dioxus_sdk::i18n::use_i18;
use dioxus_sdk::translate;
use voca_rs::Voca;

#[component]
pub(crate) fn CategoryTodayPage() -> Element {
    let today_date = Local::now().date_naive();

    let calendar_tasks_query = use_tasks_with_subtasks_in_category_query(Category::Calendar {
        date: today_date,
        reoccurrence: None,
        time: None,
    });
    let calendar_tasks_query_result = calendar_tasks_query.result();

    let long_term_tasks_query = use_tasks_with_subtasks_in_category_query(Category::LongTerm);
    let long_term_tasks_query_result = long_term_tasks_query.result();

    let i18 = use_i18();

    rsx! {
        div {
            class: "pt-4 flex flex-col gap-8",
            match long_term_tasks_query_result.value() {
                QueryResult::Ok(QueryValue::TasksWithSubtasks(tasks))
                | QueryResult::Loading(Some(QueryValue::TasksWithSubtasks(tasks))) => {
                    let mut tasks = tasks.clone();
                    tasks.sort();
                    rsx! {
                        div {
                            class: "flex flex-col gap-4",
                            div {
                                class: "px-8 flex flex-row items-center gap-2 font-bold",
                                i {
                                    class: "fa-solid fa-water text-xl w-6 text-center"
                                }
                                div {
                                    class: "mt-1",
                                    {translate!(i18, "long_term")._upper_first()}
                                }
                            }
                            div {
                                for task in tasks {
                                    div {
                                        key: "{task.task().id()}",
                                        class: format!(
                                            "px-8 pt-5 {} flex flex-row gap-4",
                                            if task.task().deadline().is_some() {
                                                "pb-0.5"
                                            } else {
                                                "pb-5"
                                            }
                                        ),
                                        TaskListItem {
                                            task: task.clone()
                                        }
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
            match calendar_tasks_query_result.value() {
                QueryResult::Ok(QueryValue::TasksWithSubtasks(tasks))
                | QueryResult::Loading(Some(QueryValue::TasksWithSubtasks(tasks))) => {
                    let today_tasks = tasks.iter().filter(|task| {
                        if let Category::Calendar { date, .. } = task.task().category() {
                            *date == today_date
                        } else {
                            panic!("Unexpected category.");
                        }
                    }).cloned().collect::<Vec<TaskWithSubtasks>>();
                    let overdue_tasks = tasks.iter().filter(|task| {
                        if let Category::Calendar { date, .. } = task.task().category() {
                            *date < today_date
                        } else {
                            panic!("Unexpected category.");
                        }
                    }).cloned().collect::<Vec<TaskWithSubtasks>>();
        
                    rsx! {
                        if !overdue_tasks.is_empty() {
                            div {
                                class: "flex flex-col gap-4",
                                div {
                                    class: "px-8 flex flex-row items-center gap-2 font-bold",
                                    i {
                                        class: "fa-solid fa-calendar-xmark text-xl w-6 text-center"
                                    }
                                    div {
                                        class: "mt-1",
                                        {translate!(i18, "overdue")._upper_first()}
                                    }
                                }
                                TaskList {
                                    tasks: overdue_tasks,
                                    class: "pb-3"
                                }
                            }
                        }
                        div {
                            class: "flex flex-col gap-4",
                            div {
                                class: "px-8 flex flex-row items-center gap-2 font-bold",
                                i {
                                    class: "fa-solid fa-calendar-check text-xl w-6 text-center"
                                }
                                div {
                                    class: "mt-1",
                                    {
                                        let format = translate!(i18, "formats.date_weekday_format");
                                        let today_date = today_date.format_localized(
                                            format.as_str(),
                                            LocaleFromLanguageIdentifier::from(
                                                &(i18.selected_language)()
                                            ).into()
                                        ).to_string();
                                        format!(
                                            "{} – {}",
                                            translate!(i18, "today")._upper_first(),
                                            if translate!(i18, "formats.weekday_lowercase_first")
                                                .parse().unwrap() {
                                                today_date._lower_first()
                                            } else {
                                                today_date
                                            }
                                        )
                                    }
                                }
                            }
                            TaskList {
                                tasks: today_tasks
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
}
