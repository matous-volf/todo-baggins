use crate::route::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

#[component]
pub(crate) fn NavigationItem<I: IconShape + Clone + PartialEq + 'static>(
    route: Route,
    icon: I,
) -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        Link {
            to: route.clone(),
            class: format!(
                "py-5 flex flex-row justify-center items-center {}",
                if current_route == route { "text-zinc-200" }
                else { "text-zinc-500" }
            ),
            Icon {
                icon,
                height: 24,
                width: 24
            }
        }
    }
}
