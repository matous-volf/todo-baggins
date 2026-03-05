use crate::components;
use crate::components::bottom_panel::BottomPanel;
use crate::components::create_button::CreateButton;
use crate::components::sticky_bottom::StickyBottom;
use crate::components::task_form::LATEST_VISITED_CATEGORY;
use crate::models::category::Category;
use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn Navigation() -> Element {
    let current_route = use_route();
    use_effect(use_reactive(&current_route, move |current_route| {
        *LATEST_VISITED_CATEGORY.write() = match current_route {
            Route::CategorySomedayMaybePage => Category::SomedayMaybe,
            Route::CategoryWaitingForPage => Category::WaitingFor(String::new()),
            Route::CategoryNextStepsPage => Category::NextSteps,
            Route::CategoryCalendarPage | Route::CategoryTodayPage => Category::Calendar {
                date: chrono::Local::now().date_naive(),
                reoccurrence: None,
                time: None,
            },
            _ => Category::Inbox,
        };
    }));

    rsx! {
        div {
            class: "grow flex flex-col sm:flex-row pb-36 sm:pb-0",
            div {
                class: "hidden sm:flex sm:flex-col gap-4 py-3.25 items-center bg-gray-800",
                components::navigation::Side {},
                div {
                    CreateButton {}
                }
            }
            div {
                class: "grow flex flex-col py-4",
                Outlet::<Route> {}
            }
        }
        StickyBottom {
            div {
                class: "m-4 self-end",
                CreateButton {},
            }
            BottomPanel {}
        }
    }
}
