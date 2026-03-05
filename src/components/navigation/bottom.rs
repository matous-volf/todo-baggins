use crate::route::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaLightbulb;
use dioxus_free_icons::icons::fa_solid_icons::{
    FaBars, FaCalendarDay, FaCalendarDays, FaHourglassHalf, FaInbox, FaList, FaSignsPost,
    FaTrashCan, FaVolcano,
};

#[component]
pub(crate) fn Bottom(is_expanded: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-5 justify-stretch",
            button {
                class: format!(
                    "py-2 flex flex-row justify-center items-center sm:hidden cursor-pointer",
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
            super::Item {
                route: Route::CategoryNextStepsPage,
                icon: FaSignsPost
            },
            super::Item {
                route: Route::CategoryCalendarPage,
                icon: FaCalendarDays
            },
            super::Item {
                route: Route::CategoryTodayPage,
                icon: FaCalendarDay
            },
            super::Item {
                route: Route::CategoryInboxPage,
                icon: FaInbox
            },
            {if is_expanded() {
                rsx! {
                    super::Item {
                        route: Route::ProjectsPage,
                        icon: FaList
                    },
                    super::Item {
                        route: Route::CategoryTrashPage,
                        icon: FaTrashCan
                    },
                    super::Item {
                        route: Route::CategoryDonePage,
                        icon: FaVolcano
                    },
                    super::Item {
                        route: Route::CategorySomedayMaybePage,
                        icon: FaLightbulb
                    },
                    super::Item {
                        route: Route::CategoryWaitingForPage,
                        icon: FaHourglassHalf
                    }
                }
            } else { VNode::empty() }}
        }
    }
}
