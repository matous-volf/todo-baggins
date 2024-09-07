use crate::components::category_input::CategoryInput;
use crate::components::reoccurrence_input::ReoccurrenceIntervalInput;
use crate::models::category::{CalendarTime, Category, Reoccurrence};
use crate::models::task::NewTask;
use crate::models::task::Task;
use crate::query::{QueryErrors, QueryKey, QueryValue};
use crate::route::Route;
use crate::server::projects::get_projects;
use crate::server::tasks::{create_task, edit_task};
use chrono::{Duration};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_query::prelude::use_query_client;

const REMINDER_OFFSETS: [Option<Duration>; 17] = [
    None,
    Some(Duration::days(1)),
    Some(Duration::hours(12)),
    Some(Duration::hours(11)),
    Some(Duration::hours(10)),
    Some(Duration::hours(9)),
    Some(Duration::hours(8)),
    Some(Duration::hours(7)),
    Some(Duration::hours(6)),
    Some(Duration::hours(5)),
    Some(Duration::hours(4)),
    Some(Duration::hours(3)),
    Some(Duration::hours(2)),
    Some(Duration::hours(1)),
    Some(Duration::minutes(30)),
    Some(Duration::minutes(10)),
    Some(Duration::zero()),
];

#[component]
pub(crate) fn TaskForm(task: Option<Task>, on_successful_submit: EventHandler<()>) -> Element {
    let projects = use_server_future(get_projects)?.unwrap().unwrap();

    let route = use_route::<Route>();
    let selected_category = use_signal(|| if let Some(task) = &task {
        task.category().clone()
    } else {
        match route {
            Route::CategorySomedayMaybePage => Category::SomedayMaybe,
            Route::CategoryWaitingForPage => Category::WaitingFor(String::new()),
            Route::CategoryNextStepsPage => Category::NextSteps,
            Route::CategoryCalendarPage | Route::CategoryTodayPage => Category::Calendar {
                date: chrono::Local::now().date_naive(),
                reoccurrence: None,
                time: None,
            },
            Route::CategoryLongTermPage => Category::LongTerm,
            _ => Category::Inbox,
        }
    }
    );
    let category_calendar_reoccurrence_interval = use_signal(|| task.as_ref().and_then(|task|
        if let Category::Calendar { reoccurrence: Some(reoccurrence), .. } = task.category() {
            Some(reoccurrence.interval().clone())
        } else {
            None
        }
    ));
    let mut category_calendar_has_time = use_signal(|| task.as_ref().is_some_and(
        |task| matches!(*task.category(), Category::Calendar { time: Some(_), .. }))
    );
    let mut category_calendar_reminder_offset_index = use_signal(|| task.as_ref().and_then(|task|
        if let Category::Calendar { time: Some(time), .. } = task.category() {
            REMINDER_OFFSETS.iter().position(|&reminder_offset|
                reminder_offset == time.reminder_offset()
            )
        } else {
            None
        }
    ).unwrap_or(REMINDER_OFFSETS.len() - 1));

    let query_client = use_query_client::<QueryValue, QueryErrors, QueryKey>();
    let task_for_submit = task.clone();

    rsx! {
        form {
            onsubmit: move |event| {
                let task = task_for_submit.clone();
                async move {
                    let new_task = NewTask::new(
                        event.values().get("title").unwrap().as_value(),
                        event.values().get("deadline").unwrap().as_value().parse().ok(),
                        match &selected_category() {
                            Category::WaitingFor(_) => Category::WaitingFor(
                                event.values().get("category_waiting_for").unwrap()
                                .as_value()
                            ),
                            Category::Calendar { .. } => Category::Calendar {
                                date: event.values().get("category_calendar_date").unwrap()
                                .as_value().parse().unwrap(),
                                reoccurrence: category_calendar_reoccurrence_interval().map(
                                    |reoccurrence_interval| Reoccurrence::new(
                                        event.values().get("category_calendar_date").unwrap()
                                        .as_value().parse().unwrap(),
                                        reoccurrence_interval,
                                        event.values().get("category_calendar_reoccurrence_length")
                                        .unwrap().as_value().parse().unwrap()
                                    )
                                ),
                                time: event.values().get("category_calendar_time").unwrap()
                                .as_value().parse().ok().map(|time|
                                    CalendarTime::new(
                                        time,
                                        REMINDER_OFFSETS[
                                            event.values()
                                            .get("category_calendar_reminder_offset_index").unwrap()
                                            .as_value().parse::<usize>().unwrap()
                                        ]
                                    )
                                )
                            },
                            category => category.clone()
                        },
                        event.values().get("project_id").unwrap()
                        .as_value().parse::<i32>().ok().filter(|&id| id > 0),
                    );
                    if let Some(task) = task {
                        let _ = edit_task(task.id(), new_task).await;
                    } else {
                        let _ = create_task(new_task).await;
                    }
                    query_client.invalidate_queries(&[
                        QueryKey::Tasks, 
                        QueryKey::TasksInCategory(selected_category())
                    ]);
                    on_successful_submit.call(());
                }
            },
            class: "p-4 flex flex-col gap-4",
            div {
                class: "flex flex-row items-center gap-3",
                label {
                    r#for: "input_title",
                    class: "min-w-6 text-center",
                    i {
                        class: "fa-solid fa-pen-clip text-zinc-400/50"
                    },
                },
                input {
                    name: "title",
                    required: true,
                    initial_value: task.as_ref().map(|task| task.title().to_owned()),
                    r#type: "text",
                    class: "py-2 px-3 grow bg-zinc-800/50 rounded-lg",
                    id: "input_title"
                },
            },
            div {
                class: "flex flex-row items-center gap-3",
                label {
                    r#for: "input_project",
                    class: "min-w-6 text-center",
                    i {
                        class: "fa-solid fa-list text-zinc-400/50"
                    }
                },
                select {
                    name: "project_id",
                    class: "px-3.5 py-2.5 bg-zinc-800/50 rounded-lg grow",
                    id: "input_project",
                    option {
                        value: 0,
                        "None"
                    },
                    for project in projects {
                        option {
                            value: project.id().to_string(),
                            selected: task.as_ref().is_some_and(
                                |task| task.project_id() == Some(project.id())
                            ),
                            {project.title()}
                        }
                    }
                },
            },
            div {
                class: "flex flex-row items-center gap-3",
                label {
                    r#for: "input_deadline",
                    class: "min-w-6 text-center",
                    i {
                        class: "fa-solid fa-bomb text-zinc-400/50"
                    }
                },
                input {
                    name: "deadline",
                    initial_value: task.as_ref().and_then(|task| task.deadline())
                        .map(|deadline| deadline.format("%Y-%m-%d").to_string()),
                    r#type: "date",
                    class: "py-2 px-3 bg-zinc-800/50 rounded-lg grow basis-0",
                    id: "input_deadline"
                }
            },
            div {
                class: "flex flex-row items-center gap-3",
                label {
                    class: "min-w-6 text-center",
                    i {
                        class: "fa-solid fa-layer-group text-zinc-400/50"
                    }
                },
                CategoryInput {
                    selected_category: selected_category,
                    class: "grow"
                }
            }
            match selected_category() {
                Category::WaitingFor(waiting_for) => rsx! {
                    div {
                        class: "flex flex-row items-center gap-3",
                        label {
                            r#for: "input_deadline",
                            class: "min-w-6 text-center",
                            i {
                                class: "fa-solid fa-hourglass-end text-zinc-400/50"
                            }
                        },
                        input {
                            name: "category_waiting_for",
                            required: true,
                            initial_value: waiting_for,
                            r#type: "text",
                            class: "py-2 px-3 bg-zinc-800/50 rounded-lg grow",
                            id: "input_category_waiting_for"
                        },
                    }
                },
                Category::Calendar { date, reoccurrence, time } => rsx! {
                    div {
                        class: "flex flex-row items-center gap-3",
                        label {
                            r#for: "input_category_calendar_date",
                            class: "min-w-6 text-center",
                            i {
                                class: "fa-solid fa-clock text-zinc-400/50"
                            }
                        },
                        div {
                            class: "grow flex flex-row gap-2",
                            input {
                                r#type: "date",
                                name: "category_calendar_date",
                                required: true,
                                initial_value: date.format("%Y-%m-%d").to_string(),
                                class: "py-2 px-3 bg-zinc-800/50 rounded-lg grow",
                                id: "input_category_calendar_date"
                            },
                            input {
                                r#type: "time",
                                name: "category_calendar_time",
                                initial_value: time.map(|calendar_time|
                                    calendar_time.time().format("%H:%M").to_string()
                                ),
                                class: "py-2 px-3 bg-zinc-800/50 rounded-lg grow",
                                id: "input_category_calendar_time",
                                oninput: move |event| {
                                    category_calendar_has_time.set(!event.value().is_empty());
                                }
                            }
                        }
                    },
                    div {
                        class: "flex flex-row items-center gap-3",
                        label {
                            r#for: "category_calendar_reoccurrence_length",
                            class: "min-w-6 text-center",
                            i {
                                class: "fa-solid fa-repeat text-zinc-400/50"
                            }
                        },
                        div {
                            class: "grow grid grid-cols-6 gap-2",
                            ReoccurrenceIntervalInput {
                                reoccurrence_interval: category_calendar_reoccurrence_interval
                            },
                            input {
                                r#type: "number",
                                inputmode: "numeric",
                                name: "category_calendar_reoccurrence_length",
                                disabled: category_calendar_reoccurrence_interval().is_none(),
                                required: true,
                                min: 1,
                                initial_value: category_calendar_reoccurrence_interval()
                                    .map_or(String::new(), |_| reoccurrence.map_or(1, |reoccurrence|
                                        reoccurrence.length()).to_string()),
                                class: "py-2 px-3 bg-zinc-800/50 rounded-lg col-span-2 text-right",
                                id: "category_calendar_reoccurrence_length"
                            }
                        }
                    },
                    if category_calendar_has_time() {
                        div {
                            class: "flex flex-row items-center gap-3",
                            label {
                                r#for: "category_calendar_reminder_offset_index",
                                class: "min-w-6 text-center",
                                i {
                                    class: "fa-solid fa-bell text-zinc-400/50"
                                }
                            },
                            input {
                                r#type: "range",
                                name: "category_calendar_reminder_offset_index",
                                min: 0,
                                max: REMINDER_OFFSETS.len() as i64 - 1,
                                initial_value: category_calendar_reminder_offset_index()
                                    .to_string(),
                                class: "grow input-range-reverse",
                                id: "category_calendar_has_reminder",
                                oninput: move |event| {
                                    category_calendar_reminder_offset_index.set(
                                        event.value().parse().unwrap()
                                    );
                                }
                            },
                            label {
                                r#for: "category_calendar_reminder_offset_index",
                                class: "pr-3 min-w-16 text-right",
                                {REMINDER_OFFSETS[category_calendar_reminder_offset_index()].map(
                                    |offset| if offset.num_hours() < 1 {
                                        format!("{} min", offset.num_minutes())
                                    } else {
                                        format!("{} h", offset.num_hours())
                                    }
                                ).unwrap_or_else(|| "none".to_string())}
                            }
                        }
                    }
                },
                _ => None
            },
            div {
                class: "flex flex-row justify-end mt-auto",
                button {
                    r#type: "submit",
                    class: "py-2 px-4 bg-zinc-300/50 rounded-lg",
                    i {
                        class: "fa-solid fa-floppy-disk"
                    }
                }
            }
        }
    }
}
