use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use diesel_async::AsyncPgConnection;
use serde_json::{json, Value};

use crate::{
    models::user::{NewUser, User},
    repositories::users::UsersRepository,
};

use crate::{get_conn, AppStateType};

pub async fn all(State(state): AppStateType) -> Json<Value> {
    println!("Get all users");

    let mut conn = get_conn(&state.pool).await;

    let users = UsersRepository::find_multiple(&mut conn).await.unwrap();
    println!("Users length: {}", users.len());

    Json(json!({"users": users }))
}

pub async fn one(State(state): AppStateType) -> Json<Value> {
    println!("Get user with id: ");

    let mut conn = get_conn(&state.pool).await;

    let user = UsersRepository::find(&mut conn, 1).await.unwrap();
    Json(json!({"user": user}))
}

pub async fn create(
    State(state): AppStateType,
    Json(payload): Json<NewUser>,
) -> (StatusCode, Json<Value>) {
    println!("Create user: {}", payload.username);

    let mut conn = get_conn(&state.pool).await;

    let user = UsersRepository::create(&mut conn, payload).await.unwrap();

    (StatusCode::CREATED, Json(json!(user)))
}

pub async fn delete(Path(user_id): Path<i32>, State(state): AppStateType) -> StatusCode {
    println!("Delete user: {}", user_id);

    let mut conn = get_conn(&state.pool).await;

    UsersRepository::delete(&mut conn, user_id).await.unwrap();

    StatusCode::NO_CONTENT
}
