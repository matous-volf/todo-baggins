use crate::models::category::Category;
use crate::models::task::Task;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_query::prelude::use_query_client;
use crate::query::{QueryErrors, QueryKey, QueryValue};
use crate::server::tasks::complete_task;

#[component]
pub(crate) fn TaskList(tasks: Vec<Task>, class: Option<&'static str>) -> Element {
    let query_client = use_query_client::<QueryValue, QueryErrors, QueryKey>();
    let mut task_being_edited = use_context::<Signal<Option<Task>>>();

    rsx! {
        div {
            class: format!("flex flex-col {}", class.unwrap_or("")),
            for task in tasks.clone() {
                div {
                    key: "{task.id()}",
                    class: format!(
                        "px-8 pt-5 {} flex flex-row gap-4 select-none {}",
                        if task.deadline().is_some() {
                            "pb-0.5"
                        } else if let Category::Calendar { time, .. } = task.category() {
                            if time.is_some() {
                                "pb-0.5"
                            } else {
                                "pb-5"
                            }
                        } else {
                            "pb-5"
                        },
                        if task_being_edited().is_some_and(|t| t.id() == task.id()) {
                            "bg-zinc-700"
                        } else { "" }
                    ),
                    onclick: {
                        let task = task.clone();
                        move |_| task_being_edited.set(Some(task.clone()))
                    },
                    i {
                        class: format!(
                            "{} text-3xl text-zinc-500",
                            if *(task.category()) == Category::Done {
                                "fa solid fa-square-check"
                            } else {
                                "fa-regular fa-square"
                            }
                        ),
                        onclick: {
                            let task = task.clone();
                            move |event| {
                                // To prevent editing the task.
                                event.stop_propagation();
                                let task = task.clone();
                                async move {
                                    let completed_task = complete_task(task.id()).await;
                                    let mut query_keys = vec![
                                        QueryKey::Tasks,
                                        QueryKey::TasksInCategory(
                                            completed_task.unwrap().category().clone()
                                        )
                                    ];
                                    if let Category::Calendar { reoccurrence: Some(_), .. }
                                        = task.category() {
                                        query_keys.push(QueryKey::SubtasksOfTaskId(task.id()));
                                    }
                                    query_client.invalidate_queries(&query_keys);
                                }
                            }
                        }
                    },
                    div {
                        class: "flex flex-col",
                        div {
                            class: "mt-1 grow font-medium",
                            {task.title()}
                        },
                        div {
                            class: "flex flex-row gap-3",
                            if let Some(deadline) = task.deadline() {
                                div {
                                    class: "text-sm text-zinc-400",
                                    i {
                                        class: "fa-solid fa-bomb"
                                    },
                                    {deadline.format(" %m. %d.").to_string()}
                                }
                            }
                            if let Category::Calendar { time, .. } = task.category() {
                                if let Some(calendar_time) = time {
                                    div {
                                        class: "text-sm text-zinc-400",
                                        i {
                                            class: "fa-solid fa-clock"
                                        },
                                        {calendar_time.time().format(" %k:%M").to_string()}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
