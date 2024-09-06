use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::errors::project_create_error::ProjectError;
use crate::models::project::{NewProject, Project};
use crate::server::database_connection::establish_database_connection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
use validator::Validate;

#[server]
pub(crate) async fn create_project(new_project: NewProject)
                                   -> Result<Project, ServerFnError<ErrorVec<ProjectError>>> {
    use crate::schema::projects;

    new_project.validate()
        .map_err::<ErrorVec<ProjectError>, _>(|errors| errors.into())?;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<ProjectError>, _>(
            |_| vec![ProjectError::Error(Error::ServerInternal)].into()
        )?;

    let new_project = diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<ProjectError>, _>(
            |_| vec![ProjectError::Error(Error::ServerInternal)].into()
        )?;

    Ok(new_project)
}

#[server]
pub(crate) async fn get_projects()
    -> Result<Vec<Project>, ServerFnError<ErrorVec<Error>>> {
    use crate::schema::projects::dsl::*;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<Error>, _>(
            |_| vec![Error::ServerInternal].into()
        )?;

    let results = projects
        .select(Project::as_select())
        .load::<Project>(&mut connection)
        .map_err::<ErrorVec<Error>, _>(
            |_| vec![Error::ServerInternal].into()
        )?;

    Ok(results)
}

#[server]
pub(crate) async fn edit_project(project_id: i32, new_project: NewProject)
    -> Result<Project, ServerFnError<ErrorVec<ProjectError>>> {
    use crate::schema::projects::dsl::*;

    new_project.validate()
        .map_err::<ErrorVec<ProjectError>, _>(|errors| errors.into())?;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<ProjectError>, _>(
            |_| vec![ProjectError::Error(Error::ServerInternal)].into()
        )?;

    let updated_project = diesel::update(projects)
        .filter(id.eq(project_id))
        .set(title.eq(new_project.title))
        .returning(Project::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<ProjectError>, _>(
            |_| vec![ProjectError::Error(Error::ServerInternal)].into()
        )?;

    Ok(updated_project)
}
