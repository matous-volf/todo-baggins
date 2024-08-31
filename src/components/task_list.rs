use crate::models::category::Category;
use crate::models::task::Task;
use crate::server::tasks::get_tasks_in_category;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn TaskList(tasks: Vec<Task>, class: Option<&'static str>) -> Element {
    rsx! {
        div {
            class: format!("pt-3 px-8 flex flex-col {}", class.unwrap_or("")),
            for task in tasks {
                div {
                    class: format!(
                        "pt-5 {} flex flex-row gap-4",
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
                        }
                    ),
                    i {
                        class: "fa-regular fa-square text-3xl text-zinc-600",
                    },
                    div {
                        class: "flex flex-col",
                        div {
                            class: "mt-1 grow",
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
