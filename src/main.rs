#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]
// TODO: Enable once false positives are fixed.
#![allow(clippy::assigning_clones)]

mod components;
mod dotenv;
mod hooks;
mod internationalization;
mod layouts;
#[cfg(feature = "server")]
mod migrations;
mod models;
mod route;
#[cfg(feature = "server")]
mod schema;
mod server;
mod utils;
mod views;

use components::app::App;
use dioxus::prelude::*;
use tracing::info;

fn main() {
    info!("Running migrations.");
    server_only!(
        migrations::run_migrations().expect("Failed to run migrations.");
    );

    #[cfg(feature = "mobile")]
    dioxus::fullstack::set_server_url(crate::dotenv::MOBILE_SERVER_URL);

    info!("Starting the app.");
    launch(App);
}
