mod components;
mod errors;
mod models;
mod route;
mod schema;
mod server;
mod query;

use components::app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to initialize logger");
    info!("starting app");

    let cfg = server_only!(
        dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8000)))
    );

    LaunchBuilder::fullstack().with_cfg(cfg).launch(App);
}
