// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int4,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    subtasks (id) {
        id -> Int4,
        task_id -> Int4,
        title -> Text,
        is_completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        deadline -> Nullable<Date>,
        category -> Jsonb,
        project_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(subtasks -> tasks (task_id));
diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    subtasks,
    tasks,
);
