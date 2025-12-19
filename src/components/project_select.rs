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
            class: "px-3.5 py-2.5 bg-zinc-800/50 rounded-lg grow cursor-pointer",
            id: "input_project",
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
