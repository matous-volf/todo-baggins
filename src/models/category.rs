use chrono::{Duration, NaiveDate, NaiveTime};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Jsonb;
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
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
        #[serde_as(as = "Option<DurationSeconds<i64>>")]
        reoccurance_interval: Option<Duration>,
        time: Option<CalendarTime>,
    },
    LongTerm,
    Done,
    Trash,
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
