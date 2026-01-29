use crate::components::select_button::SelectButton;
use crate::models::category::Category;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaLightbulb;
use dioxus_free_icons::icons::fa_solid_icons::{
    FaCalendarDays, FaHourglassHalf, FaInbox, FaSignsPost, FaWater,
};

#[component]
pub(crate) fn CategoryInput(
    selected_category: Signal<Category>,
    class: Option<&'static str>,
) -> Element {
    rsx! {
        div {
            class: format!("grid grid-cols-3 gap-3 {}", class.unwrap_or("")),
            SelectButton {
                icon: FaLightbulb,
                is_selected: matches!(selected_category(), Category::SomedayMaybe),
                on_select: move |_| {
                    selected_category.set(Category::SomedayMaybe);
                }
            }
            SelectButton {
                icon: FaWater,
                is_selected: matches!(selected_category(), Category::LongTerm),
                on_select: move |_| {
                    selected_category.set(Category::LongTerm);
                }
            }
            SelectButton {
                icon: FaHourglassHalf,
                is_selected: matches!(selected_category(), Category::WaitingFor(_)),
                on_select: move |_| {
                    selected_category.set(Category::WaitingFor(String::new()));
                }
            }
            SelectButton {
                icon: FaSignsPost,
                is_selected: matches!(selected_category(), Category::NextSteps),
                on_select: move |_| {
                    selected_category.set(Category::NextSteps);
                }
            }
            SelectButton {
                icon: FaCalendarDays,
                is_selected: matches!(selected_category(), Category::Calendar { .. }),
                on_select: move |_| {
                    selected_category.set(Category::Calendar {
                        date: chrono::Local::now().date_naive(),
                        reoccurrence: None,
                        time: None,
                    });
                }
            }
            SelectButton {
                icon: FaInbox,
                is_selected: matches!(selected_category(), Category::Inbox),
                on_select: move |_| {
                    selected_category.set(Category::Inbox);
                }
            }
        }
    }
}
