use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, query_builder::AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub username: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub name: String,
    pub username: String,
}
