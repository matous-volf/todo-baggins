use crate::components::error_boundary_message::ErrorBoundaryMessage;
use crate::models::category::Category;
use crate::views::category_page::CategoryPage;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn CategoryNextStepsPage() -> Element {
    rsx! {
        ErrorBoundaryMessage {
            CategoryPage {
                category: Category::NextSteps,
            }
        }
    }
}
