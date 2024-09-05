use dioxus::prelude::ServerFnError;
use dioxus_query::prelude::{use_get_query, QueryResult, UseQuery};
use crate::models::category::Category;
use crate::query::{QueryErrors, QueryKey, QueryValue};
use crate::server::tasks::get_tasks_in_category;



pub(crate) fn use_tasks_in_category_query(category: Category)
                                          -> UseQuery<QueryValue, QueryErrors, QueryKey> {
    use_get_query([QueryKey::TasksInCategory(category), QueryKey::Tasks], fetch_tasks_in_category)
}

async fn fetch_tasks_in_category(keys: Vec<QueryKey>) -> QueryResult<QueryValue, QueryErrors> {
    if let Some(QueryKey::TasksInCategory(category)) = keys.first() {
        match get_tasks_in_category(category.clone()).await {
            Ok(tasks) => Ok(QueryValue::Tasks(tasks)),
            Err(ServerFnError::WrappedServerError(errors)) => Err(QueryErrors::Error(errors)),
            Err(error) => panic!("Unexpected error: {:?}", error)
        }.into()
    } else {
        panic!("Unexpected query keys: {:?}", keys);
    }
}
