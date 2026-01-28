use dioxus::prelude::*;

#[component]
pub(crate) fn ButtonSecondary(
    class: Option<String>,
    children: Element,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    // TODO: Remove this once https://github.com/DioxusLabs/dioxus/issues/4019 gets resolved.
    onclick: Option<Callback<Event<MouseData>>>,
) -> Element {
    rsx! {
        button {
            class: format!(
                "cursor-pointer pb-[6px] hover:pb-[7px] active:pb-[2px] mt-[1px] hover:mt-0 active:mt-[5px] hover:*:drop-shadow-[0_7px_0_var(--color-gray-800)] active:*:drop-shadow-[0_2px_0_var(--color-gray-800)] transition-all duration-150 {}",
                class.unwrap_or("".to_owned())
            ),
            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },
            ..attributes,
            div {
                class: "py-3.5 px-4 flex flex-row justify-center items-center bg-gray-600 drop-shadow-[0_6px_0_var(--color-gray-800)] rounded-xl transition-all duration-150",
                {children}
            }
        }
    }
}
