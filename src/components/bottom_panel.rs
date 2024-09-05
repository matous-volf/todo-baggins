use dioxus::prelude::*;
use crate::components::navigation::Navigation;
use crate::components::project_form::ProjectForm;
use crate::components::task_form::TaskForm;
use crate::route::Route;

#[component]
pub(crate) fn BottomPanel(display_form: Signal<bool>) -> Element {
    let mut expanded = use_signal(|| display_form());
    let navigation_expanded = use_signal(|| false);
    let current_route = use_route();

    use_effect(use_reactive(&display_form, move |creating_task| {
        if creating_task() {
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
                match (display_form(), current_route, navigation_expanded()) {
                    (false, _, false) => "h-[64px]",
                    (false, _, true) => "h-[128px]",
                    (true, Route::ProjectsPage, _) => "h-[128px]",
                    (true, _, _) => "h-[448px]",
                }
            ),
            if expanded() {
                match current_route {
                    Route::ProjectsPage => rsx! {
                        ProjectForm {
                            on_successful_submit: move |_| {
                                display_form.set(false);
                            }
                        }
                    },
                    _ => rsx! {
                        TaskForm {
                            on_successful_submit: move |_| {
                                display_form.set(false);
                            }
                        } 
                    }
                }
            } else {
                Navigation {
                    expanded: navigation_expanded,
                }
            }
        }
    }
}
