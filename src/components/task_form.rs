use crate::components::button_primary::ButtonPrimary;
use crate::components::button_secondary::ButtonSecondary;
use crate::components::category_input::CategoryInput;
use crate::components::input::Input;
use crate::components::input_label::InputLabel;
use crate::components::project_select::ProjectSelect;
use crate::components::reoccurrence_interval_input::ReoccurrenceIntervalInput;
use crate::components::subtasks_form::SubtasksForm;
use crate::models::category::{CalendarTime, Category, Reoccurrence};
use crate::models::task::NewTask;
use crate::models::task::Task;
use crate::server::tasks::{create_task, delete_task, edit_task};
use chrono::Duration;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{
    FaBell, FaBomb, FaClock, FaFeatherPointed, FaHourglassEnd, FaList, FaRepeat, FaScroll, FaStamp,
    FaTrashCan, FaXmark,
};
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

pub(crate) static TASK_BEING_EDITED: GlobalSignal<Option<Task>> = Signal::global(|| None);
pub(crate) static LATEST_VISITED_CATEGORY: GlobalSignal<Category> =
    Signal::global(|| Category::Inbox);

#[component]
pub(crate) fn TaskForm() -> Element {
    let navigator = use_navigator();
    let task = TASK_BEING_EDITED();
    let selected_category = use_signal(|| {
        if let Some(task) = &task {
            task.category.clone()
        } else {
            LATEST_VISITED_CATEGORY()
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
            class: "grow px-4 flex flex-col gap-6.5",
            form {
                class: "flex flex-col gap-8",
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
                        let result = if let Some(task) = task {
                            edit_task(task.id, new_task).await
                        } else {
                            create_task(new_task).await
                        };
                        if result.is_ok() {
                            navigator.go_back();
                        }
                    }
                },
                div {
                    class: "flex flex-row items-center gap-3",
                    InputLabel {
                        r#for: "input_title",
                        icon: FaFeatherPointed
                    },
                    Input {
                        class: "grow",
                        name: "title",
                        required: true,
                        initial_value: task.as_ref().map(|task| task.title.clone()),
                        r#type: "text",
                        autofocus: task.is_none()
                    }
                },
                div {
                    class: "flex flex-row items-center gap-3",
                    InputLabel {
                        r#for: "input_project_id",
                        icon: FaList
                    },
                    SuspenseBoundary {
                        fallback: |_| {
                            rsx ! {
                                select {
                                    class: "px-4 pt-3 pb-2.25 bg-gray-800-muted drop-shadow-[0_calc(0px_-_var(--spacing))_0_var(--color-gray-900-muted)] rounded-xl grow cursor-pointer",
                                    option {
                                        value: 0
                                    }
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
                    InputLabel {
                        icon: FaBomb,
                        r#for: "input_deadline"
                    },
                    Input {
                        name: "deadline",
                        initial_value: task.as_ref().and_then(|task| task.deadline)
                            .map(|deadline| deadline.format("%Y-%m-%d").to_string()),
                        r#type: "date",
                        class: "grow basis-0"
                    }
                },
                div {
                    class: "flex flex-row items-center gap-3",
                    InputLabel {
                        icon: FaScroll
                    },
                    CategoryInput {
                        class: "grow",
                        selected_category: selected_category
                    }
                }
                match selected_category() {
                    Category::WaitingFor(waiting_for) => rsx! {
                        div {
                            class: "flex flex-row items-center gap-3",
                            InputLabel {
                                icon: FaHourglassEnd,
                                r#for: "input_category_waiting_for",
                            },
                            Input {
                                class: "grow",
                                name: "category_waiting_for",
                                required: true,
                                initial_value: waiting_for,
                                r#type: "text",
                            }
                        }
                    },
                    Category::Calendar { date, reoccurrence, time } => rsx! {
                        div {
                            class: "flex flex-row items-center gap-3",
                            InputLabel {
                                icon: FaClock,
                                r#for: "input_category_calendar_date"
                            },
                            div {
                                class: "grow grid grid-cols-7 gap-3",
                                Input {
                                    class: "grow col-span-4",
                                    name: "category_calendar_date",
                                    r#type: "date",
                                    required: true,
                                    initial_value: date.format("%Y-%m-%d").to_string(),
                                },
                                Input {
                                    class: "grow col-span-3",
                                    name: "category_calendar_time",
                                    r#type: "time",
                                    initial_value: time.map(|calendar_time|
                                        calendar_time.time.format("%H:%M").to_string()
                                    ),
                                    oninput: move |event: Event<FormData>| {
                                        category_calendar_has_time.set(!event.value().is_empty());
                                    }
                                }
                            }
                        },
                        div {
                            class: "flex flex-row items-center gap-3",
                            InputLabel {
                                icon: FaRepeat,
                                r#for: "category_calendar_reoccurrence_length"
                            },
                            div {
                                class: "grow grid grid-cols-5 items-end gap-3",
                                ReoccurrenceIntervalInput {
                                    reoccurrence_interval: category_calendar_reoccurrence_interval
                                },
                                Input {
                                    class: "text-right",
                                    r#type: "number",
                                    inputmode: "numeric",
                                    name: "category_calendar_reoccurrence_length",
                                    disabled: category_calendar_reoccurrence_interval().is_none(),
                                    required: true,
                                    min: "1",
                                    initial_value: category_calendar_reoccurrence_interval().map_or(
                                        String::new(),
                                        |_| reoccurrence.map_or(1, |reoccurrence|
                                            reoccurrence.length).to_string()
                                    )
                                }
                            }
                        },
                        if category_calendar_has_time() {
                            div {
                                class: "flex flex-row items-center gap-3",
                                InputLabel {
                                    r#for: "input_category_calendar_reminder_offset_index",
                                    icon: FaBell
                                },
                                input {
                                    class: "grow",
                                    name: "category_calendar_reminder_offset_index",
                                    r#type: "range",
                                    min: 0,
                                    max: REMINDER_OFFSETS.len() as i64 - 1,
                                    initial_value: category_calendar_reminder_offset_index()
                                        .to_string(),
                                    oninput: move |event| {
                                        category_calendar_reminder_offset_index.set(
                                            event.value().parse().unwrap()
                                        );
                                    }
                                },
                                label {
                                    class: "pr-3 min-w-16 text-right",
                                    r#for: "category_calendar_reminder_offset_index",
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
        }
        div {
            class: "px-4 grid grid-cols-3 gap-3 mt-auto",
            ButtonSecondary {
                r#type: "button",
                class: "grow",
                onclick: {
                    let task = task.clone();
                    move |_| {
                        let task = task.clone();
                        async move {
                            if let Some(task) = task {
                                let result = if let Category::Trash = task.category {
                                    delete_task(task.id).await
                                } else {
                                    let new_task = NewTask {
                                        title: task.title.to_owned(),
                                        deadline: task.deadline,
                                        category: Category::Trash,
                                        project_id: task.project_id
                                    };
                                    edit_task(task.id, new_task).await.map(|_| ())
                                };
                                if result.is_ok() {
                                    navigator.go_back();
                                }
                            } else {
                                navigator.go_back();
                            }
                        }
                    }
                },
                Icon {
                    icon: FaTrashCan,
                    height: 16,
                    width: 16
                }
            }
            if task.is_some() {
                div {
                    class: "grow flex flex-col items-stretch",
                    GoBackButton {
                        ButtonSecondary {
                            /* TODO: Replace w-full` with proper flexbox styling once
                            https://github.com/DioxusLabs/dioxus/issues/5269 is solved. */
                            class: "w-full",
                            r#type: "button",
                            Icon {
                                icon: FaXmark,
                                height: 16,
                                width: 16
                            }
                        }
                    }
                }
            } else {
                div {}
            }
            ButtonPrimary {
                form: "form_task",
                r#type: "submit",
                Icon {
                    icon: FaStamp,
                    height: 16,
                    width: 16
                }
            }
        }
    }
}
