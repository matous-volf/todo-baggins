#[cfg(feature = "server")]
use crate::schema::tasks;
use chrono::{Duration, NaiveDate, NaiveTime};
#[cfg(feature = "server")]
use diesel::deserialize::FromSql;
#[cfg(feature = "server")]
use diesel::pg::{Pg, PgValue};
#[cfg(feature = "server")]
use diesel::serialize::{Output, ToSql};
#[cfg(feature = "server")]
use diesel::sql_types::{Bool, Jsonb};
#[cfg(feature = "server")]
use diesel::{AsExpression, BoxableExpression, FromSqlRow, PgJsonbExpressionMethods};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use serde_json::json;
use serde_with::DurationSeconds;
use std::hash::Hash;
#[cfg(feature = "server")]
use std::io::Write;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "server", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "server", diesel(sql_type = Jsonb))]
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

#[cfg(feature = "server")]
impl Category {
    pub fn eq_sql_predicate(&self) -> Box<dyn BoxableExpression<tasks::table, Pg, SqlType = Bool>> {
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

impl Hash for Category {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for Category {}

#[cfg(feature = "server")]
impl ToSql<Jsonb, Pg> for Category {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let json = serde_json::to_string(self)?;

        // Prepend the JSONB version byte.
        out.write_all(&[1])?;
        out.write_all(json.as_bytes())?;

        Ok(diesel::serialize::IsNull::No)
    }
}

#[cfg(feature = "server")]
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

#[derive(Serialize, Deserialize, PartialEq, Hash, Clone, Debug)]
pub enum ReoccurrenceInterval {
    Day,
    Month,
    Year,
}

#[derive(Serialize, Deserialize, Hash, Clone, Debug)]
pub struct Reoccurrence {
    start_date: NaiveDate,
    interval: ReoccurrenceInterval,
    length: u32,
}

impl Reoccurrence {
    pub fn new(start_date: NaiveDate, interval: ReoccurrenceInterval, length: u32) -> Self {
        Self {
            start_date,
            interval,
            length,
        }
    }

    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub fn interval(&self) -> &ReoccurrenceInterval {
        &self.interval
    }

    pub fn length(&self) -> u32 {
        self.length
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Hash, Clone, Debug)]
pub struct CalendarTime {
    time: NaiveTime,
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    reminder_offset: Option<Duration>,
}

impl CalendarTime {
    pub fn new(time: NaiveTime, reminder_offset: Option<Duration>) -> Self {
        Self {
            time,
            reminder_offset,
        }
    }

    pub fn time(&self) -> NaiveTime {
        self.time
    }

    pub fn reminder_offset(&self) -> Option<Duration> {
        self.reminder_offset
    }
}
