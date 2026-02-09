use crate::route::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

#[component]
pub(crate) fn Item<I: IconShape + Clone + PartialEq + 'static>(route: Route, icon: I) -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        Link {
            to: route.clone(),
            class: format!(
                "py-2.5 sm:px-6 flex flex-row justify-center items-center hover:*:bg-gray-900 active:*:text-gray-400",
            ),
            div {
                class: format!("pt-2.5 px-4 {} transition-all duration-150",
                    if current_route == route { "pb-2 mt-1 bg-gray-900 text-gray-400 rounded-xl drop-shadow-[0_calc(0px_-_var(--spacing))_0_var(--color-gray-950)]" }
                    else { "pb-3 bg-gray-800 rounded-xl drop-shadow-[0_0_0_var(--color-gray-950)] text-gray-600" }
                ),
                Icon {
                    icon,
                    height: 24,
                    width: 24
                }
            }
        }
    }
}
