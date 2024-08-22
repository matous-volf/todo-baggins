use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskCreateError {
    TitleLengthInvalid,
    ProjectNotFound,
    Error(Error),
}

impl From<ValidationErrors> for ErrorVec<TaskCreateError> {
    fn from(validation_errors: ValidationErrors) -> Self {
        validation_errors.errors()
            .iter()
            .flat_map(|(&field, error_kind)| match field {
                "title" => match error_kind {
                    ValidationErrorsKind::Field(validation_errors) => validation_errors
                        .iter()
                        .map(|validation_error| validation_error.code.as_ref())
                        .map(|code| match code {
                            "title_length" => TaskCreateError::TitleLengthInvalid,
                            _ => panic!("Unexpected validation error code: `{code}`."),
                        })
                        .collect::<Vec<TaskCreateError>>(),
                    _ => panic!("Unexpected validation error kind."),
                },
                _ => panic!("Unexpected validation field name: `{field}`."),
            })
            .collect::<Vec<TaskCreateError>>()
            .into()
    }
}

// Has to be implemented for Dioxus server functions.
impl Display for TaskCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Has to be implemented for Dioxus server functions.
impl FromStr for TaskCreateError {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(TaskCreateError::Error(Error::ServerInternal))
    }
}