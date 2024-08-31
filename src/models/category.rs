use crate::schema::tasks;
use chrono::{Duration, NaiveDate, NaiveTime};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::{Bool, Jsonb};
use diesel::{AsExpression, BoxableExpression, FromSqlRow, PgJsonbExpressionMethods};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::DurationSeconds;
use std::io::Write;

#[serde_with::serde_as]
#[derive(AsExpression, FromSqlRow, Serialize, Deserialize, Clone, Debug)]
#[diesel(sql_type = Jsonb)]
pub enum Category {
    Inbox,
    SomedayMaybe,
    WaitingFor(String),
    NextSteps,
    Calendar {
        date: NaiveDate,
        reoccurrence: Option<Reoccurrence>,
        time: Option<CalendarTime>,
    },
    LongTerm,
    Done,
    Trash,
}

impl Category {
    pub fn eq_sql_predicate(&self) -> Box<dyn BoxableExpression<tasks::table, Pg, SqlType=Bool>> {
        use crate::schema::tasks::dsl::*;

        match self {
            Category::Inbox => Box::new(category.contains(json!("Inbox"))),
            Category::SomedayMaybe => Box::new(category.contains(json!("SomedayMaybe"))),
            Category::WaitingFor(_) => Box::new(category.has_key("WaitingFor")),
            Category::NextSteps => Box::new(category.contains(json!("NextSteps"))),
            Category::Calendar { .. } => Box::new(category.has_key("Calendar")),
            Category::LongTerm => Box::new(category.contains(json!("LongTerm"))),
            Category::Done => Box::new(category.contains(json!("Done"))),
            Category::Trash => Box::new(category.contains(json!("Trash"))),
        }
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl ToSql<Jsonb, Pg> for Category {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let json = serde_json::to_string(self)?;

        // Prepend the JSONB version byte.
        out.write_all(&[1])?;
        out.write_all(json.as_bytes())?;

        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<Jsonb, Pg> for Category {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        let bytes = bytes.as_bytes();
        if bytes.is_empty() {
            return Err("Unexpected empty bytes (missing the JSONB version number).".into());
        }
        let str = std::str::from_utf8(&bytes[1..])?;
        serde_json::from_str(str).map_err(Into::into)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ReoccurrenceInterval {
    Day,
    Month,
    Year,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reoccurrence {
    start_date: NaiveDate,
    interval: ReoccurrenceInterval,
    length: u32,
}

impl Reoccurrence {
    pub fn new(start_date: NaiveDate, interval: ReoccurrenceInterval, length: u32) -> Self {
        Self { start_date, interval, length }
    }
    
    pub fn interval(&self) -> &ReoccurrenceInterval {
        &self.interval
    }

    pub fn length(&self) -> u32 {
        self.length
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalendarTime {
    time: NaiveTime,
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    reminder_offset: Option<Duration>,
}

impl CalendarTime {
    pub fn new(time: NaiveTime, reminder_offset: Option<Duration>) -> Self {
        Self { time, reminder_offset }
    }
    
    pub fn time(&self) -> NaiveTime {
        self.time
    }

    pub fn reminder_offset(&self) -> Option<Duration> {
        self.reminder_offset
    }
}
