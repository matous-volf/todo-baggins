use crate::components::navigation::Navigation;
use crate::components::project_form::ProjectForm;
use crate::components::task_form::TaskForm;
use crate::models::project::Project;
use crate::models::task::Task;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn BottomPanel(display_form: Signal<bool>) -> Element {
    // A signal for delaying the application of styles.
    #[allow(clippy::redundant_closure)]
    let mut expanded = use_signal(|| display_form());
    let navigation_expanded = use_signal(|| false);
    let current_route = use_route();

    let mut project_being_edited = use_context::<Signal<Option<Project>>>();
    let mut task_being_edited = use_context::<Signal<Option<Task>>>();

    use_effect(use_reactive(&display_form, move |display_form| {
        if display_form() {
            expanded.set(true);
        } else {
            spawn(async move {
                // Necessary for a smooth – not instant – height transition.
                async_std::task::sleep(std::time::Duration::from_millis(500)).await;
                /* The check is necessary for the situation when the user expands the panel while
                   it is being closed. */
                if !display_form() {
                    expanded.set(false);
                }
            });
        }
    }));

    rsx! {
        div {
            class: format!(
                "pointer-events-auto bg-zinc-700/50 rounded-t-xl border-t-zinc-600 border-t backdrop-blur drop-shadow-[0_-5px_10px_rgba(0,0,0,0.2)] transition-[height] duration-[500ms] ease-[cubic-bezier(0.79,0.14,0.15,0.86)] {}",
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
                            project: project_being_edited(),
                            on_successful_submit: move |_| {
                                display_form.set(false);
                                project_being_edited.set(None);
                            }
                        }
                    },
                    _ => rsx! {
                        TaskForm {
                            task: task_being_edited(),
                            on_successful_submit: move |_| {
                                display_form.set(false);
                                task_being_edited.set(None);
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
