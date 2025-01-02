use crate::query::{QueryErrors, QueryKey, QueryValue};
use crate::route::Route;
use crate::server::internationalization::get_language_identifier;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::unic_langid::langid;
use dioxus_query::prelude::use_init_query_client;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/styles/tailwind_output.css");
const FONTS_CSS: Asset = asset!("/assets/styles/fonts.css");
const INPUT_NUMBER_ARROWS_CSS: Asset = asset!("/assets/styles/input_number_arrows.css");
const INPUT_RANGE_CSS: Asset = asset!("/assets/styles/input_range.css");

#[component]
pub(crate) fn App() -> Element {
    use_init_query_client::<QueryValue, QueryErrors, QueryKey>();

    let language_identifier = use_server_future(get_language_identifier)?
        .unwrap()
        .unwrap();
    use_init_i18n(|| {
        I18nConfig::new(language_identifier)
            .with_locale(Locale::new_static(
                langid!("cs-CZ"),
                include_str!("../internationalization/cs_cz.ftl"),
            ))
            .with_locale(Locale::new_static(
                langid!("en-US"),
                include_str!("../internationalization/en_us.ftl"),
            ))
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: FONTS_CSS }
        document::Link { rel: "stylesheet", href: INPUT_NUMBER_ARROWS_CSS }
        document::Link { rel: "stylesheet", href: INPUT_RANGE_CSS }
        document::Script { src: "https://kit.fontawesome.com/3c1b409f8f.js" }

        div {
            class: "min-h-screen text-zinc-200 bg-zinc-800 pt-4 pb-36",
            Router::<Route> {}
        }
    }
}
