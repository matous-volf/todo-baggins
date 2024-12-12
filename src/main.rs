mod components;
mod errors;
mod models;
mod route;
mod schema;
mod server;
mod query;
mod utils;
mod internationalization;
mod migrations;

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
