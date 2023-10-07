mod middlewares;
mod models;
mod repositories;
mod routes;
mod schema;

use axum::{extract::State, middleware, routing::get, Extension, Router};

use bb8::PooledConnection;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use std::net::SocketAddr;
use std::sync::Arc;

use crate::{
    middlewares::{
        auth::get_token,
        request::{measure_req, set_req_id},
    },
    routes::get_user_router,
};

// pub struct AppIdentifiers {
//     requestId: Option<String>,
// }

pub struct AppState {
    pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    // ids: AppIdentifiers,
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

    let state = Arc::new(AppState {
        pool,
        // ids: AppIdentifiers { requestId: None },
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 5005));

    let app = Router::new()
        .route("/", get(root))
        .nest("/users", get_user_router())
        .layer(middleware::from_fn(measure_req))
        .layer(middleware::from_fn(get_token))
        .layer(middleware::from_fn(set_req_id))
        .with_state(state);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    tracing::debug!("Axum listening on {}", &addr);
}

async fn root() -> String {
    // println!("Extensions: {}");
    return "Hello!".to_string();
}
