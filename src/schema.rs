// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        username -> Varchar,
        password -> Varchar,
        api_token -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
