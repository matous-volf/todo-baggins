// For Diesel DSL.
#![allow(clippy::wildcard_imports)]

#[cfg(feature = "server")]
pub(crate) mod database_connection;
pub(crate) mod projects;
pub(crate) mod subtasks;
pub(crate) mod tasks;
pub(crate) mod updates;
