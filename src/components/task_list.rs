use crate::components::task_form::TASK_BEING_EDITED;
use crate::components::task_list_item::TaskListItem;
use crate::models::category::Category;
use crate::models::task::TaskWithSubtasks;
use crate::route::Route;
use crate::server::tasks::complete_task;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn TaskList(
    tasks: Vec<TaskWithSubtasks>,
    /// Whether to open and complete tasks on clicks.
    is_interactive: Option<bool>,
    class: Option<&'static str>,
) -> Element {
    let navigator = use_navigator();
    rsx! {
        div {
            class: format!("flex flex-col {}", class.unwrap_or("")),
            for task in tasks.clone() {
                div {
                    key: "{task.task.id}",
                    class: format!(
                        "px-7 pt-3.75 {} flex flex-row items-start gap-4 hover:bg-gray-800 cursor-pointer select-none transition-all duration-150",
                        if task.task.deadline.is_some() || !task.subtasks.is_empty() {
                            "pb-0.25"
                        } else if let Category::Calendar { time, .. } = &task.task.category {
                            if time.is_some() {
                                "pb-0.25"
                            } else {
                                "pb-3.75"
                            }
                        } else {
                            "pb-3.75"
                        },
                    ),
                    onclick: {
                        let task = task.clone();
                        move |_| {
                            if let Some(false) = is_interactive {
                                return;
                            }
                            *TASK_BEING_EDITED.write() = Some(task.task.clone());
                            navigator.push(Route::TaskFormPage);
                        }
                    },
                    button {
                        class: format!(
                            "mt-0.5 hover:mt-0 hover:pb-0.5 transition-all duration-150 cursor-pointer {}",
                            if let Category::Done = task.task.category { "pointer-events-none" }
                            else { "" }
                        ),
                        onclick: {
                            move |event: Event<MouseData>| {
                                // To prevent editing the task.
                                event.stop_propagation();
                                async move {
                                    if let Some(false) = is_interactive {
                                        return;
                                    }
                                    let _ = complete_task(task.task.id).await;
                                }
                            }
                        },
                        div {
                            class: format!("h-8 w-8 rounded-full {}",
                                if let Category::Done = task.task.category {
                                    "mt-[3px] mb-[2px] bg-amber-300-muted"
                                } else {
                                    "mb-[5px] border-3 border-amber-300-muted drop-shadow-[0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted)]"
                                }
                            )
                        }
                    },
                    div {
                        class: "mt-1.5",
                        TaskListItem {
                            task: task.clone()
                        }
                    }
                }
            }
        }
    }
}
