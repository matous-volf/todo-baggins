use crate::query::{QueryErrors, QueryKey, QueryValue};
use crate::server::projects::get_projects;
use dioxus::prelude::ServerFnError;
use dioxus_query::prelude::{use_get_query, QueryResult, UseQuery};

pub(crate) fn use_projects_query() -> UseQuery<QueryValue, QueryErrors, QueryKey> {
    use_get_query([QueryKey::Projects, QueryKey::Tasks], fetch_projects)
}

async fn fetch_projects(keys: Vec<QueryKey>) -> QueryResult<QueryValue, QueryErrors> {
    if let Some(QueryKey::Projects) = keys.first() {
        match get_projects().await {
            Ok(projects) => Ok(QueryValue::Projects(projects)),
            Err(ServerFnError::WrappedServerError(errors)) => Err(QueryErrors::Error(errors)),
            Err(error) => panic!("Unexpected error: {:?}", error)
        }.into()
    } else {
        panic!("Unexpected query keys: {:?}", keys);
    }
}
