use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn NavigationItem(route: Route, children: Element) -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        Link {
            to: route.clone(),
            class: format!(
                "py-4 text-center text-2xl {}",
                if current_route == route { "text-zinc-200" }
                else { "text-zinc-500" }
            ),
            children
        }
    }
}
