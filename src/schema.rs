// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        username -> Varchar,
        created_at -> Timestamp,
    }
}
