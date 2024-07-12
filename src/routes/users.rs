use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};

use crate::{
    models::user::{NewUserInput, UpdatableFieldsUser},
    repositories::users::UsersRepository,
};

use crate::{get_conn, AppStateType};

pub async fn all(State(state): AppStateType) -> (StatusCode, Json<Value>) {
    let mut conn = get_conn(&state.pool).await;

    match UsersRepository::find_multiple(&mut conn).await {
        Ok(users) => (StatusCode::OK, Json(json!({"users": users }))),
        Err(err) => {
            tracing::info!("Error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Error fetching users"})),
            )
        }
    }
}

pub async fn one(State(state): AppStateType) -> Json<Value> {
    let mut conn = get_conn(&state.pool).await;

    let user = UsersRepository::find(&mut conn, 1).await.unwrap();
    Json(json!({"user": user}))
}

pub async fn create(
    State(state): AppStateType,
    Json(payload): Json<NewUserInput>,
) -> (StatusCode, Json<Value>) {
    let mut conn = get_conn(&state.pool).await;
    let user = UsersRepository::create(&mut conn, payload).await.unwrap();

    (StatusCode::CREATED, Json(json!(user)))
}

pub async fn update(
    Path(user_id): Path<i32>,
    State(state): AppStateType,
    Json(payload): Json<UpdatableFieldsUser>,
) -> (StatusCode, Json<Value>) {
    let mut conn = get_conn(&state.pool).await;
    let updated_user = UsersRepository::update(&mut conn, user_id, payload)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!(updated_user)))
}

pub async fn delete(Path(user_id): Path<i32>, State(state): AppStateType) -> StatusCode {
    let mut conn = get_conn(&state.pool).await;
    UsersRepository::delete(&mut conn, user_id).await.unwrap();

    StatusCode::NO_CONTENT
}
