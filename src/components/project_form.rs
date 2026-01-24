use crate::models::project::Project;
use crate::server::projects::{create_project, delete_project, edit_project};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaFloppyDisk, FaPenClip, FaTrashCan};

#[component]
pub(crate) fn ProjectForm(
    project: Option<Project>,
    on_successful_submit: EventHandler<()>,
) -> Element {
    let project_for_submit = project.clone();
    rsx! {
        form {
            onsubmit: move |event| {
                event.prevent_default();
                let project = project_for_submit.clone();
                async move {
                    let new_project = event.parsed_values().unwrap();
                    if let Some(project) = project {
                        let _ = edit_project(project.id, new_project).await;
                    } else {
                        let _ = create_project(new_project).await;
                    }
                    on_successful_submit.call(());
                }
            },
            class: "p-4 flex flex-col gap-4",
            div {
                class: "flex flex-row items-center gap-3",
                label {
                    r#for: "input_title",
                    class: "flex flex-row justify-center items-center min-w-6",
                    Icon {
                        class: "text-zinc-400/50",
                        icon: FaPenClip,
                        height: 16,
                        width: 16
                    }
                }
                input {
                    name: "title",
                    required: true,
                    initial_value: project.as_ref().map(|project| project.title.to_owned()),
                    r#type: "text",
                    class: "py-2 px-3 grow bg-zinc-800/50 rounded-lg",
                    id: "input_title"
                }
            }
            div {
                class: "flex flex-row justify-between mt-auto",
                button {
                    r#type: "button",
                    class: "py-3 px-4 bg-zinc-300/50 rounded-lg cursor-pointer",
                    onclick: move |_| {
                        let project = project.clone();
                        async move {
                            if let Some(project) = project {
                                let _ = delete_project(project.id).await;
                            }
                            on_successful_submit.call(());
                        }
                    },
                    Icon {
                        icon: FaTrashCan,
                        height: 16,
                        width: 16
                    }
                }
                button {
                    r#type: "submit",
                    class: "py-3 px-4 bg-zinc-300/50 rounded-lg cursor-pointer",
                    Icon {
                        icon: FaFloppyDisk,
                        height: 16,
                        width: 16
                    }
                }
            }
        }
    }
}
