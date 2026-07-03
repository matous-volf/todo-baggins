use dioxus::prelude::*;

#[component]
pub(crate) fn Input(
    class: Option<String>,
    name: String,
    r#type: String,
    id: Option<String>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
    // TODO: Remove this once https://github.com/DioxusLabs/dioxus/issues/5271 gets resolved.
    autofocus: Option<bool>,
    // TODO: Remove this once https://github.com/DioxusLabs/dioxus/issues/4019 gets resolved.
    oninput: Option<Callback<Event<FormData>>>,
    onchange: Option<Callback<Event<FormData>>>,
) -> Element {
    rsx! {
        input {
            class: format!(
                /* `w-full` is required for the Chromium renderer to allow the input to shrink
                properly. */
                "pt-3 pb-2.25 w-full {} bg-gray-800-muted enabled:hover:bg-gray-800 enabled:focus:bg-gray-800 drop-shadow-[0_calc(0px_-_var(--spacing))_0_var(--color-gray-900-muted)] rounded-xl outline-0 {} transition-all duration-150 {}",
                match r#type.as_str() {
                    "date" => "ps-3.25 pe-3",
                    _ => "px-4"
                },
                match r#type.as_str() {
                    "text" | "number" => "",
                    _ => "enabled:cursor-pointer"
                },
                class.unwrap_or(String::new())
            ),
            name: name.clone(),
            r#type,
            id: id.unwrap_or(format!("input_{name}")),
            oninput: move |event| {
                if let Some(oninput) = oninput {
                    oninput.call(event);
                }
            },
            onchange: move |event| {
                if let Some(onchange) = onchange {
                    onchange.call(event);
                }
            },
            onmounted: move |element| async move {
                if let Some(true) = autofocus {
                    let _ = element.set_focus(true).await;
                }
            },
            ..attributes
        }
    }
}
