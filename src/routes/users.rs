use axum::{extract::State, http::StatusCode, response::Json};
use diesel_async::AsyncPgConnection;
use serde_json::{json, Value};

use crate::{
    repositories::{self, users::UsersRepository},
    AppStateType,
};

pub async fn all(State(state): AppStateType) -> Json<Value> {
    let mut conn: bb8::PooledConnection<
        '_,
        diesel_async::pooled_connection::AsyncDieselConnectionManager<AsyncPgConnection>,
    > = state.pool.get().await.unwrap();

    let users = UsersRepository::find_multiple(&mut conn).await.unwrap();
    Json(json!({"data": 50, "users": users }))
}
