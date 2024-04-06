use axum::body::Body;
use uuid::Uuid;

use axum::{http::Request, middleware::Next, response::Response};

use std::fmt::{Display, Formatter, Result};
use std::time::Instant;

#[derive(Clone)]
pub struct RequestId {
    id: String,
}

#[derive(Copy, Clone)]
pub struct ReqMetrics {
    start: Instant,
}

impl ReqMetrics {
    fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl RequestId {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl Display for RequestId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.id)
    }
}

pub async fn set_req_id(mut req: Request<Body>, next: Next) -> Response {
    let req_id = RequestId::new();
    // let req_id = Uuid::new_v4().to_string();
    let start = ReqMetrics::new();
    println!("FIRST MW. req_id: {}", req_id);
    println!("FIRST MW. path: {}", req.uri());
    req.extensions_mut().insert(req_id);
    req.extensions_mut().insert(start);
    next.run(req).await
}

pub async fn measure_req(req: Request<Body>, next: Next) -> Response {
    let req_ext = req.extensions().get::<RequestId>();
    let metrics = req.extensions().get::<ReqMetrics>().unwrap();

    let req_id = match req_ext {
        Some(val) => {
            // println!("LAst MW: {}", val);
            val.id.clone()
        }
        None => "".to_string(),
    };
    println!("LAST MW: {}", req_id);
    println!("Before handler: {:.2?}", metrics.start.elapsed());

    let start = metrics.start.clone();

    let response = next.run(req).await;

    println!("Request - {} - time: {:.2?}", req_id, start.elapsed());
    response
}
