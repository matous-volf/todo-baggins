use crate::components::navigation_item::NavigationItem;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{
    FaBars, FaCalendarDay, FaCalendarDays, FaCheck, FaForward, FaHourglassHalf, FaInbox, FaList,
    FaTrashCan, FaWater,
};

#[component]
pub(crate) fn Navigation(expanded: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-5 justify-stretch",
            button {
                class: format!(
                    "py-5 flex flex-row justify-center items-center {} cursor-pointer",
                    if expanded() { "text-zinc-200" }
                    else { "text-zinc-500" }
                ),
                onclick: move |_| expanded.set(!expanded()),
                Icon {
                    icon: FaBars,
                    height: 24,
                    width: 24
                }
            },
            NavigationItem {
                route: Route::CategoryNextStepsPage,
                icon: FaForward
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
            {if expanded() {
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
                        icon: FaCheck
                    },
                    NavigationItem {
                        route: Route::CategoryLongTermPage,
                        icon: FaWater
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
