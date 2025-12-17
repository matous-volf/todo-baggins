use crate::models::project::{NewProject, Project};
#[cfg(feature = "server")]
use crate::server::database_connection::establish_database_connection;
#[cfg(feature = "server")]
use crate::server::updates::publish_update;
#[cfg(feature = "server")]
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use validator::Validate;

#[server]
pub(crate) async fn create_project(new_project: NewProject) -> Result<Project> {
    use crate::schema::projects;

    // TODO: replace with model sanitization (https://github.com/matous-volf/todo-baggins/issues/13)
    let mut new_project = new_project;
    new_project.title = new_project.title.trim().to_owned();

    new_project.validate()?;

    let mut connection = establish_database_connection()?;
    let new_project = diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(&mut connection)?;

    publish_update().await;

    Ok(new_project)
}

#[server]
pub(crate) async fn get_projects() -> Result<Vec<Project>> {
    use crate::schema::projects::dsl::*;

    let mut connection = establish_database_connection()?;
    let results = projects
        .select(Project::as_select())
        .load::<Project>(&mut connection)?;

    Ok(results)
}

#[server]
pub(crate) async fn edit_project(project_id: i32, new_project: NewProject) -> Result<Project> {
    use crate::schema::projects::dsl::*;

    // TODO: replace with model sanitization (https://github.com/matous-volf/todo-baggins/issues/13)
    let mut new_project = new_project;
    new_project.title = new_project.title.trim().to_owned();

    new_project.validate()?;

    let mut connection = establish_database_connection()?;
    let updated_project = diesel::update(projects)
        .filter(id.eq(project_id))
        .set(title.eq(new_project.title))
        .returning(Project::as_returning())
        .get_result(&mut connection)?;

    publish_update().await;
    Ok(updated_project)
}

#[server]
pub(crate) async fn delete_project(project_id: i32) -> Result<()> {
    use crate::schema::projects::dsl::*;

    let mut connection = establish_database_connection()?;
    diesel::delete(projects.filter(id.eq(project_id))).execute(&mut connection)?;

    publish_update().await;
    Ok(())
}
