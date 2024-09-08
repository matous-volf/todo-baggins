use crate::query::{QueryErrors, QueryKey, QueryValue};
use crate::server::subtasks::get_subtasks_of_task;
use dioxus::prelude::ServerFnError;
use dioxus_query::prelude::{use_get_query, QueryResult, UseQuery};

pub(crate) fn use_subtasks_of_task_query(task_id: i32)
                                         -> UseQuery<QueryValue, QueryErrors, QueryKey> {
    use_get_query([QueryKey::SubtasksOfTaskId(task_id)], fetch_subtasks_of_task)
}

async fn fetch_subtasks_of_task(keys: Vec<QueryKey>) -> QueryResult<QueryValue, QueryErrors> {
    if let Some(QueryKey::SubtasksOfTaskId(task_id)) = keys.first() {
        match get_subtasks_of_task(*task_id).await {
            Ok(subtasks) => Ok(QueryValue::Subtasks(subtasks)),
            Err(ServerFnError::WrappedServerError(errors)) => Err(QueryErrors::Error(errors)),
            Err(error) => panic!("Unexpected error: {:?}", error)
        }.into()
    } else {
        panic!("Unexpected query keys: {:?}", keys);
    }
}
