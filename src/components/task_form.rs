use crate::components::category_input::CategoryInput;
use crate::components::project_select::ProjectSelect;
use crate::components::reoccurrence_input::ReoccurrenceIntervalInput;
use crate::components::subtasks_form::SubtasksForm;
use crate::models::category::{CalendarTime, Category, Reoccurrence};
use crate::models::task::NewTask;
use crate::models::task::Task;
use crate::route::Route;
use crate::server::tasks::{create_task, delete_task, edit_task};
use chrono::Duration;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_i18n::t;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
struct InputData {
    title: String,
    deadline: Option<String>,
    category_waiting_for: Option<String>,
    category_calendar_date: Option<String>,
    category_calendar_reoccurrence_length: Option<String>,
    category_calendar_time: Option<String>,
    category_calendar_reminder_offset_index: Option<String>,
    project_id: Option<String>,
}

#[component]
pub(crate) fn TaskForm(task: Option<Task>, on_successful_submit: EventHandler<()>) -> Element {
    let route = use_route::<Route>();
    let selected_category = use_signal(|| {
        if let Some(task) = &task {
            task.category.clone()
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
    });
    let category_calendar_reoccurrence_interval = use_signal(|| {
        task.as_ref().and_then(|task| {
            if let Category::Calendar {
                reoccurrence: Some(reoccurrence),
                ..
            } = &task.category
            {
                Some(reoccurrence.interval.clone())
            } else {
                None
            }
        })
    });
    let mut category_calendar_has_time = use_signal(|| {
        task.as_ref()
            .is_some_and(|task| matches!(task.category, Category::Calendar { time: Some(_), .. }))
    });
    let mut category_calendar_reminder_offset_index = use_signal(|| {
        task.as_ref()
            .and_then(|task| {
                if let Category::Calendar {
                    time: Some(time), ..
                } = &task.category
                {
                    REMINDER_OFFSETS
                        .iter()
                        .position(|&reminder_offset| reminder_offset == time.reminder_offset)
                } else {
                    None
                }
            })
            .unwrap_or(REMINDER_OFFSETS.len() - 1)
    });
    let task_for_submit = task.clone();

    rsx! {
        div {
            class: "p-4 flex flex-col gap-4",
            form {
                class: "flex flex-col gap-4",
                id: "form_task",
                onsubmit: move |event| {
                    event.prevent_default();
                    let task = task_for_submit.clone();
                    async move {
                        let input_data = event.parsed_values::<InputData>().unwrap();
                        let new_task = NewTask {
                            title: input_data.title,
                            deadline: input_data.deadline
                                .and_then(|deadline| deadline.parse().ok()),
                            category: match &selected_category() {
                                Category::WaitingFor(_) => Category::WaitingFor(
                                    input_data.category_waiting_for.unwrap()
                                ),
                                Category::Calendar { .. } => Category::Calendar {
                                    date: input_data.category_calendar_date.clone().unwrap().parse()
                                        .unwrap(),
                                    reoccurrence: category_calendar_reoccurrence_interval().map(
                                        |reoccurrence_interval| Reoccurrence {
                                            start_date: input_data.category_calendar_date.unwrap()
                                                .parse().unwrap(),
                                            interval: reoccurrence_interval,
                                            length: input_data.category_calendar_reoccurrence_length
                                                .unwrap().parse().unwrap()
                                        }
                                    ),
                                    time: input_data.category_calendar_time.unwrap().parse().ok()
                                        .map(|time| CalendarTime {
                                            time,
                                            reminder_offset: REMINDER_OFFSETS[
                                                input_data.category_calendar_reminder_offset_index
                                                    .unwrap().parse::<usize>().unwrap()
                                            ]
                                        }
                                    )
                                },
                                category => category.clone()
                            },
                            project_id: input_data.project_id
                                .and_then(|deadline| deadline.parse().ok()).filter(|&id| id > 0),
                        };
                        if let Some(task) = task {
                            let _ = edit_task(task.id, new_task).await;
                        } else {
                            let _ = create_task(new_task).await;
                        }
                        on_successful_submit.call(());
                    }
                },
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
                        initial_value: task.as_ref().map(|task| task.title.clone()),
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
                    SuspenseBoundary {
                        fallback: |_| {
                            rsx ! {
                                select {
                                    class: "px-3.5 py-2.5 bg-zinc-800/50 rounded-lg grow cursor-pointer",
                                    option {
                                        value: 0,
                                        {t!("none")}
                                    },
                                }
                            }
                        },
                        ProjectSelect {
                            initial_selected_id: task.clone().and_then(|task| task.project_id)
                        }
                    }
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
                        initial_value: task.as_ref().and_then(|task| task.deadline)
                            .map(|deadline| deadline.format("%Y-%m-%d").to_string()),
                        r#type: "date",
                        class: "py-2 px-3 bg-zinc-800/50 rounded-lg grow basis-0 cursor-pointer",
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
                                    class:
                                        "py-2 px-3 bg-zinc-800/50 rounded-lg grow cursor-pointer",
                                    id: "input_category_calendar_date"
                                },
                                input {
                                    r#type: "time",
                                    name: "category_calendar_time",
                                    initial_value: time.map(|calendar_time|
                                        calendar_time.time.format("%H:%M").to_string()
                                    ),
                                    class: "py-2 px-3 bg-zinc-800/50 rounded-lg grow cursor-pointer",
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
                                    initial_value: category_calendar_reoccurrence_interval().map_or(
                                        String::new(),
                                        |_| reoccurrence.map_or(1, |reoccurrence|
                                            reoccurrence.length).to_string()
                                    ),
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
                                    class: "grow input-range-reverse cursor-pointer",
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
                                    {REMINDER_OFFSETS[category_calendar_reminder_offset_index()]
                                        .map(
                                            |offset| if offset.num_hours() < 1 {
                                                format!("{} min", offset.num_minutes())
                                            } else {
                                                format!("{} h", offset.num_hours())
                                            }
                                        ).unwrap_or_else(|| t!("none"))}
                                }
                            }
                        }
                    },
                    _ => VNode::empty()
                }
            },
            if let Some(task) = task.as_ref() {
                SuspenseBoundary {
                    fallback: |_| {
                        VNode::empty()
                    },
                    SubtasksForm {
                        task: task.clone()
                    }
                }
            }
            div {
                class: "flex flex-row justify-between mt-auto",
                button {
                    r#type: "button",
                    class: "py-2 px-4 bg-zinc-300/50 rounded-lg cursor-pointer",
                    onclick: move |_| {
                        let task = task.clone();
                        async move {
                            if let Some(task) = task {
                                if let Category::Trash = task.category {
                                    let _ = delete_task(task.id).await;
                                } else {
                                    let new_task = NewTask {
                                        title: task.title.to_owned(),
                                        deadline: task.deadline,
                                        category: Category::Trash,
                                        project_id: task.project_id
                                    };
                                    let _ = edit_task(task.id, new_task).await;
                                }
                            }
                            on_successful_submit.call(());
                        }
                    },
                    i {
                        class: "fa-solid fa-trash-can"
                    }
                }
                button {
                    form: "form_task",
                    r#type: "submit",
                    class: "py-2 px-4 bg-zinc-300/50 rounded-lg cursor-pointer",
                    i {
                        class: "fa-solid fa-floppy-disk"
                    }
                }
            }
        }
    }
}
