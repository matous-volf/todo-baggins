use chrono::Duration;
use crate::models::category::{CalendarTime, Category};
use crate::models::task::NewTask;
use crate::server::projects::get_projects;
use crate::server::tasks::create_task;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn TaskForm() -> Element {
    let categories = vec![
        Category::Inbox,
        Category::SomedayMaybe,
        Category::WaitingFor(String::new()),
        Category::NextSteps,
        Category::Calendar {
            date: chrono::Local::now().date_naive(),
            reoccurance_interval: None,
            time: None,
        },
        Category::LongTerm,
    ];
    let projects = use_server_future(get_projects)?.unwrap().unwrap();

    let mut selected_category_index = use_signal::<usize>(|| 0);
    let mut category_calendar_is_reoccurring = use_signal::<bool>(|| false);
    let mut category_calendar_has_time = use_signal::<bool>(|| false);
    let mut category_calendar_has_reminder = use_signal::<bool>(|| false);

    rsx! {
        form {
            onsubmit: move |event| {
                let categories = categories.clone();
                async move {
                    let new_task = NewTask::new(
                        event.values().get("title").unwrap().as_value(),
                        event.values().get("deadline").unwrap().as_value().parse().ok(),
                        match &categories[
                            event.values().get("category_index").unwrap()
                            .as_value().parse::<usize>().unwrap()
                        ] {
                            Category::WaitingFor(_) => Category::WaitingFor(
                                event.values().get("category_waiting_for").unwrap()
                                .as_value()
                            ),
                            Category::Calendar { .. } => Category::Calendar {
                                date: event.values().get("category_calendar_date").unwrap()
                                .as_value().parse().unwrap(),
                                reoccurance_interval:
                                event.values().get("category_calendar_is_reoccurring").map(
                                    |_| Duration::days(
                                        event.values().get("category_calendar_reoccurance_interval")
                                        .unwrap().as_value().parse().unwrap()
                                    )
                                ),
                                time: event.values().get("category_calendar_time").unwrap()
                                    .as_value().parse().ok().map(|time|
                                    CalendarTime::new(
                                        time,
                                        event.values().get("category_calendar_has_reminder").map(
                                            |_| Duration::minutes(
                                                event.values()
                                                .get("category_calendar_reminder_offset").unwrap()
                                                .as_value().parse().unwrap()
                                            )
                                        )
                                    )
                                )
                            },
                            category => category.clone()
                        },
                        event.values().get("project_id").unwrap()
                        .as_value().parse::<i32>().ok().filter(|&id| id > 0),
                    );
                    let _ = create_task(new_task).await;
                }
            },
            class: "p-4 flex flex-col gap-4",
            input {
                r#type: "text",
                name: "title",
                required: true,
                placeholder: "title",
                class: "p-2 bg-neutral-700 rounded",
            },
            select {
                name: "category_index",
                oninput: move |event| {
                    selected_category_index.set(event.value().parse().unwrap());
                },
                class: "p-2 bg-neutral-700 rounded",
                option {
                    value: 0,
                    "inbox"
                },
                option {
                    value: 1,
                    "someday maybe"
                },
                option {
                    value: 2,
                    "waiting for"
                },
                option {
                    value: 3,
                    "next steps"
                },
                option {
                    value: 4,
                    "calendar"
                },
                option {
                    value: 5,
                    "long term"
                },
            },
            match categories[selected_category_index()] {
                Category::WaitingFor(_) => rsx !{
                    input {
                        r#type: "text",
                        name: "category_waiting_for",
                        required: true,
                        class: "p-2 bg-neutral-700 rounded",
                    },
                },
                Category::Calendar { .. } => rsx !{
                    input {
                        r#type: "date",
                        name: "category_calendar_date",
                        required: true,
                        class: "p-2 bg-neutral-700 rounded",
                    },
                    div {
                        input {
                            r#type: "checkbox",
                            name: "category_calendar_is_reoccurring",
                            id: "category_calendar_is_reoccurring",
                            onchange: move |event| {
                                category_calendar_is_reoccurring.set(event.checked());
                            }
                        },
                        label {
                            r#for: "category_calendar_is_reoccurring",
                            " is reoccurring"
                        }
                    },
                    if category_calendar_is_reoccurring() {
                        input {
                            r#type: "number",
                            name: "category_calendar_reoccurance_interval",
                            required: true,
                            min: 1,
                            placeholder: "reoccurance interval (days)",
                            class: "p-2 bg-neutral-700 rounded",
                        }
                    },
                    input {
                        r#type: "time",
                        name: "category_calendar_time",
                        class: "p-2 bg-neutral-700 rounded",
                        oninput: move |event| {
                            category_calendar_has_time.set(!event.value().is_empty());
                        }
                    },
                    if category_calendar_has_time() {
                        div {
                            input {
                                r#type: "checkbox",
                                name: "category_calendar_has_reminder",
                                value: 0,
                                id: "category_calendar_has_reminder",
                                onchange: move |event| {
                                    category_calendar_has_reminder.set(event.checked());
                                }
                            },
                            label {
                                r#for: "category_calendar_has_reminder",
                                " set a reminder"
                            }
                        }
                    }
                    if category_calendar_has_reminder() {
                        input {
                            r#type: "number",
                            name: "category_calendar_reminder_offset",
                            required: true,
                            min: 0,
                            placeholder: "reminder offset (minutes)",
                            class: "p-2 bg-neutral-700 rounded",
                        }
                    }
                },
                _ => None
            },
            input {
                r#type: "date",
                name: "deadline",
                class: "p-2 bg-neutral-700 rounded",
            },
            select {
                name: "project_id",
                class: "p-2 bg-neutral-700 rounded",
                option {
                    value: 0,
                    "none"
                },
                for project in projects {
                    option {
                        value: project.id().to_string(),
                        {project.title()}
                    }
                }
            },
            button {
                r#type: "submit",
                "create"
            }
        }
}
}
