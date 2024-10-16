// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "flag_status"))]
    pub struct FlagStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FlagStatus;

    flags (id) {
        id -> Int4,
        flag -> Text,
        sploit -> Nullable<Text>,
        team -> Nullable<Text>,
        time -> Timestamp,
        status -> FlagStatus,
        checksystem_response -> Nullable<Text>,
    }
}
