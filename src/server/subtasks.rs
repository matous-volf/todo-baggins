use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::errors::subtask_error::SubtaskError;
use crate::models::subtask::{NewSubtask, Subtask};
use crate::server::database_connection::establish_database_connection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
use validator::Validate;
use crate::server::tasks::trigger_task_updated_at;

#[server]
pub(crate) async fn create_subtask(new_subtask: NewSubtask)
                                   -> Result<Subtask, ServerFnError<ErrorVec<SubtaskError>>> {
    use crate::schema::subtasks;

    // TODO: replace with model sanitization (https://github.com/matous-volf/todo-baggins/issues/13)
    let mut new_subtask = new_subtask;
    new_subtask.title = new_subtask.title.trim().to_owned();

    new_subtask.validate()
        .map_err::<ErrorVec<SubtaskError>, _>(|errors| errors.into())?;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<SubtaskError>, _>(
            |_| vec![SubtaskError::Error(Error::ServerInternal)].into()
        )?;

    let created_subtask = diesel::insert_into(subtasks::table)
        .values(&new_subtask)
        .returning(Subtask::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<SubtaskError>, _>(|error| vec![error.into()].into())?;
    
    trigger_task_updated_at(new_subtask.task_id).await
        .map_err::<ErrorVec<SubtaskError>, _>(|error_vec| error_vec.into())?;

    Ok(created_subtask)
}

#[server]
pub(crate) async fn get_subtasks_of_task(filtered_task_id: i32)
                                         -> Result<Vec<Subtask>, ServerFnError<ErrorVec<Error>>> {
    use crate::schema::subtasks::dsl::*;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<Error>, _>(|_| vec![Error::ServerInternal].into())?;

    let results = subtasks
        .select(Subtask::as_select())
        .filter(task_id.eq(filtered_task_id))
        .load::<Subtask>(&mut connection)
        .map_err::<ErrorVec<Error>, _>(|_| vec![Error::ServerInternal].into())?;

    Ok(results)
}

#[server]
pub(crate) async fn edit_subtask(subtask_id: i32, new_subtask: NewSubtask)
                                 -> Result<Subtask, ServerFnError<ErrorVec<SubtaskError>>> {
    use crate::schema::subtasks::dsl::*;

    // TODO: replace with model sanitization (https://github.com/matous-volf/todo-baggins/issues/13)
    let mut new_subtask = new_subtask;
    new_subtask.title = new_subtask.title.trim().to_owned();

    new_subtask.validate()
        .map_err::<ErrorVec<SubtaskError>, _>(|errors| errors.into())?;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<SubtaskError>, _>(
            |_| vec![SubtaskError::Error(Error::ServerInternal)].into()
        )?;

    let updated_task = diesel::update(subtasks)
        .filter(id.eq(subtask_id))
        .set((
            title.eq(new_subtask.title),
            is_completed.eq(new_subtask.is_completed)
        ))
        .returning(Subtask::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<SubtaskError>, _>(|error| vec![error.into()].into())?;

    trigger_task_updated_at(new_subtask.task_id).await
        .map_err::<ErrorVec<SubtaskError>, _>(|error_vec| error_vec.into())?;
    
    Ok(updated_task)
}

#[server]
pub(crate) async fn restore_subtasks_of_task(filtered_task_id: i32) -> Result<
    Vec<Subtask>,
    ServerFnError<ErrorVec<Error>>
> {
    use crate::schema::subtasks::dsl::*;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<Error>, _>(|_| vec![Error::ServerInternal].into())?;

    let updated_subtasks = diesel::update(subtasks)
        .filter(task_id.eq(filtered_task_id))
        .set(is_completed.eq(false))
        .returning(Subtask::as_returning())
        .get_results(&mut connection)
        .map_err::<ErrorVec<Error>, _>(|error| vec![error.into()].into())?;

    Ok(updated_subtasks)
}

// TODO: Get rid of this suppression.
//noinspection DuplicatedCode
#[server]
pub(crate) async fn delete_subtask(subtask_id: i32)
                                   -> Result<(), ServerFnError<ErrorVec<Error>>> {
    use crate::schema::subtasks::dsl::*;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<Error>, _>(|_| vec![Error::ServerInternal].into())?;

    let deleted_subtask = diesel::delete(subtasks.filter(id.eq(subtask_id)))
        .returning(Subtask::as_returning())
        .get_result(&mut connection)
        .map_err::<ErrorVec<Error>, _>(|error| vec![error.into()].into())?;

    trigger_task_updated_at(deleted_subtask.task_id()).await?;

    Ok(())
}
