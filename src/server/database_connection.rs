use diesel::pg::PgConnection;
use diesel::prelude::*;

const DATABASE_URL: &str = "postgres://app:app@db/todo_baggins";

pub(crate) fn establish_database_connection() -> ConnectionResult<PgConnection> {
    PgConnection::establish(DATABASE_URL)
}
