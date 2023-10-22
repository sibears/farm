// @generated automatically by Diesel CLI.

diesel::table! {
    flags (id) {
        id -> Int4,
        flag -> Text,
        sploit -> Nullable<Text>,
        team -> Nullable<Text>,
        time -> Timestamp,
        status -> Text,
        checksystem_response -> Nullable<Text>,
    }
}
