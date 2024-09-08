use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Error {
    ServerInternal,
}

impl From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self { 
        Self::ServerInternal 
    }
}

// has to be implemented for Dioxus server functions
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServerInternal => write!(f, "internal server error"),
        }
    }
}

// has to be implemented for Dioxus server functions
impl FromStr for Error {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "internal server error" => Self::ServerInternal,
            _ => return Err(()),
        })
    }
}
