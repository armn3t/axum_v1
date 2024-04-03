use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

use diesel_async::AsyncPgConnection;
use serde_json::{json, Value};

use crate::{get_conn, test_conn, AppStateType};

pub async fn healthcheck(State(state): AppStateType) -> Json<Value> {
    test_conn(&state.pool).await;
    println!("Healtcheck OK");
    Json(json!({"db": "ok"}))
}

pub async fn root() -> Json<Value> {
    Json(json!({"message": "Hello!"}))
}