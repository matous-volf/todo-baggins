use crate::models::subtask::NewSubtask;
use crate::models::task::Task;
use crate::query::subtasks::use_subtasks_of_task_query;
use crate::query::{QueryErrors, QueryKey, QueryValue};
use crate::server::subtasks::{create_subtask, delete_subtask, edit_subtask};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_query::prelude::{use_query_client, QueryResult};

#[component]
pub(crate) fn SubtasksForm(task: Task) -> Element {
    let query_client = use_query_client::<QueryValue, QueryErrors, QueryKey>();
    let subtasks_query = use_subtasks_of_task_query(task.id());

    let mut new_title = use_signal(String::new);

    rsx! {
        form {
            class: "flex flex-row items-center gap-3",
            onsubmit: move |event| {
                let task = task.clone();
                async move {
                    let new_subtask = NewSubtask::new(
                        task.id(),
                        event.values().get("title").unwrap().as_value(),
                        false
                    );
                    let _ = create_subtask(new_subtask).await;
                    query_client.invalidate_queries(&[
                        QueryKey::SubtasksOfTaskId(task.id()),
                        QueryKey::TasksWithSubtasksInCategory(task.category().clone()),
                    ]);
                    new_title.set(String::new());
                }
            },
            label {
                r#for: "input_new_title",
                class: "min-w-6 text-center",
                i {
                    class: "fa-solid fa-list-check text-zinc-400/50"
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
                    class: "py-2 col-span-1 bg-zinc-800/50 rounded-lg",
                    i {
                        class: "fa-solid fa-plus"
                    }
                }
            }
        }
        match subtasks_query.result().value() {
            QueryResult::Ok(QueryValue::Subtasks(subtasks))
            | QueryResult::Loading(Some(QueryValue::Subtasks(subtasks))) => {
                let mut subtasks = subtasks.clone();
                subtasks.sort();
                rsx! {
                    for subtask in subtasks {
                        div {
                            key: "{subtask.id()}",
                            class: "flex flex-row items-center gap-3",
                            i {
                                class: format!(
                                    "{} min-w-6 text-center text-2xl text-zinc-400/50",
                                    if subtask.is_completed() {
                                        "fa solid fa-square-check"
                                    } else {
                                        "fa-regular fa-square"
                                    }
                                ),
                                onclick: {
                                    let subtask = subtask.clone();
                                    let task = task.clone();
                                    move |_| {
                                        let subtask = subtask.clone();
                                        let task = task.clone();
                                        async move {
                                            let new_subtask = NewSubtask::new(
                                                subtask.task_id(),
                                                subtask.title().to_owned(),
                                                !subtask.is_completed()
                                            );
                                            let _ = edit_subtask(
                                                subtask.id(),
                                                new_subtask
                                            ).await;
                                            query_client.invalidate_queries(&[
                                                QueryKey::SubtasksOfTaskId(task.id()),
                                                QueryKey::TasksWithSubtasksInCategory(
                                                    task.category().clone()
                                                ),
                                            ]);
                                        }
                                    }
                                }
                            }
                            div {
                                class: "grow grid grid-cols-6 gap-2",
                                input {
                                    r#type: "text",
                                    class: "grow py-2 px-3 col-span-5 bg-zinc-800/50 rounded-lg",
                                    id: "input_title_{subtask.id()}",
                                    initial_value: subtask.title(),
                                    onchange: {
                                        let subtask = subtask.clone();
                                        let task = task.clone();
                                        move |event| {
                                            let subtask = subtask.clone();
                                            let task = task.clone();
                                            async move {
                                                let new_subtask = NewSubtask::new(
                                                    subtask.task_id(),
                                                    event.value(),
                                                    subtask.is_completed()
                                                );
                                                if new_subtask.title.is_empty() {
                                                    let _ = delete_subtask(subtask.id()).await;
                                                } else {
                                                    let _ = edit_subtask(
                                                        subtask.id(),
                                                        new_subtask
                                                    ).await;
                                                }
                                                query_client.invalidate_queries(&[
                                                    QueryKey::SubtasksOfTaskId(task.id()),
                                                    QueryKey::TasksWithSubtasksInCategory(
                                                        task.category().clone()
                                                    ),
                                                ]);
                                            }
                                        }
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "py-2 col-span-1 bg-zinc-800/50 rounded-lg",
                                    onclick: {
                                        let subtask = subtask.clone();
                                        let task = task.clone();
                                        move |_| {
                                            let subtask = subtask.clone();
                                            let task = task.clone();
                                            async move {
                                                let _ = delete_subtask(subtask.id()).await;
                                                query_client.invalidate_queries(&[
                                                    QueryKey::SubtasksOfTaskId(task.id()),
                                                    QueryKey::TasksWithSubtasksInCategory(
                                                        task.category().clone()
                                                    ),
                                                ]);
                                            }
                                        }
                                    },
                                    i {
                                        class: "fa-solid fa-trash-can"
                                    }
                                }
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
