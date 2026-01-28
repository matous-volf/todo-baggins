use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

#[component]
pub(crate) fn InputLabel<I: IconShape + Clone + PartialEq + 'static>(
    icon: I,
    r#for: Option<String>,
) -> Element {
    rsx! {
        label {
            r#for,
            class: "mt-0.5 min-w-7 flex flex-row justify-center items-center",
            Icon {
                class: "text-gray-600",
                icon,
                height: 16,
                width: 16
            }
        }
    }
}
