use crate::components::button_primary::ButtonPrimary;
use crate::components::button_secondary::ButtonSecondary;
use crate::components::input::Input;
use crate::components::input_label::InputLabel;
use crate::models::project::Project;
use crate::server::projects::{create_project, delete_project, edit_project};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaFeatherPointed, FaStamp, FaTrashCan, FaXmark};

pub(crate) static PROJECT_BEING_EDITED: GlobalSignal<Option<Project>> = Signal::global(|| None);

#[component]
pub(crate) fn ProjectForm() -> Element {
    let navigator = use_navigator();
    let project = PROJECT_BEING_EDITED();
    let project_for_submit = project.clone();
    rsx! {
        form {
            class: "px-4 flex flex-col gap-4",
            onsubmit: move |event| {
                event.prevent_default();
                let project = project_for_submit.clone();
                async move {
                    let new_project = event.parsed_values().unwrap();
                    let result = if let Some(project) = project {
                        edit_project(project.id, new_project).await
                    } else {
                        create_project(new_project).await
                    };
                    if result.is_ok() {
                        navigator.go_back();
                    }
                }
            },
            id: "form_project",
            div {
                class: "flex flex-row items-center gap-3",
                InputLabel {
                    icon: FaFeatherPointed,
                    r#for: "input_title"
                }
                Input {
                    class: "grow",
                    name: "title",
                    required: true,
                    r#type: "text",
                    initial_value: project.as_ref().map(|project| project.title.to_owned()),
                }
            }
        }
        div {
            class: "px-4 grid grid-cols-3 gap-3 mt-auto",
            ButtonSecondary {
                r#type: "button",
                class: "grow",
                onclick: {
                    let project = project.clone();
                    move |_| {
                        let project = project.clone();
                        async move {
                            if let Some(project) = project {
                                let result = delete_project(project.id).await;
                                if result.is_ok() {
                                    /* TODO: Might not work on mobile due to
                                    https://dioxuslabs.com/learn/0.7/essentials/router/navigation#history-buttons.
                                    */
                                    navigator.go_back();
                                }
                            } else {
                                navigator.go_back();
                            }
                        }
                    }
                },
                Icon {
                    icon: FaTrashCan,
                    height: 16,
                    width: 16
                }
            }
            if project.is_some() {
                div {
                    class: "grow flex flex-col items-stretch",
                    GoBackButton {
                        ButtonSecondary {
                            /* TODO: Replace w-full` with proper flexbox styling once
                            https://github.com/DioxusLabs/dioxus/issues/5269 is solved. */
                            class: "w-full",
                            r#type: "button",
                            Icon {
                                icon: FaXmark,
                                height: 16,
                                width: 16
                            }
                        }
                    }
                }
            } else {
                div {}
            }
            ButtonPrimary {
                form: "form_project",
                r#type: "submit",
                Icon {
                    icon: FaStamp,
                    height: 16,
                    width: 16
                }
            }
        }
    }
}
