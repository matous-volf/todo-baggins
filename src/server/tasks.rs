use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::models::task::{NewTask, Task};
use crate::server::database_connection::establish_database_connection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
use validator::Validate;
use crate::errors::task_error::TaskError;
use crate::models::category::Category;

#[server]
pub(crate) async fn create_task(new_task: NewTask)
                                -> Result<Task, ServerFnError<ErrorVec<TaskError>>> {
    use crate::schema::tasks;

    new_task.validate()
        .map_err::<ErrorVec<TaskError>, _>(|errors| errors.into())?;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<TaskError>, _>(
            |_| vec![TaskError::Error(Error::ServerInternal)].into()
        )?;

    let new_task = diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<TaskError>, _>(|error| vec![error.into()].into())?;

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

#[server]
pub(crate) async fn edit_task(task_id: i32, new_task: NewTask)
                              -> Result<Task, ServerFnError<ErrorVec<TaskError>>> {
    use crate::schema::tasks::dsl::*;

    new_task.validate()
        .map_err::<ErrorVec<TaskError>, _>(|errors| errors.into())?;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<TaskError>, _>(
            |_| vec![TaskError::Error(Error::ServerInternal)].into()
        )?;

    let updated_task = diesel::update(tasks)
        .filter(id.eq(task_id))
        .set((
            title.eq(new_task.title),
            deadline.eq(new_task.deadline),
            category.eq(new_task.category),
            project_id.eq(new_task.project_id),
        ))
        .returning(Task::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<TaskError>, _>(|error| vec![error.into()].into())?;

    Ok(updated_task)
}
