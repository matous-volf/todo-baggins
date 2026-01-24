use crate::hooks::use_subtasks_of_task;
use crate::models::subtask::NewSubtask;
use crate::models::task::Task;
use crate::server::subtasks::{create_subtask, delete_subtask, edit_subtask};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaSquare;
use dioxus_free_icons::icons::fa_solid_icons::{FaListCheck, FaPlus, FaSquareCheck, FaTrashCan};

#[component]
pub(crate) fn SubtasksForm(task: Task) -> Element {
    let subtasks = use_subtasks_of_task(task.id)?;
    let mut new_title = use_signal(String::new);
    rsx! {
        form {
            class: "flex flex-row items-center gap-3",
            onsubmit: move |event| {
                event.prevent_default();
                let task = task.clone();
                async move {
                    let new_subtask = NewSubtask {
                        task_id: task.id,
                        title: event.get("title").first().cloned().and_then(|value| match value {
                            FormValue::Text(value) => Some(value),
                            FormValue::File(_) => None
                        }).unwrap(),
                        is_completed: false
                    };
                    let _ = create_subtask(new_subtask).await;
                    new_title.set(String::new());
                }
            },
            label {
                r#for: "input_new_title",
                class: "min-w-6 flex flex-row justify-center items-center",
                Icon {
                    class: "text-zinc-400/50",
                    icon: FaListCheck,
                    height: 16,
                    width: 16
                }
            }
            div {
                class: "grow grid grid-cols-6 gap-2",
                input {
                    name: "title",
                    required: true,
                    value: new_title,
                    r#type: "text",
                    class: "grow py-2 px-3 col-span-5 bg-zinc-800/50 rounded-lg",
                    id: "input_new_title",
                    onchange: move |event| new_title.set(event.value())
                }
                button {
                    r#type: "submit",
                    class: "py-2 col-span-1 flex flex-row justify-center items-center bg-zinc-800/50 rounded-lg cursor-pointer",
                    Icon {
                        icon: FaPlus,
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
                    class: "min-w-6 flex flex-row justify-center items-center text-zinc-400/50 cursor-pointer",
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
                    if subtask.is_completed {
                        Icon {
                            icon: FaSquareCheck,
                            height: 24,
                            width: 24
                        }
                    } else {
                        Icon {
                            icon: FaSquare,
                            height: 24,
                            width: 24
                        }
                    }
                }
                div {
                    class: "grow grid grid-cols-6 gap-2",
                    input {
                        r#type: "text",
                        class: "grow py-2 px-3 col-span-5 bg-zinc-800/50 rounded-lg",
                        id: "input_title_{subtask.id}",
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
                    button {
                        r#type: "button",
                        class: "py-2 flex flex-row justify-center items-center col-span-1 bg-zinc-800/50 rounded-lg cursor-pointer",
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
