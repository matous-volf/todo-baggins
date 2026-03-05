use crate::components::task_list::TaskList;
use crate::hooks::use_tasks_with_subtasks_in_category;
use crate::internationalization::LocaleFromLanguageIdentifier;
use crate::models::category::Category;
use crate::models::task::{Task, TaskWithSubtasks};
use crate::route::Route;
use chrono::{Local, NaiveDateTime};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaCalendarCheck, FaCalendarXmark, FaWater};
use dioxus_i18n::t;
use dioxus_i18n::use_i18n::i18n;
use voca_rs::Voca;

#[component]
pub(crate) fn CategoryTodayTaskList() -> Element {
    let today_date = Local::now().date_naive();
    let calendar_tasks = use_tasks_with_subtasks_in_category(Category::Calendar {
        date: today_date,
        reoccurrence: None,
        time: None,
    })?;
    let today_tasks = calendar_tasks
        .iter()
        .filter(|task| {
            if let Category::Calendar { date, .. } = task.task.category {
                date == today_date
            } else {
                panic!("Unexpected category.");
            }
        })
        .cloned()
        .collect::<Vec<TaskWithSubtasks>>();
    let overdue_tasks = calendar_tasks
        .iter()
        .filter(|task| {
            if let Category::Calendar { date, .. } = task.task.category {
                date < today_date
            } else {
                panic!("Unexpected category.");
            }
        })
        .cloned()
        .collect::<Vec<TaskWithSubtasks>>();
    let long_term_tasks = use_tasks_with_subtasks_in_category(Category::LongTerm)?;
    let inbox_tasks = use_tasks_with_subtasks_in_category(Category::Inbox)?;

    rsx! {
        div {
            class: "pt-4 flex flex-col gap-8",
            if !long_term_tasks.is_empty() {
                div {
                    class: "flex flex-col gap-4",
                    div {
                        class: "px-7 sm:px-8 flex flex-row items-center gap-2 text-gray-500 font-bold",
                        Icon {
                            class: "mx-1.5",
                            icon: FaWater
                        }
                        div {
                            {t!("long-term")._upper_first()}
                        }
                    }
                    TaskList {
                        tasks: long_term_tasks
                    }
                }
            }
            if !overdue_tasks.is_empty() {
                div {
                    class: "flex flex-col gap-4",
                    div {
                        class: "px-7 sm:px-8 flex flex-row items-center gap-2 text-gray-500 font-bold",
                        Icon {
                            class: "mx-1.25",
                            height: 22,
                            width: 22,
                            icon: FaCalendarXmark
                        }
                        div {
                            {t!("overdue")._upper_first()}
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
                    class: "px-7 sm:px-8 flex flex-row items-center gap-2 text-gray-500 font-bold",
                    Icon {
                        class: "mx-1.25",
                        height: 22,
                        width: 22,
                        icon: FaCalendarCheck
                    }
                    div {
                        {
                            let format = t!("date-weekday-format");
                            let today_date = today_date.format_localized(
                                format.as_str(),
                                LocaleFromLanguageIdentifier::from(
                                    &i18n().language()
                                ).into()
                            ).to_string();
                            format!(
                                "{} – {}",
                                t!("today")._upper_first(),
                                if t!("weekday-lowercase-first").parse().unwrap() {
                                    today_date._lower_first()
                                } else {
                                    today_date
                                }
                            )
                        }
                    }
                }
                div {
                    TaskList {
                        tasks: today_tasks
                    }
                    if !inbox_tasks.is_empty() {
                        Link {
                            to: Route::CategoryInboxPage {},
                            TaskList {
                                is_interactive: false,
                                tasks: vec![
                                    TaskWithSubtasks {
                                        task: Task {
                                            id: 0,
                                            title: t!("empty-inbox")._upper_first(),
                                            deadline: None,
                                            category: Category::Inbox,
                                            project_id: None,
                                            created_at: NaiveDateTime::default(),
                                            updated_at: NaiveDateTime::default()
                                        },
                                        subtasks: Vec::new()
                                    }
                                ]
                            }
                        }
                    }
                }
            }
        }
    }
}
