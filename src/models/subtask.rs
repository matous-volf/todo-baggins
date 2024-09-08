use chrono::NaiveDateTime;
use crate::schema::subtasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

const TITLE_LENGTH_MIN: u64 = 1;
const TITLE_LENGTH_MAX: u64 = 255;

#[derive(Queryable, Selectable, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[diesel(table_name = subtasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subtask {
    id: i32,
    task_id: i32,
    title: String,
    is_completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Subtask {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn task_id(&self) -> i32 {
        self.task_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate, Clone, Debug)]
#[diesel(table_name = subtasks)]
pub struct NewSubtask {
    pub task_id: i32,
    #[validate(length(min = "TITLE_LENGTH_MIN", max = "TITLE_LENGTH_MAX", code = "title_length"))]
    pub title: String,
    pub is_completed: bool,
}

impl NewSubtask {
    pub fn new(task_id: i32, title: String, is_completed: bool) -> Self {
        Self { task_id, title, is_completed }
    }
}

impl From<Subtask> for NewSubtask {
    fn from(subtask: Subtask) -> Self {
        Self::new(subtask.task_id, subtask.title, subtask.is_completed)
    }
}
