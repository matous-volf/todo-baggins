use crate::models::category::Category;
use crate::models::task::Task;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn TaskList(tasks: Vec<Task>, class: Option<&'static str>) -> Element {
    let mut task_being_edited = use_context::<Signal<Option<Task>>>();

    rsx! {
        div {
            class: format!("flex flex-col {}", class.unwrap_or("")),
            for task in tasks {
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
                    onclick: move |_| task_being_edited.set(Some(task.clone())),
                    i {
                        class: "fa-regular fa-square text-3xl text-zinc-600",
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
