use dioxus::prelude::*;
use crate::models::project::Project;

#[component]
pub(crate) fn FormOpenButton(opened: Signal<bool>) -> Element {
    let mut project_being_edited = use_context::<Signal<Option<Project>>>();
    
    rsx! {
        button {
            class: "m-4 py-3 px-5 self-end text-center bg-zinc-300/50 rounded-xl border-t-zinc-200 border-t backdrop-blur drop-shadow-[0_-5px_10px_rgba(0,0,0,0.2)] text-2xl text-zinc-200",
            onclick: move |_| {
                if opened() {
                    project_being_edited.set(None);
                }
                opened.set(!opened());
            },
            i {
                class: format!("min-w-6 fa-solid {}", if opened() { "fa-xmark" } else { "fa-plus" }),
            }
        }
    }
}
