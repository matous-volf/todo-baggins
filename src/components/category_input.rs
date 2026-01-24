use crate::models::category::Category;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{
    FaCalendarDays, FaForward, FaHourglassHalf, FaInbox, FaQuestion, FaWater,
};

#[component]
pub(crate) fn CategoryInput(
    selected_category: Signal<Category>,
    class: Option<&'static str>,
) -> Element {
    rsx! {
        div {
            class: format!("flex flex-row gap-2 {}", class.unwrap_or("")),
            button {
                r#type: "button",
                class: format!(
                    "py-3 flex flex-row justify-center items-center rounded-lg grow basis-0 {} cursor-pointer",
                    if selected_category() == Category::SomedayMaybe { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::SomedayMaybe);
                },
                Icon {
                    icon: FaQuestion,
                    height: 16,
                    width: 16
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-3 flex flex-row justify-center items-center rounded-lg grow basis-0 {} cursor-pointer",
                    if selected_category() == Category::LongTerm { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::LongTerm);
                },
                Icon {
                    icon: FaWater,
                    height: 16,
                    width: 16
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-3 flex flex-row justify-center items-center rounded-lg grow basis-0 {} cursor-pointer",
                    if let Category::WaitingFor(_) = selected_category() { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::WaitingFor(String::new()));
                },
                Icon {
                    icon: FaHourglassHalf,
                    height: 16,
                    width: 16
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-3 flex flex-row justify-center items-center rounded-lg grow basis-0 {} cursor-pointer",
                    if selected_category() == Category::NextSteps { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::NextSteps);
                },
                Icon {
                    icon: FaForward,
                    height: 16,
                    width: 16
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-3 flex flex-row justify-center items-center rounded-lg grow basis-0 {} cursor-pointer",
                    if let Category::Calendar { .. } = selected_category() { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::Calendar {
                        date: chrono::Local::now().date_naive(),
                        reoccurrence: None,
                        time: None,
                    });
                },
                Icon {
                    icon: FaCalendarDays,
                    height: 16,
                    width: 16
                }
            },
            button {
                r#type: "button",
                class: format!(
                    "py-3 flex flex-row justify-center items-center rounded-lg grow basis-0 {} cursor-pointer",
                    if selected_category() == Category::Inbox { "bg-zinc-500/50" }
                    else { "bg-zinc-800/50" }
                ),
                onclick: move |_| {
                    selected_category.set(Category::Inbox);
                },
                Icon {
                    icon: FaInbox,
                    height: 16,
                    width: 16
                }
            }
        }
    }
}
