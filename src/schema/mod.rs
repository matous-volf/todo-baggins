// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int4,
        title -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        deadline -> Nullable<Date>,
        category -> Jsonb,
        project_id -> Nullable<Int4>,
    }
}

diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
);
