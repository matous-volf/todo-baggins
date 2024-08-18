use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProjectCreateError {
    TitleTooShort,
    Error(Error),
}

impl From<ValidationErrors> for ErrorVec<ProjectCreateError> {
    fn from(e: ValidationErrors) -> Self {
        e.errors()
            .iter()
            .flat_map(|(&field, error_kind)| match field {
                "title_length" => match error_kind {
                    ValidationErrorsKind::Field(validation_errors) => validation_errors
                        .iter()
                        .map(|validation_error| match validation_error.code.as_ref() {
                            "length" => ProjectCreateError::TitleTooShort,
                            _ => ProjectCreateError::Error(Error::ServerInternal),
                        })
                        .collect::<Vec<ProjectCreateError>>(),
                    _ => panic!("unexpected error kind"),
                },
                _ => panic!("unexpected field name"),
            })
            .collect::<Vec<ProjectCreateError>>()
            .into()
    }
}

// has to be implemented for Dioxus server functions
impl Display for ProjectCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", dbg!(self))
    }
}

// has to be implemented for Dioxus server functions
impl FromStr for ProjectCreateError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ProjectCreateError::Error(Error::ServerInternal))
    }
}
