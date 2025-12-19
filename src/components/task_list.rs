use crate::components::task_list_item::TaskListItem;
use crate::models::category::Category;
use crate::models::task::{Task, TaskWithSubtasks};
use crate::server::tasks::complete_task;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn TaskList(tasks: Vec<TaskWithSubtasks>, class: Option<&'static str>) -> Element {
    let mut task_being_edited = use_context::<Signal<Option<Task>>>();
    rsx! {
        div {
            class: format!("flex flex-col {}", class.unwrap_or("")),
            for task in tasks.clone() {
                div {
                    key: "{task.task.id}",
                    class: format!(
                        "px-8 pt-4 {} flex flex-row gap-4 select-none {}",
                        if task.task.deadline.is_some() || !task.subtasks.is_empty() {
                            "pb-0.5"
                        } else if let Category::Calendar { time, .. } = &task.task.category {
                            if time.is_some() {
                                "pb-0.5"
                            } else {
                                "pb-4"
                            }
                        } else {
                            "pb-4"
                        },
                        if task_being_edited().is_some_and(|t| t.id == task.task.id) {
                            "bg-zinc-700"
                        } else { "" }
                    ),
                    onclick: {
                        let task = task.clone();
                        move |_| task_being_edited.set(Some(task.task.clone()))
                    },
                    i {
                        class: format!(
                            "{} text-3xl align-middle h-9 text-zinc-500",
                            if let Category::Done = task.task.category {
                                "fa solid fa-square-check"
                            } else {
                                "fa-regular fa-square cursor-pointer"
                            }
                        ),
                        onclick: {
                            move |event: Event<MouseData>| {
                                // To prevent editing the task.
                                event.stop_propagation();
                                async move {
                                    let _ = complete_task(task.task.id).await;
                                }
                            }
                        }
                    },
                    TaskListItem {
                        task: task.clone()
                    }
                }
            }
        }
    }
}
