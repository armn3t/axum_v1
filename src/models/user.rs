use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, query_builder::AsChangeset, Insertable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, AsChangeset, Deserialize, Serialize, Debug, Clone)]
#[diesel(table_name=users)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub api_token: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserInput {
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name=users)]
pub struct UpdatableFieldsUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUserFields {
    pub name: String,
    pub username: String,
    pub password: String,
    pub api_token: String,
}
