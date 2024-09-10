use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub(crate) fn establish_database_connection() -> ConnectionResult<PgConnection> {
    dotenv().expect("Could not load environment variables.");

    let database_url =
        env::var("DATABASE_URL").expect("The environment variable DATABASE_URL has to be set.");
    PgConnection::establish(&database_url)
}
