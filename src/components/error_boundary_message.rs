use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaTriangleExclamation;

#[component]
pub(crate) fn ErrorBoundaryMessage(children: Element, class: Option<String>) -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |_| {
                rsx! {
                    div {
                        class: "grow flex flex-col justify-center items-center",
                        div {
                            Icon {
                                icon: FaTriangleExclamation,
                                height: 32,
                                width: 32
                            }
                        }
                    }
                }
            },
            div {
                class,
                {children}
            }
        }
    }
}
