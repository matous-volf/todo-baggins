use crate::components::navigation_item::NavigationItem;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Navigation(expanded: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-5 justify-stretch",
            button {
                class: format!(
                    "py-4 text-center text-2xl {}",
                    if expanded() { "text-zinc-200" }
                    else { "text-zinc-500" }
                ),
                onclick: move |_| expanded.set(!expanded()),
                i {
                    class: "fa-solid fa-bars"
                }
            },
            NavigationItem {
                route: Route::CategoryNextStepsPage,
                i {
                    class: "fa-solid fa-forward"
                }
            },
            NavigationItem {
                route: Route::CategoryCalendarPage,
                i {
                    class: "fa-solid fa-calendar-days"
                }
            },
            NavigationItem {
                route: Route::CategoryTodayPage,
                i {
                    class: "fa-solid fa-calendar-day"
                }
            },
            NavigationItem {
                route: Route::CategoryInboxPage,
                i {
                    class: "fa-solid fa-inbox"
                }
            },
            {if expanded() {
                rsx! {
                    NavigationItem {
                        route: Route::ProjectsPage,
                        i {
                            class: "fa-solid fa-list"
                        }
                    },
                    NavigationItem {
                        route: Route::CategoryTrashPage,
                        i {
                            class: "fa-solid fa-trash-can"
                        }
                    },
                    NavigationItem {
                        route: Route::CategoryDonePage,
                        i {
                            class: "fa-solid fa-check"
                        }
                    },
                    NavigationItem {
                        route: Route::CategoryLongTermPage,
                        i {
                            class: "fa-solid fa-water"
                        }
                    },
                    NavigationItem {
                        route: Route::CategoryWaitingForPage,
                        i {
                            class: "fa-solid fa-hourglass-half"
                        }
                    }
                }
            } else { None }}
        }
    }
}
