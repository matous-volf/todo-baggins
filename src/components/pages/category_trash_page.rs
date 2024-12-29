use crate::components::pages::category_page::CategoryPage;
use crate::models::category::Category;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub(crate) fn CategoryTrashPage() -> Element {
    rsx! {
        CategoryPage {
            category: Category::Trash,
        }
    }
}
