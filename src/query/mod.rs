use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::models::category::Category;
use crate::models::project::Project;
use crate::models::subtask::Subtask;
use crate::models::task::{Task, TaskWithSubtasks};

pub(crate) mod tasks;
pub(crate) mod projects;
pub(crate) mod subtasks;

#[derive(PartialEq, Debug)]
pub(crate) enum QueryValue {
    Projects(Vec<Project>),
    Tasks(Vec<Task>),
    TasksWithSubtasks(Vec<TaskWithSubtasks>),
    Subtasks(Vec<Subtask>),
}

#[derive(Debug)]
pub(crate) enum QueryErrors {
    Error(ErrorVec<Error>),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) enum QueryKey {
    Projects,
    Tasks,
    TasksInCategory(Category),
    TasksWithSubtasksInCategory(Category),
    SubtasksOfTaskId(i32),
}
