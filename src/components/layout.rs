use crate::components::bottom_panel::BottomPanel;
use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::form_open_button::FormOpenButton;
use crate::components::sticky_bottom::StickyBottom;
use crate::models::project::Project;

#[component]
pub(crate) fn Layout() -> Element {
    let mut display_form = use_signal(|| false);
    let project_being_edited = use_context_provider::<Signal<Option<Project>>>(
        || Signal::new(None)
    );
    
    use_effect(move || {
        display_form.set(project_being_edited().is_some());
    });
    
    rsx! {
        Outlet::<Route> {}
        StickyBottom {
            FormOpenButton {
                opened: display_form,
            }
            BottomPanel {
                display_form: display_form,
            }
        }
    }
}
