use crate::internationalization::get_language_identifier;
use crate::route::Route;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::unic_langid::langid;

const FAVICON: Asset = asset!("/assets/favicon.ico");
/* Once https://github.com/DioxusLabs/dioxus/issues/4490 is resolved, hopefully it will be
sufficient to just include the single icon.png. */
#[used]
static IMAGES_DIRECTORY: Asset = asset!(
    "/assets/images",
    AssetOptions::builder().with_hash_suffix(false)
);
#[used]
static FONTS_DIRECTORY: Asset = asset!(
    "/assets/fonts",
    AssetOptions::builder().with_hash_suffix(false)
);
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const INPUT_NUMBER_ARROWS_CSS: Asset = asset!("/assets/styles/input_number_arrows.css");
const INPUT_RANGE_CSS: Asset = asset!("/assets/styles/input_range.css");
const SELECT_ARROW_CSS: Asset = asset!("/assets/styles/select_arrow.css");
const MANIFEST: Asset = asset!("/assets/manifest.json");

#[component]
pub(crate) fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(get_language_identifier())
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
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: INPUT_NUMBER_ARROWS_CSS }
        document::Stylesheet { href: INPUT_RANGE_CSS }
        document::Stylesheet { href: SELECT_ARROW_CSS }
        document::Link { rel: "manifest", href: MANIFEST, crossorigin: "use-credentials" }

        div {
            class: "min-h-screen py-4 flex flex-col text-gray-300 bg-gray-900",
            Router::<Route> {}
        }
    }
}
