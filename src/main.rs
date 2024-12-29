mod components;
mod errors;
mod internationalization;
mod migrations;
mod models;
mod query;
mod route;
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
