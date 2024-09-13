use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub(crate) fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut connection = crate::server::database_connection::establish_database_connection()?;

    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
