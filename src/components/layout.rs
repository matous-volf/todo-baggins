use crate::components::bottom_panel::BottomPanel;
use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::create_task_button::CreateButton;
use crate::components::sticky_bottom::StickyBottom;

#[component]
pub(crate) fn Layout() -> Element {
    let display_form = use_signal(|| false);
    
    rsx! {
        Outlet::<Route> {}
        StickyBottom {
            CreateButton {
                creating: display_form,
            }
            BottomPanel {
                display_form: display_form,
            }
        }
    }
}
