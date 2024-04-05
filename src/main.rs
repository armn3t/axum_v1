mod middlewares;
mod models;
mod repositories;
mod routes;
mod lib;
mod schema;

use axum::{extract::State, middleware, Router};

use bb8::PooledConnection;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl};

use tokio::signal;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::{
    middlewares::{
        auth::get_token,
        request::{measure_req, set_req_id},
    },
    routes::{get_auth_router, get_common_router, get_user_router, get_authenticated_router},
};

// #[derive(Clone)]
pub struct AppState {
    pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

pub type AppStateType = State<Arc<AppState>>;

pub async fn get_conn(
    pool: &bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
) -> PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> {
    let conn = pool.get().await.unwrap();
    conn
}

async fn test_conn(
    pool: &bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
) {
    let mut conn = get_conn(&pool).await;

    let result = diesel::sql_query("SELECT 1").execute(&mut conn).await.expect("Healthcheck result");

    println!("Connection result: {}", result);

}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .json()
        .init();
    tracing::info!("Attempting to start axum server");

    tracing::info!("Attempting to retrieve database URL");
    let db_url = std::env::var("DATABASE_URL").unwrap();
    tracing::info!("Successfully retrieved database URL");
    
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    tracing::info!("Attempting to create database connection pool");
    let pool = bb8::Pool::builder().build(config).await.unwrap();
    tracing::info!("Successfully created database connection pool");

    let state = Arc::new(AppState {
        pool,
        // ids: AppIdentifiers { requestId: None },
    });

    test_conn(&state.pool).await;
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 5005));

    let app = Router::new()
        .nest("/", get_common_router())
        .nest("/", get_auth_router())
        .nest("/", get_authenticated_router(state.clone()))
        .nest("/users", get_user_router())
        .layer(middleware::from_fn(measure_req))
        .layer(middleware::from_fn(get_token))
        .layer(middleware::from_fn(set_req_id))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Axum listening on {}", &addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}