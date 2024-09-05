use crate::components::pages::category_inbox_page::CategoryInboxPage;
use crate::components::pages::category_next_steps_page::CategoryNextStepsPage;
use crate::components::pages::category_today_page::CategoryTodayPage;
use crate::components::pages::category_trash_page::CategoryTrashPage;
use crate::components::pages::category_waiting_for_page::CategoryWaitingForPage;
use crate::components::pages::category_someday_maybe_page::CategorySomedayMaybePage;
use crate::components::pages::category_done_page::CategoryDonePage;
use crate::components::pages::category_calendar_page::CategoryCalendarPage;
use crate::components::pages::category_long_term_page::CategoryLongTermPage;
use crate::components::pages::projects_page::ProjectsPage;
use crate::components::pages::not_found_page::NotFoundPage;
use crate::components::layout::Layout;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(Layout)]
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
