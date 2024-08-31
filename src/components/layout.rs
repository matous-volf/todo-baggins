use crate::components::bottom_panel::BottomPanel;
use crate::components::navigation::Navigation;
use crate::components::task_list::TaskList;
use crate::models::category::Category;
use crate::route::Route;
use chrono::NaiveDate;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::create_task_button::CreateTaskButton;
use crate::components::sticky_bottom::StickyBottom;
use crate::components::task_form::TaskForm;
use crate::server::tasks::get_tasks_in_category;

#[component]
pub(crate) fn Layout() -> Element {
    let creating_task = use_signal(|| false);
    
    rsx! {
        Outlet::<Route> {}
        StickyBottom {
            CreateTaskButton {
                creating: creating_task,
            }
            BottomPanel {
                creating_task: creating_task(),
            }
        }
    }
}
