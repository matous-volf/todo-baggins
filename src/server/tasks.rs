use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::models::task::{NewTask, Task};
use crate::server::database_connection::establish_database_connection;
use diesel::{RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
use validator::Validate;
use crate::errors::task_create_error::TaskCreateError;

#[server]
pub(crate) async fn create_task(new_task: NewTask)
                                -> Result<Task, ServerFnError<ErrorVec<TaskCreateError>>> {
    use crate::schema::tasks;

    new_task.validate()
        .map_err::<ErrorVec<TaskCreateError>, _>(|errors| errors.into())?;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<TaskCreateError>, _>(
            |_| vec![TaskCreateError::Error(Error::ServerInternal)].into()
        )?;

    let new_task = diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<TaskCreateError>, _>(|error| {
            let error = match error {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation, info
                ) => {
                    match info.constraint_name() { 
                        Some("tasks_project_id_fkey") => TaskCreateError::ProjectNotFound,
                        _ => TaskCreateError::Error(Error::ServerInternal)
                    }
                },
                _ => {
                    TaskCreateError::Error(Error::ServerInternal)
                }
            };
            vec![error].into()
        })?;

    Ok(new_task)
}
