use dioxus::prelude::*;

#[component]
pub(crate) fn ButtonPrimary(
    class: Option<String>,
    children: Element,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    // TODO: Remove this once https://github.com/DioxusLabs/dioxus/issues/4019 gets resolved.
    onclick: Option<Callback<Event<MouseData>>>,
) -> Element {
    rsx! {
        button {
            class: format!(
                "cursor-pointer pb-[6px] hover:pb-[7px] active:pb-[2px] mt-[1px] hover:mt-0 active:mt-[5px] hover:*:drop-shadow-[0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted)] active:*:drop-shadow-[0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted)] transition-all duration-150 {}",
                class.unwrap_or(String::new())
            ),
            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },
            ..attributes,
            div {
                class: "py-3.5 px-4 flex flex-row justify-center items-center bg-amber-300-muted drop-shadow-[0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted),0_1px_0_var(--color-amber-700-muted)] text-amber-700-muted rounded-xl transition-all duration-150",
                {children}
            }
        }
    }
}
