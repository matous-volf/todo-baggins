use crate::models::category::Category;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::pages::category_page::CategoryPage;

#[component]
pub(crate) fn CategoryLongTermPage() -> Element {
    rsx! {
        CategoryPage {
            category: Category::LongTerm,
        }
    }
}
