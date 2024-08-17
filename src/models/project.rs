use crate::schema::projects;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: &'a str,
}
