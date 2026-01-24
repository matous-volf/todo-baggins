use crate::models::category::ReoccurrenceInterval;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaBan, FaEarthEurope, FaMoon, FaSun};

#[component]
pub(crate) fn ReoccurrenceIntervalInput(
    reoccurrence_interval: Signal<Option<ReoccurrenceInterval>>,
    class_buttons: Option<&'static str>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: format!(
                "py-2 flex flex-row justify-center items-center rounded-lg {} {} cursor-pointer",
                class_buttons.unwrap_or(""),
                if reoccurrence_interval().is_none() { "bg-zinc-500/50" }
                else { "bg-zinc-800/50" }
            ),
            onclick: move |_| {
                reoccurrence_interval.set(None);
            },
            Icon {
                icon: FaBan,
                height: 16,
                width: 16
            }
        },
        button {
            r#type: "button",
            class: format!(
                "py-2 flex flex-row justify-center items-center rounded-lg {} {} cursor-pointer",
                class_buttons.unwrap_or(""),
                if let Some(ReoccurrenceInterval::Day) = reoccurrence_interval()
                { "bg-zinc-500/50" }
                else { "bg-zinc-800/50" }
            ),
            onclick: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Day))
            },
            Icon {
                icon: FaSun,
                height: 16,
                width: 16
            }
        },
        button {
            r#type: "button",
            class: format!(
                "py-2 flex flex-row justify-center items-center rounded-lg {} {} cursor-pointer",
                class_buttons.unwrap_or(""),
                if let Some(ReoccurrenceInterval::Month) = reoccurrence_interval()
                { "bg-zinc-500/50" }
                else { "bg-zinc-800/50" }
            ),
            onclick: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Month))
            },
            Icon {
                icon: FaMoon,
                height: 16,
                width: 16
            }
        },
        button {
            r#type: "button",
            class: format!(
                "py-2 flex flex-row justify-center items-center rounded-lg {} {} cursor-pointer",
                class_buttons.unwrap_or(""),
                if let Some(ReoccurrenceInterval::Year) = reoccurrence_interval()
                { "bg-zinc-500/50" }
                else { "bg-zinc-800/50" }
            ),
            onclick: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Year))
            },
            Icon {
                icon: FaEarthEurope,
                height: 16,
                width: 16
            }
        }
    }
}
