use crate::models::project::{NewProject, Project};
use crate::server::database_connection::establish_database_connection;
use diesel::{RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;

#[server]
pub(crate) async fn create_project(title: String) -> Result<Project, ServerFnError> {
    use crate::schema::projects;

    let mut connection = establish_database_connection();

    let new_project = NewProject {
        title: title.as_str(),
    };

    Ok(diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(&mut connection)
        .expect("error saving a new project"))
}
