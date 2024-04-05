pub mod users;
pub mod common;
pub mod auth;

use std::sync::Arc;

use axum::{
    extract::State,
    middleware::{self, FromExtractor},
    routing::{any, delete, get, patch, post},
    Router,
};

use crate::{middlewares::auth::is_authenticated, AppState, AppStateType};

pub fn get_user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(users::all))
        .route("/", post(users::create))
        .route("/:user_id", patch(users::update))
        .route("/:user_id", get(users::one))
        .route("/:user_id", delete(users::delete))
}

pub fn get_auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
}

pub fn get_authenticated_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/authenticated", any(auth::authenticated))
        .layer(middleware::from_fn_with_state(state.clone(), is_authenticated))
}

pub fn get_common_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(common::root))
        .route("/healthcheck", get(common::healthcheck))
}