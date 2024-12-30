use crate::internationalization::COLLATOR;
#[cfg(feature = "server")]
use crate::schema::projects;
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
#[cfg_attr(
    feature = "server",
    diesel(table_name = crate::schema::projects, check_for_backend(diesel::pg::Pg))
)]
pub struct Project {
    id: i32,
    title: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[allow(dead_code)]
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
        COLLATOR
            .lock()
            .unwrap()
            .collate(self.title(), other.title())
    }
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = projects))]
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
