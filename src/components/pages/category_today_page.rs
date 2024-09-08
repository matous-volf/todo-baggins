use crate::components::task_list::TaskList;
use crate::models::category::Category;
use crate::models::task::TaskWithSubtasks;
use crate::query::tasks::{use_tasks_with_subtasks_in_category_query};
use crate::query::QueryValue;
use chrono::{Local, Locale};
use dioxus::prelude::*;
use dioxus_query::prelude::QueryResult;

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

    rsx! {
        div {
            class: "pt-4 flex flex-col gap-8",
            match long_term_tasks_query_result.value() {
                QueryResult::Ok(QueryValue::TasksWithSubtasks(tasks))
                | QueryResult::Loading(Some(QueryValue::TasksWithSubtasks(tasks))) => rsx! {
                    div {
                        class: "flex flex-col gap-4",
                        div {
                            class: "px-8 flex flex-row items-center gap-2 font-bold",
                            i {
                                class: "fa-solid fa-water text-xl w-6 text-center"
                            }
                            div {
                                class: "mt-1",
                                "Long-term"
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
                                    div {
                                        class: "flex flex-col",
                                        div {
                                            class: "mt grow font-medium",
                                            {task.task().title()}
                                        },
                                        div {
                                            class: "flex flex-row gap-3",
                                            if let Some(deadline) = task.task().deadline() {
                                                div {
                                                    class: "text-sm text-zinc-400",
                                                    i {
                                                        class: "fa-solid fa-bomb"
                                                    },
                                                    {deadline.format(" %m. %d.").to_string()}
                                                }
                                            }
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
                                        "Overdue"
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
                                        today_date
                                        .format_localized("Today, %A %-d. %B", Locale::en_US)
                                        .to_string()
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
