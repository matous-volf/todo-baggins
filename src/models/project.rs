use std::cmp::Ordering;
use chrono::NaiveDateTime;
use crate::schema::projects;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::internationalization::COLLATOR;

const TITLE_LENGTH_MIN: u64 = 1;
const TITLE_LENGTH_MAX: u64 = 255;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    id: i32,
    title: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Project {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    
    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

impl Eq for Project {}

impl PartialOrd<Self> for Project {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Project {
    fn cmp(&self, other: &Self) -> Ordering {
        COLLATOR.lock().unwrap().collate(self.title(), other.title())
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate, Clone, Debug)]
#[diesel(table_name = projects)]
pub struct NewProject {
    #[validate(length(min = "TITLE_LENGTH_MIN", max = "TITLE_LENGTH_MAX", code = "title_length"))]
    pub title: String,
}

impl NewProject {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}
