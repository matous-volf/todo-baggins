use std::fmt::Display;
use std::str::FromStr;
use serde::Deserialize;
use serde_with::serde_derive::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorVec<T> {
    errors: Vec<T>,
}

impl<T> From<ErrorVec<T>> for Vec<T> {
    fn from(e: ErrorVec<T>) -> Self {
        e.errors
    }
}

impl<T> From<Vec<T>> for ErrorVec<T> {
    fn from(e: Vec<T>) -> Self {
        ErrorVec { errors: e }
    }
}

// has to be implemented for Dioxus server functions
impl<T: Display> Display for ErrorVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

// has to be implemented for Dioxus server functions
impl<T> FromStr for ErrorVec<T> {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(ErrorVec { errors: Vec::new() })
    }
}
