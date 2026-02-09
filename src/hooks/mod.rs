use dioxus::{
    CapturedError,
    fullstack::{Loader, Loading, WebSocketOptions},
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

fn use_on_document_become_visible(mut callback: impl FnMut() + 'static) {
    let callback = use_callback(move |()| callback());
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"
                document.addEventListener("visibilitychange", () => {
                if (!document.hidden) {
                    dioxus.send(0);
                }
                });
                "#,
            );
            loop {
                eval.recv::<u8>()
                    .await
                    .expect("The JS code returned a value not parsable to `u8`.");
                callback.call(());
            }
        });
    });
}

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
    let mut websocket_reset_tick = use_signal(|| 0u64);

    let loader = use_loader(move || {
        let _ = refresh_tick(); // Read => dependency.
        future()
    });

    use_effect(move || {
        let initial_websocket_reset_tick = websocket_reset_tick();
        spawn(async move {
            let Ok(socket) =
                subscribe_to_updates(WebSocketOptions::new().with_automatic_reconnect()).await
            else {
                return;
            };
            while socket.recv().await.is_ok() {
                if websocket_reset_tick() != initial_websocket_reset_tick {
                    // A new WebSocket has been created (a new task spawned), cleaning this one up.
                    break;
                }
                refresh_tick += 1;
            }
        });
    });

    /* So that when the device goes to sleep or suspends the app, the WebSocket gets recreated on
    waking up. It is important to do this only on becoming visible (document.hidden == false),
    because becoming hidden is the part when network may not work and thus cause errors. */
    use_on_document_become_visible(move || {
        websocket_reset_tick += 1;
        refresh_tick += 1;
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
