use crate::models::category::Category;
use crate::server::tasks::get_tasks_in_category;
use chrono::NaiveDate;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use std::fmt::format;

#[component]
pub(crate) fn CategoryInput(selected_category: Signal<Category>, class: Option<&'static str>) -> Element {
    rsx! {
        div {
            class: format!("flex flex-row gap-2 {}", class.unwrap_or("")),
            button {
                r#type: "button",
                class: format!(
                    "py-2 rounded-lg grow basis-0 {}",
                    if selected_category() == Category::SomedayMaybe { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::SomedayMaybe);
                },
                i {
                    class: "fa-solid fa-question"
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-2 rounded-lg grow basis-0 {}",
                    if selected_category() == Category::LongTerm { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::LongTerm);
                },
                i {
                    class: "fa-solid fa-water"
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-2 rounded-lg grow basis-0 {}",
                    if let Category::WaitingFor(_) = selected_category() { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::WaitingFor(String::new()));
                },
                i {
                    class: "fa-solid fa-hourglass-half"
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-2 rounded-lg grow basis-0 {}",
                    if selected_category() == Category::NextSteps { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::NextSteps);
                },
                i {
                    class: "fa-solid fa-forward"
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-2 rounded-lg grow basis-0 {}",
                    if let Category::Calendar { .. } = selected_category() { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::Calendar {
                        date: NaiveDate::default(),
                        reoccurrence: None,
                        time: None,
                    });
                },
                i {
                    class: "fa-solid fa-calendar-days"
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-2 rounded-lg grow basis-0 {}",
                    if selected_category() == Category::Inbox { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::Inbox);
                },
                i {
                    class: "fa-solid fa-inbox"
                }
            }
        }
    }
}
