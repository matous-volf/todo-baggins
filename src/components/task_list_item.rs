use crate::internationalization::LocaleFromLanguageIdentifier;
use crate::models::category::Category;
use crate::models::task::TaskWithSubtasks;
use chrono::{Datelike, Local};
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaBomb, FaClock, FaListCheck};
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::t;
use voca_rs::Voca;

#[component]
pub(crate) fn TaskListItem(task: TaskWithSubtasks) -> Element {
    let today_date = Local::now().date_naive();
    rsx! {
        div {
            class: "pt-0.75 flex flex-col",
            div {
                class: "grow font-medium text-pretty wrap-anywhere",
                {task.task.title}
            },
            div {
                class: "flex flex-row gap-4",
                if let Some(deadline) = task.task.deadline {
                    div {
                        class: "flex flex-row items-center gap-1 text-sm text-zinc-400",
                        Icon {
                            icon: FaBomb,
                            height: 14,
                            width: 14
                        }
                        {
                            format!(
                                " {}",
                                if deadline == today_date - chrono::Days::new(1) {
                                    t!("yesterday")
                                } else if deadline == today_date {
                                    t!("today")
                                } else if deadline == today_date + chrono::Days::new(1) {
                                    t!("tomorrow")
                                } else if deadline > today_date
                                    && deadline <= today_date + chrono::Days::new(7) {
                                    let deadline = deadline.format_localized(
                                        "%A",
                                        LocaleFromLanguageIdentifier::from(
                                            &i18n().language()
                                        ).into()
                                    ).to_string();
                                    if t!("weekday-lowercase-first")
                                        .parse().unwrap() {
                                        deadline._lower_first()
                                    } else {
                                        deadline
                                    }
                                } else {
                                    let format = t!(
                                        if deadline.year() == today_date.year() {
                                            "date-format"
                                        } else {
                                            "date-year-format"
                                        }
                                    );
                                    deadline.format_localized(
                                        format.as_str(),
                                        LocaleFromLanguageIdentifier::from(
                                            &i18n().language()
                                        ).into()
                                    ).to_string()
                                }
                            )
                        }
                    }
                }
                if let Category::Calendar { time, .. } = task.task.category {
                    if let Some(calendar_time) = time {
                        div {
                            class: "flex flex-row items-center gap-1 text-sm text-zinc-400",
                            Icon {
                                icon: FaClock,
                                height: 14,
                                width: 14
                            }
                            {
                                let format = t!("time-format");
                                format!(" {}", calendar_time.time.format(format.as_str()))
                            }
                        }
                    }
                }
                if !task.subtasks.is_empty() {
                    div {
                        class: "flex flex-row items-center gap-1 text-sm text-zinc-400",
                        Icon {
                            icon: FaListCheck,
                            height: 14,
                            width: 14
                        }
                        {format!(
                            " {}/{}",
                            task.subtasks.iter()
                                .filter(|subtask| subtask.is_completed)
                                .count(),
                            task.subtasks.len()
                        )}
                    }
                }
            }
        }
    }
}
