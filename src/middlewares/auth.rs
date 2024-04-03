use axum::{
    body::Body, http::Request, middleware::Next, response::Response
};

pub async fn get_token(mut req: Request<Body>, next: Next) -> Response {
    println!("GET TOKEN MIDDLEWARE {:?}", req.headers());
    req.extensions_mut().insert("whatever");
    next.run(req).await
}
