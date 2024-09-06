use crate::models::category::Category;
use crate::schema::tasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

const TITLE_LENGTH_MIN: u64 = 1;
const TITLE_LENGTH_MAX: u64 = 255;

#[derive(Queryable, Selectable, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    id: i32,
    title: String,
    deadline: Option<chrono::NaiveDate>,
    category: Category,
    project_id: Option<i32>,
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
        title: String, deadline: Option<chrono::NaiveDate>,
        category: Category, project_id: Option<i32>,
    ) -> Self {
        Self { title, deadline, category, project_id }
    }
}
