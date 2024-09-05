use dioxus::prelude::*;

#[component]
pub(crate) fn StickyBottom(children: Element) -> Element {
    rsx! {
        div {
            class: "fixed bottom-0 left-0 right-0 flex flex-col",
            {children}
        }
    }
}
