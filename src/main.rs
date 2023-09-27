mod models;
mod repositories;
mod schema;

// use diesel::PgConnection;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};

// use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use bb8::PooledConnection;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::repositories::users;

#[tokio::main]
async fn main() {
    // let manager = Manager::new(":memory:", Runtime::Tokio1);
    // let pool = Pool::builder(manager)
    //     .max_size(8)
    //     .build()
    //     .unwrap();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let conn = pool.get().await.unwrap();

    let app = Router::new().route("/", get(root)).with_state(pool);

    axum::Server::bind(&"127.0.0.1:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root() -> String {
    return "Hello!".to_string();
}
