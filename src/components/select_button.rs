use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

#[component]
pub(crate) fn SelectButton<I: IconShape + Clone + PartialEq + 'static>(
    icon: I,
    is_selected: bool,
    on_select: Callback,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: format!(
                "pt-4.5 flex flex-row justify-center items-center {} rounded-xl transition-all duration-150",
                if is_selected { "pb-3.75 bg-gray-900 drop-shadow-[0_0_0_var(--color-gray-900-muted)]" }
                else { "pb-2.75 mt-1 bg-gray-800-muted hover:bg-gray-800 drop-shadow-[0_calc(0px_-_var(--spacing))_0_var(--color-gray-900-muted)] text-gray-400 cursor-pointer" }
            ),
            onclick: move |_| {
                on_select.call(());
            },
            Icon {
                icon,
                height: 16,
                width: 16
            }
        },
    }
}
