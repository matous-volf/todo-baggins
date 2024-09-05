use dioxus::prelude::*;
use dioxus_query::prelude::QueryResult;
use crate::query::projects::use_projects_query;
use crate::query::QueryValue;

#[component]
pub(crate) fn ProjectsPage() -> Element {
    let projects_query = use_projects_query();
    
    rsx! {
        match projects_query.result().value() {
            QueryResult::Ok(QueryValue::Projects(projects))
            | QueryResult::Loading(Some(QueryValue::Projects(projects))) => rsx! {
                div {
                    class: "flex flex-col",
                    for project in projects {
                        div {
                            key: "{project.id()}",
                            class: "px-8 py-4",
                            {project.title()}
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
