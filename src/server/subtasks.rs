use crate::models::subtask::{NewSubtask, Subtask};
#[cfg(feature = "server")]
use crate::server::database_connection::establish_database_connection;
#[cfg(feature = "server")]
use crate::server::tasks::trigger_task_updated_at;
#[cfg(feature = "server")]
use crate::server::updates::publish_update;
#[cfg(feature = "server")]
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use validator::Validate;

#[server]
pub(crate) async fn create_subtask(new_subtask: NewSubtask) -> Result<Subtask> {
    use crate::schema::subtasks;

    // TODO: replace with model sanitization (https://github.com/matous-volf/todo-baggins/issues/13)
    let mut new_subtask = new_subtask;
    new_subtask.title = new_subtask.title.trim().to_owned();

    new_subtask.validate()?;

    let mut connection = establish_database_connection()?;

    let created_subtask = diesel::insert_into(subtasks::table)
        .values(&new_subtask)
        .returning(Subtask::as_returning())
        .get_result(&mut connection)?;

    trigger_task_updated_at(new_subtask.task_id).await?;

    publish_update().await;
    Ok(created_subtask)
}

#[server]
pub(crate) async fn get_subtasks_of_task(filtered_task_id: i32) -> Result<Vec<Subtask>> {
    use crate::schema::subtasks::dsl::*;

    let mut connection = establish_database_connection()?;

    let results = subtasks
        .select(Subtask::as_select())
        .filter(task_id.eq(filtered_task_id))
        .load::<Subtask>(&mut connection)?;

    Ok(results)
}

#[server]
pub(crate) async fn edit_subtask(subtask_id: i32, new_subtask: NewSubtask) -> Result<Subtask> {
    use crate::schema::subtasks::dsl::*;

    // TODO: replace with model sanitization (https://github.com/matous-volf/todo-baggins/issues/13)
    let mut new_subtask = new_subtask;
    new_subtask.title = new_subtask.title.trim().to_owned();

    new_subtask.validate()?;

    let mut connection = establish_database_connection()?;

    let updated_task = diesel::update(subtasks)
        .filter(id.eq(subtask_id))
        .set((
            title.eq(new_subtask.title),
            is_completed.eq(new_subtask.is_completed),
        ))
        .returning(Subtask::as_returning())
        .get_result(&mut connection)?;

    trigger_task_updated_at(new_subtask.task_id).await?;

    publish_update().await;
    Ok(updated_task)
}

#[cfg(feature = "server")]
pub(super) async fn restore_subtasks_of_task(filtered_task_id: i32) -> Result<Vec<Subtask>> {
    use crate::schema::subtasks::dsl::*;

    let mut connection = establish_database_connection()?;

    let updated_subtasks = diesel::update(subtasks)
        .filter(task_id.eq(filtered_task_id))
        .set(is_completed.eq(false))
        .returning(Subtask::as_returning())
        .get_results(&mut connection)?;

    Ok(updated_subtasks)
}

#[server]
pub(crate) async fn delete_subtask(subtask_id: i32) -> Result<()> {
    use crate::schema::subtasks::dsl::*;

    let mut connection = establish_database_connection()?;

    let deleted_subtask = diesel::delete(subtasks.filter(id.eq(subtask_id)))
        .returning(Subtask::as_returning())
        .get_result(&mut connection)?;

    trigger_task_updated_at(deleted_subtask.task_id).await?;

    publish_update().await;
    Ok(())
}
