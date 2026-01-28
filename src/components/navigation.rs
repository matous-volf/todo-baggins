use crate::components::navigation_item::NavigationItem;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaLightbulb;
use dioxus_free_icons::icons::fa_solid_icons::{
    FaBars, FaCalendarDay, FaCalendarDays, FaHourglassHalf, FaInbox, FaList, FaSignsPost,
    FaTrashCan, FaVolcano,
};

#[component]
pub(crate) fn Navigation(is_expanded: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-5 justify-stretch",
            button {
                class: format!(
                    "py-2 flex flex-row justify-center items-center cursor-pointer",
                ),
                onclick: move |_| is_expanded.set(!is_expanded()),
                div {
                    class: format!("pt-2.5 px-4 {} transition-all duration-150",
                        if is_expanded() { "pb-2 mt-1 bg-gray-900 text-gray-400 rounded-xl drop-shadow-[0_calc(0px_-_var(--spacing))_0_var(--color-gray-950)]" }
                        else { "pb-3 bg-gray-800 rounded-xl drop-shadow-[0_0_0_var(--color-gray-950)] text-gray-600" }
                    ),
                    Icon {
                        icon: FaBars,
                        height: 24,
                        width: 24
                    }
                }
            },
            NavigationItem {
                route: Route::CategoryNextStepsPage,
                icon: FaSignsPost
            },
            NavigationItem {
                route: Route::CategoryCalendarPage,
                icon: FaCalendarDays
            },
            NavigationItem {
                route: Route::CategoryTodayPage,
                icon: FaCalendarDay
            },
            NavigationItem {
                route: Route::CategoryInboxPage,
                icon: FaInbox
            },
            {if is_expanded() {
                rsx! {
                    NavigationItem {
                        route: Route::ProjectsPage,
                        icon: FaList
                    },
                    NavigationItem {
                        route: Route::CategoryTrashPage,
                        icon: FaTrashCan
                    },
                    NavigationItem {
                        route: Route::CategoryDonePage,
                        icon: FaVolcano
                    },
                    NavigationItem {
                        route: Route::CategorySomedayMaybePage,
                        icon: FaLightbulb
                    },
                    NavigationItem {
                        route: Route::CategoryWaitingForPage,
                        icon: FaHourglassHalf
                    }
                }
            } else { VNode::empty() }}
        }
    }
}
