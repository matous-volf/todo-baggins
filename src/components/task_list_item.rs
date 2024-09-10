use crate::internationalization::LocaleFromLanguageIdentifier;
use crate::models::category::Category;
use crate::models::task::TaskWithSubtasks;
use chrono::{Datelike, Local};
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_sdk::i18n::use_i18;
use dioxus_sdk::translate;
use voca_rs::Voca;

#[component]
pub(crate) fn TaskListItem(task: TaskWithSubtasks) -> Element {
    let i18 = use_i18();

    rsx! {
        div {
            class: "flex flex-col",
            div {
                class: "mt-1 grow font-medium",
                {task.task().title()}
            },
            div {
                class: "flex flex-row gap-4",
                if let Some(deadline) = task.task().deadline() {
                    div {
                        class: "text-sm text-zinc-400",
                        i {
                            class: "fa-solid fa-bomb"
                        },
                        {
                            let today_date = Local::now().date_naive();
                            format!(
                                " {}",
                                if deadline == today_date - chrono::Days::new(1) {
                                    translate!(i18, "yesterday")
                                } else if deadline == today_date {
                                    translate!(i18, "today")
                                } else if deadline == today_date + chrono::Days::new(1) {
                                    translate!(i18, "tomorrow")
                                } else if deadline > today_date
                                    && deadline <= today_date + chrono::Days::new(7) {
                                    let deadline = deadline.format_localized(
                                        "%A",
                                        LocaleFromLanguageIdentifier::from(
                                            &(i18.selected_language)()
                                        ).into()
                                    ).to_string();
                                    if translate!(i18, "formats.weekday_lowercase_first")
                                        .parse().unwrap() {
                                        deadline._lower_first()
                                    } else {
                                        deadline
                                    }
                                } else {
                                    let format = translate!(i18,
                                        if deadline.year() == today_date.year() {
                                            "formats.date_format"
                                        } else {
                                            "formats.date_year_format"
                                        }
                                    );
                                    deadline.format_localized(
                                        format.as_str(),
                                        LocaleFromLanguageIdentifier::from(
                                            &(i18.selected_language)()
                                        ).into()
                                    ).to_string()
                                }
                            )
                        }
                    }
                }
                if let Category::Calendar { time, .. } = task.task().category() {
                    if let Some(calendar_time) = time {
                        div {
                            class: "text-sm text-zinc-400",
                            i {
                                class: "fa-solid fa-clock"
                            },
                            {
                                let format = translate!(i18, "formats.time_format");
                                format!(" {}",calendar_time.time().format(format.as_str()))
                            }
                        }
                    }
                }
                if !task.subtasks().is_empty() {
                    div {
                        class: "text-sm text-zinc-400",
                        i {
                            class: "fa-solid fa-list-check"
                        },
                        {format!(
                            " {}/{}",
                            task.subtasks().iter()
                                .filter(|subtask| subtask.is_completed())
                                .count(),
                            task.subtasks().len()
                        )}
                    }
                }
            }
        }
    }
}
