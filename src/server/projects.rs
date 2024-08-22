use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::errors::project_create_error::ProjectCreateError;
use crate::models::project::{NewProject, Project};
use crate::server::database_connection::establish_database_connection;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
use validator::Validate;

#[server]
pub(crate) async fn create_project(new_project: NewProject)
                                   -> Result<Project, ServerFnError<ErrorVec<ProjectCreateError>>> {
    use crate::schema::projects;

    new_project.validate()
        .map_err::<ErrorVec<ProjectCreateError>, _>(|errors| errors.into())?;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<ProjectCreateError>, _>(
            |_| vec![ProjectCreateError::Error(Error::ServerInternal)].into()
        )?;

    let new_project = diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<ProjectCreateError>, _>(
            |_| vec![ProjectCreateError::Error(Error::ServerInternal)].into()
        )?;

    Ok(new_project)
}

#[server]
pub(crate) async fn get_projects()
    -> Result<Vec<Project>, ServerFnError> {
    use crate::schema::projects::dsl::*;

    let mut connection = establish_database_connection()?;

    let results = projects
        .select(Project::as_select())
        .load::<Project>(&mut connection)?;

    Ok(results)
}
