use reqwest::{blocking::Client, StatusCode};

pub mod common;

#[test]
fn test_healthcheck() {
    let client = Client::new();

    let res = client.get("/healthcheck").send().unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}