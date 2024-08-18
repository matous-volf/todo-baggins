use crate::components::home::Home;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    Home {},
}
