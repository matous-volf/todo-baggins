use crate::schema::projects;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

const TITLE_LENGTH_MIN: u64 = 1;
const TITLE_LENGTH_MAX: u64 = 255;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    id: i32,
    title: String,
}

impl Project {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate, Clone, Debug)]
#[diesel(table_name = projects)]
pub struct NewProject {
    #[validate(length(
        min = "TITLE_LENGTH_MIN",
        max = "TITLE_LENGTH_MAX",
        code = "title_length"
    ))]
    pub title: String,
}

impl NewProject {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}
