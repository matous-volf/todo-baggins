use crate::components::bottom_panel::BottomPanel;
use crate::components::form_open_button::FormOpenButton;
use crate::components::sticky_bottom::StickyBottom;
use crate::models::project::Project;
use crate::models::task::Task;
use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaCog;

#[component]
pub(crate) fn Main() -> Element {
    let mut display_form = use_signal(|| false);
    let project_being_edited =
        use_context_provider::<Signal<Option<Project>>>(|| Signal::new(None));
    let task_being_edited = use_context_provider::<Signal<Option<Task>>>(|| Signal::new(None));

    use_effect(move || {
        display_form.set(project_being_edited().is_some() || task_being_edited().is_some());
    });

    rsx! {
        SuspenseBoundary {
            fallback: |_| {
                rsx! {
                    div {
                        class: "grow flex flex-col justify-center items-center",
                        Icon {
                            class: "animate-[spin_2000ms_linear_infinite]",
                            icon: FaCog,
                            height: 32,
                            width: 32
                        }
                    }
                }
            },
            Outlet::<Route> {}
        }
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
