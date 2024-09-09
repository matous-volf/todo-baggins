use chrono::NaiveDateTime;
use crate::models::category::Category;
use crate::schema::tasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::subtask::Subtask;

const TITLE_LENGTH_MIN: u64 = 1;
const TITLE_LENGTH_MAX: u64 = 255;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    id: i32,
    title: String,
    deadline: Option<chrono::NaiveDate>,
    category: Category,
    project_id: Option<i32>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Task {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn deadline(&self) -> Option<chrono::NaiveDate> {
        self.deadline
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn project_id(&self) -> Option<i32> {
        self.project_id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TaskWithSubtasks {
    task: Task,
    subtasks: Vec<Subtask>,
}

impl TaskWithSubtasks {
    pub fn new(task: Task, subtasks: Vec<Subtask>) -> Self {
        Self { task, subtasks }
    }

    pub fn task(&self) -> &Task {
        &self.task
    }

    pub fn subtasks(&self) -> &Vec<Subtask> {
        &self.subtasks
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate, Clone, Debug)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    #[validate(length(min = "TITLE_LENGTH_MIN", max = "TITLE_LENGTH_MAX", code = "title_length"))]
    pub title: String,
    pub deadline: Option<chrono::NaiveDate>,
    pub category: Category,
    pub project_id: Option<i32>,
}

impl NewTask {
    pub fn new(
        title: String,
        deadline: Option<chrono::NaiveDate>,
        category: Category,
        project_id: Option<i32>,
    ) -> Self {
        Self { title, deadline, category, project_id }
    }
}

impl From<Task> for NewTask {
    fn from(task: Task) -> Self {
        Self::new(task.title, task.deadline, task.category, task.project_id)
    }
}
