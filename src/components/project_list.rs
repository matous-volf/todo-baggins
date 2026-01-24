use crate::{hooks::use_projects, models::project::Project};
use dioxus::prelude::*;

#[component]
pub(crate) fn ProjectList() -> Element {
    let projects = use_projects()?;
    let mut project_being_edited = use_context::<Signal<Option<Project>>>();

    rsx! {
        div {
            class: "flex flex-col",
            for project in projects {
                div {
                    key: "{project.id}",
                    class: format!(
                        "px-7 py-4 select-none {}",
                        if project_being_edited().is_some_and(|p| p.id == project.id) {
                            "bg-zinc-700"
                        } else { "" }
                    ),
                    onclick: move |_| project_being_edited.set(Some(project.clone())),
                    {project.title.clone()}
                }
            }
        }
    }
}
