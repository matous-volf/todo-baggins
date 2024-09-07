use crate::models::project::{NewProject, Project};
use crate::server::projects::{create_project, edit_project};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_query::prelude::use_query_client;
use crate::query::{QueryErrors, QueryKey, QueryValue};

#[component]
pub(crate) fn ProjectForm(project: Option<Project>, on_successful_submit: EventHandler<()>)
                          -> Element {
    let query_client = use_query_client::<QueryValue, QueryErrors, QueryKey>();
    let project_for_submit = project.clone();

    rsx! {
        form {
            onsubmit: move |event| {
                let project = project_for_submit.clone();
                async move {
                    let new_project = NewProject::new(
                        event.values().get("title").unwrap().as_value()
                    );
                    if let Some(project) = project {
                        let _ = edit_project(project.id(), new_project).await;
                    } else {
                        let _ = create_project(new_project).await;
                    }
                    query_client.invalidate_queries(&[
                        QueryKey::Projects
                    ]);
                    on_successful_submit.call(());
                }
            },
            class: "p-4 flex flex-col gap-4",
            div {
                class: "flex flex-row items-center gap-3",
                label {
                    r#for: "input_title",
                    class: "min-w-6 text-center",
                    i {
                        class: "fa-solid fa-pen-clip text-zinc-400/50"
                    }
                }
                input {
                    name: "title",
                    required: true,
                    initial_value: project.map(|project| project.title().to_owned()),
                    r#type: "text",
                    class: "py-2 px-3 grow bg-zinc-800/50 rounded-lg",
                    id: "input_title"
                }
            }
            div {
                class: "flex flex-row justify-end mt-auto",
                button {
                    r#type: "submit",
                    class: "py-2 px-4 bg-zinc-300/50 rounded-lg",
                    i {
                        class: "fa-solid fa-floppy-disk"
                    }
                }
            }
        }
    }
}
