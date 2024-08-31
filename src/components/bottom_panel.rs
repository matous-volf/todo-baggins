use std::thread::sleep;
use dioxus::prelude::*;
use crate::components::navigation::Navigation;
use crate::components::task_form::TaskForm;
use crate::components::task_list::TaskList;
use crate::models::category::Category;
use crate::route::Route;

#[component]
pub(crate) fn BottomPanel(creating_task: bool) -> Element {
    let mut expanded = use_signal(|| creating_task);
    let navigation_expanded = use_signal(|| false);

    use_effect(use_reactive(&creating_task, move |creating_task| {
        if creating_task {
            expanded.set(true);
        } else {
            spawn(async move {
                async_std::task::sleep(std::time::Duration::from_millis(500)).await;
                expanded.set(false);
            });
        }
    }));

    rsx! {
        div {
            class: format!(
                "bg-zinc-700/50 rounded-t-xl border-t-zinc-600 border-t backdrop-blur drop-shadow-[0_-5px_10px_rgba(0,0,0,0.2)] transition-[height] duration-[500ms] ease-[cubic-bezier(0.79,0.14,0.15,0.86)] {}",
                match (creating_task, navigation_expanded()) {
                    (false, false) => "h-[64px]",
                    (false, true) => "h-[128px]",
                    (true, _) => "h-[448px]",
                }
            ),
            if expanded() {
                TaskForm {}
            } else {
                Navigation {
                    expanded: navigation_expanded,
                }
            }
        }
    }
}
