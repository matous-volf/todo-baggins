use chrono::{Datelike, Days, Months, NaiveDate};
use crate::errors::error::Error;
use crate::errors::error_vec::ErrorVec;
use crate::models::task::{NewTask, Task};
use crate::server::database_connection::establish_database_connection;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use dioxus::prelude::*;
use time::util::days_in_year_month;
use validator::Validate;
use crate::errors::task_error::TaskError;
use crate::models::category::{Category, ReoccurrenceInterval};

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
pub(crate) async fn get_task(task_id: i32) -> Result<Task, ServerFnError<ErrorVec<Error>>> {
    use crate::schema::tasks::dsl::*;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<Error>, _>(|_| vec![Error::ServerInternal].into())?;

    let task = tasks
        .find(task_id)
        .select(Task::as_select())
        .first(&mut connection)
        .optional()
        .map_err::<ErrorVec<Error>, _>(|_| vec![Error::ServerInternal].into())?;

    // TODO: Handle not finding the task.
    Ok(task.unwrap())
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

#[server]
pub(crate) async fn complete_task(task_id: i32) -> Result<Task, ServerFnError<ErrorVec<Error>>> {
    let task = get_task(task_id).await?;
    let mut new_task = NewTask::from(task);

    if let Category::Calendar {
        reoccurrence: Some(reoccurrence),
        date,
        ..
    } = &mut new_task.category {
        match reoccurrence.interval() {
            ReoccurrenceInterval::Day => *date = *date + Days::new(reoccurrence.length() as u64),
            ReoccurrenceInterval::Month | ReoccurrenceInterval::Year => {
                *date = *date + Months::new(
                    reoccurrence.length() *
                        if *(reoccurrence.interval()) == ReoccurrenceInterval::Year
                        { 12 } else { 1 }
                );
                *date = NaiveDate::from_ymd_opt(
                    date.year(),
                    date.month(),
                    reoccurrence.start_date().day().min(days_in_year_month(
                        date.year(),
                        (date.month() as u8).try_into().unwrap(),
                    ) as u32),
                ).unwrap()
            }
        }
    } else {
        new_task.category = Category::Done;
    }

    let updated_task = edit_task(task_id, new_task).await
        .map_err::<ErrorVec<Error>, _>(|_| vec![Error::ServerInternal].into())?;

    Ok(updated_task)
}

// TODO: Get rid of this suppression.
//noinspection DuplicatedCode
#[server]
pub(crate) async fn delete_task(task_id: i32)
                                -> Result<(), ServerFnError<ErrorVec<Error>>> {
    use crate::schema::tasks::dsl::*;

    let mut connection = establish_database_connection()
        .map_err::<ErrorVec<Error>, _>(|_| vec![Error::ServerInternal].into())?;

    diesel::delete(tasks.filter(id.eq(task_id))).execute(&mut connection)
        .map_err::<ErrorVec<Error>, _>(|error| vec![error.into()].into())?;

    Ok(())
}
