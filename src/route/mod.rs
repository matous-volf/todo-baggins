use crate::layouts;
use crate::views::category_calendar_page::CategoryCalendarPage;
use crate::views::category_done_page::CategoryDonePage;
use crate::views::category_inbox_page::CategoryInboxPage;
use crate::views::category_long_term_page::CategoryLongTermPage;
use crate::views::category_next_steps_page::CategoryNextStepsPage;
use crate::views::category_someday_maybe_page::CategorySomedayMaybePage;
use crate::views::category_today_page::CategoryTodayPage;
use crate::views::category_trash_page::CategoryTrashPage;
use crate::views::category_waiting_for_page::CategoryWaitingForPage;
use crate::views::not_found_page::NotFoundPage;
use crate::views::projects_page::ProjectsPage;
use dioxus::prelude::*;

// All variants have the same postfix because they have to match the component names.
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(layouts::Main)]
        #[redirect("/", || Route::CategoryTodayPage {})]
        #[route("/today")]
        CategoryTodayPage,
        #[route("/inbox")]
        CategoryInboxPage,
        #[route("/someday-maybe")]
        CategorySomedayMaybePage,
        #[route("/waiting-for")]
        CategoryWaitingForPage,
        #[route("/next-steps")]
        CategoryNextStepsPage,
        #[route("/calendar")]
        CategoryCalendarPage,
        #[route("/long-term")]
        CategoryLongTermPage,
        #[route("/done")]
        CategoryDonePage,
        #[route("/trash")]
        CategoryTrashPage,
        #[route("/projects")]
        ProjectsPage,
    #[end_layout]
    #[redirect("/", || Route::CategoryTodayPage)]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
