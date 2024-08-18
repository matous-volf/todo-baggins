use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub(crate) fn establish_database_connection() -> ConnectionResult<PgConnection> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("The environment variable DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
}
