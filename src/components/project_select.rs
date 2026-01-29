use crate::hooks::use_projects;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub(crate) fn ProjectSelect(initial_selected_id: Option<i32>) -> Element {
    let projects = use_projects()?;
    rsx! {
        select {
            name: "project_id",
            class: "px-4 pt-3 pb-2.25 bg-gray-800-muted enabled:hover:bg-gray-800 enabled:active:bg-gray-800 drop-shadow-[0_calc(0px_-_var(--spacing))_0_var(--color-gray-900-muted)] rounded-xl grow cursor-pointer transition-all duration-150",
            id: "input_project_id",
            option {
                value: 0,
                {t!("none")}
            },
            for project in projects {
                option {
                    value: project.id.to_string(),
                    initial_selected: initial_selected_id.is_some_and(
                        |id| id == project.id
                    ),
                    {project.title}
                }
            }
        }
    }
}
