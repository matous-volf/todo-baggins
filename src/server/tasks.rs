use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::models::task::{NewTask, Task};
use crate::server::database_connection::establish_database_connection;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
use validator::Validate;
use crate::errors::task_create_error::TaskCreateError;
use crate::models::category::Category;

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

#[server]
pub(crate) async fn get_tasks_in_category(filtered_category: Category)
    -> Result<Vec<Task>, ServerFnError<ErrorVec<Error>>> {
    use crate::schema::tasks::dsl::*;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<Error>, _>(
            |_| vec![Error::ServerInternal].into()
        )?;

    let results = tasks
        .select(Task::as_select())
        .filter(filtered_category.eq_sql_predicate())
        .load::<Task>(&mut connection)
        .map_err::<ErrorVec<Error>, _>(
            |_| vec![Error::ServerInternal].into()
        )?;

    Ok(results)
}
