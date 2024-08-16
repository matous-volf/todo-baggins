use crate::schema::projects;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub(crate) id: i32,
    pub(crate) title: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: &'a str,
}
