mod components;
mod errors;
mod internationalization;
#[cfg(feature = "server")]
mod migrations;
mod models;
mod query;
mod route;
#[cfg(feature = "server")]
mod schema;
mod server;
mod utils;

use components::app::App;
use dioxus::prelude::*;
use tracing::info;

fn main() {
    info!("Running migrations.");
    server_only!(
        migrations::run_migrations().expect("Failed to run migrations.");
    );

    info!("Starting app.");
    launch(App);
}
