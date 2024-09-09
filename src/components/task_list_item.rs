use chrono::{Datelike, Local};
use crate::models::category::Category;
use crate::models::task::TaskWithSubtasks;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn TaskListItem(task: TaskWithSubtasks) -> Element {
    rsx! {
        div {
            class: "flex flex-col",
            div {
                class: "mt-1 grow font-medium",
                {task.task().title()}
            },
            div {
                class: "flex flex-row gap-4",
                if let Some(deadline) = task.task().deadline() {
                    div {
                        class: "text-sm text-zinc-400",
                        i {
                            class: "fa-solid fa-bomb"
                        },
                        {deadline.format(if deadline.year() == Local::now().year() {
                            " %m. %-d."
                        } else {
                            " %m. %-d. %Y"
                        }).to_string()}
                    }
                }
                if let Category::Calendar { time, .. } = task.task().category() {
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
                if !task.subtasks().is_empty() {
                    div {
                        class: "text-sm text-zinc-400",
                        i {
                            class: "fa-solid fa-list-check"
                        },
                        {format!(
                            " {}/{}",
                            task.subtasks().iter()
                                .filter(|subtask| subtask.is_completed())
                                .count(),
                            task.subtasks().len()
                        )}
                    }
                }
            }
        }
    }
}
