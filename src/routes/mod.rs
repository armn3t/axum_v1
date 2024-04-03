pub mod users;
pub mod common;

use std::sync::Arc;

use axum::{
    extract::State,
    middleware::FromExtractor,
    routing::{delete, get, post},
    Router,
};

use crate::AppState;

pub fn get_user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(users::all))
        .route("/", post(users::create))
        .route("/:user_id", get(users::one))
        .route("/:user_id", delete(users::delete))
}

pub fn get_common_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(common::root))
        .route("/healthcheck", get(common::healthcheck))
}