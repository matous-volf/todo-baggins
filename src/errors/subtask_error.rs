use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Serialize, Deserialize, Debug)]
pub enum SubtaskError {
    TitleLengthInvalid,
    TaskNotFound,
    Error(Error),
}

impl From<ValidationErrors> for ErrorVec<SubtaskError> {
    fn from(validation_errors: ValidationErrors) -> Self {
        validation_errors.errors()
            .iter()
            .flat_map(|(&field, error_kind)| match field {
                "title" => match error_kind {
                    ValidationErrorsKind::Field(validation_errors) => validation_errors
                        .iter()
                        .map(|validation_error| validation_error.code.as_ref())
                        .map(|code| match code {
                            "title_length" => SubtaskError::TitleLengthInvalid,
                            _ => panic!("Unexpected validation error code: `{code}`."),
                        })
                        .collect::<Vec<SubtaskError>>(),
                    _ => panic!("Unexpected validation error kind."),
                },
                _ => panic!("Unexpected validation field name: `{field}`."),
            })
            .collect::<Vec<SubtaskError>>()
            .into()
    }
}

impl From<diesel::result::Error> for SubtaskError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::ForeignKeyViolation, info
            ) => {
                match info.constraint_name() {
                    Some("subtasks_task_id_fkey") => Self::TaskNotFound,
                    _ => Self::Error(Error::ServerInternal)
                }
            }
            _ => {
                Self::Error(Error::ServerInternal)
            }
        }
    }
}

// Has to be implemented for Dioxus server functions.
impl Display for SubtaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Has to be implemented for Dioxus server functions.
impl FromStr for SubtaskError {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self::Error(Error::ServerInternal))
    }
}
