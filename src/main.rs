use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};

use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod repositories;
// use crate::repositories::*;

fn main() {
    println!("Hello, world!");
}
