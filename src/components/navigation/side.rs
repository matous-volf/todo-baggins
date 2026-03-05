use crate::route::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaLightbulb;
use dioxus_free_icons::icons::fa_solid_icons::{
    FaCalendarDay, FaCalendarDays, FaHourglassHalf, FaInbox, FaList, FaSignsPost, FaTrashCan,
    FaVolcano,
};

#[component]
pub(crate) fn Side(class: Option<String>) -> Element {
    rsx! {
        div {
            class: format!("flex flex-col {}", class.unwrap_or(String::new())),
            super::Item {
                route: Route::CategoryInboxPage,
                icon: FaInbox
            },
            super::Item {
                route: Route::CategoryTodayPage,
                icon: FaCalendarDay
            },
            super::Item {
                route: Route::CategoryCalendarPage,
                icon: FaCalendarDays
            },
            super::Item {
                route: Route::CategoryNextStepsPage,
                icon: FaSignsPost
            },
            super::Item {
                route: Route::CategoryWaitingForPage,
                icon: FaHourglassHalf
            },
            super::Item {
                route: Route::CategorySomedayMaybePage,
                icon: FaLightbulb
            },
            super::Item {
                route: Route::CategoryDonePage,
                icon: FaVolcano
            },
            super::Item {
                route: Route::CategoryTrashPage,
                icon: FaTrashCan
            },
            super::Item {
                route: Route::ProjectsPage,
                icon: FaList
            }
        }
    }
}
