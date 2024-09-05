use crate::models::category::ReoccurrenceInterval;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn ReoccurrenceIntervalInput(
    reoccurrence_interval: Signal<Option<ReoccurrenceInterval>>,
    class_buttons: Option<&'static str>
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: format!(
                "py-2 rounded-lg {} {}",
                class_buttons.unwrap_or(""),
                if reoccurrence_interval().is_none() { "bg-zinc-500/50" }
                else { "bg-zinc-800/50" }
            ),
            onclick: move |_| {
                reoccurrence_interval.set(None);
            },
            i {
                class: "fa-solid fa-ban"
            }
        },
        button {
            r#type: "button",
            class: format!(
                "py-2 rounded-lg {} {}",
                class_buttons.unwrap_or(""),
                if let Some(ReoccurrenceInterval::Day) = reoccurrence_interval()
                { "bg-zinc-500/50" }
                else { "bg-zinc-800/50" }
            ),
            onclick: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Day))
            },
            i {
                class: "fa-solid fa-sun"
            }
        },
        button {
            r#type: "button",
            class: format!(
                "py-2 rounded-lg {} {}",
                class_buttons.unwrap_or(""),
                if let Some(ReoccurrenceInterval::Month) = reoccurrence_interval()
                { "bg-zinc-500/50" }
                else { "bg-zinc-800/50" }
            ),
            onclick: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Month))
            },
            i {
                class: "fa-solid fa-moon"
            }
        },
        button {
            r#type: "button",
            class: format!(
                "py-2 rounded-lg {} {}",
                class_buttons.unwrap_or(""),
                if let Some(ReoccurrenceInterval::Year) = reoccurrence_interval()
                { "bg-zinc-500/50" }
                else { "bg-zinc-800/50" }
            ),
            onclick: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Year))
            },
            i {
                class: "fa-solid fa-earth-europe"
            }
        }
    }
}
