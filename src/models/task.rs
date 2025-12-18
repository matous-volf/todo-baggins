use crate::models::category::Category;
use crate::models::subtask::Subtask;
#[cfg(feature = "server")]
use crate::schema::tasks;
use crate::utils::reverse_ord_option::ReverseOrdOption;
use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use validator::Validate;

const TITLE_LENGTH_MIN: u64 = 1;
const TITLE_LENGTH_MAX: u64 = 255;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable, Identifiable))]
#[cfg_attr(feature = "server", diesel(table_name = tasks, check_for_backend(diesel::pg::Pg)))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub deadline: Option<chrono::NaiveDate>,
    pub category: Category,
    pub project_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Eq for Task {}

impl PartialOrd<Self> for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.category, &other.category) {
            (Category::Inbox, Category::Inbox) => self.created_at.cmp(&other.created_at),
            (
                Category::Calendar {
                    date: self_date,
                    time: self_time,
                    ..
                },
                Category::Calendar {
                    date: other_date,
                    time: other_time,
                    ..
                },
            ) => self_date
                .cmp(other_date)
                .then(
                    ReverseOrdOption::from(
                        &self_time.as_ref().map(|calendar_time| calendar_time.time),
                    )
                    .cmp(&ReverseOrdOption::from(
                        &other_time.as_ref().map(|calendar_time| calendar_time.time),
                    )),
                )
                .then(
                    ReverseOrdOption::from(&self.deadline)
                        .cmp(&ReverseOrdOption::from(&other.deadline)),
                )
                .then(self.created_at.cmp(&other.created_at)),
            (Category::Done, Category::Done) | (Category::Trash, Category::Trash) => {
                self.updated_at.cmp(&other.updated_at).reverse()
            }
            (_, _) => ReverseOrdOption::from(&self.deadline)
                .cmp(&ReverseOrdOption::from(&other.deadline))
                .then(self.created_at.cmp(&other.created_at)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TaskWithSubtasks {
    pub task: Task,
    pub subtasks: Vec<Subtask>,
}

impl Eq for TaskWithSubtasks {}

impl PartialOrd<Self> for TaskWithSubtasks {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TaskWithSubtasks {
    fn cmp(&self, other: &Self) -> Ordering {
        self.task.cmp(&other.task)
    }
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = tasks))]
pub struct NewTask {
    #[validate(length(
        min = "TITLE_LENGTH_MIN",
        max = "TITLE_LENGTH_MAX",
        code = "title_length"
    ))]
    pub title: String,
    pub deadline: Option<chrono::NaiveDate>,
    pub category: Category,
    pub project_id: Option<i32>,
}

impl From<Task> for NewTask {
    fn from(task: Task) -> Self {
        Self {
            title: task.title,
            deadline: task.deadline,
            category: task.category,
            project_id: task.project_id,
        }
    }
}
