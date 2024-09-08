use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProjectError {
    TitleLengthInvalid,
    Error(Error),
}

impl From<ValidationErrors> for ErrorVec<ProjectError> {
    fn from(validation_errors: ValidationErrors) -> Self {
        validation_errors.errors()
            .iter()
            .flat_map(|(&field, error_kind)| match field {
                "title" => match error_kind {
                    ValidationErrorsKind::Field(validation_errors) => validation_errors
                        .iter()
                        .map(|validation_error| validation_error.code.as_ref())
                        .map(|code| match code {
                            "title_length" => ProjectError::TitleLengthInvalid,
                            _ => panic!("Unexpected validation error code: `{code}`."),
                        })
                        .collect::<Vec<ProjectError>>(),
                    _ => panic!("Unexpected validation error kind."),
                },
                _ => panic!("Unexpected validation field name: `{field}`."),
            })
            .collect::<Vec<ProjectError>>()
            .into()
    }
}

impl From<diesel::result::Error> for ProjectError {
    fn from(_: diesel::result::Error) -> Self {
        Self::Error(Error::ServerInternal)
    }
}

// Has to be implemented for Dioxus server functions.
impl Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Has to be implemented for Dioxus server functions.
impl FromStr for ProjectError {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self::Error(Error::ServerInternal))
    }
}
