mod models;
mod repositories;
mod routes;
mod schema;

// use diesel::PgConnection;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::{delete, get, post},
    Router,
};

use serde_json::{json, Value};

use bb8::PooledConnection;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use std::sync::Arc;
use std::{error::Error, net::SocketAddr};

pub struct AppState {
    pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

pub type AppStateType = State<Arc<AppState>>;

pub async fn get_conn(
    pool: &bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
) -> PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> {
    let conn = pool.get().await.unwrap();
    // .map_err(db_error);
    conn
}

#[tokio::main]
async fn main() {
    // let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_url = "postgres://db_user:secret@db_host:5432/app_db";
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(routes::users::all))
        .route("/users", post(routes::users::create))
        .route("/users/:user_id", get(routes::users::one))
        .route("/users/:user_id", delete(routes::users::delete))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5005));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    tracing::debug!("Axum listening on {}", &addr);
}

// async fn db_error(&error: dyn Error) -> (StatusCode, Json<Value>) {
//     (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         Json(json!({ "message": error.to_string() })),
//     )
// }

async fn root() -> String {
    return "Hello!".to_string();
}
