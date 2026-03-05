use dioxus::prelude::*;

use crate::components::navigation;

#[component]
pub(crate) fn BottomPanel() -> Element {
    let navigation_expanded = use_signal(|| false);

    rsx! {
        div {
            class: format!(
                "flex flex-col pointer-events-auto bg-gray-800 transition-[height] duration-[500ms] ease-[cubic-bezier(0.79,0.14,0.15,0.86)] overflow-y-scroll {}",
                if navigation_expanded() {
                    "h-[132px]"
                } else {
                    "h-[66px]"
                }
            ),
            navigation::Bottom {
                is_expanded: navigation_expanded,
            }
        }
    }
}
