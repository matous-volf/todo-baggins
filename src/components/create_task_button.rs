use dioxus::prelude::*;
use crate::components::task_list::TaskList;
use crate::models::category::Category;
use crate::route::Route;

#[component]
pub(crate) fn CreateButton(creating: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "m-4 py-3 px-5 self-end text-center bg-zinc-300/50 rounded-xl border-t-zinc-200 border-t backdrop-blur drop-shadow-[0_-5px_10px_rgba(0,0,0,0.2)] text-2xl text-zinc-200",
            onclick: move |_| {
                creating.set(!creating());
            },
            i {
                class: format!("min-w-6 fa-solid {}", if creating() { "fa-xmark" } else { "fa-plus" }),
            }
        }
    }
}
