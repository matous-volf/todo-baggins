use crate::components::project_form::PROJECT_BEING_EDITED;
use crate::components::{button_primary::ButtonPrimary, task_form::TASK_BEING_EDITED};
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaGavel};

#[component]
pub(crate) fn CreateButton() -> Element {
    let navigator = use_navigator();
    let current_route = use_route();
    rsx! {
        ButtonPrimary {
            class: "pointer-events-auto sm:self-auto *:rounded-full! *:p-4",
            onclick: move |_| {
                *TASK_BEING_EDITED.write() = None;
                *PROJECT_BEING_EDITED.write() = None;
                navigator.push(
                    match current_route {
                        Route::ProjectsPage => Route::ProjectFormPage,
                        _ => Route::TaskFormPage,
                    }
                );
            },
            Icon {
                icon: FaGavel,
                height: 24,
                width: 24
            }
        }
    }
}
