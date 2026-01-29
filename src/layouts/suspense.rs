use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaCog;

#[component]
pub(crate) fn Suspense() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| {
                rsx! {
                    div {
                        class: "grow flex flex-col justify-center items-center",
                        Icon {
                            class: "text-gray-500 animate-[spin_3000ms_linear_infinite]",
                            icon: FaCog,
                            height: 32,
                            width: 32
                        }
                    }
                }
            },
            Outlet::<Route> {}
        }
    }
}
