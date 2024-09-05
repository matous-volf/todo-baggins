use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::models::category::Category;
use crate::models::task::Task;

pub(crate) mod tasks;

#[derive(PartialEq, Debug)]
pub(crate) enum QueryValue {
    Tasks(Vec<Task>),
}

#[derive(Debug)]
pub(crate) enum QueryErrors {
    Error(ErrorVec<Error>),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) enum QueryKey {
    Tasks,
    TasksInCategory(Category),
}
