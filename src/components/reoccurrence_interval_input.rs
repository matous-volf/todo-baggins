use crate::components::select_button::SelectButton;
use crate::models::category::ReoccurrenceInterval;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::{FaBan, FaEarthEurope, FaMoon, FaSun};

#[component]
pub(crate) fn ReoccurrenceIntervalInput(
    reoccurrence_interval: Signal<Option<ReoccurrenceInterval>>,
    class_buttons: Option<&'static str>,
) -> Element {
    rsx! {
        SelectButton {
            icon: FaBan,
            is_selected: reoccurrence_interval().is_none(),
            on_select: move |_| {
                reoccurrence_interval.set(None);
            }
        }
        SelectButton {
            icon: FaSun,
            is_selected: matches!(reoccurrence_interval(), Some(ReoccurrenceInterval::Day)),
            on_select: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Day))
            }
        }
        SelectButton {
            icon: FaMoon,
            is_selected: matches!(reoccurrence_interval(), Some(ReoccurrenceInterval::Month)),
            on_select: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Month));
            }
        }
        SelectButton {
            icon: FaEarthEurope,
            is_selected: matches!(reoccurrence_interval(), Some(ReoccurrenceInterval::Year)),
            on_select: move |_| {
                reoccurrence_interval.set(Some(ReoccurrenceInterval::Year));
            }
        }
    }
}
