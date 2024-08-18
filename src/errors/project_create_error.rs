use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProjectCreateError {
    TitleLengthInvalid,
    Error(Error),
}

impl From<ValidationErrors> for ErrorVec<ProjectCreateError> {
    fn from(e: ValidationErrors) -> Self {
        e.errors()
            .iter()
            .flat_map(|(&field, error_kind)| match field {
                "title" => match error_kind {
                    ValidationErrorsKind::Field(validation_errors) => validation_errors
                        .iter()
                        .map(|validation_error| validation_error.code.as_ref())
                        .map(|code| match code {
                            "title_length" => ProjectCreateError::TitleLengthInvalid,
                            _ => panic!("unexpected validation error code: {code}"),
                        })
                        .collect::<Vec<ProjectCreateError>>(),
                    _ => panic!("unexpected validation error kind"),
                },
                _ => panic!("unexpected validation field name: {field}"),
            })
            .collect::<Vec<ProjectCreateError>>()
            .into()
    }
}

// has to be implemented for Dioxus server functions
impl Display for ProjectCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// has to be implemented for Dioxus server functions
impl FromStr for ProjectCreateError {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(ProjectCreateError::TitleLengthInvalid)
    }
}
