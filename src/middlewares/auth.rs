use axum::{
    http::{self, Request},
    middleware::{self, Next},
    response::Response,
};

pub async fn get_token<B>(mut req: Request<B>, next: Next<B>) -> Response {
    println!("GET TOKEN MIDDLEWARE {:?}", req.headers());
    req.extensions_mut().insert("whatever");
    next.run(req).await
}
