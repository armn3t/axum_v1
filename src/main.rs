mod models;
mod repositories;
mod routes;
mod schema;

// use diesel::PgConnection;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Router,
};

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use std::net::SocketAddr;
use std::sync::Arc;

use crate::repositories::users;

pub struct AppState {
    pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

pub type AppStateType = State<Arc<AppState>>;

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
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5005));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    tracing::debug!("Axum listening on {}", &addr);
}

async fn root() -> String {
    return "Hello!".to_string();
}
