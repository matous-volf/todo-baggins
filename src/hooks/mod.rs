use dioxus::{
    CapturedError,
    fullstack::{Loader, Loading, WebSocketOptions, use_websocket},
    prelude::*,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    models::{category::Category, project::Project, subtask::Subtask, task::TaskWithSubtasks},
    server::{
        projects::get_projects, subtasks::get_subtasks_of_task,
        tasks::get_tasks_with_subtasks_in_category, updates::subscribe_to_updates,
    },
};

#[allow(clippy::result_large_err)]
fn sort_loader_result<T: Ord + Clone>(
    result: Result<Loader<Vec<T>>, Loading>,
) -> Result<Vec<T>, Loading> {
    result.map(|loader| {
        let mut items_sorted = loader();
        items_sorted.sort();
        items_sorted
    })
}

#[allow(clippy::result_large_err)]
fn use_loader_with_update_subscription<F, T, E>(
    mut future: impl FnMut() -> F + 'static,
) -> Result<Loader<T>, Loading>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: 'static + PartialEq + Serialize + DeserializeOwned,
    E: Into<CapturedError> + 'static,
{
    let mut refresh_tick = use_signal(|| 0u64);

    let loader = use_loader(move || {
        let _ = refresh_tick(); // Read => dependency.
        future()
    });

    let mut socket = use_websocket(|| subscribe_to_updates(WebSocketOptions::default()));
    use_future(move || async move {
        while socket.recv().await.is_ok() {
            refresh_tick += 1;
        }
    });

    loader
}

#[allow(clippy::result_large_err)]
pub(crate) fn use_projects() -> Result<Vec<Project>, Loading> {
    let result = use_loader_with_update_subscription(get_projects);
    sort_loader_result(result)
}

#[allow(clippy::result_large_err)]
pub(crate) fn use_tasks_with_subtasks_in_category(
    filtered_category: Category,
) -> Result<Vec<TaskWithSubtasks>, Loading> {
    let result = use_loader_with_update_subscription(move || {
        get_tasks_with_subtasks_in_category(filtered_category.clone())
    });
    sort_loader_result(result)
}

#[allow(clippy::result_large_err)]
pub(crate) fn use_subtasks_of_task(task_id: i32) -> Result<Vec<Subtask>, Loading> {
    let result = use_loader_with_update_subscription(move || get_subtasks_of_task(task_id));
    sort_loader_result(result)
}
