use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn ErrorBoundaryMessage(children: Element, class: Option<String>) -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |_| {
                rsx! {
                    div {
                        class: "grow flex flex-col justify-center items-center",
                        div {
                            i {
                                class: "text-3xl fa-solid fa-triangle-exclamation"
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
