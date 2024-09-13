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
use dioxus_logger::tracing::{info, Level};

fn main() {
    dioxus_logger::init(Level::INFO).expect("Failed to initialize the logger.");

    info!("Running migrations.");
    migrations::run_migrations().expect("Failed to run migrations.");

    info!("Starting app.");
    let cfg = server_only!(
        dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8000)))
    );
    LaunchBuilder::fullstack().with_cfg(cfg).launch(App);
}
