use crate::components::button_secondary::ButtonSecondary;
use crate::components::input::Input;
use crate::components::input_label::InputLabel;
use crate::hooks::use_subtasks_of_task;
use crate::models::subtask::NewSubtask;
use crate::models::task::Task;
use crate::server::subtasks::{create_subtask, delete_subtask, edit_subtask};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaGavel, FaListCheck, FaTrashCan};

#[component]
pub(crate) fn SubtasksForm(task: Task) -> Element {
    let subtasks = use_subtasks_of_task(task.id)?;
    let mut new_title = use_signal(String::new);
    rsx! {
        div {
            class: "flex flex-col gap-3",
            form {
                class: "flex flex-row items-center gap-3",
                onsubmit: move |event| {
                    event.prevent_default();
                    let task = task.clone();
                    async move {
                        let new_subtask = NewSubtask {
                            task_id: task.id,
                            title: event.get("new_title").first().cloned().and_then(|value| match value {
                                FormValue::Text(value) => Some(value),
                                FormValue::File(_) => None
                            }).unwrap(),
                            is_completed: false
                        };
                        let _ = create_subtask(new_subtask).await;
                        new_title.set(String::new());
                    }
                },
                InputLabel {
                    icon: FaListCheck,
                    r#for: "input_new_title"
                }
                div {
                    class: "grow flex flex-row items-end gap-3",
                    Input {
                        class: "grow",
                        name: "new_title",
                        r#type: "text",
                        required: true,
                        value: new_title,
                        onchange: move |event: Event<FormData>| new_title.set(event.value())
                    }
                    ButtonSecondary {
                        r#type: "submit",
                        Icon {
                            icon: FaGavel,
                            height: 16,
                            width: 16
                        }
                    }
                }
            }
            for subtask in subtasks {
                div {
                    key: "{subtask.id}",
                    class: "flex flex-row items-center gap-3",
                    button {
                        class: "mt-1.5 hover:mt-1 hover:pb-0.5 min-w-7 cursor-pointer transition-all duration-150",
                        onclick: {
                            let subtask = subtask.clone();
                            move |_| {
                                let subtask = subtask.clone();
                                async move {
                                    let new_subtask = NewSubtask {
                                        task_id: subtask.task_id,
                                        title: subtask.title.clone(),
                                        is_completed: !subtask.is_completed
                                    };
                                    let _ = edit_subtask(
                                        subtask.id,
                                        new_subtask
                                    ).await;
                                }
                            }
                        },
                        div {
                            class: format!("grow h-7 w-7 mb-[4px] drop-shadow-[0_1px_0_var(--color-gray-800),0_1px_0_var(--color-gray-800),0_1px_0_var(--color-gray-800),0_1px_0_var(--color-gray-800)] rounded-full {}",
                                if subtask.is_completed {"bg-gray-600"} else {"border-3 border-gray-600"}
                            )
                        }
                    }
                    div {
                        class: "grow flex flex-row items-end gap-3",
                        Input {
                            class: "grow",
                            name: "title_edit_{subtask.id}",
                            r#type: "text",
                            initial_value: subtask.title.clone(),
                            onchange: {
                                let subtask = subtask.clone();
                                move |event: Event<FormData>| {
                                    let subtask = subtask.clone();
                                    async move {
                                        let new_subtask = NewSubtask {
                                            task_id: subtask.task_id,
                                            title: event.value(),
                                            is_completed: subtask.is_completed
                                        };
                                        if new_subtask.title.is_empty() {
                                            let _ = delete_subtask(subtask.id).await;
                                        } else {
                                            let _ = edit_subtask(
                                                subtask.id,
                                                new_subtask
                                            ).await;
                                        }
                                    }
                                }
                            }
                        }
                        ButtonSecondary {
                            r#type: "button",
                            onclick: {
                                let subtask = subtask.clone();
                                move |_| {
                                    let subtask = subtask.clone();
                                    async move {
                                        let _ = delete_subtask(subtask.id).await;
                                    }
                                }
                            },
                            Icon {
                                icon: FaTrashCan,
                                height: 16,
                                width: 16
                            }
                        }
                    }
                }
            }
        }
    }
}
