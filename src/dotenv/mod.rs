load_dotenv::load_dotenv!();

pub(crate) const LANGUAGE_CODE: &str = env!("LANGUAGE_CODE");
#[cfg(feature = "mobile")]
pub(crate) const MOBILE_SERVER_URL: &str = env!("MOBILE_SERVER_URL");
