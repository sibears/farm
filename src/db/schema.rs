// @generated automatically by Diesel CLI.

diesel::table! {
    flags (id) {
        id -> Integer,
        flag -> Text,
        sploit -> Nullable<Text>,
        team -> Nullable<Text>,
        time -> Timestamp,
        status -> Nullable<Text>,
        checksystem_response -> Nullable<Text>,
    }
}
