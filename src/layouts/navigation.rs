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
            class: "grow flex flex-col pb-36",
            Outlet::<Route> {}
        }
        StickyBottom {
            CreateButton {},
            BottomPanel {}
        }
    }
}
