#[cfg(feature = "server")]
use crate::models::task::Task;
#[cfg(feature = "server")]
use crate::schema::subtasks;
use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use validator::Validate;

const TITLE_LENGTH_MIN: u64 = 1;
const TITLE_LENGTH_MAX: u64 = 255;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Identifiable, Associations)
)]
#[cfg_attr(
    feature = "server",
    diesel(
        table_name = subtasks,
        belongs_to(Task, foreign_key = task_id),
        check_for_backend(diesel::pg::Pg)
    )
)]
pub struct Subtask {
    pub id: i32,
    pub task_id: i32,
    pub title: String,
    pub is_completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Eq for Subtask {}

impl PartialOrd<Self> for Subtask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Subtask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.is_completed
            .cmp(&other.is_completed)
            .then(self.created_at.cmp(&other.created_at))
    }
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = subtasks))]
pub struct NewSubtask {
    pub task_id: i32,
    #[validate(length(
        min = "TITLE_LENGTH_MIN",
        max = "TITLE_LENGTH_MAX",
        code = "title_length"
    ))]
    pub title: String,
    pub is_completed: bool,
}

impl From<Subtask> for NewSubtask {
    fn from(subtask: Subtask) -> Self {
        Self {
            task_id: subtask.task_id,
            title: subtask.title,
            is_completed: subtask.is_completed,
        }
    }
}
