use crate::route::Route;
use crate::{components::project_form::PROJECT_BEING_EDITED, hooks::use_projects};
use dioxus::prelude::*;

#[component]
pub(crate) fn ProjectList() -> Element {
    let navigator = use_navigator();
    let projects = use_projects()?;
    rsx! {
        div {
            class: "flex flex-col",
            for project in projects {
                div {
                    class: "px-7 sm:px-8 py-4 hover:bg-gray-800 font-medium text-pretty wrap-anywhere select-none transition-all duration-150 cursor-pointer",
                    key: "{project.id}",
                    onclick: move |_| {
                        *PROJECT_BEING_EDITED.write() = Some(project.clone());
                        navigator.push(Route::ProjectFormPage);
                    },
                    {project.title.clone()}
                }
            }
        }
    }
}
