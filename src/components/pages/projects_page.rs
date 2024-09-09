use dioxus::prelude::*;
use dioxus_query::prelude::QueryResult;
use crate::models::project::Project;
use crate::query::projects::use_projects_query;
use crate::query::QueryValue;

#[component]
pub(crate) fn ProjectsPage() -> Element {
    let projects_query = use_projects_query();
    let mut project_being_edited = use_context::<Signal<Option<Project>>>();

    rsx! {
        match projects_query.result().value() {
            QueryResult::Ok(QueryValue::Projects(projects))
            | QueryResult::Loading(Some(QueryValue::Projects(projects))) => {
                let mut projects = projects.clone();
                projects.sort();
                rsx! {
                    div {
                        class: "flex flex-col",
                        for project in projects {
                            div {
                                key: "{project.id()}",
                                class: format!(
                                    "px-8 py-4 select-none {}",
                                    if project_being_edited().is_some_and(|p| p.id() == project.id()) {
                                        "bg-zinc-700"
                                    } else { "" }
                                ),
                                onclick: move |_| project_being_edited.set(Some(project.clone())),
                                {project.title()}
                            }
                        }
                    }
                }
            },
            QueryResult::Loading(None) => rsx! {
                // TODO: Add a loading indicator.
            },
            QueryResult::Err(errors) => rsx! {
                div {
                    "Errors occurred: {errors:?}"
                }
            },
            value => panic!("Unexpected query result: {value:?}")
        }
    }
}
